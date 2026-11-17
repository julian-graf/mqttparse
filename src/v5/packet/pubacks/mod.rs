use core::marker::PhantomData;

use crate::{
    Error, Header, Result, Status, base::{FromParserBase, ParserBase, Storage}, types::{
        PacketIdentifier, VarByteInt, v5::property::{OnceHelper, PropertyType, ReasonString, UserProperty},
    },
};

pub mod puback;
pub mod pubcomp;
pub mod pubrec;
pub mod pubrel;

pub trait PubackPacketType {
    const FLAGS: u8;
    const PACKET_TYPE: u8;
}

pub trait OmissibleReasonCode {
    fn omitted() -> Self;
}

pub struct GenericPuback<P: PubackPacketType, B, RC, UPS> {
    pub _packet_type: PhantomData<P>,

    pub packet_identifier: PacketIdentifier,
    pub reason_code: RC,

    pub reason_string: Option<ReasonString<B>>,
    pub user_properties: UPS,
}

pub fn parse_generic_puback<
    T: Into<u8>,
    P: PubackPacketType,
    B: ParserBase,
    RCE: Into<Error>,
    RC: FromParserBase<B, Error = RCE> + OmissibleReasonCode,
    UPS: Storage<Element = UserProperty<B>>,
>(
    header: Header<T>,
    mut data: B,
) -> Result<GenericPuback<P, B, RC, UPS>> {
    assert_eq!(header.packet_type.into(), P::PACKET_TYPE);

    let len: usize = header
        .remaining_len
        .try_into()
        .map_err(|_| Error::PacketTooLarge)?;

    if header.flags != P::FLAGS {
        return Err(Error::InvalidFixedHeaderFlags);
    }

    let mut backing = match data.split_to_checked(len) {
        Some(d) => d,
        None => return Ok(Status::Partial),
    };

    let data = &mut backing;

    let packet_identifier = PacketIdentifier::parse(data)?;

    let reason_code = if len == 2 {
        RC::omitted()
    } else {
        RC::parse(data).map_err(Into::into)?
    };

    let properties_length: usize = if len < 4 {
        0
    } else {
        VarByteInt::parse(data)?
            .try_into()
            .map_err(|_| Error::InvalidPropertyLength)?
    };

    if properties_length != data.len() {
        return Err(todo!("illegal payload"));
    }

    let properties_data = data;

    let mut reason_string: Option<ReasonString<B>> = None;
    let mut user_properties = UPS::default();

    // TODO convert RemainingLengthExceeded to PropertyLengthExceeded here
    while properties_data.len() > 0 {
        let property_type = PropertyType::parse(properties_data)?;

        match property_type {
            PropertyType::ReasonString => reason_string.set(properties_data)?,
            PropertyType::UserProperty => {
                let user_property = UserProperty::parse(properties_data)?;
                if let Some(limit) = UPS::LIMIT
                    && user_properties.len() == limit
                {
                    // Storage is full
                } else {
                    user_properties.extend(Some(user_property));
                }
            }
            p => return Err(Error::InvalidPropertyType),
        }
    }

    Ok(Status::Complete(GenericPuback {
        _packet_type: PhantomData,
        packet_identifier,
        reason_code,
        reason_string,
        user_properties,
    }))
}
