use crate::uint::Uint;
use crate::B;
use std::any::Any;
use std::cmp::Ordering;
use std::ops::{Div, DivAssign, Rem, RemAssign};

impl<const SIZE: usize> Div for Uint<SIZE>
where
    [(); SIZE + 1]: Any,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self._div_rem::<false>(rhs).0
    }
}

impl<const SIZE: usize> DivAssign for Uint<SIZE>
where
    [(); SIZE + 1]: Any,
{
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<const SIZE: usize> Rem for Uint<SIZE>
where
    [(); SIZE + 1]: Any,
{
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        self._div_rem::<true>(rhs).1
    }
}

impl<const SIZE: usize> RemAssign for Uint<SIZE>
where
    [(); SIZE + 1]: Any,
{
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

impl<const SIZE: usize> Uint<SIZE>
where
    [(); SIZE + 1]: Any,
{
    /// Divives `self` by b, returning `(self / b, self % b)`.
    ///
    /// # Panics
    ///
    /// This function panics if `b` is zero
    pub fn div_rem(self, b: Self) -> (Uint<SIZE>, Uint<SIZE>) {
        self._div_rem::<true>(b)
    }

    // `REM` indicates wheather the remainder should be calculated. If it's `false`, the remainder will be 0.
    fn _div_rem<const REM: bool>(self, b: Self) -> (Uint<SIZE>, Uint<SIZE>) {
        assert_ne!(b, Uint::ZERO, "attempt to divide by zero");

        if b == Uint::ONE {
            return (self, Uint::ZERO);
        }

        match self.cmp(&b) {
            Ordering::Less => return (Uint::ZERO, self),
            Ordering::Equal => return (Uint::ONE, Uint::ZERO),
            Ordering::Greater => {}
        };

        // a.len() > b.len() holds from now on

        // normalize a and b
        // resize a and b to allow for normalization
        let mut a = self.resized::<{ SIZE + 1 }>();
        let mut b = b.resized::<{ SIZE + 1 }>();
        let normalize = b.significant_digits() > 1 && b.msd() < (B / 2) as u32;
        let normalization_factor = if normalize {
            let f = (B / (b.msd() as u64 + 1)) as u32;
            a *= f;
            b *= f;
            f
        } else {
            1
        };

        let idd_len = b.significant_digits() + 1;
        let mut a_digits = a.digits_be().iter();
        let mut quotient = Uint::<SIZE>::ZERO;
        let mut quotient_digits = 0;

        // in the worst-case, we need b+1 digits here
        // the IDD is left-padded with 0s.
        let mut idd = Uint::<{ SIZE + 1 }>::ZERO;
        let idd_msd = (SIZE + 1) - idd_len;

        // create initial IDD
        // the initial idd must start with 0, followed by b.len() digits from a
        for i in 0..b.significant_digits() {
            let next = a_digits.next().unwrap();
            idd.digits[idd_msd + 1 + i] = *next;
        }

        loop {
            // first estimate of d
            let d1 = {
                let idd_1msd = idd.digits[idd_msd];
                let idd_2msd = idd.digits[idd_msd + 1];
                let d1 = join_u32(idd_1msd, idd_2msd) / b.msd() as u64;
                d1.min(u32::MAX as u64) as u32
            };

            let (d, d_mul_b) = correct_d(d1, b.resized(), idd);

            // add to quotient
            quotient.digits[quotient_digits] = d;
            quotient_digits += 1;

            // calculate next IDD
            idd -= d_mul_b;

            // push next digit from 'a' to 'idd'
            match a_digits.next() {
                None => break,
                Some(next_a) => {
                    idd.push_lsd(*next_a);
                    debug_assert!(&idd.digits[0..idd_msd].iter().all(|i| *i == 0));
                }
            }
        }

        // shift quotient
        // in the loop above, we added elements to `quotient`, starting from the index 0
        // now, we shift it to the right so that the LSD is at (SIZE - 1).
        quotient.shr_digits(SIZE - quotient_digits);

        let rem = if REM {
            idd.resized::<SIZE>() / Uint::from_u32(normalization_factor)
        } else {
            Uint::ZERO
        };

        (quotient, rem)
    }
}

fn correct_d<const S: usize>(
    mut d: u32,
    divisor: Uint<S>,
    idd: Uint<S>,
) -> (u32, Uint<S>) {
    let mut t = divisor * d;
    if t > idd {
        d -= 1;
        t -= divisor;

        if t > idd {
            d -= 1;
            t -= divisor;

            debug_assert!(
                t <= idd,
                "this should be impossible, since we tested d, d-1 and d-2"
            );
        }
    }
    (d, t)
}

fn join_u32(hi: u32, lo: u32) -> u64 {
    (lo as u64) | ((hi as u64) << 32)
}

#[cfg(test)]
mod tests {
    use crate::uint::Uint;
    use num_bigint::BigUint;
    use num_integer::Integer;
    use rand::Rng;

    #[test]
    fn trivial() {
        check_correct_u128(84, 2);
        check_correct_u128(120, 4);
        check_correct_u128(u128::MAX, u128::MAX);
        check_correct_u128(u128::MAX, 1); // TODO broken
        check_correct_u128(u128::MAX, 2);
        check_correct_u128(u32::MAX as u128 + 1, 2);
        check_correct_u128(6_000_000_001, 6_000_000_000);
        check_correct(
            Uint::new([
                0, 0x9f678ffd, 0xb20a2013, 0xc045f6f0, 0xaf815561, 0x8475d5d1, 0x3781413e,
                0xebaae81b, 0x648a6037, 0x62743c0a, 0x8e1f4e2c, 0x7bce12e2, 0x0c5c33ea, 0xfb7a7f93,
                0x0172ccba, 0xcac9819c, 0x6b925af6,
            ]),
            Uint::new([
                0, 0, 0, 0, 0, 0, 0, 0, 0x6b0094a1, 0x4e6f3e6e, 0xeb8d33ce, 0x70e3497b, 0x5121a8ff,
                0x38f0e793, 0xe454f891, 0xc4aac428, 0x014c3e47,
            ]),
        );
    }

    #[test]
    fn against_num_bigint() {
        for _ in 0..5000 {
            fuzz::<1, 1>();
            fuzz::<10, 10>();
            fuzz::<10, 7>();
            fuzz::<16, 16>();
            fuzz::<16, 11>();
        }
    }

    fn check_correct_u128(a: u128, b: u128) {
        check_correct::<4>(Uint::from_u128(a), Uint::from_u128(b));
    }

    fn check_correct<const S: usize>(a: Uint<S>, b: Uint<S>)
    where
        [(); S + 1]:,
    {
        let (my_q, my_r) = a.div_rem(b);
        let (c_q, c_r) = BigUint::from(a).div_rem(&BigUint::from(b));

        assert_eq!(my_q, c_q, "quotient differs");
        assert_eq!(my_r, c_r, "remainder differs");
    }

    fn fuzz<const A: usize, const B: usize>()
    where
        [(); A + 1]:,
    {
        let mut a = [0u32; A];
        let mut b = [0u32; B];
        rand::thread_rng().fill(&mut a[..]);
        rand::thread_rng().fill(&mut b[..]);

        // construct bigints
        let my_a = Uint::new(a);
        let my_b = Uint::new(b).resized::<A>();

        check_correct(my_a, my_b);
    }
}
