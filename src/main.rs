pub mod poly;
pub mod reduce;
pub mod params;
pub mod back;
mod polyvec;
mod sample;
mod sign;
mod hints;
mod pack;
mod utils;

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
