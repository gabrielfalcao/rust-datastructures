use std::alloc::Layout;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::fmt::{Debug, Display, Formatter, Pointer};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use crate::{color, decr_ref_nonzero, internal, step, RefCounter};

pub struct UniquePointer<'c, T> {
    mut_addr: usize,
    mut_ptr: *mut T,
    const_addr: usize,
    const_ptr: *const T,
    orig_addr: usize,
    refs: RefCounter,
    alloc: bool,
    written: bool,
    _marker: PhantomData<&'c T>,
}

impl<'c, T: Sized + 'c> UniquePointer<'c, T> {
    pub fn null() -> UniquePointer<'c, T> {
        UniquePointer {
            mut_addr: 0,
            mut_ptr: std::ptr::null_mut::<T>(),
            const_addr: 0,
            const_ptr: std::ptr::null::<T>(),
            orig_addr: 0,
            refs: RefCounter::new(),
            written: false,
            alloc: false,
            _marker: PhantomData,
        }
    }

    pub fn addr(&self) -> usize {
        self.mut_addr
    }

    pub fn orig_addr(&self) -> usize {
        self.orig_addr
    }

    pub fn refs(&self) -> usize {
        *self.refs
    }

    pub fn is_null(&self) -> bool {
        step!();
        let mut_is_null = self.mut_ptr.is_null();
        if mut_is_null {
            step!("((self.mut_addr == {}) == 0) == true?", color::addr(self.mut_addr));
            assert!(self.mut_addr == 0);
        } else {
            step!("((self.mut_addr == {}) == 0) == false?", color::addr(self.mut_addr));
            assert!(self.mut_addr != 0);
        }
        let const_is_null = self.const_ptr.is_null();
        if const_is_null {
            step!("((self.const_addr == {}) == 0) == true?", color::addr(self.const_addr));
            assert!(self.const_addr == 0);
        } else {
            step!("((self.const_addr == {}) == 0) == false?", color::addr(self.const_addr));
            assert!(self.const_addr != 0);
        }

        let is_null = dbg!(dbg!(mut_is_null) && dbg!(const_is_null));
        step!("self.mut_addr == {:#?}", self.mut_addr);
        step!("self.const_addr == {:#?}", self.const_addr);
        step!("self.is_null == {:#?}", is_null);
        // !self.mut_ptr.is_null() || self.mut_addr == 0
        is_null
    }

    pub fn is_allocated(&self) -> bool {
        let is_allocated = !self.is_null() && self.alloc;
        step!("self.alloc == {:#?}", self.alloc);
        step!("self.is_allocated == {:#?}", is_allocated);
        is_allocated
    }

    pub fn is_written(&self) -> bool {
        step!("self.written == {:#?}", self.written);
        let is_written = self.is_allocated() && self.written;
        step!("self.is_written == {:#?}", is_written);
        is_written
    }

    pub fn alloc(&mut self) {
        step!("start");

        step!("check if self is allocated");
        if self.is_allocated() {
            step!("self.is_allocated, no need for allocation");
            return;
        } else {
            step!("self is not allocated");
        }

        let layout = Layout::new::<T>();
        let mut_ptr = unsafe {
            let ptr = std::alloc::alloc_zeroed(layout);
            if ptr.is_null() {
                std::alloc::handle_alloc_error(layout);
            }
            ptr as *mut T
        };
        self.mut_ptr = mut_ptr;
        let const_ptr = mut_ptr.cast_const();
        self.const_ptr = const_ptr;

        let mut_provenance = UniquePointer::<'c, T>::provenance_of_mut_ptr(mut_ptr);
        self.mut_addr = mut_provenance;
        let const_provenance = UniquePointer::<'c, T>::provenance_of_const_ptr(const_ptr);
        self.const_addr = const_provenance;
        step!("setting self.alloc = true");
        self.alloc = true;
        self.refs.incr();
        step!("end");
    }

    pub fn write(&mut self, data: T) {
        let orig_addr = UniquePointer::<'c, T>::raw_addr_of_ref(&data);
        // step!(
        //     "\n\rraw_addr_of_ref(&data: {}) == \n\r{} == \n\r{}",
        //     std::any::type_name::<T>(),
        //     color::addr(orig_addr),
        //     color::ref_addr(&data)
        // );
        // step!(
        //     "\n\rstd::ptr::from_ref(&data: {}).addr() == \n\r{} == \n\r{}",
        //     std::any::type_name::<T>(),
        //     color::addr(std::ptr::from_ref(&data).addr()),
        //     color::ref_addr(&data)
        // );
        step!("start");
        let mut up = self.meta_mut();
        step!("check if self is written");
        if self.is_written() {
            // panic!("already written {:#?}", self);
            step!("self is written, so: free it");
            // self.free();
            // self.reset();
        } else {
            step!("self is not written");
        }
        self.alloc();
        let mut ptr = self.cast_mut();
        unsafe {
            step!("unsafe write");
            ptr.write(data);
        }
        step!("setting self.written = true");
        self.written = true;
        self.orig_addr = orig_addr;
        step!("end");
    }

    pub fn write_ref_mut<'r>(&'c self, data: &'r mut T) {
        step!();
        let mut up = self.meta_mut();
        up.write(unsafe {
            let ptr = data as *mut T;
            ptr.read()
        });
    }

    pub fn write_ref<'r>(&'c self, data: &'r T) {
        let mut up = self.meta_mut();
        up.write(unsafe {
            let ptr = data as *const T;
            ptr.read()
        });
    }

    pub fn read(&self) -> T {
        step!();
        if !self.is_written() {
            panic!("{:#?} not written", self);
        }
        self.incr_ref();
        let mut ptr = self.cast_const();
        unsafe { ptr.read() }
    }

    pub fn cast_mut(&self) -> *mut T {
        step!();
        if self.is_null() {
            panic!("{:#?} is null", self);
            // return ptr;
        }
        self.mut_ptr
    }

    pub fn cast_const(&self) -> *const T {
        step!();
        if self.is_null() {
            panic!("{:#?} is null", self);
        }
        self.const_ptr
    }

    pub fn inner_ref(&self) -> &'c T {
        step!();
        self.incr_ref();
        unsafe { std::mem::transmute::<&T, &'c T>(&*self.const_ptr) }
    }

    pub fn inner_mut(&mut self) -> &'c mut T {
        step!();
        self.incr_ref();
        unsafe { std::mem::transmute::<&mut T, &'c mut T>(&mut *self.mut_ptr) }
    }

    pub fn as_ref(&self) -> Option<&'c T> {
        step!();
        if self.is_written() {
            Some(self.inner_ref())
        } else {
            None
        }
    }

    pub fn as_mut(&mut self) -> Option<&'c mut T> {
        step!();
        if self.is_written() {
            Some(self.inner_mut())
        } else {
            None
        }
    }

    pub fn dealloc(&mut self, soft: bool) {
        step!();
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
        step!();
        self.mut_addr = 0;
        self.const_addr = 0;
        self.refs.reset();
        self.alloc = false;
        self.written = false;
    }

    fn free(&mut self) {
        step!();
        if !self.is_null() {
            let layout = Layout::new::<T>();
            let mut_ptr = self.mut_ptr;
            let const_ptr = self.const_ptr;
            self.mut_ptr = std::ptr::null_mut::<T>();
            self.mut_addr = 0;
            self.const_ptr = std::ptr::null::<T>();
            self.const_addr = 0;
            unsafe {
                std::alloc::dealloc(mut_ptr as *mut u8, layout);
                // todo!();
                // step!("trying to dealloc const ptr");
                // std::alloc::dealloc(const_ptr as *mut u8, layout);
            };
        }
    }

    pub fn from_ref<'r>(data: &'r T) -> UniquePointer<'c, T> {
        step!();
        let up = UniquePointer::<'c, T>::null();
        up.write_ref(data);
        up
    }

    pub fn from_ref_mut<'r>(data: &'r mut T) -> UniquePointer<'c, T> {
        step!();
        let up = UniquePointer::<T>::null();
        up.write_ref_mut(data);
        up
    }
}

