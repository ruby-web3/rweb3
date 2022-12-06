use std::borrow::Borrow;
use std::cell::RefCell;
use std::ops::Deref;

use magnus::{function, method, Attr, Error, Module, Object, TryConvert, TypedData};
use web3::signing::{
    hash_message, Key as Web3Key, SecretKeyRef, Signature as SignatureImpl, SigningError,
};

use crate::ruby_api::types::Bytes;
use secp256k1::rand::rngs::OsRng;
use secp256k1::{Secp256k1, SecretKey};

use super::root;

#[derive(Debug)]
#[magnus::wrap(class = "RWeb3::Key", free_immediately, size)]
pub struct Key {
    inner: RefCell<SecretKey>,
}

impl Key {
    /// @yard
    /// @return [Key]
    pub fn new() -> Self {
        let secp = Secp256k1::new();
        let mut rng = OsRng::new().expect("OsRng");
        let (secret_key, _) = secp.generate_keypair(&mut rng);
        Self {
            inner: RefCell::new(secret_key),
        }
    }

    pub fn sign(&self, message: &Bytes, chain_id: Option<u64>) -> Result<Signature, Error> {
        let secret_key = self.inner.borrow();
        let secret_key_ref = SecretKeyRef::new(secret_key.deref());
        Ok(Signature::from_inner(
            secret_key_ref
                .sign(hash_message(message.get().0.as_slice()).as_ref(), chain_id)
                .unwrap(),
        ))
    }
}

#[magnus::wrap(class = "RWeb3::Signature", free_immediately, size)]
pub struct Signature {
    inner: SignatureImpl,
}

impl Signature {
    pub fn from_inner(inner: SignatureImpl) -> Self {
        Self { inner }
    }

    pub fn get(&self) -> &SignatureImpl {
        &self.inner
    }

    pub fn r(&self) -> Result<Bytes, Error> {
        return Bytes::new(self.inner.borrow().r.0.to_vec());
    }
}

pub fn init() -> Result<(), Error> {
    let key_class = root().define_class("Key", Default::default())?;
    key_class.define_singleton_method("new", function!(Key::new, 0))?;
    key_class.define_method("sign", method!(Key::sign, 2))?;

    let signature_class = root().define_class("Signature", Default::default())?;
    signature_class.define_attr("r", Attr::Read)?;

    Ok(())
}
