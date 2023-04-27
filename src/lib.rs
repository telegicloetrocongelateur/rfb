#![feature(exclusive_range_pattern)]
#![feature(generic_const_exprs)]
#![feature(array_try_from_fn)]
pub mod client;
pub mod error;
pub mod io;
pub mod messages;
pub mod server;

pub use error::Error;
