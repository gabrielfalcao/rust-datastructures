use std::borrow::{Borrow, Cow, ToOwned};
use std::ops::Deref;
use std::rc::Rc;

use crate::{car, cdr, cons, ListValue, Value};

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Default)]
pub struct Cell<'c> {
    pub head: Value<'c>,
    pub tail: Option<Rc<Cell<'c>>>,
}
impl<'c> Cell<'_> {
    pub fn nil() -> Cell<'c> {
        Cell::from(Value::Nil)
    }

    pub fn is_empty(&self) -> bool {
        self.len() > 0
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        if !self.head.is_nil() {
            len += 1
        }
        if !self.tail.clone().map(|rc| rc.as_ref().is_nil()).unwrap_or_else(|| false) {
            len += 1;
        }
        len
    }

    pub fn tail(&self) -> Value<'c> {
        match self.tail.clone().map(|rc| rc.as_ref().clone()) {
            Some(cell) => match cell.head.clone() {
                Value::Symbol(h) => Value::from(h.into_owned()),
                Value::Cell(cell) => {
                    let cell = cell.as_ref().clone();
                    Value::from(cell)
                },
                Value::Nil => Value::Nil,
            },
            None => Value::Nil,
        }
    }

    pub fn is_nil(&self) -> bool {
        self.head.is_nil() && self.tail().is_nil()
    }

    pub fn split_string(&self) -> [Option<String>; 2] {
        let head: Option<String> =
            if self.head.is_nil() { None } else { Some(format!("{}", self.head)) };
        let tail: Option<String> = match &self.tail {
            Some(cell) => {
                let cell = cell.as_ref().clone();
                let tail = match cell.head {
                    Value::Symbol(head) => Some(format!("{}", head)),
                    Value::Cell(cell) => {
                        let head = cell.tail.clone().map(|cell| cell.as_ref().clone());
                        match head {
                            Some(concept) => match concept.head {
                                Value::Nil => None,
                                Value::Symbol(r) => Some(format!("{}", r)),
                                Value::Cell(cell) => {
                                    let cell = cell.as_ref().clone();
                                    let parts = cell.split_string();
                                    if let Some(string) = parts[0].clone() {
                                        Some(string)
                                    } else if let Some(string) = parts[1].clone() {
                                        Some(string)
                                    } else {
                                        None
                                    }
                                },
                            },
                            None => {
                                let parts = cell.split_string();
                                if let Some(string) = parts[0].clone() {
                                    Some(string)
                                } else if let Some(string) = parts[1].clone() {
                                    Some(string)
                                } else {
                                    None
                                }
                            },
                        }
                    },
                    Value::Nil => {
                        let head = cell.tail.clone().map(|cell| cell.as_ref().clone());
                        match head {
                            Some(concept) => match concept.head {
                                Value::Nil => None,
                                Value::Symbol(r) => Some(format!("{}", r)),
                                Value::Cell(cell) => {
                                    let cell = cell.as_ref().clone();
                                    let parts = cell.split_string();
                                    if let Some(string) = parts[0].clone() {
                                        Some(string)
                                    } else if let Some(string) = parts[1].clone() {
                                        Some(string)
                                    } else {
                                        None
                                    }
                                },
                            },
                            None => {
                                let parts = cell.split_string();
                                if let Some(string) = parts[0].clone() {
                                    Some(string)
                                } else if let Some(string) = parts[1].clone() {
                                    Some(string)
                                } else {
                                    None
                                }
                            },
                        }
                    },
                };
                match tail {
                    Some(string) => Some(string),
                    None => {
                        let head = cell.tail.clone().map(|cell| cell.as_ref().clone());
                        match head {
                            Some(concept) => match concept.head {
                                Value::Nil => None,
                                Value::Symbol(r) => Some(format!("{}", r)),
                                Value::Cell(cell) => {
                                    let cell = cell.as_ref().clone();
                                    let parts = cell.split_string();
                                    if let Some(string) = parts[0].clone() {
                                        Some(string)
                                    } else if let Some(string) = parts[1].clone() {
                                        Some(string)
                                    } else {
                                        None
                                    }
                                },
                            },
                            None => None,
                        }
                    },
                }
            },
            None => None,
        };
        [head, tail]
    }
}
impl<'c> From<Value<'c>> for Cell<'c> {
    fn from(head: Value<'c>) -> Cell<'c> {
        Cell { head, tail: None }
    }
}

