#![feature(bigint_helper_methods)]
#![feature(generic_const_exprs)]

extern crate core;

mod uint;
mod rational;
mod complex;
mod sign;

pub(crate) const B: u64 = 0x1_00_00_00_00;

pub use uint::Uint;
pub use rational::Rational;
pub use complex::Complex;
pub use sign::Sign;


#[cfg(test)]
mod test_support {
    use num_bigint::{BigUint, BigInt};
    use num_rational::BigRational;
    use num_traits::Signed;

    use crate::{Uint, rational::Rational, Sign};

    impl<const SIZE: usize> PartialEq<BigUint> for Uint<SIZE> {
        fn eq(&self, other: &BigUint) -> bool {
            let mut other = other.to_u32_digits();
            other.reverse();
            if other.is_empty() {
                other.push(0);
            }
            self.digits_be() == other
        }
    } 

    impl<const SIZE: usize> PartialEq<Uint<SIZE>> for BigUint {
        fn eq(&self, other: &Uint<SIZE>) -> bool {
            other == self
        }
    }

    impl<const SIZE: usize> From<Uint<SIZE>> for BigUint {
        fn from(v: Uint<SIZE>) -> Self {
            BigUint::new(v.digits_be().iter().copied().rev().collect())
        }
    }

    // --

    impl<const SIZE: usize> PartialEq<BigRational> for Rational<SIZE> {
        fn eq(&self, other: &BigRational) -> bool {
            let other_sign = match other.numer().sign() * other.denom().sign() {
                num_bigint::Sign::Minus => Sign::Neg,
                num_bigint::Sign::NoSign => todo!(),
                num_bigint::Sign::Plus => Sign::Pos,
            };
            self.sign == other_sign && self.num == other.numer().abs().to_biguint().unwrap() && self.den == other.denom().abs().to_biguint().unwrap()
        }
    } 

    impl<const SIZE: usize> PartialEq<Rational<SIZE>> for BigRational {
        fn eq(&self, other: &Rational<SIZE>) -> bool {
            other == self
        }
    }

    impl<const SIZE: usize> From<Rational<SIZE>> for BigRational {
        fn from(v: Rational<SIZE>) -> Self {
            let sign = match v.sign {
                Sign::Pos => num_bigint::Sign::Plus,
                Sign::Neg => num_bigint::Sign::Minus
            };
            let num = BigInt::from_biguint(sign, v.num.into());
            let den = BigInt::from_biguint(num_bigint::Sign::Plus, v.den.into());
            BigRational::new(num, den)
        }
    }
}
