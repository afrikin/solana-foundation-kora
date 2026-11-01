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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signature_matches_message_length() {
        let s = MemorySigner { key: "k".into() };
        assert_eq!(s.sign(b"abc").len(), 3);
    }

    #[test]
    fn public_key_is_returned_verbatim() {
        let s = MemorySigner { key: "payer-1".into() };
        assert_eq!(s.public_key(), "payer-1");
    }
}
