use std::rc::Rc;

use crate::{ListValue, Nil, Head, Cell, Value};


#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct Cons<T: ListValue> {
    pub head: T,
    pub tail: Value<T>,
}

impl<T> Cons<T>
where
    T: ListValue,
{
    fn new(head: T, tail: Value<T>) -> Cons<T> {
        Cons { head, tail }
    }
}

pub fn cons<T: ListValue>(head: T, tail: Value<T>) -> Cons<T> {
    Cons::new(head, tail)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cons_head_nil_tail() {
        let result = cons("a".to_string(), Nil);
        assert_eq!(
            result,
            Cons {
                head: "a".to_string(),
                tail: Nil
            }
        );
    }
    #[test]
    fn cons_head_and_tail() {
        let result = cons("a".to_string(), Head("Z".to_string()));
        assert_eq!(
            result,
            Cons {
                head: "a".to_string(),
                tail: Head("Z".to_string())
            }
        );
    }
}
