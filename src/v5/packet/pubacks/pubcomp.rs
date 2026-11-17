use crate::{
    base::{ParserBase, Storage}, reason_code, types::{
        PacketIdentifier,
        v5::property::{ReasonString, UserProperty},
    }, v5::packet::pubacks::{GenericPuback, OmissibleReasonCode, PubackPacketType},
};

reason_code! {
    PubcompReasonCode {
        /// Packet Identifier released. Publication of QoS 2 message is complete.
        Success = 0x00,
        /// The Packet Identifier is not known. This is not an error during recovery, but at other times indicates a mismatch between the Session State on the Client and Server.
        PacketIdentifierNotFound = 0x92
    }
}

impl OmissibleReasonCode for PubcompReasonCode {
    fn omitted() -> Self {
        Self::Success
    }
}
pub type Pubcomp<B, UPS> = GenericPuback<PubcompPacket, B, PubcompReasonCode, UPS>;

pub struct PubcompPacket;

impl PubackPacketType for PubcompPacket {
    const FLAGS: u8 = 0x00;
    const PACKET_TYPE: u8 = 0x70;
}
