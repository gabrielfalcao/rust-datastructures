use std::alloc::Layout;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::fmt::{Debug, Display, Formatter, Pointer};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use crate::{color, decr_ref_nonzero, internal, step, warn, RefCounter};

/// [`UniquePointer`] is an experimental data structure that makes
/// extensive use of unsafe rust to provide a shared pointer
/// throughout the runtime of a rust library or application as
/// transparently as possible.
///
/// [`UniquePointer`] is designed for practicing the design of basic
/// computer science data-structures (e.g.: Binary Trees, Linked Lists
/// etc) such that the concept of pointer is as close to C as possible
/// in terms of developer experience and so when a CS teacher speaks
/// in terms of pointers, students can use [`UniquePointer`] in their
/// data-structures knowing that cloning their data-structures also
/// means cloning the pointers transparently.
///
/// In fact, the author designed [`UniquePointer`] while studying the
/// MIT CourseWare material of professor Erik Demaine in addition to
/// studying lisp "cons" cells.
///
/// To this point the author reiterates: [`UniquePointer`] is an
/// **experimental** data-structure designed primarily as a
/// building-block of other data-structures in rust.
///
/// [`UniquePointer`] provides the methods [`cast_mut`] and `cast_const`
/// not unlike those of raw pointers, and also implements the methods
/// [`as_ref`] and [`as_mut`] with a signature compatible with that of the
/// [`AsRef`] and [`AsMut`] traits such that users of raw pointers can
/// migrate to [`UniquePointer`] without much difficulty.
///
/// [`UniquePointer`] is designed a way such that Enums and Structs
/// using [`UniquePointer`] can safely clone [`UniquePointer`] while
/// the memory address and provenance of its value is
/// shared.
///
/// [`UniquePointer`] is able to extend lifetimes because it maintains its
/// own reference counting outside of the rust compiler.
///
/// Reference Counting is provided by [`RefCounter`] which uses unsafe
/// rust to ensure that ref counts are shared across cloned objects
/// memory.
///
/// Both [`UniquePointer`] and [`RefCounter`] use relatively obscure
/// rust techniques under the hood to allow writing in non-mut
/// references in strategic occasions such as incrementing its
/// reference count within its [`Clone`] implementation.
///
/// UniquePointer only supports [`Sized`] types, that is,
/// Zero-Sized-Types (ZSTs) are not supported.
///
/// Example
///
/// ```
/// use ds::UniquePointer;
///
/// fn create_unique_pointer() -> UniquePointer<&'a str> {
///     UniquePointer::from("string")
/// }
/// let mut value: UniquePointer<&'_ str> = create_unique_pointer();
///
/// assert_eq!(value.is_null(), false);
/// assert_eq!(value.is_allocated(), true);
/// assert!(value.addr() > 0, "address should not be null");
/// assert_eq!(value.is_written(), true);
/// assert_eq!(value.inner_ref(), &"string");
///
/// assert_eq!(value.read(), "string");
/// assert_eq!(value.as_ref(), Some(&"string"));
/// ```
///
/// > # NOTE: **[`UniquePointer`] IS NOT THREAD SAFE** (yet)
///
pub struct UniquePointer<T> {
    mut_addr: usize,
    mut_ptr: *mut T,
    orig_addr: usize,
    refs: RefCounter,
    alloc: bool,
    is_copy: bool,
    written: bool,
}

impl<'c, T: Sized + 'c> UniquePointer<T> {
    /// `null` creates a NULL [`UniquePointer`] ready to be written via [`write`].
    pub fn null() -> UniquePointer<T> {
        UniquePointer {
            mut_addr: 0,
            mut_ptr: std::ptr::null_mut::<T>(),
            orig_addr: 0,
            refs: RefCounter::new(),
            written: false,
            alloc: false,
            is_copy: false,
        }
    }

