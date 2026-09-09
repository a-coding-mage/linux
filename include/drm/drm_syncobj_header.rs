/*
 * Copyright © 2017 Red Hat
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the next
 * paragraph) shall be included in all copies or substantial portions of the
 * Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
 * IN THE SOFTWARE.
 *
 * Authors:
 *
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct kref {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dma_fence {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dma_fence_chain {
    _private: [u8; 0],
}
#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}
#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct file {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_file {
    _private: [u8; 0],
}

pub type u32 = ::core::ffi::c_uint;
pub type u64 = ::core::ffi::c_ulonglong;

/**
 * struct drm_syncobj - sync object.
 *
 * This structure defines a generic sync object which wraps a &dma_fence.
 */
#[repr(C)]
pub struct drm_syncobj {
    /** @refcount: Reference count of this object. */
    pub refcount: kref,
    /**
     * @fence:
     * NULL or a pointer to the fence bound to this object.
     *
     * This field should not be used directly. Use drm_syncobj_fence_get()
     * and drm_syncobj_replace_fence() instead.
     */
    pub fence: *mut dma_fence,
    /** @cb_list: List of callbacks to call when the &fence gets replaced. */
    pub cb_list: list_head,
    /** @ev_fd_list: List of registered eventfd. */
    pub ev_fd_list: list_head,
    /** @lock: Protects &cb_list and &ev_fd_list, and write-locks &fence. */
    pub lock: spinlock_t,
    /** @file: A file backing for this syncobj. */
    pub file: *mut file,
}

extern "C" {
    pub fn drm_syncobj_free(kref: *mut kref);
    fn kref_get(kref: *mut kref);
    fn kref_put(kref: *mut kref, release: unsafe extern "C" fn(*mut kref));
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn dma_fence_get_rcu_safe(fence: *mut *mut dma_fence) -> *mut dma_fence;

    pub fn drm_syncobj_find(file_private: *mut drm_file, handle: u32) -> *mut drm_syncobj;
    pub fn drm_syncobj_add_point(
        syncobj: *mut drm_syncobj,
        chain: *mut dma_fence_chain,
        fence: *mut dma_fence,
        point: u64,
    );
    pub fn drm_syncobj_replace_fence(syncobj: *mut drm_syncobj, fence: *mut dma_fence);
    pub fn drm_syncobj_find_fence(
        file_private: *mut drm_file,
        handle: u32,
        point: u64,
        flags: u64,
        fence: *mut *mut dma_fence,
    ) -> ::core::ffi::c_int;
    pub fn drm_syncobj_create(
        out_syncobj: *mut *mut drm_syncobj,
        flags: u32,
        fence: *mut dma_fence,
    ) -> ::core::ffi::c_int;
    pub fn drm_syncobj_get_handle(
        file_private: *mut drm_file,
        syncobj: *mut drm_syncobj,
        handle: *mut u32,
    ) -> ::core::ffi::c_int;
    pub fn drm_syncobj_get_fd(syncobj: *mut drm_syncobj, p_fd: *mut ::core::ffi::c_int)
        -> ::core::ffi::c_int;
}

/** drm_syncobj_get - acquire a syncobj reference */
#[inline]
pub unsafe fn drm_syncobj_get(obj: *mut drm_syncobj) {
    kref_get(&mut (*obj).refcount);
}

/** drm_syncobj_put - release a reference to a sync object. */
#[inline]
pub unsafe fn drm_syncobj_put(obj: *mut drm_syncobj) {
    kref_put(&mut (*obj).refcount, drm_syncobj_free);
}

/** drm_syncobj_fence_get - get a reference to a fence in a sync object */
#[inline]
pub unsafe fn drm_syncobj_fence_get(syncobj: *mut drm_syncobj) -> *mut dma_fence {
    let fence: *mut dma_fence;
    rcu_read_lock();
    fence = dma_fence_get_rcu_safe(&mut (*syncobj).fence);
    rcu_read_unlock();
    fence
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
