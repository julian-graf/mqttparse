use core::marker::PhantomData;

use crate::{
    Error, Result, Status::Complete, base::ParserBase, v5::{
        packet::{
            Auth, Connack, Connect, Disconnect, Pingreq, Pingresp, Puback, Pubcomp, Publish,
            Pubrec, Pubrel, Suback, Subscribe, Unsuback, Unsubscribe,
        },
        roles::Role,
    },
};

#[derive(Debug, Clone, Copy)]
pub struct Client;
#[derive(Debug, Clone, Copy)]
pub struct NeverSubscriptionIdentifier(!);
#[derive(Debug, Clone, Copy)]
pub struct NeverSubscriptionIdentifierError;

#[derive(Debug, Clone, Copy)]
pub enum ClientPacketType {
    Connect,
    Publish,
    Puback,
    Pubrec,
    Pubrel,
    Pubcomp,
    Subscribe,
    Unsubscribe,
    Pingreq,
    Disconnect,
    Auth,
}

impl Role for Client {
    type PacketType = ClientPacketType;
    type PublishSubscriptionIdentifierError = NeverSubscriptionIdentifierError;
    type PublishSubscriptionIdentifier<B: ParserBase> = NeverSubscriptionIdentifier;

    // type PeerConnect<_S> = !;
    // type PeerConnack<U> = Connack<U>;
    // type PeerPublish<_S> = Publish;
    // type PeerPuback<_S> = Puback;
    // type PeerPubrec<_S> = Pubrec;
    // type PeerPubrel<_S> = Pubrel;
    // type PeerPubcomp<_S> = Pubcomp;
    // type PeerSubscribe<_S> = !;
    // type PeerSuback<_S> = Suback;
    // type PeerUnsubscribe<_S> = !;
    // type PeerUnsuback<_S> = Unsuback;
    // type PeerPingreq<_S> = !;
    // type PeerPingresp<_S> = Pingresp;
    // type PeerDisconnect<_S> = Disconnect;
    // type PeerAuth<_S> = Auth;

    // fn parse_packet(data: &[u8]) -> Result<Packet<Self, T, S>> {
    //     todo!()
    // }

    // fn parse_connect(data: &[u8]) -> Result<!> {
    //     Err(Error::UnexpectedPacketType)
    // }
    // fn parse_subscribe(data: &[u8]) -> Result<!> {
    //     Err(Error::UnexpectedPacketType)
    // }
    // fn parse_unsubscribe(data: &[u8]) -> Result<!> {
    //     Err(Error::UnexpectedPacketType)
    // }
    // fn parse_pingreq(data: &[u8]) -> Result<!> {
    //     Err(Error::UnexpectedPacketType)
    // }

    // fn parse_connack(data: &[u8]) -> Result<Connack<S>> {
    //     Ok(Complete(Connack {
    //         user_properties: S::default(),
    //     }))
    // }

    // fn parse_publish(data: &[u8]) -> Result<Publish> {
    //     todo!()
    // }

    // fn parse_puback(data: &[u8]) -> Result<Puback> {
    //     todo!()
    // }

    // fn parse_pubrec(data: &[u8]) -> Result<Pubrec> {
    //     todo!()
    // }

    // fn parse_pubrel(data: &[u8]) -> Result<Pubrel> {
    //     todo!()
    // }

    // fn parse_pubcomp(data: &[u8]) -> Result<Pubcomp> {
    //     todo!()
    // }

    // fn parse_suback(data: &[u8]) -> Result<Suback> {
    //     todo!()
    // }

    // fn parse_unsuback(data: &[u8]) -> Result<Unsuback> {
    //     todo!()
    // }

    // fn parse_pingresp(data: &[u8]) -> Result<Pingresp> {
    //     todo!()
    // }

    // fn parse_disconnect(data: &[u8]) -> Result<Disconnect> {
    //     todo!()
    // }

    // fn parse_auth(data: &[u8]) -> Result<Auth> {
    //     todo!()
    // }

    // const IS_SERVER: bool = false;

    // type PacketType = ClientPacketType;
    // type ConnectPacket = Connect;
    // type ConnackPacket = !;
    // type PublishPacket = Publish;

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
}
