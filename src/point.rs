#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub len: u16,
    pub dir: u16,
}

impl TryFrom<u32> for Point {
    type Error = ();

    fn try_from(bits: u32) -> Result<Self, Self::Error> {
        if (bits & 0x80_80_80_80) != 0x80_00_00_00 {
            return Err(());
        }

        const CBITS: [u8; 256] = [
            0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4, 1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3,
            4, 4, 5, 1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4,
            4, 5, 4, 5, 5, 6, 1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 2, 3, 3, 4, 3, 4, 4,
            5, 3, 4, 4, 5, 4, 5, 5, 6, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 3, 4, 4, 5,
            4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, 1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 2,
            3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5,
            5, 6, 3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4,
            5, 4, 5, 5, 6, 3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, 3, 4, 4, 5, 4, 5, 5, 6,
            4, 5, 5, 6, 5, 6, 6, 7, 4, 5, 5, 6, 5, 6, 6, 7, 5, 6, 6, 7, 6, 7, 7, 8,
        ];

        let l2 = (bits & 0b1111) as u16;
        let cb = ((bits >> 4) & 0b111) as u8; // `cb` for "check sum bits"
        let l1 = ((bits >> 8) & 0b111_1111) as u16;
        let d1 = ((bits >> 16) & 0b11_1111) as u16;
        let l0 = ((bits >> 22) & 1) as u16;
        let d0 = ((bits >> 24) & 0b111_1111) as u16;

        let bytes = unsafe { std::slice::from_raw_parts(&bits as *const u32 as *const u8, 4) };
        if bytes[1..].iter().map(|b| CBITS[*b as usize]).sum::<u8>() & 0b111 == cb {
            let len = (l2 << 8) | (l1 << 1) | l0;
            let dir = (d1 << 7) | d0;
            if len < 0x800 {
                Ok(Self { len, dir })
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}
