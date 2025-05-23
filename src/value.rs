use std::borrow::{Borrow, Cow, ToOwned};
use std::ops::Deref;
use std::rc::Rc;

use crate::{step, Cons};

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Default)]
pub enum Value<'v> {
    #[default]
    Nil,
    Symbol(Cow<'v, str>),
    Byte(u8),
    UInt(u64),
    Int(i64),
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
                Value::Byte(h) => format!("{}", h),
                Value::UInt(h) => format!("{}", h),
                Value::Int(h) => format!("{}", h),
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
                Value::Byte(h) => format!("{}", h),
                Value::UInt(h) => format!("{}", h),
                Value::Int(h) => format!("{}", h),
            }
        )
    }
}

impl<'v> From<u8> for Value<'v> {
    fn from(value: u8) -> Value<'v> {
        Value::Byte(value)
    }
}
impl<'v> From<u64> for Value<'v> {
    fn from(value: u64) -> Value<'v> {
        Value::UInt(value)
    }
}
impl<'v> From<i64> for Value<'v> {
    fn from(value: i64) -> Value<'v> {
        Value::Int(value)
    }
}
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
