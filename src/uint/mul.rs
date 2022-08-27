use crate::uint::Uint;
use std::ops::{Mul, MulAssign};

impl<const SIZE: usize> Uint<SIZE> {}

impl<const SIZE: usize> Mul<u32> for Uint<SIZE> {
    type Output = Self;

    fn mul(mut self, rhs: u32) -> Self::Output {
        // needs some benchmarking when if makes sense to do this check
        if SIZE >= 64 {
            match rhs {
                0 => return Self::ZERO,
                1 => return self,
                _ => {}
            };
        }

        let mut carry = 0;
        for j in (0..SIZE).rev() {
            let (result, c) = self.digits[j].carrying_mul(rhs, carry);
            carry = c;
            self.digits[j] = result;
        }
        if carry != 0 {
            println!("{:?} * {:?}", self, rhs);
            assert_eq!(carry, 0, "attempt to multiply with overflow");
        }
        self
    }
}

impl<const SIZE: usize> MulAssign<u32> for Uint<SIZE> {
    fn mul_assign(&mut self, rhs: u32) {
        *self = *self * rhs;
    }
}

impl<const SIZE: usize> Mul for Uint<SIZE> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut out = Self::ZERO;

        for j in (0..SIZE).rev() {
            let digit = rhs.digits[j];
            let mut row = self * digit;
            row.shl_digits(SIZE - 1 - j);
            out += row;
        }

        out
    }
}

impl<const SIZE: usize> MulAssign for Uint<SIZE> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::BigUint;
    use rand::Rng;

    use crate::uint::Uint;

    #[test]
    fn identity() {
        let a: Uint<5> = Uint {
            digits: [0, 21213, 0, 18321, 12412],
        };
        assert_eq!(a * 1u32, a);
    }

    #[test]
    fn trivial() {
        check_correct_u128(1234, 2);
        check_correct_u128(2, 1234);
        check_correct_u128(u64::MAX as u128, 4);
        check_correct_u128(4, u64::MAX as u128);
    }

    #[test]
    fn against_num_bigint() {
        for _ in 0..5_000 {
            fuzz::<1>();
            fuzz::<3>();
            fuzz::<6>();
            fuzz::<10>();
            fuzz::<13>();
            fuzz::<16>();
        }
    }

    fn check_correct<const S: usize>(a: Uint<S>, b: Uint<S>) {
        let my_res = a * b;
        let c_res = BigUint::from(a) * BigUint::from(b);
        assert_eq!(c_res, my_res);
    }

    fn check_correct_u128(a: u128, b: u128) {
        check_correct::<4>(Uint::from_u128(a), Uint::from_u128(b));
    }

    fn fuzz<const A: usize>()
    where
        [(); A * 2]:,
    {
        // random input
        let mut a = [0u32; A];
        let mut b = [0u32; A];
        rand::thread_rng().fill(&mut a[..]);
        rand::thread_rng().fill(&mut b[..]);

        // construct bigints
        let my_a = Uint::new(a).resized();
        let my_b = Uint::new(b).resized();

        check_correct::<{ A * 2 }>(my_a, my_b);
    }
}
