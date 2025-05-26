#[macro_export]
macro_rules! assert_display_equal {
    ($left:expr, $right:literal) => {{
        k9::assert_equal!(format!("{}", $left), $right);
    }};
    ($left:expr, $right:expr) => {{
        k9::assert_equal!(format!("{}", $left), format!("{}", $right));
    }};
}

#[macro_export]
macro_rules! assert_debug_equal {
    ($left:expr, $right:literal) => {{
        k9::assert_equal!(format!("{:#?}", $left), $right);
    }};
    ($left:expr, $right:expr) => {{
        k9::assert_equal!(format!("{:#?}", $left), format!("{:#?}", $right));
    }};
}

#[macro_export]
macro_rules! step {
    ($text:literal) => {{
        $crate::step!(format!("{}", $text))
    }};
    ($text:literal, $( $arg:expr ),* ) => {{
        $crate::step!(format_args!($text, $($arg,)*))
    }};
    ($text:expr) => {{
        let (bg, fg) = $crate::color::couple(line!() as usize);
        let text = $text.to_string();
        eprintln!(
            "{}{}",
            // "\n{}\n{}\n{}\n",
            // crate::color::reset(crate::color::bg(" ".repeat(80), fg)),
            crate::color::ansi(
                format!(
                    "{}:{}",
                    $crate::function_name!(),
                    line!(),
                ),
                fg.into(),
                bg.into(),
            ),
            if text.is_empty() { String::new() } else { format!(" {}", text) }

            // crate::color::reset(crate::color::bg(" ".repeat(80), fg)),
        );
    }};
    () => {{
        $crate::step!("")
    }};
}
#[macro_export]
macro_rules! step_test {
    ($text:literal) => {{
        $crate::step_test!(format!("{}", $text))
    }};
    ($text:expr) => {{
        let (bg, fg) = $crate::color::couple(line!() as usize);
        let text = $text.to_string();
        eprintln!(
            "\n{}\n{}\n",
            crate::color::reset(crate::color::bg(" ".repeat(80), bg.into())),
            crate::color::ansi(
                format!(
                    "{}{}{}",
                    $crate::function_name!(),
                    " ".repeat(80),
                    if text.is_empty() {
                        String::new()
                    } else {
                        format!(
                            "\nline {}:\t\t{}{}\n{}",
                            line!() + 1,
                            text,
                            " ".repeat(50),
                            " ".repeat(80)
                        )
                    }
                ),
                fg.into(),
                bg.into(),
            ),
        );
    }};
    () => {{
        $crate::step_test!("")
    }};
}

#[macro_export]
macro_rules! function_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        let name = name
            .strip_suffix("::f")
            .unwrap()
            .replace(format!("{}::", module_path!()).as_str(), "");
        name
    }};
}

// #[macro_export]
// macro_rules! dbg {
//     ($( $arg:expr ),* ) => {{
//         let obj = format!("{}", [$(
//             format!("{}", format!("{:#?}", $arg)),
//         )*].iter().map($crate::color::reset).collect::<Vec<String>>().join("\n"));
//         eprintln!("{}", $crate::color::reset([$crate::location!(begin), obj.clone(), $crate::location!(end)].join("\n")));
//         obj
//     }};
// }
// #[macro_export]
// macro_rules! location {
//     () => {{
//         let location = format!(
//             "{}{}{}:{}",
//             $crate::color::fg($crate::function_name!(), 28),
//             $crate::color::fg(" @ ", 220),
//             $crate::filename!(),
//             $crate::color::fg(line!().to_string(), 49)
//         );
//         location
//     }};
//     (begin) => {
//         $crate::tag!($crate::color::fg(
//             format!("in function {}", $crate::location!()),
//             178
//         ))
//     };
//     (end) => {
//         $crate::tag!(
//             close,
//             $crate::color::fg(format!("from function {}", $crate::location!()), 178)
//         )
//     };
//     (unexpected) => {
//         $crate::color::fg(
//             format!("<unexpected branch in function {}>", $crate::location!()),
//             160,
//         )
//     };
// }
// #[macro_export]
// macro_rules! filename {
//     () => {
//         $crate::filename!(237, 49)
//     };
//     ($folder_color:literal, $file_color:literal) => {{
//         let mut parts = file!()
//             .split(std::path::MAIN_SEPARATOR_STR)
//             .map(String::from)
//             .collect::<Vec<String>>();
//         let (folder, filename) = if parts.len() > 1 {
//             let last = $crate::color::fg(parts.remove(parts.len() - 1), $file_color);
//             let mut parts = parts
//                 .iter()
//                 .map(|part| $crate::color::fg(part, $folder_color))
//                 .collect::<Vec<String>>();
//             (parts, last)
//         } else {
//             (
//                 Vec::<String>::new(),
//                 $crate::color::fg(parts[0].to_string(), $file_color)
//             )
//         };
//         if folder.len() > 1 {
//             format!("{}{}{}", filename, $crate::color::fg(" in ", 7), folder.join(std::path::MAIN_SEPARATOR_STR))
//         } else {
//             filename
//         }
//     }};
// }
// #[macro_export]
// macro_rules! tag {
//     ($arg:expr) => {{
//         $crate::tag!($arg, 7)
//     }};
//     (close, $arg:expr) => {{
//         $crate::tag!(close, $arg, 7)
//     }};
//     ($arg:expr, $color:literal) => {{
//         format!(
//             "{}{}{}",
//             $crate::color::fg("<", $color),
//             $arg,
//             $crate::color::fg(">", $color),
//         )
//     }};
//     (close, $arg:expr, $color:literal) => {{
//         format!(
//             "{}{}{}",
//             $crate::color::fg("</", $color),
//             $arg,
//             $crate::color::fg(">", $color),
//         )
//     }};
// }
