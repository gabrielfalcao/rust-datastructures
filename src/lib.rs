#![allow(unused)]

pub mod traits;
pub use traits::ListValue;
pub mod cons;
pub use cons::{car, cdr, cons, Cons};
pub mod cell;
pub use cell::Cell;
pub mod value;
pub use value::Value;
pub mod test;

pub fn color_addr<T: Sized>(t: &T) -> String {
    let addr = std::ptr::from_ref(t).addr();
    let (bg, fg) = colors(addr);
    format!("\x1b[1;48;5;{}m\x1b[1;38;5;{}m{}\x1b[0m", bg, fg, addr)
}

pub fn color_fg<T: std::fmt::Display>(text: T, fg: u8) -> String {
    format!("\x1b[1;38;5;{}m{}", fg, text)
}
pub fn color_bg<T: std::fmt::Display>(text: T, bg: u8) -> String {
    format!("\x1b[1;48;5;{}m{}", bg, text)
}
pub fn reset<T: std::fmt::Display>(text: T) -> String {
    format!("{}\x1b[0m", text)
}
pub fn color_bgfg<T: std::fmt::Display>(text: T, fg: u8, bg: u8) -> String {
    color_bg(color_fg(text, fg), bg)
}
pub fn colorize<T: std::fmt::Display>(text: T, fg: u8, bg: u8) -> String {
    reset(color_bgfg(text, fg, bg))
}

pub fn colors(addr: usize) -> (u8, u8) {
    let fg = if addr > 0 { addr % 255 } else { 16 };
    let bg = match fg {
        0 | 8 | 16..24 | 232..237 => 255,
        _ => 16,
    };
    (bg as u8, fg as u8)
}
