pub mod poly;
pub mod reduce;
pub mod params;

use poly::Poly;

fn main() {
    let mut a : Poly = Poly { coeffs: [1; 256] };
    let mut b : Poly = Poly { coeffs: [2; 256] };
    a.ntt();
    b.ntt();
    a.point_wise_mul(&mut b);
    a.intt();
    print!("{:?}", a.coeffs);
    println!("Hello, world!");
}
