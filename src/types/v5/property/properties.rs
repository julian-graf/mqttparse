use core::num::NonZero;

use crate::{
    base::{BoolError, FromParserBase, NonZeroError, ParserBase, StableRef, TooShort},
    types::{
        KeepAlive, MqttBinary, MqttString, QoS, TopicName, VarByteInt,
        v5::{
            MqttStringPair,
            property::{Property, PropertyError, PropertyType},
        },
    },
};

#[derive(Debug)]
pub struct NeverProperty(!);
pub struct NeverPropertyError;

impl<B: ParserBase> FromParserBase<B> for NeverProperty {
    type Error = NeverPropertyError;

    fn parse(b: &mut B) -> core::result::Result<Self, Self::Error> {
        Err(NeverPropertyError)
    }
}

/// Implements a newtype with the given identifier and wrapped type.
///
/// * Implements [`FromParserBase`]: Only content is read.
macro_rules! property {
    ($name:ident, $inner:ty) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name(pub(crate) $inner);

        impl crate::types::v5::property::Property for $name {
            const TYPE: crate::types::v5::property::PropertyType =
                crate::types::v5::property::PropertyType::$name;
            type Inner = $inner;
        }
        impl From<$name> for $inner {
            fn from(value: $name) -> Self {
                value.0
            }
        }
        impl<B: crate::base::ParserBase> crate::base::FromParserBase<B> for $name {
            type Error = crate::types::v5::property::PropertyError<
                <$inner as crate::base::FromParserBase<B>>::Error,
            >;
            fn parse(b: &mut B) -> Result<Self, Self::Error> {
                <$inner as crate::base::FromParserBase<B>>::parse(b)
                    .map(Self)
                    .map_err(crate::types::v5::property::PropertyError)
            }
        }
    };
    ($name:ident < $b:ident >, $inner:ident) => {
        #[derive(Clone, PartialEq, Eq)]
        pub struct $name<$b>(pub(crate) $inner<$b>);

        impl<B> crate::types::v5::property::Property for $name<B> {
            const TYPE: crate::types::v5::property::PropertyType =
                crate::types::v5::property::PropertyType::$name;
            type Inner = $inner<B>;
        }
        impl<B> From<$name<B>> for $inner<B> {
            fn from(value: $name<B>) -> Self {
                value.0
            }
        }
        impl<B: crate::base::ParserBase> crate::base::FromParserBase<B> for $name<B> {
            type Error = crate::types::v5::property::PropertyError<
                <$inner<B> as crate::base::FromParserBase<B>>::Error,
            >;
            fn parse(b: &mut B) -> Result<Self, Self::Error> {
                <$inner<B> as crate::base::FromParserBase<B>>::parse(b)
                    .map(Self)
                    .map_err(crate::types::v5::property::PropertyError)
            }
        }

        impl<B: crate::base::StableRef> core::fmt::Debug for $name<B> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_tuple(stringify!($name)).field(&&self.0).finish()
            }
        }
    };
}

property!(PayloadFormatIndicator, bool);
property!(MessageExpiryInterval, u32);
property!(ContentType<B>, MqttString);
property!(ResponseTopic<B>, TopicName);
property!(CorrelationData<B>, MqttBinary);
property!(SubscriptionIdentifier, VarByteInt);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SessionExpiryInterval(crate::types::v5::SessionExpiryInterval);
property!(AssignedClientIdentifier<B>, MqttString);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// #[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ServerKeepAlive(KeepAlive);
property!(AuthenticationMethod<B>, MqttString);
property!(AuthenticationData<B>, MqttBinary);
property!(RequestProblemInformation, bool);
property!(WillDelayInterval, u32);
property!(RequestResponseInformation, bool);
property!(ResponseInformation<B>, MqttString);
property!(ServerReference<B>, MqttString);
property!(ReasonString<B>, MqttString);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// #[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ReceiveMaximum(NonZero<u16>);
property!(TopicAliasMaximum, u16);
property!(TopicAlias, NonZero<u16>);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// #[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct MaximumQoS(QoS);
property!(RetainAvailable, bool);
property!(UserProperty<B>, MqttStringPair);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaximumPacketSize(crate::types::v5::MaximumPacketSize);
property!(WildcardSubscriptionAvailable, bool);
property!(SubscriptionIdentifierAvailable, bool);
property!(SharedSubscriptionAvailable, bool);

