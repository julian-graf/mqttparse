use crate::{
    Error,
    base::{ParserBase, TooShort, VarByteIntError},
    types::{VarByteInt, v5::property::SubscriptionIdentifier},
    v5::{client::NeverSubscriptionIdentifierError, roles::Role},
};

#[derive(Debug, Clone, Copy)]
pub struct Server;

#[derive(Debug, Clone, Copy)]
pub enum ServerPacketType {
    Connack,
    Publish,
    Puback,
    Pubrec,
    Pubrel,
    Pubcomp,
    Suback,
    Unsuback,
    Pingresp,
    Disconnect,
    Auth,
}

impl Role for Server {
    // const IS_SERVER: bool = true;

    type PacketType = ServerPacketType;
    type PublishSubscriptionIdentifierError = VarByteIntError;
    type PublishSubscriptionIdentifier<B: ParserBase> = SubscriptionIdentifier;
    // type ConnectPacket = !;
    // type ConnackPacket = Connack;
    // type PublishPacket = Publish;

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
}
