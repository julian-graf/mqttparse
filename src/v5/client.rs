use core::marker::PhantomData;

use super::Parser;
use crate::{
    Error, Header, MapExt, Result,
    Status::{self, Complete},
    base::{ParserBase, PropertyTypeError, Storage, TooShort},
    types::v5::property::{
        NeverProperty, NeverPropertyError, PropertyError, SessionExpiryInterval, UserProperty,
    },
    v5::{
        packet::{
            Auth, ClientAuthReasonCode, ClientDisconnectReasonCode, ClientPubackReasonCode,
            ClientPubrecReasonCode, Connack, Connect, Disconnect, Packet, Pingreq, Pingresp,
            Puback, Pubcomp, Publish, Pubrec, Pubrel, Suback, SubackReasonCode, Subscribe,
            Unsuback, UnsubackReasonCode, Unsubscribe,
        },
        role::Role,
    },
};

#[derive(Debug, Clone, Copy)]
pub struct Client;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum ClientPacketType {
    Connect = 0x10,
    Publish = 0x30,
    Puback = 0x40,
    Pubrec = 0x50,
    Pubrel = 0x60,
    Pubcomp = 0x70,
    Subscribe = 0x80,
    Unsubscribe = 0xA0,
    Pingreq = 0xC0,
    Disconnect = 0xE0,
    Auth = 0xF0,
}

impl From<ClientPacketType> for u8 {
    fn from(value: ClientPacketType) -> Self {
        value as u8
    }
}

impl Role for Client {
    type PacketType = ClientPacketType;

    type Connect = Connect;
    type Connack<B, UPS> = !;
    type Subscribe = Subscribe;
    type Suback<B, UPS, RCS> = !;
    type Unsubscribe = Subscribe;
    type Unsuback<B, UPS, RCS> = !;
    type Pingreq = Pingreq;
    type Pingresp = !;

    type PublishSubscriptionIdentifierError = NeverPropertyError;
    type PublishSubscriptionIdentifier<B: ParserBase> = NeverProperty;

    type PubackReasonCode<B: ParserBase> = ClientPubackReasonCode;

    type PubrecReasonCode<B: ParserBase> = ClientPubrecReasonCode;

    type DisconnectReasonCode<B: ParserBase> = ClientDisconnectReasonCode;
    type DisconnectSessionExpiryIntervalError = PropertyError<TooShort>;
    type DisconnectSessionExpiryInterval<B: ParserBase> = SessionExpiryInterval;
    type DisconnectServerReferenceError = NeverPropertyError;
    type DisconnectServerReference<B: ParserBase> = NeverProperty;

    type AuthReasonCode<B: ParserBase> = ClientAuthReasonCode;

    fn packet_type(first_byte: u8) -> core::result::Result<Self::PacketType, Error> {
        #[rustfmt::skip]
        let r = match first_byte >> 4 {
            0  => Err(Error::ReservedPacketType),
            1  => Ok(ClientPacketType::Connect),
            2  => Err(Error::UnexpectedPacketType), // Connack
            3  => Ok(ClientPacketType::Publish),
            4  => Ok(ClientPacketType::Puback),
            5  => Ok(ClientPacketType::Pubrec),
            6  => Ok(ClientPacketType::Pubrel),
            7  => Ok(ClientPacketType::Pubcomp),
            8  => Ok(ClientPacketType::Subscribe),
            9  => Err(Error::UnexpectedPacketType), // Suback
            10 => Ok(ClientPacketType::Unsubscribe),
            11 => Err(Error::UnexpectedPacketType), // Unsuback
            12 => Ok(ClientPacketType::Pingreq),
            13 => Err(Error::UnexpectedPacketType), // Pingresp
            14 => Ok(ClientPacketType::Disconnect),
            15 => Ok(ClientPacketType::Auth),

            16.. => unreachable!("u8 shifted 4 bits: all values 0x0..=0xF are covered"),
        };

        r
    }

