use crate::{
    Error, Result,
    base::{FromParserBase, ParserBase},
    v5::packet::{Connack, Connect, Publish},
};

pub trait Role {
    // const IS_SERVER: bool;
    // const IS_CLIENT: bool = !Self::IS_SERVER;

    type PacketType;
    type PublishSubscriptionIdentifierError: Into<Error>;
    type PublishSubscriptionIdentifier<B: ParserBase>: FromParserBase<B, Error = Self::PublishSubscriptionIdentifierError>;

    // fn parse_packet<T, S: Storage<T>>(data: &[u8]) -> Result<Packet<Self, T, S>>;
    // fn parse_connect<T, S: Storage<T>>(data: &[u8]) -> Result<Self::PeerConnect<S>>;
    // fn parse_connack<T, S: Storage<T>>(data: &[u8]) -> Result<Self::PeerConnack<S>>;
    // fn parse_publish(data: &[u8]) -> Result<Self::PeerPublish<S>>;
    // fn parse_puback(data: &[u8]) -> Result<Self::PeerPuback<S>>;
    // fn parse_pubrec(data: &[u8]) -> Result<Self::PeerPubrec<S>>;
    // fn parse_pubrel(data: &[u8]) -> Result<Self::PeerPubrel<S>>;
    // fn parse_pubcomp(data: &[u8]) -> Result<Self::PeerPubcomp<S>>;
    // fn parse_subscribe(data: &[u8]) -> Result<Self::PeerSubscribe<S>>;
    // fn parse_suback(data: &[u8]) -> Result<Self::PeerSuback<S>>;
    // fn parse_unsubscribe(data: &[u8]) -> Result<Self::PeerUnsubscribe<S>>;
    // fn parse_unsuback(data: &[u8]) -> Result<Self::PeerUnsuback<S>>;
    // fn parse_pingreq(data: &[u8]) -> Result<Self::PeerPingreq<S>>;
    // fn parse_pingresp(data: &[u8]) -> Result<Self::PeerPingresp<S>>;
    // fn parse_disconnect(data: &[u8]) -> Result<Self::PeerDisconnect<S>>;
    // fn parse_auth(data: &[u8]) -> Result<Self::PeerAuth<S>>;

    fn packet_type(first_byte: u8) -> core::result::Result<Self::PacketType, Error>;
}
