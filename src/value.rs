use std::borrow::{Borrow, Cow, ToOwned};
use std::ops::Deref;
use std::rc::Rc;

use crate::{color, step};

#[derive(Clone, PartialOrd, Ord, Default, PartialEq, Eq)]
pub enum Value<'c> {
    #[default]
    Nil,
    String(Cow<'c, str>),
    Byte(u8),
    UInt(u64),
    Int(i64),
}
impl<'c> Value<'_> {
    pub fn nil() -> Value<'c> {
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

impl<'c> Drop for Value<'c> {
    fn drop(&mut self) {}
}

impl std::fmt::Display for Value<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::Nil => "nil".to_string(),
                Value::String(h) => format!("{}", h),
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
                Value::String(h) => format!("{:#?}", h),
                Value::Byte(h) => format!("{}u8", h),
                Value::UInt(h) => format!("{}u64", h),
                Value::Int(h) => format!("{}i64", h),
            }
        )
    }
}

impl<'c> From<u8> for Value<'c> {
    fn from(value: u8) -> Value<'c> {
        Value::Byte(value)
    }
}
impl<'c> From<u64> for Value<'c> {
    fn from(value: u64) -> Value<'c> {
        Value::UInt(value)
    }
}
impl<'c> From<i64> for Value<'c> {
    fn from(value: i64) -> Value<'c> {
        Value::Int(value)
    }
}
impl<'c> From<&'c str> for Value<'c> {
    fn from(value: &'c str) -> Value<'c> {
        Value::String(Cow::from(value))
    }
}
impl<'c> From<Cow<'c, str>> for Value<'c> {
    fn from(value: Cow<'c, str>) -> Value<'c> {
        Value::from(value.into_owned())
    }
}
impl<'c> From<&'c mut str> for Value<'c> {
    fn from(value: &'c mut str) -> Value<'c> {
        Value::String(Cow::<'c, str>::Borrowed(&*value))
    }
}
impl<'c> From<String> for Value<'c> {
    fn from(value: String) -> Value<'c> {
        Value::String(Cow::from(value))
    }
}
impl<'c> From<Option<String>> for Value<'c> {
    fn from(value: Option<String>) -> Value<'c> {
        match value {
            None => Value::Nil,
            Some(value) => Value::from(value),
        }
    }
}

impl<'c> PartialEq<&Value<'c>> for Value<'c> {
    fn eq(&self, other: &&Value<'c>) -> bool {
        let other = unsafe { &**other };
        self == other
    }
}

impl<'c> PartialEq<&mut Value<'c>> for Value<'c> {
    fn eq(&self, other: &&mut Value<'c>) -> bool {
        let other = unsafe { &**other };
        self == other
    }
}
