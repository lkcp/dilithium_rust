use std::arch::asm;
use std::arch::x86_64::_rdtsc;

pub fn cpucycles() -> u64 {
    let x: u64;
    unsafe {
        x = _rdtsc();
    }
    x
}

pub fn cpucycles_overhead() -> u64 {
    let mut overhead: u64 = u64::MAX;
    for _ in 0..1000 {
        let t0 = cpucycles();
        unsafe {
            asm!("");
        }
        let t1 = cpucycles();
        if t1 - t0 < overhead {
            overhead = t1 - t0;
        }
    }
    overhead
}