use std::borrow::{Borrow, Cow, ToOwned};
use std::ops::Deref;
use std::rc::Rc;

use crate::{color, step, Cons};

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Default)]
pub enum Value<'v> {
    #[default]
    Nil,
    String(Cow<'v, str>),
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
impl<'v> Drop for Value<'v> {
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
        Value::String(Cow::from(value))
    }
}
impl<'v> From<Cow<'v, str>> for Value<'v> {
    fn from(value: Cow<'v, str>) -> Value<'v> {
        Value::from(value.into_owned())
    }
}
impl<'v> From<&'v mut str> for Value<'v> {
    fn from(value: &'v mut str) -> Value<'v> {
        Value::String(Cow::<'v, str>::Borrowed(&*value))
    }
}
impl<'v> From<String> for Value<'v> {
    fn from(value: String) -> Value<'v> {
        Value::String(Cow::from(value))
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
// impl <'c>AsRef<Value<'c>> for Value<'c>
// {
//     fn as_ref(&self) -> &'c Value<'c> {
//         &self.clone()
//     }
// }
