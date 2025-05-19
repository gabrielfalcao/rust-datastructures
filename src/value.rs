use std::borrow::{Borrow, Cow, ToOwned};
use std::ops::Deref;
use std::rc::Rc;

use crate::Cons;

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Default)]
pub enum Value<'v> {
    #[default]
    Nil,
    Symbol(Cow<'v, str>),
}
impl<'v> Value<'_> {
    pub fn nil() -> Value<'v> {
        Value::Nil
    }

    pub fn is_nil(&self) -> bool {
        if *self == Value::Nil {
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
                Value::Symbol(h) => format!("'{}", h),
            }
        )
    }
}

#[cfg(test)]
// mod value_tests {
//     use std::rc::Rc;

//     use k9::assert_equal;

//     use crate::*;

//     #[test]
//     fn value_from_static_str() {
//         let value = "static-str";
//         assert_equal!(Value::from(value).to_string(), "static-str");
//         let value = "static-str";
//         assert_display_equal!(Value::from(value), "static-str");
//         let value = "static-str";
//         assert_debug_equal!(Value::from(value), "'static-str");
//     }
//     #[test]
//     fn value_from_str() {
//         let value = "str".to_string().leak();
//         assert_equal!(Value::from(value).to_string(), "str");
//         let value = "str".to_string().leak();
//         assert_display_equal!(Value::from(value), "str");
//         let value = "str".to_string().leak();
//         assert_debug_equal!(Value::from(value), "'str");
//     }
//     #[test]
//     fn value_from_string() {
//         let value = "string".to_string();
//         assert_equal!(Value::from(value).to_string(), "string");
//         let value = "string".to_string();
//         assert_display_equal!(Value::from(value), "string");
//         let value = "string".to_string();
//         assert_debug_equal!(Value::from(value), "'string");
//     }
//     #[test]
//     fn value_display_nil() {
//         assert_display_equal!(Value::Nil, "nil");
//     }
//     #[test]
//     fn value_debug_nil() {
//         assert_debug_equal!(Value::Nil, "nil");
//     }
// }

impl<'v> From<&'v str> for Value<'v> {
    fn from(value: &'v str) -> Value<'v> {
        Value::Symbol(Cow::from(value))
    }
}
impl<'v> From<Cow<'v, str>> for Value<'v> {
    fn from(value: Cow<'v, str>) -> Value<'v> {
        Value::from(value.into_owned())
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
impl<'v> From<Option<String>> for Value<'v> {
    fn from(value: Option<String>) -> Value<'v> {
        match value {
            None => Value::Nil,
            Some(value) => Value::from(value),
        }
    }
}