    /// `from_ref` creates a new [`UniquePointer`] by effectively
    /// reading the value from [`reference`] after extending its
    /// lifetime from `'r` to `'c`
    ///
    pub fn from_ref<'r>(reference: &'r T) -> UniquePointer<T> {
        let reference = unsafe { std::mem::transmute::<&'r T, &'c T>(reference) };
        let mut up = UniquePointer::<T>::null();
        up.write_ref(reference);
        up
    }

    /// `from_ref_mut` creates a new [`UniquePointer`] by effectively
    /// reading the value from [`mutable_reference`] after extending its
    /// lifetime from `'r` to `'c`
    ///
    pub fn from_ref_mut<'r>(mutable_reference: &'r mut T) -> UniquePointer<T> {
        let mut mutable_reference =
            unsafe { std::mem::transmute::<&'r mut T, &'c mut T>(mutable_reference) };
        let mut up = UniquePointer::<T>::null();
        up.write_ref_mut(mutable_reference);
        up
    }

    /// `copy` is designed for use within the [`Clone`] implementation
    /// of `UniquePointer`.
    ///
    /// The [`copy`] method creates a NULL [`UniquePointer`] flagged as
    /// [`is_copy`] such that a double-free does not happen in
    /// [`dealloc`].
    fn copy() -> UniquePointer<T> {
        let mut up = UniquePointer::<T>::null();
        up.is_copy = true;
        up
    }

    /// `propagate` creates a copy of a [`UniquePointer`] which is not
    /// a copy in the sense that: - [`is_copy`] returns true; and -
    /// [`orig_addr`] points to the same memory address
    ///
    /// Because of that rationale a double-free occurs if there are
    /// two or more "containers" (i.e.: structs, enums, and/or unions)
    /// implementing [`Drop`] and holding the same propagated
    /// [`UniquePointer`] instance. For this reason [`propagate`] is
    /// unsafe.
    ///
    /// [`propagate`] can be relatively observed as a drop-in
    /// replacement for [`clone`] when, for instance, swapping
    /// [`UniquePointer`] "instances" between instances of
    /// `UniquePointer`-containing (structs, enums and/or unions) is
    /// desired.
    pub unsafe fn propagate(&self) -> UniquePointer<T> {
        self.incr_ref();
        let mut back_node = UniquePointer::<T>::null();
        back_node.set_mut_ptr(self.mut_ptr, false);
        back_node.refs = self.refs.clone();
        back_node.orig_addr = self.orig_addr;
        back_node.alloc = self.alloc;
        back_node.written = self.written;
        back_node
    }

    pub fn copy_from_ref(data: &T, refs: usize, orig_addr: usize) -> UniquePointer<T> {
        let ptr = (data as *const T).cast_mut();
        UniquePointer::copy_from_mut_ptr(ptr, refs, orig_addr)
    }

    pub fn copy_from_mut_ptr(ptr: *mut T, refs: usize, orig_addr: usize) -> UniquePointer<T> {
        let addr = UniquePointer::provenance_of_mut_ptr(ptr);
        let refs = RefCounter::from(refs);
        UniquePointer {
            mut_addr: addr,
            mut_ptr: ptr,
            orig_addr: orig_addr,
            refs: refs,
            written: true,
            alloc: true,
            is_copy: true,
        }
    }

    /// `addr` returns the value containing both the provenance and
    /// memory address of a pointer
    pub fn addr(&self) -> usize {
        self.mut_addr
    }

    /// `orig_addr` returns the address of the value written into [`UniquePointer`] via [`write`]
    pub fn orig_addr(&self) -> usize {
        self.orig_addr
    }

    /// `refs` returns the reference count of a `UniquePointer`
    pub fn refs(&self) -> usize {
        *self.refs
    }

    /// `is_null` returns true if the [`UniquePointer`] is NULL.
    pub fn is_null(&self) -> bool {
        let mut_is_null = self.mut_ptr.is_null();
        if mut_is_null {
            assert!(self.mut_addr == 0);
        } else {
            assert!(self.mut_addr != 0);
        }
        let is_null = mut_is_null;
        is_null
    }

    /// `is_not_null` returns true if the [`UniquePointer`] is not
    /// NULL. [`is_not_null`] is a idiomatic shortcut to negating a call
    /// to [`is_null`] such that the negation is less likely to be
    /// clearly visible.
    pub fn is_not_null(&self) -> bool {
        !self.is_null()
    }

    /// `is_not_copy` returns true if the [`UniquePointer`] is not a
    /// copy. [`is_not_copy`] is a idiomatic shortcut to negating a call
    /// to [`is_copy`] such that the negation is less likely to be
    /// clearly visible.
    pub fn is_not_copy(&self) -> bool {
        !self.is_copy
    }

    /// `can_dealloc` returns true if the [`UniquePointer`] is not NULL
    /// and is not flagged as a copy, meaning it can be deallocated
    /// without concern for double-free.
    pub fn can_dealloc(&self) -> bool {
        self.alloc && self.is_not_copy() && self.is_not_null()
    }

    /// `is_allocated` returns true if the [`UniquePointer`] has been
    /// allocated and therefore is no longer a NULL pointer.
    pub fn is_allocated(&self) -> bool {
        let is_allocated = self.is_not_null() && self.alloc;
        is_allocated
    }

    /// `is_written` returns true if the [`UniquePointer`] has been written to
    pub fn is_written(&self) -> bool {
        let is_written = self.is_allocated() && self.written;
        is_written
    }

    /// `is_copy` returns true if a [`UniquePointer`] is a "copy" of
    /// another [`UniquePointer`] in the sense that dropping or
    /// "hard-deallocating" said [`UniquePointer`] does not incur a
    /// double-free.
    pub fn is_copy(&self) -> bool {
        self.is_copy
    }

    /// `alloc` allocates memory in a null `UniquePointer`
    pub fn alloc(&mut self) {
        if self.is_allocated() {
            // warn!("{:#?} is already allocated, force-deallocating now", &self);
            self.dealloc(false);
            return;
        }

        let layout = Layout::new::<T>();
        let mut_ptr = unsafe {
            let ptr = std::alloc::alloc_zeroed(layout);
            if ptr.is_null() {
                std::alloc::handle_alloc_error(layout);
            }
            ptr as *mut T
        };
        self.set_mut_ptr(mut_ptr, false);
        self.alloc = true;
        // step!("self.incr_ref()");
        // self.incr_ref();
    }

    /// `write` allocates memory and writes the given value into the
    /// newly allocated area.
    pub fn write(&mut self, data: T) {
        let orig_addr = UniquePointer::<T>::raw_addr_of_ref(&data);
        self.alloc();

        unsafe {
            self.mut_ptr.write(data);
        }

        self.written = true;
        self.orig_addr = orig_addr;
    }

    /// `read` increments reference count and returns the internal value `T`.
    pub fn read(&self) -> T {
        if !self.is_written() {
            panic!("{:#?} not written", self);
        }
        let mut ptr = self.cast_const();
        unsafe { ptr.read() }
    }

    /// `write_ref_mut` takes a mutable reference to a value and
    /// writes to a `UniquePointer`
    pub fn write_ref_mut(&mut self, data: &mut T) {
        self.write(unsafe {
            let ptr = data as *mut T;
            ptr.read()
        });
    }

    /// `write_ref` takes a read-only reference to a value and
    /// writes to a `UniquePointer`
    pub fn write_ref(&mut self, data: &T) {
        self.write(unsafe {
            let ptr = data as *const T;
            ptr.read()
        });
    }

    /// `cast_mut` is a compatibility API to a raw mut pointer's [`pointer::cast_mut`].
    pub fn cast_mut(&self) -> *mut T {
        if self.is_null() {
            // return std::ptr::null_mut::<T>()
            panic!("{:#?}", self);
        } else {
            self.mut_ptr
        }
    }

    /// `cast_mut` is a compatibility API to a raw const pointer's [`pointer::cast_const`].
    pub fn cast_const(&self) -> *const T {
        if self.is_null() {
            // return std::ptr::null::<T>()
            panic!("{:#?}", self);
        } else {
            self.mut_ptr.cast_const()
        }
    }

    /// `peek_ref` obtains a read-only reference to the value inside
    /// [`UniquePointer`] but does not increment references
    pub fn peek_ref(&self) -> &'c T {
        unsafe { std::mem::transmute::<&T, &'c T>(&*self.cast_const()) }
    }

    /// `peek_mut` obtains a mutable reference to the value inside
    /// [`UniquePointer`] but does not increment references
    pub fn peek_mut(&mut self) -> &'c mut T {
        unsafe { std::mem::transmute::<&mut T, &'c mut T>(&mut *self.mut_ptr) }
    }

    /// `inner_ref` obtains a read-only reference to the value inside
    /// [`UniquePointer`] and increments reference
    pub fn inner_ref<'a>(&self) -> &'a T {
        unsafe { std::mem::transmute::<&'c T, &'a T>(self.peek_ref()) }
    }

    /// `inner_mut` obtains a mutable reference to the value inside
    /// [`UniquePointer`] and increments reference
    pub fn inner_mut<'a>(&mut self) -> &'a mut T {
        // step!("{:#?}", self);
        unsafe { std::mem::transmute::<&'c mut T, &'a mut T>(self.peek_mut()) }
    }

    /// `as_ref` is a compatibility layer to the [`AsRef`] implementation in raw pointers
    pub fn as_ref(&self) -> Option<&'c T> {
        if self.is_written() {
            Some(self.inner_ref())
        } else {
            None
        }
    }

    /// `as_mut` is a compatibility layer to the [`AsMut`] implementation in raw pointers
    pub fn as_mut(&mut self) -> Option<&'c mut T> {
        if self.is_written() {
            Some(self.inner_mut())
        } else {
            None
        }
    }

    /// `dealloc` deallocates a [`UniquePointer`].
    ///
    /// The [`soft`] boolean argument indicates whether the
    /// [`UniquePointer`] should have its reference count decremented or
    /// deallocated immediately.
    ///
    /// During "soft" deallocation (`soft=true`) calls to `dealloc`
    /// only really deallocate memory when the reference gets down to
    /// zero, until then each `dealloc(true)` call simply decrements
    /// the reference count.
    ///
    /// Conversely during "hard" deallocation (`soft=false`) the
    /// UniquePointer in question gets immediately deallocated,
    /// possibly incurring a double-free or causing Undefined
    /// Behavior.
    pub fn dealloc(&mut self, soft: bool) {
        if self.is_null() {
            return;
        }
        if !soft && self.refs > 0 {
            self.decr_ref();
        } else {
            self.free();
        }
    }

    /// `set_mut_ptr` sets the internal raw pointer of a `UniquePointer`.
    ///
    /// Prior to setting the new pointer, it checks whether the
    /// internal pointer is non-null and matches its provenance
    /// address, such that "copies" do not incur a double-free.
    ///
    /// When [`ptr`] is a NULL pointer and the internal pointer of
    /// [`UniquePointer`] in question is NOT NULL, then it is
    /// deallocated prior to setting it to NULL.
    fn set_mut_ptr(&mut self, ptr: *mut T, dealloc: bool) {
        if ptr.is_null() {
            if dealloc && self.can_dealloc() {
                // unsafe {
                //     self.mut_ptr.drop_in_place();
                // }
                self.alloc = false;
                self.written = false;
                // warn!("deallocating {:#?}", self);
                let layout = Layout::new::<T>();
                let mut_ptr = self.mut_ptr;
                unsafe {
                    std::alloc::dealloc(self.mut_ptr as *mut u8, layout);
                };
            }

            self.set_mut_addr(0);
        } else {
            self.set_mut_addr(UniquePointer::<T>::provenance_of_mut_ptr(ptr));
        }
        self.mut_ptr = ptr;
    }

    fn set_mut_addr(&mut self, addr: usize) {
        self.mut_addr = addr;
    }

    /// `free` is internally used by [`dealloc`] when the number of
    /// references gets down to zero in a "soft" deallocation and
    /// immediately in a "hard" deallocation.
    ///
    /// See [`dealloc`] for more information regarding the difference
    /// between "soft" and "hard" deallocation.
    fn free(&mut self) {
        if !self.is_null() {
            self.set_mut_ptr(std::ptr::null_mut::<T>(), false);
            self.refs.dealloc();
        }
        self.alloc = false;
        self.written = false;
    }
}

