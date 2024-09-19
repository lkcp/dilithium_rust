mod cpucycle;

use dilithium_rust::sign::{key_pair, sign, verify};
use crate::cpucycle::{cpucycles_overhead, cpucycles};
use rand::RngCore;

const NTEST:u64 = 10;
const LEVEL:u8 = 2;

#[test]
pub fn test_key_gen_speed() {
    let mut t0;
    let mut t1;
    let mut cycles;
    let mut min_cycles = u64::MAX;
    let mut max_cycles = 0u64;
    let mut total_cycles = 0u64;
    let mut i = 0u32;
    let overhead = cpucycles_overhead();
    while i < NTEST as u32 {
        t0 = cpucycles();
        let (_pk, _sk) = key_pair(LEVEL);
        t1 = cpucycles();
        cycles = t1 - t0 - overhead;
        if cycles < min_cycles {
            min_cycles = cycles;
        }
        if cycles > max_cycles {
            max_cycles = cycles;
        }
        total_cycles += cycles;
        i += 1;
    }
    let avg_cycles = total_cycles / NTEST;
    println!("key_gen cycles: min: {}, max: {}, avg: {}", min_cycles, max_cycles, avg_cycles);
}

#[test]
pub fn test_sign_speed() {
    let mut rng = rand::thread_rng();
    let mut t0;
    let mut t1;
    let mut cycles;
    let mut min_cycles = u64::MAX;
    let mut max_cycles = 0u64;
    let mut total_cycles = 0u64;
    let mut i = 0u32;
    let overhead = cpucycles_overhead();
    let (_, sk) = key_pair(2);
    while i < NTEST as u32 {
        let mut msg = [0u8; 32];
        rng.fill_bytes(&mut msg);
        t0 = cpucycles();
        sign(&sk, &msg.to_vec(), LEVEL);
        t1 = cpucycles();
        cycles = t1 - t0 - overhead;
        if cycles < min_cycles {
            min_cycles = cycles;
        }
        if cycles > max_cycles {
            max_cycles = cycles;
        }
        total_cycles += cycles;
        i += 1;
    }
    let avg_cycles = total_cycles / NTEST;
    println!("sign cycles: min: {}, max: {}, avg: {}", min_cycles, max_cycles, avg_cycles);
}

#[test]
pub fn test_verify_speed() {
    let mut rng = rand::thread_rng();
    let mut t0;
    let mut t1;
    let mut cycles;
    let mut min_cycles = u64::MAX;
    let mut max_cycles = 0u64;
    let mut total_cycles = 0u64;
    let mut i = 0u32;
    let overhead = cpucycles_overhead();
    let (pk, sk) = key_pair(2);
    while i < NTEST as u32 {
        let mut msg = [0u8; 32];
        rng.fill_bytes(&mut msg);
        let sig = sign(&sk, &msg.to_vec(), LEVEL);
        t0 = cpucycles();
        verify(&sig, &pk, &msg.to_vec());
        t1 = cpucycles();
        cycles = t1 - t0 - overhead;
        if cycles < min_cycles {
            min_cycles = cycles;
        }
        if cycles > max_cycles {
            max_cycles = cycles;
        }
        total_cycles += cycles;
        i += 1;
    }
    let avg_cycles = total_cycles / NTEST;
    println!("verify cycles: min: {}, max: {}, avg: {}", min_cycles, max_cycles, avg_cycles);
}
