use core::assert_matches;

use super::Parser;
use crate::{
    Error, Header, MapExt, Result, Status,
    base::{FromParserBase, ParserBase, Storage, StringError, TooShort, VarByteIntError},
    types::{
        VarByteInt,
        v5::{
            ReasonCode,
            property::{
                AssignedClientIdentifier, AuthenticationData, AuthenticationMethod,
                MaximumPacketSize, MaximumQoS, NeverProperty, NeverPropertyError, OnceHelper,
                PropertyError, PropertyType, ReasonString, ReceiveMaximum, ResponseInformation,
                RetainAvailable, ServerKeepAlive, ServerReference, SessionExpiryInterval,
                SharedSubscriptionAvailable, SubscriptionIdentifier,
                SubscriptionIdentifierAvailable, TopicAliasMaximum, UserProperty,
                WildcardSubscriptionAvailable,
            },
        },
    },
    v5::{
        packet::{
            Connack, Packet, Pingresp, ServerAuthReasonCode, ServerDisconnectReasonCode,
            ServerPubackReasonCode, ServerPubrecReasonCode, Suback, SubackReasonCode, Unsuback,
            UnsubackReasonCode,
        },
        role::Role,
    },
};

#[derive(Debug, Clone, Copy)]
pub struct Server;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum ServerPacketType {
    Connack = 0x20,
    Publish = 0x30,
    Puback = 0x40,
    Pubrec = 0x50,
    Pubrel = 0x60,
    Pubcomp = 0x70,
    Suback = 0x90,
    Unsuback = 0xB0,
    Pingresp = 0xD0,
    Disconnect = 0xE0,
    Auth = 0xF0,
}

impl From<ServerPacketType> for u8 {
    fn from(value: ServerPacketType) -> Self {
        value as u8
    }
}

impl Role for Server {
    type PacketType = ServerPacketType;

    type Connect = !;
    type Connack<B, UPS> = Connack<B, UPS>;
    type Subscribe = !;
    type Suback<B, UPS, RCS> = Suback<B, UPS, RCS>;
    type Unsubscribe = !;
    type Unsuback<B, UPS, RCS> = Unsuback<B, UPS, RCS>;
    type Pingreq = !;
    type Pingresp = Pingresp;

    type PublishSubscriptionIdentifierError = PropertyError<VarByteIntError>;
    type PublishSubscriptionIdentifier<B: ParserBase> = SubscriptionIdentifier;

    type PubackReasonCode<B: ParserBase> = ServerPubackReasonCode;
    type PubrecReasonCode<B: ParserBase> = ServerPubrecReasonCode;

    type DisconnectReasonCode<B: ParserBase> = ServerDisconnectReasonCode;
    type DisconnectSessionExpiryIntervalError = NeverPropertyError;
    type DisconnectSessionExpiryInterval<B: ParserBase> = NeverProperty;
    type DisconnectServerReferenceError = PropertyError<StringError>;
    type DisconnectServerReference<B: ParserBase> = ServerReference<B>;

    type AuthReasonCode<B: ParserBase> = ServerAuthReasonCode;

    fn packet_type(first_byte: u8) -> core::result::Result<Self::PacketType, Error> {
        #[rustfmt::skip]
        let r = match first_byte >> 4 {
            0  => Err(Error::ReservedPacketType),
            1  => Err(Error::UnexpectedPacketType), // Connect
            2  => Ok(ServerPacketType::Connack),
            3  => Ok(ServerPacketType::Publish),
            4  => Ok(ServerPacketType::Puback),
            5  => Ok(ServerPacketType::Pubrec),
            6  => Ok(ServerPacketType::Pubrel),
            7  => Ok(ServerPacketType::Pubcomp),
            8  => Err(Error::UnexpectedPacketType), // Subscribe
            9  => Ok(ServerPacketType::Suback),
            10 => Err(Error::UnexpectedPacketType), // Unsubscribe
            11 => Ok(ServerPacketType::Unsuback),
            12 => Err(Error::UnexpectedPacketType), // Pingreq
            13 => Ok(ServerPacketType::Pingresp),
            14 => Ok(ServerPacketType::Disconnect),
            15 => Ok(ServerPacketType::Auth),

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
            ServerPacketType::Connack => Self::parse_connack(header, data).map_status(Packet::Connack),
            ServerPacketType::Publish => Self::parse_publish(header, data).map_status(Packet::Publish),
            ServerPacketType::Puback => Self::parse_puback(header, data).map_status(Packet::Puback),
            ServerPacketType::Pubrec => Self::parse_pubrec(header, data).map_status(Packet::Pubrec),
            ServerPacketType::Pubrel => Self::parse_pubrel(header, data).map_status(Packet::Pubrel),
            ServerPacketType::Pubcomp => Self::parse_pubcomp(header, data).map_status(Packet::Pubcomp),
            ServerPacketType::Suback => Self::parse_suback(header, data).map_status(Packet::Suback),
            ServerPacketType::Unsuback => Self::parse_unsuback(header, data).map_status(Packet::Unsuback),
            ServerPacketType::Pingresp => Self::parse_pingresp(header, data).map_status(Packet::Pingresp),
            ServerPacketType::Disconnect => Self::parse_disconnect(header, data).map_status(Packet::Disconnect),
            ServerPacketType::Auth => Self::parse_auth(header, data).map_status(Packet::Auth),
        };

        r
    }

