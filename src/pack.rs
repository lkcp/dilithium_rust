use std::sync::BarrierWaitResult;

use crate::params::d;
use crate::poly::Poly;
use crate::polyvec::polyvec::PolyVec;

// pack the polyvec t1(coeffs is 10-bits) into byte arrays
// every 4 coeffs will be packed into 5 bytes
pub fn pack_pk(t1: &PolyVec, k: u8, rho: &[u8; 32]) -> Vec<u8> {
    let mut pk = Vec::new();
    pk.append(&mut Vec::from(*rho));
    for i in 0..t1.vec.len() {
        let mut j = 0;
        loop {
            // pack 4 coeffs into 5 bytes
            pk.push((t1.vec[i].coeffs[j] & 0xFF) as u8); //8
            pk.push(
                (((t1.vec[i].coeffs[j] >> 8) & 0x03) | ((t1.vec[i].coeffs[j + 1]) & 0x3F) << 2)
                    as u8,
            ); // 2 6
            pk.push(
                ((((t1.vec[i].coeffs[j + 1]) >> 6) & 0x0F)
                    | (((t1.vec[i].coeffs[j + 2]) & 0x0F) << 4)) as u8,
            ); // 4 4
            pk.push(
                (((t1.vec[i].coeffs[j + 2] >> 4) & 0x3F) | ((t1.vec[i].coeffs[j + 3] & 0x03) << 6))
                    as u8,
            ); // 6 2
            pk.push(((t1.vec[i].coeffs[j + 3] >> 2) & 0xFF) as u8); // 8
            j += 4;
            if j == 256 {
                break;
            }
        }
    }
    pk
}

// unpack pk into t1 and rho
// l = 4/5/7
// unpack 5 bytes into 4 coeffs, 320 bytes into 256 coeffs(1 poly), total k polys
pub fn unpack_pk(pk: Vec<u8>, l: i32) -> ([u8; 32], PolyVec) {
    let rho = pk[0..32].try_into().unwrap();
    let mut t1 = PolyVec::new(l as usize);
    for i in 0..l as usize {
        let mut j = 0;
        loop {
            t1.vec[i].coeffs[j*4] = (pk[32 + i * 320 + j * 5] as i32)
                | ((pk[32 + i * 320 + j * 5 + 1] as u16 & 0x03) << 8) as i32; // 8 2
            t1.vec[i].coeffs[j*4 + 1] = ((pk[32 + i * 320 + j * 5 + 1] >> 2) & 0x3F) as i32
                | ((pk[32 + i * 320 + j * 5 + 2] as u16 & 0x0F) << 6) as i32; // 6 4
            t1.vec[i].coeffs[j*4 + 2] = ((pk[32 + i * 320 + j * 5 + 2] >> 4) & 0x0F) as i32
                | ((pk[32 + i * 320 + j * 5 + 3] as u16 & 0x3F) << 4) as i32; // 4 6
            t1.vec[i].coeffs[j*4 + 3] = ((pk[32 + i * 320 + j * 5 + 3] >> 6) & 0x03) as i32
                | ((pk[32 + i * 320 + j * 5 + 4] as u16) << 2) as i32; // 2 8
            j += 1;
            if j*4 == 256 {
                break;
            }
        }
    }

    (rho, t1)
}

pub fn pack_sk(
    rho: &[u8; 32],
    K: &[u8; 32],
    tr: &[u8; 32],
    s1: &PolyVec,
    s2: &PolyVec,
    t0: &PolyVec,
    eta: i32,
) -> Vec<u8> {
    let mut sk = Vec::new();
    sk.append(&mut Vec::from(*rho));
    sk.append(&mut Vec::from(*K));
    sk.append(&mut Vec::from(*tr));
    sk.append(&mut pack_eta(eta, s1));
    sk.append(&mut pack_eta(eta, s2));
    sk.append(&mut pack_t0(t0));
    sk
}

pub fn unpack_sk(
    sk: Vec<u8>,
    eta: i32,
    k: i32,
    l: i32,
) -> ([u8; 32], [u8; 32], [u8; 32], PolyVec, PolyVec, PolyVec) {
    let rho = sk[0..32].try_into().unwrap();
    let K = sk[32..64].try_into().unwrap();
    let tr = sk[64..96].try_into().unwrap();
    let (s1, s2) = unpack_eta(eta, k , l, sk[96..sk.len() - 416*k as usize].to_vec());
    let t0 = unpack_t0(k, sk[sk.len()-416*k as usize..].to_vec());
    (rho, K, tr, s1, s2, t0)
}

