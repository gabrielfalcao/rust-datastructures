use std::rc::Rc;

pub trait ListValue:
    Sized + PartialOrd + Ord + PartialEq + Eq + Clone + std::fmt::Display + std::fmt::Debug
{
}
impl<
        T: Sized + PartialOrd + Ord + PartialEq + Eq + Clone + std::fmt::Display + std::fmt::Debug,
    > ListValue for T
{
}