    fn parse_connack<B: ParserBase, UPS: Storage<Element = UserProperty<B>>>(
        header: Header<Self::PacketType>,
        mut data: B,
    ) -> Result<Self::Connack<B, UPS>> {
        assert_matches!(header.packet_type, ServerPacketType::Connack);

        let len: usize = header
            .remaining_len
            .try_into()
            .map_err(|_| Error::PacketTooLarge)?;

        if header.flags != 0 {
            return Err(Error::InvalidFixedHeaderFlags);
        }

        let mut backing = match data.split_to_checked(len) {
            Some(d) => d,
            None => return Ok(Status::Partial),
        };

        let data = &mut backing;

        let connack_flags = u8::parse(data)?;

        // first 7 bits have to be set to 0
        let session_present = match connack_flags {
            0 => false,
            1 => true,
            2.. => return Err(todo!("Some error for invalid flag field")),
        };

        let connect_reason_code = ReasonCode::parse(data)?;
        if !matches!(
            connect_reason_code,
            ReasonCode::Success
                | ReasonCode::UnspecifiedError
                | ReasonCode::MalformedPacket
                | ReasonCode::ProtocolError
                | ReasonCode::ImplementationSpecificError
                | ReasonCode::UnsupportedProtocolVersion
                | ReasonCode::ClientIdentifierNotValid
                | ReasonCode::BadUserNameOrPassword
                | ReasonCode::NotAuthorized
                | ReasonCode::ServerUnavailable
                | ReasonCode::ServerBusy
                | ReasonCode::Banned
                | ReasonCode::BadAuthenticationMethod
                | ReasonCode::TopicNameInvalid
                | ReasonCode::PacketTooLarge
                | ReasonCode::QuotaExceeded
                | ReasonCode::PayloadFormatInvalid
                | ReasonCode::RetainNotSupported
                | ReasonCode::QoSNotSupported
                | ReasonCode::UseAnotherServer
                | ReasonCode::ServerMoved
                | ReasonCode::ConnectionRateExceeded
        ) {
            return Err(Error::IllegalReasonCode);
        }

        let properties_length: usize = VarByteInt::parse(data)?
            .try_into()
            .map_err(|_| Error::InvalidPropertyLength)?;

        if properties_length != data.len() {
            return Err(todo!("illegal payload"));
        }

        let properties_data = data;

        let mut session_expiry_interval: Option<SessionExpiryInterval> = None;
        let mut receive_maximum: Option<ReceiveMaximum> = None;
        let mut maximum_qos: Option<MaximumQoS> = None;
        let mut retain_available: Option<RetainAvailable> = None;
        let mut maximum_packet_size: Option<MaximumPacketSize> = None;
        let mut assigned_client_identifier: Option<AssignedClientIdentifier<B>> = None;
        let mut topic_alias_maximum: Option<TopicAliasMaximum> = None;
        let mut reason_string: Option<ReasonString<B>> = None;
        let mut user_properties = UPS::default();
        let mut wildcard_subscription_available: Option<WildcardSubscriptionAvailable> = None;
        let mut subscription_identifier_available: Option<SubscriptionIdentifierAvailable> = None;
        let mut shared_subscription_available: Option<SharedSubscriptionAvailable> = None;
        let mut server_keep_alive: Option<ServerKeepAlive> = None;
        let mut response_information: Option<ResponseInformation<B>> = None;
        let mut server_reference: Option<ServerReference<B>> = None;
        let mut authentication_method: Option<AuthenticationMethod<B>> = None;
        let mut authentication_data: Option<AuthenticationData<B>> = None;

        // TODO convert RemainingLengthExceeded to PropertyLengthExceeded here
        while properties_data.len() > 0 {
            let property_type = PropertyType::parse(properties_data)?;

            match property_type {
                PropertyType::SessionExpiryInterval => {
                    session_expiry_interval.set(properties_data)?
                }
                PropertyType::ReceiveMaximum => receive_maximum.set(properties_data)?,
                PropertyType::MaximumQoS => maximum_qos.set(properties_data)?,
                PropertyType::RetainAvailable => retain_available.set(properties_data)?,
                PropertyType::MaximumPacketSize => maximum_packet_size.set(properties_data)?,
                PropertyType::AssignedClientIdentifier => {
                    assigned_client_identifier.set(properties_data)?
                }
                PropertyType::TopicAliasMaximum => topic_alias_maximum.set(properties_data)?,
                PropertyType::ReasonString => reason_string.set(properties_data)?,
                PropertyType::UserProperty => {
                    let user_property = UserProperty::parse(properties_data)?;
                    if let Some(limit) = UPS::LIMIT
                        && user_properties.len() == limit
                    {
                        // Storage is full
                    } else {
                        user_properties.extend(Some(user_property));
                    }
                }
                PropertyType::WildcardSubscriptionAvailable => {
                    wildcard_subscription_available.set(properties_data)?
                }
                PropertyType::SubscriptionIdentifierAvailable => {
                    subscription_identifier_available.set(properties_data)?
                }
                PropertyType::SharedSubscriptionAvailable => {
                    shared_subscription_available.set(properties_data)?
                }
                PropertyType::ServerKeepAlive => server_keep_alive.set(properties_data)?,
                PropertyType::ResponseInformation => response_information.set(properties_data)?,
                PropertyType::ServerReference => server_reference.set(properties_data)?,
                PropertyType::AuthenticationMethod => authentication_method.set(properties_data)?,
                PropertyType::AuthenticationData => authentication_data.set(properties_data)?,
                p => return Err(Error::InvalidPropertyType),
            }
        }

        Ok(Status::Complete(Connack {
            session_present,
            reason_code: todo!(),
            session_expiry_interval,
            receive_maximum,
            maximum_qos,
            retain_available,
            maximum_packet_size,
            assigned_client_identifier,
            topic_alias_maximum,
            reason_string,
            user_properties,
            wildcard_subscription_available,
            subscription_identifier_available,
            shared_subscription_available,
            server_keep_alive,
            response_information,
            server_reference,
            authentication_method,
            authentication_data,
        }))
    }

