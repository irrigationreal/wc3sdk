//! Data-driven SDK helpers for WC3 reverse engineering.
//!
//! This crate is intentionally dynamic: layouts and symbols are loaded from
//! JSON dumps generated from a matching PDB+EXE. The shim can then resolve
//! addresses and read structured state without hardcoding offsets.

pub mod call;
pub mod config;
pub mod accessor;
pub mod layout;
pub mod memory;
pub mod snapshot;
pub mod symbols;
pub mod generated;

pub use call::*;
pub use accessor::*;
pub use config::*;
pub use layout::*;
pub use memory::*;
pub use snapshot::*;
pub use symbols::*;

#[cfg(test)]
mod lib_tests;
