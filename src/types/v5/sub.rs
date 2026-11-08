use crate::types::TopicFilter;

#[derive(Clone, PartialEq, Eq)]
pub struct SubscriptionFilter<B> {
    topic: TopicFilter<B>,
    subscription_options: SubscriptionOptions,
}

/// The no local flag is set for a shared subscription, which is not allowed:
///
/// It is a Protocol Error to set the No Local bit to 1 on a Shared Subscription [MQTT-3.8.3-4].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoLocalSharedSubscription;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscriptionOptions {
    encoded: u8,
}

// impl<'t> SubscriptionFilter<'t> {
//     pub fn new(
//         topic: TopicFilter<'t>,
//         options: &SubscriptionOptions,
//     ) -> Result<Self, NoLocalSharedSubscription> {
//         if options.no_local && topic.is_shared() {
//             return Err(NoLocalSharedSubscription);
//         }

//         let retain_handling_bits = match options.retain_handling {
//             RetainHandling::AlwaysSend => 0x00,
//             RetainHandling::SendIfNotSubscribedBefore => 0x10,
//             RetainHandling::NeverSend => 0x20,
//         };

//         let retain_as_published_bit = match options.retain_as_published {
//             true => 0x08,
//             false => 0x00,
//         };

//         let no_local_bit = match options.no_local {
//             true => 0x04,
//             false => 0x00,
//         };

//         let qos_bits = options.qos.into_bits(0);

//         let subscribe_options_bits =
//             retain_handling_bits | retain_as_published_bit | no_local_bit | qos_bits;

//         Ok(Self {
//             topic,
//             subscription_options: subscribe_options_bits,
//         })
//     }
// }
