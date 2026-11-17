use crate::{
    base::{ParserBase, Storage}, reason_code, types::{
        PacketIdentifier,
        v5::property::{ReasonString, UserProperty},
    }, v5::packet::pubacks::{GenericPuback, OmissibleReasonCode, PubackPacketType},
};

reason_code! {
    ClientPubrecReasonCode {
        /// The message is accepted. Publication of the QoS 2 message proceeds.
        Success = 0x00,
        /// The receiver does not accept the publish but either does not want to reveal the reason, or it does not match one of the other values.
        UnspecifiedError = 0x80,
        /// The PUBLISH is valid but the receiver is not willing to accept it.
        ImplementationSpecificError = 0x83,
        /// The PUBLISH is not authorized.
        NotAuthorized = 0x87,
        /// The Topic Name is not malformed, but is not accepted by this Client or Server.
        TopicNameInvalid = 0x90,
        /// The Packet Identifier is already in use. This might indicate a mismatch in the Session State between the Client and Server.
        PacketIdentifierInUse = 0x91,
        /// An implementation or administrative imposed limit has been exceeded.
        QuotaExceeded = 0x97,
        /// The payload format does not match the one specified in the Payload Format Indicator.
        PayloadFormatInvalid = 0x99
    }
}

reason_code! {
    ServerPubrecReasonCode {
        /// The message is accepted. Publication of the QoS 2 message proceeds.
        Success = 0x00,
        /// The message is accepted but there are no subscribers. This is sent only by the Server. If the Server knows that there are no matching subscribers, it MAY use this Reason Code instead of 0x00 (Success).
        NoMatchingSubscribers = 0x10,
        /// The receiver does not accept the publish but either does not want to reveal the reason, or it does not match one of the other values.
        UnspecifiedError = 0x80,
        /// The PUBLISH is valid but the receiver is not willing to accept it.
        ImplementationSpecificError = 0x83,
        /// The PUBLISH is not authorized.
        NotAuthorized = 0x87,
        /// The Topic Name is not malformed, but is not accepted by this Client or Server.
        TopicNameInvalid = 0x90,
        /// The Packet Identifier is already in use. This might indicate a mismatch in the Session State between the Client and Server.
        PacketIdentifierInUse = 0x91,
        /// An implementation or administrative imposed limit has been exceeded.
        QuotaExceeded = 0x97,
        /// The payload format does not match the one specified in the Payload Format Indicator.
        PayloadFormatInvalid = 0x99
    }
}

impl OmissibleReasonCode for ClientPubrecReasonCode {
    fn omitted() -> Self {
        Self::Success
    }
}
impl OmissibleReasonCode for ServerPubrecReasonCode {
    fn omitted() -> Self {
        Self::Success
    }
}

pub type Pubrec<B, RC, UPS> = GenericPuback<PubrecPacket, B, RC, UPS>;

pub struct PubrecPacket;

impl PubackPacketType for PubrecPacket {
    const FLAGS: u8 = 0x00;
    const PACKET_TYPE: u8 = 0x50;
}
