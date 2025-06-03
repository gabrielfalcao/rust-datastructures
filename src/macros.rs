#[macro_export]
macro_rules! cast_node_mut {
    ($ptr:expr, noincr) => {{
        $crate::cast_node_mut!($ptr, 'c, noincr)
    }};
    ($ptr:expr, incr) => {{
        $crate::cast_node_mut!($ptr, 'c, incr)
    }};
    ($ptr:expr, $lt:lifetime, incr ) => {{
        let node = unsafe {
            let mut node = &mut *$ptr;
            node.incr_ref();
            node
        };
        unsafe { std::mem::transmute::<&mut Node<$lt>, &$lt mut Node<$lt>>(node) }
    }};
    ($ptr:expr, $lt:lifetime, noincr ) => {{
        let node = unsafe {
            let mut node = &mut *$ptr;
            node
        };
        unsafe { std::mem::transmute::<&mut Node<$lt>, &$lt mut Node<$lt>>(node) }
    }};
}

#[macro_export]
macro_rules! cast_node_ref {
    ($ptr:expr) => {{
        $crate::cast_node_ref!($ptr, 'c)
    }};
    ($ptr:expr, $lt:lifetime ) => {{
        let node = unsafe {
            let mut node = & *$ptr;
            node
        };
        unsafe { std::mem::transmute::<& Node<$lt>, &$lt Node<$lt>>(node) }
    }};
}

#[macro_export]
macro_rules! decr_ref_nonzero {
    ($data_structure:expr) => {
        if $data_structure.refs > 0 {
            $data_structure.refs -= 1;
        } else {
            // eprintln!(
            //     "\r{}",
            //     crate::color::ansi(
            //         format!(
            //             "[{}:{}] WARNING: attempt to decrement references of {}",
            //             file!(),
            //             line!(),
            //             &$data_structure
            //         ),
            //         16,
            //         220
            //     )
            // );
        }
    };
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
                $crate::location!(),
                bg.into(),
                fg.into(),
            ),
            crate::color::ansi(
                if text.is_empty() { String::new() } else { format!(" {}", text) },
                bg.into(),
                fg.into(),
            )
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
                $crate::location!(),
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

#[macro_export]
macro_rules! location {
    () => {
        format!("{}:{}:{}", file!(), $crate::function_name!(), line!(),)
    };
}

#[macro_export]
macro_rules! warn {
    ($text:literal) => {{
        $crate::warn!(format!("{}", $text))
    }};
    ($text:literal, $( $arg:expr ),* ) => {{
        $crate::warn!(format_args!($text, $($arg,)*))
    }};
    ($text:expr) => {{
        let bg = 178usize;
        let fg = 16usize;
        let text = $text.to_string();
        eprintln!(
            "{} {}",
            crate::color::ansi(
                $crate::location!(),
                fg.into(),
                bg.into(),
            ),
            crate::color::ansi(
                if text.is_empty() { String::new() } else { format!("{}", text) },
                bg.into(),
                fg.into(),
            )
        );
    }};
    () => {{
        $crate::warn!("")
    }};
}

#[macro_export]
macro_rules! warn_inv {
    ($text:literal) => {{
        $crate::warn_inv!(format!("{}", $text))
    }};
    ($text:literal, $( $arg:expr ),* ) => {{
        $crate::warn_inv!(format_args!($text, $($arg,)*))
    }};
    ($text:expr) => {{
        let bg = 178usize;
        let fg = 16usize;
        let text = $text.to_string();
        eprintln!(
            "{} {}",
            crate::color::ansi(
                $crate::location!(),
                bg.into(),
                fg.into(),
            ),
            crate::color::ansi(
                if text.is_empty() { String::new() } else { format!("{}", text) },
                fg.into(),
                bg.into(),
            )
        );
    }};
    () => {{
        $crate::warn_inv!("")
    }};
}
