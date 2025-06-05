use std::fmt::{Debug, Display};

pub trait Value: Sized + PartialOrd + PartialEq + Clone + Debug + Display + Default {
    fn nil() -> Self {
        Default::default()
    }
}
impl<T: Sized + PartialOrd + PartialEq + Clone + Debug + Display + Default> Value for T {}
