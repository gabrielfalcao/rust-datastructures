use std::rc::Rc;

use crate::{step, Cell, Value};

#[macro_export]
macro_rules! list {
    ($( $item:literal ),* ) => {{
        let mut cell = Cell::nil();
        $(cell.add(&mut Cell::from($item));
        )*
        cell
    }};
}

pub fn cons<'c, H: Into<Value<'c>>>(head: H, tail: &mut Cell<'c>) -> Cell<'c> {
    let mut head = Cell::new(head.into());
    head.add(tail);
    head
}
pub fn cdr<'c>(cell: &Cell<'c>) -> Cell<'c> {
    if let Some(tail) = cell.tail() {
        tail.clone()
    } else {
        Cell::nil()
    }
}
pub fn car<'c>(cell: &Cell<'c>) -> Value<'c> {
    if let Some(head) = cell.head() {
        head
    } else {
        Value::nil()
    }
}
