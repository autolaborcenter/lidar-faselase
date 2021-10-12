use driver::{SupervisorEventForMultiple::*, SupervisorForMultiple};
use lidar_faselase::{FrameCollector, D10};
use serial_port::PortKey;
use std::{thread::sleep, time::Duration};

fn main() {
    let mut indexer = Indexer::new();
    let mut collectors = [FrameCollector::new(), FrameCollector::new()];
    SupervisorForMultiple::<D10>::new().join(2, |e| {
        match e {
            Connected(k, _) => {
                indexer.add(k.clone());
                collectors.iter_mut().for_each(|c| c.clear());
                println!("connected: COM{}", k);
            }
            ConnectFailed { current, target } => {
                println!("{}/{}", current, target);
                sleep(Duration::from_secs(1));
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

struct Indexer(Vec<PortKey>);

impl Indexer {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn add(&mut self, k: PortKey) {
        match self.0.len() {
            0 => self.0.push(k),
            1 => {
                if k < self.0[0] {
                    self.0.push(k);
                    self.0.swap(0, 1);
                } else {
                    self.0.push(k);
                }
            }
            2 => {
                if k < self.0[0] {
                    self.0.swap(0, 1);
                    self.0[0] = k;
                } else if k < self.0[1] {
                    self.0[1] = k;
                }
            }
            _ => {}
        }
    }

    fn remove(&mut self, k: PortKey) {
        for i in 0..self.0.len() {
            if k == self.0[i] {
                self.0.remove(i);
                return;
            }
        }
    }

    fn find(&self, k: &PortKey) -> Option<usize> {
        for i in 0..self.0.len() {
            if *k == self.0[i] {
                return Some(i);
            }
        }
        None
    }
}