impl<T: Sized> UniquePointer<T> {
    /// `provenance_of_const_ptr` is a helper method that returns the
    /// address and provenance of a const pointer
    pub fn provenance_of_const_ptr(ptr: *const T) -> usize {
        ptr.expose_provenance()
    }

    /// `provenance_of_mut_ptr` is a helper method that returns the
    /// address and provenance of a mut pointer
    pub fn provenance_of_mut_ptr(ptr: *mut T) -> usize {
        ptr.expose_provenance()
    }

    /// `provenance_of_ref` is a helper method that returns the
    /// address and provenance of a reference
    pub fn provenance_of_ref(ptr: &T) -> usize {
        (&raw const ptr).expose_provenance()
    }

    /// `provenance_of_mut` is a helper method that returns the
    /// address and provenance of a mutable reference
    pub fn provenance_of_mut(mut ptr: &mut T) -> usize {
        (&raw mut ptr).expose_provenance()
    }

    /// `raw_addr_of_const_ptr` is a helper method that returns the
    /// address (as opposed to the provenance) of a const pointer
    pub fn raw_addr_of_const_ptr(ptr: *const T) -> usize {
        ptr.addr()
    }

    /// `raw_addr_of_mut_ptr` is a helper method that returns the
    /// address (as opposed to the provenance) of a mut pointer
    fn raw_addr_of_mut_ptr(ptr: *mut T) -> usize {
        ptr.addr()
    }

