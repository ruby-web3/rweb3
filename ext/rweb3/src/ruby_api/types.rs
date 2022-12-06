use magnus::{
    function, r_typed_data::DataTypeBuilder, Error, Module, Object, RClass, TypedData, Value,
};
use web3::types::Bytes as BytesImpl;

use super::root;

#[derive(Debug)]
#[magnus::wrap(class = "Bytes", free_immediately, size)]
pub struct Bytes {
    inner: BytesImpl,
}

impl Bytes {
    pub fn new(bytes: Vec<u8>) -> Result<Self, Error> {
        let inner = BytesImpl::from(bytes);

        Ok(Self { inner })
    }

    pub fn from_inner(inner: BytesImpl) -> Self {
        Self { inner }
    }

    pub fn get(&self) -> &BytesImpl {
        &self.inner
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("Bytes", Default::default())?;
    class.define_singleton_method("new", function!(Bytes::new, 1))?;

    Ok(())
}
