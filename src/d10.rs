use driver::Driver;
use serial_port::{Port, PortKey, SerialPort};
use std::time::{Duration, Instant};

pub(super) mod point;
mod sections;

use point::{Point, PortBuffer};

use self::sections::Sections;

const POINT_RECEIVE_TIMEOUT: Duration = Duration::from_millis(200);
const POINT_PARSE_TIMEOUT: Duration = Duration::from_millis(250);
const OPEN_TIMEOUT: Duration = Duration::from_secs(3);

pub struct D10 {
    port: Port,
    buffer: PortBuffer<64>,
    last_time: Instant,
    sections: Sections,
    filter: fn(Point) -> bool,
}

impl Driver for D10 {
    type Key = PortKey;
    type Pacemaker = ();
    type Event = Vec<Point>;
    type Command = fn(Point) -> bool;

    fn keys() -> Vec<Self::Key> {
        Port::list().into_iter().map(|id| id.key).collect()
    }

    fn open_timeout() -> Duration {
        OPEN_TIMEOUT
    }

    fn new(key: &Self::Key) -> Option<(Self::Pacemaker, Self)> {
        match Port::open(key, 460800, POINT_RECEIVE_TIMEOUT.as_millis() as u32) {
            Ok(port) => Some((
                (),
                D10 {
                    port,
                    buffer: Default::default(),
                    last_time: Instant::now(),
                    sections: Sections::new(8),
                    filter: |_| true,
                },
            )),
            Err(_) => None,
        }
    }

    fn send(&mut self, f: Self::Command) {
        self.filter = f;
    }

    fn join<F>(&mut self, mut f: F) -> bool
    where
        F: FnMut(&mut Self, Option<(std::time::Instant, Self::Event)>) -> bool,
    {
        let mut time = Instant::now();
        loop {
            if let Some(mut p) = self.buffer.next() {
                time = self.last_time;
                // dir>=5760 的不是采样数据，不知道有什么用
                if p.dir < 5760 {
                    // 过滤
                    if p.len != 0 && !(self.filter)(p) {
                        p.len = 0;
                    }
                    if let Some(section) = self.sections.push(p) {
                        if !f(self, Some((time, section))) {
                            // 如果回调指示不要继续阻塞，立即退出
                            return true;
                        }
                    }
                }
            } else if self.last_time > time + POINT_PARSE_TIMEOUT {
                // 解析超时
                return false;
            } else {
                // 重新接收
                match self.port.read(self.buffer.as_buf()) {
                    // 成功接收
                    Some(n) => {
                        if n == 0 {
                            // 串口超时
                            return false;
                        } else {
                            // 成功接收
                            self.last_time = Instant::now();
                            self.buffer.notify_received(n);
                        }
                    }
                    // 无法接收
                    None => return false,
                };
            }
        }
    }
}
