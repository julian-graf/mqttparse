use crate::{
    base::{ParserBase, Storage}, reason_code, types::v5::property::{ReasonString, UserProperty}, v5::packet::OmissibleReasonCode,
};

reason_code! {
    ClientDisconnectReasonCode {
    /// Close the connection normally. Do not send the Will Message.
    NormalDisconnection = 0x00,

    /// The Client wishes to disconnect but requires that the Server also publishes its Will Message.
    DisconnectWithWillMessage = 0x04,

    /// The Connection is closed but the sender either does not wish to reveal the reason, or none of the other Reason Codes apply.
    UnspecifiedError = 0x80,

    /// The received packet does not conform to this specification.
    MalformedPacket = 0x81,

    /// An unexpected or out of order packet was received.
    ProtocolError = 0x82,

    /// The packet received is valid but cannot be processed by this implementation.
    ImplementationSpecificError = 0x83,

    /// The Topic Name is correctly formed, but is not accepted by this Client or Server.
    TopicNameInvalid = 0x90,

    /// The Client or Server has received more than Receive Maximum publication for which it has not sent PUBACK or PUBCOMP.
    ReceiveMaximumExceeded = 0x93,

    /// The Client or Server has received a PUBLISH packet containing a Topic Alias which is greater than the Maximum Topic Alias it sent in the CONNECT or CONNACK packet.
    TopicAliasInvalid = 0x94,

    /// The packet size is greater than Maximum Packet Size for this Client or Server.
    PacketTooLarge = 0x95,

    /// The received data rate is too high.
    MessageRateTooHigh = 0x96,

    /// An implementation or administrative imposed limit has been exceeded.
    QuotaExceeded = 0x97,

    /// The Connection is closed due to an administrative action.
    AdministrativeAction = 0x98,

    /// The payload format does not match the one specified by the Payload Format Indicator.
    PayloadFormatInvalid = 0x99,
}}

reason_code! {
    ServerDisconnectReasonCode {
    /// Close the connection normally. Do not send the Will Message.
    NormalDisconnection = 0x00,

    /// The Connection is closed but the sender either does not wish to reveal the reason, or none of the other Reason Codes apply.
    UnspecifiedError = 0x80,

    /// The received packet does not conform to this specification.
    MalformedPacket = 0x81,

    /// An unexpected or out of order packet was received.
    ProtocolError = 0x82,

    /// The packet received is valid but cannot be processed by this implementation.
    ImplementationSpecificError = 0x83,

    /// The request is not authorized.
    NotAuthorized = 0x87,

    /// The Server is busy and cannot continue processing requests from this Client.
    ServerBusy = 0x89,

    /// The Server is shutting down.
    ServerShuttingDown = 0x8B,

    /// The Connection is closed because no packet has been received for 1.5 times the Keepalive time.
    KeepAliveTimeout = 0x8D,

    /// Another Connection using the same ClientID has connected causing this Connection to be closed.
    SessionTakenOver = 0x8E,

    /// The Topic Filter is correctly formed, but is not accepted by this Sever.
    TopicFilterInvalid = 0x8F,

    /// The Topic Name is correctly formed, but is not accepted by this Client or Server.
    TopicNameInvalid = 0x90,

    /// The Client or Server has received more than Receive Maximum publication for which it has not sent PUBACK or PUBCOMP.
    ReceiveMaximumExceeded = 0x93,

    /// The Client or Server has received a PUBLISH packet containing a Topic Alias which is greater than the Maximum Topic Alias it sent in the CONNECT or CONNACK packet.
    TopicAliasInvalid = 0x94,

    /// The packet size is greater than Maximum Packet Size for this Client or Server.
    PacketTooLarge = 0x95,

    /// The received data rate is too high.
    MessageRateTooHigh = 0x96,

    /// An implementation or administrative imposed limit has been exceeded.
    QuotaExceeded = 0x97,

    /// The Connection is closed due to an administrative action.
    AdministrativeAction = 0x98,

    /// The payload format does not match the one specified by the Payload Format Indicator.
    PayloadFormatInvalid = 0x99,

    /// The Server does not support retained messages.
    RetainNotSupported = 0x9A,

    /// The Client specified a QoS greater than the QoS specified in a Maximum QoS in the CONNACK.
    QoSNotSupported = 0x9B,

    /// The Client should temporarily change its Server.
    UseAnotherServer = 0x9C,

    /// The Server is moved and the Client should permanently change its server location.
    ServerMoved = 0x9D,

    /// The Server does not support Shared Subscriptions.
    SharedSubscriptionsNotSupported = 0x9E,

    /// This connection is closed because the connection rate is too high.
    ConnectionRateExceeded = 0x9F,

    /// The maximum connection time authorized for this connection has been exceeded.
    MaximumConnectTime = 0xA0,

    /// The Server does not support Subscription Identifiers; the subscription is not accepted.
    SubscriptionIdentifiersNotSupported = 0xA1,

    /// The Server does not support Wildcard Subscriptions; the subscription is not accepted.
    WildcardSubscriptionsNotSupported = 0xA2,
}}

impl OmissibleReasonCode for ClientDisconnectReasonCode {
    fn omitted() -> Self {
        Self::NormalDisconnection
    }
}
impl OmissibleReasonCode for ServerDisconnectReasonCode {
    fn omitted() -> Self {
        Self::NormalDisconnection
    }
}

pub struct Disconnect<B, RC, SessionExpiryInterval, ServerReference, UPS> {
    pub reason_code: RC,

    /// Never sent by server
    pub session_expiry_interval: Option<SessionExpiryInterval>,
    pub reason_string: Option<ReasonString<B>>,
    pub user_properties: UPS,
    pub server_reference: Option<ServerReference>,
}
