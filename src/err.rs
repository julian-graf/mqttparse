/// An error in parsing.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Error {
    // COMMON ERRORS
    /// The packet type value is reserved.
    ReservedPacketType,
    /// The packet type is not expected to be sent by the peer.
    UnexpectedPacketType,
    /// The 4-Bit flags field in the fixed header is invalid.
    InvalidFixedHeaderFlags,
    /// The remaining length field of the fixed header is invalid.
    InvalidRemainingLength,
    /// The packet is too large for the target's pointer size.
    PacketTooLarge,
    /// The version field in CONNECT is not matching the parser.
    IncorrectVersion,
    /// A field in the variable header or payload exceeds the remaining length.
    RemainingLengthExceeded,
    /// The property length is greater than the remaining packet length.
    InvalidPropertyLength,
    // COMMON ERRORS

    // PRIMITIVE VALUE ERRORS
    /// A boolean field has a value other than `0` or `1`.
    InvalidBoolValue,
    /// An [`MqttString`] contained a null character.
    ///
    /// [`MqttString`]: crate::types::MqttString
    NullCharacterString,
    /// An [`MqttString`] contains invalid UTF-8
    ///
    /// [`MqttString`]: crate::types::MqttString
    InvalidUtf8,
    /// A [`VarByteInt`] encoding is not minimal.
    ///
    /// [`VarByteInt`]: crate::types::VarByteInt
    NonMinimalVarByteInt,
    /// A [`VarByteInt`] encoding is longer than four bytes.
    ///
    /// [`VarByteInt`]: crate::types::VarByteInt
    TooLongVarByteInt,
    // PRIMITIVE VALUE ERRORS

    // COMPLEX VALUE ERRORS
    /// A [`TopicName`] is empty or contains a wildcard.
    ///
    /// [`TopicName`]: crate::types::TopicName
    InvalidTopicName,
    /// An MQTTv5 PUBLISH packet contains neither a non-empty topic name nor a topic alias.
    NoTopicNameNoAlias,
    /// A [`PacketIdentifier`] field has the value `0`.
    ///
    /// [`PacketIdentifier`]: crate::types::PacketIdentifier
    ZeroPacketIdentifier,
    /// An MQTTv5 packet contains an invalid property type.
    InvalidPropertyType,
    /// An MQTTv5 packet contains a value that is not a valid reason code.
    InvalidReasonCode,
    /// An MQTTv5 packet contains a reason code that is not allowed in this packet type.
    IllegalReasonCode,
    // COMPLEX VALUE ERRORS

    // OTHER ERRORS
    /// A property that is only allowed once occured twice.
    DuplicateProperty,
    /// A property type is not allowed for this packet type.
    IllegalPropertyType,
    /// A property of an MQTTv5 packet is encoded incorrectly.
    InvalidProperty,
    /// The [`MaximumQoS`] property has a value other than [`QoS::AtMostOnce`] and [`QoS::AtLeastOnce`].
    ///
    /// [`MaximumQoS`]: crate::types::v5::property::MaximumQoS
    /// [`QoS::AtMostOnce`]: crate::types::QoS::AtMostOnce
    /// [`QoS::AtLeastOnce`]: crate::types::QoS::AtLeastOnce
    InvalidMaximumQoS,
    /// A property exceeded the property length field of an MQTTv5 packet
    PropertyLengthExceeded,
}

impl Error {
    #[inline]
    fn description_str(&self) -> &'static str {
        match self {
            Self::ReservedPacketType => "reserved packet type",
            Self::UnexpectedPacketType => "unexpected packet type",
            Self::InvalidFixedHeaderFlags => "invalid fixed header flags",
            Self::InvalidRemainingLength => "incorrect remaining length encoding",
            Self::PacketTooLarge => "packet is too large for target's pointer size",
            Self::IncorrectVersion => "incorrect version field",
            Self::RemainingLengthExceeded => "not enough remaining length to parse field",
            Self::InvalidPropertyLength => "property length is greater than remaining data",

            Self::InvalidBoolValue => "non 0 or 1 value in boolean field",
            Self::NullCharacterString => "null character in string",
            Self::InvalidUtf8 => "invalid UTF-8 in string",
            Self::NonMinimalVarByteInt => "variable byte integer encoding is not minimal",
            Self::TooLongVarByteInt => "variable byte interger encoding is longer than 4 bytes",

            Self::ZeroPacketIdentifier => "value of 0 in packet identifier field",
            Self::InvalidTopicName => "topic is empty or contains wildcard",
            Self::NoTopicNameNoAlias => "empty topic name without topic alias",
            Self::InvalidPropertyType => "invalid property type",
            Self::InvalidReasonCode => "invalid reason code",
            Self::IllegalReasonCode => "illegal reason code",

            Self::DuplicateProperty => "at most once property occured twice",
            Self::IllegalPropertyType => "property type not allowed",
            Self::InvalidProperty => "invalid property encoding",
            Self::InvalidMaximumQoS => "invalid maximum qos",
            Self::PropertyLengthExceeded => "not enough property length to parse property",
        }
    }
    #[inline]
    fn category(&self) -> ErrorCategory {
        match self {
            Self::ReservedPacketType => ErrorCategory::MalformedPacket,
            Self::UnexpectedPacketType => ErrorCategory::ProtocolError,
            Self::InvalidFixedHeaderFlags => ErrorCategory::MalformedPacket,
            Self::InvalidRemainingLength => ErrorCategory::MalformedPacket,
            Self::PacketTooLarge => ErrorCategory::Tbd,
            Self::IncorrectVersion => ErrorCategory::Tbd,
            Self::RemainingLengthExceeded => ErrorCategory::MalformedPacket,
            Self::InvalidPropertyLength => ErrorCategory::MalformedPacket,

            Self::InvalidBoolValue => ErrorCategory::MalformedPacket,
            Self::NullCharacterString => ErrorCategory::MalformedPacket,
            Self::InvalidUtf8 => ErrorCategory::MalformedPacket,
            Self::NonMinimalVarByteInt => ErrorCategory::Tbd,
            Self::TooLongVarByteInt => ErrorCategory::MalformedPacket,

            Self::ZeroPacketIdentifier => ErrorCategory::ProtocolError,
            Self::InvalidTopicName => ErrorCategory::Tbd,
            Self::NoTopicNameNoAlias => ErrorCategory::ProtocolError,
            Self::InvalidPropertyType => ErrorCategory::MalformedPacket,
            Self::InvalidReasonCode => ErrorCategory::MalformedPacket,
            Self::IllegalReasonCode => ErrorCategory::ProtocolError,

            Self::DuplicateProperty => ErrorCategory::ProtocolError,
            // Malformed packet according to <https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901029>
            Self::IllegalPropertyType => ErrorCategory::MalformedPacket,
            // Malformed packet according to <https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901029>
            Self::InvalidProperty => ErrorCategory::MalformedPacket,
            Self::InvalidMaximumQoS => ErrorCategory::ProtocolError,
            Self::PropertyLengthExceeded => ErrorCategory::MalformedPacket,
        }
    }
}
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.description_str())
    }
}
impl core::error::Error for Error {
    fn description(&self) -> &str {
        self.description_str()
    }
}

