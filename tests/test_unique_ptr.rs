#![allow(unused)]
use ds::*;
use k9::{assert_equal, assert_greater_than};

#[derive(Clone)]
pub struct Data<'t> {
    pub value: UniquePointer<Value<'t>>,
}

#[test]
fn test_unique_pointer_null() {
    let data = Data {
        value: UniquePointer::null(),
    };

    assert_equal!(data.value.is_null(), true);
    assert_equal!(data.value.addr(), 0);
    assert_equal!(data.value.refs(), 0);
    assert_equal!(data.value.is_written(), false);
    assert_equal!(data.value.as_ref(), None);
}

#[test]
fn test_unique_pointer_write() {
    let mut data = Data {
        value: UniquePointer::null(),
    };

    data.value.write(Value::from("string"));

    assert_equal!(data.value.is_null(), false);
    assert_equal!(data.value.is_allocated(), true);
    assert_greater_than!(data.value.addr(), 0, "address should not be null");
    assert_equal!(data.value.is_written(), true);
    assert_equal!(data.value.inner_ref(), &Value::from("string"));
    assert_equal!(data.value.read(), Value::from("string"));
    assert_equal!(data.value.as_ref(), Some(&Value::from("string")));
    assert_equal!(data.value.as_mut(), Some(&mut Value::from("string")));
}

#[test]
fn test_unique_pointer_write_ref_mut() {
    let mut data = Data {
        value: UniquePointer::null(),
    };

    data.value.write_ref_mut(&mut Value::from("string"));

    assert_equal!(data.value.is_null(), false);
    assert_equal!(data.value.is_allocated(), true);
    assert_greater_than!(data.value.addr(), 0, "address should not be null");
    assert_equal!(data.value.is_written(), true);
    assert_equal!(data.value.inner_ref(), &Value::from("string"));
    assert_equal!(data.value.read(), Value::from("string"));
    assert_equal!(data.value.as_ref(), Some(&Value::from("string")));
    assert_equal!(data.value.as_mut(), Some(&mut Value::from("string")));
}

#[test]
fn test_unique_pointer_write_ref() {
    let mut data = Data {
        value: UniquePointer::null(),
    };

    data.value.write_ref(&Value::from("string"));

    assert_equal!(data.value.is_null(), false);
    assert_equal!(data.value.is_allocated(), true);
    assert_greater_than!(data.value.addr(), 0, "address should not be null");
    assert_equal!(data.value.is_written(), true);
    assert_equal!(data.value.inner_ref(), &Value::from("string"));
    assert_equal!(data.value.read(), Value::from("string"));
    assert_equal!(data.value.as_ref(), Some(&Value::from("string")));
    assert_equal!(data.value.as_mut(), Some(&mut Value::from("string")));
}

#[test]
fn test_unique_pointer_from_value() {
    let mut data = Data {
        value: UniquePointer::from(Value::from("string")),
    };

    assert_equal!(data.value.is_null(), false);
    assert_equal!(data.value.is_allocated(), true);
    assert_greater_than!(data.value.addr(), 0, "address should not be null");
    assert_equal!(data.value.is_written(), true);
    assert_equal!(data.value.inner_ref(), &Value::from("string"));
    assert_equal!(data.value.read(), Value::from("string"));
    assert_equal!(data.value.as_ref(), Some(&Value::from("string")));
    assert_equal!(data.value.as_mut(), Some(&mut Value::from("string")));
}

#[test]
fn test_unique_pointer_from_ref_clone() {
    // step_test!("create unique pointer");
    let mut data = Data {
        value: UniquePointer::from_ref(&Value::from("string")),
    };
    // step_test!("assert value is NOT null");

    assert_equal!(data.value.is_null(), false);

    // step_test!("assert value is written");
    assert_equal!(data.value.is_written(), true);
    // assert_equal!(data.value.is_allocated(), true);
    assert_greater_than!(data.value.addr(), 0, "address should not be null");
    assert_equal!(data.value.inner_ref(), &Value::from("string"));
    assert_equal!(data.value.read(), Value::from("string"));
    assert_equal!(data.value.as_ref(), Some(&Value::from("string")));
    assert_equal!(data.value.as_mut(), Some(&mut Value::from("string")));
}
#[test]
fn test_unique_pointer_from_ref_copy() {
    let mut value: UniquePointer<u8> = UniquePointer::from_ref(&0xF1);

    assert_equal!(value.is_null(), false);
    assert_equal!(value.is_allocated(), true);
    assert_greater_than!(value.addr(), 0, "address should not be null");
    assert_equal!(value.is_written(), true);
    assert_equal!(value.inner_ref(), &0xF1);
    assert_equal!(value.read(), 0xF1);
    assert_equal!(value.as_ref(), Some(&0xF1));
    assert_equal!(value.as_mut(), Some(&mut 0xF1));
}

