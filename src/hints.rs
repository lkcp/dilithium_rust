use crate::{polyvec::polyvec::PolyVec, params::Q};


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

fn decompose(r: i32, gamma2: i32) -> (i32, i32)
{
    let mut r1 = (r+17) >> 7;
    if gamma2 == 95232 {
        r1 = (r1*1025 + (1<<21)) >> 22;
        r1 &= 15;
    }
    else if gamma2 == 261888 {
        r1 = (r1*11275 + (1 << 23)) >> 24;
        r1 ^= ((43-r1) >> 31) & r1;
    }

    let mut r0 = r - r1*2*gamma2;
    r0 -= (((Q-1)/2 - r0) >> 31) & Q;
    return (r1, r0);
}

pub fn high_bits(r:i32, gamma2: i32) -> i32
{
    let (r1, r0) = decompose(r, gamma2);
    r1
}

pub fn low_bits(r: i32, gamma2: i32) -> i32
{
    let (r1, r0) = decompose(r, gamma2);
    r0
}

pub fn make_hints(r: i32, z:i32, gamma2: i32) -> i32 {
    if r > gamma2 || r < -gamma2 || (r == -gamma2 && z != 0) {return 1;}
    else {return 0;}
}

pub fn use_hints(h:i32, r:i32, gamma2: i32) -> i32 {
    let (r1, r0) = decompose(r, gamma2);
    if h == 0 {return r1;}

    if gamma2 == 95232 {
        if r0 > 0 {
            return match r1 == 43 {
                true => 0,
                false => r1 + 1,
            }
        }
        else {
            return match r1 == 0 {
                true => 43,
                false => r1 - 1,
            }
        }
    } // Q-1 / 88

    else if gamma2 == 261888 {
        if r0 > 0 {return (r1 + 1) & 15;}
        else {return (r1 - 1)&15;}
    } // Q-1 / 32
    
    else {
        panic!("gamma2 not supported");
    }
}