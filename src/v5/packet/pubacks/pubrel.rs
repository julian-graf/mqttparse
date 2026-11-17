use crate::{
    base::{ParserBase, Storage}, reason_code, types::{
        PacketIdentifier,
        v5::property::{ReasonString, UserProperty},
    }, v5::packet::pubacks::{GenericPuback, OmissibleReasonCode, PubackPacketType},
};

reason_code! {
    PubrelReasonCode {
        /// Message released.
        Success = 0x00,
        /// The Packet Identifier is not known. This is not an error during recovery, but at other times indicates a mismatch between the Session State on the Client and Server.
        PacketIdentifierNotFound = 0x92
    }
}

impl OmissibleReasonCode for PubrelReasonCode {
    fn omitted() -> Self {
        Self::Success
    }
}

pub type Pubrel<B, UPS> = GenericPuback<PubrelPacket, B, PubrelReasonCode, UPS>;

pub struct PubrelPacket;

impl PubackPacketType for PubrelPacket {
    const FLAGS: u8 = 0x02;
    const PACKET_TYPE: u8 = 0x60;
}
