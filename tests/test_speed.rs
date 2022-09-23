mod cpucycle;

use dilithium_rust::sign::{key_pair, sign, verify};
use crate::cpucycle::{cpucycles_overhead, cpucycles};
use rand::RngCore;

const NTEST:u64 = 1000;

#[test]
pub fn test_key_gen_speed() {
    let mut rng = rand::thread_rng();
    let mut seed = [0u8; 32];
    rng.fill_bytes(&mut seed);
    let mut t0 = 0u64;
    let mut t1 = 0u64;
    let mut cycles = 0u64;
    let mut overhead = 0u64;
    let mut min_cycles = u64::MAX;
    let mut max_cycles = 0u64;
    let mut total_cycles = 0u64;
    let mut avg_cycles = 0u64;
    let mut i = 0u32;
    overhead = cpucycles_overhead();
    while i < NTEST as u32 {
        t0 = cpucycles();
        let (_pk, _sk) = key_pair(&seed, 2);
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
    avg_cycles = total_cycles / NTEST;
    println!("key_gen cycles: min: {}, max: {}, avg: {}", min_cycles, max_cycles, avg_cycles);
}

#[test]
pub fn test_sign_speed() {
    let mut rng = rand::thread_rng();
    let mut seed = [0u8; 32];
    rng.fill_bytes(&mut seed);
    let mut t0 = 0u64;
    let mut t1 = 0u64;
    let mut cycles = 0u64;
    let mut overhead = 0u64;
    let mut min_cycles = u64::MAX;
    let mut max_cycles = 0u64;
    let mut total_cycles = 0u64;
    let mut avg_cycles = 0u64;
    let mut i = 0u32;
    overhead = cpucycles_overhead();
    let (pk, sk) = key_pair(&seed, 2);
    while i < NTEST as u32 {
        let mut msg = [0u8; 32];
        rng.fill_bytes(&mut msg);
        t0 = cpucycles();
        let sig = sign(&sk, &msg.to_vec());
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
    avg_cycles = total_cycles / NTEST;
    println!("sign cycles: min: {}, max: {}, avg: {}", min_cycles, max_cycles, avg_cycles);
}

#[test]
pub fn test_verify_speed() {
    let mut rng = rand::thread_rng();
    let mut seed = [0u8; 32];
    rng.fill_bytes(&mut seed);
    let mut t0 = 0u64;
    let mut t1 = 0u64;
    let mut cycles = 0u64;
    let mut overhead = 0u64;
    let mut min_cycles = u64::MAX;
    let mut max_cycles = 0u64;
    let mut total_cycles = 0u64;
    let mut avg_cycles = 0u64;
    let mut i = 0u32;
    overhead = cpucycles_overhead();
    let (pk, sk) = key_pair(&seed, 2);
    while i < NTEST as u32 {
        let mut msg = [0u8; 32];
        rng.fill_bytes(&mut msg);
        let sig = sign(&sk, &msg.to_vec());
        t0 = cpucycles();
        let res = verify(&sig, &pk, &msg.to_vec());
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
    avg_cycles = total_cycles / NTEST;
    println!("verify cycles: min: {}, max: {}, avg: {}", min_cycles, max_cycles, avg_cycles);
}