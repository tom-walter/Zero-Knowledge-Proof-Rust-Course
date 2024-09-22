use num_bigint::{BigUint, RandBigInt};
use rand;

pub struct ZKP {
    prime: BigUint,
    order: BigUint,
    alpha: BigUint,
    beta: BigUint,
}

impl ZKP {
    /// a^x mod p
    pub fn exponentiate(number: &BigUint, exponent: &BigUint, modulo: &BigUint) -> BigUint{
        number.modpow(exponent, modulo)
    }

    /// s = k - c * x mod q
    pub fn solve(&self, k: &BigUint, c: &BigUint, x: &BigUint) -> BigUint {
        let exponent1 = &BigUint::from(1u32);
        let cx = c * x;
        if *k >= cx {
            (k - cx).modpow(exponent1, &self.order)
        }
        else {
            &self.order - (cx - k).modpow(exponent1, &self.order)
        }
    }

    /// r1 = a^s y1^c
    /// r2 = b^s y2^c
    /// a, b are generators g
    pub fn verify(
        &self,
        r_1: &BigUint,
        r_2: &BigUint,
        y_1: &BigUint,
        y_2: &BigUint,
        c: &BigUint,
        s: &BigUint,
    ) -> bool {
        let exponent1 = &BigUint::from(1u32);
        let cond1 = *r_1
            == (self.alpha.modpow(s, &self.prime) * y_1.modpow(c, &self.prime)).modpow(exponent1, &self.prime);
        let cond2 = *r_2
            == (self.beta.modpow(s, &self.prime) * y_2.modpow(c, &self.prime)).modpow(exponent1, &self.prime);
        cond1 && cond2
    }

    pub fn generate_random_below(bound: &BigUint) -> BigUint {
        let mut rng = rand::thread_rng();

        rng.gen_biguint_below(bound)
    }
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
        let zkp = ZKP{ 
            prime: prime.clone(),
            order,
            alpha: alpha.clone(),
            beta: beta.clone(),
        }; 

        let secret = BigUint::from(6_u32);
        let random_k = BigUint::from(7_u32);
        let random_c = BigUint::from(4_u32);

        let y_1 = ZKP::exponentiate(&alpha, &secret, &prime);
        let y_2 = ZKP::exponentiate(&beta, &secret, &prime);

        assert_eq!(y_1, BigUint::from(2_u32));
        assert_eq!(y_2, BigUint::from(3_u32));

        let r_1 = ZKP::exponentiate(&alpha, &random_k, &prime);
        let r_2 = ZKP::exponentiate(&beta, &random_k, &prime);

        assert_eq!(r_1, BigUint::from(8_u32));
        assert_eq!(r_2, BigUint::from(4_u32));

        let solution = zkp.solve(&random_k, &random_c, &secret);
        assert_eq!(solution, BigUint::from(5_u32));

        let cond = zkp.verify(&r_1, &r_2, &y_1, &y_2, &random_c, &solution);
        assert!(cond)
    }

    #[test]
    fn test_toy_example_with_random_numbers() {
        let alpha = BigUint::from(4_u32);
        let beta = BigUint::from(9_u32);
        let prime = BigUint::from(23_u32);
        let order = BigUint::from(11_u32);
        let zkp = ZKP{ 
            prime: prime.clone(),
            order: order.clone(),
            alpha: alpha.clone(),
            beta: beta.clone(),
        }; 

        let secret = BigUint::from(6_u32);
        let random_k = ZKP::generate_random_below(&order);
        let random_c = ZKP::generate_random_below(&order);

        let y_1 = ZKP::exponentiate(&alpha, &secret, &prime);
        let y_2 = ZKP::exponentiate(&beta, &secret, &prime);

        let r_1 = ZKP::exponentiate(&alpha, &random_k, &prime);
        let r_2 = ZKP::exponentiate(&beta, &random_k, &prime);

        let solution = zkp.solve(&random_k, &random_c, &secret);

        let cond = zkp.verify(&r_1, &r_2, &y_1, &y_2, &random_c, &solution);
        assert!(cond)
    }
}