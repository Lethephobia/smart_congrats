#![no_std]

pub mod prelude {
    pub use crate::types::*;
}

pub mod types {
    #[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub struct DateYmd(pub u32);

    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
    pub struct Hash32(pub [u8; 32]);
}
