use std::fs;
use std::path::{Path};
use crate::io::excel::xlsx_writer;
use crate::io::html::club_points::ClubPoints;
use crate::io::html::html_writer;
use crate::settings::settings::Settings;

pub fn check_and_create_dir(path: &String) -> bool {
    match Path::new(path).exists() {
        true => {
            println!("\"{}\" exists.", path);
            true
        }
        false => {
            match fs::create_dir_all(&path) {
                Ok(_) => {
                    println!("\"{}\" has been created", path);
                    true
                }
                Err(_) => {
                    eprintln!("Failed to create directory \"{}\"", &path.as_str());
                    false
                }
            }
        }
    }
}

pub fn output_files(club_points: &Vec<ClubPoints>, settings: &Settings, competition_name: &String) {
    if settings.generate_xlsx {
        write_xlsx(club_points, settings);
    }

    if settings.generate_html {
        write_html(club_points, settings, competition_name);
    }
}

fn write_xlsx(results: &Vec<ClubPoints>, settings: &Settings) {
    xlsx_writer::create_xlsx(&results, settings.clone());
}

fn write_html(results: &Vec<ClubPoints>, settings: &Settings, competition_name: &String) {
    html_writer::create_html(&results, settings.clone(), competition_name);
}