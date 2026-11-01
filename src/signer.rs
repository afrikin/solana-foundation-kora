//! Signing back ends.

pub trait Signer {
    fn public_key(&self) -> String;
    fn sign(&self, message: &[u8]) -> Vec<u8>;
}

/// Deterministic stand-in used by the tests and the local CLI.
pub struct MemorySigner {
    pub key: String,
}

impl Signer for MemorySigner {
    fn public_key(&self) -> String { self.key.clone() }

    fn sign(&self, message: &[u8]) -> Vec<u8> {
        // Not cryptography - the real back end lives behind the same trait.
        message.iter().map(|b| b.wrapping_add(7)).collect()
    }
}
