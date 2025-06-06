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
macro_rules! assert_addr_equal {
    ($left:expr, $right:literal) => {{
        k9::assert_equal!(format!("{:016x}", $left), $right);
    }};
    ($left:expr, $right:expr) => {{
        k9::assert_equal!(format!("{:016x}", $left), format!("{:016x}", $right));
    }};
}
#[macro_export]
macro_rules! assert_nonzero {
    ($value:expr, $desc:literal) => {{
        k9::assert_greater_than!($value, 0, $desc);
    }};
    ($value:expr) => {{
        k9::assert_greater_than!($value, 0);
    }};
}
