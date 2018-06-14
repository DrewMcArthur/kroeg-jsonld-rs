#![feature(proc_macro, generators)]

#[macro_use]
extern crate lazy_static;

extern crate url;

extern crate serde;
extern crate serde_json;

extern crate futures_await as futures;

mod compact;
mod context;
mod creation;
mod expand;
pub mod rdf;

mod api;
pub use api::*;

pub mod error {
    pub use compact::CompactionError;
    pub use creation::{ContextCreationError, TermCreationError};
    pub use expand::ExpansionError;
}

use futures::prelude::*;

/// This trait is implemented by consumers of the API, to provide remote contexts.
pub trait RemoteContextLoader {
    type Future: Future<Item = serde_json::Value, Error = Box<std::error::Error + Send>> + Send + 'static;

    /// Loads a remote JSON-LD context into memory.
    fn load_context(url: String) -> Self::Future;
}
