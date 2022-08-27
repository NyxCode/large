use std::{fmt::Debug, fmt::Display};

use crate::{Uint, Sign};

use super::Rational;

impl<const S: usize> Rational<S>
where
    [(); S + 1]:,
{
    pub const fn from_u32(v: u32) -> Self {
        Rational {
            sign: Sign::Pos,
            num: Uint::from_u32(v),
            den: Uint::ONE
        }
    }
}

impl<const S: usize> Debug for Rational<S> where [(); S + 1]: {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rational {{ sign: {:?}, num: {}, den: {} }}", self.sign, self.num, self.den)
    }
}

impl<const S: usize> Display for Rational<S> where [(); S + 1]: {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.sign == Sign::Neg {
            write!(f, "-")?;
        }
        write!(f, "{}/{}", self.num, self.den)
    }
}

impl<const S: usize, T> From<T> for Rational<S> where Uint<S>: From<T> {
    fn from(v: T) -> Self {
        Rational { num: Uint::from(v), den: Uint::ONE, sign: Sign::Pos }
    }
}

macro_rules! impl_signed_from {
    ($($t:tt),*) => {
        $(
            impl<const S: usize> From<$t> for Rational<S> {
                fn from(v: $t) -> Self {
                    Rational {
                        num: Uint::from(v.unsigned_abs()),
                        den: Uint::ONE,
                        sign: if v.is_negative() {
                            Sign::Neg
                        } else {
                            Sign::Pos
                        }
                    }
                }
            }
        )*
    };
}

impl_signed_from!(i8, i16, i32, i64, i128);