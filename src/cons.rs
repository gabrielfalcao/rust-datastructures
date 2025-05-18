use std::rc::Rc;

use crate::Value;
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Default, Debug)]
pub enum Cons<'c> {
    Cell(Value<'c>, Rc<Option<Cons<'c>>>),
    #[default]
    Empty,
}

impl<'c> Cons<'c> {
    pub fn new(head: Value<'c>) -> Cons<'c> {
        match head {
            Value::Nil => Cons::Empty,
            _ => Cons::Cell(head, Rc::new(None)),
        }
    }

    pub fn nil() -> Cons<'c> {
        Cons::Empty
    }

    pub fn len(&self) -> usize {
        match self {
            Cons::Empty => 0,
            Cons::Cell(_, tail) =>
                1 + if let Some(tail) = tail.as_ref().clone() { tail.len() } else { 0 },
        }
    }

    pub fn head(&self) -> Cons<'c> {
        let head: Value<'c> = match self {
            Cons::Empty => Value::Nil,
            Cons::Cell(head, tail) => {
                // dbg!(&head, &tail);
                match head {
                    Value::Nil => {
                        let mut tail = tail.as_ref().clone().unwrap_or_default();
                        match tail {
                            Cons::Empty => Value::Nil,
                            Cons::Cell(head, tail) => {
                                dbg!(&head, &tail);
                                if !head.is_nil() {
                                    head
                                } else {
                                    let mut tail = tail.as_ref().clone().unwrap_or_default();
                                    match tail {
                                        Cons::Empty => Value::Nil,
                                        Cons::Cell(head, tail) => {
                                            dbg!(&head, &tail);
                                            head
                                        },
                                    }
                                }
                            },
                        }
                    },
                    head => head.clone(),
                }
            },
        };
        let tail = match self {
            Cons::Empty => Cons::Empty,
            Cons::Cell(_, tail) => {
                // dbg!(&tail);
                let tail = tail.as_ref().clone().unwrap_or_default();
                match tail {
                    Cons::Empty => Cons::Empty,
                    Cons::Cell(_, tail) => {
                        dbg!(&tail);
                        let tail = tail.as_ref().clone().unwrap_or_default();
                        match tail {
                            Cons::Empty => Cons::Empty,
                            _ => tail.head(),
                        }
                    },
                }
            },
        };
        match head {
            Value::Nil => {
                dbg!(&tail);
                Cons::Empty
            },
            head => {
                dbg!(&head, &tail);
                match tail {
                    Cons::Empty => return Cons::Cell(head, Rc::new(None)),
                    ref cell => {
                        if let Cons::Cell(value, _) = cell {
                            if head == *value {
                                return cell.clone()
                            }
                        }
                    },
                }
                Cons::Cell(head.clone(), Rc::new(Some(tail)))
            },
        }
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
            Cons::Cell(head, tail) => {
                if !head.is_nil() {
                    return head;
                }
                let tail = tail.as_ref().clone().unwrap_or_default();

                if let Cons::Cell(head, tail) = tail {
                    if !head.is_nil() {
                        return head;
                    } else {
                        let tail = tail.as_ref().clone().unwrap_or_default();
                        match tail {
                            Cons::Empty => Value::Nil,
                            Cons::Cell(head, tail) =>
                                if !head.is_nil() {
                                    return head;
                                } else {
                                    let tail = tail.as_ref().clone().unwrap_or_default();
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
            match self {
                Cons::Empty => "()".to_string(),
                Cons::Cell(head, tail) => {
                    let mut expressions = Vec::<String>::new();
                    if !head.is_nil() {
                        expressions.push(head.to_string());
                    }
                    if let Some(tail) = tail.as_ref().clone() {
                        if tail.len() > 0 {
                            expressions.push(tail.to_string());
                        }
                    }
                    let wrap = expressions.len() > 1;
                    let expressions = expressions.join(" ");
                    if wrap {
                        format!("({})", expressions)
                    } else {
                        expressions
                    }
                },
            }
        })
    }
}
// impl std::fmt::Debug for Cons<'_> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "{}", {
//             match self {
//                 Cons::Empty => "nil".to_string(),
//                 Cons::Cell(head, tail) => {
//                     let mut expressions = Vec::<String>::new();
//                     if !head.is_nil() {
//                         expressions.push(head.to_string());
//                     }
//                     if let Some(tail) = tail.as_ref().clone() {
//                         if tail.len() > 0 {
//                             expressions.push(tail.to_string());
//                         }
//                     }
//                     format!("({})", expressions.join(" . "))
//                 },
//             }
//         })
//     }
// }

pub fn cons<'c>(cell: Cons<'c>, tail: Cons<'c>) -> Cons<'c> {
    match tail.head() {
        Cons::Empty => cell.head(),
        cell => cell.head(),
    }
}

pub fn car<'c>(cons: Cons<'c>) -> Cons<'c> {
    Cons::Empty
}
pub fn cdr<'c>(cons: Cons<'c>) -> Cons<'c> {
    Cons::Empty
}

