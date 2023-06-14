use std::fs;

use scraper::{CaseSensitivity, Element, ElementRef, Html, Selector};
use scraper::selector::CssLocalName;
use stringmetrics::jaccard;
use crate::parser::ScoringSystem::{IJS, SixO};
use crate::settings::Settings;

pub fn parse_results(path: String, settings: Settings) {
    let dir = match fs::read_dir(path.clone()) {
        Ok(e) => e,
        Err(err) => panic!("{}", err),
    };

    let files = dir.map(|f| {
        String::from(f.unwrap().file_name().to_str().unwrap())
    }).collect::<Vec<String>>();

    let mut files_60 = vec![];
    let mut files_ijs = vec![];

    println!("Finding results...");

    //Get all files for 6.0 and IJS separately.
    for file in files {
        //Get all 6.0 results files. These files seem to have names ending in c1.htm.
        if file.ends_with("c1.htm") {
            files_60.push(String::from(path.clone() + "/" + file.as_str()));
            continue;
        }

        //Reading from the protocol sheets seems to be the easiest way to do this locally.
        //The protocol sheets seem to be contained in files that start with SEGM
        if file.starts_with("SEGM") {
            files_ijs.push(String::from(path.clone() + "/" + file.as_str()));
            continue
        }
    }

    files_60.sort();
    files_ijs.sort();

    println!("Found results for {} events", files_60.len() + files_ijs.len());

    let mut results_ijs = parser_ijs(files_ijs, settings.clone());
    let results_60 = parser_60(files_60, settings.clone());
    println!("Retrieved {} IJS results and {} 6.0 results.", results_ijs.len(), results_60.len());
    results_ijs.extend(results_60);

    let results = sum_results(results_ijs, settings.clone());

    let mut team_totals = vec![];
    for result in results {
        team_totals.push(format!("{}: IJS: {}, 6.0: {}, Total: {}", result.club, result.points_ijs, result.points_60, result.calc_total()));
    }

    fs::write("./team_totals.txt", team_totals.join("\n")).unwrap();
}

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
            scoring_system
        }
    }
}

pub fn parser_ijs(ijs_events: Vec<String>, _settings: Settings) -> Vec<ResultSet> {
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
                    .clone()
                    .split("</td>").nth(0)
                    .unwrap()
                    .clone()
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
                        .clone()
                        .split("</td>")
                        .nth(0)
                        .unwrap()
                        .clone()
                        .split(", ")
                        .nth(1)
                        .unwrap()
                        .clone()
                    )
                );
            }

            results.push(result_set);
        }
    }

    results
}

pub fn parser_60(files_60: Vec<String>, _settings: Settings) -> Vec<ResultSet> {
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
                let temp_club =                     String::from(element.1.html()
                    .split("<td rowspan=\"1\" colspan=\"1\">")
                    .nth(1)
                    .unwrap()
                    .clone()
                    .split("</td>").nth(0)
                    .unwrap()
                    .clone()
                    .split(", ")
                    .nth(1)
                    .unwrap()
                    .clone()
                );

                if temp_club.contains("<br>") { // Handle duets and such by disregarding them. This is a terrible solution.
                    continue;
                }

                result_set.club = Some(temp_club);

                has_name = true;
            } else if element.1.html().contains("<td colspan=\"1\" rowspan=\"1\">") { // Name first this time because the name has more distinctive markings for it in the HTML.
                let temp_club =                     String::from(element.1.html()
                    .split("<td colspan=\"1\" rowspan=\"1\">")
                    .nth(1)
                    .unwrap()
                    .clone()
                    .split("</td>").nth(0)
                    .unwrap()
                    .clone()
                    .split(", ")
                    .nth(1)
                    .unwrap()
                    .clone()
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
                .clone()
                .split(".</td>")
                .nth(0)
                .unwrap()
                .clone()
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

#[derive(Clone)]
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
                break
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

                continue
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

    for mut club in &mut ret_club_points {
        for broken_club in &broken_60_club_names {
            let mut limited_club = club.club.clone();
            if club.club.len() > 21 {
                let mut limited_club = limited_club.as_bytes();
                let mut limited_club = limited_club[0..21].to_vec();
                let mut limited_club = String::from_utf8(limited_club).unwrap();
            }
            let similarity = jaccard(limited_club.chars(), broken_club.club.clone().chars());

            if similarity >= 0.66 {
                club.points_ijs += broken_club.points_ijs;
                club.points_60 += broken_club.points_60;
            }
        }
    }

    ret_club_points
}