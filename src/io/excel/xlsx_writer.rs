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

use std::fs;
use std::path::Path;
use rust_xlsxwriter;
use rust_xlsxwriter::{ColNum, Format, FormatAlign, Formula, Workbook};
use crate::file_utils;
use crate::parser::ClubPoints;
use crate::settings::Settings;

pub fn create_xlsx(club_points: &Vec<ClubPoints>, settings: Settings) {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.set_print_gridlines(true);
    for (column, width) in settings.xlsx_column_widths.iter().enumerate() {
        if *width > 0 { // Use negative value to not set the width.
            worksheet
                .set_column_width(column as ColNum, *width)
                .expect(
                    format!(
                        "Could not set column {} to specified width of {}", column, width
                    ).as_str());
        }
    }

    let text_format = Format::new().set_font_size(settings.xlsx_font_size).set_align(FormatAlign::Center);

    for (column, value) in settings.xlsx_header_cell_values.iter().enumerate() {
        worksheet
            .write_with_format(0, column as ColNum, value.as_str(), &text_format)
            .expect(format!("Failed to write \"{}\" to worksheet at (0, {})", value, column)
                .as_str());
    }

    for (i, result) in club_points.iter().enumerate() {
        worksheet.write_with_format(i as u32 + 1, 0, i as u32 + 1, &text_format).expect("Failed to write the rank");
        worksheet.write_with_format(i as u32 + 1, 1, result.club(), &text_format).expect(format!("Failed to write club name for {}", result.club()).as_str());
        worksheet.write_with_format(i as u32 + 1, 2, result.points_ijs(), &text_format).expect(format!("Failed to write IJS score for {}", result.club()).as_str());
        worksheet.write_with_format(i as u32 + 1, 3, result.points_60(), &text_format).expect(format!("Failed to write 6.0 score for {}", result.club()).as_str());
        worksheet.write_with_format(i as u32 + 1, 4, Formula::new(format!("=SUM(C{}:D{})", i as u32 + 2, i as u32 + 2).as_str()), &text_format).expect(format!("Failed to write total for {}", result.club()).as_str());
    }

    file_utils::check_and_create_dir(&settings.output_directory);

    let path = settings.xlsx_path();

    for i in 0..i32::MAX {
        let modified_path = if i != 0 {
            path.clone().replace(".xlsx", format!("({}).xlsx", i).as_str())
        } else {
            path.clone()
        };

        if Path::new(modified_path.as_str()).exists() {
            continue;
        }

        match workbook.save(&modified_path) {
            Ok(_) => {
                break;
            }
            Err(_) => {
                continue;
            }
        }
    }
}