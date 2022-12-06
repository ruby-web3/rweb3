#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_rust_codeblocks)]
use magnus::{define_module, memoize, Error, Module, RModule};

mod key;
mod types;

/// The "Wasmtime" Ruby module.
pub fn root() -> RModule {
    *memoize!(RModule: define_module("RWeb3").unwrap())
}

pub fn init() -> Result<(), Error> {
    let _ = root();

    types::init()?;
    key::init()?;

    Ok(())
}
