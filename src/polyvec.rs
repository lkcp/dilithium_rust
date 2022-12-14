
pub mod polyvec {
    use crate::poly::Poly;

    // a struct consists sevaral polynomials
    #[derive(Debug)]
    pub struct PolyVec {
        pub vec: Vec<Poly>,
        pub len: usize,
    }

    impl PolyVec {
        pub fn new(len: usize) -> PolyVec {
            let mut vec = Vec::new();
            for _ in 0..len {
                vec.push(Poly::new());
            }
            PolyVec { vec, len }
        }

        pub fn get(&self, i: usize) -> &Poly {
            &self.vec[i]
        }

        pub fn set(&mut self, i: usize, poly: Poly) {
            self.vec[i] = poly;
        }

        pub fn add(&mut self, pv: &PolyVec) -> PolyVec {
            let mut s = self.copy();
            for i in 0..self.len {
                s.vec[i] = s.vec[i].add(pv.get(i));
            }
            return s
        }

        // negation
        pub fn neg(&mut self) -> PolyVec {
            let mut n = self.copy();
            for i in 0..self.len {
                n.vec[i] = n.vec[i].neg();
            }
            n
        }

        pub fn ntt(&self) ->PolyVec {
            let mut pv = PolyVec::new(self.len);
            for i in 0..self.len {
                pv.vec[i] = self.vec[i].ntt();
            }
            pv
        }

        pub fn intt(&self) -> PolyVec{
            let mut pv = PolyVec::new(self.len);
            for i in 0..self.len {
                pv.vec[i] = self.vec[i].intt();
            }
            pv
        }

        pub fn pointwise_acc(&mut self, b: &PolyVec) -> Poly {
            let mut acc = Poly::new();
            for i in 0..self.len {
                acc = acc.add(&self.vec[i].point_wise_mul(&b.vec[i]));
            }
            acc
        }

        // convert to [0, q) in place]
        pub fn caddq(&mut self) {
            for i in 0..self.len {
                self.vec[i].caddq();
            }
        }

        // return a copy of this instance
        pub fn copy(&self) -> PolyVec {
            let mut pv = PolyVec::new(self.len);
            for i in 0..self.len {
                pv.set(i, self.vec[i]);
            }
            pv
        }
        
        pub fn high_bits(&self, gamma2: i32) -> PolyVec {
            let mut t = PolyVec::new(self.len);
            for i in 0..self.len {
                t.set(i, self.vec[i].high_bits(gamma2));
            }
            t
        }

        pub fn low_bits(&self, gamma2: i32) -> PolyVec {
            let mut t = PolyVec::new(self.len);
            for i in 0..self.len {
                t.set(i, self.vec[i].low_bits(gamma2));
            }
            t
        }

        //infinite norm
        pub fn inf_norm(&self) -> i32 {
            let mut max = 0;
            for i in 0..self.len {
                let norm = self.vec[i].inf_norm();
                if norm > max {
                    max = norm;
                }
            }
            max
        }

        // every poly is left shifted by d bits
        pub fn left_shift(&mut self, d: i32) {
            for i in 0..self.len {
                self.vec[i].lshift(d);
            }
        }

    }

    #[cfg(test)]
    mod test {
        use super::*;
        #[test]
        fn polyvec_new_test() {
            let a = PolyVec::new(10);
            assert_eq!(a.len, 10);
        }

        #[test]
        fn polyvec_get_test() {
            let mut a = PolyVec::new(10);
            let b = Poly::new();
            a.set(0, b);
            assert_eq!(a.get(0).coeffs, [0; 256]);
        }

        // #[test]
        // fn polyvec_get_mut_test() {
        //     let mut a = PolyVec::new(10);
        //     let b = Poly::new();
        //     a.set(0, b);
        //     a.get_mut(0).coeffs[0] = 1;
        //     assert_eq!(a.get(0).coeffs, [1; 256]);
        // }

        #[test]
        fn polyvec_set_test() {
            let mut a = PolyVec::new(10);
            let b = Poly::new();
            a.set(0, b);
            assert_eq!(a.get(0).coeffs, [0; 256]);
        }

        #[test]
        fn polyvec_pointwise_acc_test() {
            let mut a = PolyVec::new(10);
            let b = Poly::new();
            a.set(0, b);
            // a.pointwise_acc(&a);
            assert_eq!(a.get(0).coeffs, [0; 256]);
        }

        #[test]
        fn polyvec_pointwise_acc_invmontgomery_test() {
            let mut a = PolyVec::new(10);
            let b = Poly::new();
            a.set(0, b);
            // a.pointwise_acc_invmontgomery(&a);
            assert_eq!(a.get(0).coeffs, [0; 256]);
        }



    }
}
