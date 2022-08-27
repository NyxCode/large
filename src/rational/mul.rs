use std::ops::{Div, Mul};

use crate::Rational;

impl<const S: usize> Mul for Rational<S>
where
    [(); S + 1]:,
    [(); 2 * S + 1]:,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        // this might take S*2 space
        let lhs = self.resized::<{ 2* S }>();
        let rhs = rhs.resized::<{ 2* S }>();

        let num = lhs.num * rhs.num;
        let den = lhs.den * rhs.den;
        Rational {
            sign: lhs.sign * rhs.sign,
            num,
            den,
        }
        .resized()
        .reduced()
    }
}

impl<const S: usize> Div for Rational<S>
where
    [(); S + 1]:,
    [(); 2 * S + 1]:,
{
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.recip()
    }
}
