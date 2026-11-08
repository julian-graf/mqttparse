use crate::{base::StableRef, types::MqttString};

/// A name-value pair of two [`MqttString`]'s.
#[derive(Default, Clone, PartialEq, Eq)]
pub struct MqttStringPair<B> {
    /// The name part of the string pair.
    pub name: MqttString<B>,

    /// The value part of the string pair.
    pub value: MqttString<B>,
}

// impl<'s> core::fmt::Debug for MqttStringPair<B> {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         f.debug_struct("MqttStringPair")
//             .field("name", &self.name.as_str())
//             .field("value", &self.value.as_str())
//             .finish()
//     }
// }

impl<B> MqttStringPair<B> {
    /// Creates a new [`MqttStringPair`]
    #[must_use]
    pub const fn new(name: MqttString<B>, value: MqttString<B>) -> Self {
        Self { name, value }
    }
}

impl<B: StableRef> core::fmt::Debug for MqttStringPair<B> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MqttStringPair")
            .field("name", &self.name.as_str())
            .field("value", &self.value.as_str())
            .finish()
    }
}