/// The category of an error.
///
/// This is somewhat how the parsing party should act on the error.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ErrorCategory {
    MalformedPacket,
    ProtocolError,
    Tbd,
}

impl ErrorCategory {
    #[inline]
    fn description_str(&self) -> &'static str {
        match self {
            Self::MalformedPacket => "malformed packet",
            Self::ProtocolError => "protocol error",
            Self::Tbd => todo!(),
        }
    }
}
impl From<Error> for ErrorCategory {
    fn from(e: Error) -> Self {
        e.category()
    }
}
impl core::fmt::Display for ErrorCategory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.description_str())
    }
}
impl core::error::Error for ErrorCategory {
    fn description(&self) -> &str {
        self.description_str()
    }
}

mod froms {
    use crate::{
        Error,
        base::{
            BoolError, NonZeroError, PacketIdentifierError, PropertyTypeError, ReasonCodeError,
            StringError, TooShort, TopicNameError, VarByteIntError,
        },
        types::v5::property::{MaximumQoS, MaximumQoSError, NeverPropertyError, PropertyError},
    };

    impl From<TooShort> for Error {
        fn from(_: TooShort) -> Self {
            Self::RemainingLengthExceeded
        }
    }
    impl From<BoolError> for Error {
        fn from(e: BoolError) -> Self {
            match e {
                BoolError::TooShort => Self::RemainingLengthExceeded,
                BoolError::InvalidValue => Self::InvalidBoolValue,
            }
        }
    }
    impl From<NonZeroError> for Error {
        fn from(e: NonZeroError) -> Self {
            match e {
                NonZeroError::TooShort => Self::RemainingLengthExceeded,
                NonZeroError::Zero => todo!(), // Would be a protocol error in case of Receive maximum
            }
        }
    }
    impl From<PacketIdentifierError> for Error {
        fn from(PacketIdentifierError(e): PacketIdentifierError) -> Self {
            match e {
                NonZeroError::TooShort => Self::RemainingLengthExceeded,
                NonZeroError::Zero => Self::ZeroPacketIdentifier,
            }
        }
    }
    impl From<StringError> for Error {
        fn from(e: StringError) -> Self {
            match e {
                StringError::TooShort => Self::RemainingLengthExceeded,
                StringError::Utf8Error => Self::InvalidUtf8,
                StringError::NullCharacter => Self::NullCharacterString,
            }
        }
    }
    impl From<TopicNameError> for Error {
        fn from(e: TopicNameError) -> Self {
            match e {
                TopicNameError::String(e) => e.into(),
                TopicNameError::Topic(e) => Self::InvalidTopicName,
            }
        }
    }
    impl From<VarByteIntError> for Error {
        fn from(e: VarByteIntError) -> Self {
            match e {
                VarByteIntError::TooShort => Self::RemainingLengthExceeded,
                VarByteIntError::MaxEncodableExceeded => Self::TooLongVarByteInt,
                VarByteIntError::NotMinimal => Self::NonMinimalVarByteInt,
            }
        }
    }
    impl From<PropertyTypeError> for Error {
        fn from(e: PropertyTypeError) -> Self {
            match e {
                PropertyTypeError::TooShort => Self::RemainingLengthExceeded,
                PropertyTypeError::InvalidIdentifier => Self::InvalidPropertyType,
            }
        }
    }
    impl From<ReasonCodeError> for Error {
        fn from(e: ReasonCodeError) -> Self {
            match e {
                ReasonCodeError::TooShort => Self::RemainingLengthExceeded,
                ReasonCodeError::InvalidIdentifier => Self::InvalidReasonCode,
            }
        }
    }
    impl From<MaximumQoSError> for Error {
        fn from(e: MaximumQoSError) -> Self {
            match e {
                MaximumQoSError::TooShort => Self::RemainingLengthExceeded,
                MaximumQoSError::IllegalValue => Self::InvalidProperty,
            }
        }
    }
    impl<E: Into<Error>> From<PropertyError<E>> for Error {
        fn from(e: PropertyError<E>) -> Self {
            match e.0.into() {
                Self::RemainingLengthExceeded => Self::PropertyLengthExceeded,
                other => other,
            }
        }
    }
    impl From<NeverPropertyError> for Error {
        fn from(_: NeverPropertyError) -> Self {
            Self::IllegalPropertyType
        }
    }
}
