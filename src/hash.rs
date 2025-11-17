use sha2::{Sha256, Digest};
use hex;

pub fn hash_digest(input: &[u8]) -> String {
    hex::encode(Sha256::digest(input))
}
