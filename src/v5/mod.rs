use core::{convert::Infallible, marker::PhantomData};

use crate::{
    Error, Header, Result, Status,
    base::{FromParserBase, ParserBase, Storage},
    types::{
        IdentifiedQoS, MqttString, PacketIdentifier, QoS, TopicName, TopicNameError, VarByteInt,
        v5::{
            TopicReference,
            property::{
                ContentType, CorrelationData, MessageExpiryInterval, OnceHelper,
                PayloadFormatIndicator, PropertyType, ResponseTopic, SubscriptionIdentifier,
                TopicAlias, UserProperty,
            },
        },
    },
    v5::{
        packet::{Connect, Publish},
        roles::Role,
    },
};

mod client;
mod server;

pub mod packet;
pub mod roles;

pub use client::{Client, NeverSubscriptionIdentifier, NeverSubscriptionIdentifierError};
pub use server::Server;

pub enum ServerPacketType {}

#[derive(Clone, Copy)]
pub struct Parser<PEER> {
    _phantom_data: PhantomData<(PEER)>,
}

impl<PEER: Role> Parser<PEER> {
    pub const fn new() -> Self {
        Self {
            _phantom_data: PhantomData,
        }
    }

    pub fn parse_header(self, data: &[u8]) -> Result<Header<PEER::PacketType>> {
        if data.len() < 2 {
            return Ok(Status::Partial);
        }

        let packet_type = PEER::packet_type(data[0])?;
        let flags = data[0] & 0x0F;

        let remaining_len = match VarByteInt::parse_remaining_length(&data[1..])? {
            Status::Complete((r, _)) => r,
            Status::Partial => return Ok(Status::Partial),
        };

        Ok(Status::Complete(Header {
            packet_type,
            flags,
            remaining_len,
        }))
    }

