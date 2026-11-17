use crate::{
    reason_code,
    v5::packet::subacks::{GenericSuback, SubackPacketType},
};

reason_code! {
    SubackReasonCode {
        /// The subscription is accepted and the maximum QoS sent will be QoS 0. This might be a lower QoS than was requested.
        GrantedQoS0 = 0x00,
        /// The subscription is accepted and the maximum QoS sent will be QoS 1. This might be a lower QoS than was requested.
        GrantedQoS1 = 0x01,
        /// The subscription is accepted and any received QoS will be sent to this subscription.
        GrantedQoS2 = 0x02,
        /// The subscription is not accepted and the Server either does not wish to reveal the reason or none of the other Reason Codes apply.
        UnspecifiedError = 0x80,
        /// The SUBSCRIBE is valid but the Server does not accept it.
        ImplementationSpecificError = 0x83,
        /// The Client is not authorized to make this subscription.
        NotAuthorized = 0x87,
        /// The Topic Filter is correctly formed but is not allowed for this Client.
        TopicFilterInvalid = 0x8F,
        /// An implementation or administrative imposed limit has been exceeded.
        QuotaExceeded = 0x97,
        /// The Server does not support Shared Subscriptions for this Client.
        SharedSubscriptionsNotSupported = 0x9E,
        /// The Server does not support Subscription Identifiers; the subscription is not accepted.
        SubscriptionIdentifiersNotSupported = 0xA1,
        /// The Server does not support Wildcard Subscriptions; the subscription is not accepted.
        WildcardSubscriptionsNotSupported = 0xA2
    }
}

pub type Suback<B, UPS, RCS> = GenericSuback<B, SubackReasonCode, UPS, RCS, SubackPacket>;

pub struct SubackPacket;

impl SubackPacketType for SubackPacket {
    const FLAGS: u8 = 0x00;
    const PACKET_TYPE: u8 = 0x90;
}
