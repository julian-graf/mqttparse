use core::str::Utf8Error;

use crate::{
    base::StableRef,
    types::{MqttBinary, TooLargeToEncode},
};

/// Error returned when creating [`MqttString`] failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MqttStringError {
    /// The passed data is not valid UTF-8.
    Utf8Error(Utf8Error),

    /// The passed data contains at least one null character.
    NullCharacter,

    /// The passed data exceeds the max length of [`MqttString::MAX_LENGTH`].
    TooLargeToEncode,
}

impl From<Utf8Error> for MqttStringError {
    fn from(e: Utf8Error) -> Self {
        Self::Utf8Error(e)
    }
}
impl From<TooLargeToEncode> for MqttStringError {
    fn from(_: TooLargeToEncode) -> Self {
        Self::TooLargeToEncode
    }
}

/// Arbitrary UTF-8 encoded string with a length in bytes less than or equal to
/// [`MqttString::MAX_LENGTH`] ([`u16::MAX`]) and no null characters.
/// Exceeding this size ultimately leads to malformed packets.
///
/// # Examples
///
/// ```rust
/// use rust_mqtt::types::{MqttBinary, MqttString, MqttStringError};
///
/// let bytes = [b'a'; MqttString::MAX_LENGTH];
/// let too_long = [b'a'; MqttString::MAX_LENGTH + 1];
/// let null_character = "hi\0there";
///
/// let slice = core::str::from_utf8(&bytes)?;
/// let too_long = core::str::from_utf8(&too_long)?;
///
/// let b = MqttBinary::from_slice(&bytes)?;
/// let s = MqttString::from_utf8_binary(b)?;
/// assert_eq!(s.as_str(), slice);
/// let b = MqttBinary::from_slice(null_character.as_bytes())?;
/// assert_eq!(MqttString::from_utf8_binary(b).unwrap_err(), MqttStringError::NullCharacter);
///
/// let s = MqttString::from_str(slice)?;
/// assert_eq!(s.as_str(), slice);
/// assert_eq!(MqttString::from_str(too_long).unwrap_err(), MqttStringError::TooLargeToEncode);
/// assert_eq!(MqttString::from_str(&null_character).unwrap_err(), MqttStringError::NullCharacter);
///
/// let s = MqttString::from_str_unchecked(slice);
/// assert_eq!(s.as_str(), slice);
///
/// let b = MqttBinary::from_slice_unchecked(slice.as_bytes());
/// let s = unsafe { MqttString::from_utf8_binary_unchecked(b) };
/// assert_eq!(s.as_str(), slice);
///
/// let from_string = MqttString::try_from("abc".to_string())?;
/// assert_eq!(from_string.as_str(), "abc");
///
/// let from_vec = MqttString::try_from(vec![b'd', b'e', b'f'])?;
/// assert_eq!(from_vec.as_str(), "def");
///
/// let from_boxed_str = MqttString::try_from(Box::<str>::from("ghi"))?;
/// assert_eq!(from_boxed_str.as_str(), "ghi");
///
/// let from_boxed_byte_slice = MqttString::try_from(vec![b'j', b'k', b'l'].into_boxed_slice())?;
/// assert_eq!(from_boxed_byte_slice.as_str(), "jkl");
///
/// # Ok::<(), MqttStringError>(())
/// ```
#[derive(Default, Clone, PartialEq, Eq)]
pub struct MqttString<B>(MqttBinary<B>);

impl<B: StableRef> core::fmt::Debug for MqttString<B> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MqttString").field(&self.as_str()).finish()
    }
}

impl<B: StableRef> AsRef<str> for MqttString<B> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<B> MqttString<B> {
    /// The maximum length of a string in bytes so that it can be encoded.
    /// This value is limited by the 2-byte length field.
    pub const MAX_LENGTH: usize = MqttBinary::<B>::MAX_LENGTH;
}

impl<B: StableRef> MqttString<B> {
    /// Converts [`MqttBinary`] into [`MqttString`] by checking for null characters and valid UTF-8.
    /// Valid length is guaranteed by [`MqttBinary`]'s invariant.
    ///
    /// # Errors
    ///
    /// * [`MqttStringError::Utf8Error`] if `b` is not valid UTF-8.
    /// * [`MqttStringError::NullCharacter`] if `b` contains an ASCII `\0` character.
    /// * [`MqttStringError::TooLargeToEncode`] if `b`'s length exceeds [`MqttString::MAX_LENGTH`].
    pub fn new(b: MqttBinary<B>) -> Result<Self, MqttStringError> {
        if b.as_bytes().contains(&0) {
            return Err(MqttStringError::NullCharacter);
        }
        if let Err(e) = str::from_utf8(b.as_bytes()) {
            return Err(MqttStringError::Utf8Error(e));
        }

        // SAFETY: UTF-8 and length safety invariants checked
        Ok(unsafe { Self::new_unchecked(b) })
    }

    /// Converts [`MqttBinary`] into [`MqttString`] without checking for null characters or valid UTF-8.
    /// Valid length is guaranteed by [`MqttBinary`]'s invariant.
    ///
    /// # Safety
    ///
    /// The binary passed in must be valid UTF-8.
    ///
    /// # Invariants
    ///
    /// The binary data does not contain any null characters.
    ///
    /// # Panics
    ///
    /// In debug builds, this function will panic if the binary contains a null character or is not
    /// valid UTF-8.
    #[must_use]
    pub unsafe fn new_unchecked(b: MqttBinary<B>) -> Self {
        debug_assert!(!b.as_bytes().contains(&0));
        debug_assert!(str::from_utf8(b.as_bytes()).is_ok());

        Self(b)
    }

    /// Returns the length of the underlying data in bytes.
    #[inline]
    #[must_use]
    pub fn len(&self) -> u16 {
        self.0.len()
    }

    /// Returns whether the underlying data is empty.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the underlying string as `&str`
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        // Safety: MqttString contains valid UTF-8
        unsafe { str::from_utf8_unchecked(self.0.as_bytes()) }
    }
}

impl<'s> MqttString<&'s [u8]> {
    /// Converts a string slice into [`MqttString`] by checking for null characters and the max
    /// length of [`MqttString::MAX_LENGTH`].
    ///
    /// # Errors
    ///
    /// * [`MqttStringError::NullCharacter`] if `s` contains an ASCII `\0` character.
    /// * [`MqttStringError::TooLargeToEncode`] if `s`' length exceeds [`MqttString::MAX_LENGTH`].
    pub fn from_str(s: &'s str) -> Result<Self, MqttStringError> {
        if s.contains('\0') {
            return Err(MqttStringError::NullCharacter);
        }

        MqttBinary::new(s.as_bytes())
            .map_err(|_| MqttStringError::TooLargeToEncode)
            .map(Self)
    }

    /// Converts a string slice into [`MqttString`] without checking for null characters or the max
    /// length of [`MqttString::MAX_LENGTH`].
    ///
    /// # Safety
    ///
    /// The length of the string slice must be less than or equal to [`MqttString::MAX_LENGTH`].
    ///
    /// # Invariants
    ///
    /// The string slice must not contain any null characters.
    ///
    /// # Panics
    ///
    /// In debug builds, this function will panic if the slice contains a null character or its length is greater
    /// than [`MqttString::MAX_LENGTH`].
    #[must_use]
    pub unsafe fn from_str_unchecked(s: &'s str) -> Self {
        debug_assert!(!s.contains('\0'));

        // SAFETY: Safety preconditions
        Self(unsafe { MqttBinary::new_unchecked(s.as_bytes()) })
    }
}
