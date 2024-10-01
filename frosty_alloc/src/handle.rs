use std::ptr::NonNull;

use crate::{frosty_box::FrostyBox, FrostyAllocatable};

/*  What is up with all the pointers?
 *      1) FrostyBox<T>
 *      2) InterimPtr<T>
 *      3) ObjectHandle(Mut)<T>
 *      4) DataAccess(Mut)<T>
 *
 *  FrostyBox<T>
 *      This isn't really a pointer, but a bundling of some [T]
 *      with a semaphore to control access. This is the baseline
 *      used by all other pointers even though they use a ref
 *      and mut interface.
 *
 *  InterimPtr<T>
 *      This exists between the [FrostyBox<T>] and [ObjectHandle<T>].
 *      Since [Query]s exist outside the scope of [Allocator], there
 *      needs to be some way for information about a [FrostyBox<T>]
 *      to exist after the data has been free'd. That is what
 *      [InterimPtr<T>] is for.
 *
 *  ObjectHandle<T>
 *      This is a ptr held by a [Query] or a {Component} to access
 *      some [FrostyBox<T>]
 *
 *  DataAccess<T>
 *      This is a nice ptr interface that automatically locks and
 *      unlocks a [FrostyBox<T>] as it enters and leaves scope.
 *      It is returned by ObjectHandle<T> and should not be stored
 *      in a {Component}
 *
 *
 *
 *      ------------------------------
 *      | System    --------------   |
 *      |           | DataAccess |   |
 *      |           --------------   |
 *      ----------------|-------------
 *                      |
 *              --------|----------------------
 *              | Query |                     |
 *              |       |  ----------------   |
 *              |       |  | ObjectHandle |   |
 *              |       |  |--------------|   |
 *              |       -> | ObjectHandle |-| |
 *              |          ---------------- | |
 *              ----------------------------|--
 *                                          |
 *                        ------------------|--------------------------------
 *                        | Allocator       |                                |
 *                        |                 V                                |
 *                        |           --------------                        |
 *                        |           | InterimPtr |                        |
 *                        |           --------------                        |
 *                        |                  |                              |
 *                        |                  V                              |
 *                        |   -------------------------------------------   |
 *                        |   | FrostyBox | FrostyBox | FrostyBox | ... |   |
 *                        |   -------------------------------------------   |
 *`                       ---------------------------------------------------
 */

// An [ObjectHandle<T>] and a [ObjectHandleMut<T>] are both
// interfaces that allow threads to safely interact with
// [FrostyBox<T>]s stored in the [Allocator]. The underlying
// data stored in each handle is the same, but the mut is
// used for code distinction
pub struct ObjectHandle<T: FrostyAllocatable> {
    ptr: NonNull<FrostyBox<T>>,
}

impl<T: FrostyAllocatable> ObjectHandle<T> {
    pub fn as_ref(&mut self, thread: u32) -> &T {
        let ptr = unsafe { self.ptr.as_mut() };
        ptr.get_access(thread);
        ptr.get_ref()
    }

    pub fn drop_ref(&mut self, thread: u32) {
        unsafe { self.ptr.as_mut().drop_read_access(thread) }
    }
}

pub struct ObjectHandleMut<T: FrostyAllocatable> {
    ptr: NonNull<FrostyBox<T>>,
}

impl<T: FrostyAllocatable> ObjectHandleMut<T> {
    pub fn as_ref(&mut self, thread: u32) -> &T {
        let ptr = unsafe { self.ptr.as_mut() };
        ptr.get_access(thread);
        ptr.get_ref()
    }

    pub fn drop_ref(&mut self, thread: u32) {
        unsafe { self.ptr.as_mut().drop_read_access(thread) }
    }

    pub fn as_mut(&mut self, thread: u32) -> &mut T {
        let ptr = unsafe { self.ptr.as_mut() };
        ptr.get_access_mut(thread);
        ptr.get_mut()
    }

    pub fn drop_mut(&mut self, thread: u32) {
        unsafe { self.ptr.as_mut().drop_write_access() }
    }
}
