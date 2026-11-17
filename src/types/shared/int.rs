use core::num::TryFromIntError;

use crate::{Error, Status, base::ParserBase, types::TooLargeToEncode};

pub enum VarByteIntError {
    TooShort,
    FourBytesExceeded,
    EncodingNotMinimal,
}

/// MQTT's variable byte integer encoding. The value has to be less than or equal to
/// [`VarByteInt::MAX_ENCODABLE`] (`268_435_455`).
///
/// Used for packet length, reason codes (though the encoding of all valid reason codes
/// is the same as if it would be a one byte integer) and the subscription identifier
/// property.
///
/// Use its [`TryFrom`] ([`u32`]) and [`From`] ([`u16`], [`u8`]) implementations to
/// construct a value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VarByteInt(u32);

impl VarByteInt {
    /// The maximum encodable value using the variable byte integer encoding according to
    /// <https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901011>.
    pub const MAX_ENCODABLE: u32 = 268_435_455;

    /// Creates a variable byte integer by checking for the maximum value of
    /// [`VarByteInt::MAX_ENCODABLE`].
    /// For a version accepting [`u16`] and [`u8`], use [`From::from`].
    #[must_use]
    pub const fn new(value: u32) -> Option<Self> {
        if value > Self::MAX_ENCODABLE {
            None
        } else {
            Some(Self(value))
        }
    }

    /// Creates a variable byte integer without checking for the maximum value of
    /// [`VarByteInt::MAX_ENCODABLE`].
    /// For a fallible version, use [`VarByteInt::new`].
    /// For an infallible version accepting [`u16`] and [`u8`], use [`From::from`].
    ///
    /// # Invariants
    /// The value parameter must be less than or equal to [`VarByteInt::MAX_ENCODABLE`].
    ///
    /// # Panics
    /// Panics in debug builds if `value` exceeds [`VarByteInt::MAX_ENCODABLE`].
    #[must_use]
    pub const fn new_unchecked(value: u32) -> Self {
        debug_assert!(
            value <= Self::MAX_ENCODABLE,
            "the value exceeds MAX_ENCODABLE"
        );

        Self(value)
    }

    fn decode(encoding: &[u8]) -> Option<Self> {
        let value = encoding
            .iter()
            .enumerate()
            .map(|(i, &b)| (u32::try_from(i).unwrap(), u32::from(b)))
            .map(|(i, b)| (128u32.pow(i), b & 0x7F))
            .fold(0, |sum, (b, mul)| sum + b * mul);

        let minimal_encoding_len = match value {
            0..=127 => 1,
            128..=16_383 => 2,
            16_384..=2_097_151 => 3,
            2_097_152..=Self::MAX_ENCODABLE => 4,
            _ => unreachable!("we decode at max 4 bytes"),
        };

        (minimal_encoding_len == encoding.len()).then_some(Self::new_unchecked(value))
    }

    pub fn parse_remaining_length(slice: &[u8]) -> Result<Status<(Self, usize)>, Error> {
        let is_complete_encoding = slice.len() >= 4;

        let Some((i, _)) = slice
            .iter()
            .enumerate()
            .take(4)
            .find(|&(_, b)| b & 128 == 0)
        else {
            return if is_complete_encoding {
                Err(Error::InvalidRemainingLength)
            } else {
                Ok(Status::Partial)
            };
        };
        let encoding_len = i + 1;

        Self::decode(&slice[..encoding_len])
            .ok_or(Error::InvalidRemainingLength)
            .map(|v| Status::Complete((v, encoding_len)))
    }

    pub fn parse_regular<B: ParserBase>(b: &mut B) -> Result<Self, VarByteIntError> {
        let slice = b.as_ref();

        let is_complete_encoding = slice.len() >= 4;

        let Some((i, _)) = slice
            .iter()
            .enumerate()
            .take(4)
            .find(|&(_, b)| b & 128 == 0)
        else {
            return if is_complete_encoding {
                Err(VarByteIntError::FourBytesExceeded)
            } else {
                Err(VarByteIntError::TooShort)
            };
        };

        let encoding_len = i + 1;

        Self::decode(b.split_to(encoding_len).as_ref()).ok_or(VarByteIntError::EncodingNotMinimal)
    }
}

impl TryFrom<usize> for VarByteInt {
    type Error = TooLargeToEncode;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        u32::try_from(value)
            .map_err(|_| TooLargeToEncode)
            .and_then(Self::try_from)
    }
}
impl TryFrom<u32> for VarByteInt {
    type Error = TooLargeToEncode;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(TooLargeToEncode)
    }
}
impl From<u16> for VarByteInt {
    fn from(value: u16) -> Self {
        Self(u32::from(value))
    }
}
impl From<u8> for VarByteInt {
    fn from(value: u8) -> Self {
        Self(u32::from(value))
    }
}

impl TryInto<usize> for VarByteInt {
    type Error = TryFromIntError;

    fn try_into(self) -> Result<usize, Self::Error> {
        usize::try_from(self.0)
    }
}
impl From<VarByteInt> for u32 {
    fn from(value: VarByteInt) -> Self {
        value.0
    }
}
impl TryInto<u16> for VarByteInt {
    type Error = TryFromIntError;

    fn try_into(self) -> Result<u16, Self::Error> {
        u16::try_from(self.0)
    }
}
impl TryInto<u8> for VarByteInt {
    type Error = TryFromIntError;

    fn try_into(self) -> Result<u8, Self::Error> {
        u8::try_from(self.0)
    }
}
