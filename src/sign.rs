mod sign {

    use crate::hints::power_2_round_q;
    use crate::pack::{pack_pk, pack_sk};
    use crate::params::{get_params, d};
    use crate::polyvec::polyvec::PolyVec;
    use sha3::{
        digest::{ExtendableOutput, Update, XofReader},
        Shake256,
    };
    use crate::pack::pack_pk;
    use crate::hints::power_2_round_q;

    fn key_pair(seed: [u8; 32], security_level: u8) -> (Vec<u8>, Vec<u8>) {
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

        // genA
        for i in 0..k as usize {
            for j in 0..l as usize {
                A[i].vec[j] = crate::sample::reject_sample(rho, i as u8, j as u8);
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
        let mut t = PolyVec::new(l as usize);
        for i in 0..k as usize {
            t.vec[i] = A[i].pointwise_acc(&s1);
        }
        t.add(&s2);
        t.intt();

        // calculate t1 and t0
        let (t1, t0) = power_2_round_q(t, d);

        // pack pk
        let pk = pack_pk(&t1, k, &rho);

        // get tr
        H = Shake256::default();
        H.update(&pk);
        let mut reader = H.finalize_xof();
        let mut tr = [0u8; 32];
        reader.read(&mut tr);

        // pack sk
        let sk = pack_sk(&rho, &K, &tr, &s1, &s2, &t0, eta as i32);

        (pk, sk)
    }

    #[cfg(test)]
    mod test {
        use sha3::{Shake256, digest::{Update, ExtendableOutput, XofReader}};


        #[test]
        fn test_shake256() {
            let seed = [0x7a, 0x24, 0xb6, 0x66, 0xda, 0x34, 0x5c, 0x98, 0xc3, 0xa4, 0x0, 0xaa, 0xfd, 0x14, 0xa5, 0x1a, 0x6c, 0x7, 0xd7, 0x48, 0xc6, 0xfc, 0x4, 0xfb, 0xd1, 0x30, 0x88, 0xed, 0x8b, 0x33, 0x94, 0x8d];

            let out:[u8; 128] = [0xff, 0x7d, 0x49, 0x92, 0x84, 0xbe, 0xf2, 0x59, 0x65, 0x25, 0x63, 0x89, 0xb2, 0xa1, 0x6, 0x1b, 0x30, 0x2, 0x39, 0xe6, 0xd, 0xc2, 0x99, 0x81, 0xf2, 0x7, 0xdf, 0x2, 0xc5, 0xaa, 0xae, 0x3e, 0x57, 0x2, 0x34, 0x8d, 0xe6, 0x1, 0x9a, 0x89, 0x7f, 0x8e, 0x1, 0x50, 0x91, 0x7d, 0x77, 0x3, 0xa9, 0xb9, 0x67, 0x83, 0xf2, 0x36, 0x3c, 0x5d, 0x92, 0xb5, 0x8a, 0x6b, 0x28, 0x63, 0x93, 0xd6, 0xa9, 0xcb, 0x49, 0xcb, 0x3a, 0x6, 0x43, 0xf5, 0x80, 0x4c, 0x92, 0xcd, 0x75, 0x34, 0xcb, 0xd6, 0x9d, 0x55, 0x25, 0x3d, 0x4a, 0x26, 0xad, 0xfa, 0x77, 0xcf, 0x5b, 0x7d, 0x92, 0xae, 0x5b, 0x20, 0x31, 0x23, 0xe3, 0xb1, 0xad, 0x1d, 0x1b, 0x73, 0xc, 0x24, 0x20, 0x4f, 0x6e, 0xb8, 0x93, 0x21, 0xb5, 0x3c, 0xe, 0xb0, 0xa9, 0x72, 0xe6, 0x7f, 0x21, 0x9d, 0x49, 0xf8, 0xdb, 0x67, 0x79, 0x6b];

            let mut H = Shake256::default();
            H.update(&seed);
            let mut reader = H.finalize_xof();
            let mut rho = [0u8; 128];
            reader.read(&mut rho);
            println!("{:?}", rho);
            assert!(rho == out);
        }

        #[test]
        fn test_key_gen() {
            let seed = [0x9f, 0xd9, 0xaa, 0xfd, 0x8f, 0xc9, 0x01, 0xf5, 0x00, 0x85, 0xde, 0x82, 0x68, 0xc2, 0xd6, 0x30, 0x26, 0xdd, 0x8e, 0x35, 0xf8, 0x9d, 0xd1, 0xe2, 0xbc, 0x15, 0x1d, 0x7d, 0x20, 0xd0, 0x97, 0x96];

            let security_level = 2;
            let (pk, sk) = super::key_pair(seed, security_level);
            let mut hpk = [0u8; 32];
            let mut H = Shake256::default();
            H.update(&pk);
            let mut reader = H.finalize_xof();
            reader.read(&mut hpk);
            println!("{:?}", hpk);
            // println!("{:?}", sk);
            assert_eq!(pk.len(), 122);
        }
    }
}
