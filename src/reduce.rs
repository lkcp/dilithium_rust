use crate::params::{QINV, Q};

pub fn montgomery_reduce(a: i64) -> i32 {
    let m = ((a as i32) as i64 * (QINV as i64)) as i32;
    let t = (a - (m as i64)* (Q as i64)) >> 32;
    t as i32
}

