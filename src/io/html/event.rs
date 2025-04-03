use std::{fs, thread};
use std::sync::{Arc, mpsc, RwLock};
use crate::io::html::parser::State;
use crate::io::html::scoring_system::ScoringSystem;

#[derive(Debug, Clone)]
pub struct Event {
    pub(crate) event_name: String,
    pub(crate) file_path: String,
    pub(crate) scoring_system: ScoringSystem,
    pub(crate) active: bool,
}

impl Event {
    pub fn new(event_name: String, file_path: String, scoring_system: ScoringSystem, active: bool) -> Self {
        Self {
            event_name,
            file_path,
            scoring_system,
            active,
        }
    }
}

pub fn retrieve_events(path: String) -> (Vec<Event>, String, State) {
    let dir = match fs::read_dir(path.clone()) {
        Ok(e) => e,
        Err(err) => panic!("{} ({})", err, path),
    };

    let files = dir.map(|f| {
        String::from(f.unwrap().file_name().to_str().unwrap())
    }).collect::<Vec<String>>();

    let files_60 = Arc::new(RwLock::new(vec![]));
    let files_ijs = Arc::new(RwLock::new(vec![]));

    //Get all files for 6.0 and IJS separately.
    for file in files {
        //Get all 6.0 results files. These files seem to have names ending in c1.htm.
        if file.ends_with("c1.htm") {
            files_60.write().unwrap().push(String::from(path.clone() + "/" + file.as_str()));
            continue;
        }

        //Reading from the protocol sheets seems to be the easiest way to do this locally.
        //The protocol sheets seem to be contained in files that start with SEGM
        if file.starts_with("SEGM") {
            files_ijs.write().unwrap().push(String::from(path.clone() + "/" + file.as_str()));
            continue;
        }
    }

    let files_ijs_clones = files_ijs.clone();
    let files_ijs_thread = thread::spawn(move || {
        files_ijs_clones.write().unwrap().sort();
    });

    let files_60_clone = files_60.clone();
    let files_60_thread = thread::spawn(move || {
        files_60_clone.write().unwrap().sort();
    });

    files_ijs_thread.join().unwrap();
    files_60_thread.join().unwrap();

    let files_ijs_clones = files_ijs.clone();
    let (events_ijs_sender, events_ijs_receiver) = mpsc::channel::<Vec<Event>>();
    thread::spawn(move || {
        let event_names_ijs = crate::io::html::parser::parse_ijs_event_names(&files_ijs_clones.read().unwrap());
        events_ijs_sender.send(event_names_ijs).unwrap();
    });

    let files_60_clones = files_60.clone();
    let (events_60_sender, events_60_receiver) = mpsc::channel::<Vec<Event>>();
    thread::spawn(move || {
        let event_names_60 = crate::io::html::parser::parse_60_event_names(&files_60_clones.read().unwrap());
        events_60_sender.send(event_names_60).unwrap();
    });

    let mut event_names = events_ijs_receiver.recv().unwrap();
    event_names.extend(events_60_receiver.recv().unwrap());

    let event_names_clone = event_names.clone();
    let mut event_names_temp = event_names_clone.iter().map(|v| {
        v.event_name.as_str()
    }).collect::<Vec<&str>>();

    human_sort::sort(&mut event_names_temp);

    for (i, event_name) in event_names_temp.iter().enumerate() {
        event_names[i].event_name = event_name.to_string();
    }

    event_names = crate::io::html::parser::clean_event_names(event_names);

    if event_names.len() == 0 {
        return (event_names, "The specified competition exists, but there are no results at this time.".to_string(), State::Error);
    }

    (event_names, format!("Found {} IJS events and {} 6.0 events.", files_ijs.read().unwrap().len(), files_60.read().unwrap().len()), State::Ok)
}