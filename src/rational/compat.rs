use super::Rational;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

macro_rules! impl_ops {
    ($($t:ty),*) => {
        $(
            impl<const S: usize> Add<$t> for Rational<S>
            where
                [(); S + 1]:,
                [(); 2 * S + 1]:
            {
                type Output = Self;

                fn add(self, rhs: $t) -> Self {
                    self + Rational::from(rhs)
                }
            }

            impl<const S: usize> AddAssign<$t> for Rational<S>
            where
                [(); S + 1]:,
                [(); 2 * S + 1]:

            {
                fn add_assign(&mut self, rhs: $t) {
                    *self = *self + Rational::from(rhs);
                }
            }
            
            impl<const S: usize> Sub<$t> for Rational<S>
            where
                [(); S + 1]:,
                [(); 2 * S + 1]:
            {
                type Output = Self;

                fn sub(self, rhs: $t) -> Self {
                    self - Rational::from(rhs)
                }
            }

            impl<const S: usize> SubAssign<$t> for Rational<S>
            where
                [(); S + 1]:,
                [(); 2 * S + 1]:

            {
                fn sub_assign(&mut self, rhs: $t) {
                    *self = *self - Rational::from(rhs);
                }
            }
            
            impl<const S: usize> Mul<$t> for Rational<S>
            where
                [(); S + 1]:,
                [(); 2 * S + 1]:
            {
                type Output = Self;

                fn mul(self, rhs: $t) -> Self {
                    self * Rational::from(rhs)
                }
            }
            
            impl<const S: usize> MulAssign<$t> for Rational<S>
            where
                [(); S + 1]:,
                [(); 2 * S + 1]:

            {
                fn mul_assign(&mut self, rhs: $t) {
                    *self = *self * Rational::from(rhs);
                }
            }

            impl<const S: usize> Div<$t> for Rational<S>
            where
                [(); S + 1]:,
                [(); 2 * S + 1]:
            {
                type Output = Self;

                fn div(self, rhs: $t) -> Self {
                    self / Rational::from(rhs)
                }
            }
            
            impl<const S: usize> DivAssign<$t> for Rational<S>
            where
                [(); S + 1]:,
                [(); 2 * S + 1]:

            {
                fn div_assign(&mut self, rhs: $t) {
                    *self = *self / Rational::from(rhs);
                }
            }
        )*
    };
}

impl_ops!(u8, u16, u32, u64, u128);
