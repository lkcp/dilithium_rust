
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

        pub fn get_mut(&mut self, i: usize) -> &mut Poly {
            &mut self.vec[i]
        }

        pub fn set(&mut self, i: usize, poly: Poly) {
            self.vec[i] = poly;
        }

        pub fn add(&mut self, i: usize, poly: &Poly) {
            self.vec[i] = self.vec[i].add(poly);
        }

        pub fn sub(&mut self, i: usize, poly: &Poly) {
            self.vec[i] = self.vec[i].sub(poly);
        }

        pub fn ntt(&mut self) {
            for i in 0..self.len {
                self.vec[i].ntt();
            }
        }

        pub fn  intt(&mut self) {
            for i in 0..self.len {
                self.vec[i].intt();
            }
        }

        pub fn pointwise_acc(&mut self, b: &PolyVec) -> Poly {
            for i in 0..self.len {
                self.vec[i] = self.vec[i].point_wise_mul(&b.vec[i]);
                if i > 0 {
                    self.vec[0] = self.vec[0].add(&self.vec[i]);
                }
            }
            self.vec[0]
        }

        pub fn pointwise_acc_invmontgomery(&mut self, b: &PolyVec) {
            self.pointwise_acc(b);
            let _ = self.vec[0].intt();
            self.vec[0];
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
