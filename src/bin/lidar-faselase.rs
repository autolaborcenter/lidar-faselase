use driver::{Indexer, SupervisorEventForMultiple::*, SupervisorForMultiple};
use lidar_faselase::{FrameCollector, D10};
use std::time::{Duration, Instant};

fn main() {
    let mut indexer = Indexer::new(2);
    let mut collectors = [FrameCollector::new(), FrameCollector::new()];
    SupervisorForMultiple::<D10>::new().join(2, |e| {
        match e {
            Connected(k, _) => {
                indexer.add(k.clone());
                collectors.iter_mut().for_each(|c| c.clear());
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
            Event(k, Some((_, (i, s))), _) => {
                if let Some(j) = indexer.find(&k) {
                    collectors[j].put(i as usize, s);
                }
            }
            Event(_, _, _) => {}
            Disconnected(k) => {
                indexer.remove(k);
                collectors.iter_mut().for_each(|c| c.clear());
                println!("disconnected: COM{}", k)
            }
        }
        2
    });
}
