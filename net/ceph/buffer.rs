// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// linux/ceph/ceph_debug.h, linux/module.h, linux/slab.h,
// linux/ceph/buffer.h, linux/ceph/decode.h, and linux/ceph/libceph.h.

use core::ffi::c_void;

extern "C" {
    fn kmalloc(size: usize, gfp: gfp_t) -> *mut c_void;
    fn kvmalloc(size: usize, gfp: gfp_t) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn kvfree(ptr: *mut c_void);
    fn kref_init(kref: *mut kref);
    fn dout(fmt: *const u8, ...);
    fn ceph_decode_32(p: *mut *mut c_void) -> u32;
    fn ceph_decode_copy(p: *mut *mut c_void, dst: *mut c_void, len: usize);
}

pub type gfp_t = u32;
pub const GFP_NOFS: gfp_t = 0;

#[repr(C)]
pub struct kref {
    pub refcount: u32,
}

#[repr(C)]
pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: usize,
}

#[repr(C)]
pub struct ceph_buffer {
    pub kref: kref,
    pub alloc_len: usize,
    pub vec: iovec,
}

#[no_mangle]
pub unsafe extern "C" fn ceph_buffer_new(len: usize, gfp: gfp_t) -> *mut ceph_buffer {
    let b = kmalloc(core::mem::size_of::<ceph_buffer>(), gfp) as *mut ceph_buffer;
    if b.is_null() {
        return core::ptr::null_mut();
    }

    (*b).vec.iov_base = kvmalloc(len, gfp);
    if (*b).vec.iov_base.is_null() {
        kfree(b as *mut c_void);
        return core::ptr::null_mut();
    }

    kref_init(&mut (*b).kref);
    (*b).alloc_len = len;
    (*b).vec.iov_len = len;
    dout(b"buffer_new %p\n\0".as_ptr(), b);
    b
}

#[no_mangle]
pub unsafe extern "C" fn ceph_buffer_release(kref: *mut kref) {
    let b = (kref as *mut u8).sub(core::mem::offset_of!(ceph_buffer, kref))
        as *mut ceph_buffer;

    dout(b"buffer_release %p\n\0".as_ptr(), b);
    kvfree((*b).vec.iov_base);
    kfree(b as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn ceph_decode_buffer(
    b: *mut *mut ceph_buffer,
    p: *mut *mut c_void,
    end: *mut c_void,
) -> i32 {
    let len: usize;

    if (*p as *mut u8).add(core::mem::size_of::<u32>()) > end as *mut u8 {
        return -22;
    }
    len = ceph_decode_32(p) as usize;
    dout(b"decode_buffer len %d\n\0".as_ptr(), len as i32);
    if (*p as *mut u8).add(len) > end as *mut u8 {
        return -22;
    }
    *b = ceph_buffer_new(len, GFP_NOFS);
    if (*b).is_null() {
        return -12;
    }
    ceph_decode_copy(p, (**b).vec.iov_base, len);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
