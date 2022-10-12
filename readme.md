# What't it?

I learned to be a new Ruster recently and make some study about dilithium, so I treat it as a practice of my Rust leanring.

**warning**: It shouldn't be applied to any realistic project.

Several **tips**:
- We use [sha3](https://docs.rs/sha3) crate as our CRH and XOF
- We offer several test example of dilithium in our unit test

# How's about its performance?
I compared our performance with [official ref and avx implemenation](https://github.com/pq-crystals/dilithium) in Intel Core i5-8265U @ 8x 1.8GHz of my HUAWEI Laptop, the OS is Ubuntu20.04-WSL:


|                        	| KeyPair 	| Sign   	| Verify 	|
|------------------------	|---------	|--------	|--------	|
| My rust implementation 	| 200842  	| 639286 	| 200787 	|
| ref implementation     	| 176933  	| 787787 	| 196560 	|
| AVX optimization       	| 106025  	| 303899 	| 107837 	|

# How to use?

The **sign** crate offers 3 apis:
```rust
key_pair(seed: &[u8; 32], security_level: u8) -> (Vec<u8>, Vec<u8>)
sign(sk: &Vec<u8>, m: &Vec<u8>, security_level: u8) -> Vec<u8>
verify(delta: &Vec<u8>, pk: &Vec<u8>, m: &Vec<u8>) -> bool
```