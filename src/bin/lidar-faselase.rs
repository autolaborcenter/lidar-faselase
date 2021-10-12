use driver::{SupervisorEventForMultiple::*, SupervisorForMultiple};
use lidar_faselase::{FrameCollector, D10};
use std::{thread::sleep, time::Duration};

fn main() {
    let mut collectors = [FrameCollector::new(), FrameCollector::new()];
    SupervisorForMultiple::<D10>::new().join(2, |e| {
        match e {
            Connected(k, _) => println!("connected: COM{}", k),
            ConnectFailed { current, target } => {
                println!("{}/{}", current, target);
                sleep(Duration::from_secs(1));
            }
            Event(which, Some((_, (i, s))), _) => {
                collectors[0].put(i as usize, s);
            }
            Event(_, _, _) => {}
            Disconnected(k) => println!("disconnected: COM{}", k),
        }
        2
    });
}
