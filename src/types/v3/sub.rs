use crate::types::{QoS, TopicFilter};

#[derive(Clone, PartialEq, Eq)]
pub struct SubscriptionFilter<B> {
    topic: TopicFilter<B>,
    qos: QoS,
}

impl<B> SubscriptionFilter<B> {
    pub fn new(topic: TopicFilter<B>, qos: QoS) -> Self {
        Self { topic, qos }
    }
}
