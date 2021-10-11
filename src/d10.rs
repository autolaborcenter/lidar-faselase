use driver::Driver;
use serial_port::{Port, SerialPort};
use std::time::{Duration, Instant};

pub mod point;

use point::{Point, PortBuffer};

const POINT_RECEIVE_TIMEOUT: Duration = Duration::from_millis(200);
const POINT_PARSE_TIMEOUT: Duration = Duration::from_millis(250);
const OPEN_TIMEOUT: Duration = Duration::from_secs(3);

pub struct D10 {
    port: Port,
    buffer: PortBuffer<64>,
    last_time: Instant,
}

impl Driver for D10 {
    type Key = String;
    type Pacemaker = ();
    type Event = Point;
    type Command = ();

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

    fn open_timeout() -> Duration {
        OPEN_TIMEOUT
    }

    fn new(name: &String) -> Option<(Self::Pacemaker, Self)> {
        match Port::open(
            name.as_str(),
            460800,
            POINT_RECEIVE_TIMEOUT.as_millis() as u32,
        ) {
            Ok(port) => Some((
                (),
                D10 {
                    port,
                    buffer: Default::default(),
                    last_time: Instant::now(),
                },
            )),
            Err(_) => None,
        }
    }

    fn send(&mut self, _: (std::time::Instant, Self::Command)) {}

    fn join<F>(&mut self, mut f: F) -> bool
    where
        F: FnMut(&mut Self, Option<(std::time::Instant, Self::Event)>) -> bool,
    {
        let mut time = Instant::now();
        loop {
            if let Some(p) = self.buffer.next() {
                time = self.last_time;
                // dir>=5760 的不是采样数据，不知道有什么用
                if p.dir < 5760 {
                    if !f(self, Some((time, p))) {
                        // 如果回调指示不要继续阻塞，立即退出
                        return true;
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

// impl DriverStatus for D10Frame {
//     type Event = Point;

//     fn update(&mut self, p: Self::Event) {
//         // 交换缓存
//         if let Some(Point { len: _, dir }) = self.0.back() {
//             if p.dir <= *dir {
//                 std::mem::swap(&mut self.0, &mut self.1);
//             }
//         }
//         // 销毁上一帧
//         while let Some(Point { len: _, dir }) = self.1.front() {
//             if p.dir < *dir {
//                 break;
//             }
//             self.1.pop_front();
//         }
//         // 保存
//         if p.len > 0 {
//             self.0.push_back(p);
//         }
//     }
// }
