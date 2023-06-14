use std::path::Path;
use crate::settings::Settings;

pub fn main_menu(settings: &Settings) -> String {
    if settings.use_event_name_for_results_path {
        println!("Enter the name of the competition you wish to talley");

        let path = loop {
            let mut name = String::new();
            match std::io::stdin().read_line(&mut name) {
                Ok(_) => {},
                Err(error) => {
                    println!("Could not read input: {}\n", error);
                    continue;
                }
            };

            println!("{}", name);

            let path_string = format!("{}{}", settings.isu_calc_base_directory, name).replace("\r", "").replace("\n", "");
            let path = Path::new(path_string.as_str());

            println!("{}", path_string);

            if path.is_dir() {
                break path_string;
            } else {
                println!("Could not find a competition with name \"{}\", check the competition's name and try again.", name);
                continue
            }
        };

        path
    } else {
        println!("Enter the absolute path of the competition you wish to talley");

        let path = loop {
            let mut path = String::new();
            match std::io::stdin().read_line(&mut path) {
                Ok(_) => {},
                Err(error) => {
                    println!("Could not read input: {}\n", error);
                    continue;
                }
            };

            let path_string = String::from(path).replace("\r", "").replace("\n", "");
            let path = Path::new(path_string.as_str());

            if path.is_dir() {
                break path_string;
            } else {
                println!("No such file or directory: \"{}\", check the competition's path and try again.", path_string);
                continue
            }
        };

        path
    }
}