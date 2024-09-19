use crate::rounding::{
    count_h, make_hints_pv, power_2_round_q,
    use_hints_pv,
};
use crate::pack::{
    pack_delta, pack_pk, pack_sk, pack_w1, unpack_delta, unpack_pk, unpack_sk, unpack_t1,
};
use crate::params::{D, get_params, get_params_sign};
use crate::polyvec::polyvec::PolyVec;
use crate::sample::{expand_a, expand_mask, sample_in_ball};
use sha3::{
    digest::{ExtendableOutput, Update, XofReader},
    Shake256,
};
use rand::{thread_rng, Rng};

pub fn key_pair(security_level: u8) -> (Vec<u8>, Vec<u8>) {
    let (k, l, eta) = get_params(security_level);
    
    let mut rng = thread_rng();
    let seed: [u8; 32] = rng.gen();
    // use SHAKE256 to generate rho, rho' and K, whose length are 32, 64 and 32 bytes respectively
    let mut h = Shake256::default();
    h.update(&seed);
    h.update(&[k as u8, l as u8]);
    let mut reader = h.finalize_xof();
    let mut rho = [0u8; 32];
    let mut rhoprime = [0u8; 64];
    let mut kp = [0u8; 32];
    reader.read(&mut rho);
    reader.read(&mut rhoprime);
    reader.read(&mut kp);

    // use SHAKE256 to generate a random polynomial A (k*l polynomials)
    let mut a_hat = expand_a(rho, k, l);

    let mut s1 = PolyVec::new(l as usize);
    let mut s2 = PolyVec::new(k as usize);

    // gen s1, s2
    for i in 0..(k + l) as usize {
        if i < k as usize {
            s1.vec[i] = crate::sample::error_sample(rhoprime, i as u8, eta as u8);
        } else {
            s2.vec[i - k as usize] = crate::sample::error_sample(rhoprime, i as u8, eta as u8);
        }
    }

    // calculate t = NTT^-1(A_hat * NTT(s1))+s2
    let s1_hat = s1.ntt();
    let mut t = PolyVec::new(l as usize);
    for i in 0..k as usize {
        t.vec[i] = a_hat[i].pointwise_acc(&s1_hat).intt();
    }
    t = t.add(&s2);

    // calculate t1 and t0
    t.caddq();
    let (t1, t0) = power_2_round_q(t, D);

    // pack pk
    let pk = pack_pk(&t1, &rho);

    // get tr = H(pk, 64)
    h = Shake256::default();
    h.update(&pk);
    let mut reader = h.finalize_xof();
    let mut tr = [0u8; 64];
    reader.read(&mut tr);

    // pack sk
    let sk = pack_sk(&rho, &kp, &tr, &s1, &s2, &t0, eta as i32);

    (pk, sk)
}

pub fn sign(sk: &Vec<u8>, m: &Vec<u8>, security_level: u8) -> Vec<u8> {
    let (k, l, eta, gamma1, gamma2, tau, omega) = get_params_sign(2);

    let (rho, kp, tr, mut s1, mut s2, mut t0) = unpack_sk(sk, eta, k, l);

    s1 = s1.ntt();
    s2 = s2.ntt();
    t0 = t0.ntt();

    // use SHAKE256 to generate a random polynomial A (k*l polynomials)
    let mut a = expand_a(rho, k, l);

    let mut mu = [0u8; 64];
    let mut rhoprime = [0u8; 64];

    // mu = H(tr || m)
    let mut h = Shake256::default();
    h.update(&tr);
    h.update(&m);
    let mut reader = h.finalize_xof();
    reader.read(&mut mu);

    // rhoprime = H(K || rnd || mu)
    let mut rng = thread_rng();
    let rnd: [u8; 32] = rng.gen();
    h = Shake256::default();
    h.update(&kp);
    h.update(&rnd);
    h.update(&mu);
    reader = h.finalize_xof();
    reader.read(&mut rhoprime);

    let mut nonce = 0;
    let mut delta = vec![];
    let mut pass = false;
    let mut z = PolyVec::new(l as usize);
    // let mut hints = PolyVec::new(k as usize);
    
    while !pass {
       
        let mut y = PolyVec::new(l as usize);
        for i in 0..l as usize {
            y.set(i, expand_mask(rhoprime, nonce, i as i32, gamma1));
        }
        nonce += l;
        let y_hat = y.ntt();
        let mut w = PolyVec::new(k as usize);
        for i in 0..k as usize {
            w.vec[i] = a[i].pointwise_acc(&y_hat).intt();
        }
        w.caddq();
        let w1 = w.high_bits(gamma2);
        let w1_ba = pack_w1(&w1, gamma2, k);
        h = Shake256::default();
        h.update(&mu);
        h.update(&w1_ba);
        reader = h.finalize_xof();
        let mut cp = [0u8; 32];
        reader.read(&mut cp);
        let c = sample_in_ball(cp, tau).ntt();
	
        //  Compute z, reject if it reveals secret
        for i in 0..l as usize {
            z.vec[i] = c.point_wise_mul(&s1.vec[i]).intt();
            z.vec[i] = z.vec[i].add(&y.vec[i]);
        }
	
        if z.inf_norm() >= (gamma1 - tau * eta) {
            continue;
        }

        //Check that subtracting cs2 does not change high bits of w and low bitsdo not reveal secret information
        let w0 = w.low_bits(gamma2);
        let mut pv0 = PolyVec::new(w0.len); // record w - cs2
        for i in 0..k as usize {
            pv0.vec[i] = c.point_wise_mul(&s2.vec[i]).intt();
            pv0.vec[i] = w0.vec[i].sub(&pv0.vec[i]);
        }
        // let r0 = pv0.low_bits(gamma2);
        if pv0.inf_norm() >= (gamma2 - tau * eta) {
            continue;
        }

        // Compute hints for w1
        let mut pv1 = PolyVec::new(k as usize);
        for i in 0..k as usize {
            pv1.vec[i] = c.point_wise_mul(&t0.vec[i]).intt();
        }

        let hints = make_hints_pv(pv1.add(&pv0), pv1.neg(), gamma2);
        if pv1.inf_norm() >= gamma2 {
            continue;
        }
        let n = count_h(&hints);
        if n > omega {
            continue;
        }
        pass = true;
	
        delta = pack_delta(&cp, &z, &hints, security_level as i32, omega);
    }

    delta
}

