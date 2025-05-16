use std::borrow::{Borrow, Cow, ToOwned};
use std::ops::Deref;
use std::rc::Rc;

use crate::{ListValue, Cell, cons};

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Debug, Default)]
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

#[cfg(test)]
mod value_tests {
    use k9::assert_equal;

    use super::*;

    #[test]
    fn value_from_static_str() {
        let value = "static-str";
        assert_equal!(Value::from(value).to_string(), "'static-str");
    }
    #[test]
    fn value_from_str() {
        let value = "str".to_string().leak();
        assert_equal!(Value::from(value).to_string(), "'str");
    }
    #[test]
    fn value_from_string() {
        let value = "string".to_string();
        assert_equal!(Value::from(value).to_string(), "'string");
    }
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
impl std::fmt::Display for Value<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::Nil => "nil".to_string(),
                Value::Symbol(h) => format!("'{}", h),
                Value::Cell(c) => c.to_string(),
            }
        )
    }
}
