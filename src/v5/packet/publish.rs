use core::marker::PhantomData;

use crate::{
    base::{ParserBase, Storage},
    types::{
        IdentifiedQoS,
        v5::{
            TopicReference,
            property::{
                ContentType, CorrelationData, MessageExpiryInterval, PayloadFormatIndicator,
                ResponseTopic, UserProperty,
            },
        },
    },
};

#[derive(Debug, Clone)]
pub struct Publish<B: ParserBase, SIS, UPS> {
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
    pub user_properties: UPS,
    pub subscription_identifiers: SIS,
    pub content_type: Option<ContentType<B>>,
    pub message: B,
}
