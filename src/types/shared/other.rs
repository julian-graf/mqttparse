use core::num::NonZero;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeepAlive {
    Infinite,
    Seconds(NonZero<u16>),
}

impl KeepAlive {
    pub fn new(value: u16) -> Self {
        NonZero::new(value).map_or(Self::Infinite, Self::Seconds)
    }
}
