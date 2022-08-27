use crate::uint::Uint;
use std::ops::{Add, AddAssign, Sub, SubAssign};

impl<const SIZE: usize> Add for Uint<SIZE> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        let mut carry = false;
        for i in (0..SIZE).rev() {
            let l = &mut self.digits[i];
            let (res, c) = l.carrying_add(rhs.digits[i], carry);
            carry = c;
            *l = res;
        }
        debug_assert!(!carry, "attempt to add with overflow");
        self
    }
}

impl<const SIZE: usize> AddAssign for Uint<SIZE> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const SIZE: usize> Sub for Uint<SIZE> {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        let mut borrow = false;
        for i in (0..SIZE).rev() {
            let l = &mut self.digits[i];
            let (res, b) = l.borrowing_sub(rhs.digits[i], borrow);
            borrow = b;
            *l = res;
        }
        debug_assert!(!borrow, "attempt to subtract with overflow");
        self
    }
}

impl<const SIZE: usize> SubAssign for Uint<SIZE> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

#[cfg(test)]
mod tests {
    use crate::uint::Uint;

    #[test]
    fn underflowing_digit() {
        let a = Uint {
            digits: [0, 1, 0]
        };
        let b = Uint {
            digits: [0, 0, 1]
        };
        assert_eq!(a - b, Uint {
            digits: [0, 0, u32::MAX]
        })
    }
}