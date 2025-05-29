#![allow(unused)]
pub mod traits;
pub use traits::ListValue;
pub mod cons;
pub use cons::{car, cdr, cons};
pub mod cell;
pub use cell::Cell;
pub mod value;
pub use value::Value;
pub mod node;
pub use node::{Node, subtree_delete};
pub mod color;
pub mod macros;
pub(crate) mod internal;
pub mod test;
