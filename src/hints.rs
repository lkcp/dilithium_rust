use crate::polyvec::polyvec::PolyVec;


// a mod^+ Q = a1*2^D + a0 with -2^{D-1} < a0 <= 2^{D-1}.
pub fn power_2_round_q(t: PolyVec, d: u8) -> (PolyVec, PolyVec) {
    let mut t1 = PolyVec::new(t.vec.len());
    let mut t0 = PolyVec::new(t.vec.len());
    for i in 0..t.vec.len() {
        for j in 0..t.vec[i].coeffs.len() {
            // t0.vec[i].coeffs[j] = t.vec[i].coeffs[j] & ((1 << d)-1);
            // t1.vec[i].coeffs[j] = t.vec[i].coeffs[j] >> d;
            t1.vec[i].coeffs[j] = (t.vec[i].coeffs[j] + (1 << (d - 1))) >> d;
            t0.vec[i].coeffs[j] = t.vec[i].coeffs[j] - (t1.vec[i].coeffs[j] << d);
            // 这地方为啥这么实现没太明白
            // 比如第d位是0，那么说明低d位小于2^{d-1}，直接留下即可
            // 但如果第d位是1，那么低d位就是位于2^{d-1}到2^d之间，那t1加上的那个进位就刚好是2^{d-1}，这样就可以保证t0的范围是-2^{d-1}到2^{d-1}之间
            // 自然，也就保证了t1 = (t-t0)/2^d
        }
    }
    (t1, t0)
}