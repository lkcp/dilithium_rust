#[cfg(target_arch = "aarch64")]
pub fn cpucycles() -> u64 {
    let value: u64;
    unsafe {
        std::arch::asm!("mrs {}, cntvct_el0", out(reg) value);
    }
    value
}
pub fn cpucycles_overhead() -> u64 {
    let mut overhead: u64 = u64::MAX;
    for _ in 0..1000 {
        let t0 = cpucycles();
	
        let t1 = cpucycles();
        if t1 - t0 < overhead {
            overhead = t1 - t0;
        }
    }
    overhead
}
