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
use std::sync::{Arc, mpsc, RwLock};
use scraper::{CaseSensitivity, Element, ElementRef, Html, Selector};
use scraper::selector::CssLocalName;

use crate::parser::ScoringSystem::{IJS, SixO};
use crate::{results_sorter};
use crate::settings::Settings;

pub enum State {
    Ok,
    Error,
}

#[derive(Debug, Clone)]
pub struct Event {
    pub(crate) event_name: String,
    pub(crate) file_path: String,
    pub(crate) scoring_system: ScoringSystem,
    pub(crate) active: bool,
}

impl Event {
    fn new(event_name: String, file_path: String, scoring_system: ScoringSystem, active: bool) -> Self {
        Self {
            event_name,
            file_path,
            scoring_system,
            active,
        }
    }
}

pub fn retrieve_events(path: String) -> (Vec<Event>, String, State) {
    let dir = match fs::read_dir(path.clone()) {
        Ok(e) => e,
        Err(err) => panic!("{} ({})", err, path),
    };

    let files = dir.map(|f| {
        String::from(f.unwrap().file_name().to_str().unwrap())
    }).collect::<Vec<String>>();

    let files_60 = Arc::new(RwLock::new(vec![]));
    let files_ijs = Arc::new(RwLock::new(vec![]));

    //Get all files for 6.0 and IJS separately.
    for file in files {
        //Get all 6.0 results files. These files seem to have names ending in c1.htm.
        if file.ends_with("c1.htm") {
            files_60.write().unwrap().push(String::from(path.clone() + "/" + file.as_str()));
            continue;
        }

        //Reading from the protocol sheets seems to be the easiest way to do this locally.
        //The protocol sheets seem to be contained in files that start with SEGM
        if file.starts_with("SEGM") {
            files_ijs.write().unwrap().push(String::from(path.clone() + "/" + file.as_str()));
            continue;
        }
    }

    let files_ijs_clones = files_ijs.clone();
    let files_ijs_thread = thread::spawn(move || {
        files_ijs_clones.write().unwrap().sort();
    });

    let files_60_clone = files_60.clone();
    let files_60_thread = thread::spawn(move || {
        files_60_clone.write().unwrap().sort();
    });

    files_ijs_thread.join().unwrap();
    files_60_thread.join().unwrap();

    let files_ijs_clones = files_ijs.clone();
    let (events_ijs_sender, events_ijs_receiver) = mpsc::channel::<Vec<Event>>();
    thread::spawn(move || {
        let event_names_ijs = parse_ijs_event_names(&files_ijs_clones.read().unwrap());
        events_ijs_sender.send(event_names_ijs).unwrap();
    });

    let files_60_clones = files_60.clone();
    let (events_60_sender, events_60_receiver) = mpsc::channel::<Vec<Event>>();
    thread::spawn(move || {
        let event_names_60 = parse_60_event_names(&files_60_clones.read().unwrap());
        events_60_sender.send(event_names_60).unwrap();
    });

    let mut event_names = events_ijs_receiver.recv().unwrap();
    event_names.extend(events_60_receiver.recv().unwrap());

    let event_names_clone = event_names.clone();
    let mut event_names_temp = event_names_clone.iter().map(|v| {
        v.event_name.as_str()
    }).collect::<Vec<&str>>();

    human_sort::sort(&mut event_names_temp);

    for (i, event_name) in event_names_temp.iter().enumerate() {
        event_names[i].event_name = event_name.to_string();
    }

    event_names = clean_event_names(event_names);

    if event_names.len() == 0 {
        return (event_names, "The specified competition exists, but there are no results at this time.".to_string(), State::Error);
    }

    (event_names, format!("Found {} IJS events and {} 6.0 events.", files_ijs.read().unwrap().len(), files_60.read().unwrap().len()), State::Ok)
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
    thread::spawn(move || {
        results_ijs_sender.send(parse_ijs(files_ijs_clone.to_vec())).unwrap();
    });

    let (results_60_sender, results_60_receiver) = mpsc::channel::<Vec<ResultSet>>();

    let files_60_clone = files_60.clone();
    thread::spawn(move || {
        results_60_sender.send(parse_60(files_60_clone.to_vec())).unwrap();
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

const HTML_CHARACTER_ENTITIES: [(&'static str, &'static str); 12] = [
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

fn clean_event_names(mut event_names: Vec<Event>) -> Vec<Event> {
    for event_name in &mut event_names {
        for character_entities in HTML_CHARACTER_ENTITIES {
            let temp = event_name.event_name.replace(character_entities.0, character_entities.1).clone();
            event_name.event_name = temp;
        }
    }

    event_names
}

fn clean_club_names(result_sets: &mut Vec<ResultSet>) {
    for result_set in result_sets {
        let name = match &result_set.club {
            Some(name) => { name }
            None => { continue }
        };

        let mut temp = name.clone();
        for character_entities in HTML_CHARACTER_ENTITIES {
            temp = temp.replace(character_entities.0, character_entities.1);
        }

        result_set.club = Some(temp);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScoringSystem {
    IJS,
    SixO,
}

pub struct ResultSet {
    rank: Option<u64>,
    club: Option<String>,
    scoring_system: ScoringSystem,
}

impl ResultSet {
    fn new(scoring_system: ScoringSystem) -> Self {
        Self {
            rank: None,
            club: None,
            scoring_system,
        }
    }
}

fn parse_ijs_event_names(ijs_events: &Vec<String>) -> Vec<Event> {
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

fn parse_60_event_names(ijs_events: &Vec<String>) -> Vec<Event> {
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

pub fn parse_ijs(ijs_events: Vec<String>) -> Vec<ResultSet> {
    let mut results = vec![];

    for results_file_path in ijs_events {
        let results_file_contents = fs::read_to_string(results_file_path).unwrap();
        let document = Html::parse_document(&results_file_contents);
        let selector = Selector::parse(r#"table > tbody > tr > td"#).unwrap();

        let document_select = document.select(&selector);
        let document_select_collection = document_select.collect::<Vec<ElementRef>>();
        for element in document_select_collection.clone().into_iter().enumerate() {
            let mut has_rank = false;
            let mut result_set = ResultSet::new(IJS);
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

pub fn parse_60(files_60: Vec<String>) -> Vec<ResultSet> {
    let mut results = vec![];
    for results_file_path in files_60 {
        let results_file_contents = fs::read_to_string(results_file_path.clone()).unwrap();
        let document = Html::parse_document(&results_file_contents);
        let selector = Selector::parse(r#"table > tbody > tr > td"#).unwrap();

        let document_select = document.select(&selector);
        let document_select_collection = document_select.collect::<Vec<ElementRef>>();
        for element in document_select_collection.clone().into_iter().enumerate() {
            let mut has_name = false;
            let mut result_set = ResultSet::new(SixO);

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

#[derive(Clone, Debug)]
pub struct ClubPoints {
    club: String,
    points_ijs: f64,
    points_60: f64,
}

impl ClubPoints {
    pub fn new(club: String) -> Self {
        Self {
            club,
            points_ijs: 0.0,
            points_60: 0.0,
        }
    }

    pub fn club(&self) -> &String {
        &self.club
    }

    pub fn points_ijs(&self) -> f64 {
        self.points_ijs
    }

    pub fn points_60(&self) -> f64 {
        self.points_60
    }

    pub fn set_club(&mut self, club: String) {
        self.club = club;
    }

    pub fn set_points_ijs(&mut self, points_ijs: f64) {
        self.points_ijs = points_ijs;
    }

    pub fn set_points_60(&mut self, points_60: f64) {
        self.points_60 = points_60;
    }

    pub fn calc_total(&self) -> f64 {
        self.points_ijs + self.points_60
    }
}

pub fn sum_results(results_sets: Vec<ResultSet>, settings: Settings) -> Vec<ClubPoints> {
    let mut club_points_vec: Vec<ClubPoints> = vec![];

    for results_set in &results_sets {
        let club = results_set.club.clone().unwrap();
        let mut club_exists = false;
        for club_points in &club_points_vec {
            if club_points.club.eq(&results_set.club.clone().unwrap()) {
                club_exists = true;
                break;
            }
        }

        if !club_exists {
            club_points_vec.push(ClubPoints::new(club));
        }
    }

    for results_set in &results_sets {
        for club in &mut club_points_vec {
            if results_set.club.clone().unwrap().eq(&club.club) {
                if results_set.rank.clone().unwrap() <= settings.points_for_each_placement.len() as u64 {
                    match results_set.scoring_system {
                        IJS => { club.points_ijs += settings.points_for_each_placement[(results_set.rank.clone().unwrap() - 1) as usize] }
                        SixO => { club.points_60 += settings.points_for_each_placement[(results_set.rank.clone().unwrap() - 1) as usize] }
                    }
                }

                continue;
            }
        }
    }

    club_points_vec
}

#[allow(unused)]
pub fn auto_club_combiner(mut club_points: Vec<ClubPoints>) -> Vec<ClubPoints> {
    let mut broken_60_club_names = vec![];
    let mut indices_to_remove = vec![];

    let mut i = 0;
    for mut club in club_points.clone() {
        if club.club.ends_with("...") { // 6.0 results html files used a clipped club name if they are longer than 21 characters with "..." at the end.
            let corrected_club_name = club.club.split("...").nth(0);
            club.club = corrected_club_name.unwrap().to_string();
            broken_60_club_names.push(club);
            indices_to_remove.push(i);
        }

        i += 1;
    }

    indices_to_remove.reverse();

    let mut ret_club_points = vec![];

    'outer: for (i, club) in club_points.into_iter().enumerate() {
        for r in &indices_to_remove {
            if i == r.clone() {
                continue 'outer;
            }
        }

        ret_club_points.push(club.clone());
    }

    let mut unicorn_clubs = vec![];

    'outer: for broken_club in &broken_60_club_names {
        for mut club in &mut ret_club_points {
            let mut limited_club = club.club.clone();
            if club.club.len() > 21 {
                let mut limited_club = limited_club.as_bytes();
                let mut limited_club = limited_club[0..21].to_vec();
                let mut limited_club = String::from_utf8(limited_club).unwrap();
            }

            if limited_club.starts_with(&broken_club.club.clone()) {
                club.points_ijs += broken_club.points_ijs;
                club.points_60 += broken_club.points_60;
                continue 'outer;
            }
        }

        let mut broken_club_copy = broken_club.clone();
        broken_club_copy.club = broken_club_copy.club + "...";
        unicorn_clubs.push(broken_club_copy);
    }

    ret_club_points.extend(unicorn_clubs);
    ret_club_points
}
