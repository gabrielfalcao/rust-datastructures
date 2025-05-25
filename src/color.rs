use std::fmt::Debug;

pub fn fg<T: std::fmt::Display>(text: T, fg: usize) -> String {
    format!("\x1b[1;38;5;{}m{}", wrap(fg), text)
}
pub fn bg<T: std::fmt::Display>(text: T, bg: usize) -> String {
    format!("\x1b[1;48;5;{}m{}", wrap(bg), text)
}
pub fn reset<T: std::fmt::Display>(text: T) -> String {
    format!("{}\x1b[0m", text)
}
pub fn bgfg<T: std::fmt::Display>(text: T, fore: usize, back: usize) -> String {
    bg(fg(text, wrap(fore) as usize), wrap(back) as usize)
}
pub fn ansi<T: std::fmt::Display>(text: T, fore: usize, back: usize) -> String {
    reset(bgfg(text, fore as usize, back as usize))
}
pub fn fore<T: std::fmt::Display>(text: T, fore: usize) -> String {
    let (fore, back) = couple(fore);
    ansi(text, fore as usize, back as usize)
}
pub fn back<T: std::fmt::Display>(text: T, back: usize) -> String {
    let (back, fore) = couple(back);
    ansi(text, fore as usize, back as usize)
}

pub fn couple(color: usize) -> (u8, u8) {
    let fore = wrap(color);
    let back = invert_bw(fore);
    (fore, back)
}

pub fn invert_bw(color: u8) -> u8 {
    match color {
        0 | 8 | 16..21 | 52..61 | 88..93 | 232..239 => 231,
        _ => 16,
    }
}

pub fn wrap(color: usize) -> u8 {
    (if color > 0 { color % 255 } else { color }) as u8
}

pub fn addr<T: Sized + Debug>(t: &T) -> String {
    let addr = std::ptr::from_ref(t);
    ptr(addr)
}
pub fn ptr_colors<T: Sized>(addr: *const T) -> (u8, u8) {
    match addr.addr() {
        0 => (255, 9),
        8 => (16, 137),
        addr => couple(addr),
    }
}
pub fn ptr_repr<T: Sized + Debug>(
    ptr: *const T,
    bg: u8,
    fg: u8,
    null_bg: u8,
    null_fg: u8,
    nonnull_bg: u8,
    nonnull_fg: u8,
) -> String {
    format!(
        "{}{}{}",
        reset(bgfg(format!("0x{:016x}", ptr.addr()), fg.into(), bg.into())),
        bgfg(":", 231, 16),
        if ptr.is_null() {
            reset(bgfg("null", null_fg.into(), null_bg.into()))
        } else {
            reset(bgfg("non-null", nonnull_fg.into(), nonnull_bg.into()))
        }
    )
}
pub fn ptr<T: Sized + Debug>(ptr: *const T) -> String {
    let (bg, fg) = ptr_colors(ptr);
    let (null_bg, null_fg) = couple(9);
    let (nonnull_bg, nonnull_fg) = couple(101);
    ptr_repr(ptr, bg, fg, null_bg, null_fg, nonnull_bg, nonnull_fg)
}
pub fn ptr_inv<T: Sized + Debug>(ptr: *const T) -> String {
    let (fg, bg) = ptr_colors(ptr);
    let (null_fg, null_bg) = couple(9);
    let (nonnull_fg, nonnull_bg) = couple(101);
    ptr_repr(ptr, bg, fg, null_bg, null_fg, nonnull_bg, nonnull_fg)
}
