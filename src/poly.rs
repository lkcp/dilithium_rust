use crate::reduce::montgomery_reduce;

static ZETAS: [i32; 255] = [25847, -2608894, -518909, 237124, -777960, -876248, 466468, 1826347, 2353451, -359251, -2091905, 3119733, -2884855, 3111497, 2680103, 2725464, 1024112, -1079900, 3585928, -549488, -1119584, 2619752, -2108549, -2118186, -3859737, -1399561, -3277672, 1757237, -19422, 4010497, 280005, 2706023, 95776, 3077325, 3530437, -1661693, -3592148, -2537516, 3915439, -3861115, -3043716, 3574422, -2867647, 3539968, -300467, 2348700, -539299, -1699267, -1643818, 3505694, -3821735, 3507263, -2140649, -1600420, 3699596, 811944, 531354, 954230, 3881043, 3900724, -2556880, 2071892, -2797779, -3930395, -1528703, -3677745, -3041255, -1452451, 3475950, 2176455, -1585221, -1257611, 1939314, -4083598, -1000202, -3190144, -3157330, -3632928, 126922, 3412210, -983419, 2147896, 2715295, -2967645, -3693493, -411027, -2477047, -671102, -1228525, -22981, -1308169, -381987, 1349076, 1852771, -1430430, -3343383, 264944, 508951, 3097992, 44288, -1100098, 904516, 3958618, -3724342, -8578, 1653064, -3249728, 2389356, -210977, 759969, -1316856, 189548, -3553272, 3159746, -1851402, -2409325, -177440, 1315589, 1341330, 1285669, -1584928, -812732, -1439742, -3019102, -3881060, -3628969, 3839961, 2091667, 3407706, 2316500, 3817976, -3342478, 2244091, -2446433, -3562462, 266997, 2434439, -1235728, 3513181, -3520352, -3759364, -1197226, -3193378, 900702, 1859098, 909542, 819034, 495491, -1613174, -43260, -522500, -655327, -3122442, 2031748, 3207046, -3556995, -525098, -768622, -3595838, 342297, 286988, -2437823, 4108315, 3437287, -3342277, 1735879, 203044, 2842341, 2691481, -2590150, 1265009, 4055324, 1247620, 2486353, 1595974, -3767016, 1250494, 2635921, -3548272, -2994039, 1869119, 1903435, -1050970, -1333058, 1237275, -3318210, -1430225, -451100, 1312455, 3306115, -1962642, -1279661, 1917081, -2546312, -1374803, 1500165, 777191, 2235880, 3406031, -542412, -2831860, -1671176, -1846953, -2584293, -3724270, 594136, -3776993, -2013608, 2432395, 2454455, -164721, 1957272, 3369112, 185531, -1207385, -3183426, 162844, 1616392, 3014001, 810149, 1652634, -3694233, -1799107, -3038916, 3523897, 3866901, 269760, 2213111, -975884, 1717735, 472078, -426683, 1723600, -1803090, 1910376, -1667432, -1104333, -260646, -3833893, -2939036, -2235985, -420899, -2286327, 183443, -976891, 1612842, -3545687, -554416, 3919660, -48306, -1362209, 3937738, 1400424, -846154, 1976782];

static Q: i32 = 8380417;
const F: i32 = 41978; // mont^2 /256

pub struct Poly {
    pub coeffs: [i32; 256],
}

impl Poly {
    pub fn new() -> Poly {
        Poly { coeffs: [0; 256] }
    }

    pub fn point_wise_mul(&mut self, b: Poly) -> () {
        let mut i: usize = 0;
        loop {
            self.coeffs[i] = montgomery_reduce(self.coeffs[i] as i64 * b.coeffs[i] as i64); // mont(a, b) = abR^{-1}
            i += 1;
            if i == 256 {
                break;
            }
        }
    }

    pub fn add(&self, b: &Poly) -> Poly {
        let mut c: Poly = Poly::new();
        let mut i: usize = 0;
        loop {
            c.coeffs[i] = (self.coeffs[i] + b.coeffs[i]) % Q;
            i += 1;
            if i == 256 {
                break;
            }
        }
        c
    }

