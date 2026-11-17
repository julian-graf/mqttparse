use crate::{
    base::{ParserBase, Storage},
    reason_code,
    types::v5::{
        ReasonCode,
        property::{
            AssignedClientIdentifier, AuthenticationData, AuthenticationMethod, MaximumPacketSize,
            MaximumQoS, ReasonString, ReceiveMaximum, ResponseInformation, RetainAvailable,
            ServerKeepAlive, ServerReference, SessionExpiryInterval, SharedSubscriptionAvailable,
            SubscriptionIdentifierAvailable, TopicAliasMaximum, UserProperty,
            WildcardSubscriptionAvailable,
        },
    },
};

reason_code! {
    ConnackReasonCode {
        /// The Connection is accepted.
        Success = 0x00,
        /// The Server does not wish to reveal the reason for the failure, or none of the other Reason Codes apply.
        UnspecifiedError = 0x80,
        /// Data within the CONNECT packet could not be correctly parsed.
        MalformedPacket = 0x81,
        /// Data in the CONNECT packet does not conform to this specification.
        ProtocolError = 0x82,
        /// The CONNECT is valid but is not accepted by this Server.
        ImplementationSpecificError = 0x83,
        /// The Server does not support the version of the MQTT protocol requested by the Client.
        UnsupportedProtocolVersion = 0x84,
        /// The Client Identifier is a valid string but is not allowed by the Server.
        ClientIdentifierNotValid = 0x85,
        /// The Server does not accept the User Name or Password specified by the Client.
        BadUserNameOrPassword = 0x86,
        /// The Client is not authorized to connect.
        NotAuthorized = 0x87,
        /// The MQTT Server is not available.
        ServerUnavailable = 0x88,
        /// The Server is busy. Try again later.
        ServerBusy = 0x89,
        /// This Client has been banned by administrative action. Contact the server administrator.
        Banned = 0x8A,
        /// The authentication method is not supported or does not match the authentication method currently in use.
        BadAuthenticationMethod = 0x8C,
        /// The Will Topic Name is not malformed, but is not accepted by this Server.
        TopicNameInvalid = 0x90,
        /// The CONNECT packet exceeded the maximum permissible size.
        PacketTooLarge = 0x95,
        /// An implementation or administrative imposed limit has been exceeded.
        QuotaExceeded = 0x97,
        /// The Will Payload does not match the specified Payload Format Indicator.
        PayloadFormatInvalid = 0x99,
        /// The Server does not support retained messages, and Will Retain was set to 1.
        RetainNotSupported = 0x9A,
        /// The Server does not support the QoS set in Will QoS.
        QoSNotSupported = 0x9B,
        /// The Client should temporarily use another server.
        UseAnotherServer = 0x9C,
        /// The Client should permanently use another server.
        ServerMoved = 0x9D,
        /// The connection rate limit has been exceeded.
        ConnectionRateExceeded = 0x9F
    }
}

#[derive(Clone)]
pub struct Connack<B, UPS> {
    pub session_present: bool,
    pub reason_code: ConnackReasonCode,

    // CONNACK properties
    pub session_expiry_interval: Option<SessionExpiryInterval>,
    pub receive_maximum: Option<ReceiveMaximum>,
    pub maximum_qos: Option<MaximumQoS>,
    pub retain_available: Option<RetainAvailable>,
    pub maximum_packet_size: Option<MaximumPacketSize>,
    pub assigned_client_identifier: Option<AssignedClientIdentifier<B>>,
    pub topic_alias_maximum: Option<TopicAliasMaximum>,
    pub reason_string: Option<ReasonString<B>>,
    pub user_properties: UPS,
    pub wildcard_subscription_available: Option<WildcardSubscriptionAvailable>,
    pub subscription_identifier_available: Option<SubscriptionIdentifierAvailable>,
    pub shared_subscription_available: Option<SharedSubscriptionAvailable>,
    pub server_keep_alive: Option<ServerKeepAlive>,
    pub response_information: Option<ResponseInformation<B>>,
    pub server_reference: Option<ServerReference<B>>,
    pub authentication_method: Option<AuthenticationMethod<B>>,
    pub authentication_data: Option<AuthenticationData<B>>,
}

impl<B: ParserBase, UPS: core::fmt::Debug> core::fmt::Debug for Connack<B, UPS> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Connack")
            .field("session_present", &self.session_present)
            .field("reason_code", &self.reason_code)
            .field("session_expiry_interval", &self.session_expiry_interval)
            .field("receive_maximum", &self.receive_maximum)
            .field("maximum_qos", &self.maximum_qos)
            .field("retain_available", &self.retain_available)
            .field("maximum_packet_size", &self.maximum_packet_size)
            .field(
                "assigned_client_identifier",
                &self.assigned_client_identifier,
            )
            .field("topic_alias_maximum", &self.topic_alias_maximum)
            .field("reason_string", &self.reason_string)
            .field("user_properties", &self.user_properties)
            .field(
                "wildcard_subscription_available",
                &self.wildcard_subscription_available,
            )
            .field(
                "subscription_identifier_available",
                &self.subscription_identifier_available,
            )
            .field(
                "shared_subscription_available",
                &self.shared_subscription_available,
            )
            .field("server_keep_alive", &self.server_keep_alive)
            .field("response_information", &self.response_information)
            .field("server_reference", &self.server_reference)
            .field("authentication_method", &self.authentication_method)
            .field("authentication_data", &self.authentication_data)
            .finish()
    }
}
