#![cfg_attr(not(any(test, feature = "alloc", feature = "std")), no_std)]
#![warn(
    // missing_docs,
    clippy::missing_safety_doc,
    clippy::undocumented_unsafe_blocks
)]
#![allow(unused)]

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;

use crate::{base::ParserBase, types::VarByteInt};

mod base;
mod err;

pub use err::{Error, ErrorCategory};
pub mod types;
pub mod v5;

/// A Result of any parsing action.
///
/// If the input is invalid, an `Error` will be returned. Note that incomplete
/// data is not considered invalid, and so will not return an error, but rather
/// a `Ok(Status::Partial)`.
pub type Result<T> = core::result::Result<Status<T>, Error>;

pub trait MapExt<T> {
    fn map_status<U>(self, map: impl FnOnce(T) -> U) -> Result<U>;
}

impl<T> MapExt<T> for Result<T> {
    fn map_status<U>(self, map: impl FnOnce(T) -> U) -> Result<U> {
        self.map(|status| status.map(map))
    }
}

/// The result of a successful parse pass.
///
/// `Complete` is used when the buffer contained the complete value.
/// `Partial` is used when parsing did not reach the end of the expected value,
/// but no invalid data was found.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Status<T> {
    /// The completed result.
    Complete(T),
    /// A partial result.
    Partial,
}

impl<T> Status<T> {
    pub fn map<U>(self, map: impl FnOnce(T) -> U) -> Status<U> {
        match self {
            Status::Complete(value) => Status::Complete(map(value)),
            Status::Partial => Status::Partial,
        }
    }

    /// Convenience method to check if status is complete.
    #[inline]
    pub fn is_complete(&self) -> bool {
        match *self {
            Status::Complete(_) => true,
            Status::Partial => false,
        }
    }

    /// Convenience method to check if status is partial.
    #[inline]
    pub fn is_partial(&self) -> bool {
        match *self {
            Status::Complete(_) => false,
            Status::Partial => true,
        }
    }

    /// Convenience method to unwrap a Complete value. Panics if the status is
    /// `Partial`.
    #[inline]
    pub fn unwrap(self) -> T {
        match self {
            Status::Complete(t) => t,
            Status::Partial => panic!("Tried to unwrap Status::Partial"),
        }
    }
}

#[derive(Debug)]
pub struct Header<T> {
    packet_type: T,
    flags: u8,
    remaining_len: VarByteInt,
}
