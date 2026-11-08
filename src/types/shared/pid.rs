use core::num::NonZero;

use crate::{Error, Status};

/// A simple wrapper around [`NonZero`]<[`u16`]>.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PacketIdentifier(NonZero<u16>);

impl PacketIdentifier {
    pub(crate) const fn new(value: NonZero<u16>) -> Self {
        Self(value)
    }

    /// Returns the underlying value.
    #[must_use]
    pub const fn get(self) -> NonZero<u16> {
        self.0
    }
}

impl core::fmt::Debug for PacketIdentifier {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.get().fmt(f)
    }
}
impl core::fmt::Display for PacketIdentifier {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}
