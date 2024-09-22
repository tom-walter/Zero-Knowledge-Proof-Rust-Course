use num_bigint::{BigUint, RandBigInt};
use rand;

/// a^x mod p
pub fn exponentiate(number: &BigUint, exponent: &BigUint, modulo: &BigUint) -> BigUint{
    number.modpow(exponent, modulo)
}

/// s = k - c * x mod q
pub fn solve(k: &BigUint, c: &BigUint, x: &BigUint, q: &BigUint) -> BigUint {
    let exponent1 = &BigUint::from(1u32);
    let cx = c * x;
    if *k >= cx {
        (k - c * x).modpow(exponent1, q)
    }
    else {
        q - (cx - k).modpow(exponent1, q)
    }
}

/// r1 = a^s y1^c
/// r2 = b^s y2^c
/// a, b are generators g
pub fn verify(g: &BigUint, s: &BigUint, y: &BigUint, c: &BigUint, r: &BigUint, p: &BigUint) -> bool {
    let exponent1 = &BigUint::from(1u32);
    let cond = *r == (g.modpow(s, p) * y.modpow(c, p)).modpow(exponent1, &p);
    cond
}

pub fn generate_random_below(bound: &BigUint) -> BigUint {
    let mut rng = rand::thread_rng();

    rng.gen_biguint_below(bound)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_toy_example() {
        let alpha = BigUint::from(4_u32);
        let beta = BigUint::from(9_u32);
        let prime = BigUint::from(23_u32);
        let order = BigUint::from(11_u32);

        let secret = BigUint::from(6_u32);
        let random_k = BigUint::from(7_u32);
        let random_c = BigUint::from(4_u32);

        let y_1 = exponentiate(&alpha, &secret, &prime);
        let y_2 = exponentiate(&beta, &secret, &prime);

        assert_eq!(y_1, BigUint::from(2_u32));
        assert_eq!(y_2, BigUint::from(3_u32));

        let r_1 = exponentiate(&alpha, &random_k, &prime);
        let r_2 = exponentiate(&beta, &random_k, &prime);

        assert_eq!(r_1, BigUint::from(8_u32));
        assert_eq!(r_2, BigUint::from(4_u32));

        let solution = solve(&random_k, &random_c, &secret, &order);
        assert_eq!(solution, BigUint::from(5_u32));

        let cond1 = verify(&alpha, &solution, &y_1, &random_c, &r_1, &prime);
        let cond2 = verify(&beta, &solution, &y_2, &random_c, &r_2, &prime);
        assert_eq!(cond1, true);
        assert_eq!(cond2, true);
    }

    #[test]
    fn test_toy_example_with_random_numbers() {
        let alpha = BigUint::from(4_u32);
        let beta = BigUint::from(9_u32);
        let prime = BigUint::from(23_u32);
        let order = BigUint::from(11_u32);

        let secret = BigUint::from(6_u32);
        let random_k = generate_random_below(&order);
        let random_c = generate_random_below(&order);

        let y_1 = exponentiate(&alpha, &secret, &prime);
        let y_2 = exponentiate(&beta, &secret, &prime);

        let r_1 = exponentiate(&alpha, &random_k, &prime);
        let r_2 = exponentiate(&beta, &random_k, &prime);

        let solution = solve(&random_k, &random_c, &secret, &order);

        let cond1 = verify(&alpha, &solution, &y_1, &random_c, &r_1, &prime);
        let cond2 = verify(&beta, &solution, &y_2, &random_c, &r_2, &prime);
        assert_eq!(cond1, true);
        assert_eq!(cond2, true);
    }
}