use std::borrow::{Borrow, Cow, ToOwned};
use std::ops::Deref;
use std::rc::Rc;

use crate::{cons, Cell, ListValue};

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Default)]
pub enum Value<'v> {
    #[default]
    Nil,
    Symbol(Cow<'v, str>),
    Cell(Rc<Cell<'v>>),
}
impl<'v> Value<'_> {
    pub fn nil() -> Value<'v> {
        Value::Nil
    }

    pub fn is_nil(&self) -> bool {
        if let Value::Cell(cell) = self {
            cell.as_ref().is_nil()
        } else if *self == Value::Nil {
            true
        } else {
            false
        }
    }
}
impl std::fmt::Display for Value<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::Nil => "nil".to_string(),
                Value::Symbol(h) => format!("{}", h),
                Value::Cell(c) => c.to_string(),
            }
        )
    }
}
impl std::fmt::Debug for Value<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::Nil => "nil".to_string(),
                Value::Symbol(h) => format!("{}", h),
                Value::Cell(c) => format!("{}", c),
            }
        )
    }
}

#[cfg(test)]
mod value_tests {
    use std::rc::Rc;

    use k9::assert_equal;

    use crate::*;

    #[test]
    fn value_from_static_str() {
        let value = "static-str";
        assert_equal!(Value::from(value).to_string(), "static-str");
        let value = "static-str";
        assert_display_equal!(Value::from(value), "static-str");
        let value = "static-str";
        assert_debug_equal!(Value::from(value), "static-str");
    }
    #[test]
    fn value_from_str() {
        let value = "str".to_string().leak();
        assert_equal!(Value::from(value).to_string(), "str");
        let value = "str".to_string().leak();
        assert_display_equal!(Value::from(value), "str");
        let value = "str".to_string().leak();
        assert_debug_equal!(Value::from(value), "str");
    }
    #[test]
    fn value_from_string() {
        let value = "string".to_string();
        assert_equal!(Value::from(value).to_string(), "string");
        let value = "string".to_string();
        assert_display_equal!(Value::from(value), "string");
        let value = "string".to_string();
        assert_debug_equal!(Value::from(value), "string");
    }
    #[test]
    fn value_display_nil() {
        assert_display_equal!(Value::Nil, "nil");
    }
    #[test]
    fn value_debug_nil() {
        assert_debug_equal!(Value::Nil, "nil");
    }
    #[test]
    fn value_display_cell_nil() {
        assert_display_equal!(Value::from(Cell::nil()), "nil");
    }
    #[test]
    fn value_debug_cell_nil() {
        assert_debug_equal!(Value::from(Cell::nil()), "nil");
    }
    #[test]
    fn value_display_cell_head_symbol_tail_nil() {
        assert_display_equal!(
            Value::from(Cell {
                head: Value::from("head"),
                tail: None
            }),
            "head"
        );
        assert_display_equal!(
            Value::from(Cell {
                head: Value::from("head"),
                tail: Some(Rc::new(Cell {
                    head: Value::Nil,
                    tail: None
                }))
            }),
            "head"
        );
    }
    // #[test]
    // fn value_display_cell_head_symbol_tail_symbol() {
    //     assert_display_equal!(
    //         Value::from(Cell {
    //             head: Value::from("head"),
    //             tail: Some(Rc::new(Cell {
    //                 head: Value::from("tail"),
    //                 tail: None
    //             }))
    //         }),
    //         "head tail"
    //     );

    // }
    // #[test]
    // fn value_debug_cell_head_symbol_tail_symbol() {
    //     assert_equal!(cons("head", Some(Cell::from(Value::from("tail"))))), "(head . tail)");
    //     assert_display_equal!(Value::from(cons("head", Some(Cell::from(Value::from("tail"))))), "(head . tail)");
    // }
}

impl<'v> From<&'v str> for Value<'v> {
    fn from(value: &'v str) -> Value<'v> {
        Value::Symbol(Cow::from(value))
    }
}
impl<'v> From<&'v mut str> for Value<'v> {
    fn from(value: &'v mut str) -> Value<'v> {
        Value::Symbol(Cow::<'v, str>::Borrowed(&*value))
    }
}
impl<'v> From<String> for Value<'v> {
    fn from(value: String) -> Value<'v> {
        Value::Symbol(Cow::from(value))
    }
}
impl<'v, 'c> From<Cell<'c>> for Value<'v> {
    fn from(cell: Cell<'c>) -> Value<'v> {
        match cell.head {
            Value::Symbol(h) => Value::from(h.into_owned()),
            Value::Cell(cell) => {
                let cell = cell.as_ref().clone();
                if !cell.head.is_nil() && cell.tail().is_nil() {
                    match cell.tail() {
                        Value::Symbol(h) => Value::from(h.into_owned()),
                        Value::Nil => Value::Nil,
                        Value::Cell(cell) => {
                            let cell = cell.as_ref().clone();
                            Value::from(cell)
                        },
                    }
                } else if !cell.tail().is_nil() {
                    Value::from(cell.tail())
                } else {
                    Value::Nil
                }
            },
            Value::Nil => Value::Nil,
        }
    }
}
