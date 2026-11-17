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
        packet::{
            Auth, Connack, Connect, Disconnect, Packet, Puback, Pubcomp, Publish, Pubrec, Pubrel,
            Suback, SubackReasonCode, Subscribe, Unsuback, UnsubackReasonCode, Unsubscribe,
        },
        role::Role,
    },
};

mod client;
mod server;

pub mod packet;
pub mod role;

pub use client::Client;
pub use server::Server;

#[derive(Clone, Copy)]
pub struct Parser<PEER> {
    _phantom_data: PhantomData<(PEER)>,
}

impl<PEER: Role> Default for Parser<PEER> {
    fn default() -> Self {
        Self::new()
    }
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

    pub fn parse<B, UPS, SIS, SRCS, URCS>(
        self,
        data: B,
    ) -> Result<Packet<PEER, B, UPS, SIS, SRCS, URCS>>
    where
        B: ParserBase,
        UPS: Storage<Element = UserProperty<B>>,
        SIS: Storage<Element = PEER::PublishSubscriptionIdentifier<B>>,
        SRCS: Storage<Element = SubackReasonCode>,
        URCS: Storage<Element = UnsubackReasonCode>,
    {
        PEER::parse(data)
    }

    pub(crate) fn parse_packet_header<B: ParserBase>(
        data: B,
    ) -> Result<(Header<PEER::PacketType>, B)> {
        if data.len() < 2 {
            return Ok(Status::Partial);
        }

        let packet_type = PEER::packet_type(data.as_ref()[0])?;
        let flags = data.as_ref()[0] & 0x0F;
        let (remaining_len, remaining_len_bytes) =
            match VarByteInt::parse_remaining_length(&data.as_ref()[1..])? {
                Status::Complete(value) => value,
                Status::Partial => return Ok(Status::Partial),
            };

        let mut data = data;
        data.advance(1 + remaining_len_bytes);

        Ok(Status::Complete((
            Header {
                packet_type,
                flags,
                remaining_len,
            },
            data,
        )))
    }

    pub fn parse_publish<
        B: ParserBase,
        SIS: Storage<Element = PEER::PublishSubscriptionIdentifier<B>>,
        UPS: Storage<Element = UserProperty<B>>,
    >(
        self,
        header: Header<PEER::PacketType>,
        mut data: B,
    ) -> Result<Publish<B, SIS, UPS>> {
        PEER::parse_publish(header, data)
    }

    pub fn parse_connect<B: ParserBase>(
        self,
        header: Header<PEER::PacketType>,
        data: B,
    ) -> Result<PEER::Connect> {
        PEER::parse_connect(header, data)
    }

    pub fn parse_connack<B: ParserBase, UPS: Storage<Element = UserProperty<B>>>(
        self,
        header: Header<PEER::PacketType>,
        data: B,
    ) -> Result<PEER::Connack<B, UPS>> {
        PEER::parse_connack(header, data)
    }

    pub fn parse_puback<B: ParserBase, UPS: Storage<Element = UserProperty<B>>>(
        self,
        header: Header<PEER::PacketType>,
        data: B,
    ) -> Result<Puback<B, PEER::PubackReasonCode<B>, UPS>> {
        PEER::parse_puback(header, data)
    }

    pub fn parse_pubrec<B: ParserBase, UPS: Storage<Element = UserProperty<B>>>(
        self,
        header: Header<PEER::PacketType>,
        data: B,
    ) -> Result<Pubrec<B, PEER::PubrecReasonCode<B>, UPS>> {
        PEER::parse_pubrec(header, data)
    }

    pub fn parse_pubrel<B: ParserBase, UPS: Storage<Element = UserProperty<B>>>(
        self,
        header: Header<PEER::PacketType>,
        data: B,
    ) -> Result<Pubrel<B, UPS>> {
        PEER::parse_pubrel(header, data)
    }

    pub fn parse_pubcomp<B: ParserBase, UPS: Storage<Element = UserProperty<B>>>(
        self,
        header: Header<PEER::PacketType>,
        data: B,
    ) -> Result<Pubcomp<B, UPS>> {
        PEER::parse_pubcomp(header, data)
    }

    pub fn parse_subscribe<B: ParserBase>(
        self,
        header: Header<PEER::PacketType>,
        data: B,
    ) -> Result<PEER::Subscribe> {
        PEER::parse_subscribe(header, data)
    }

    pub fn parse_suback<
        B: ParserBase,
        UPS: Storage<Element = UserProperty<B>>,
        RCS: Storage<Element = SubackReasonCode>,
    >(
        self,
        header: Header<PEER::PacketType>,
        data: B,
    ) -> Result<PEER::Suback<B, UPS, RCS>> {
        PEER::parse_suback(header, data)
    }

    pub fn parse_unsubscribe<B: ParserBase>(
        self,
        header: Header<PEER::PacketType>,
        data: B,
    ) -> Result<PEER::Unsubscribe> {
        PEER::parse_unsubscribe(header, data)
    }

    pub fn parse_unsuback<
        B: ParserBase,
        UPS: Storage<Element = UserProperty<B>>,
        RCS: Storage<Element = UnsubackReasonCode>,
    >(
        self,
        header: Header<PEER::PacketType>,
        data: B,
    ) -> Result<PEER::Unsuback<B, UPS, RCS>> {
        PEER::parse_unsuback(header, data)
    }

    pub fn parse_pingreq<B: ParserBase>(
        self,
        header: Header<PEER::PacketType>,
        data: B,
    ) -> Result<PEER::Pingreq> {
        PEER::parse_pingreq(header, data)
    }

    pub fn parse_pingresp<B: ParserBase>(
        self,
        header: Header<PEER::PacketType>,
        data: B,
    ) -> Result<PEER::Pingresp> {
        PEER::parse_pingresp(header, data)
    }

    pub fn parse_disconnect<B: ParserBase, UPS: Storage<Element = UserProperty<B>>>(
        self,
        header: Header<PEER::PacketType>,
        data: B,
    ) -> Result<
        Disconnect<
            B,
            PEER::DisconnectReasonCode<B>,
            PEER::DisconnectSessionExpiryInterval<B>,
            PEER::DisconnectServerReference<B>,
            UPS,
        >,
    > {
        PEER::parse_disconnect(header, data)
    }

    pub fn parse_auth<B: ParserBase>(
        self,
        header: Header<PEER::PacketType>,
        data: B,
    ) -> Result<Auth<PEER::AuthReasonCode<B>>> {
        PEER::parse_auth(header, data)
    }
}
