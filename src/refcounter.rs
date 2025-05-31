use std::alloc::Layout;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::marker::PhantomData;
use std::ops::{AddAssign, Deref, DerefMut, SubAssign};

/// `RefCounter` is a data-structure designed specifically for
/// internal use in [`UniquePointer`] allowing reference counts to be
/// shared across clones of [`UniquePointer`].
///
/// [`RefCounter`] uses relatively obscure rust techniques under
/// the hood to allow writing in non-mut references in strategic
/// occasions such as incrementing its reference count within its
/// [`Clone`] implementation.
pub struct RefCounter {
    data: *mut usize,
}

impl RefCounter {
    pub fn new() -> RefCounter {
        RefCounter {
            data: std::ptr::null_mut::<usize>(),
        }
    }

    pub fn reset(&mut self) {
        self.write(0);
    }

    pub fn incr(&mut self) {
        self.incr_by(1);
    }

    pub fn incr_by(&mut self, by: usize) {
        self.write(self.read() + by);
    }

    pub fn decr(&mut self) {
        self.decr_by(1);
    }

    pub fn decr_by(&mut self, by: usize) {
        let data = self.read();
        if data >= by {
            self.write(data - by);
        }
    }

    pub fn read(&self) -> usize {
        if self.data.is_null() {
            0
        } else {
            let mut ptr = self.cast_const();
            unsafe { ptr.read() }
        }
    }

    fn alloc(&self) {
        if !self.data.is_null() {
            return;
        }

        let layout = Layout::new::<usize>();
        let ptr = unsafe {
            let ptr = std::alloc::alloc(layout);
            if ptr.is_null() {
                std::alloc::handle_alloc_error(layout);
            }
            ptr as *mut usize
        };
        let mut up = self.meta_mut();
        up.data = ptr;
        up.write(1);
    }

    fn write(&self, data: usize) {
        let mut up = self.meta_mut();
        up.alloc();
        let mut ptr = up.cast_mut();
        unsafe {
            ptr.write(data);
        }
    }

    fn cast_mut(&self) -> *mut usize {
        self.data
    }

    fn cast_const(&self) -> *const usize {
        self.data.cast_const()
    }

    fn inner_ref<'c>(&self) -> &'c usize {
        if self.data.is_null() {
            &0
        } else {
            let ptr = self.cast_const();
            unsafe { std::mem::transmute::<&usize, &'c usize>(&*ptr) }
        }
    }

    fn inner_mut<'c>(&self) -> &'c mut usize {
        if self.data.is_null() {
            panic!("uninitialized");
        }
        let mut ptr = self.cast_mut();
        unsafe { std::mem::transmute::<&mut usize, &'c mut usize>(&mut *ptr) }
    }
}
impl From<usize> for RefCounter {
    fn from(refs: usize) -> RefCounter {
        let mut ref_counter = RefCounter::new();
        ref_counter.write(refs);
        ref_counter
    }
}
impl Deref for RefCounter {
    type Target = usize;

    fn deref(&self) -> &usize {
        self.inner_ref()
    }
}

impl Drop for RefCounter {
    fn drop(&mut self) {
        if self.data.is_null() {
            return;
        }
    }
}

impl Clone for RefCounter {
    fn clone(&self) -> RefCounter {
        let mut clone = RefCounter::new();
        clone.data = self.data;
        clone
    }
}
impl std::fmt::Debug for RefCounter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            crate::color::reset(
                [
                    crate::color::fg("RefCounter@", 231),
                    format!("{:016x}", self.data.addr()),
                    format!("[data={}]", self.read()),
                ]
                .join("")
            )
        )
    }
}
impl std::fmt::Display for RefCounter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.read())
    }
}

impl<'c> RefCounter {
    fn meta_mut(&'c self) -> &'c mut RefCounter {
        let ptr = self.meta_mut_ptr();
        unsafe {
            let mut up = &mut *ptr;
            std::mem::transmute::<&mut RefCounter, &'c mut RefCounter>(up)
        }
    }
}
#[allow(invalid_reference_casting)]
impl RefCounter {
    fn meta_mut_ptr(&self) -> *mut RefCounter {
        let ptr = self as *const RefCounter;
        unsafe {
            let ptr: *mut RefCounter =
                std::mem::transmute::<*const RefCounter, *mut RefCounter>(ptr);
            ptr
        }
    }
}

impl AddAssign<usize> for RefCounter {
    fn add_assign(&mut self, other: usize) {
        self.incr_by(other)
    }
}

impl SubAssign<usize> for RefCounter {
    fn sub_assign(&mut self, other: usize) {
        self.decr_by(other)
    }
}

impl PartialOrd<usize> for RefCounter {
    fn partial_cmp(&self, other: &usize) -> Option<Ordering> {
        self.read().partial_cmp(other)
    }
}

impl PartialEq<usize> for RefCounter {
    fn eq(&self, other: &usize) -> bool {
        self.read().eq(other)
    }
}

impl PartialOrd for RefCounter {
    fn partial_cmp(&self, other: &RefCounter) -> Option<Ordering> {
        self.read().partial_cmp(other.inner_ref())
    }
}

impl Ord for RefCounter {
    fn cmp(&self, other: &RefCounter) -> Ordering {
        self.read().cmp(other.inner_ref())
    }
}

impl PartialEq for RefCounter {
    fn eq(&self, other: &RefCounter) -> bool {
        self.read().eq(other.inner_ref())
    }
}

impl Eq for RefCounter {}

#[cfg(test)]
mod tests {
    use k9::assert_equal;

    use crate::*;
    #[test]
    fn test_refcounter_incr_decr_read() {
        let mut counter = RefCounter::new();
        assert_equal!(counter.read(), 0);
        counter.incr();
        assert_equal!(counter.read(), 1);
        counter.incr();
        assert_equal!(counter.read(), 2);
        {
            let mut clone = counter.clone();
            clone.incr();
            assert_equal!(counter.read(), 3);
            assert_equal!(clone.read(), 3);
        }
        assert_equal!(counter.read(), 3);
        counter.decr();
        assert_equal!(counter.read(), 2);
        counter.decr();
        assert_equal!(counter.read(), 1);
        counter.decr();
        assert_equal!(counter.read(), 0);
        counter.decr();
        assert_equal!(counter.read(), 0);
    }
    #[test]
    fn test_refcounter_deref() {
        let mut counter = RefCounter::new();
        assert_equal!(counter.read(), 0);
        counter.incr();
        assert_equal!(counter.read(), 1);
        counter.incr();
        assert_equal!(counter.read(), 2);
        let refs: usize = *counter;
        assert_equal!(refs, 2);
    }
    #[test]
    fn test_refcounter_add_assign() {
        let mut counter = RefCounter::new();
        assert_equal!(counter.read(), 0);
        counter += 2;
        assert_equal!(counter.read(), 2);
        counter -= 1;
        assert_equal!(counter.read(), 1);
        counter -= 1;
        assert_equal!(counter.read(), 0);
        counter += 1;
        let refs: usize = *counter;
        assert_equal!(refs, 1);
    }
}
