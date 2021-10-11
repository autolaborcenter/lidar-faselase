use driver::{SupervisorEventForMultiple::*, SupervisorForMultiple};
use lidar_faselase::*;
use std::{thread::sleep, time::Duration};

fn main() {
    SupervisorForMultiple::<d10::D10>::new().join(2, |e| {
        match e {
            Connected(name, _) => println!("connected: {}", name),
            ConnectFailed { current, target } => {
                println!("{}/{}", current, target);
                sleep(Duration::from_secs(1));
            }
            Event(_, _) => {}
            Disconnected(name) => println!("disconnected: {}", name),
        }
        2
    });
}
