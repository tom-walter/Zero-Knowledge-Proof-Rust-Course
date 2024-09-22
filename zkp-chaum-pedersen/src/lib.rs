use num_bigint::BigUint;

/// a^x mod p
pub fn exponentiate(number: &BigUint, exponent: &BigUint, modulo: &BigUint) -> BigUint{
    number.modpow(exponent, modulo)
}

/// s = k - c * x mod q
pub fn solve(k: &BigUint, c: &BigUint, x: &BigUint, q: &BigUint) -> BigUint {
    let exponent1 = &BigUint::from(1u32);
    if *k >= c * x {
        (k - c * x).modpow(exponent1, q)
    }
    else {
        q - (k - c * x).modpow(exponent1, q)
    }
}

/// r1 = a^s y1^c
/// r2 = b^s y2^c
/// a, b are generators g
pub fn verify(g: &BigUint, s: &BigUint, y: &BigUint, c: &BigUint, r: &BigUint, p: &BigUint) -> bool {
    let cond = *r == g.modpow(s, p) * y.modpow(c, p);
    cond
}