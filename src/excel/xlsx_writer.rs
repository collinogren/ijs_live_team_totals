use rust_xlsxwriter;
use rust_xlsxwriter::{Format, FormatAlign, Formula, Workbook};
use crate::parser::ClubPoints;

pub fn create_xlsx(club_points: &Vec<ClubPoints>) {

    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.set_print_gridlines(true);
    worksheet.set_column_width(0, 15).expect("Could not set column 0 to specified width");
    worksheet.set_column_width(1, 100).expect("Could not set column 1 to specified width");
    worksheet.set_column_width(2, 11).expect("Could not set column 2 to specified width");
    worksheet.set_column_width(3, 11).expect("Could not set column 3 to specified width");
    worksheet.set_column_width(4, 15).expect("Could not set column 4 to specified width");

    let text_format = Format::new().set_font_size(32).set_align(FormatAlign::Center);

    worksheet.write_with_format(0, 0, "Rank", &text_format).expect("Failed to write \"Rank\" to worksheet at (0, 0)");
    worksheet.write_with_format(0, 1, "Club", &text_format).expect("Failed to write \"Club\" to worksheet at (0, 1)");
    worksheet.write_with_format(0, 2, "IJS", &text_format).expect("Failed to write \"IJS\" to worksheet at (0, 2)");
    worksheet.write_with_format(0, 3, "6.0", &text_format).expect("Failed to write \"6.0\" to worksheet at (0, 3)");
    worksheet.write_with_format(0, 4, "Total", &text_format).expect("Failed to write \"Total\" to worksheet at (0, 4)");


    for (i, result) in club_points.iter().enumerate() {
        worksheet.write_with_format(i as u32 + 1, 0, i as u32 + 1, &text_format).expect("Failed to write the rank");
        worksheet.write_with_format(i as u32 + 1, 1, &result.club, &text_format).expect(format!("Failed to write club name for {}", result.club).as_str());
        worksheet.write_with_format(i as u32 + 1, 2, result.points_ijs, &text_format).expect(format!("Failed to write IJS score for {}", result.club).as_str());
        worksheet.write_with_format(i as u32 + 1, 3, result.points_60, &text_format).expect(format!("Failed to write 6.0 score for {}", result.club).as_str());
        worksheet.write_with_format(i as u32 + 1, 4, Formula::new(format!("=SUM(C{}:D{})", i as u32 + 2, i as u32 + 2).as_str()), &text_format).expect(format!("Failed to write total for {}", result.club).as_str());
    }

    workbook.save("team_totals.xlsx").expect("Failed to write xlsx file");
}