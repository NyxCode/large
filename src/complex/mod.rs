use std::{ops::{Add, Mul, Sub}, fmt::Display};

#[derive(Clone, Copy, Debug)]
pub struct Complex<T> {
    pub r: T,
    pub i: T
}

impl<T> Complex<T> where T: Add<Output = T> + Mul<Output = T> + Copy {
    pub fn abs_squared(self) -> T {
        self.r * self.r + self.i * self.i
    }
}

impl<T> Add for Complex<T> where T: Add<Output = T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            i: self.i + rhs.i
        }
    }
}

impl<T> Mul for Complex<T> where T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        //   (a + b*i)(x + y*i) 
        // = a*x + b*x*i + a*y*i - b*y
        // = (ax - b*y) + (b*x + a*y)*i 
        Self {
            r: self.r * rhs.r - self.i * rhs.i,
            i: self.i * rhs.r + self.r * rhs.i
        }
    }
}

impl<T> Display for Complex<T> where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {}i", self.r, self.i)
    }
}