    fn parse<B, UPS, SIS, SRCS, URCS>(data: B) -> Result<Packet<Self, B, UPS, SIS, SRCS, URCS>>
    where
        B: ParserBase,
        UPS: Storage<Element = UserProperty<B>>,
        SIS: Storage<Element = Self::PublishSubscriptionIdentifier<B>>,
        SRCS: Storage<Element = SubackReasonCode>,
        URCS: Storage<Element = UnsubackReasonCode>,
    {
        let (header, data) = match Parser::<Self>::parse_packet_header(data)? {
            Status::Complete(value) => value,
            Status::Partial => return Ok(Status::Partial),
        };

        #[rustfmt::skip]
        let r = match header.packet_type {
            ClientPacketType::Connect => Self::parse_connect(header, data).map_status(Packet::Connect),
            ClientPacketType::Publish => Self::parse_publish(header, data).map_status(Packet::Publish),
            ClientPacketType::Puback => Self::parse_puback(header, data).map_status(Packet::Puback),
            ClientPacketType::Pubrec => Self::parse_pubrec(header, data).map_status(Packet::Pubrec),
            ClientPacketType::Pubrel => Self::parse_pubrel(header, data).map_status(Packet::Pubrel),
            ClientPacketType::Pubcomp => Self::parse_pubcomp(header, data).map_status(Packet::Pubcomp),
            ClientPacketType::Subscribe => Self::parse_subscribe(header, data).map_status(Packet::Subscribe),
            ClientPacketType::Unsubscribe => Self::parse_unsubscribe(header, data).map_status(Packet::Unsubscribe),
            ClientPacketType::Pingreq => Self::parse_pingreq(header, data).map_status(Packet::Pingreq),
            ClientPacketType::Disconnect => Self::parse_disconnect(header, data).map_status(Packet::Disconnect),
            ClientPacketType::Auth => Self::parse_auth(header, data).map_status(Packet::Auth),
        };

        r
    }

    fn parse_connect<B: ParserBase>(
        header: Header<Self::PacketType>,
        data: B,
    ) -> Result<Self::Connect> {
        todo!()
    }
    fn parse_subscribe<B: ParserBase>(
        header: Header<Self::PacketType>,
        data: B,
    ) -> Result<Self::Subscribe> {
        todo!()
    }
    fn parse_unsubscribe<B: ParserBase>(
        header: Header<Self::PacketType>,
        data: B,
    ) -> Result<Self::Unsubscribe> {
        todo!()
    }
    fn parse_pingreq<B: ParserBase>(
        header: Header<Self::PacketType>,
        data: B,
    ) -> Result<Self::Pingreq> {
        todo!()
    }

    fn parse_connack<B: ParserBase, UPS>(
        header: Header<Self::PacketType>,
        data: B,
    ) -> Result<Self::Connack<B, UPS>> {
        Err(Error::UnexpectedPacketType)
    }
    fn parse_suback<
        B: ParserBase,
        UPS: Storage<Element = UserProperty<B>>,
        RCS: Storage<Element = SubackReasonCode>,
    >(
        header: Header<Self::PacketType>,
        data: B,
    ) -> Result<Self::Suback<B, UPS, RCS>> {
        Err(Error::UnexpectedPacketType)
    }
    fn parse_unsuback<
        B: ParserBase,
        UPS: Storage<Element = UserProperty<B>>,
        RCS: Storage<Element = UnsubackReasonCode>,
    >(
        header: Header<Self::PacketType>,
        data: B,
    ) -> Result<Self::Unsuback<B, UPS, RCS>> {
        Err(Error::UnexpectedPacketType)
    }
    fn parse_pingresp<B: ParserBase>(
        header: Header<Self::PacketType>,
        data: B,
    ) -> Result<Self::Pingresp> {
        Err(Error::UnexpectedPacketType)
    }
}
