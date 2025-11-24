//! Generated bindings for cupti.

#![allow(nonstandard_style)]
#![allow(rustdoc::all)]
#![allow(clippy::all)]

mod bindings;

pub use bindings::*;

macro_rules! cupti_struct_size {
    ($type:ty, $lastfield:ident) => {{
        let instance: $type = unsafe { std::mem::zeroed() };

        ::core::mem::offset_of!($type, $lastfield) + std::mem::size_of_val(&instance.$lastfield)
    }};
}

include!("sizes.rs");
