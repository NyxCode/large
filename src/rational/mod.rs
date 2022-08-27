use std::cmp::Ordering;

use crate::{Uint, Sign, Sign::*};

mod add;
mod compat;
mod convert;
mod mul;

#[derive(Copy, Clone)]
pub struct Rational<const S: usize> {
    pub num: Uint<S>,
    pub den: Uint<S>,
    pub sign: Sign,
}

impl<const S: usize> Rational<S>
where
    [(); S + 1]:,
{
    pub const ZERO: Self = Self::new(Pos, Uint::from_u32(0), Uint::from_u32(1));
    pub const ONE: Self = Self::new(Pos, Uint::from_u32(1), Uint::from_u32(1));

    pub const fn new(sign: Sign, num: Uint<S>, den: Uint<S>) -> Self {
        Rational { sign, num, den }
    }

    fn reduced(mut self) -> Self {
        let gcd = self.num.gcd_binary(self.den);
        if gcd == Uint::ZERO {
            return self;
        }
        self.num /= gcd;
        self.den /= gcd;
        self
    }

    pub fn recip(self) -> Self {
        assert!(self.num != Uint::ZERO);
        Rational {
            num: self.den,
            den: self.num,
            sign: self.sign,
        }
    }

    /// resizes this number.
    /// If the number does not fit into the new size, rounding takes place.
    pub fn resized<const NEW_S: usize>(mut self) -> Rational<NEW_S> {
        if NEW_S < S {
            let significant_digits = self
                .num
                .significant_digits()
                .max(self.den.significant_digits());
            if NEW_S < significant_digits {
                // round down
                // TODO: this is kinda inprecise, since we always div by 2^32
                self.num.shr_digits(significant_digits - NEW_S);
                self.den.shr_digits(significant_digits - NEW_S);
            }
        }
        Rational {
            sign: self.sign,
            num: self.num.resized(),
            den: self.den.resized(),
        }
    }
}

impl<const S: usize> PartialEq for Rational<S>
where
    [(); S + 1]:,
    [(); 2 * S + 1]:,
{
    fn eq(&self, other: &Self) -> bool {
        if self.sign != other.sign {
            return false;
        }
        let mut a = self.resized::<{ 2 * S }>();
        let mut b = other.resized::<{ 2 * S }>();
        to_same_denominator(&mut a, &mut b);
        a.num == b.num
    }
}

impl<const S: usize> PartialOrd for Rational<S>
where
    [(); S + 1]:,
    [(); 2 * S + 1]:,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.sign.cmp(&other.sign) {
            Ordering::Equal => {}
            sign => return Some(sign),
        };
        let mut a = self.resized::<{ 2 * S }>();
        let mut b = other.resized::<{ 2 * S }>();
        to_same_denominator(&mut a, &mut b);

        if self.sign == Neg {
            b.num.partial_cmp(&a.num)
        } else {
            a.num.partial_cmp(&b.num)
        }
    }
}

fn to_same_denominator<const S: usize>(a: &mut Rational<S>, b: &mut Rational<S>)
where
    [(); S + 1]:,
{
    if a.den == b.den {
        return;
    }

    a.num *= b.den;
    b.num *= a.den;

    let den = a.den * b.den;
    a.den = den;
    b.den = den;
}
