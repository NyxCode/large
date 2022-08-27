use std::fmt::Debug;
use std::ops::{Index, IndexMut, Shl, Shr};

mod add;
mod base;
mod convert;
mod div;
mod gcd;
mod mul;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Uint<const SIZE: usize> {
    // digits in big-endian order - digits[0] is the MSB
    digits: [u32; SIZE],
}

impl<const SIZE: usize> Uint<SIZE> {
    pub const ZERO: Self = Self::from_u32(0);
    pub const ONE: Self = Self::from_u32(1);
    pub const TWO: Self = Self::from_u32(2);
    pub const MAX: Self = Uint {
        digits: [u32::MAX; SIZE],
    };
    pub const SIZE_TWICE: usize = SIZE * 2;

    pub const fn new(digits: [u32; SIZE]) -> Self {
        Uint { digits }
    }

    // resizes this number, behaving like an `as` cast would.
    pub fn resized<const NEW_SIZE: usize>(self) -> Uint<NEW_SIZE> {
        let mut new = Uint::<NEW_SIZE>::ZERO;
        for i in 0..NEW_SIZE.min(SIZE) {
            let old = self.digits[SIZE - 1 - i];
            new.digits[NEW_SIZE - 1 - i] = old;
        }

        new
    }

    // returns the index of the most significant digit which is not 0.
    // If self == ZERO, this will return SIZE-1.
    pub fn msd_idx(&self) -> usize {
        self.digits.iter().position(|d| *d != 0).unwrap_or(SIZE - 1)
    }

    pub fn msd(&self) -> u32 {
        self.digits[self.msd_idx()]
    }

    pub fn digits_be(&self) -> &[u32] {
        &self.digits[self.msd_idx()..]
    }

    // push a least significant digit to this number
    pub fn push_lsd(&mut self, lsd: u32) {
        self.shl_digits(1);
        self.digits[SIZE - 1] = lsd;
    }

    // pushed all digits one to the left.
    // equivalent to `self *= 2^32`
    pub fn shl_digits(&mut self, n: usize) {
        self.digits.rotate_left(n);

        for i in (SIZE - n)..SIZE {
            self.digits[i] = 0;
        }
    }

    // pushed all digits to the right by n places.
    pub fn shr_digits(&mut self, n: usize) {
        self.digits.rotate_right(n);

        for i in 0..n {
            self.digits[i] = 0;
        }
    }

    // returns the number on non-zero digits this number has.
    // the function will always return at least 0
    pub fn significant_digits(&self) -> usize {
        SIZE - self.msd_idx()
    }

    pub fn trailing_zeros(&self) -> u32 {
        let mut result = 0;
        for i in (0..SIZE).rev() {
            let digit = self.digits[i];
            let zeros = digit.trailing_zeros();
            result += zeros;
            if zeros < 32 {
                break;
            }
        }
        result
    }
}

impl<const SIZE: usize> Shl<u64> for Uint<SIZE> {
    type Output = Self;

    fn shl(mut self, rhs: u64) -> Self::Output {
        debug_assert!(rhs <= u32::MAX as u64);
        if rhs == 0 {
            return self;
        }
        let mut rhs = rhs as u32;

        self.shl_digits(rhs as usize / 32);
        rhs %= 32;
        if rhs == 0 {
            return self;
        }

        let mask = (2u32.pow(rhs) - 1) << (32 - rhs); 
        let mut carry = 0u32;
        for i in (0..SIZE).rev() {
            let digit = &mut self.digits[i];
            let last_carry = carry;
            carry = (*digit & mask) >> (32 - rhs);
            *digit = digit.wrapping_shl(rhs) | last_carry;
        }

        self
    }
}

impl<const SIZE: usize> Shr<u64> for Uint<SIZE> {
    type Output = Self;

    fn shr(mut self, rhs: u64) -> Self::Output {
        debug_assert!(rhs <= u32::MAX as u64);
        if rhs == 0 {
            return self;
        }
        let mut rhs = rhs as u32;

        self.shr_digits(rhs as usize / 32);
        rhs %= 32;

        if rhs == 0 {
            return self;
        }

        let mask = 2u32.pow(rhs) - 1; 
        let mut carry = 0u32;
        for digit in &mut self.digits {
            let last_carry = carry;
            carry = (*digit & mask) << (32 - rhs);
            *digit = digit.wrapping_shr(rhs as u32) | last_carry;
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_sub() {
        let a: Uint<4> = Uint::from_u32(12);
        let b: Uint<4> = Uint::from_u32(24);
        assert_eq!(b - a, Uint::from_u32(12));
        assert_eq!(a + b, b + a);
        assert_eq!(a + a, b);
        assert_eq!(a + b, Uint::from_u32(36));
    }

    #[test]
    fn trailing_zeros() {
        assert_eq!(Uint::<1>::ZERO.trailing_zeros(), 32);
        assert_eq!(Uint::<17>::ZERO.trailing_zeros(), 17 * 32);
        assert_eq!(Uint::<17>::ONE.trailing_zeros(), 0);
        assert_eq!(
            Uint::<6>::new([0, 0, 1, 0, 0, 0]).trailing_zeros(),
            3 * 32
        );
        assert_eq!(
            Uint::<6>::new([0, 0, 2, 0, 0, 0]).trailing_zeros(),
            3 * 32 + 1
        );
    }
}

/// an array holding 2*N elements of type T
/// required since `let x: [u32; S * 2]`, where `const S: usize` is not possible right now.
#[derive(Debug)]
struct Array2N<const N: usize, T>([[T; N]; 2]);

impl<const N: usize, T: Copy> Array2N<N, T> {
    fn new(v: T) -> Self {
        Array2N([[v; N]; 2])
    }
}

impl<const N: usize, T> Index<usize> for Array2N<N, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let first = index / N;
        let second = index - first * N;
        &self.0[first][second]
    }
}

impl<const N: usize, T> IndexMut<usize> for Array2N<N, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let first = index / N;
        let second = index - first * N;
        &mut self.0[first][second]
    }
}
