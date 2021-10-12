use driver::{SupervisorEventForMultiple::*, SupervisorForMultiple};
use lidar_faselase::*;
use std::{thread::sleep, time::Duration};

fn main() {
    SupervisorForMultiple::<d10::D10>::new().join(2, |e| {
        match e {
            Connected(k, _) => println!("connected: COM{}", k),
            ConnectFailed { current, target } => {
                println!("{}/{}", current, target);
                sleep(Duration::from_secs(1));
            }
            Event(_, _, _) => {}
            Disconnected(k) => println!("disconnected: COM{}", k),
        }
        2
    });
}
