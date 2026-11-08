use core::num::NonZero;

use crate::{
    base::{FromParserBase, ParserBase, StableRef},
    types::{
        MqttBinary, MqttString, TopicName, VarByteInt,
        v5::{
            MqttStringPair,
            property::{Property, PropertyType},
        },
    },
};
/// Implements a newtype with the given identifier and wrapped type.
///
/// * Implements [`FromParserBase`]: Only content is read.
macro_rules! property {
    ($name:ident, $inner:ty) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name(pub(crate) $inner);

        impl Property for $name {
            const TYPE: PropertyType = PropertyType::$name;
            type Inner = $inner;
        }
        impl Into<$inner> for $name {
            fn into(self) -> $inner {
                self.0
            }
        }
        impl<B: ParserBase> FromParserBase<B> for $name {
            type Error = <$inner as FromParserBase<B>>::Error;
            fn parse(b: &mut B) -> Result<Self, Self::Error> {
                <$inner>::parse(b).map(Self)
            }
        }

        // impl<R: Read> Readable<R> for $name {
        //     async fn read(read: &mut R) -> Result<Self, ReadError<R::Error>> {
        //         let content = <$inner as Readable<R>>::read(read).await?;
        //         Ok(Self(content))
        //     }
        // }

        // impl Writable for $name {
        //     fn written_len(&self) -> usize {
        //         Self::TYPE.written_len() + self.0.written_len()
        //     }
        //     async fn write<W: Write>(&self, write: &mut W) -> Result<(), WriteError<W::Error>> {
        //         Self::TYPE.write(write).await?;
        //         self.0.write(write).await?;

        //         Ok(())
        //     }
        // }

        // impl From<$inner> for $name {
        //     fn from(value: $inner) -> Self {
        //         Self(value)
        //     }
        // }
    };
    ($name:ident < $b:ident >, $inner:ident) => {
        #[derive(Clone, PartialEq, Eq)]
        pub struct $name<$b>(pub(crate) $inner<$b>);

        impl<B> Property for $name<B> {
            const TYPE: PropertyType = PropertyType::$name;
            type Inner = $inner<B>;
        }
        impl<B> Into<$inner<B>> for $name<B> {
            fn into(self) -> $inner<B> {
                self.0
            }
        }
        impl<B: ParserBase> FromParserBase<B> for $name<B> {
            type Error = <$inner<B> as FromParserBase<B>>::Error;
            fn parse(b: &mut B) -> Result<Self, Self::Error> {
                <$inner<B>>::parse(b).map(Self)
            }
        }

        impl<B: StableRef> core::fmt::Debug for $name<B> {
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
property!(AssignedClientIdentifier<B>, MqttString);
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// #[cfg_attr(feature = "defmt", derive(defmt::Format))]
// pub struct ServerKeepAlive(pub(crate) KeepAlive);
property!(AuthenticationMethod<B>, MqttString);
property!(AuthenticationData<B>, MqttBinary);
property!(RequestProblemInformation, bool);
property!(WillDelayInterval, u32);
property!(RequestResponseInformation, bool);
property!(ResponseInformation<B>, MqttString);
property!(ServerReference<B>, MqttString);
property!(ReasonString<B>, MqttString);
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// #[cfg_attr(feature = "defmt", derive(defmt::Format))]
// pub struct ReceiveMaximum(pub(crate) NonZero<u16>);
property!(TopicAliasMaximum, u16);
property!(TopicAlias, NonZero<u16>);
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// #[cfg_attr(feature = "defmt", derive(defmt::Format))]
// pub struct MaximumQoS(pub(crate) QoS);
property!(RetainAvailable, bool);
property!(UserProperty<B>, MqttStringPair);
