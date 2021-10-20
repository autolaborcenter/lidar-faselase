/// 雷达的一个点，压缩到 3 字节
#[derive(Clone, Copy)]
pub struct PointZipped(pub [u8; 3]);

impl PointZipped {
    #[inline]
    pub const fn new(len: u16, dir: u16) -> Self {
        Self([len as u8, dir as u8, ((len >> 8 << 5) | (dir >> 8)) as u8])
    }

    #[inline]
    pub const fn len(&self) -> u16 {
        (self.0[2] as u16 >> 5 << 8) | (self.0[0] as u16)
    }

    #[inline]
    pub const fn dir(&self) -> u16 {
        (((self.0[2] & 0x1f) as u16) << 8) | (self.0[1] as u16)
    }
}

#[test]
fn assert_size() {
    assert_eq!(std::mem::size_of::<[PointZipped; 7]>(), 21);
}

#[test]
fn assert_assign() {
    const POINT: PointZipped = PointZipped::new(999, 7777);
    assert_eq!(POINT.len(), 999);
    assert_eq!(POINT.dir(), 7777);
}
