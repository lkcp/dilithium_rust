use crate::polyvec::polyvec::PolyVec;
use crate::params::d;

// pack the polyvec t1(coeffs is 10-bits) into byte arrays
// every 4 coeffs will be packed into 5 bytes
pub fn pack_pk(t1: &PolyVec, k: u8, rho: &[u8; 32]) -> Vec<u8> {
    let mut pk = Vec::new();
    pk.append(&mut Vec::from(*rho));
    for i in 0..t1.vec.len() {
        let mut j = 0;
        loop {
            // pack 4 coeffs into 5 bytes
            pk.push((t1.vec[i].coeffs[j] & 0xFF) as u8);  //8
            pk.push(
                (((t1.vec[i].coeffs[j] >> 8) & 0x03) | ((t1.vec[i].coeffs[j + 1]) & 0x3F) << 2)
                    as u8,
            ); // 2 6
            pk.push(
                ((((t1.vec[i].coeffs[j + 1]) >> 6) & 0x0F)
                    | (((t1.vec[i].coeffs[j + 2]) & 0x0F) << 4)) as u8,
            );  // 4 4
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

// pack to into byte array, coeffs in t0 have 13 bits(in [-2^12, 2^12], 12=d-1)
// pack 8 coeffs into 13 bytes
fn pack_t0(t0: &PolyVec) -> Vec<u8> {
    let mut buf:Vec<u8> = Vec::new();

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
            buf.push(
                (((a[0] >> 8) & 0x1F) | ((a[1] & 0x07) << 5)) as u8,
            );  // 5 3
            buf.push(
                (((a[1] >> 3) & 0xFF)) as u8,
            ); // 8
            buf.push(
                (((a[1] >> 11) & 0x03) | ((a[2] & 0x3F) << 2)) as u8,
            ); // 2 6

            buf.push(
                (((a[2] >> 6) & 0x7F) | ((a[3] & 0x01) << 7)) as u8,
            ); // 7 1
            buf.push(
                (((a[3] >> 1) & 0xFF)) as u8,
            ); // 8
            buf.push(
                (((a[3] >> 9) & 0x0F) | ((a[4] & 0x0F) << 4)) as u8,
            ); // 4 4
            buf.push(
                (((a[4] >> 4) & 0xFF)) as u8,
            ); // 8
            buf.push(
                (((a[4] >> 12) & 0x01) | ((a[5] & 0x7F) << 1)) as u8,
            ); // 1 7
            buf.push(
                (((a[5] >> 7) & 0x3F) | ((a[6] & 0x03) << 6)) as u8,
            ); // 6 2
            buf.push(
                (((a[6] >> 2) & 0xFF)) as u8,
            ); // 8
            buf.push(
                (((a[6] >> 10) & 0x07) | ((a[7] & 0x1F) << 3)) as u8,
            ); // 3 5
            buf.push(
                (((a[7] >> 5) & 0xFF)) as u8,
            ); // 8

            j += 8;
            if j == 256 {
                break;
            }
         }
    }


    buf
}