// pack s1 and s2 into byte arrays for different eta
fn pack_eta(eta: i32, t: &PolyVec) -> Vec<u8> {
    let mut buf = Vec::new();

    for i in 0..t.vec.len() {
        let mut j: usize = 0;
        loop {
            // coeff is in [-eta, eta]
            if eta == 2 {
                // pack 8 coeffs into 3 bytes
                let mut a = [0; 8];
                a[0] = eta - t.vec[i].coeffs[j];
                a[1] = eta - t.vec[i].coeffs[j + 1];
                a[2] = eta - t.vec[i].coeffs[j + 2];
                a[3] = eta - t.vec[i].coeffs[j + 3];
                a[4] = eta - t.vec[i].coeffs[j + 4];
                a[5] = eta - t.vec[i].coeffs[j + 5];
                a[6] = eta - t.vec[i].coeffs[j + 6];
                a[7] = eta - t.vec[i].coeffs[j + 7];
                buf.push((a[0] >> 0) as u8 | (a[1] << 3) as u8 | (a[2] << 6) as u8); // 3 3 2
                buf.push(
                    (a[2] >> 2) as u8 | (a[3] << 1) as u8 | (a[4] << 4) as u8 | (a[5] << 7) as u8,
                ); // 1 3 3 1
                buf.push((a[5] >> 1) as u8 | (a[6] << 2) as u8 | (a[7] << 5) as u8); // 2 3 3
                j += 8;
            } else if eta == 4 {
                // pack 2 coeffs into 1 byte
                let mut a = [0, 2];
                a[0] = eta - t.vec[i].coeffs[j];
                a[1] = eta - t.vec[i].coeffs[j + 1];
                buf.push(a[0] as u8 | (a[1] << 4) as u8);
                j += 2;
            } else {
                panic!("eta should be 2 or 4");
            }
            if j == 256 {
                break;
            }
        }
    }

    buf
}

// unpack s1 and s2 from byte arrays for different eta
// eta = 2: 3 bytes into 8 coeffs, 96 bytes into 256 coeffs(1 poly), s1 l polys, s2 k polys
fn unpack_eta(eta: i32, k: i32, l:i32, ba: Vec<u8>) -> (PolyVec, PolyVec) {
    // let l = match k {
    //     4 => 4,
    //     6 => 5,
    //     8 => 7,
    // };
    let mut s1 = PolyVec::new(l as usize);
    let mut s2 = PolyVec::new(k as usize);

    for i in 0..l as usize {
        let mut j = 0;
        loop {
            if eta == 2 {
                s1.vec[i].coeffs[j*8] = (ba[i * 96 + j * 3] & 0x07) as i32; // 3
                s1.vec[i].coeffs[j*8 + 1] = ((ba[i * 96 + j * 3] >> 3) & 0x07) as i32; // 3
                s1.vec[i].coeffs[j*8 + 2] = ((ba[i * 96 + j * 3] >> 6) & 0x03) as i32
                    | ((ba[i * 96 + j * 3 + 1] & 0x01) << 2) as i32; // 2 1
                s1.vec[i].coeffs[j*8 + 3] = ((ba[i * 96 + j * 3 + 1] >> 1) & 0x07) as i32; // 3
                s1.vec[i].coeffs[j*8 + 4] = ((ba[i * 96 + j * 3 + 1] >> 4) & 0x07) as i32; // 3
                s1.vec[i].coeffs[j*8 + 5] = ((ba[i * 96 + j * 3 + 1] >> 7) & 0x01) as i32
                    | ((ba[i * 96 + j * 3 + 2] & 0x03) << 1) as i32; // 1 2
                s1.vec[i].coeffs[j*8 + 6] = ((ba[i * 96 + j * 3 + 2] >> 2) & 0x07) as i32; // 3
                s1.vec[i].coeffs[j*8 + 7] = ((ba[i * 96 + j * 3 + 2] >> 5) & 0x07) as i32; // 3

                s1.vec[i].coeffs[j*8] = eta - s1.vec[i].coeffs[j*8];
                s1.vec[i].coeffs[j*8 + 1] = eta - s1.vec[i].coeffs[j*8 + 1];
                s1.vec[i].coeffs[j*8 + 2] = eta - s1.vec[i].coeffs[j*8 + 2];
                s1.vec[i].coeffs[j*8 + 3] = eta - s1.vec[i].coeffs[j*8 + 3];
                s1.vec[i].coeffs[j*8 + 4] = eta - s1.vec[i].coeffs[j*8 + 4];
                s1.vec[i].coeffs[j*8 + 5] = eta - s1.vec[i].coeffs[j*8 + 5];
                s1.vec[i].coeffs[j*8 + 6] = eta - s1.vec[i].coeffs[j*8 + 6];
                s1.vec[i].coeffs[j*8 + 7] = eta - s1.vec[i].coeffs[j*8 + 7];

                j += 1;
                if j*8 == 256 {
                    break;
                }
            } else if eta == 4 {
                s1.vec[i].coeffs[j*2] = (ba[i * 32 + j * 1] & 0x0F) as i32; // 4
                s1.vec[i].coeffs[j*2 + 1] = ((ba[i * 32 + j * 1] >> 4) & 0x0F) as i32; // 4

                s1.vec[i].coeffs[j*2] = eta - s1.vec[i].coeffs[j*2];
                s1.vec[i].coeffs[j*2 + 1] = eta - s1.vec[i].coeffs[j*2 + 1];

                j += 1;
                if j*2 == 256 {
                    break;
                }
            } else {
                panic!("eta should be 2 or 4");
            }
        }
    }

    (s1, s2)
}

