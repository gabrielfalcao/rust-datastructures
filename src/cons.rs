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

pub fn cons<'c, H: Value + 'c>(head: H, tail: &mut Cell<'c, H>) -> Cell<'c, H> {
    let mut head = Cell::new(head.into());
    head.add(tail);
    head
}
pub fn cdr<'c, H: Value + 'c>(cell: &Cell<'c, H>) -> Cell<'c, H> {
    if let Some(tail) = cell.tail() {
        tail.clone()
    } else {
        Cell::nil()
    }
}
pub fn car<'c, H: Value + 'c>(cell: &Cell<'c, H>) -> H {
    if let Some(head) = cell.head() {
        head
    } else {
        Value::nil()
    }
}
