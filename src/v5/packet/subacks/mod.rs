use core::marker::PhantomData;

use crate::{
    base::{ParserBase, Storage},
    types::{
        PacketIdentifier,
        v5::property::{ReasonString, UserProperty},
    },
};

pub mod suback;
pub mod unsuback;

pub trait SubackPacketType {
    const FLAGS: u8;
    const PACKET_TYPE: u8;
}

pub struct GenericSuback<B, RC, UPS, RCS, P: SubackPacketType> {
    pub _packet_type: PhantomData<P>,
    pub _reason_code_type: PhantomData<RC>,

    pub reason_string: Option<ReasonString<B>>,
    pub user_properties: UPS,

    pub reason_codes: RCS,
}
