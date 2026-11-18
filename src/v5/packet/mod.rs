use crate::{
    base::{ParserBase, Storage},
    types::v5::property::UserProperty,
    v5::role::Role,
};

mod auth;
mod connack;
mod connect;
mod disconnect;
mod pingreq;
mod pingresp;
mod pubacks;
mod publish;
mod subacks;
mod subscribe;
mod unsubscribe;

pub use auth::{Auth, ClientAuthReasonCode, ServerAuthReasonCode};
pub use connack::{Connack, ConnackReasonCode};
pub use connect::Connect;
pub use disconnect::{ClientDisconnectReasonCode, Disconnect, ServerDisconnectReasonCode};
pub use pingreq::Pingreq;
pub use pingresp::Pingresp;
pub use pubacks::{
    GenericPuback, OmissibleReasonCode, PubackPacketType, parse_generic_puback,
    puback::{ClientPubackReasonCode, Puback, ServerPubackReasonCode},
    pubcomp::{Pubcomp, PubcompReasonCode},
    pubrec::{ClientPubrecReasonCode, Pubrec, ServerPubrecReasonCode},
    pubrel::{Pubrel, PubrelReasonCode},
};
pub use publish::Publish;
pub use subacks::{
    GenericSuback,
    suback::{Suback, SubackReasonCode},
    unsuback::{Unsuback, UnsubackReasonCode},
};
pub use subscribe::Subscribe;
pub use unsubscribe::Unsubscribe;

pub enum Packet<FROM: Role + ?Sized, B: ParserBase, UPS, SIS, SRCS, URCS> {
    Connect(FROM::Connect),
    Connack(FROM::Connack<B, UPS>),
    Publish(Publish<B, SIS, UPS>),
    Puback(Puback<B, FROM::PubackReasonCode<B>, UPS>),
    Pubrec(Pubrec<B, FROM::PubrecReasonCode<B>, UPS>),
    Pubrel(Pubrel<B, UPS>),
    Pubcomp(Pubcomp<B, UPS>),
    Subscribe(FROM::Subscribe),
    Suback(FROM::Suback<B, UPS, SRCS>),
    Unsubscribe(FROM::Unsubscribe),
    Unsuback(FROM::Unsuback<B, UPS, URCS>),
    Pingreq(FROM::Pingreq),
    Pingresp(FROM::Pingresp),
    Disconnect(
        Disconnect<
            B,
            FROM::DisconnectReasonCode<B>,
            FROM::DisconnectSessionExpiryInterval<B>,
            FROM::DisconnectServerReference<B>,
            UPS,
        >,
    ),
    Auth(Auth<FROM::AuthReasonCode<B>, B, UPS>),
}
