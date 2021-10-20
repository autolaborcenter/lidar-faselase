use driver::{Indexer, SupervisorEventForMultiple::*, SupervisorForMultiple};
use lidar_faselase::D10;
use std::time::{Duration, Instant};

fn main() {
    let mut indexer = Indexer::new(2);
    SupervisorForMultiple::<D10>::new().join(2, |e| {
        match e {
            Connected(k, _) => {
                indexer.add(k.clone());
                println!("connected: COM{}", k);
            }
            ConnectFailed {
                current,
                target,
                next_try,
            } => {
                println!("{}/{}", current, target);
                *next_try = Instant::now() + Duration::from_secs(1);
            }
            Event(k, Some((_, (_, _))), _) => if let Some(_) = indexer.find(&k) {},
            Event(_, _, _) => {}
            Disconnected(k) => {
                indexer.remove(k);
                println!("disconnected: COM{}", k)
            }
        }
        2
    });
}