    fn parse_suback<
        B: ParserBase,
        UPS: Storage<Element = UserProperty<B>>,
        RCS: Storage<Element = SubackReasonCode>,
    >(
        header: Header<Self::PacketType>,
        data: B,
    ) -> Result<Self::Suback<B, UPS, RCS>> {
        todo!()
    }

    fn parse_unsuback<
        B: ParserBase,
        UPS: Storage<Element = UserProperty<B>>,
        RCS: Storage<Element = UnsubackReasonCode>,
    >(
        header: Header<Self::PacketType>,
        data: B,
    ) -> Result<Self::Unsuback<B, UPS, RCS>> {
        todo!()
    }

    fn parse_pingresp<B: ParserBase>(
        header: Header<Self::PacketType>,
        data: B,
    ) -> Result<Self::Pingresp> {
        todo!()
    }

    fn parse_connect<B: ParserBase>(
        header: Header<Self::PacketType>,
        data: B,
    ) -> Result<Self::Connect> {
        Err(Error::UnexpectedPacketType)
    }
    fn parse_subscribe<B: ParserBase>(
        header: Header<Self::PacketType>,
        data: B,
    ) -> Result<Self::Subscribe> {
        Err(Error::UnexpectedPacketType)
    }
    fn parse_unsubscribe<B: ParserBase>(
        header: Header<Self::PacketType>,
        data: B,
    ) -> Result<Self::Unsubscribe> {
        Err(Error::UnexpectedPacketType)
    }
    fn parse_pingreq<B: ParserBase>(
        header: Header<Self::PacketType>,
        data: B,
    ) -> Result<Self::Pingreq> {
        Err(Error::UnexpectedPacketType)
    }
}
