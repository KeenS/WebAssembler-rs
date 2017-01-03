mod util;
mod module;
mod types;
mod ops;
pub mod builder;

pub use types::*;
pub use module::*;
pub use ops::*;

pub trait Dump {
    fn dump(&self, buf: &mut Vec<u8>) -> usize;
}
