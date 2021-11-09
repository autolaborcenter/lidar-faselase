use serial_port::{Port, PortKey, SerialPort};
use std::time::Duration;

mod port_buffer;
mod zip;

use port_buffer::PortBuffer;

pub use lidar::driver;
pub use lidar::{Lidar, LidarDriver, Point};
pub use zip::PointZipped;

const POINT_RECEIVE_TIMEOUT: Duration = Duration::from_millis(200);
const OPEN_TIMEOUT: Duration = Duration::from_secs(1);
const PARSE_TIMEOUT: Duration = Duration::from_millis(250);

pub struct D10 {
    port: Port,
    buffer: PortBuffer<64>,
}

impl LidarDriver for D10 {
    type Key = PortKey;

    fn keys() -> Vec<Self::Key> {
        Port::list().into_iter().map(|id| id.key).collect()
    }

    fn open_timeout() -> Duration {
        OPEN_TIMEOUT
    }

    fn parse_timeout() -> Duration {
        PARSE_TIMEOUT
    }

    fn max_dir() -> u16 {
        5760
    }

    fn new(key: &Self::Key) -> Option<Self> {
        Port::open(key, 460800, POINT_RECEIVE_TIMEOUT.as_millis() as u32)
            .ok()
            .map(|port| Self {
                port,
                buffer: Default::default(),
            })
    }

    fn receive(&mut self) -> bool {
        self.port
            .read(self.buffer.as_buf())
            .filter(|n| *n > 0)
            .map(|n| self.buffer.notify_received(n))
            .is_some()
    }

    fn parse(&mut self) -> Option<lidar::Point> {
        self.buffer.next()
    }
}