    /// `raw_addr_of_ref` is a helper method that returns the address
    /// (as opposed to the provenance) of the underlying `*const`
    /// pointer of a reference
    pub fn raw_addr_of_ref(ptr: &T) -> usize {
        std::ptr::from_ref(ptr).addr()
    }

    /// `raw_addr_of_ref` is a helper method that returns the address
    /// (as opposed to the provenance) of the underlying `*mut`
    /// pointer of a mutable reference
    pub fn raw_addr_of_mut(mut ptr: &mut T) -> usize {
        std::ptr::from_mut(ptr).addr()
    }
}

impl<'c, T: Sized + 'c> UniquePointer<T> {
    /// `meta_mut` is an unsafe method that turns a "self reference"
    /// into a mutable "self reference"
    unsafe fn meta_mut(&'c self) -> &'c mut UniquePointer<T> {
        unsafe {
            let ptr = self.meta_mut_ptr();
            let mut up = &mut *ptr;
            std::mem::transmute::<&mut UniquePointer<T>, &'c mut UniquePointer<T>>(up)
        }
    }

    /// `meta_mut_ptr` is an unsafe method that turns a [`*mut UniquePointer`] from a "self reference"
    unsafe fn meta_mut_ptr(&self) -> *mut UniquePointer<T> {
        let ptr = self as *const UniquePointer<T>;
        unsafe {
            let ptr: *mut UniquePointer<T> =
                std::mem::transmute::<*const UniquePointer<T>, *mut UniquePointer<T>>(ptr);
            ptr
        }
    }
}
#[allow(invalid_reference_casting)]
impl<T: Sized> UniquePointer<T> {
    fn incr_ref(&self) {
        if self.is_null() {
            // panic!("null {:#?}", self);
            return;
        }
        unsafe {
            let ptr = self.meta_mut_ptr();
            let mut up = &mut *ptr;
            up.refs.incr();
        }
    }

