use crate::params::Q;
use crate::poly::Poly;
use crate::polyvec::polyvec::PolyVec;
use sha3::digest::{ExtendableOutput, Update, XofReader};
use sha3::Shake256;

// generate a polynomial with coefficients in Z_q
pub fn reject_sample(seed: [u8; 32], nonce: u8) -> Poly {
    let mut p = Poly::new();
    let mut H = Shake256::default();
    H.update(&seed);
    H.update(&[nonce]);
    let mut reader = H.finalize_xof();
    let mut buf = [0u8; 3];
    for i in 0..256 {
        loop {
            reader.read(&mut buf);
            let mut t = (buf[0] as i32) | ((buf[1] as i32) << 8) | ((buf[2] as i32) << 16);
            t &= 0x7FFFFF;
            if t < Q {
                p.coeffs[i] = t;
                break;
            }
        }
    }
    p
}

// byte = b0 || b1
// eta == 4: if b0 or b1 < 9 accept return eta - b0 eta - b1
// eta == 2: if b0 or b1 < 15 accept return eta - (b0 mod 5) eta - (b1 mod 5)
pub fn error_sample(seed: [u8; 64], nonce: u8, eta: u8) -> Poly {
    let mut p = Poly::new();
    let mut H = Shake256::default();
    H.update(&seed);
    H.update(&[nonce]);
    let mut reader = H.finalize_xof();
    let mut buf = [0u8; 1];
    let mut i: usize = 0;
    loop {
        reader.read(&mut buf);
        let t0 = buf[0] & 0x0F;
        let t1 = buf[0] >> 4;
        if eta == 4 {
            if t0 < 9 && i < 256 {
                p.coeffs[i] = eta as i32 - t0 as i32;
                i += 1;
            }
            if t1 < 9 && i < 256 {
                p.coeffs[i] = eta as i32 - t1 as i32;
                i += 1;
            }
        } else if eta == 2 {
            if t0 < 15 && i < 256 {
                p.coeffs[i] = eta as i32 - (t0 % 5) as i32;
                i += 1;
            }
            if t1 < 15 && i < 256 {
                p.coeffs[i] = eta as i32 - (t1 % 5) as i32;
                i += 1;
            }
            if i == 256 {
                break;
            }
        }
    }
    p
}
