mod shared;

pub use shared::*;
pub mod v3;
pub mod v5;

/// The error returned when types are larger than what is representable according to the specification.
///
/// * [`VarByteInt`]: If [`VarByteInt::MAX_ENCODABLE`] is exceeded, an error of this type is returned.
/// * [`MqttBinary`]: If [`MqttBinary::MAX_LENGTH`] is exceeded, an error of this type is returned.
#[derive(Debug)]
pub struct TooLargeToEncode;
