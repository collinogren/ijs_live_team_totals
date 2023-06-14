# ijs_live_team_totals
This software is an independent addon for U.S. Figure Skating and ISU accounting software. The purpose of this software is to provide a unified and flexible solution to calculating team points.

# Free and Open Source Software
This software is free and open source which not only allows you, the end user, to request changes from myself, but it also allows you to fix and adjust the program to suit your needs.

# Building the Software
This software is written using the Rust programming language, a modern, fast, cross-platform, safe language.
The Rust compiler and associated software can be found here: https://www.rust-lang.org/tools/install
If you are unfamiliar with Rust and would like to learn more, check out the book *The Rust Programming Language*: https://doc.rust-lang.org/book/

Once the Rust compiler has been installed, download the source and run
`cargo build --release`
to build the release version and
`cargo build`
to build the debug version.

# Using the Software
Once you have built the program, simply run the executable.
A new folder will appear called settings and in it will be a `settings.toml` file in this file you will find by default:
>points_for_each_placement = [3.0, 2.0, 1.0]  
include_60 = true  
include_ijs = true  
attempt_automatic_60_club_name_recombination_inop = false  
use_event_name_for_results_path = true  
isu_calc_base_directory = "C:/ISUCalcFS/"  
html_relative_directory = "/IJScompanion_html_winnercomm"

## Understanding and Editing Settings
This program uses TOML (Tom's Obvious Minimal Language) for its settings file as it is extremely well supported by Rust libraries and is used extensivly within the language's own tools.
More information on TOML can be found here: https://toml.io/en/  

`points_for_each_placement` is an array of points allocated for each finishing position in the competition. The first number being for first place, the second for second place, and so on.  
`include_60` is a boolean value that dictates whether 6.0 results should be included in the calculation or not.  
`include_ijs` is a boolean value that dictates whether IJS results should be included in the calculation or not.  
`attempt_automatic_60_club_name_recombination_inop` is a boolean value that dictates whether the program should combine the (potentially) truncated 6.0 club name with the full length name found on IJS results. This value is currently inoperational.  
`use_event_name_for_results_path` is a boolean value that dictates whether the program should ask for a competition name or a full path to the competition folder.  
`isu_calc_base_directory` is a String value that contains the path to ISUCalcFS. It is a companion value to `use_event_name_for_results_path = true`
`html_relative_directory` is a String value that contains the path to the HTML results files relative to the competition directory.

## Calculating Team Totals
Once the settings have been configured to your needs as required, the program can be run and the prompts followed.
Note that unlike typical Windows software, the forward slash `/` is used instead of the `\` for paths. This gives two main advantages:  
1. `\` is used extensively for things like `\n`, the newline escape sequence on Linux and MacOS or `\r\n`, the same for Windows. Thus, using `/` eliminates complexity.
2. `/` is the standard for non-Windows operating systems and for URLs. The Windows convention should be treated as an abnormality and not as the standard.
3. The standard Rust library feature `std::path::Path` automatically converts `/` into `\` as required by the operating system.

The output will be saved to a file named `team_totals.txt`. Later this should be changed to write .xlsx files instead.

# TODO
This software is a work in progress, the current goals for new features include:
+ Implement a proper grapical user interface
+ Implement a feature to ignore certain results files (for example Compete USA events are often excluded from talleys)
+ Implement support for partnered events for IJS and 6.0