// pack to into byte array, coeffs in t0 have 13 bits(in [-2^12, 2^12], 12=d-1)
// pack 8 coeffs into 13 bytes
fn pack_t0(t0: &PolyVec) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();

    for i in 0..t0.vec.len() {
        let mut j = 0;
        loop {
            let mut a = [0; 8];
            // a = (1 << (d-1)) - t0.vec[i].coeffs[j];
            a[0] = (1 << (d - 1)) - t0.vec[i].coeffs[j];
            a[1] = (1 << (d - 1)) - t0.vec[i].coeffs[j + 1];
            a[2] = (1 << (d - 1)) - t0.vec[i].coeffs[j + 2];
            a[3] = (1 << (d - 1)) - t0.vec[i].coeffs[j + 3];
            a[4] = (1 << (d - 1)) - t0.vec[i].coeffs[j + 4];
            a[5] = (1 << (d - 1)) - t0.vec[i].coeffs[j + 5];
            a[6] = (1 << (d - 1)) - t0.vec[i].coeffs[j + 6];
            a[7] = (1 << (d - 1)) - t0.vec[i].coeffs[j + 7];

            buf.push((a[0] & 0xFF) as u8); // 8
            buf.push((((a[0] >> 8) & 0x1F) | ((a[1] & 0x07) << 5)) as u8); // 5 3
            buf.push(((a[1] >> 3) & 0xFF) as u8); // 8
            buf.push((((a[1] >> 11) & 0x03) | ((a[2] & 0x3F) << 2)) as u8); // 2 6

            buf.push((((a[2] >> 6) & 0x7F) | ((a[3] & 0x01) << 7)) as u8); // 7 1
            buf.push(((a[3] >> 1) & 0xFF) as u8); // 8
            buf.push((((a[3] >> 9) & 0x0F) | ((a[4] & 0x0F) << 4)) as u8); // 4 4
            buf.push(((a[4] >> 4) & 0xFF) as u8); // 8
            buf.push((((a[4] >> 12) & 0x01) | ((a[5] & 0x7F) << 1)) as u8); // 1 7
            buf.push((((a[5] >> 7) & 0x3F) | ((a[6] & 0x03) << 6)) as u8); // 6 2
            buf.push(((a[6] >> 2) & 0xFF) as u8); // 8
            buf.push((((a[6] >> 10) & 0x07) | ((a[7] & 0x1F) << 3)) as u8); // 3 5
            buf.push(((a[7] >> 5) & 0xFF) as u8); // 8

            j += 8;
            if j == 256 {
                break;
            }
        }
    }

    buf
}

