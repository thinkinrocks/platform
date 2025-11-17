use std::iter::repeat_with;

use hex;
use rand::{
    RngCore, TryRngCore,
    distr::{Alphanumeric, Distribution, SampleString},
    rngs::OsRng,
};
use sha2::{Digest, Sha256};
use snowflaked::sync::Generator;

pub fn hash_digest(input: &[u8]) -> String {
    hex::encode(Sha256::digest(input))
}

static GENERATOR: Generator = Generator::new(0);

pub fn generate_id() -> i64 {
    GENERATOR.generate()
}

pub fn generate_code() -> Result<String, ()> {
    let mut rng = OsRng;
    let candidates = b"ABCDEFGHJIKLMNOPQRSTUVWXYZ0123456789";
    
    (0..12usize)
        .map(|_| {
            rng.try_next_u64()
                .map(|i| candidates[i as usize % candidates.len()])
                .map(char::from)
        })
        .collect::<Result<String, _>>()
        .map_err(drop)
}