impl<'c> From<Option<Cell<'c>>> for Cell<'c> {
    fn from(cell: Option<Cell<'c>>) -> Cell<'c> {
        match cell {
            Some(cell) => cell,
            None => Cell::nil(),
        }
    }
}
impl std::fmt::Display for Cell<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", {
            let parts = self.split_string();
            let expressions = parts
                .clone()
                .iter()
                .filter(|part| part.is_some())
                .map(|expression| expression.clone().map(String::from).unwrap())
                .collect::<Vec<String>>();
            let expressions = expressions.join(" ");
            let mut wrap = expressions.len() > 0;
            // let mut wrap = parts.iter().all(|part|part.is_some());
            format!("({})", expressions)
        })
    }
}
impl std::fmt::Debug for Cell<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", {
            let parts = self.split_string();
            let expressions = parts
                .clone()
                .iter()
                .filter(|part| part.is_some())
                .map(|expression| expression.clone().map(String::from).unwrap())
                .collect::<Vec<String>>();
            let expressions = expressions.join(" . ");
            let mut wrap = expressions.len() > 0;
            // let mut wrap = parts.iter().all(|part|part.is_some());
            if wrap {
                format!("({})", expressions)
            } else {
                "nil".to_string()
            }
        })
    }
}

#[cfg(test)]
mod cell_tests {
    use std::rc::Rc;

    use k9::assert_equal;

    use crate::*;

    #[test]
    fn from_value_nil() {
        assert_equal!(
            Cell::from(Value::Nil),
            Cell {
                head: Value::Nil,
                tail: None
            }
        );
    }
    #[test]
    fn from_value_symbol() {
        assert_equal!(
            Cell::from(Value::from("symbol")),
            Cell {
                head: Value::from("symbol"),
                tail: None
            }
        );
    }
    #[test]
    fn nil() {
        assert_equal!(Cell::nil(), Cell::from(Value::Nil));
        assert_equal!(Cell::nil().split_string(), [None, None]);
        // assert_debug_equal!(Cell::nil(), Cell::from(Value::Nil));
        // assert_debug_equal!(Cell::nil(), "nil");
        // assert_display_equal!(Cell::nil(), Cell::from(Value::Nil));
        // assert_display_equal!(Cell::nil(), "()");
    }
    #[test]
    fn from_cell_nil() {
        assert_equal!(Cell::nil(), Cell::from(Cell::nil()));
        assert_debug_equal!(Cell::nil(), Cell::from(Value::Nil));
        assert_debug_equal!(Cell::nil(), "nil");
        assert_display_equal!(Cell::nil(), Cell::from(Value::Nil));
        assert_display_equal!(Cell::nil(), "()");
    }
    #[test]
    fn from_cell_debug_head_and_tail_with_head_symbol_tail_nil() {
        assert_equal!(
            Cell {
                head: Value::from("head"),
                tail: Some(Rc::new(Cell {
                    head: Value::from("tail"),
                    tail: None
                }))
            }
            .split_string(),
            [Some("head".to_string()), Some("tail".to_string()),]
        );

        // assert_debug_equal!(cons("head", Some(Cell::from(Value::from("tail")))), "(head . tail)");
        // assert_display_equal!(cons("head", Some(Cell::from(Value::from("tail")))), "(head tail)");
    }
    #[test]
    fn from_cell_debug_head_and_tail_with_head_nil_tail_head_symbol() {
        assert_equal!(
            Cell {
                head: Value::from("head"),
                tail: Some(Rc::new(Cell {
                    head: Value::Nil,
                    tail: Some(Rc::new(Cell {
                        head: Value::from("tail"),
                        tail: None
                    }))
                }))
            }
            .split_string(),
            [Some("head".to_string()), Some("tail".to_string()),]
        );
        // assert_debug_equal!(cons("head", Some(Cell::from(Value::from("tail")))), "(head . tail)");
        // assert_display_equal!(cons("head", Some(Cell::from(Value::from("tail")))), "(head tail)");
    }
}
