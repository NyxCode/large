#![feature(generic_const_exprs)]

use large::{Complex, Rational};

type C = Complex<Rational<8>>;
const ITERATIONS: usize = 15;
const THRESHOLD_SQUARED: Rational<8> = Rational::from_u32(4);

fn main() {
    let viewport = (
        C {
            r: (-2).into(),
            i: (-1).into(),
        },
        C {
            r: Rational::from(1) / Rational::from(2),
            i: 1.into(),
        },
    );
    let rows = 20_u32;
    let cols = 80_u32;

    let r_step = (viewport.1.r - viewport.0.r) / cols;
    let i_step = (viewport.1.i - viewport.0.i) / rows;

    let mut y = viewport.0.i;
    for _ in 0..rows {
        let mut x = viewport.0.r;
        for _ in 0..cols {
            let ch = match z(C { r: x, i: y }) {
                Ok(_) => '#',
                Err(n) => [' ', '.', '+', 'x'][3 * n / ITERATIONS],
            };
            print!("{}", ch);
            x += r_step;
        }
        y += i_step;
        println!();
    }
}

fn z(c: C) -> Result<(), usize> {
    let mut z = C {
        r: 0.into(),
        i: 0.into(),
    };
    for i in 1..=ITERATIONS {
        z = z * z + c;

        if z.abs_squared() > THRESHOLD_SQUARED {
            return Err(i);
        }
    }
    Ok(())
}
