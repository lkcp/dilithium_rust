use crate::params::{QINV, Q};

pub fn montgomery_reduce(a: i64) -> i32 {
    let m = ((a as i32) as i64 * (QINV as i64)) as i32;
    let t = (a - (m as i64)* (Q as i64)) >> 32;
    t as i32
}



#[cfg(test)]
mod test {
    use crate::params::Q;

    use super::montgomery_reduce;

    #[test]
    fn mont_reduce_test() {
        let mut a: i64 = -12232142;
        let mut b: i64 = a << 32 % Q;
        assert_eq!(montgomery_reduce(b), a as i32, "mont reduce: basic correct");
        a = -12314545;
        b = a << 32 % Q;
        assert_eq!(montgomery_reduce(b), a as i32);

        assert_eq!(montgomery_reduce(-518909*3572224), -853297);
    }
}