use std::ops::Mul;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(i8)]
pub enum Sign {
    Pos = 1,
    Neg = -1
}

impl Mul for Sign {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match self as i8 * rhs as i8 {
            1 => Sign::Pos,
            -1 => Sign::Neg,
            _ => unreachable!()
        }
    }
}

impl PartialOrd for Sign {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (*self as i8).partial_cmp(&(*other as i8))
    }
}

impl Ord for Sign {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i8).cmp(&(*other as i8))
    }
}