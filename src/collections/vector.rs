use std::slice;
use std::ptr::{Unique, NonNull, self};
use std::mem;
use std::ops::{Deref, DerefMut};
use std::heap::{Alloc, Layout, Heap, oom};
use std::ops::{Index, IndexMut};
struct VecPtr<T> {
    ptr: Unique<T>,
    cap: usize,
}

impl<T> VecPtr<T> {
    fn new() -> Self {
        let cap = if mem::size_of::<T>() == 0 {!0} else {0};
        VecPtr{ptr: Unique::empty(), cap: cap}
    }

    fn current_layout(&self) -> Option<Layout> {
        if self.cap == 0 {
            None
        } else {
            Some(Layout::array::<T>(self.cap).unwrap())
        }
    }
    
    fn double_cap(&mut self) {
        unsafe {
            let elem_size = mem::size_of::<T>();
            assert!(elem_size!=0, "cap overflow");
            let curr_layout = self.current_layout();

            let (new_cap, ptr) = match curr_layout {
                Some(layout) => {
                    let cap = self.cap * 2;
                    let nptr = Heap.realloc(NonNull::from(self.ptr).as_opaque(), layout, cap * elem_size);
                    
                    match nptr {
                        Ok(ptr) => (cap, ptr.cast().into()),
                        Err(_) => oom(),
                    }
                },
                None => {
                    let nptr = Heap.alloc_array::<T>(1);

                    match nptr {
                        Ok(ptr) => (1, ptr.into()),
                        Err(_) => oom(),
                    }
                }
            };

            self.ptr = ptr;
            self.cap = new_cap;
        }
    }
}

impl<T> Drop for VecPtr<T> {
    fn drop(&mut self) {
        let elem_size = mem::size_of::<T>();
        if self.cap != 0 && elem_size != 0 {
            if let Some(layout) = self.current_layout() {
                unsafe {Heap.dealloc(NonNull::from(self.ptr).as_opaque(), layout);}
            }
        }
    }
}

pub struct Vector<T> {
    buf: VecPtr<T>,
    len: usize,
}

impl<T> Vector<T> {
    pub fn new() -> Self {
        Vector{buf: VecPtr::new(), len: 0}
    }
    
    pub fn len(&self) -> usize {
        self.len
    }

    fn cap(&self) -> usize {
        self.buf.cap
    }

    fn ptr(&self) -> *mut T { self.buf.ptr.as_ptr() }

    pub fn push(&mut self, elem: T) {
        if self.len == self.cap() {
            self.buf.double_cap();
        }
        unsafe {
            ptr::write(self.ptr().offset(self.len as isize), elem);
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe {
                Some(ptr::read(self.ptr().offset(self.len as isize)))
            }
        }
    }

    pub fn insert(&mut self, index: usize, elem: T) {
        assert!(index <= self.len, "index out of bounds");
        if self.len == self.cap() { self.buf.double_cap(); }
        unsafe {
            if index < self.len {
                ptr::copy(self.ptr().offset(index as isize), self.ptr().offset(index as isize + 1), self.len - index);
            }
            ptr::write(self.ptr().offset(index as isize), elem);
            self.len += 1;
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len, "index out of bounds");
        unsafe {
            self.len -= 1;
            let ret = ptr::read(self.ptr().offset(index as isize));
            ptr::copy(self.ptr().offset(index as isize + 1), self.ptr().offset(index as isize), self.len - index);
            ret
        }
    }
}

impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

impl<T> Deref for Vector<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe {
            slice::from_raw_parts(self.ptr(), self.len)
        }
    }
}

impl<T> DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe {
            slice::from_raw_parts_mut(self.ptr(), self.len)
        }
    }
}

impl<T> Index<usize> for Vector<T> {
    type Output = T;
    #[inline]
    fn index(&self, index: usize) -> &T {
        &(**self)[index]
    }
}

impl<T> IndexMut<usize> for Vector<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut(**self)[index]
    }
}

