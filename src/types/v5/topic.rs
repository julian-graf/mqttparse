use core::num::NonZero;

use crate::{base::StableRef, types::TopicName};

/// The options for specifying which topic to publish to. Topic aliases only last for the
/// duration of a single network connection and not necessarily until the session end.
///
/// Topic aliases must not be 0
#[derive(Clone, PartialEq, Eq)]
pub enum TopicReference<B> {
    /// Publish to the inner topic name without creating an alias.
    Name(TopicName<B>),

    /// Publish to an already mapped topic alias. The alias must have been defined earlier
    /// in the network connection.
    Alias(NonZero<u16>),

    /// Create a new topic alias or replace an existing topic alias.
    /// The alias lasts until the end of the network connection.
    Mapping(TopicName<B>, NonZero<u16>),
}

impl<B> TopicReference<B> {
    /// If present in the variant, returns the topic alias value of the [`TopicReference`].
    /// This is the case for [`TopicReference::Mapping`] and [`TopicReference::Alias`].
    pub fn alias(&self) -> Option<NonZero<u16>> {
        match self {
            Self::Name(_) => None,
            Self::Alias(alias) => Some(*alias),
            Self::Mapping(_, alias) => Some(*alias),
        }
    }

    /// If present in the variant, returns the topic name value of the [`TopicReference`].
    /// This is the case for [`TopicReference::Name`] and [`TopicReference::Mapping`].
    pub fn name(&self) -> Option<&TopicName<B>> {
        match self {
            Self::Name(topic_name) => Some(topic_name),
            Self::Alias(_) => None,
            Self::Mapping(topic_name, _) => Some(topic_name),
        }
    }
}

impl<B: StableRef> core::fmt::Debug for TopicReference<B> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // TODO this prints Name(TopicName("a/b/c")) instead of Name("a/b/c")
        match self {
            Self::Name(name) => f.debug_tuple("Name").field(name).finish(),
            Self::Alias(alias) => f.debug_tuple("Alias").field(alias).finish(),
            Self::Mapping(name, alias) => f.debug_tuple("Mapping").field(name).field(alias).finish(),
        }
    }
}
