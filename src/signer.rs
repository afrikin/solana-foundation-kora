//! Signing back ends.

pub trait Signer {
    fn public_key(&self) -> String;
}