impl Property for SessionExpiryInterval {
    const TYPE: PropertyType = PropertyType::SessionExpiryInterval;
    type Inner = crate::types::v5::SessionExpiryInterval;
}
impl From<SessionExpiryInterval> for crate::types::v5::SessionExpiryInterval {
    fn from(value: SessionExpiryInterval) -> Self {
        value.0
    }
}
impl<B: ParserBase> FromParserBase<B> for SessionExpiryInterval {
    type Error = PropertyError<TooShort>;
    fn parse(b: &mut B) -> Result<Self, Self::Error> {
        let value = u32::parse(b).map_err(PropertyError)?;

        Ok(Self(match value {
            0 => crate::types::v5::SessionExpiryInterval::EndOnDisconnect,
            // Safety: the value is in the range 1..u32::MAX which does not include zero
            1..u32::MAX => crate::types::v5::SessionExpiryInterval::Seconds(unsafe {
                NonZero::new_unchecked(value)
            }),
            u32::MAX => crate::types::v5::SessionExpiryInterval::NeverEnd,
        }))
    }
}

impl Property for ServerKeepAlive {
    const TYPE: PropertyType = PropertyType::ServerKeepAlive;
    type Inner = crate::types::KeepAlive;
}
impl From<ServerKeepAlive> for crate::types::KeepAlive {
    fn from(value: ServerKeepAlive) -> Self {
        value.0
    }
}
impl<B: ParserBase> FromParserBase<B> for ServerKeepAlive {
    type Error = PropertyError<TooShort>;
    fn parse(b: &mut B) -> Result<Self, Self::Error> {
        u16::parse(b)
            .map(NonZero::new)
            .map(|v| v.map_or(KeepAlive::Infinite, KeepAlive::Seconds))
            .map(Self)
            .map_err(PropertyError)
    }
}

impl Property for ReceiveMaximum {
    const TYPE: PropertyType = PropertyType::ReceiveMaximum;
    type Inner = NonZero<u16>;
}
impl From<ReceiveMaximum> for NonZero<u16> {
    fn from(value: ReceiveMaximum) -> Self {
        value.0
    }
}
impl<B: ParserBase> FromParserBase<B> for ReceiveMaximum {
    type Error = PropertyError<NonZeroError>;
    fn parse(b: &mut B) -> Result<Self, Self::Error> {
        NonZero::parse(b).map(Self).map_err(PropertyError)
    }
}

pub enum MaximumQoSError {
    TooShort,
    IllegalValue,
}
impl Property for MaximumQoS {
    const TYPE: PropertyType = PropertyType::MaximumQoS;
    type Inner = QoS;
}
impl From<MaximumQoS> for QoS {
    fn from(value: MaximumQoS) -> Self {
        value.0
    }
}
impl<B: ParserBase> FromParserBase<B> for MaximumQoS {
    type Error = PropertyError<MaximumQoSError>;
    fn parse(b: &mut B) -> Result<Self, Self::Error> {
        bool::parse(b)
            .map(|b| if b { QoS::AtLeastOnce } else { QoS::AtMostOnce })
            .map(Self)
            .map_err(|e| match e {
                BoolError::TooShort => MaximumQoSError::TooShort,
                BoolError::InvalidValue => MaximumQoSError::IllegalValue,
            })
            .map_err(PropertyError)
    }
}

impl Property for MaximumPacketSize {
    const TYPE: PropertyType = PropertyType::MaximumPacketSize;
    type Inner = crate::types::v5::MaximumPacketSize;
}
impl From<MaximumPacketSize> for crate::types::v5::MaximumPacketSize {
    fn from(value: MaximumPacketSize) -> Self {
        value.0
    }
}
impl<B: ParserBase> FromParserBase<B> for MaximumPacketSize {
    type Error = PropertyError<NonZeroError>;
    fn parse(b: &mut B) -> Result<Self, Self::Error> {
        NonZero::parse(b)
            .map(crate::types::v5::MaximumPacketSize::Limit)
            .map(Self)
            .map_err(PropertyError)
    }
}