    pub fn parse_publish<
        B: ParserBase,
        SubscriptionIdentifierStorage: Storage<Element = PEER::PublishSubscriptionIdentifier<B>>,
        UserPropertyStorage: Storage<Element = UserProperty<B>>,
    >(
        self,
        header: Header<PEER::PacketType>,
        mut data: B,
    ) -> Result<
        Publish<
            B,
            PEER::PublishSubscriptionIdentifier<B>,
            SubscriptionIdentifierStorage,
            UserPropertyStorage,
        >,
    > {
        let len: usize = header
            .remaining_len
            .try_into()
            .map_err(|_| Error::PacketTooLarge)?;

        // qos, retain, dup
        let dup = header.flags >> 3 == 1;

        let qos = match header.flags & 0x06 {
            0x00 => QoS::AtMostOnce,
            0x02 => QoS::AtLeastOnce,
            0x04 => QoS::ExactlyOnce,
            _ => return Err(Error::InvalidFixedHeaderFlags),
        };
        let retain = header.flags & 0x01 == 1;

        let mut backing = match data.split_to_checked(len) {
            Some(d) => d,
            None => return Ok(Status::Partial),
        };
        let data = &mut backing;

        let topic_name = MqttString::parse(data)?;
        let topic_name = if topic_name.is_empty() {
            None
        } else {
            Some(TopicName::new(topic_name).map_err(|e| match e {
                TopicNameError::Empty => unreachable!(),
                TopicNameError::Wildcard => Error::InvalidTopicName,
            })?)
        };

        let identified_qos = match qos {
            QoS::AtMostOnce => IdentifiedQoS::AtMostOnce,
            QoS::AtLeastOnce => IdentifiedQoS::AtLeastOnce(PacketIdentifier::parse(data)?),
            QoS::ExactlyOnce => IdentifiedQoS::ExactlyOnce(PacketIdentifier::parse(data)?),
        };

        let properties_length: usize = VarByteInt::parse(data)?
            .try_into()
            .map_err(|_| Error::InvalidPropertyLength)?;
        let mut properties_data = data
            .split_to_checked(properties_length)
            .ok_or(Error::InvalidPropertyLength)?;
        let properties_data = &mut properties_data;

        let mut payload_format_indicator: Option<PayloadFormatIndicator> = None;
        let mut message_expiry_interval: Option<MessageExpiryInterval> = None;
        let mut topic_alias: Option<TopicAlias> = None;
        let mut response_topic: Option<ResponseTopic<B>> = None;
        let mut correlation_data: Option<CorrelationData<B>> = None;
        let mut user_properties = UserPropertyStorage::default();
        let mut subscription_identifiers = SubscriptionIdentifierStorage::default();
        let mut content_type: Option<ContentType<B>> = None;

        // TODO convert RemainingLengthExceeded to PropertyLengthExceeded here
        while properties_data.len() > 0 {
            let property_type = PropertyType::parse(properties_data)?;

            match property_type {
                PropertyType::PayloadFormatIndicator => {
                    payload_format_indicator.set(properties_data)?;
                }
                PropertyType::MessageExpiryInterval => {
                    message_expiry_interval.set(properties_data)?;
                }
                PropertyType::TopicAlias => {
                    topic_alias.set(properties_data)?;
                }
                PropertyType::ResponseTopic => {
                    response_topic.set(properties_data)?;
                }

                PropertyType::CorrelationData => {
                    correlation_data.set(properties_data)?;
                }
                PropertyType::UserProperty => {
                    let user_property = UserProperty::parse(properties_data)?;
                    if let Some(limit) = UserPropertyStorage::LIMIT
                        && user_properties.len() == limit
                    {
                        // Storage is full
                    } else {
                        user_properties.extend(Some(user_property));
                    }
                }
                PropertyType::SubscriptionIdentifier => {
                    let subscription_identifier =
                        PEER::PublishSubscriptionIdentifier::parse(properties_data)
                            .map_err(Into::into)?;
                    if let Some(limit) = SubscriptionIdentifierStorage::LIMIT
                        && user_properties.len() == limit
                    {
                        // Storage is full
                    } else {
                        subscription_identifiers.extend(Some(subscription_identifier));
                    }
                }

                PropertyType::ContentType => {
                    content_type.set(properties_data)?;
                }
                p => {
                    // Malformed packet according to <https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901029>
                    return Err(Error::InvalidPropertyType);
                }
            }
        }

        let topic = match (topic_name, topic_alias) {
            (None, None) => return Err(Error::InvalidTopicName),
            (None, Some(alias)) => TopicReference::Alias(alias.into()),
            (Some(name), None) => TopicReference::Name(name),
            (Some(name), Some(alias)) => TopicReference::Mapping(name, alias.into()),
        };

        let message = backing;

        Ok(Status::Complete(Publish {
            dup,
            identified_qos,
            retain,
            topic,
            payload_format_indicator,
            message_expiry_interval,
            response_topic,
            correlation_data,
            user_properties,
            subscription_identifiers,
            content_type,
            message,
        }))
    }

    // pub fn parse<B: ParserBase>(self, data: B) -> Result<Packet<PEER>> {
    //     let bytes = data.as_ref();

    //     if bytes.len() < 2 {
    //         return Ok(Status::Partial);
    //     }

    //     let packet_type = PEER::packet_type(bytes[0])?;
    //     let flags = bytes[0] & 0x0F;

    //     let (remaining_len, header_len) = match parse_varbyteint(&bytes[1..])? {
    //         Status::Complete(i) => i,
    //         Status::Partial => return Ok(Status::Partial),
    //     };

    //     let remaining_len = usize::try_from(remaining_len).map_err(|_| Error::PacketTooLarge)?;

    //     let post_header = &bytes[header_len..];

    //     let Some(remaining) = post_header.get(remaining_len) else {
    //         return Ok(Status::Partial);
    //     };

    //     todo!()
    // }

    // pub fn parse_connect<B: ParserBase>(
    //     self,
    //     header: Header<PEER::PacketType>,
    //     data: B,
    // ) -> Result<PEER::ConnectPacket> {
    //     todo!()
    // }
}
