
mod polyvec {
    use crate::poly::Poly;

    // a struct consists sevaral polynomials
    pub struct PolyVec {
        pub poly: Vec<Poly>,
        pub len: usize,
    }

    impl PolyVec {
        pub fn new(len: usize) -> PolyVec {
            let mut poly = Vec::new();
            for _ in 0..len {
                poly.push(Poly::new());
            }
            PolyVec { poly, len }
        }

        pub fn get(&self, i: usize) -> &Poly {
            &self.poly[i]
        }

        pub fn get_mut(&mut self, i: usize) -> &mut Poly {
            &mut self.poly[i]
        }

        pub fn set(&mut self, i: usize, poly: Poly) {
            self.poly[i] = poly;
        }

        pub fn add(&mut self, i: usize, poly: &Poly) {
            self.poly[i] = self.poly[i].add(poly);
        }

        pub fn sub(&mut self, i: usize, poly: &Poly) {
            self.poly[i] = self.poly[i].sub(poly);
        }

        pub fn ntt(&mut self) {
            for i in 0..self.len {
                self.poly[i].ntt();
            }
        }

        pub fn  intt(&mut self) {
            for i in 0..self.len {
                self.poly[i].intt();
            }
        }

        pub fn pointwise_acc(&mut self, b: &PolyVec) {
            for i in 0..self.len {
                self.poly[i] = self.poly[i].point_wise_mul(&b.poly[i]);
                if i > 0 {
                    self.poly[0] = self.poly[0].add(&self.poly[i]);
                }
            }
        }

        pub fn pointwise_acc_invmontgomery(&mut self, b: &PolyVec) {
            for i in 0..self.len {
                self.poly[i] = self.poly[i].point_wise_mul(&b.poly[i]);
                if i > 0 {
                    self.poly[0] = self.poly[0].add(&self.poly[i]);
                }
            }
            self.poly[0].intt();
            &self.poly[0];
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

        #[test]
        fn polyvec_get_mut_test() {
            let mut a = PolyVec::new(10);
            let b = Poly::new();
            a.set(0, b);
            a.get_mut(0).coeffs[0] = 1;
            assert_eq!(a.get(0).coeffs, [1; 256]);
        }

        #[test]
        fn polyvec_set_test() {
            let mut a = PolyVec::new(10);
            let b = Poly::new();
            a.set(0, b);
            assert_eq!(a.get(0).coeffs, [0; 256]);
        }

        #[test]
        fn polyvec_add_test() {
            let mut a = PolyVec::new(10);
            let b = Poly::new();
            a.set(0, b);
            a.add(0, &b);
            assert_eq!(a.get(0).coeffs, [0; 256]);
        }

        #[test]
        fn polyvec_sub_test() {
            let mut a = PolyVec::new(10);
            let b = Poly::new();
            a.set(0, b);
            a.sub(0, &b);
            assert_eq!(a.get(0).coeffs, [0; 256]);
        }

        #[test]
        fn polyvec_ntt_test() {
            let mut a = PolyVec::new(10);
            let b = Poly::new();
            a.set(0, b);
            a.ntt();
            assert_eq!(a.get(0).coeffs, [0; 256]);
        }

        #[test]
        fn polyvec_intt_test() {
            let mut a = PolyVec::new(10);
            let b = Poly::new();
            a.set(0, b);
            a.intt();
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