// unpack t0 from byte array, 13 bytes -> 8 coeffs, 13*32=416 bytes for 1 poly
// t0 has k polys
fn unpack_t0(k: i32, ba: Vec<u8>) -> PolyVec {
    let mut t0 = PolyVec::new(k as usize);
    for i in 0..k as usize {
        let mut j = 0;
        loop {
            let mut a = [0; 8];
            t0.vec[i].coeffs[j*8] = (ba[i as usize * 416 + j * 13] as i32)
                | ((ba[i as usize * 416 + j * 13 + 1] as i32 & 0x1F) << 8); // 8 5
            t0.vec[i].coeffs[j*8+1] = ((ba[i as usize * 416 + j * 13 + 1] as i32 >> 5) & 0x07)
                | (ba[i as usize * 416 + j * 13 + 2] << 3) as i32
                | ((ba[i as usize * 416 + j * 13 + 3] as i32 & 0x03) << 11); // 3 8 2
            t0.vec[i].coeffs[j*8+2] = ((ba[i as usize * 416 + j * 13 + 3] as i32 >> 2) & 0x3F)
                | ((ba[i as usize * 416 + j * 13 + 4] as i32 & 0x7F) << 6); // 6 7
            t0.vec[i].coeffs[j*8+3] = ((ba[i as usize * 416 + j * 13 + 4] as i32 >> 7) & 0x01)
                | ((ba[i as usize * 416 + j * 13 + 5] as i32 & 0xFF) << 1)
                | ((ba[i as usize * 416 + j * 13 + 6] as i32 & 0x0F) << 9); // 1 8 4
            t0.vec[i].coeffs[j*8+4] = ((ba[i as usize * 416 + j * 13 + 6] as i32 >> 4) & 0x0F)
                | ((ba[i as usize * 416 + j * 13 + 7] as i32 & 0xFF) << 4)
                | ((ba[i as usize * 416 + j * 13 + 8] as i32 & 0x01) << 12); // 4 8 1
            t0.vec[i].coeffs[j*8+5] = ((ba[i as usize * 416 + j * 13 + 8] as i32 >> 1) & 0x7F)
                | ((ba[i as usize * 416 + j * 13 + 9] as i32 & 0x3F) << 7); // 7 6
            t0.vec[i].coeffs[j*8+6] = ((ba[i as usize * 416 + j * 13 + 9] as i32 >> 6) & 0x03)
                | ((ba[i as usize * 416 + j * 13 + 10] as i32 & 0xFF) << 2)
                | ((ba[i as usize * 416 + j * 13 + 11] as i32 & 0x07) << 10); // 2 8 3
            t0.vec[i].coeffs[j*8+7] = ((ba[i as usize * 416 + j * 13 + 11] as i32 >> 3) & 0x1F)
                | ((ba[i as usize * 416 + j * 13 + 12] as i32 & 0xFF) << 5); // 5 8

            t0.vec[i].coeffs[j*8] = (1 << (d-1)) - t0.vec[i].coeffs[j];
            t0.vec[i].coeffs[j*8+1] = (1 << (d-1)) - t0.vec[i].coeffs[j+1];
            t0.vec[i].coeffs[j*8+2] = (1 << (d-1)) - t0.vec[i].coeffs[j+2];
            t0.vec[i].coeffs[j*8+3] = (1 << (d-1)) - t0.vec[i].coeffs[j+3];
            t0.vec[i].coeffs[j*8+4] = (1 << (d-1)) - t0.vec[i].coeffs[j+4];
            t0.vec[i].coeffs[j*8+5] = (1 << (d-1)) - t0.vec[i].coeffs[j+5];
            t0.vec[i].coeffs[j*8+6] = (1 << (d-1)) - t0.vec[i].coeffs[j+6];
            t0.vec[i].coeffs[j*8+7] = (1 << (d-1)) - t0.vec[i].coeffs[j+7];

            j += 1;
            if j*8 == 256 {
                break;
            }
        }
    }
    t0
}


