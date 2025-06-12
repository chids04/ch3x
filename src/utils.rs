use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};
use openssl::sha::sha256;


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Hash32([u8; 32]);

pub fn current_time() -> u64 {

    let now = SystemTime::now();
    let timestamp = now
        .duration_since(UNIX_EPOCH)
        .expect("Invalid time").as_secs();

    timestamp
}

impl Hash32 {
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Hash32(sha256(&bytes))
    }

    pub fn from_string(string: &str) -> Self{
        Hash32(sha256(string.as_bytes()))
    }

    pub fn get_substr(&self, len: usize) -> &[u8] {
        &self.0[0..len]
    }

}

impl Default for Hash32{
    fn default() -> Self {
        Hash32([0u8; 32])
    }
}

impl fmt::Display for Hash32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_bytes(){
        
    }
}