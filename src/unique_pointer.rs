use std::alloc::Layout;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use crate::{color, decr_ref_nonzero, internal, step};

pub struct UniquePointer<T> {
    addr: usize,
    refs: usize,
    alloc: bool,
    written: bool,
    _marker: PhantomData<T>,
}

impl<T: Sized> UniquePointer<T> {
    pub fn null() -> UniquePointer<T> {
        UniquePointer {
            addr: 0,
            refs: 0,
            written: false,
            alloc: false,
            _marker: PhantomData,
        }
    }

    pub fn addr(&self) -> usize {
        self.addr
    }

    pub fn refs(&self) -> usize {
        self.refs
    }

    pub fn is_null(&self) -> bool {
        self.addr == 0
    }

    pub fn is_allocated(&self) -> bool {
        self.addr > 0 && self.alloc
    }

    pub fn is_written(&self) -> bool {
        self.is_allocated() && self.written
    }

    pub fn alloc(&self) {
        if self.is_allocated() {
            return;
        }

        let layout = Layout::new::<T>();
        let ptr = unsafe {
            let ptr = std::alloc::alloc(layout);
            if ptr.is_null() {
                std::alloc::handle_alloc_error(layout);
            }
            ptr as *mut T
        };
        let mut up = self.meta_mut();
        up.addr = ptr.addr();
        up.alloc = true;
        up.refs = 1;
    }

    pub fn write(&self, data: T) {
        let mut up = self.meta_mut();
        up.alloc();
        let mut ptr = up.cast_mut();
        unsafe {
            ptr.write(data);
        }
        up.written = true;
    }

    pub fn read(&self) -> T {
        if !self.is_written() {
            panic!("{:#?} not written", self);
        }
        let mut ptr = self.cast_const();
        unsafe { ptr.read() }
    }

    pub fn cast_mut(&self) -> *mut T {
        let mut ptr: *mut T = std::ptr::null_mut::<T>();
        if self.is_null() {
            return ptr;
        }
        ptr.with_addr(self.addr)
    }

    pub fn cast_const(&self) -> *const T {
        let mut ptr: *const T = std::ptr::null::<T>();
        if self.is_null() {
            return ptr;
        }
        ptr.with_addr(self.addr)
    }

    pub fn inner_ref<'c>(&self) -> &'c T {
        let ptr = self.cast_const();
        unsafe { std::mem::transmute::<&T, &'c T>(&*ptr) }
    }

    pub fn inner_mut<'c>(&self) -> &'c mut T {
        let mut ptr = self.cast_mut();
        unsafe { std::mem::transmute::<&mut T, &'c mut T>(&mut *ptr) }
    }

    pub fn as_ref<'c>(&self) -> Option<&'c T> {
        if self.is_written() {
            Some(self.inner_ref())
        } else {
            None
        }
    }

    pub fn as_mut<'c>(&self) -> Option<&'c mut T> {
        if self.is_written() {
            Some(self.inner_mut())
        } else {
            None
        }
    }

    pub fn dealloc(&self) {
        if !std::mem::needs_drop::<T>() {
            return;
        }

        if self.is_null() {
            return;
        }
        let mut up = self.meta_mut();
        if up.refs > 0 {
            up.decr_ref();
        } else {
            let layout = Layout::new::<T>();
            let mut ptr = up.cast_mut();
            unsafe {
                eprintln!(
                    "\n{}\n",
                    format!(
                        "{} {} at {}",
                        crate::color::fore("deallocating", 196),
                        crate::color::fore(std::any::type_name::<T>(), 231),
                        crate::color::fore(format!("{:p}", ptr), 178)
                    )
                );
                std::alloc::dealloc(ptr as *mut u8, layout);
                crate::step_test!("deallocated {} at {:p}", std::any::type_name::<T>(), ptr);
            };
        }
    }
}

impl <'c, T: 'c> UniquePointer<T>  {
    fn meta_mut(&'c self) -> &'c mut UniquePointer<T> {
        let ptr = self.meta_mut_ptr();
        unsafe {
            let mut up = &mut *ptr;
            std::mem::transmute::<&mut UniquePointer<T>, &'c mut UniquePointer<T>>(up)
        }
    }

}
#[allow(invalid_reference_casting)]
impl<T: Sized> UniquePointer<T> {
    // private methods
    fn meta_mut_ptr(&self) -> *mut UniquePointer<T> {
        let ptr = self as *const UniquePointer<T>;
        unsafe {
            let ptr: *mut UniquePointer<T> =
                std::mem::transmute::<*const UniquePointer<T>, *mut UniquePointer<T>>(ptr);
            ptr
        }
    }

    fn incr_ref(&self) {
        let ptr = self.meta_mut_ptr();
        unsafe {
            let mut up = &mut *ptr;
            up.refs += 1;
        }
    }

    fn decr_ref(&self) {
        let ptr = self.meta_mut_ptr();
        unsafe {
            let mut up = &mut *ptr;
            up.refs -= 1;
        }
    }
}

impl<T> Drop for UniquePointer<T> {
    fn drop(&mut self) {
        self.dealloc()
    }
}

impl<T: Sized + Copy> UniquePointer<T> {
    pub fn from_ref_copy(data: &T) -> UniquePointer<T> {
        let mut up = UniquePointer::null();
        up.write(*data);
        up
    }

    pub fn from_ref_mut_copy(data: &mut T) -> UniquePointer<T> {
        let mut up = UniquePointer::null();
        up.write(*data);
        up
    }
}
impl<T: Sized + Clone> UniquePointer<T> {
    pub fn from_ref(data: &T) -> UniquePointer<T> {
        let mut up = UniquePointer::null();
        up.write(data.clone());
        up
    }

    pub fn from_ref_mut(data: &mut T) -> UniquePointer<T> {
        let mut up = UniquePointer::null();
        up.write(data.clone());
        up
    }
}
impl<T> From<&T> for UniquePointer<T>
where
    T: Clone,
{
    fn from(data: &T) -> UniquePointer<T> {
        UniquePointer::<T>::from_ref(data)
    }
}
impl<T> From<&mut T> for UniquePointer<T>
where
    T: Clone,
{
    fn from(data: &mut T) -> UniquePointer<T> {
        UniquePointer::<T>::from_ref_mut(data)
    }
}
impl<T> From<T> for UniquePointer<T> {
    fn from(data: T) -> UniquePointer<T> {
        let mut up = UniquePointer::null();
        up.write(data);
        up
    }
}
impl<T> Clone for UniquePointer<T> {
    fn clone(&self) -> UniquePointer<T> {
        self.incr_ref();
        let mut clone = UniquePointer::<T>::null();
        clone.addr = self.addr;
        clone.refs = self.refs;
        clone.alloc = self.alloc;
        clone.written = self.written;
        clone
    }
}


// impl<T: std::fmt::Display> std::fmt::Display for UniquePointer<T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "{}", if self.is_null() { format!("{:#?}", self) } else {self.as_ref()})
//     }
// }
impl<T> std::fmt::Debug for UniquePointer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            crate::color::reset(
                [
                    crate::color::fg("UniquePointer@", 231),
                    format!("{:016x}", self.addr()),
                    format!("[refs={}]", self.refs),
                ]
                .join("")
            )
        )
    }
}
