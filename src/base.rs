use core::{hint::assert_unchecked, num::NonZero};

use crate::{
    types::{
        MqttBinary, MqttString, MqttStringError, PacketIdentifier, TopicName, VarByteInt,
        v5::{MqttStringPair, property::PropertyType},
    },
    v5::{NeverSubscriptionIdentifier, NeverSubscriptionIdentifierError},
};

/// Marker trait establishing that [`AsRef::as_ref`] always returns
/// equal slices with equal length and contens.
///
/// # Safety
///
/// The [`AsRef`] implementation must always return equal slices.
pub unsafe trait StableRef: AsRef<[u8]> {}

unsafe impl StableRef for [u8] {}
unsafe impl StableRef for &[u8] {}
unsafe impl StableRef for str {}
unsafe impl StableRef for &str {}
unsafe impl StableRef for bytes::Bytes {}
unsafe impl StableRef for bytes::BytesMut {}

pub unsafe trait ParserBase: StableRef + Sized {
    fn split_to_checked(&mut self, at: usize) -> Option<Self>;
    fn advance_checked(&mut self, n: usize) -> Option<()> {
        self.split_to_checked(n).map(core::mem::drop)
    }
    fn split_to(&mut self, at: usize) -> Self {
        match self.split_to_checked(at) {
            Some(head) => head,
            None => panic!("mid > len"),
        }
    }
    fn advance(&mut self, n: usize) {
        self.advance_checked(n).unwrap()
    }
    fn len(&self) -> usize {
        self.as_ref().len()
    }
}

unsafe impl ParserBase for &[u8] {
    fn split_to_checked(&mut self, at: usize) -> Option<Self> {
        self.split_at_checked(at).map(|(head, tail)| {
            *self = tail;
            head
        })
    }
}

unsafe impl ParserBase for bytes::Bytes {
    fn split_to_checked(&mut self, at: usize) -> Option<Self> {
        (at <= self.len()).then(|| self.split_to(at))
    }
}
unsafe impl ParserBase for bytes::BytesMut {
    fn split_to_checked(&mut self, at: usize) -> Option<Self> {
        (at <= self.len()).then(|| self.split_to(at))
    }
}

pub trait Storage: Default + Extend<Self::Element> {
    type Element;
    const LIMIT: Option<usize>;
    fn len(&self) -> usize;
}

impl<T> Storage for alloc::vec::Vec<T> {
    type Element = T;
    const LIMIT: Option<usize> = None;
    fn len(&self) -> usize {
        self.len()
    }
}
#[cfg(feature = "heapless")]
impl<T, const N: usize> Storage for heapless::Vec<T, N> {
    type Element = T;
    const LIMIT: Option<usize> = Some(N);
    fn len(&self) -> usize {
        self.as_slice().len()
    }
}

pub trait FromParserBase<B: ParserBase>: Sized {
    type Error;
    fn parse(b: &mut B) -> Result<Self, Self::Error>;
}

pub struct TooShort;
pub enum NonZeroError {
    TooShort,
    Zero,
}
pub enum BoolError {
    TooShort,
    InvalidValue,
}
pub enum VarByteIntError {
    TooShort,
    MaxEncodableExceeded,
    NotMinimal,
}
pub struct PacketIdentifierError(pub(crate) NonZeroError);
pub enum StringError {
    TooShort,
    Utf8Error,
    NullCharacter,
}
pub enum TopicNameError {
    String(StringError),
    Topic(crate::types::TopicNameError),
}
pub enum PropertyTypeError {
    TooShort,
    InvalidIdentifier,
}

impl<B: ParserBase> FromParserBase<B> for NeverSubscriptionIdentifier {
    type Error = NeverSubscriptionIdentifierError;
    fn parse(b: &mut B) -> Result<Self, Self::Error> {
        Err(NeverSubscriptionIdentifierError)
    }
}

impl<B: ParserBase, const N: usize> FromParserBase<B> for [u8; N] {
    type Error = TooShort;
    fn parse(b: &mut B) -> Result<Self, Self::Error> {
        b.split_to_checked(N).ok_or(TooShort).map(|b| {
            let b = b.as_ref();

            // SAFETY: split_to_checked guarantees this is valid length
            unsafe {
                assert_unchecked(b.len() == N);
            }

            let mut a = [0; N];
            a.copy_from_slice(b);
            a
        })
    }
}

impl<B: ParserBase> FromParserBase<B> for u8 {
    type Error = TooShort;
    fn parse(b: &mut B) -> Result<Self, Self::Error> {
        <[u8; 1]>::parse(b).map(u8::from_be_bytes)
    }
}
impl<B: ParserBase> FromParserBase<B> for u16 {
    type Error = TooShort;
    fn parse(b: &mut B) -> Result<Self, Self::Error> {
        <[u8; 2]>::parse(b).map(u16::from_be_bytes)
    }
}
impl<B: ParserBase> FromParserBase<B> for u32 {
    type Error = TooShort;
    fn parse(b: &mut B) -> Result<Self, Self::Error> {
        <[u8; 4]>::parse(b).map(u32::from_be_bytes)
    }
}