#[cfg(test)]
mod cons_tests {
    use std::rc::Rc;

    use k9::assert_equal;

    use crate::*;

    #[test]
    fn from_value_nil() {
        let cell = Cons::from(Value::Nil);
        assert_equal!(cell, Cons::Empty);
        assert_equal!(cell, Cons::nil());
        assert_equal!(cell.len(), 0);
        //assert_debug_equal!(cell, "nil");
        assert_display_equal!(cell, "()");
    }
    #[test]
    fn from_value_symbol() {
        let cell = Cons::from(Value::from("symbol"));
        assert_equal!(cell, Cons::Cell(Value::from("symbol"), Default::default()));
        assert_equal!(cell.len(), 1);
        assert_display_equal!(cell, "symbol");
        //assert_debug_equal!(cell, "(symbol)");
    }
    #[test]
    fn cons_head() {
        assert_equal!(
            Cons::Cell(Value::from("head"), Rc::new(None)).head(),
            Cons::Cell(Value::from("head"), Rc::new(None))
        );
        assert_equal!(
            Cons::Cell(Value::Nil, Rc::new(Some(Cons::Cell(Value::from("head"), Rc::new(None)))))
                .head(),
            Cons::Cell(Value::from("head"), Rc::new(None))
        );
        assert_equal!(
            Cons::Cell(
                Value::Nil,
                Rc::new(Some(Cons::Cell(
                    Value::Nil,
                    Rc::new(Some(Cons::Cell(
                        Value::from("head"),
                        Rc::new(Some(Cons::Cell(
                            Value::Nil,
                            Rc::new(Some(Cons::Cell(Value::from("tail"), Rc::new(None))))
                        )))
                    )))
                )))
            )
            .head(),
            Cons::Cell(
                Value::from("head"),
                Rc::new(Some(Cons::Cell(Value::from("tail"), Rc::new(None))))
            )
        );
        assert_equal!(
            Cons::Cell(
                Value::from("head"),
                Rc::new(Some(Cons::Cell(Value::from("tail"), Rc::new(None))))
            )
            .head(),
            Cons::Cell(
                Value::from("head"),
                Rc::new(Some(Cons::Cell(Value::from("tail"), Rc::new(None))))
            )
        );
    }
    // #[test]
    // fn from_head_and_tail_with_head_symbol_tail_nil() {
    //     let cell = Cons::Cell(
    //         Value::from("head"),
    //         Rc::new(Some(Cons::Cell(Value::from("tail"), Default::default()))),
    //     );
    //     assert_equal!(cons(Cons::from(Value::from("head")), Cons::from(Value::from("tail"))), cell);
    //     assert_equal!(cell.len(), 2);
    //     assert_display_equal!(cell, "(head tail)");
    //     //assert_debug_equal!(cell, "(head . tail)");
    // }
    // #[test]
    // fn from_cons_debug_head_and_tail_with_head_nil_tail_head_symbol() {
    //     let cell = Cons::Cell(Value::from("head"), Rc::new(Some(Cons::from(Value::from("tail")))));
    //     assert_equal!(
    //         cons(
    //             Cons::Cell(Value::from("head"), Default::default()),
    //             Cons::Cell(Value::Nil, Rc::new(Some(Cons::from(Value::from("tail"))))).into()
    //         ),
    //         cell
    //     );
    //     assert_display_equal!(
    //         cons(
    //             Cons::new(Value::from("head")),
    //             Cons::Cell(Value::Nil, Rc::new(Some(Cons::from(Value::from("tail"))))).into()
    //         ),
    //         cell
    //     );
    //     assert_display_equal!(cell, "(head tail)");
    //     //assert_debug_equal!(cell, "(head . tail)");
    // }
    // #[test]
    // fn from_cons_head_and_tail_with_head_nil_tail_head_symbol() {
    //     let cell = cons(
    //         Cons::Cell(
    //             Value::Nil,
    //             Rc::new(Some(Cons::Cell(
    //                 Value::Nil,
    //                 Rc::new(Some(Cons::from(Value::from("head")))),
    //             ))),
    //         ),
    //         Cons::Cell(
    //             Value::Nil,
    //             Rc::new(Some(Cons::Cell(
    //                 Value::Nil,
    //                 Rc::new(Some(Cons::from(Value::from("tail")))),
    //             ))),
    //         ),
    //     );

    //     assert_equal!(
    //         cell.clone(),
    //         cons(
    //             Cons::Empty,
    //             Cons::Cell(Value::from("head"), Rc::new(Some(Cons::from(Value::from("tail")))))
    //                 .into()
    //         )
    //     );
    //     assert_display_equal!(
    //         cons(
    //             Cons::new(Value::from("head")),
    //             Cons::Cell(Value::Nil, Rc::new(Some(Cons::from(Value::from("tail"))))).into()
    //         ),
    //         cell
    //     );
    //     // assert_display_equal!(cell, "(head tail)");
    //     //assert_debug_equal!(cell, "(head . tail)");
    // }
}