impl<T: Sized> UniquePointer<'_, T> {
    pub fn provenance_of_const_ptr(ptr: *const T) -> usize {
        ptr.expose_provenance()
    }

    pub fn provenance_of_mut_ptr(ptr: *mut T) -> usize {
        ptr.expose_provenance()
    }

    pub fn provenance_of_ref(ptr: &T) -> usize {
        (&raw const ptr).expose_provenance()
    }

    pub fn provenance_of_mut(mut ptr: &mut T) -> usize {
        (&raw mut ptr).expose_provenance()
    }

    pub fn raw_addr_of_const_ptr(ptr: *const T) -> usize {
        ptr.addr()
    }

    fn raw_addr_of_mut_ptr(ptr: *mut T) -> usize {
        ptr.addr()
    }

    pub fn raw_addr_of_ref(ptr: &T) -> usize {
        std::ptr::from_ref(ptr).addr()
        // (&raw const ptr).addr()
    }

    pub fn raw_addr_of_mut(mut ptr: &mut T) -> usize {
        std::ptr::from_mut(ptr).addr()
        // (&raw mut ptr).addr()
    }
}

impl<'c, T: 'c> UniquePointer<'c, T> {
    fn meta_mut(&'c self) -> &'c mut UniquePointer<'c, T> {
        let ptr = self.meta_mut_ptr();
        unsafe {
            let mut up = &mut *ptr;
            std::mem::transmute::<&mut UniquePointer<'c, T>, &'c mut UniquePointer<'c, T>>(up)
        }
    }
}
#[allow(invalid_reference_casting)]
impl<'c, T: Sized + 'c> UniquePointer<'c, T> {
    // private methods
    fn meta_mut_ptr(&self) -> *mut UniquePointer<'c, T> {
        let ptr = self as *const UniquePointer<'c, T>;
        unsafe {
            let ptr: *mut UniquePointer<'c, T> =
                std::mem::transmute::<*const UniquePointer<'c, T>, *mut UniquePointer<'c, T>>(ptr);
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
impl<'c, T: 'c> Deref for UniquePointer<'c, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.inner_ref()
    }
}

