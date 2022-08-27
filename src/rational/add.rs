use std::{
    mem::swap,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use super::Rational;
use crate::{rational::to_same_denominator, Sign::*};

impl<const S: usize> Add for Rational<S>
where
    [(); S + 1]:,
    [(); 2 * S + 1]:,
{
    type Output = Self;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        match (self.sign, rhs.sign) {
            (Pos, Pos) => do_add_positive(self, rhs),
            (Neg, Neg) => {
                self.sign = Pos;
                rhs.sign = Pos;
                let mut result = do_add_positive(self, rhs);
                result.sign = Neg;
                result
            }
            (Pos, Neg) => {
                rhs.sign = Pos;
                do_sub_positive(self, rhs)
            }
            (Neg, Pos) => {
                self.sign = Pos;
                do_sub_positive(rhs, self)
            }
        }
    }
}
impl<const S: usize> AddAssign for Rational<S>
where
    [(); S + 1]:,
    [(); 2 * S + 1]:,
{
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const S: usize> Sub for Rational<S>
where
    [(); S + 1]:,
    [(); 2 * S + 1]:,
{
    type Output = Self;

    fn sub(mut self, mut rhs: Self) -> Self::Output {
        match (self.sign, rhs.sign) {
            (Pos, Pos) => do_sub_positive(self, rhs),
            (Neg, Neg) => {
                // (-a) - (-b) = b - a
                self.sign = Pos;
                rhs.sign = Pos;
                do_sub_positive(rhs, self)
            }
            (Pos, Neg) => {
                // a - (-b) = a + b
                rhs.sign = Pos;
                do_add_positive(self, rhs)
            }
            (Neg, Pos) => {
                // (-a) - b = -(a + b)
                let mut res = do_add_positive(self, rhs);
                res.sign = Neg;
                res
            }
        }
    }
}

impl<const S: usize> SubAssign for Rational<S>
where
    [(); S + 1]:,
    [(); 2 * S + 1]:,
{
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

fn do_add_positive<const S: usize>(a: Rational<S>, b: Rational<S>) -> Rational<S>
where
    [(); S + 1]:,
    [(); 2 * S + 1]:,
{
    assert!(a.sign == Pos && b.sign == Pos);

    // in the worst case, we need 2s space
    let mut a: Rational<{ 2 * S }> = a.resized();
    let mut b: Rational<{ 2 * S }> = b.resized();
    to_same_denominator(&mut a, &mut b);

    a.num += b.num;
    a.reduced().resized()
}

fn do_sub_positive<const S: usize>(mut a: Rational<S>, mut b: Rational<S>) -> Rational<S>
where
    [(); S + 1]:,
    [(); 2 * S + 1]:,
{
    assert!(a.sign == Pos);
    assert!(b.sign == Pos);

    let sign = if b > a {
        swap(&mut a, &mut b);
        Neg
    } else {
        Pos
    };

    // in the worst case, we need s^2 spaces
    let mut a: Rational<{ 2 * S }> = a.resized();
    let mut b: Rational<{ 2 * S }> = b.resized();
    to_same_denominator(&mut a, &mut b);

    a.num -= b.num;
    a.sign = sign;
    a.reduced().resized()
}

#[cfg(test)]
mod tests {
    use num_rational::BigRational;

    use crate::rational::Rational;
    use crate::{Uint, Sign::*};

    #[test]
    fn trivial() {
        assert_eq!(
            Rational::<4>::new(Pos, Uint::from_u32(21), Uint::ONE)
                + Rational::new(Pos, Uint::from_u32(42), Uint::from_u32(2)),
            Rational::<4>::new(Pos, Uint::from_u32(42), Uint::ONE)
        )
    }

    #[test]
    fn against_num_rational() {
        for _ in 0..500 {
            let a = Rational::new(
                Pos,
                Uint::from_u128(rand::random()),
                Uint::from_u128(rand::random()),
            );
            let b = Rational::new(
                Pos,
                Uint::from_u128(rand::random()),
                Uint::from_u128(rand::random()),
            );
            check::<12>(a, b, Op::Add);
            check::<12>(a, b, Op::Sub);
        }
    }

    enum Op {
        Add,
        Sub,
    }

    fn check<const S: usize>(a: Rational<S>, b: Rational<S>, op: Op)
    where
        [(); S + 1]:,
        [(); 2 * S + 1 + 1]:,
    {
        let c_a = BigRational::from(a);
        let c_b = BigRational::from(b);
        let (this, c) = match op {
            Op::Add => (a + b, c_a + c_b),
            Op::Sub => (a - b, c_a - c_b),
        };
        assert_eq!(this, c);
    }
}
