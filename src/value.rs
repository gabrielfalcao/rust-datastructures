use std::rc::Rc;
use crate::{ListValue, Cons};


#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Debug, Default)]
pub enum Value<T: ListValue> {
    Head(T),
    Cell(Rc<Cons<T>>),
    #[default]
    Nil,
}
