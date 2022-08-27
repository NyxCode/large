use std::mem::swap;

use crate::Uint;

impl<const S: usize> Uint<S>
where
    [(); S + 1]:,
{
    pub fn gcd_euclidean(self, other: Self) -> Self {
        // gcm(a, b) = max { c | c * xa = a and c * xb = b }
        let (mut a, mut b) = (self, other);
        while b != Self::ZERO {
            (a, b) = (b, a % b);
        }
        a
    }

    pub fn gcd_binary(self, other: Self) -> Self {
        let (mut u, mut v) = (self, other);

        if u == Self::ZERO {
            return v;
        } else if v == Self::ZERO {
            return u;
        }

        let i = u.trailing_zeros();
        let j = v.trailing_zeros();
        u = u >> i as u64;
        v = v >> j as u64;
        let k = i.min(j);

        loop {
            // u and v are odd at the start of the loop
            debug_assert!(u % Uint::TWO == Uint::ONE, "u = {} is even", u);
            debug_assert!(v % Uint::TWO == Uint::ONE, "v = {} is even", v);
    
            // Swap if necessary so u <= v
            if u > v {
                swap(&mut u, &mut v);
            }
            // u and v are still both odd after (potentially) swapping
    
            // Using identity 4 (gcd(u, v) = gcd(|v-u|, min(u, v))
            v -= u;
            // v is now even, but u is unchanged (and odd)
    
            // Identity 1: gcd(u, 0) = u
            // The shift by k is necessary to add back the 2ᵏ factor that was removed before the loop
            if v == Self::ZERO {
                return u << k as u64;
            }
    
            // Identity 3: gcd(u, 2ʲ v) = gcd(u, v) (u is known to be odd)
            v = v >> v.trailing_zeros() as u64;
            // v is now odd again
        }
    }


    #[inline(never)]
    pub fn lcm(self, other: Self) -> Self {
        (self * other) / self.gcd_euclidean(other)
    }
}

#[cfg(test)]
mod tests {
    use num_integer::Integer;

    use crate::Uint;

    #[test]
    fn gcm_euclidean() {
        fn check(a: u128, b: u128) {
            let actual = a.gcd(&b);
            let mine = Uint::<4>::from_u128(a).gcd_euclidean(Uint::from_u128(b));
            assert_eq!(actual, mine.to_u128().unwrap());
        }

        check(1, 1);
        check(0, 0);
        check(1, 0);
        check(1024, 512);
        check(2183213, 59521);
        check(123, 821012);
        check(55, 912921);

        for _ in 0..1_000 {
            check(rand::random(), rand::random());
            check(rand::random::<u64>() as u128, rand::random());
            check(rand::random(), rand::random::<u64>() as u128);
        }
    }
}
