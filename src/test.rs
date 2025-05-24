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
    ($text:expr) => {{
        let (bg, fg) = $crate::color::couple(line!() as usize);
        eprintln!(
            "{}",
            // "\n{}\n{}\n{}\n",
            // crate::color::reset(crate::color::bg(" ".repeat(80), fg)),
            crate::color::ansi(
                format!(
                    "{}:{}{}",
                    $crate::function_name!(),
                    line!(),
                    if $text.is_empty() { String::new() } else { format!("\t{}", $text) }
                ),
                bg.into(),
                fg.into(),
            ),
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
        eprintln!(
            "\n{}\n{}\n",
            crate::color::reset(crate::color::bg(" ".repeat(80), bg.into())),
            crate::color::ansi(
                format!(
                    "{}{}{}",
                    $crate::function_name!(),
                    " ".repeat(80),
                    if $text.is_empty() {
                        String::new()
                    } else {
                        format!(
                            "\nline {}:\t\t{}{}\n{}",
                            line!() + 1,
                            $text,
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
