use core::str::Utf8Error;

use crate::{
    Error,
    base::{FromParserBase, ParserBase},
    types::{MqttBinary, MqttString, MqttStringError},
};

mod properties;
mod types;

pub use properties::*;
pub use types::PropertyType;

pub trait Property
where
    Self: Into<Self::Inner>,
{
    const TYPE: PropertyType;
    type Inner;
}

pub struct PropertyError<E>(pub(crate) E);

pub trait OnceHelper<B> {
    fn set(&mut self, b: &mut B) -> Result<(), Error>;
}
impl<B, T> OnceHelper<B> for Option<T>
where
    B: ParserBase,
    T: FromParserBase<B>,
    T::Error: Into<Error>,
{
    fn set(&mut self, b: &mut B) -> Result<(), Error> {
        if self.is_some() {
            Err(Error::DuplicateProperty)
        } else {
            self.replace(T::parse(b).map_err(Into::into)?);
            Ok(())
        }
    }
}
