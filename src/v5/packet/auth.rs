use crate::reason_code;

reason_code! {
    ClientAuthReasonCode {
        /// Continue the authentication with another step.
        ContinueAuthentication = 0x18,
        /// Initiate a re-authentication.
        ReAuthenticate = 0x19
    }
}

reason_code! {
    ServerAuthReasonCode {
        /// Authentication is successful.
        Success = 0x00,
        /// Continue the authentication with another step.
        ContinueAuthentication = 0x18
    }
}

pub struct Auth<RC> {
    pub reason_code: RC,
}
