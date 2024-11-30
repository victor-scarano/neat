extern crate alloc;
use core::{alloc::{Allocator, Layout, AllocError}, ptr::NonNull};
use alloc::rc::Rc;

#[derive(Clone, Debug)]
pub struct Bump(Rc<bumpalo::Bump>);

impl Bump {
    pub fn new() -> Self {
        Self(Rc::new(bumpalo::Bump::new()))
    }
}

unsafe impl Allocator for Bump {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        self.0.as_ref().allocate(layout)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        self.0.as_ref().deallocate(ptr, layout);
    }

    fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        self.0.as_ref().allocate_zeroed(layout)
    }

    unsafe fn grow(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout
    ) -> Result<NonNull<[u8]>, AllocError> {
        self.0.as_ref().grow(ptr, old_layout, new_layout)
    }

    unsafe fn grow_zeroed(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout
    ) -> Result<NonNull<[u8]>, AllocError> {
        self.0.as_ref().grow_zeroed(ptr, old_layout, new_layout)
    }

    unsafe fn shrink(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        self.0.as_ref().shrink(ptr, old_layout, new_layout)
    }

    fn by_ref(&self) -> &Self
    where
        Self: Sized
    {
        self
    }
}

