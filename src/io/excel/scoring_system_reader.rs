use calamine::{deserialize_as_f64_or_none, Data, DeError, RangeDeserializer};
use calamine::deserialize_as_i64_or_none;
use std::collections::HashMap;
use calamine::{open_workbook, RangeDeserializerBuilder, Reader, Xlsx};
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct Record {
    #[serde(deserialize_with = "deserialize_as_i64_or_none")]
    number_of_placements: Option<i64>,
    #[serde(deserialize_with = "deserialize_as_f64_or_none")]
    points: Option<f64>,
}

pub fn deserialize(path: String) -> HashMap<i64, Vec<f64>> {
    let mut workbook: Xlsx<_> = open_workbook(path).unwrap();
    let range = workbook.worksheet_range(workbook.sheet_names().get(0).unwrap()).unwrap();

    let iter: RangeDeserializer<'_, Data, Record> = RangeDeserializerBuilder::new().from_range(&range).unwrap();

    let mut scoring_system_map: HashMap<i64, Vec<f64>> = HashMap::new();

    iter.for_each(|result| {
        match result {
            Ok(record) => {
                match record.number_of_placements {
                    Some(number_of_placements) => {
                        if scoring_system_map.contains_key(&number_of_placements) {
                            scoring_system_map.get_mut(&number_of_placements).unwrap().push(record.points.unwrap());
                        } else {
                            scoring_system_map.insert(number_of_placements.clone(), vec![record.points.unwrap()]);
                        }
                    }
                    None => {}
                }
            }
            Err(err) => {
                eprintln!("{:?}", err);
            }
        }
    });

    println!("{:?}", scoring_system_map);

    scoring_system_map
}