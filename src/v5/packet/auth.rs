use crate::{
    base::ParserBase,
    reason_code,
    types::v5::property::{AuthenticationData, AuthenticationMethod, ReasonString, UserProperty},
};

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

pub struct Auth<RC, B: ParserBase, UPS> {
    pub reason_code: RC,
    pub authentication_method: AuthenticationMethod<B>,
    pub authentication_data: Option<AuthenticationData<B>>,
    pub reason_string: Option<ReasonString<B>>,
    pub user_properties: UPS,
}
