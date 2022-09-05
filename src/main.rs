pub mod poly;
pub mod reduce;
pub mod params;
pub mod back;
mod polyvec;
mod sample;
mod sign;
<<<<<<< HEAD
=======
mod hints;
mod pack;
>>>>>>> a96b6b2fc49e29c054c09a7926660e7f94d3c6be

use poly::Poly;

fn main() {
    let mut a : Poly = Poly { coeffs: [1; 256] };
    let mut b : Poly = Poly { coeffs: [0; 256] };
    b.coeffs[0] = 1;
    a.ntt();
    b.ntt();
    a.point_wise_mul(&b);
    a.intt();
    print!("{:?}", a.coeffs);
}
