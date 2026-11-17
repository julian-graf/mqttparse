use crate::{
    reason_code,
    v5::packet::subacks::{GenericSuback, SubackPacketType},
};

reason_code! {
    UnsubackReasonCode {
        /// The subscription is deleted.
        Success = 0x00,
        /// No matching Topic Filter is being used by the Client.
        NoSubscriptionExisted = 0x11,
        /// The unsubscribe could not be completed and the Server either does not wish to reveal the reason or none of the other Reason Codes apply.
        UnspecifiedError = 0x80,
        /// The UNSUBSCRIBE is valid but the Server does not accept it.
        ImplementationSpecificError = 0x83,
        /// The Client is not authorized to unsubscribe.
        NotAuthorized = 0x87,
        /// The Topic Filter is correctly formed but is not allowed for this Client.
        TopicFilterInvalid = 0x8F,
        /// An implementation or administrative imposed limit has been exceeded.
        QuotaExceeded = 0x97
    }
}

pub type Unsuback<B, UPS, RCS> = GenericSuback<B, UnsubackReasonCode, UPS, RCS, UnsubackPacket>;

pub struct UnsubackPacket;

impl SubackPacketType for UnsubackPacket {
    const FLAGS: u8 = 0x00;
    const PACKET_TYPE: u8 = 0xB0;
}
