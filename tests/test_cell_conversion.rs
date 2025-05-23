#![allow(unused)]
use ds::*;
use k9::assert_equal;

// #[test]
// fn test_cell_from_u8() {
//     step_test!();
//     let cell = Cell::from(0xf1u8);
//     step_test!();
//     let head = cell.head();
//     step_test!();
//     assert_equal!(head, Some(Value::Byte(0xf1u8)));
//     step_test!();
// }
// // #[test]
// // fn test_cell_from_str() {
// //     step_test!();
// //     let cell = Cell::from("head");
// //     step_test!();
// //     let head = cell.head();
// //     step_test!();
// //     assert_equal!(head, Some(Value::from("head")));
// //     step_test!();
// // }
// // #[test]
// // fn test_cell_from_value() {
// //     let cell = Cell::from(Value::Nil);
// //     assert_equal!(cell.head(), Some(Value::Nil));
// //     let cell = Cell::from(Value::from("string"));
// //     assert_equal!(cell.head(), Some(Value::from("string")));
// //     let cell = Cell::from(Value::from(0xF1u8));
// //     assert_equal!(cell.head(), Some(Value::from(0xF1u8)));
// // }

// // #[test]
// // fn test_cell_from_u8() {
// //     let cell = Cell::from(0x47);
// //     assert_equal!(cell.head(), Some(Value::Byte(0x47)));
// // }
// // #[test]
// // fn test_cell_from_u64() {
// //     let cell = Cell::from(0xBEEF);
// //     assert_equal!(cell.head(), Some(Value::UInt(0xBEEF)));
// // }
// // #[test]
// // fn test_cell_from_i64() {
// //     let cell = Cell::from(-47);
// //     assert_equal!(cell.head(), Some(Value::Int(-47)));
// // }
