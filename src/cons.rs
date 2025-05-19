use std::rc::Rc;

use crate::Value;
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Cons<'c> {
    Head(Value<'c>),
    Cell(Value<'c>, Rc<Cons<'c>>),
    #[default]
    Empty,
}

impl<'c> Cons<'c> {
    pub fn new(head: Value<'c>) -> Cons<'c> {
        match head {
            Value::Nil => Cons::Empty,
            _ => Cons::Head(head),
        }
    }

    pub fn nil() -> Cons<'c> {
        Cons::Empty
    }

    pub fn len(&self) -> usize {
        match self {
            Cons::Empty => 0,
            Cons::Head(_) => 1,
            Cons::Cell(_, tail) => 1 + tail.as_ref().len(),
        }
    }

    pub fn values(&self) -> Vec<Value> {
        let mut values = Vec::<Value>::new();
        match self {
            Cons::Empty => {},
            Cons::Head(head) => {
                values.push(head.clone());
            },
            Cons::Cell(head, tail) => {
                values.push(head.clone());
                values.extend(tail.as_ref().values());
            },
        }
        values
    }
}

impl<'c> From<Value<'c>> for Cons<'c> {
    fn from(head: Value<'c>) -> Cons<'c> {
        Cons::new(head)
    }
}
impl<'v> From<Cons<'v>> for Value<'v> {
    fn from(cons: Cons<'v>) -> Value<'v> {
        match cons {
            Cons::Empty => Value::Nil,
            Cons::Head(value) => value.clone(),
            Cons::Cell(head, tail) => {
                if !head.is_nil() {
                    return head;
                }
                let tail = tail.as_ref().clone();

                if let Cons::Cell(head, tail) = tail {
                    if !head.is_nil() {
                        return head;
                    } else {
                        let tail = tail.as_ref().clone();
                        match tail {
                            Cons::Empty => Value::Nil,
                            Cons::Head(value) => value.clone(),
                            Cons::Cell(head, tail) =>
                                if !head.is_nil() {
                                    return head;
                                } else {
                                    let tail = tail.as_ref().clone();
                                    return Value::from(tail);
                                },
                        }
                    }
                } else {
                    head
                }
            },
        }
    }
}

impl std::fmt::Display for Cons<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", {
            let values = self
                .values()
                .iter()
                .filter(|value| !value.is_nil())
                .map(|value| value.to_string())
                .collect::<Vec<String>>();
            if values.is_empty() {
                "()".to_string()
            } else if values.len() == 1 {
                values.join(" ")
            } else {
                format!("({})", values.join(" "))
            }
        })
    }
}
impl std::fmt::Debug for Cons<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", {
            let mut values = self.values();
            values.push(Value::Nil);

            let values = values
                .iter()
                .map(|value| format!("{:#?}", value))
                .collect::<Vec<String>>();
            if values.is_empty() {
                "(nil)".to_string()
            } else {
                format!("({})", values.join(" . "))
            }
        })
    }
}

pub fn cons<'c>(head: Cons<'c>, tail: Cons<'c>) -> Cons<'c> {
    Cons::Cell(head.into(), Rc::new(tail))
}

pub fn car<'c>(cons: Cons<'c>) -> Cons<'c> {
    Cons::Empty
}
pub fn cdr<'c>(cons: Cons<'c>) -> Cons<'c> {
    Cons::Empty
}

// #[cfg(test)]
// mod cons_tests {
//     use std::rc::Rc;

//     use k9::assert_equal;

//     use crate::*;

//     #[test]
//     fn from_value_nil() {
//         let cell = Cons::from(Value::Nil);
//         assert_equal!(cell, Cons::Empty);
//         assert_equal!(cell, Cons::nil());
//         assert_equal!(cell.len(), 0);
//         assert_debug_equal!(cell, "(nil)");
//         assert_display_equal!(cell, "()");
//     }
//     #[test]
//     fn from_value_symbol() {
//         let cell = Cons::from(Value::from("symbol"));
//         assert_equal!(cell, Cons::Head(Value::from("symbol")));
//         assert_equal!(cell.len(), 1);
//         assert_display_equal!(cell, "symbol");
//         assert_debug_equal!(cell, "('symbol . nil)");
//     }
//     #[test]
//     fn cons_function_simple_head_tail() {
//         let head = Cons::from(Value::from("head"));
//         let tail = Cons::from(Value::from("tail"));
//         let cell = Cons::Cell(Value::from("head"), Rc::new(tail.clone()));
//         assert_equal!(cons(head, tail), cell);
//         assert_equal!(cell.len(), 2);
//         assert_display_equal!(cell, "(head tail)");
//         assert_debug_equal!(cell, "('head . 'tail . nil)");
//     }
//     #[test]
//     fn cons_function_head_tail_with_tail() {
//         let head = Cons::from(Value::from("head"));
//         let tail = Cons::Cell(Value::from("cell"), Rc::new(Cons::from(Value::from("tail"))));
//         let cell = Cons::Cell(
//             Value::from("head"),
//             Rc::new(Cons::Cell(Value::from("cell"), Rc::new(Cons::from(Value::from("tail"))))),
//         );
//         assert_equal!(cell.len(), 3);
//         assert_equal!(cons(head, tail), cell);
//         assert_display_equal!(cell, "(head cell tail)");
//         assert_debug_equal!(cell, "('head . 'cell . 'tail . nil)");
//     }
// }
