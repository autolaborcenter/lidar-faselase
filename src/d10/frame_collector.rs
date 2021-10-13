use crate::{zip::PointZipped, Point};
use nalgebra::Point2;
use std::{f64::consts::PI, io::Write};

pub struct FrameCollector {
    pub sections: Vec<(Vec<PointZipped>, Vec<Point2<i16>>)>,
    pub trans: (i16, i16, f64),
}

impl FrameCollector {
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
            trans: (0, 0, 0.0),
        }
    }

    pub fn clear(&mut self) {
        self.sections.clear();
    }

    pub fn put(&mut self, i: usize, section: Vec<Point>) {
        if self.sections.len() <= i {
            self.sections
                .resize_with(i + 1, || (Vec::new(), Vec::new()));
        }
        self.sections[i] = (
            section
                .iter()
                .map(|p| PointZipped::new(p.len, p.dir))
                .collect(),
            section
                .iter()
                .map(|Point { len, dir }| {
                    let (x, y, t) = self.trans;
                    let len = *len as f64 * 10.0;
                    let dir = *dir as f64 * 2.0 * PI / 5760.0 + t;
                    Point2::new(
                        (dir.cos() * len).round() as i16 + x,
                        (dir.sin() * len).round() as i16 + y,
                    )
                })
                .collect(),
        );
    }

    pub fn write_to(&self, buf: &mut Vec<u8>) {
        const LEN: usize = std::mem::size_of::<PointZipped>();

        let len = self.sections.iter().map(|(s, _)| s.len()).sum::<usize>();
        buf.reserve(len * LEN + std::mem::size_of::<u16>());
        unsafe {
            let _ = buf.write_all(std::slice::from_raw_parts(
                &(len as u16) as *const _ as *const u8,
                std::mem::size_of::<u16>(),
            ));
            for (s, _) in &self.sections {
                let _ = buf.write_all(std::slice::from_raw_parts(
                    s.as_ptr() as *const u8,
                    s.len() * LEN,
                ));
            }
        }
    }
}
