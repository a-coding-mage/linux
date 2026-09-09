/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the C header.  The following types and functions are
// supplied by the corresponding kernel dependencies.

/*
 * a simple reference counted buffer.
 *
 * use kmalloc for smaller sizes, vmalloc for larger sizes.
 */
#[repr(C)]
pub struct ceph_buffer {
    pub kref: kref,
    pub vec: kvec,
    pub alloc_len: size_t,
}

extern "C" {
    pub fn ceph_buffer_new(len: size_t, gfp: gfp_t) -> *mut ceph_buffer;
    pub fn ceph_buffer_release(kref: *mut kref);
}

#[inline]
pub unsafe fn ceph_buffer_get(b: *mut ceph_buffer) -> *mut ceph_buffer {
    kref_get(&mut (*b).kref);
    b
}

#[inline]
pub unsafe fn ceph_buffer_put(b: *mut ceph_buffer) {
    if !b.is_null() {
        kref_put(&mut (*b).kref, ceph_buffer_release);
    }
}

extern "C" {
    pub fn ceph_decode_buffer(
        b: *mut *mut ceph_buffer,
        p: *mut *mut core::ffi::c_void,
        end: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
