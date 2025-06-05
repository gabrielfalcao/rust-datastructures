pub(super) mod null {
    use crate::color;

    use crate::{Cell, Value};
    pub(crate) fn ptr<T>() -> *mut T {
        std::ptr::null_mut::<T>()
    }
    pub(crate) fn cell<'c, T: Value>() -> *mut Cell<'c, T> {
        self::ptr::<Cell<'c, T>>()
    }
}
pub(super) mod alloc {
    use std::alloc::Layout;

    use crate::color;

    use crate::{Cell, Value};
    pub(crate) unsafe fn new<T>() -> *mut T {
        let layout = Layout::new::<T>();
        let ptr = unsafe {
            let ptr = std::alloc::alloc_zeroed(layout);
            if ptr.is_null() {
                std::alloc::handle_alloc_error(layout);
            }
            ptr
        };
        ptr as *mut T
    }
    pub(crate) unsafe fn cell<'c, T: Value>() -> *mut Cell<'c, T> {
        unsafe { self::new::<Cell<'c, T>>() }
    }
}
pub(super) mod dealloc {
    use std::alloc::Layout;

    use crate::color;

    use crate::{Cell, Value};
    pub(crate) unsafe fn free<T>(mut ptr: *mut T) {
        let layout = Layout::new::<T>();
        unsafe {
            if !std::mem::needs_drop::<T>() {
                #[rustfmt::skip]
                eprintln!("no need to drop {}", color::fore(std::any::type_name::<T>(), 178));
                return;
            }

            #[rustfmt::skip]#[cfg(feature="debug")]
            eprintln!("{} {} {}", color::fg("freeing", 9), color::fg("ptr", 231), color::ptr_inv(ptr));

            std::alloc::dealloc(ptr as *mut u8, layout);
        };
    }
    pub(crate) unsafe fn cell<'c, T: Value>(mut cell: *mut Cell<'c, T>) {
        #[rustfmt::skip]#[cfg(feature="debug")]
        eprintln!("{} {} {}", color::fg("freeing", 9), color::fg("cell", 137), color::ptr_inv(cell));
        unsafe { self::free::<Cell<'c, T>>(cell) }
    }
}
