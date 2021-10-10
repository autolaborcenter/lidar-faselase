use driver::{SupersivorEventForSingle::*, SupervisorForSingle};
use lidar_faselase::D10Supervisor;
use std::{thread, time::Duration};

fn main() {
    D10Supervisor::new().join(|e| {
        match e {
            Connected(_) => println!("Connected."),
            ConnectFailed => {
                println!("Failed.");
                thread::sleep(Duration::from_secs(1));
            }
            Disconnected => {
                println!("Disconnected.");
                thread::sleep(Duration::from_secs(1));
            }
            Event(_, Some((_, event))) => println!("Event: {:?}", event),
            Event(_, None) => {}
        };
        true
    });
}