    fn decr_ref(&self) {
        if self.refs < 2 {
            panic!("refs {}", self.refs);
            return;
        }
        unsafe {
            let ptr = self.meta_mut_ptr();
            let mut up = &mut *ptr;
            up.refs.decr();
        }
    }
}
impl<T: Sized> Deref for UniquePointer<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.inner_ref()
    }
}

impl<T: Sized> DerefMut for UniquePointer<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.inner_mut()
    }
}

impl<T: Sized> Drop for UniquePointer<T> {
    fn drop(&mut self) {
        // if self.is_written() {
        //     self.dealloc(true);
        // }
    }
}

impl<T: Sized> From<&T> for UniquePointer<T> {
    fn from(data: &T) -> UniquePointer<T> {
        UniquePointer::<T>::from_ref(data)
    }
}
impl<T: Sized> From<&mut T> for UniquePointer<T> {
    fn from(data: &mut T) -> UniquePointer<T> {
        UniquePointer::<T>::from_ref_mut(data)
    }
}
impl<T: Sized> From<T> for UniquePointer<T> {
    fn from(data: T) -> UniquePointer<T> {
        let mut up = UniquePointer::<T>::null();
        up.write(data);
        up
    }
}
/// The [`Clone`] implementation of [`UniquePointer`] is special because
/// it flags cloned values as clones such that a double-free doesn not
/// occur.
impl<T: Sized> Clone for UniquePointer<T> {
    fn clone(&self) -> UniquePointer<T> {
        self.incr_ref();
        let mut clone = UniquePointer::<T>::copy();
        clone.set_mut_ptr(self.mut_ptr, false);
        clone.refs = self.refs.clone();
        clone.alloc = self.alloc;
        clone.written = self.written;
        clone
    }
}