impl<B: ParserBase> FromParserBase<B> for NonZero<u8> {
    type Error = NonZeroError;
    fn parse(b: &mut B) -> Result<Self, Self::Error> {
        u8::parse(b)
            .map_err(|_| NonZeroError::TooShort)
            .and_then(|i| NonZero::new(i).ok_or(NonZeroError::Zero))
    }
}
impl<B: ParserBase> FromParserBase<B> for NonZero<u16> {
    type Error = NonZeroError;
    fn parse(b: &mut B) -> Result<Self, Self::Error> {
        u16::parse(b)
            .map_err(|_| NonZeroError::TooShort)
            .and_then(|i| NonZero::new(i).ok_or(NonZeroError::Zero))
    }
}
impl<B: ParserBase> FromParserBase<B> for NonZero<u32> {
    type Error = NonZeroError;
    fn parse(b: &mut B) -> Result<Self, Self::Error> {
        u32::parse(b)
            .map_err(|_| NonZeroError::TooShort)
            .and_then(|i| NonZero::new(i).ok_or(NonZeroError::Zero))
    }
}

impl<B: ParserBase> FromParserBase<B> for bool {
    type Error = BoolError;
    fn parse(b: &mut B) -> Result<Self, Self::Error> {
        u8::parse(b)
            .map_err(|_| BoolError::TooShort)
            .and_then(|b| Self::try_from(b).map_err(|_| BoolError::InvalidValue))
    }
}
impl<B: ParserBase> FromParserBase<B> for VarByteInt {
    type Error = VarByteIntError;
    fn parse(b: &mut B) -> Result<Self, Self::Error> {
        VarByteInt::parse_regular(b).map_err(|e| match e {
            crate::types::VarByteIntError::TooShort => VarByteIntError::TooShort,
            crate::types::VarByteIntError::FourBytesExceeded => {
                VarByteIntError::MaxEncodableExceeded
            }
            crate::types::VarByteIntError::EncodingNotMinimal => VarByteIntError::NotMinimal,
        })
    }
}
impl<B: ParserBase> FromParserBase<B> for PacketIdentifier {
    type Error = PacketIdentifierError;
    fn parse(b: &mut B) -> Result<Self, Self::Error> {
        NonZero::parse(b)
            .map_err(PacketIdentifierError)
            .map(Self::new)
    }
}

impl<B: ParserBase> FromParserBase<B> for MqttBinary<B> {
    type Error = TooShort;

    fn parse(b: &mut B) -> Result<Self, Self::Error> {
        let len = u16::parse(b).map(usize::from)?;
        let data = b.split_to_checked(len).ok_or(TooShort)?;

        // SAFETY: split_to_unchecked called with a length value obtained from a u16
        //         guarantees that the slice is not longer than MqttBinary::MAX_LENGTH
        Ok(unsafe { MqttBinary::new_unchecked(data) })
    }
}
impl<B: ParserBase> FromParserBase<B> for MqttString<B> {
    type Error = StringError;

    fn parse(b: &mut B) -> Result<Self, Self::Error> {
        let binary = MqttBinary::parse(b).map_err(|e| StringError::TooShort)?;

        match MqttString::new(binary) {
            Ok(s) => Ok(s),
            Err(MqttStringError::Utf8Error(_)) => Err(StringError::Utf8Error),
            Err(MqttStringError::NullCharacter) => Err(StringError::NullCharacter),
            Err(MqttStringError::TooLargeToEncode) => unreachable!(),
        }
    }
}

impl<B: ParserBase> FromParserBase<B> for MqttStringPair<B> {
    type Error = StringError;

    fn parse(b: &mut B) -> Result<Self, Self::Error> {
        let name = MqttString::parse(b)?;
        let value = MqttString::parse(b)?;

        Ok(Self::new(name, value))
    }
}
impl<B: ParserBase> FromParserBase<B> for TopicName<B> {
    type Error = TopicNameError;

    fn parse(b: &mut B) -> Result<Self, Self::Error> {
        let string = MqttString::parse(b).map_err(TopicNameError::String)?;

        TopicName::new(string).map_err(TopicNameError::Topic)
    }
}
impl<B: ParserBase> FromParserBase<B> for PropertyType {
    type Error = PropertyTypeError;
    fn parse(b: &mut B) -> Result<Self, Self::Error> {
        u8::parse(b)
            .map_err(|_| PropertyTypeError::TooShort)
            .and_then(|i| Self::from_identifier(i).ok_or(PropertyTypeError::InvalidIdentifier))
    }
}
