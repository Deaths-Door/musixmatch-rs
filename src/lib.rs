#![doc = include_str!("../README.md")]

#![forbid(    
    unsafe_code,
        
    unused_imports,
    unused_variables,
    unused_mut,
    unused_results,
    unused_allocation,
    unused_must_use,
    
    unreachable_patterns,
    
    trivial_casts,
    
    unsafe_op_in_unsafe_fn,
        
    overflowing_literals,
)]

#![deny(missing_docs)]


mod structs;
mod enums;
mod client;

pub use self::structs::*;
pub use self::enums::*;
pub use self::client::*;