impl<T: Sized> Pointer for UniquePointer<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:016x}", self.addr())
    }
}

impl<T: Sized> Debug for UniquePointer<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            crate::color::reset(
                [
                    crate::color::fg("UniquePointer@", 237),
                    format!("{:016x}", self.addr()),
                    format!("[refs={}]", self.refs),
                    format!("[alloc={}]", self.alloc),
                    format!("[written={}]", self.written),
                    format!("[is_copy={}]", self.is_copy),
                    format!("[orig_addr={:016x}]", self.orig_addr),
                ]
                .join("")
            )
        )
    }
}

impl<T: Deref, S: Deref> PartialEq<UniquePointer<S>> for UniquePointer<T>
where
    T: PartialEq<S::Target>,
{
    fn eq(&self, other: &UniquePointer<S>) -> bool {
        T::eq(self, other)
    }

    fn ne(&self, other: &UniquePointer<S>) -> bool {
        T::ne(self, other)
    }
}

impl<T: Deref<Target: Eq> + Eq + PartialEq<<T as Deref>::Target>> Eq for UniquePointer<T> {}

impl<T: Deref, S: Deref> PartialOrd<UniquePointer<S>> for UniquePointer<T>
where
    T: PartialOrd<S::Target>,
{
    fn partial_cmp(&self, other: &UniquePointer<S>) -> Option<Ordering> {
        T::partial_cmp(self, other)
    }

    fn lt(&self, other: &UniquePointer<S>) -> bool {
        T::lt(self, other)
    }

    fn le(&self, other: &UniquePointer<S>) -> bool {
        T::le(self, other)
    }

    fn gt(&self, other: &UniquePointer<S>) -> bool {
        T::gt(self, other)
    }

    fn ge(&self, other: &UniquePointer<S>) -> bool {
        T::ge(self, other)
    }
}

impl<T: Deref<Target: Ord> + Ord + PartialOrd<<T as Deref>::Target>> Ord for UniquePointer<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        T::cmp(self, other)
    }
}

impl<T: Deref<Target: Hash> + Hash> Hash for UniquePointer<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        T::hash(self, state);
    }
}