pub fn unpack_y(gamma1: i32, ba: Vec<u8>) -> Poly {
    let mut y = Poly::new();
    let mut i = 0;

    // coeff has 18 bits when gamma1 == 1 << 17
    // unpack 9 bytes -> 4 coeffs, 9*64=576 bytes for 1 poly
    if gamma1 == 1 << 17 {
        loop {
            y.coeffs[i*4] = (ba[i * 9] as i32)
                | ((ba[i * 9 + 1] << 8) as i32) | ((ba[i*9+2] & 0x03) << 16) as i32; // 8 8 2
            y.coeffs[i*4+1] = ((ba[i * 9 + 2] >> 2) & 0x3F) as i32 | ((ba[i * 9 + 3] << 6) as i32)
                | (((ba[i * 9 + 4] & 0xF) << 14) as i32); // 6 8 4
            y.coeffs[i*4+2] = ((ba[i * 9 + 4] >> 4) & 0x0F) as i32 | ((ba[i * 9 + 5] << 4) as i32) | ((ba[i*9+6] & 0x3F) << 12) as i32; // 4 8 6
            y.coeffs[i*4+3] = ((ba[i * 9 + 6] >> 6) & 0x03) as i32 | ((ba[i * 9 + 7] << 2) as i32) | ((ba[i*9+8] & 0xFF) << 10) as i32; // 2 8 8

            y.coeffs[i*4] = gamma1 - y.coeffs[i];
            y.coeffs[i*4+1] = gamma1 - y.coeffs[i+1];
            y.coeffs[i*4+2] = gamma1 - y.coeffs[i+2];
            y.coeffs[i*4+3] = gamma1 - y.coeffs[i+3];

            i += 1;
            if i*4 == 256 {
                break;
            }
        }
    }

    // coeffs has 20 bits when gamma1 == 1 << 19
    // unpack 5 bytes -> 2 coeffs, 5*128=640 bytes for 1 poly
    else if gamma1 == 1<< 19 {
        loop {
            y.coeffs[i*2] = ba[i*5] as i32 | (ba[i*5+1] << 8) as i32 | ((ba[i*5+2] & 0x0F) << 16) as i32; // 8 8 4
            y.coeffs[i*2+1] = ((ba[i*5+2] >> 4) & 0x0F) as i32 | (ba[i*5+3] << 4) as i32 | (ba[i*5+4] << 12) as i32; // 4 8 8
            
            y.coeffs[i*2] = gamma1 - y.coeffs[i];
            y.coeffs[i*2+1] = gamma1 - y.coeffs[i+1];

            i += 1;
            if i*2 == 256 {
                break;
            }
        }

    }

    else {
        panic!("gamma1 is not 2^17 or 2^19");
    }

    y
}

pub fn pack_w1(w1: PolyVec, gamma1: i32, k: i32) -> Vec<u8> {
    // coeff of w1 is in [0, 43], takes 6 bits, k*256*6/8
    // 4 coeffs into 3 bytes
    if gamma1 == 95232 {
        let mut buf = vec![0u8; 192*k as usize];
        for i in 0..w1.len {
            let mut j = 0;
            loop {
                buf[i*192+j*3] = w1.vec[i].coeffs[j*4] as u8 | ((w1.vec[i].coeffs[j*4+1] & 0x03) << 6) as u8; // 6 2
                buf[i*192+j*3+1] = ((w1.vec[i].coeffs[j*4+1] >> 2) & 0x0F) as u8 | ((w1.vec[i].coeffs[j*4+2] & 0x0F) << 4) as u8; // 4 4
                buf[i*192+j*3+2] = ((w1.vec[i].coeffs[j*4+2] >> 4) & 0x03) as u8 | (w1.vec[i].coeffs[j*4+3] << 2) as u8; // 2 6 

                j += 4;
                if j == 256 {
                    break;
                }
            }
        }

        buf
    }
     // coeff of w1 is in [0, 15], takes 4 bits, k*256*4/8
    else if gamma1 == 261888 {
        let mut buf = vec![0u8; 128*k as usize];
        for i in 0..w1.len {
            let mut j = 0;
            loop {
                buf[i*128+j] = w1.vec[i].coeffs[j*2] as u8 | (w1.vec[i].coeffs[j*2+1] << 4) as u8;
                j += 2;
                if j == 256 {
                    break;
                }
            }
        }
        buf
    }
    else {
        panic!("ga");
    }
}