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
        let (bg, fg) = $crate::colors(line!() as usize);
        eprintln!(
            "\n{}\n{}\n{}\n",
            crate::reset(crate::color_bg(" ".repeat(80), fg)),
            crate::colorize(
                format!(
                    "{}:{}{}",
                    $crate::function_name!(),
                    line!(),
                    if $text.is_empty() { String::new() } else { format!("\t{}", $text) }
                ),
                fg,
                bg
            ),
            crate::reset(crate::color_bg(" ".repeat(80), fg)),
        );
    }};
    () => {{
        $crate::step!("")
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
