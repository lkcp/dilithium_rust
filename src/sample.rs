use crate::pack::unpack_y;
use crate::params::Q;
use crate::poly::Poly;
use crate::polyvec::polyvec::PolyVec;
use sha3::digest::{ExtendableOutput, Update, XofReader};
use sha3::{Shake256, Shake128};

// generate a polynomial with coefficients in Z_q
pub fn reject_sample(seed: [u8; 32], i: u8, j: u8) -> Poly {
    let mut p = Poly::new();
    let mut H = Shake128::default();
    H.update(&seed);
    H.update(&[j, i]);
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
    H.update(&[nonce, 0]);
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


pub fn expand_mask(rhoprime: [u8; 64], nonce: i32, i: i32, gamma1: i32) -> Poly {
    let mut y = Poly::new();

    let mut H = Shake256::default();
    H.update(&rhoprime);
    H.update(&[(nonce+i) as u8, ((nonce+i) >> 8) as u8]);
    let mut reader = H.finalize_xof();

    if gamma1 == 1 << 17 {
        let mut buf = [0u8; 576];
        reader.read(&mut buf);
        y = unpack_y(gamma1, (&buf).to_vec());
    }
    else if gamma1 == 1 << 19 {
        let mut buf = [0u8; 640];
        reader.read(&mut buf);
        y = unpack_y(gamma1, (&buf).to_vec());
    } 
    else {
        panic!("gamma1 not supported");
    }
    y
}

// return a poly with \tau 1/-1's and 256-\tau 0's
pub fn sample_in_ball(cp: [u8; 32], tau: i32) -> Poly {
    let mut c = Poly::new();
    let mut H = Shake128::default();
    H.update(&cp);
    let mut reader = H.finalize_xof();
    let mut buf1 = [0u8; 8];
    let mut buf2 = [0u8; 1];
    // the first 8 bytes are used to generate the \tau signs, the rest 64-\tau is discarded
    reader.read(&mut buf1);
    for i in 256-tau as usize..256 {
        let mut j = 257;
        while j > i {
            reader.read(&mut buf2);
            j = buf2[0] as usize;
        }
        c.coeffs[i] = c.coeffs[j];
        c.coeffs[j] = match (buf1[((i+tau as usize-256) / 8) as usize] >> (((i + tau as usize -256) % 8))) & 0x01 {
            0 => 1,
            _ => -1,
        };
    }
    c
}

#[cfg(test)]
mod test {
    use crate::poly;

    use super::error_sample;


    #[test]
    fn test_reject_sample() {
        let seed = [0x7a, 0x24, 0xb6, 0x66, 0xda, 0x34, 0x5c, 0x98, 0xc3, 0xa4, 0x0, 0xaa, 0xfd, 0x14, 0xa5, 0x1a, 0x6c, 0x7, 0xd7, 0x48, 0xc6, 0xfc, 0x4, 0xfb, 0xd1, 0x30, 0x88, 0xed, 0x8b, 0x33, 0x94, 0x8d];
        let p = super::reject_sample(seed, 0, 0);
        let a = poly::Poly { coeffs: [515465, 4351027, 4662892, 1511398, 7911262, 7675129, 305583, 4718489, 7776432, 1153277, 2671761, 4483530, 3749673, 6891660, 574022, 633160, 7498162, 795918, 3289466, 6585660, 2306005, 7456262, 1625149, 1034783, 8220779, 6679451, 239524, 7520505, 4966163, 5200378, 7011872, 2127423, 8122682, 5393553, 6191886, 704030, 2013924, 2007408, 1306336, 8080312, 6769667, 5605761, 4285167, 3960375, 1411145, 4138387, 4123672, 7197524, 5871662, 3515854, 6881988, 1919477, 2316520, 6593001, 3884742, 4288255, 6111102, 4482336, 3624185, 4992883, 3011045, 8073744, 5640451, 3898439, 2086027, 2730688, 4274821, 6145841, 7178325, 1241351, 3160109, 3860846, 1411277, 2954312, 7141743, 3975066, 1802912, 3811162, 5251413, 1523235, 1756540, 4308538, 486143, 3801958, 7039955, 2717529, 990972, 5545473, 5385836, 1203965, 4381160, 7472284, 4577843, 4795205, 98348, 5504347, 3730047, 1088097, 7520181, 3986413, 4181138, 3118276, 627227, 7359701, 2566925, 6239396, 7752531, 512484, 3163184, 4570411, 6937087, 8087861, 5450685, 1490024, 2223490, 2965576, 3199943, 6062043, 861007, 566400, 7262626, 2005641, 6021090, 5763994, 254421, 7348402, 6167033, 5398521, 3191032, 1614997, 859766, 1549051, 5189309, 1113008, 6164714, 4569329, 1935544, 6150604, 2751071, 727525, 5524833, 5063891, 2673296, 4649789, 6461992, 3409884, 4326651, 6337540, 5370137, 3464442, 488692, 755022, 2791578, 5101166, 4580232, 6769945, 2332899, 4367482, 7613574, 2968524, 5078556, 3090445, 3450148, 5501401, 1891166, 2170991, 1528718, 8012790, 3623492, 6150594, 5345495, 5054617, 4698359, 2222416, 7729103, 2756625, 2724974, 3924633, 6243623, 7428928, 5504680, 4227264, 3558543, 2420703, 3506235, 3921978, 5345060, 4200477, 5373055, 3810710, 3088897, 4061582, 7980520, 1571518, 6354351, 380650, 2610478, 922196, 5766452, 6643272, 7646641, 2593225, 1074055, 6779430, 5229708, 821480, 2627205, 6757142, 4664905, 7437861, 715112, 4696800, 5863116, 7081778, 624948, 3411173, 166849, 1515090, 5300053, 4914940, 2317846, 3031462, 5680048, 2499574, 1562550, 7331078, 5168767, 3225531, 4945022, 6371727, 6271888, 5575174, 1325196, 4869700, 5589737, 406369, 1246876, 1497253, 3134030, 3034411, 8297543, 3508808, 1319848, 2885839, 3846153, 4343525, 7804227, 3266094, 6267979, 8297152, 1160534, 5656235, 1885767, 1216799, 864444, 5773147] };

        assert_eq!(p.coeffs, a.coeffs);
    }

    #[test]
    fn test_err_sample()
    {
        let seed = [0x7a, 0x24, 0xb6, 0x66, 0xda, 0x34, 0x5c, 0x98, 0xc3, 0xa4, 0x0, 0xaa, 0xfd, 0x14, 0xa5, 0x1a, 0x6c, 0x7, 0xd7, 0x48, 0xc6, 0xfc, 0x4, 0xfb, 0xd1, 0x30, 0x88, 0xed, 0x8b, 0x33, 0x94, 0x8d, 0xc4, 0x30, 0x1a, 0xf2, 0xa0, 0x6c, 0x21, 0x3a, 0xf, 0x9e, 0x5d, 0x23, 0x2f, 0x10, 0x7c, 0x1c, 0x89, 0xb3, 0x7e, 0xd, 0x59, 0xba, 0x59, 0x26, 0x74, 0x4c, 0x7a, 0x6f, 0x4, 0x4d, 0x33, 0xe9];

        let p = error_sample(seed, 0, 2);

        assert_eq!(p.coeffs, [-2, 0, -2, -1, 2, 2, 2, 0, -2, 1, 1, 2, 2, -1, 0, 1, 2, 2, 1, 1, 0, 0, -2, 1, 1, 1, 1, -1, -1, -1, -2, -1, 0, 2, -1, 1, 1, -2, -1, 1, -1, 1, 0, -2, 0, 2, 0, -2, 0, 2, 1, 0, -1, -1, 0, 2, 2, 1, 1, 2, -1, -1, 1, 1, 2, -2, 2, 1, 0, -1, 2, -2, 2, -1, -2, -2, -2, 2, -1, 2, 2, -1, 2, -2, 1, 1, -2, 0, 1, 2, 1, 2, 0, 0, -1, 1, 0, 0, -2, 0, 1, -1, -2, 2, -2, 2, -2, -2, -1, -2, -1, -1, -1, 0, 1, -2, 1, 2, 0, 2, 1, 1, -2, 2, 0, 2, -2, 1, 2, 2, -1, -1, -2, -2, 0, -2, -2, 2, 1, -1, 2, 1, -1, -1, 0, 0, -2, 1, -2, 2, 2, 0, 1, 2, 0, 0, 0, -2, 0, 1, 2, 1, -2, -1, -2, -2, -2, 1, 1, 1, 1, 1, -1, -1, 2, 1, 2, 2, -2, 2, -2, -2, 0, -2, 1, 2, -1, 0, 2, 1, 2, 1, -2, -2, -1, 0, 1, 2, -1, 0, -2, 1, 2, -1, 1, 1, -1, 2, 1, 1, -1, -2, -2, -2, 0, 1, 1, 1, -2, -2, 2, 2, 2, 1, 0, 1, 0, -1, 2, -1, -1, 2, 2, -1, 1, 1, -1, 2, 0, 0, -2, -1, -2, 0, -2, -1, 1, -2, -2, 1, -1, 2, 1, 1, 1, 0])
    }
}