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
            crate::color::ansi(
                format!(
                    "{}:{}",
                    $crate::function_name!(),
                    line!(),
                ),
                bg.into(),
                fg.into(),
            ),
            if text.is_empty() { String::new() } else { format!(" {}", text) }

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
    ($text:literal, $( $arg:expr ),* ) => {{
        $crate::step_test!(format_args!($text, $($arg,)*))
    }};
    ($text:expr) => {{
        let (bg, fg) = $crate::color::couple(line!() as usize);
        let text = $text.to_string();
        let full_text =
            format!("{}:{} {}", $crate::function_name!(), line!(), &text);

        eprintln!(
            "\n{}\n{} {}",
            crate::color::bg(" ".repeat(full_text.len()), bg as usize),
            crate::color::ansi(
                format!(
                    "{}:{}",
                    $crate::function_name!(),
                    line!(),
                ),
                fg.into(),
                bg.into(),
            ),
            crate::color::ansi(
                if text.is_empty() { String::new() } else { format!("{}", text) },
                bg.into(),
                fg.into(),
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
