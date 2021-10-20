use super::Point;

pub(super) struct PortBuffer<const LEN: usize> {
    buffer: [u8; LEN],
    cursor_r: usize,
    cursor_w: usize,
}

impl<const LEN: usize> Default for PortBuffer<LEN> {
    fn default() -> Self {
        Self {
            buffer: [0u8; LEN],
            cursor_r: 0,
            cursor_w: 0,
        }
    }
}

impl<const LEN: usize> PortBuffer<LEN> {
    pub fn as_buf<'a>(&'a mut self) -> &'a mut [u8] {
        &mut self.buffer[self.cursor_w..]
    }

    pub fn notify_received<'a>(&'a mut self, n: usize) {
        self.cursor_w += n;
    }
}

impl<const LEN: usize> Iterator for PortBuffer<LEN> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            const LEN: usize = std::mem::size_of::<u32>();
            let slice = &self.buffer[self.cursor_r..self.cursor_w];
            if slice.len() >= LEN {
                if let Ok(p) = unsafe { *(slice.as_ptr() as *const u32) }.try_into() {
                    self.cursor_r += LEN;
                    return Some(p);
                } else {
                    self.cursor_r += 1;
                }
            } else {
                self.buffer.copy_within(self.cursor_r..self.cursor_w, 0);
                self.cursor_w -= self.cursor_r;
                self.cursor_r = 0;
                return None;
            }
        }
    }
}
