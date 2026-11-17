use crate::{base::StableRef, types::MqttString};

pub enum TopicNameError {
    Empty,
    Wildcard,
}

/// A topic name string for that messages can be published on according to <https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901241>.
/// Cannot contain wildcard characters.
///
/// Examples:
/// - "sport/tennis/player1"
/// - "sport/tennis/player1/ranking"
/// - "sport/tennis/player1/score/wimbledon"
#[derive(Clone, PartialEq, Eq)]
pub struct TopicName<B>(MqttString<B>);

impl<B: StableRef> TopicName<B> {
    fn is_valid(s: &MqttString<B>) -> Result<(), TopicNameError> {
        let s = s.as_str().as_bytes();

        // [MQTT-4.7.3-1]
        // Topic names must be at least one character long.
        if s.is_empty() {
            return Err(TopicNameError::Empty);
        }

        let mut i = 0;

        while i < s.len() {
            let b = s[i];

            // [MQTT-4.7.3-2]
            // Topic names must not include the null character.
            // No null characters are an invariant of `MqttString`

            // [MQTT-4.7.0-1]
            // Wildcard characters must not be used within a topic name.
            if b == b'+' || b == b'#' {
                return Err(TopicNameError::Wildcard);
            }

            i += 1;
        }

        Ok(())
    }

    /// Creates a new topic name while checking for correct syntax of the topic name string.
    pub fn new(string: MqttString<B>) -> Result<Self, TopicNameError> {
        Self::is_valid(&string).map(|_| Self(string))
    }

    /// Creates a new topic name without checking for correct syntax of the topic name string.
    ///
    /// # Invariants
    /// The syntax of the topic name is valid. For a fallible version, use [`TopicName::new`]
    ///
    /// # Panics
    /// In debug builds, this function will panic if the syntax of `string` is incorrect.
    #[must_use]
    pub fn new_unchecked(string: MqttString<B>) -> Self {
        debug_assert!(
            Self::is_valid(&string).is_ok(),
            "the provided string is not valid TopicName syntax"
        );

        Self(string)
    }
}

impl<B: StableRef> core::fmt::Debug for TopicName<B> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TopicName").field(&self.0.as_str()).finish()
    }
}
impl<B> AsRef<MqttString<B>> for TopicName<B> {
    fn as_ref(&self) -> &MqttString<B> {
        &self.0
    }
}
impl<B> From<TopicName<B>> for MqttString<B> {
    fn from(value: TopicName<B>) -> Self {
        value.0
    }
}

/// A topic filter string for subscribing to certain topics according to <https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901241>.
/// Can contain wildcard characters.
///
/// Examples:
/// - "sport/tennis/#"
/// - "sport/+/player1"
#[derive(Clone, PartialEq, Eq)]
pub struct TopicFilter<B>(MqttString<B>);

impl<B: StableRef> TopicFilter<B> {
    fn is_valid(s: &MqttString<B>) -> bool {
        let s = s.as_str().as_bytes();

        // [MQTT-4.7.3-1]
        // Topic filters must be at least one character long.
        if s.is_empty() {
            return false;
        }

        let is_shared = s.len() >= 6
            && s[0] == b'$'
            && s[1] == b's'
            && s[2] == b'h'
            && s[3] == b'a'
            && s[4] == b'r'
            && s[5] == b'e';

        if is_shared {
            // Minimal shared filter "$share/a/a" has length 10
            if s.len() < 10 {
                return false;
            }
            // Check first '/' so that checking the sharename is simpler in the loop
            if s[6] != b'/' {
                return false;
            }
        }

        let mut i = match is_shared {
            true => 7,
            false => 0,
        };
        let mut level_len = 0;
        let mut level_contains_wildcard = false;

        let mut checking_share_name = is_shared;

        while i < s.len() {
            let b = s[i];

            // [MQTT-4.7.3-2]
            // Topic filters must not include the null character.
            // No null characters are an invariant of `MqttString`

            if b == b'#' {
                // [MQTT-4.7.1-1]
                // The multi-level wildcard character must be specified on its own.
                // The multi-level wildcard character must be the last character specified in the topic filter.
                if i == s.len() - 1 {
                    level_contains_wildcard = true;
                } else {
                    return false;
                }
            }

            if b == b'+' {
                level_contains_wildcard = true;
            }

            if b == b'/' {
                if checking_share_name {
                    // ... a ShareName that is at least one character long [MQTT-4.8.2-1].
                    if level_len == 0 {
                        return false;
                    }

                    // The ShareName MUST NOT contain the characters "/", "+" or "#", but MUST be
                    // followed by a "/" character. ...
                    // Topic Filter [MQTT-4.8.2-2]
                    if level_contains_wildcard {
                        return false;
                    }

                    // ... but MUST be followed by a "/" character. This "/" character MUST be
                    // followed by a Topic Filter [MQTT-4.8.2-2]
                    if i + 1 == s.len() {
                        return false;
                    }

                    checking_share_name = false;
                }

                level_len = 0;
                level_contains_wildcard = false;
            } else {
                level_len += 1;

                // [MQTT-4.7.1-2]
                // The single-level wildcard must occupy an entire level of the filter.
                // [MQTT-4.7.1-1]
                // The multi-level wildcard character must be specified on its own.
                if level_len > 1 && level_contains_wildcard {
                    return false;
                }
            }

            i += 1;
        }

        !checking_share_name
    }

    /// Returns whether the topic filter is the topic filter of a shared subscription.
    pub fn is_shared(&self) -> bool {
        let s = self.0.as_str().as_bytes();

        s.len() >= 10
            && s[0] == b'$'
            && s[1] == b's'
            && s[2] == b'h'
            && s[3] == b'a'
            && s[4] == b'r'
            && s[5] == b'e'
    }

    /// Returns whether the topic filter contains one or more of the wildcard characters `#` or `+`.
    pub fn has_wildcard(&self) -> bool {
        let s = self.0.as_str().as_bytes();

        let mut i = 0;
        while i < s.len() {
            let b = s[i];

            if b == b'#' || b == b'+' {
                return true;
            }

            i += 1;
        }

        false
    }

    /// Creates a new topic filter while checking for correct syntax of the topic filter string.
    /// If the filter starts with "$share", the constraints for a shared subscription's topic
    /// filter are also enforced.
    #[must_use]
    pub fn new(string: MqttString<B>) -> Option<Self> {
        if Self::is_valid(&string) {
            Some(Self(string))
        } else {
            None
        }
    }

    /// Creates a new topic filter without checking for correct syntax of the topic filter string.
    ///
    /// # Invariants
    /// The syntax of the topic filter is valid. For a fallible version, use [`TopicFilter::new`].
    ///
    /// # Panics
    /// In debug builds, this function will panic if the syntax of `string` is incorrect.
    #[must_use]
    pub fn new_unchecked(string: MqttString<B>) -> Self {
        debug_assert!(
            Self::is_valid(&string),
            "the provided string is not valid TopicFilter syntax"
        );

        Self(string)
    }
}

impl<B: StableRef> core::fmt::Debug for TopicFilter<B> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TopicFilter")
            .field(&self.0.as_str())
            .finish()
    }
}
impl<B> AsRef<MqttString<B>> for TopicFilter<B> {
    fn as_ref(&self) -> &MqttString<B> {
        &self.0
    }
}
impl<B> From<TopicFilter<B>> for MqttString<B> {
    fn from(value: TopicFilter<B>) -> Self {
        value.0
    }
}