#[test]
fn test_unique_pointer_from_mut_clone() {
    let mut value: UniquePointer<Value> = UniquePointer::from_ref_mut(&mut Value::from("string"));

    assert_equal!(value.is_null(), false);
    assert_equal!(value.is_allocated(), true);
    assert_greater_than!(value.addr(), 0, "address should not be null");
    assert_equal!(value.is_written(), true);
    assert_equal!(value.inner_ref(), &Value::from("string"));

    assert_equal!(value.read(), Value::from("string"));
    assert_equal!(value.as_ref(), Some(&Value::from("string")));
    assert_equal!(value.as_mut(), Some(&mut Value::from("string")));
}

#[test]
fn test_unique_pointer_inner_mut() {
    let mut data = Data {
        value: UniquePointer::from(&mut Value::from("string")),
    };

    assert_equal!(data.value.is_null(), false);
    assert_equal!(data.value.is_allocated(), true);
    assert_greater_than!(data.value.addr(), 0, "address should not be null");
    assert_equal!(data.value.is_written(), true);
    assert_equal!(data.value.inner_mut(), &mut Value::from("string"));
    assert_equal!(data.value.refs(), 2);
    {
        let mut value = &*data.value;
        assert_equal!(value, &mut Value::from("string"));
        assert_equal!(data.value.refs(), 3);
    }
    {
        let value = &*data.value;
        assert_equal!(value, &Value::from("string"));
        assert_equal!(data.value.refs(), 4);
    }
    assert_equal!(data.value.refs(), 4);

    assert_equal!(data.value.read(), Value::from("string"));
    assert_equal!(data.value.as_ref(), Some(&Value::from("string")));
    assert_equal!(data.value.as_mut(), Some(&mut Value::from("string")));
}

#[test]
fn test_unique_pointer_from_ref_mut() {
    let mut data = Data {
        value: UniquePointer::from(&mut Value::from("string")),
    };

    assert_equal!(data.value.is_null(), false);
    assert_equal!(data.value.is_allocated(), true);
    assert_greater_than!(data.value.addr(), 0, "address should not be null");
    assert_equal!(data.value.is_written(), true);
    assert_equal!(data.value.inner_ref(), &Value::from("string"));
    assert_equal!(data.value.read(), Value::from("string"));
    assert_equal!(data.value.as_ref(), Some(&Value::from("string")));
    assert_equal!(data.value.as_mut(), Some(&mut Value::from("string")));
}

#[test]
fn test_unique_pointer_from_ref() {
    let mut data = Data {
        value: UniquePointer::from(&Value::from("string")),
    };

    assert_equal!(data.value.is_null(), false);
    assert_equal!(data.value.is_allocated(), true);
    assert_greater_than!(data.value.addr(), 0, "address should not be null");
    assert_equal!(data.value.is_written(), true);
    assert_equal!(data.value.inner_ref(), &Value::from("string"));
    assert_equal!(data.value.read(), Value::from("string"));
    assert_equal!(data.value.as_ref(), Some(&Value::from("string")));
    assert_equal!(data.value.as_mut(), Some(&mut Value::from("string")));
}

#[test]
fn test_unique_pointer_clone() {
    let mut data = Data {
        value: UniquePointer::from(Value::from("string")),
    };
    let mut clone = data.clone();

    assert_equal!(data.value.is_null(), false);
    assert_equal!(data.value.is_allocated(), true);
    assert_greater_than!(data.value.addr(), 0, "address should not be null");
    assert_equal!(data.value.is_written(), true);
    assert_equal!(data.value.inner_mut(), &mut Value::from("string"));
    assert_equal!(data.value.read(), Value::from("string"));
    assert_equal!(data.value.as_ref(), Some(&Value::from("string")));
    assert_equal!(data.value.as_mut(), Some(&mut Value::from("string")));

    assert_equal!(clone.value.is_null(), false);
    assert_greater_than!(clone.value.addr(), 0, "address should not be null");
    assert_equal!(clone.value.is_written(), true);
    assert_equal!(clone.value.inner_mut(), &mut Value::from("string"));
    assert_equal!(clone.value.read(), Value::from("string"));
    assert_equal!(clone.value.as_ref(), Some(&Value::from("string")));
    assert_equal!(clone.value.as_mut(), Some(&mut Value::from("string")));

    data.value.write(Value::from("updated"));

    assert_equal!(data.value.is_null(), false);
    assert_equal!(data.value.is_allocated(), true);
    assert_greater_than!(data.value.addr(), 0, "address should not be null");
    // assert_equal!(data.value.is_written(), true);
    // assert_equal!(data.value.inner_mut(), &mut Value::from("updated"));
    // assert_equal!(data.value.read(), Value::from("updated"));
    // assert_equal!(data.value.as_ref(), Some(&Value::from("updated")));
    // assert_equal!(data.value.as_mut(), Some(&mut Value::from("updated")));

    // assert_equal!(clone.value.is_null(), false);
    // assert_greater_than!(clone.value.addr(), 0, "address should not be null");
    // assert_equal!(clone.value.is_written(), true);
    // assert_equal!(clone.value.inner_mut(), &mut Value::from("updated"));
    // assert_equal!(clone.value.read(), Value::from("updated"));
    // assert_equal!(clone.value.as_ref(), Some(&Value::from("updated")));
    // assert_equal!(clone.value.as_mut(), Some(&mut Value::from("updated")));
}
