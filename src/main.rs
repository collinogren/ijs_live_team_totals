use std::time::Instant;
use crate::settings::Settings;
use crate::terminal_ui::main_menu;

#[path = "html_utilities/parser.rs"]
mod parser;

#[path = "settings/settings.rs"]
mod settings;

#[path = "ui/terminal_ui.rs"]
mod terminal_ui;

fn main() {
    let settings = Settings::read_settings();
    let path = main_menu(&settings);
    let elapsed = Instant::now();
    let directory = String::from(path + &settings.html_relative_directory);

    parser::parse_results(directory, settings);
    println!("Calculation finished in {} milliseconds", elapsed.elapsed().as_millis());

}
