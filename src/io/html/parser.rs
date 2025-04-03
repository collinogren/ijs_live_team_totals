/*
Copyright (c) 2023 Collin Ogren

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use std::{fs, thread};
use std::sync::{Arc, mpsc};
use scraper::{CaseSensitivity, Element, ElementRef, Html, Selector};
use scraper::selector::CssLocalName;
use crate::io::html::club_points::{auto_club_combiner, ClubPoints, sum_results};
use crate::io::html::event::Event;
use crate::io::html::points_system::PointsSystem;
use crate::io::html::result_set::{clean_club_names, ResultSet};
use crate::io::html::results_sorter;
use crate::io::html::scoring_system::ScoringSystem::{IJS, SixO};
use crate::settings::settings::Settings;

pub enum State {
    Ok,
    Error,
}

pub fn parse_results(events: Vec<Event>, settings: &Settings, competition_name: &String) -> (Vec<ClubPoints>, String, State) {
    let mut files_ijs = vec![];
    let mut files_60 = vec![];

    for event in &events {
        if event.active {
            if event.scoring_system == IJS {
                files_ijs.push(event.file_path.clone());
            } else if event.scoring_system == SixO {
                files_60.push(event.file_path.clone());
            }
        }
    }

    let files_ijs = Arc::new(files_ijs);
    let files_60 = Arc::new(files_60);

    let (results_ijs_sender, results_ijs_receiver) = mpsc::channel::<Vec<ResultSet>>();

    let files_ijs_clone = files_ijs.clone();
    let settings_clone = settings.clone();
    thread::spawn(move || {
        results_ijs_sender.send(parse_ijs(files_ijs_clone.to_vec(), settings_clone)).unwrap();
    });

    let (results_60_sender, results_60_receiver) = mpsc::channel::<Vec<ResultSet>>();

    let settings_copy = settings.clone();
    let files_60_clone = files_60.clone();
    thread::spawn(move || {
        results_60_sender.send(parse_60(files_60_clone.to_vec(), settings_copy)).unwrap();
    });

    //while !results_ijs_thread.is_finished() || !results_60_thread.is_finished() {}

    let results_ijs = results_ijs_receiver.recv().unwrap();
    let results_60 = results_60_receiver.recv().unwrap();

    let number_results_60_found = format!("Retrieved {} IJS results and {} 6.0 results.", results_ijs.len(), results_60.len());
    println!("{}", number_results_60_found);

    let mut combined_raw_results = results_ijs;
    combined_raw_results.extend(results_60);

    clean_club_names(&mut combined_raw_results);

    let mut results = sum_results(combined_raw_results, settings.clone());

    if settings.attempt_automatic_60_club_name_recombination {
        results = auto_club_combiner(results);
    }

    results_sorter::sort_results(&mut results);

    (results, String::from("Results Successfully Calculated"), State::Ok)
}

pub(crate) const HTML_CHARACTER_ENTITIES: [(&'static str, &'static str); 12] = [
    ("&nbsp;", " "),
    ("&lt;", "<"),
    ("&gt;", ">"),
    ("&amp;", "&"),
    ("&quot;", "\""),
    ("&apos;", "\'"),
    ("&cent;", "¢"),
    ("&pound;", "£"),
    ("&yen;", "¥"),
    ("&euro;", "€"),
    ("&copy;", "©"),
    ("&reg;", "®")
];

pub(crate) fn clean_event_names(mut event_names: Vec<Event>) -> Vec<Event> {
    for event_name in &mut event_names {
        for character_entities in HTML_CHARACTER_ENTITIES {
            let temp = event_name.event_name.replace(character_entities.0, character_entities.1).clone();
            event_name.event_name = temp;
        }
    }

    event_names
}

pub(crate) fn parse_ijs_event_names(ijs_events: &Vec<String>) -> Vec<Event> {
    let mut event_names = vec![];

    for results_file_path in ijs_events {
        let results_file_contents = fs::read_to_string(results_file_path).unwrap();
        let document = Html::parse_document(&results_file_contents);
        let selector = Selector::parse(r#"body > h2"#).unwrap();

        let document_select = document.select(&selector);
        let document_select_collection = document_select.collect::<Vec<ElementRef>>();
        for element in document_select_collection.clone().into_iter().enumerate() {
            if element.1.has_class(&CssLocalName::from("catseg"), CaseSensitivity::CaseSensitive) {
                let event_name = match
                String::from(element.1.html()
                    .split("<h2 class=\"catseg\">")
                    .nth(1)
                    .unwrap()
                    .split("</h2>")
                    .nth(0)
                    .unwrap()
                ).parse() {
                    Ok(n) => n,
                    Err(err) => panic!("{}", err),
                };

                event_names.push(Event::new(event_name, results_file_path.clone(), IJS, true));
                continue;
            }
        }
    }

    event_names
}

pub(crate) fn parse_60_event_names(ijs_events: &Vec<String>) -> Vec<Event> {
    let mut event_names = vec![];
    for results_file_path in ijs_events {
        let results_file_contents = fs::read_to_string(results_file_path).unwrap();
        let document = Html::parse_document(&results_file_contents);
        let selector = Selector::parse(r#"table > caption > h2"#).unwrap();

        let document_select = document.select(&selector);
        let document_select_collection = document_select.collect::<Vec<ElementRef>>();
        for element in document_select_collection.clone().into_iter().enumerate() {
            if element.1.html().starts_with("<h2>") {
                let next_element = document_select_collection.get(element.0 + 1).unwrap();
                if next_element.html().starts_with("<h2>") {
                    let event_name = match
                    String::from(next_element.html()
                        .split("<h2>")
                        .nth(1)
                        .unwrap()
                        .split("</h2>")
                        .nth(0)
                        .unwrap()
                    ).parse() {
                        Ok(n) => n,
                        Err(err) => panic!("{}", err),
                    };

                    event_names.push(Event::new(event_name, results_file_path.clone(), SixO, true));
                    break;
                }
            }
        }
    }

    event_names
}

pub fn parse_ijs(ijs_events: Vec<String>, settings: Settings) -> Vec<ResultSet> {
    let mut results = vec![];

    for results_file_path in ijs_events {
        let results_file_contents = fs::read_to_string(results_file_path).unwrap();
        let document = Html::parse_document(&results_file_contents);
        let selector = Selector::parse(r#"table > tbody > tr > td"#).unwrap();

        let document_select = document.select(&selector);
        let document_select_collection = document_select.collect::<Vec<ElementRef>>();
        for element in document_select_collection.clone().into_iter().enumerate() {
            let mut has_rank = false;
            let mut result_set = ResultSet::new(IJS, PointsSystem::default(&settings));
            if element.1.has_class(&CssLocalName::from("rank"), CaseSensitivity::CaseSensitive) {
                result_set.rank = Some(match
                String::from(element.1.html()
                    .split("<td class=\"rank\">")
                    .nth(1)
                    .unwrap()
                    .split("</td>")
                    .nth(0)
                    .unwrap()
                ).parse() {
                    Ok(n) => n,
                    Err(err) => panic!("{}", err),
                }
                );
                has_rank = true;
            }

            if !has_rank { continue; };

            let next_element = document_select_collection.get(element.0 + 1).unwrap();
            if next_element.has_class(&CssLocalName::from("name"), CaseSensitivity::CaseSensitive) {
                result_set.club = Some(
                    String::from(next_element.html()
                        .split("<td class=\"name\">")
                        .nth(1)
                        .unwrap()
                        .split("</td>")
                        .nth(0)
                        .unwrap()
                        .split(", ")
                        .nth(1)
                        .unwrap()
                    )
                );
            }

            results.push(result_set);
        }
    }

    results
}

pub fn parse_60(files_60: Vec<String>, settings: Settings) -> Vec<ResultSet> {
    let mut results = vec![];
    for results_file_path in files_60 {
        let results_file_contents = fs::read_to_string(results_file_path.clone()).unwrap();
        let document = Html::parse_document(&results_file_contents);
        let selector = Selector::parse(r#"table > tbody > tr > td"#).unwrap();

        let document_select = document.select(&selector);
        let document_select_collection = document_select.collect::<Vec<ElementRef>>();
        for element in document_select_collection.clone().into_iter().enumerate() {
            let mut has_name = false;
            let mut result_set = ResultSet::new(SixO, PointsSystem::default(&settings));

            if element.1.html().contains("<td rowspan=\"1\" colspan=\"1\">") { // Name first this time because the name has more distinctive markings for it in the HTML.
                let temp_club = String::from(element.1.html()
                    .split("<td rowspan=\"1\" colspan=\"1\">")
                    .nth(1)
                    .unwrap()
                    .split("</td>").nth(0)
                    .unwrap()
                    .split(", ")
                    .nth(1)
                    .unwrap()
                );

                if temp_club.contains("<br>") { // Handle duets and such by disregarding them. This is a terrible solution.
                    continue;
                }

                result_set.club = Some(temp_club);

                has_name = true;
            } else if element.1.html().contains("<td colspan=\"1\" rowspan=\"1\">") { // Name first this time because the name has more distinctive markings for it in the HTML.
                let temp_club = String::from(element.1.html()
                    .split("<td colspan=\"1\" rowspan=\"1\">")
                    .nth(1)
                    .unwrap()
                    .split("</td>").nth(0)
                    .unwrap()
                    .split(", ")
                    .nth(1)
                    .unwrap()
                );

                if temp_club.contains("<br>") { // Handle duets and such by disregarding them. This is a terrible solution.
                    continue;
                }

                result_set.club = Some(temp_club);

                has_name = true;
            }

            if !has_name {
                continue;
            };

            let next_element = document_select_collection.get(element.0 - 1).unwrap();
            let temp_rank = String::from(next_element.html()
                .split("<td>")
                .nth(1)
                .unwrap()
                .split(".</td>")
                .nth(0)
                .unwrap()
            );

            if temp_rank.contains("&nbsp;") {
                continue;
            }

            result_set.rank = Some(match temp_rank.parse() {
                Ok(n) => n,
                Err(err) => panic!("{}", err),
            });

            results.push(result_set);
        }
    }

    results
}