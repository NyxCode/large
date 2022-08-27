use core::{fmt, iter};

use super::Array2N;
use super::Uint;

impl<const SIZE: usize> Uint<SIZE> where [(); SIZE + 1]: {
    /// Converts this number to the given base.  
    /// Each element of the returned iterator represents one digit in the given base.
    /// Digits are yielded in little-endian order, starting with the least significant digit.
    ///
    /// The iterator is guaranteed to always yield at least one digit.
    pub fn to_base_le(mut self, base: u32) -> impl Iterator<Item = u32> {
        let base = Uint::from_u32(base);
        let mut i = 0;

        iter::from_fn(move || {
            if i != 0 && self == Self::ZERO {
                return None;
            }
            let (q, r) = self.div_rem(base);
            self = q;
            i += 1;

            // since we divided by base < 2^32, r < base < 2^32. So the remainder fits in one u32.
            Some(r.digits[SIZE - 1])
        })
    }

    // can this be generalized to work with any basis?
    // for that to work, some math has to be done (and a lot of const trickery).
    // definitely something to look into - the more generic `to_string_radix` needs to allocate.
    pub fn to_base_10_be(self) -> impl Iterator<Item = u32> {
        // digits in base (10^9). Since (10^9)^(SIZE * 2) > (2^32)^SIZE, everything fits in here
        let mut digits = Array2N::<SIZE, u32>::new(0);
        let mut len = 0;
        for (i, digit) in self.to_base_le(1_000_000_000).enumerate() {
            digits[i] = digit;
            len = i + 1;
        }
        debug_assert!(len > 0);

        (0..len).map(move |i| digits[i]).rev()
    }

    pub fn to_string_radix(self, radix: u32) -> String {

        self.to_base_le(radix)
            .map(|c| char::from_digit(c, radix).unwrap())
            .collect::<Vec<char>>()
            .into_iter()
            .rev()
            .collect::<String>()
    }
}

impl <const SIZE: usize> fmt::Display for Uint<SIZE> where [(); SIZE + 1]: {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut digits = self.to_base_10_be();


        // print the MS digit without leading 0's
        write!(f, "{}", digits.next().unwrap())?;

        for digit in digits {
            write!(f, "{:09}", digit)?;
        }

        Ok(())
    }
}

impl <const SIZE: usize> fmt::Binary for Uint<SIZE> where [(); SIZE + 1]: {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_radix(2))
    }
}

impl <const SIZE: usize> fmt::LowerHex for Uint<SIZE> where [(); SIZE + 1]: {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_radix(16))
    }
}

impl <const SIZE: usize> fmt::UpperHex for Uint<SIZE> where [(); SIZE + 1]: {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut hex = self.to_string_radix(16);
        hex.make_ascii_uppercase();
        write!(f, "{}", hex)
    }
}

impl <const SIZE: usize> fmt::Octal for Uint<SIZE> where [(); SIZE + 1]: {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_radix(8))
    }
}

#[cfg(test)]
mod tests {
    use crate::Uint;

    #[test]
    fn big() {
        let v = Uint::<100>::new([u32::MAX; 100]);
        assert_eq!(v.to_string_radix(16), "ff".repeat(4 * 100));
    }
}