    pub fn sub(&self, b: &Poly) -> Poly {
        let mut c: Poly = Poly::new();
        let mut i: usize = 0;
        loop {
            c.coeffs[i] = (self.coeffs[i] - b.coeffs[i]) % Q;
            i += 1;
            if i == 256 {
                break;
            }
        }
        c
    }
}

impl Poly {
    pub fn ntt(&mut self) -> () {
        let mut k: usize = 0;
        let mut len: usize = 128;
        loop {
            let mut start: usize = 0;
            loop {
                let zeta: i32 = ZETAS[k];
                k += 1;
                let mut i: usize = 0;
                loop {
                    let tiwddle: i32 =
                        montgomery_reduce(zeta as i64 * self.coeffs[start + i + len] as i64); // mont(c * zR) = cz mod Q
                    self.coeffs[start + i] += tiwddle;
                    self.coeffs[start + len + i] -= tiwddle;
                    i += 1;
                    if i == len {
                        break;
                    }
                }
                start += len * 2;
                if start == 256 {
                    break;
                }
            }
            len >>= 1;
            if len == 0 {
                break;
            }
        }
    }

    pub fn intt(&mut self) -> () {
        let mut k: usize = 255;
        let mut len: usize = 1;
        loop {
            let mut start: usize = 0;
            loop {
                k -= 1;
                let zeta: i32 = -ZETAS[k];
                
                let mut i: usize = 0;
                loop {
                    let temp: i32 = self.coeffs[start + i];
                    self.coeffs[start + i] = self.coeffs[start + i] + self.coeffs[start + i + len];
                    self.coeffs[start + i + len] =
                        montgomery_reduce((temp - self.coeffs[start + i + len]) as i64 * zeta as i64);
                    i += 1;
                    if i == len {
                        break;
                    }
                }
                start += len * 2;
                if start == 256 {
                    break;
                }
            }
            len <<= 1;
            if len == 256 {
                break;
            }
        }
        for coeff in self.coeffs.iter_mut() {
            *coeff = montgomery_reduce(*coeff as i64 * F as i64)
        } // eliminate the R^{-1} introduced by point-wise multiplication
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn poly_add_test() {
        let a : Poly = Poly { coeffs: [1; 256] };
        let b : Poly = Poly { coeffs: [2; 256] };
        let c = a.add(&b);
        assert_eq!(c.coeffs, [3; 256]);
    }

    #[test]
    fn ntt_base_test() {
        let mut a : Poly = Poly { coeffs: [-123; 256] };
        let mut b : Poly = Poly { coeffs: [-123; 256] };
        a.ntt();
        a.intt();
        for coeff in b.coeffs.iter_mut() {
            *coeff = montgomery_reduce(*coeff as i64 * F as i64 * 256);
        }
        assert_eq!(a.coeffs, b.coeffs);
    }

    #[test]
    fn poly_mul_test() {
        let mut a: Poly = Poly { coeffs: [11; 256] };
        let mut b: Poly = Poly { coeffs: [11; 256] };
        a.ntt();
        b.ntt();
        a.point_wise_mul(b);
        a.intt();

        let c: Poly = Poly { coeffs: [11; 256] };
        let d: Poly = Poly { coeffs: [11; 256] };
        let mut e: Poly = Poly { coeffs: [0; 256] };


        let mut i = 0;
        loop {
            let mut j = 0;
            loop {
                if i+j < 256 {e.coeffs[i+j] += c.coeffs[i]*d.coeffs[j];}
                else {e.coeffs[i+j-256] -= c.coeffs[i]*d.coeffs[j];}        
                j += 1;
                if j == 256 {break;}
            }
            i += 1;
            if i == 256 {break;}
        }
        assert_eq!(a.coeffs, e.coeffs);
    }

    // test for Poly::poly_add_test()
    #[test]
    fn poly_add_test_2() {
        let a : Poly = Poly { coeffs: [1; 256] };
        let b : Poly = Poly { coeffs: [2; 256] };
        let c = a.add(&b);
        assert_eq!(c.coeffs, [3; 256]);
    }
    
}
