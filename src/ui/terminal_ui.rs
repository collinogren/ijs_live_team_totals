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

use std::path::Path;
use crate::settings::Settings;

#[deprecated]
#[allow(unused)]
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