use driver::{Driver, DriverStatus};
use serial_port::{Port, SerialPort};
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

pub mod point;

use point::{Point, PortBuffer};

const POINT_RECEIVE_TIMEOUT: Duration = Duration::from_millis(200);
const POINT_PARSE_TIMEOUT: Duration = Duration::from_millis(250);

pub struct D10 {
    port: Port,
    buffer: PortBuffer<64>,
    last_time: Instant,

    frame: D10Frame,
}

#[derive(Clone)]
pub struct D10Frame(VecDeque<Point>, VecDeque<Point>);

impl Driver<String> for D10 {
    type Pacemaker = ();
    type Status = D10Frame;
    type Command = ();

    fn new(name: String) -> Option<(Self::Pacemaker, Self)> {
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

                    frame: D10Frame(VecDeque::with_capacity(300), VecDeque::with_capacity(300)),
                },
            )),
            Err(_) => None,
        }
    }

    fn status<'a>(&'a self) -> &'a Self::Status {
        &self.frame
    }

    fn send(&mut self, _: (std::time::Instant, Self::Command)) {}

    fn join<F>(&mut self, mut f: F) -> bool
    where
        F: FnMut(
            &mut Self,
            Option<(
                std::time::Instant,
                <Self::Status as driver::DriverStatus>::Event,
            )>,
        ) -> bool,
    {
        let mut time = Instant::now();
        loop {
            if let Some(p) = self.buffer.next() {
                time = self.last_time;
                // dir>=5760 的不是采样数据，不知道有什么用
                if p.dir < 5760 {
                    self.frame.update(p);
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

impl DriverStatus for D10Frame {
    type Event = Point;

    fn update(&mut self, p: Self::Event) {
        // 交换缓存
        if let Some(Point { len: _, dir }) = self.0.back() {
            if p.dir <= *dir {
                std::mem::swap(&mut self.0, &mut self.1);
            }
        }
        // 销毁上一帧
        while let Some(Point { len: _, dir }) = self.1.front() {
            if p.dir < *dir {
                break;
            }
            self.1.pop_front();
        }
        // 保存
        if p.len > 0 {
            self.0.push_back(p);
        }
    }
}
