use crate::{
    base::{ParserBase, Storage},
    types::{
        IdentifiedQoS,
        v5::{
            TopicReference,
            property::{
                ContentType, CorrelationData, MessageExpiryInterval, PayloadFormatIndicator,
                ResponseTopic, SubscriptionIdentifier, UserProperty,
            },
        },
    },
    v5::Role,
};

// pub enum Packet<FROM: Role<S> + ?Sized, T, S: Storage<T>> {
//     PhantomVariant(T, !),
//     Connect(PEER::PeerConnect<S>),
//     Connack(PEER::PeerConnack<S>),
//     Publish(PEER::PeerPublish<S>),
//     Puback(PEER::PeerPuback<S>),
//     Pubrec(PEER::PeerPubrec<S>),
//     Pubrel(PEER::PeerPubrel<S>),
//     Pubcomp(PEER::PeerPubcomp<S>),
//     Subscribe(PEER::PeerSubscribe<S>),
//     Suback(PEER::PeerSuback<S>),
//     Unsubscribe(PEER::PeerUnsubscribe<S>),
//     Unsuback(PEER::PeerUnsuback<S>),
//     Pingreq(PEER::PeerPingreq<S>),
//     Pingresp(PEER::PeerPingresp<S>),
//     Disconnect(PEER::PeerDisconnect<S>),
//     Auth(PEER::PeerAuth<S>),
// }

pub struct Connect {}
pub struct Connack {}


#[derive(Debug, Clone)]
pub struct Publish<
    B: ParserBase,
    SubscriptionIdentifier,
    SubscriptionIdentifierStorage: Storage<Element = SubscriptionIdentifier>,
    UserPropertyStorage: Storage<Element = UserProperty<B>>,
> {
    pub dup: bool,
    pub identified_qos: IdentifiedQoS,
    pub retain: bool,

    pub topic: TopicReference<B>,

    // TODO clarify whether PayloadFormatIndicator can be included only once
    pub payload_format_indicator: Option<PayloadFormatIndicator>,
    // TODO clarify whether MessageExpiryInterval can be included only once
    pub message_expiry_interval: Option<MessageExpiryInterval>,
    pub response_topic: Option<ResponseTopic<B>>,
    pub correlation_data: Option<CorrelationData<B>>,
    pub user_properties: UserPropertyStorage,
    pub subscription_identifiers: SubscriptionIdentifierStorage,
    pub content_type: Option<ContentType<B>>,
    pub message: B,
}
pub struct Puback {}
pub struct Pubrec {}
pub struct Pubrel {}
pub struct Pubcomp {}
pub struct Subscribe {}
pub struct Suback {}
pub struct Unsubscribe {}
pub struct Unsuback {}
pub struct Pingreq {}
pub struct Pingresp {}
pub struct Disconnect {}
pub struct Auth {}
