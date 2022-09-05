use crate::polyvec::polyvec::PolyVec;



pub fn power_2_round_q(t: PolyVec, d: u8) -> (PolyVec, PolyVec) {
    let mut t1 = PolyVec::new(t.vec.len());
    let mut t0 = PolyVec::new(t.vec.len());
    for i in 0..t.vec.len() {
        for j in 0..t.vec[i].coeffs.len() {;
            t0.vec[i].coeffs[j] = t.vec[i].coeffs[j] & ((1 << d)-1);
            t1.vec[i].coeffs[j] = t.vec[i].coeffs[j] >> d;
        }
    }
    (t1, t0)
}