pub fn verify(delta: &Vec<u8>, pk: &Vec<u8>, m: &Vec<u8>) -> bool {
    let (k, l, eta, gamma1, gamma2, tau, omega) = get_params_sign(2);
    let (rho, t1_ba) = unpack_pk(pk);

    let mut a = expand_a(rho, k, l);
    let mut h = Shake256::default();
    h.update(pk);
    let mut reader = h.finalize_xof();
    let mut tr = [0u8; 64];
    reader.read(&mut tr);
    h = Shake256::default();
    h.update(&tr);
    h.update(&m);
    reader = h.finalize_xof();
    let mut mu = [0u8; 64];
    reader.read(&mut mu);
    let (cp, z, hints) = unpack_delta(delta, l, k, omega);
    let c = sample_in_ball(cp, tau).ntt();
    let z_hat = z.ntt();
    let mut t1 = unpack_t1(&t1_ba, k);
    t1.left_shift(D as i32);
    t1 = t1.ntt();
    for i in 0..t1.len {
        t1.vec[i] = c.point_wise_mul(&t1.vec[i]).neg();
        t1.vec[i] = t1.vec[i].add(&(a[i].pointwise_acc(&z_hat))).intt();
    }
    t1.caddq();
    let w1 = use_hints_pv(&hints, &t1, gamma2);
    h = Shake256::default();
    h.update(&mu);
    h.update(&pack_w1(&w1, gamma2, k));
    reader = h.finalize_xof();
    let mut cp2 = [0u8; 32];
    reader.read(&mut cp2);
    (z.inf_norm() < gamma1 - tau * eta) && (cp2 == cp) && (count_h(&hints) <= omega)
}

#[cfg(test)]
mod test {
    use sha3::{
        digest::{ExtendableOutput, Update, XofReader},
        Shake256,
    };

    #[test]
    fn test_shake256() {
        let seed = [
            0x7a, 0x24, 0xb6, 0x66, 0xda, 0x34, 0x5c, 0x98, 0xc3, 0xa4, 0x0, 0xaa, 0xfd, 0x14,
            0xa5, 0x1a, 0x6c, 0x7, 0xd7, 0x48, 0xc6, 0xfc, 0x4, 0xfb, 0xd1, 0x30, 0x88, 0xed, 0x8b,
            0x33, 0x94, 0x8d,
        ];

        let out: [u8; 128] = [
            0xff, 0x7d, 0x49, 0x92, 0x84, 0xbe, 0xf2, 0x59, 0x65, 0x25, 0x63, 0x89, 0xb2, 0xa1,
            0x6, 0x1b, 0x30, 0x2, 0x39, 0xe6, 0xd, 0xc2, 0x99, 0x81, 0xf2, 0x7, 0xdf, 0x2, 0xc5,
            0xaa, 0xae, 0x3e, 0x57, 0x2, 0x34, 0x8d, 0xe6, 0x1, 0x9a, 0x89, 0x7f, 0x8e, 0x1, 0x50,
            0x91, 0x7d, 0x77, 0x3, 0xa9, 0xb9, 0x67, 0x83, 0xf2, 0x36, 0x3c, 0x5d, 0x92, 0xb5,
            0x8a, 0x6b, 0x28, 0x63, 0x93, 0xd6, 0xa9, 0xcb, 0x49, 0xcb, 0x3a, 0x6, 0x43, 0xf5,
            0x80, 0x4c, 0x92, 0xcd, 0x75, 0x34, 0xcb, 0xd6, 0x9d, 0x55, 0x25, 0x3d, 0x4a, 0x26,
            0xad, 0xfa, 0x77, 0xcf, 0x5b, 0x7d, 0x92, 0xae, 0x5b, 0x20, 0x31, 0x23, 0xe3, 0xb1,
            0xad, 0x1d, 0x1b, 0x73, 0xc, 0x24, 0x20, 0x4f, 0x6e, 0xb8, 0x93, 0x21, 0xb5, 0x3c, 0xe,
            0xb0, 0xa9, 0x72, 0xe6, 0x7f, 0x21, 0x9d, 0x49, 0xf8, 0xdb, 0x67, 0x79, 0x6b,
        ];

        let mut h = Shake256::default();
        h.update(&seed);
        let mut reader = h.finalize_xof();
        let mut rho = [0u8; 128];
        reader.read(&mut rho);
        println!("{:?}", rho);
        assert!(rho == out);
    }

   
    #[test]
    fn test_verify() {
        use super::*;
        let (pk, sk) = key_pair(2);
        let msg = [
            0xea, 0xcd, 0xc0, 0x82, 0x36, 0x1d, 0xe7, 0x10, 0x1b, 0x69, 0x6e, 0xe1, 0xa0, 0xa4,
            0xf3, 0x51, 0x4a, 0x65, 0xb6, 0xcf, 0xb3, 0x42, 0xb, 0xa4, 0x6a, 0x8d, 0x41, 0x10,
            0x2f, 0xdf, 0xa2, 0x47,
        ];
        let sig = sign(&sk, &msg.to_vec(), 2);
        assert!(verify(&sig, &pk, &msg.to_vec()));
    }
}
