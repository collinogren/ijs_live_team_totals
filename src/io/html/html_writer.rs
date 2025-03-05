use std::fs;
use crate::file_utils;
use crate::parser::ClubPoints;
use crate::settings::Settings;

pub fn create_html(club_points: &Vec<ClubPoints>, settings: Settings, competition_name: &String) {
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
<title>{competition_name}</title>
<meta name='description' content='Team Points Results For {competition_name}'>
<meta http-equiv="Content-Type" content="text/html; charset=iso-8859-1">
<meta http-equiv="Cache-Control" content="no-cache, no-store, must-revalidate">
<meta http-equiv="Pragma" content="no-cache">
<meta http-equiv="Expires" content="0">
<meta HTTP-EQUIV="REFRESH" CONTENT="30">

<style>
   table, th, td {{
      border: 1px solid black;
      border-collapse: collapse;
    }}
</style>

</head>
<body>
<table style="width:50%">
  <tr>
    <th>#</th>
    <th>Club</th>
    <th>Points</th>
  </tr>
{table_contents}
</table>
</body>"#,
        competition_name=competition_name,
        table_contents=generate_club_points_table(&club_points),
    );

    file_utils::check_and_create_dir(&settings.output_directory);
    fs::write(settings.output_directory + "/team_points.html", html).expect("Failed to write team_points.html");
}

fn generate_club_points_table(club_points: &Vec<ClubPoints>) -> String {
    let mut club_rows: String = String::new();
    for club_points in club_points.into_iter().enumerate() {
        let (placement, club_points) = club_points;
        club_rows.push_str(format!("  <tr>\n    <td>{}</td>\n    <td>{}</td>\n    <td>{}</td>\n  </tr>\n", placement + 1, club_points.club(), club_points.calc_total()).as_str());
    }

    club_rows
}
