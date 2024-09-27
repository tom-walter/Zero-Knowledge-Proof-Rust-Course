use num_bigint::{BigUint, RandBigInt};
use rand::{self, Rng};

pub struct ZKP {
    pub prime: BigUint,
    pub order: BigUint,
    pub alpha: BigUint,
    pub beta: BigUint,
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

    pub fn generate_random_number(bound: &BigUint) -> BigUint {
        let mut rng = rand::thread_rng();

        rng.gen_biguint_below(bound)
    }

    pub fn generate_random_string(size: usize) -> String {
        rand::thread_rng()
            .sample_iter(rand::distributions::Alphanumeric)
            .take(size)
            .map(char::from)
            .collect()
    }

    /// returns (alpha, beta, p, q) of 1024-bit Diffie Hellman group
    pub fn get_constants() -> (BigUint, BigUint, BigUint, BigUint) {
        let p = hex::decode("B10B8F96A080E01DDE92DE5EAE5D54EC52C99FBCFB06A3C69A6A9DCA52D23B616073E28675A23D189838EF1E2EE652C013ECB4AEA906112324975C3CD49B83BFACCBDD7D90C4BD7098488E9C219A73724EFFD6FAE5644738FAA31A4FF55BCCC0A151AF5F0DC8B4BD45BF37DF365C1A65E68CFDA76D4DA708DF1FB2BC2E4A4371")
            .expect("could not decode hex");
        let p = BigUint::from_bytes_be(&p);

        let q = hex::decode("F518AA8781A8DF278ABA4E7D64B7CB9D49462353")
            .expect("could not decode hex");
        let q = BigUint::from_bytes_be(&q);

        let alpha = hex::decode("A4D1CBD5C3FD34126765A442EFB99905F8104DD258AC507FD6406CFF14266D31266FEA1E5C41564B777E690F5504F213160217B4B01B886A5E91547F9E2749F4D7FBD7D3B9A92EE1909D0D2263F80A76A6A24C087A091F531DBF0A0169B6A28AD662A4D18E73AFA32D779D5918D08BC8858F4DCEF97C2A24855E6EEB22B3B2E5")
            .expect("could not decode hex");
        let alpha = BigUint::from_bytes_be(&alpha);
        let beta = alpha.modpow(&ZKP::generate_random_number(&q), &p);

        (alpha, beta, p, q)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use hex;

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
        let random_k = ZKP::generate_random_number(&order);
        let random_c = ZKP::generate_random_number(&order);

        let y_1 = ZKP::exponentiate(&alpha, &secret, &prime);
        let y_2 = ZKP::exponentiate(&beta, &secret, &prime);

        let r_1 = ZKP::exponentiate(&alpha, &random_k, &prime);
        let r_2 = ZKP::exponentiate(&beta, &random_k, &prime);

        let solution = zkp.solve(&random_k, &random_c, &secret);

        let cond = zkp.verify(&r_1, &r_2, &y_1, &y_2, &random_c, &solution);
        assert!(cond)
    }

    #[test]
    fn test_toy_example_with_1024_bit_constants() {
        let p = hex::decode("B10B8F96A080E01DDE92DE5EAE5D54EC52C99FBCFB06A3C69A6A9DCA52D23B616073E28675A23D189838EF1E2EE652C013ECB4AEA906112324975C3CD49B83BFACCBDD7D90C4BD7098488E9C219A73724EFFD6FAE5644738FAA31A4FF55BCCC0A151AF5F0DC8B4BD45BF37DF365C1A65E68CFDA76D4DA708DF1FB2BC2E4A4371")
            .expect("could not decode hex");
        let p = BigUint::from_bytes_be(&p);

        let q = hex::decode("F518AA8781A8DF278ABA4E7D64B7CB9D49462353")
            .expect("could not decode hex");
        let q = BigUint::from_bytes_be(&q);

        let alpha = hex::decode("A4D1CBD5C3FD34126765A442EFB99905F8104DD258AC507FD6406CFF14266D31266FEA1E5C41564B777E690F5504F213160217B4B01B886A5E91547F9E2749F4D7FBD7D3B9A92EE1909D0D2263F80A76A6A24C087A091F531DBF0A0169B6A28AD662A4D18E73AFA32D779D5918D08BC8858F4DCEF97C2A24855E6EEB22B3B2E5")
            .expect("could not decode hex");
        let alpha = BigUint::from_bytes_be(&alpha);
        let beta = alpha.modpow(&ZKP::generate_random_number(&q), &p);

        let zkp = ZKP{
            prime: p.clone(),
            order: q.clone(),
            alpha: alpha.clone(),
            beta: beta.clone(),
        };

        let x = ZKP::generate_random_number(&q);
        let k = ZKP::generate_random_number(&q);
        let c = ZKP::generate_random_number(&q);

        let y_1 = ZKP::exponentiate(&alpha, &x, &p);
        let y_2 = ZKP::exponentiate(&beta, &x, &p);

        let r_1 = ZKP::exponentiate(&alpha, &k, &p);
        let r_2 = ZKP::exponentiate(&beta, &k, &p);
        
        let s = zkp.solve(&k, &c, &x);
        let cond = zkp.verify(&r_1, &r_2, &y_1, &y_2, &c, &s);
        assert!(cond)
    }
}