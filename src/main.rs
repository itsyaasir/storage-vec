use std::{
    alloc::{GlobalAlloc, Layout, System},
    marker::PhantomData,
    ops::Deref,
};

use std::alloc::{alloc, dealloc};

#[derive(Debug, Clone)]
pub struct Storage<T> {
    pub data: *mut T,
    pub len: usize,
    pub _marker: PhantomData<T>,
}

#[allow(dead_code)]
impl<T: std::fmt::Debug> Storage<T> {
    pub fn new() -> Self {
        Storage {
            data: std::ptr::null_mut(),
            len: 0,
            _marker: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub unsafe fn add(&mut self, value: T) {
        let size = std::mem::size_of::<T>();
        let capacity = self.len * size;
        let layout = Layout::from_size_align(capacity + size, size).unwrap();
        let byte_ptr = alloc(layout);

        if self.data.is_null() {
            self.data = byte_ptr as *mut T;
        } else {
            let old_data = self.data;
            self.data = byte_ptr as *mut T;
            std::ptr::copy_nonoverlapping(old_data, self.data, self.len);
            dealloc(old_data as *mut u8, layout);
        }

        self.len += 1;

        let data = self.data.offset(self.len as isize - 1);
        std::ptr::write(data, value);
    }

    pub unsafe fn remove(&mut self) -> T {
        let size = std::mem::size_of::<T>();
        let capacity = self.len * size;
        let layout = Layout::from_size_align(capacity + size, size).unwrap();
        let byte_ptr = alloc(layout);
        let data = self.data.offset(self.len as isize - 1);
        let value = std::ptr::read(data);
        dealloc(self.data as *mut u8, layout);
        self.data = byte_ptr as *mut T;
        self.len -= 1;
        value
    }

    pub unsafe fn read(&self) {
        let data = self.data.offset(self.len as isize - 1);
        for i in 0..self.len {
            let value = std::ptr::read(data.offset(-(i as isize)));
            println!("{:?}", value);
        }
    }
}

impl<T> Drop for Storage<T> {
    fn drop(&mut self) {
        if !self.data.is_null() {
            unsafe {
                System.dealloc(
                    self.data as *mut u8,
                    Layout::from_size_align(self.len * std::mem::size_of::<T>(), 1).unwrap(),
                );
            }
        }
    }
}

impl<T> Deref for Storage<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.data, self.len) }
    }
}

fn main() {
    unsafe {
        let storage = storage![1, 2, 3];

        storage.iter().for_each(|value| {
            println!("{:?}", value);
        });
    }
}

#[macro_export]
macro_rules! storage {
    ($($x:expr),*) => {
        {
            let mut storage = Storage::new();
            $(
                storage.add($x);
            )*
            storage
        }
    };
}
