// For benchmarks:
// * Make sure it's a library (not just a main.rs but a lib.rs)
// * Benchmarks imports your crate as an external crate, meaning `use storage_vec::*` rather than `create::*`

use std::alloc::Layout;

use std::alloc::{alloc, dealloc};

#[derive(Debug, Clone)]
pub struct Storage<T> {
    pub byte_ptr: *mut T,
    pub len: usize,
    pub cap: usize,
    layout: Layout,
}

#[allow(dead_code)]
impl<T> Storage<T> {
    pub fn with_capacity(cap: usize) -> Self {
        let size = std::mem::size_of::<T>();
        let layout = Layout::from_size_align(cap, size.next_power_of_two()).unwrap();
        let byte_ptr = unsafe { alloc(layout) } as *mut T;

        Storage {
            byte_ptr,
            len: 0,
            cap,
            layout,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, pos: usize) -> Option<&T> {
        if pos >= self.len {
            return None;
        }

        let ptr = unsafe { self.byte_ptr.add(pos) };
        Some(unsafe { &*ptr })
    }

    pub fn add(&mut self, value: T) {
        unsafe { std::ptr::write(self.byte_ptr.add(self.len), value) };
        self.len += 1;

        if self.len > self.cap {
            panic!("this should allocate and move all the values to the new ptr");
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        let pos = self.len - 1;
        self.len -= 1;

        let inst = unsafe { self.byte_ptr.add(pos).read() };
        Some(inst)
    }

    pub fn iter_this_thing(&self) -> Iter<'_, T> {
        Iter {
            inner: self,
            offset: 0,
        }
    }
}

impl<T> Drop for Storage<T> {
    fn drop(&mut self) {
        unsafe {
            while self.pop().is_some() {}
            dealloc(self.byte_ptr as *mut u8, self.layout);
        }
    }
}

pub struct Iter<'storage, T> {
    inner: &'storage Storage<T>,
    offset: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset == self.inner.len {
            return None;
        }

        let ret = self.inner.get(self.offset)?;
        self.offset += 1;

        Some(ret)
    }
}
