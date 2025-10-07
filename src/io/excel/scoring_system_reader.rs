use calamine::{deserialize_as_f64_or_none, Data, DeError, RangeDeserializer, open_workbook_auto, Range, DataType};
use calamine::deserialize_as_i64_or_none;
use std::collections::HashMap;
use calamine::{open_workbook, RangeDeserializerBuilder, Reader, Xlsx};
use serde_derive::Deserialize;

pub fn deserialize(path: Option<String>) -> HashMap<u64, Vec<f64>> {
    if path.is_none() {
        return HashMap::new();
    }

    let path = path.unwrap();

    let mut workbook: Xlsx<_> = open_workbook(path).unwrap();

    let range: Range<Data> = match workbook.worksheet_range("Points Chart") {
        Ok(range) => range,
        Err(_) => panic!("Failed to read \"Points Chart\""),
    };

    let mut columns: HashMap<u64, Vec<f64>> = HashMap::new();

    let mut header_row = true;
    let mut headers: Vec<u64> = Vec::new();

    for row in range.rows() {
        if header_row {
            for cell in row {
                headers.push(cell.as_i64().unwrap() as u64);
                columns.insert(cell.as_i64().unwrap() as u64, Vec::new());
            }

            header_row = false;
        } else {
            for (i, cell) in row.iter().enumerate() {
                if let Some(header) = headers.get(i) {
                    if let Some(column_vec) = columns.get_mut(header) {
                        column_vec.push(match cell.as_f64() {
                            Some(cell_value) => cell_value,
                            None => continue,
                        });
                    }
                }
            }
        }
    }

    columns
}