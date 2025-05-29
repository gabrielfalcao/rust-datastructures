use std::alloc::Layout;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use crate::{color, decr_ref_nonzero, internal, step, RefCounter};

pub struct UniquePointer<T> {
    addr: usize,
    // ptr: *mut T,
    refs: RefCounter,
    alloc: bool,
    written: bool,
    _marker: PhantomData<T>,
}

impl<T: Sized> UniquePointer<T> {
    pub fn null() -> UniquePointer<T> {
        UniquePointer {
            addr: 0,
            refs: RefCounter::new(),
            // ptr: std::ptr::null_mut::<T>(),
            written: false,
            alloc: false,
            _marker: PhantomData,
        }
    }

    pub fn addr(&self) -> usize {
        self.addr
    }

    pub fn refs(&self) -> usize {
        *self.refs
    }

    pub fn is_null(&self) -> bool {
        let is_null = self.addr == 0;
        // step!("self.addr == {:#?}", self.addr);
        // step!("self.is_null == {:#?}", is_null);
        // !self.ptr.is_null() || self.addr == 0
        is_null
    }

    pub fn is_allocated(&self) -> bool {
        let is_allocated = !self.is_null() && self.alloc;
        // step!("self.alloc == {:#?}", self.alloc);
        // step!("self.is_allocated == {:#?}", is_allocated);
        is_allocated
    }

    pub fn is_written(&self) -> bool {
        // step!("self.written == {:#?}", self.written);
        let is_written = self.is_allocated() && self.written;
        // step!("self.is_written == {:#?}", is_written);
        is_written
    }

    pub fn alloc(&mut self) {
        // step!("start");

        // step!("check if self is allocated");
        if self.is_allocated() {
            // step!("self.is_allocated, do nothing");
            return;
        } else {
            // step!("self is not allocated");
        }

        let layout = Layout::new::<T>();
        let ptr = unsafe {
            let ptr = std::alloc::alloc(layout);
            if ptr.is_null() {
                std::alloc::handle_alloc_error(layout);
            }
            ptr as *mut T
        };
        // self.ptr = ptr;
        self.addr = ptr.expose_provenance();
        // step!("setting self.alloc = true");
        self.alloc = true;
        self.refs.incr();
        // step!("end");
    }

    pub fn write(&mut self, data: T) {
        // step!("start");
        // let mut up = self.meta_mut();
        // step!("check if self is written");
        if self.is_written() {
            // step!("self is written, so: free it");
            self.free();
        } else {
            // step!("self is not written");
        }
        self.alloc();
        let mut ptr = self.cast_mut();
        unsafe {
            // step!("unsafe write");
            ptr.write(data);
        }
        // step!("setting self.written = true");
        self.written = true;
        // step!("end");
    }

    pub fn write_ref_mut(&mut self, data: &mut T) {
        self.write(unsafe {
            let ptr = data as *mut T;
            ptr.read()
        });
    }

    pub fn write_ref(&mut self, data: &T) {
        self.write(unsafe {
            let ptr = data as *const T;
            ptr.read()
        });
    }

    pub fn read(&self) -> T {
        if !self.is_written() {
            panic!("{:#?} not written", self);
        }
        self.incr_ref();
        let mut ptr = self.cast_const();
        unsafe { ptr.read() }
    }

    pub fn cast_mut(&self) -> *mut T {
        let mut ptr: *mut T = std::ptr::null_mut::<T>();
        if self.is_null() {
            panic!("{:#?} is null", self);
            // return ptr;
        }
        std::ptr::with_exposed_provenance::<T>(self.addr).cast_mut()
    }

    pub fn cast_const(&self) -> *const T {
        let mut ptr: *const T = std::ptr::null::<T>();
        if self.is_null() {
            panic!("{:#?} is null", self);

            // return ptr;
        }
        std::ptr::with_exposed_provenance::<T>(self.addr)
    }

    pub fn inner_ref<'c>(&self) -> &'c T {
        self.incr_ref();
        let ptr = self.cast_const();
        unsafe { std::mem::transmute::<&T, &'c T>(&*ptr) }
    }

    pub fn inner_mut<'c>(&mut self) -> &'c mut T {
        self.incr_ref();
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

    pub fn as_mut<'c>(&mut self) -> Option<&'c mut T> {
        if self.is_written() {
            Some(self.inner_mut())
        } else {
            None
        }
    }

    pub fn dealloc(&mut self, soft: bool) {
        if self.is_null() {
            return;
        }
        // let mut up = self.meta_mut();
        if !soft && self.refs > 0 {
            // step!("decr_ref {:#?}", self);
            self.decr_ref();
        } else {
            // step!("free(ptr) {:#?}", self);
            self.free();
            self.reset();
        }
    }

    fn reset(&mut self) {
        self.addr = 0;
        self.refs.reset();
        self.alloc = false;
        self.written = false;
    }

    fn free(&mut self) {
        if !self.is_null() {
            let layout = Layout::new::<T>();
            let mut ptr = self.cast_mut();
            unsafe {
                std::alloc::dealloc(ptr as *mut u8, layout);
            };
        }
    }

    pub fn from_ref(data: &T) -> UniquePointer<T> {
        let mut up = UniquePointer::<T>::null();
        up.write_ref(data);
        up
    }

    pub fn from_ref_mut(data: &mut T) -> UniquePointer<T> {
        let mut up = UniquePointer::<T>::null();
        up.write_ref_mut(data);
        up
    }
}

impl<'c, T: 'c> UniquePointer<T> {
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
        if self.is_null() {
            panic!("null {:#?}", self);
        }
        let ptr = self.meta_mut_ptr();
        unsafe {
            let mut up = &mut *ptr;
            up.refs += 1;
        }
    }

    fn decr_ref(&self) {
        if self.refs < 2 {
            panic!("refs {}", self.refs);
            return;
        }
        let ptr = self.meta_mut_ptr();
        unsafe {
            let mut up = &mut *ptr;
            up.refs -= 1;
        }
    }
}
impl<T> Deref for UniquePointer<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.inner_ref()
    }
}

impl<T> DerefMut for UniquePointer<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.incr_ref();
        self.inner_mut()
    }
}

impl<T> Drop for UniquePointer<T> {
    fn drop(&mut self) {
        // step!("drop {:#?}", self);
        // self.dealloc(true);
    }
}

impl<T> From<&T> for UniquePointer<T> {
    fn from(data: &T) -> UniquePointer<T> {
        UniquePointer::<T>::from_ref(data)
    }
}
impl<T> From<&mut T> for UniquePointer<T> {
    fn from(data: &mut T) -> UniquePointer<T> {
        UniquePointer::<T>::from_ref_mut(data)
    }
}
impl<T> From<T> for UniquePointer<T> {
    fn from(data: T) -> UniquePointer<T> {
        let mut up = UniquePointer::<T>::null();
        up.write(data);
        up
    }
}
impl<T> Clone for UniquePointer<T> {
    fn clone(&self) -> UniquePointer<T> {
        self.incr_ref();
        let mut clone = UniquePointer::<T>::null();
        clone.addr = self.addr;
        clone.refs = self.refs.clone();
        // clone.ptr = self.ptr;
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
