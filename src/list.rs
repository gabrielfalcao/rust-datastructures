use std::rc::Rc;

use crate::ListValue;

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum List<T: ListValue> {
    Head(Option<T>),
    Tail((T, Rc<List<T>>)),
}

impl<T: ListValue> std::fmt::Debug for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "({})",
            match self {
                List::Head(Some(head)) => format!("{:#?}", head),
                List::Head(None) => String::new(),
                List::Tail((head, tail)) => match Rc::into_inner(tail.into()) {
                    Some(tail) => format!("{:#?}, {:#?}", head, tail),
                    None => format!("{:#?}, nil", head),
                },
            }
        )
    }
}
impl<T: ListValue> List<T> {
    pub fn new(head: T) -> List<T> {
        List::Head(Some(head))
    }

    pub fn add(&self, value: T) -> List<T> {
        match self {
            List::Head(head) => match head {
                Some(head) => List::Tail((head.clone(), Rc::new(List::new(value)))),
                None => List::new(value),
            },
            List::Tail((head, tail)) => List::Tail((head.clone(), List::add(tail, value).into())),
        }
    }
    pub fn append(&self, list: List<T>) -> List<T> {
        match self {
            List::Head(head) => match head {
                Some(head) => List::Tail((head.clone(), Rc::new(list))),
                None => list.clone(),
            },
            List::Tail((head, tail)) => List::Tail((
                head.clone(),
                Rc::new(match Rc::into_inner(tail.into()) {
                    Some(tail) => tail.append(list),
                    None => list,
                }),
            )),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            List::Head(_) => 1,
            List::Tail((_, tail)) => 1 + tail.len(),
        }
    }
}

pub fn car<T: ListValue>(list: List<T>) -> Option<T> {
    match list {
        List::Head(head) => head,
        List::Tail((head, _)) => Some(head),
    }
}
pub fn cdr<T: ListValue>(list: List<T>) -> Option<T> {
    match list {
        List::Head(head) => head,
        List::Tail((_, tail)) => match Rc::into_inner(tail) {
            Some(list) => cdr(list),
            None => None,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_new() {
        let result = List::new(1);
        assert_eq!(result, List::Head(Some(1)));
    }

    #[test]
    fn list_add() {
        let head = List::new(1);
        let result = head.add(2);
        assert_eq!(result, List::Tail((1, Rc::new(List::Head(Some(2))))));
    }

    #[test]
    fn list_car() {
        let result = List::new(1).add(2);
        assert_eq!(result, List::Tail((1, Rc::new(List::Head(Some(2))))));
        assert_eq!(car(result), Some(1));
    }

    #[test]
    fn list_cdr() {
        let result = List::new(1).add(2);
        assert_eq!(result, List::Tail((1, Rc::new(List::Head(Some(2))))));
        assert_eq!(cdr(result), Some(2));
    }

    #[test]
    fn list_append() {
        let head = List::new(1);
        let tail = List::new(2).add(3);
        let result = head.append(tail);
        assert_eq!(
            result,
            List::Tail((1, Rc::new(List::Tail((2, Rc::new(List::Head(Some(3))))))))
        );
        let result = head.append(List::new(2));
        assert_eq!(result, List::Tail((1, Rc::new(List::Head(Some(2))))));
    }
    #[test]
    fn list_len() {
        let head = List::new(1);
        assert_eq!(head.len(), 1);
        let tail = List::new(2).add(3);
        assert_eq!(tail.len(), 2);
        let result = head.append(tail);
        assert_eq!(result.len(), 3);
    }
    // #[test]
    // fn list_make() {
    //     let list = List::make(5, 2);
    //     assert_eq!(list.len(), 5);
    // }
}

// #[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
// pub enum Value {
//     String(String),
//     Integer(i64),
//     Byte(u8),
//     Unsigned(u64),
// }