impl<'c, T: 'c> DerefMut for UniquePointer<'c, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.incr_ref();
        self.inner_mut()
    }
}

impl<'c, T: 'c> Drop for UniquePointer<'c, T> {
    fn drop(&mut self) {
        // step!("drop {:#?}", self);
        // self.dealloc(true);
    }
}

impl<'c, T: 'c> From<&T> for UniquePointer<'c, T> {
    fn from(data: &T) -> UniquePointer<'c, T> {
        UniquePointer::<T>::from_ref(data)
    }
}
impl<'c, T: 'c> From<&mut T> for UniquePointer<'c, T> {
    fn from(data: &mut T) -> UniquePointer<'c, T> {
        UniquePointer::<T>::from_ref_mut(data)
    }
}
impl<'c, T: 'c> From<T> for UniquePointer<'c, T> {
    fn from(data: T) -> UniquePointer<'c, T> {
        let mut up = UniquePointer::<T>::null();
        up.write(data);
        up
    }
}
impl<'c, T: 'c> Clone for UniquePointer<'c, T> {
    fn clone(&self) -> UniquePointer<'c, T> {
        self.incr_ref();
        let mut clone = UniquePointer::<T>::null();

        clone.mut_addr = self.mut_addr;
        clone.mut_ptr = self.mut_ptr;
        clone.const_addr = self.const_addr;
        clone.const_ptr = self.const_ptr;

        clone.refs = self.refs.clone();
        clone.alloc = self.alloc;
        clone.written = self.written;
        clone
    }
}
// impl<T: Display> Display for UniquePointer<'c, T> {
//     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
//         write!(f, "{}", if self.is_null() { format!("{:#?}", self) } else {self.as_ref()})
//     }
// }

impl<'c, T: 'c> Pointer for UniquePointer<'c, T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:016x}", self.addr())
    }
}

impl<'c, T: 'c> Debug for UniquePointer<'c, T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
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

impl<'t, T: Deref, S: Deref> PartialEq<UniquePointer<'t, S>> for UniquePointer<'t, T>
where
    T: PartialEq<S::Target>,
{
    fn eq(&self, other: &UniquePointer<'t, S>) -> bool {
        T::eq(self, other)
    }

    fn ne(&self, other: &UniquePointer<'t, S>) -> bool {
        T::ne(self, other)
    }
}

impl<'t, T: Deref<Target: Eq> + 't + Eq + PartialEq<<T as Deref>::Target>> Eq
    for UniquePointer<'t, T>
{
}

impl<'t, T: Deref, S: Deref> PartialOrd<UniquePointer<'t, S>> for UniquePointer<'t, T>
where
    T: PartialOrd<S::Target>,
{
    fn partial_cmp(&self, other: &UniquePointer<'t, S>) -> Option<Ordering> {
        T::partial_cmp(self, other)
    }

    fn lt(&self, other: &UniquePointer<'t, S>) -> bool {
        T::lt(self, other)
    }

    fn le(&self, other: &UniquePointer<'t, S>) -> bool {
        T::le(self, other)
    }

    fn gt(&self, other: &UniquePointer<'t, S>) -> bool {
        T::gt(self, other)
    }

    fn ge(&self, other: &UniquePointer<'t, S>) -> bool {
        T::ge(self, other)
    }
}

impl<'t, T: Deref<Target: Ord> + 't + Ord + PartialOrd<<T as Deref>::Target>> Ord
    for UniquePointer<'t, T>
{
    fn cmp(&self, other: &Self) -> Ordering {
        T::cmp(self, other)
    }
}

impl<'t, T: Deref<Target: Hash> + 't + Hash> Hash for UniquePointer<'t, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        T::hash(self, state);
    }
}
