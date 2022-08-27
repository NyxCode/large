use crate::uint::Uint;

impl<const SIZE: usize> Uint<SIZE> {
    pub const fn from_u32(v: u32) -> Self {
        assert!(SIZE >= 1);
        let mut data = [0; SIZE];
        data[SIZE - 1] = v;
        Uint { digits: data }
    }

    pub const fn from_u64(v: u64) -> Self {
        assert!(SIZE >= 2);
        let mut data = [0; SIZE];
        data[SIZE - 1] = v as u32;
        data[SIZE - 2] = (v >> 32) as u32;
        Uint { digits: data }
    }

    pub const fn from_u128(v: u128) -> Self {
        assert!(SIZE >= 4);
        let mut data = [0; SIZE];
        data[SIZE - 1] = v as u32;
        data[SIZE - 2] = (v >> 32) as u32;
        data[SIZE - 3] = (v >> 64) as u32;
        data[SIZE - 4] = (v >> 96) as u32;
        Uint { digits: data }
    }
    
    pub fn to_u128(self) -> Option<u128> {
        let digits = self.digits_be();
        if digits.len() > 4 {
            None
        } else {
            let out = digits
                .iter()
                .rev()
                .enumerate()
                .fold(0u128, |acc, (i, d)| acc | ((*d as u128) << (i * 32)));
            Some(out)
        }
    }
}

impl<const S: usize> From<u8> for Uint<S> {
    fn from(v: u8) -> Self {
        Uint::from_u32(v as u32)
    }
}

impl<const S: usize> From<u16> for Uint<S> {
    fn from(v: u16) -> Self {
        Uint::from_u32(v as u32)
    }
}

impl<const S: usize> From<u32> for Uint<S> {
    fn from(v: u32) -> Self {
        Uint::from_u32(v)
    }
}

impl<const S: usize> From<u64> for Uint<S> {
    fn from(v: u64) -> Self {
        Uint::from_u64(v)
    }
}

impl<const S: usize> From<u128> for Uint<S> {
    fn from(v: u128) -> Self {
        Uint::from_u128(v)
    }
}