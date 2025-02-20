extern crate alloc;
use alloc::boxed::Box;
use core::{cell::Cell, mem::MaybeUninit, ptr::NonNull};

#[derive(Debug)]
pub(crate) struct Arena<T, const N: usize = 32> {
    curr: Cell<Option<NonNull<Chunk<T, N>>>>,
}

impl<T, const N: usize> Arena<T, N> {
    pub(crate) fn push<'a>(&self, value: T) -> &'a T {
        if let Some(mut curr) = self.curr.get() {
            if let Some(uninit) = {
                let curr: &'a mut Chunk<T, N> = unsafe { curr.as_mut() };
                let uninit = curr.buf.get_mut(curr.len);
                curr.len += 1;
                uninit
            } { uninit.write(value) } else {
                let mut new = Box::into_non_null(Box::new(Chunk::<T, N>::default()));
                // set curr to new
                self.curr.set(Some(new));
                // set new prev to curr
                unsafe { new.as_mut().prev = Some(curr); }
                // push value to new
                // return pushed value
                unsafe { new.as_mut().buf.first_mut().unwrap_unchecked().write(value) }
            }
        } else {
            let mut new = Box::into_non_null(Box::new(Chunk::<T, N>::default()));
            // set curr to new
            self.curr.set(Some(new));
            unsafe { new.as_mut().len += 1; }
            // push value to new
            // return pushed value
            unsafe { new.as_mut().buf.first_mut().unwrap_unchecked().write(value) }
        }
    }
}

impl<T, const N: usize> Default for Arena<T, N> {
    fn default() -> Self {
        assert_ne!(N, 0);
        Self { curr: Cell::new(None) }
    }
}

struct Chunk<T, const N: usize> {
    buf: [MaybeUninit<T>; N],
    len: usize,
    prev: Option<NonNull<Self>>,
}

impl<T, const N: usize> Default for Chunk<T, N> {
    fn default() -> Self {
        Self {
            buf: MaybeUninit::uninit_array::<N>(),
            len: 0,
            prev: None,
        }
    }
}

