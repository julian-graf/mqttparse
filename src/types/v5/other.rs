use core::num::NonZero;

/// The handling of a session after a disconnection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
// #[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SessionExpiryInterval {
    /// The session ends the moment a DISCONNECT packet is sent or the network connection closes.
    #[default]
    EndOnDisconnect,
    /// The session is not ended under any circumstances.
    NeverEnd,
    /// The session ends after this many seconds have passed after a DISCONNECT packet is sent or the network connection closes.
    Seconds(NonZero<u32>),
}

/// Maximum packet size. Exceeding this is a protocol error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
// #[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MaximumPacketSize {
    /// There is no imposed limit on how large packets can be. The technical limit is
    /// [`VarByteInt::MAX_ENCODABLE`] + 5 (size of fixed header).
    ///
    /// [`VarByteInt::MAX_ENCODABLE`]: crate::types::VarByteInt::MAX_ENCODABLE
    #[default]
    Unlimited,

    /// There is a limit on how large packets can be. The packet size is the value of its remaining length
    /// plus the size of the fixed header. A value of 0 is not allowed by the specification. A value less
    /// than 2 does not make sense because every MQTT packet contains a fixed header of at least 2 bytes.
    Limit(NonZero<u32>),
}
