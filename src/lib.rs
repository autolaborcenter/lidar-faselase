use driver::SupervisorForSingle;
use serial_port::{Port, SerialPort};
use std::time::Duration;

pub mod d10;
pub mod zip;

use d10::D10;

pub struct D10Supervisor(Box<Option<D10>>);

impl D10Supervisor {
    pub fn new() -> Self {
        Self(Box::new(None))
    }
}

impl SupervisorForSingle<String, D10> for D10Supervisor {
    fn context<'a>(&'a mut self) -> &'a mut Box<Option<D10>> {
        &mut self.0
    }

    fn open_timeout() -> Duration {
        const TIMEOUT: Duration = Duration::from_secs(3);
        TIMEOUT
    }

    fn keys() -> Vec<String> {
        Port::list()
            .into_iter()
            .map(|name| {
                if cfg!(target_os = "windows") {
                    name.rmatch_indices("COM")
                        .next()
                        .map(|m| &name.as_str()[m.0..name.len() - 1])
                        .unwrap()
                        .into()
                } else {
                    name
                }
            })
            .collect()
    }
}
