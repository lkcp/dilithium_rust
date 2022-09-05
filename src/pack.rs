use crate::polyvec::polyvec::PolyVec;



pub fn pack_t1_t0(t1: &PolyVec, t0: &PolyVec, d: u8) -> [u8; 122] {
    let mut c = [0u8; 122];
    let mut pos = 0;
    for i in 0..t1.vec.len() {
        for j in 0..t1.vec[i].coeffs.len() {
            let mut t = t1.vec[i].coeffs[j] << d;
            t = t | t0.vec[i].coeffs[j];
            c[pos] = (t & 0xff) as u8;
            c[pos+1] = ((t >> 8) & 0xff) as u8;
            c[pos+2] = ((t >> 16) & 0xff) as u8;
            pos += 3;
        }
    }
    c
}

// pack the polyvec t1(coeffs is 13-bits) into byte arrays
pub fn pack_pk(t1: &PolyVec, k:u8, rho: &[u8;32]) -> Vec<u8> {
    let mut pk = Vec::new();
    pk.append(&mut Vec::from(*rho));
    for i in 0..t1.vec.len() {
        let mut j = 0;
        loop {
            // pack 8 13-bit coefficients into 13 bytes
            pk.push((t1.vec[i].coeffs[j] & 0xff) as u8);


            j += 13
        }
    }
    pk
}