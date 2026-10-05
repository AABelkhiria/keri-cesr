//! Pure CESR parsing, encoding, and qualified-material primitives.
//!
//! With the `crypto` feature, the `crypto` module adds signing, verification, digest, key derivation, and
//! encryption over those primitives.

#![forbid(unsafe_code)]

pub mod base64;
pub mod bexter;
pub mod bytes;
pub mod code;
pub mod counter;
#[cfg(feature = "crypto")]
pub mod crypto;
pub mod error;
pub mod indexer;
pub mod matter;
pub mod number;
pub mod path;
pub mod sequence;

pub use error::CesrError;
