use crate::polyvec::polyvec::PolyVec;


// pack the polyvec t1(coeffs is 10-bits) into byte arrays
// every 4 coeffs will be packed into 5 bytes
pub fn pack_pk(t1: &PolyVec, k:u8, rho: &[u8;32]) -> Vec<u8> {
    let mut pk = Vec::new();
    pk.append(&mut Vec::from(*rho));
    for i in 0..t1.vec.len() {
        let mut j = 0;
        loop {
            // pack 4 coeffs into 5 bytes
            pk.push((t1.vec[i].coeffs[j] & 0xFF) as u8);
            pk.push((((t1.vec[i].coeffs[j] >> 8) & 0x03) | ((t1.vec[i].coeffs[j+1]) & 0x3F) << 2) as u8);
            pk.push(((((t1.vec[i].coeffs[j+1]) >> 6) & 0x0F) | (((t1.vec[i].coeffs[j+2]) & 0x0F) << 4)) as u8);
            pk.push((((t1.vec[i].coeffs[j+2] >> 4) & 0x0F) | ((t1.vec[i].coeffs[j+3] & 0x03) << 6)) as u8);
            pk.push(((t1.vec[i].coeffs[j+3] >> 2) & 0xFF) as u8);
            j += 4;
        }
    }
    pk
}