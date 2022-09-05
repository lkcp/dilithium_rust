mod sign {

    use crate::params::{get_params, d};
    use crate::polyvec::polyvec::PolyVec;
    use sha3::{
        digest::{ExtendableOutput, Update, XofReader},
        Shake256,
    };
    use crate::pack::pack_pk;
    use crate::hints::power_2_round_q;

    fn key_gen(seed: [u8; 32], security_level: u8) -> (Vec<u8>, Vec<u8>) {
        let (k, l, eta) = get_params(security_level);
        // use SHAKE256 to generaterho, rho' and K, whose length are 32, 64 and 32 bytes respectively
        let mut H = Shake256::default();
        H.update(&seed);
        let mut reader = H.finalize_xof();
        let mut rho = [0u8; 32];
        let mut rhoprime = [0u8; 64];
        let mut K = [0u8; 32];
        reader.read(&mut rho);
        reader.read(&mut rhoprime);
        reader.read(&mut K);

        // use SHAKE256 to generate a random polynomial A (k*l polynomials)
        let mut A = Vec::new();
        for i in 0..k {
            A.push(PolyVec::new(l as usize));
        }

        let mut s1 = PolyVec::new(l as usize);
        let mut s2 = PolyVec::new(k as usize);


        // gen A
        for i in 0..k as usize {
            for j in 0..l as usize {
                A[i].vec[j] = crate::sample::reject_sample(rho, (i as u8 * l + j as u8) as u8);
            }
        }

        // gen s1, s2
        for i in 0..(k+l) as usize {
            if i < k as usize {
            s1.vec[i] = crate::sample::error_sample(rhoprime, i as u8, eta);
            } else {
                s2.vec[i-k as usize] = crate::sample::error_sample(rhoprime, i as u8, eta);
            }
        }
        
        // calculate t = NTT^-1(A_hat * s1_hat+s2_hat)
        s1.ntt();
        s2.ntt();
        let mut t = PolyVec::new(l as usize);
        for i in 0..k as usize {
            t.vec[i] = A[i].pointwise_acc(&s1);
            t.vec[i] = t.vec[i].add(&s2.vec[i]);
        }

        let (t1, t0) = power_2_round_q(t, d);
        let pk = pack_pk(&t1, k, &rho);

        // convert a vec<u8> to byte array
        let mut pk_bytes = [0u8; 122];
        for i in 0..pk.len() {
            pk_bytes[i] = pk[i];
        }

        H = Shake256::default();
        H.update(&pk_bytes);
        reader = H.finalize_xof();
        let mut tr = [0u8; 32];
        reader.read(&mut tr);

        let sk = Vec::new();

        (pk, sk)
    }

    #[cfg(test)]
    mod test {
        use super::*;
        
    }
}
