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
// impl<'c> PartialEq<&Value<'c>> for Value<'c> {
//     fn eq(&self, other: &&Value<'c>) -> bool {
//         match *other {
//             Value::Nil => *self == &Value::Nil,
//             #[rustfmt::skip]
//             Value::Byte(other) => if let Value::Byte(byte) = self {*byte == *other} else {false},
//             #[rustfmt::skip]
//             Value::Int(other) => if let Value::Int(int) = self {*int == *other} else {false},
//             #[rustfmt::skip]
//             Value::UInt(other) => if let Value::UInt(uint) = self {*uint == *other} else {false},
//             #[rustfmt::skip]
//             Value::String(other) => if let Value::String(uint) = self {*uint == *other} else {false},
//         }
//     }
// }
// impl<'c> Eq<&Value<'c>> for Value<'c> {}

impl<'c> Drop for Value<'c> {
    fn drop(&mut self) {
        // eprintln!(
        //     "{}",
        //     color::reset(color::bgfg(
        //         format!(
        //             "{}{} {}{}: {}",
        //             crate::color::fg("dropping ", 237),
        //             crate::color::fg("value", 136),
        //             color::bgfg(format!(" @ "), 231, 16),
        //             color::ptr_inv(self),
        //             color::fore(format!("{:#?}", self), 201),
        //         ),
        //         197,
        //         16,
        //     ))
        // );
        // eprintln!(
        //     "{}",
        //     color::reset(color::fg(
        //         format!(
        //             "\n{}\n{}{} {}{}: {}{}\n{}",
        //             crate::color::reset(crate::color::bg(" ".repeat(80), color::wrap(197).into())),
        //             crate::color::bgfg("DROPPING ", 16, 197),
        //             crate::color::fg("VALUE", 16),
        //             color::bgfg(format!(" @ "), 16, 231),
        //             color::ptr_inv(self),
        //             color::fore(self.to_string(), 201),
        //             crate::color::reset(crate::color::bg(" ".repeat(39), color::wrap(197).into())),
        //             crate::color::reset(crate::color::bg(" ".repeat(80), color::wrap(197).into())),
        //         ),
        //         237
        //     ))
        // )
    }
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
// impl <'c>AsRef<Value<'c>> for Value<'c>
// {
//     fn as_ref(&self) -> &'c Value<'c> {
//         &self.clone()
//     }
// }
