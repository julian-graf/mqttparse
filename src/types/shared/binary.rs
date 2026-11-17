use core::hint::unreachable_unchecked;

use crate::{
    base::{ParserBase, StableRef},
    types::TooLargeToEncode,
};

/// Arbitrary binary data with a length less than or equal to [`MqttBinary::MAX_LENGTH`] ([`u16::MAX`]).
/// Exceeding this size ultimately leads to malformed packets.
///
/// # Examples
///
/// ```rust
/// use rust_mqtt::Bytes;
/// use rust_mqtt::types::{MqttBinary, MqttString, TooLargeToEncode};
///
/// let slice = [0x00; MqttBinary::MAX_LENGTH];
/// let too_long = [0x00; MqttBinary::MAX_LENGTH + 1];
///
/// let b = MqttBinary::from_slice(&slice)?;
/// assert_eq!(b.as_bytes(), &slice);
/// assert!(MqttBinary::from_slice(&too_long).is_err());
///
/// let b = MqttBinary::from_bytes(Bytes::Borrowed(&slice))?;
/// assert_eq!(b.as_bytes(), &slice);
/// assert!(MqttBinary::from_bytes(Bytes::Borrowed(&too_long)).is_err());
///
/// let from_slice_unchecked = MqttBinary::from_slice_unchecked(&slice);
/// assert_eq!(from_slice_unchecked.as_bytes(), &slice);
///
/// let from_bytes_unchecked = MqttBinary::from_bytes_unchecked(Bytes::Borrowed(&slice));
/// assert_eq!(from_bytes_unchecked.as_bytes(), &slice);
///
/// let from_vec = MqttBinary::try_from(vec![0, 1, 2])?;
/// assert_eq!(from_vec.as_bytes(), &[0, 1, 2]);
///
/// let from_boxed_slice = MqttBinary::try_from(vec![3, 4, 5].into_boxed_slice())?;
/// assert_eq!(from_boxed_slice.as_bytes(), &[3, 4, 5]);
///
/// # Ok::<(), TooLargeToEncode>(())
/// ```
#[derive(Default, Clone, PartialEq, Eq)]
pub struct MqttBinary<B>(B);

impl<B: StableRef> core::fmt::Debug for MqttBinary<B> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MqttBinary").field(&self.as_ref()).finish()
    }
}

impl<'b> TryFrom<&'b str> for MqttBinary<&'b str> {
    type Error = TooLargeToEncode;

    fn try_from(value: &'b str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<B: StableRef> AsRef<[u8]> for MqttBinary<B> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<B> MqttBinary<B> {
    /// The maximum length of binary data so that it can be encoded. This value is limited by the 2-byte length field.
    pub const MAX_LENGTH: usize = u16::MAX as usize;
}

impl<B: StableRef> MqttBinary<B> {
    /// Converts `B` into [`MqttBinary`] by checking for the max length of
    /// [`MqttBinary::MAX_LENGTH`].
    ///
    /// # Errors
    ///
    /// Returns [`TooLargeToEncode`] if `bytes`' length exceeds [`MqttBinary::MAX_LENGTH`].
    pub fn new(bytes: B) -> Result<Self, TooLargeToEncode> {
        match bytes.as_ref().len() {
            ..=Self::MAX_LENGTH => Ok(Self(bytes)),
            _ => Err(TooLargeToEncode),
        }
    }

    /// Converts `B` into [`MqttBinary`] without checking for the max length of
    /// [`MqttBinary::MAX_LENGTH`].
    ///
    /// # Safety
    ///
    /// The length of the slice parameter in bytes must less than or equal to
    /// [`MqttBinary::MAX_LENGTH`].
    ///
    /// # Panics
    ///
    /// In debug builds, this function will panic if the bytes' length is greater than
    /// [`MqttBinary::MAX_LENGTH`].
    #[must_use]
    pub unsafe fn new_unchecked(bytes: B) -> Self {
        debug_assert!(
            bytes.as_ref().len() <= Self::MAX_LENGTH,
            "the slice's length exceeds MAX_LENGTH"
        );

        Self(bytes)
    }

    /// Returns the length of the underlying data.
    #[inline]
    #[must_use]
    pub fn len(&self) -> u16 {
        /// SAFETY: Invariant guarantees a length <= `u16::MAX`
        unsafe {
            u16::try_from(self.0.as_ref().len()).unwrap_unchecked()
        }
    }

    /// Returns whether the underlying data is empty.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the underlying bytes as `&[u8]`
    #[inline]
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_ref()
    }
}
