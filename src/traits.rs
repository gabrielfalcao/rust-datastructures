use std::rc::Rc;

pub trait ListValue:
    Sized + PartialOrd + Ord + PartialEq + Eq + Clone + std::fmt::Debug + std::fmt::Display
{
}
impl<
        T: Sized + PartialOrd + Ord + PartialEq + Eq + Clone + std::fmt::Debug + std::fmt::Display,
    > ListValue for T
{
}
