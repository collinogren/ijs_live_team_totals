use calamine::{Data, Range, DataType};
use std::collections::HashMap;
use calamine::{open_workbook, Reader, Xlsx};

pub struct ScoringSystemReadError;

pub fn read_scoring_system_spreadsheet(path: Option<String>) -> Result<HashMap<u64, Vec<f64>>, String> {
    let path = match path {
        Some(path) => path,
        None => return Err("No scoring system spreadsheet path given".to_string()),
    };

    let mut workbook: Xlsx<_> = match open_workbook(path) {
        Ok(value) => value,
        Err(_) => return Err("Failed to read scoring system spreadsheet. Is it in the right format?".to_string()),
    };

    let range: Range<Data> = match workbook.worksheet_range("Points Chart") {
        Ok(range) => range,
        Err(_) => return Err("Failed to read page \"Points Chart.\" Does it exist?".to_string()),
    };

    let mut columns: HashMap<u64, Vec<f64>> = HashMap::new();

    let mut header_row = true;
    let mut headers: Vec<u64> = Vec::new();

    for row in range.rows() {
        if header_row {
            for cell in row {
                let cell: i64 = match cell.as_i64() {
                    Some(value) => value,
                    None => return Err("One or more column headers are not integers.".to_string()),
                };
                headers.push(cell as u64);
                columns.insert(cell as u64, Vec::new());
            }

            header_row = false;
        } else {
            for (i, cell) in row.iter().enumerate() {
                if let Some(header) = headers.get(i) {
                    if let Some(column_vec) = columns.get_mut(header) {
                        column_vec.push(match cell.as_f64() {
                            Some(cell_value) => cell_value,
                            None => if cell.is_empty() {
                                continue
                            } else {
                                return Err(format!("Cell in column {} with value {} is not a number", header, cell.to_string()));
                            },
                        });
                    }
                }
            }
        }
    }

    Ok(columns)
}