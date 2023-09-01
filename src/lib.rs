#![doc = include_str!("../README.md")]

#![forbid(    
    unsafe_code,
        
    unused_imports,
    unused_variables,
    unused_mut,

    unused_allocation,
    unused_must_use,
    
    unreachable_patterns,
    
    trivial_casts,
    
    unsafe_op_in_unsafe_fn,
        
    overflowing_literals,
)]

#![deny(missing_docs,unused_results)]


mod structs;
mod enums;
mod client;
mod builder;

pub use self::structs::*;
pub use self::enums::*;
pub use self::client::*;
pub use self::builder::*;

#[cfg(feature ="marcos")]
mod macros;

#[cfg(feature ="marcos")]
pub use self::macros::*;