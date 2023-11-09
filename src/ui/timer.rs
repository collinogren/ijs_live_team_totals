use std::cell::RefCell;
use std::sync::{Arc, mpsc, RwLock};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct TimerData {
    time: u128,
    tick_rate_milliseconds: u64,
    running: Arc<RwLock<bool>>,
}

impl TimerData {
    pub fn new(time: u128, tick_rate_milliseconds: u64, running: bool) -> Self {
        Self {
            time,
            tick_rate_milliseconds,
            running: Arc::new(RwLock::new(running)),
        }
    }
}

pub struct Timer {
    started: bool,
    timer_data: TimerData,
    pub(crate) receiver: RefCell<Option<Receiver<TimerData>>>,
    sender: Sender<TimerData>,
}

impl Timer {
    pub fn new(duration: u128, tick_rate_milliseconds: u64) -> Self {
        let (sender, receiver) = mpsc::channel::<TimerData>();
        Self {
            started: false,
            timer_data: TimerData::new(duration, tick_rate_milliseconds, false),
            receiver: RefCell::new(Some(receiver)),
            sender,
        }
    }

    pub fn start(&mut self) {
        if self.started {
            return;
        }

        self.started = true;
        self.timer_data.running = Arc::new(RwLock::new(true));

        let timer_data = self.timer_data.clone();
        let sender = self.sender.clone();
        let sleep = Duration::from_millis(self.timer_data.tick_rate_milliseconds);
        thread::spawn(move || {
            let start = Instant::now();
            let time = timer_data.time;

            let running = timer_data.running.clone();
            while running.read().unwrap().clone() {
                if start.elapsed().as_millis() >= time {
                    let mut running_lock = running.write().unwrap();
                    *running_lock = false;
                    sender.send(timer_data).unwrap();
                    break;
                }

                thread::sleep(sleep);
            }
        });
    }

    pub fn stop(&mut self) {
        let mut running_lock = self.timer_data.running.write().unwrap();
        *running_lock = false;
    }
}