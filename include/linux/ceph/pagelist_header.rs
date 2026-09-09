/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding kernel headers:
// asm/byteorder.h, linux/refcount.h, linux/list.h, linux/types.h

use core::ffi::c_void;

#[repr(C)]
pub struct ceph_pagelist {
    pub head: list_head,
    pub mapped_tail: *mut c_void,
    pub length: usize,
    pub room: usize,
    pub free_list: list_head,
    pub num_pages_free: usize,
    pub refcnt: refcount_t,
}

extern "C" {
    pub fn ceph_pagelist_alloc(gfp_flags: gfp_t) -> *mut ceph_pagelist;
    pub fn ceph_pagelist_release(pl: *mut ceph_pagelist);
    pub fn ceph_pagelist_append(pl: *mut ceph_pagelist, d: *const c_void, l: usize) -> i32;
    pub fn ceph_pagelist_reserve(pl: *mut ceph_pagelist, space: usize) -> i32;
    pub fn ceph_pagelist_free_reserve(pl: *mut ceph_pagelist) -> i32;
}

#[inline]
pub unsafe fn ceph_pagelist_encode_64(pl: *mut ceph_pagelist, v: u64) -> i32 {
    let ev: __le64 = v.to_le();
    ceph_pagelist_append(pl, &ev as *const __le64 as *const c_void, core::mem::size_of::<__le64>())
}

#[inline]
pub unsafe fn ceph_pagelist_encode_32(pl: *mut ceph_pagelist, v: u32) -> i32 {
    let ev: __le32 = v.to_le();
    ceph_pagelist_append(pl, &ev as *const __le32 as *const c_void, core::mem::size_of::<__le32>())
}

#[inline]
pub unsafe fn ceph_pagelist_encode_16(pl: *mut ceph_pagelist, v: u16) -> i32 {
    let ev: __le16 = v.to_le();
    ceph_pagelist_append(pl, &ev as *const __le16 as *const c_void, core::mem::size_of::<__le16>())
}

#[inline]
pub unsafe fn ceph_pagelist_encode_8(pl: *mut ceph_pagelist, v: u8) -> i32 {
    ceph_pagelist_append(pl, &v as *const u8 as *const c_void, 1)
}

#[inline]
pub unsafe fn ceph_pagelist_encode_string(
    pl: *mut ceph_pagelist,
    s: *mut i8,
    len: u32,
) -> i32 {
    let ret = ceph_pagelist_encode_32(pl, len);
    if ret != 0 {
        return ret;
    }
    if len != 0 {
        return ceph_pagelist_append(pl, s as *const c_void, len as usize);
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
