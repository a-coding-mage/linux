/* SPDX-License-Identifier: GPL-2.0 */
/*
 * A security context is a set of security attributes
 * associated with each subject and object controlled
 * by the security policy.  Security contexts are
 * externally represented as variable-length strings
 * that can be interpreted by a user or application
 * with an understanding of the security policy.
 * Internally, the security server uses a simple
 * structure.  This structure is private to the
 * security server and can be changed without affecting
 * clients of the security server.
 *
 * Author : Stephen Smalley, <stephen.smalley.work@gmail.com>
 */

/* Dependencies from the original header: ebitmap.h, mls_types.h, security.h. */

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

use crate::ebitmap::ebitmap;
use crate::mls_types::mls_range;

pub type gfp_t = u32;

pub const EINVAL: c_int = 22;
pub const ENOMEM: c_int = 12;

unsafe extern "C" {
    pub static GFP_ATOMIC: gfp_t;

    pub fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    pub fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
    pub fn kstrdup(s: *const c_char, gfp: gfp_t) -> *mut c_char;
    pub fn kfree(objp: *const c_void);

    pub fn ebitmap_cpy(dst: *mut ebitmap, src: *const ebitmap) -> c_int;
    pub fn ebitmap_destroy(e: *mut ebitmap);
    pub fn ebitmap_and(dst: *mut ebitmap, e1: *const ebitmap, e2: *const ebitmap) -> c_int;
    pub fn ebitmap_equal(e1: *const ebitmap, e2: *const ebitmap) -> bool;
}

/*
 * A security context consists of an authenticated user
 * identity, a role, a type and a MLS range.
 */
#[repr(C)]
pub struct context {
    pub user: u32,
    pub role: u32,
    pub type_: u32,
    pub len: u32, /* length of string in bytes */
    pub range: mls_range,
    pub str: *mut c_char, /* string representation if context cannot be mapped. */
}

pub unsafe fn mls_context_init(c: *mut context) {
    unsafe {
        memset(
            ptr::addr_of_mut!((*c).range).cast::<c_void>(),
            0,
            size_of::<mls_range>(),
        );
    }
}

pub unsafe fn mls_context_cpy(dst: *mut context, src: *const context) -> c_int {
    let mut rc: c_int;

    unsafe {
        (*dst).range.level[0].sens = (*src).range.level[0].sens;
        rc = ebitmap_cpy(
            ptr::addr_of_mut!((*dst).range.level[0].cat),
            ptr::addr_of!((*src).range.level[0].cat),
        );
        if rc != 0 {
            return rc;
        }

        (*dst).range.level[1].sens = (*src).range.level[1].sens;
        rc = ebitmap_cpy(
            ptr::addr_of_mut!((*dst).range.level[1].cat),
            ptr::addr_of!((*src).range.level[1].cat),
        );
        if rc != 0 {
            ebitmap_destroy(ptr::addr_of_mut!((*dst).range.level[0].cat));
        }
    }

    rc
}

/*
 * Sets both levels in the MLS range of 'dst' to the low level of 'src'.
 */
pub unsafe fn mls_context_cpy_low(dst: *mut context, src: *const context) -> c_int {
    let mut rc: c_int;

    unsafe {
        (*dst).range.level[0].sens = (*src).range.level[0].sens;
        rc = ebitmap_cpy(
            ptr::addr_of_mut!((*dst).range.level[0].cat),
            ptr::addr_of!((*src).range.level[0].cat),
        );
        if rc != 0 {
            return rc;
        }

        (*dst).range.level[1].sens = (*src).range.level[0].sens;
        rc = ebitmap_cpy(
            ptr::addr_of_mut!((*dst).range.level[1].cat),
            ptr::addr_of!((*src).range.level[0].cat),
        );
        if rc != 0 {
            ebitmap_destroy(ptr::addr_of_mut!((*dst).range.level[0].cat));
        }
    }

    rc
}

/*
 * Sets both levels in the MLS range of 'dst' to the high level of 'src'.
 */
pub unsafe fn mls_context_cpy_high(dst: *mut context, src: *const context) -> c_int {
    let mut rc: c_int;

    unsafe {
        (*dst).range.level[0].sens = (*src).range.level[1].sens;
        rc = ebitmap_cpy(
            ptr::addr_of_mut!((*dst).range.level[0].cat),
            ptr::addr_of!((*src).range.level[1].cat),
        );
        if rc != 0 {
            return rc;
        }

        (*dst).range.level[1].sens = (*src).range.level[1].sens;
        rc = ebitmap_cpy(
            ptr::addr_of_mut!((*dst).range.level[1].cat),
            ptr::addr_of!((*src).range.level[1].cat),
        );
        if rc != 0 {
            ebitmap_destroy(ptr::addr_of_mut!((*dst).range.level[0].cat));
        }
    }

    rc
}

pub unsafe fn mls_context_glblub(
    dst: *mut context,
    c1: *const context,
    c2: *const context,
) -> c_int {
    unsafe {
        let dr: *mut mls_range = ptr::addr_of_mut!((*dst).range);
        let r1: *const mls_range = ptr::addr_of!((*c1).range);
        let r2: *const mls_range = ptr::addr_of!((*c2).range);
        let mut rc: c_int = 0;

        if (*r1).level[1].sens < (*r2).level[0].sens
            || (*r2).level[1].sens < (*r1).level[0].sens
        {
            /* These ranges have no common sensitivities */
            return -EINVAL;
        }

        /* Take the greatest of the low */
        (*dr).level[0].sens = (*r1).level[0].sens.max((*r2).level[0].sens);

        /* Take the least of the high */
        (*dr).level[1].sens = (*r1).level[1].sens.min((*r2).level[1].sens);

        rc = ebitmap_and(
            ptr::addr_of_mut!((*dr).level[0].cat),
            ptr::addr_of!((*r1).level[0].cat),
            ptr::addr_of!((*r2).level[0].cat),
        );
        if rc != 0 {
            return rc;
        }

        rc = ebitmap_and(
            ptr::addr_of_mut!((*dr).level[1].cat),
            ptr::addr_of!((*r1).level[1].cat),
            ptr::addr_of!((*r2).level[1].cat),
        );

        rc
    }
}

pub unsafe fn mls_context_equal(c1: *const context, c2: *const context) -> bool {
    unsafe {
        (*c1).range.level[0].sens == (*c2).range.level[0].sens
            && ebitmap_equal(
                ptr::addr_of!((*c1).range.level[0].cat),
                ptr::addr_of!((*c2).range.level[0].cat),
            )
            && (*c1).range.level[1].sens == (*c2).range.level[1].sens
            && ebitmap_equal(
                ptr::addr_of!((*c1).range.level[1].cat),
                ptr::addr_of!((*c2).range.level[1].cat),
            )
    }
}

pub unsafe fn mls_context_destroy(c: *mut context) {
    unsafe {
        ebitmap_destroy(ptr::addr_of_mut!((*c).range.level[0].cat));
        ebitmap_destroy(ptr::addr_of_mut!((*c).range.level[1].cat));
        mls_context_init(c);
    }
}

pub unsafe fn context_init(c: *mut context) {
    unsafe {
        memset(c.cast::<c_void>(), 0, size_of::<context>());
    }
}

pub unsafe fn context_cpy(dst: *mut context, src: *const context) -> c_int {
    let mut rc: c_int;

    unsafe {
        (*dst).user = (*src).user;
        (*dst).role = (*src).role;
        (*dst).type_ = (*src).type_;
        if !(*src).str.is_null() {
            (*dst).str = kstrdup((*src).str, GFP_ATOMIC);
            if (*dst).str.is_null() {
                return -ENOMEM;
            }
            (*dst).len = (*src).len;
        } else {
            (*dst).str = ptr::null_mut();
            (*dst).len = 0;
        }
        rc = mls_context_cpy(dst, src);
        if rc != 0 {
            kfree((*dst).str.cast::<c_void>());
            (*dst).str = ptr::null_mut();
            (*dst).len = 0;
            return rc;
        }
    }

    0
}

pub unsafe fn context_destroy(c: *mut context) {
    unsafe {
        (*c).type_ = 0;
        (*c).role = (*c).type_;
        (*c).user = (*c).role;
        kfree((*c).str.cast::<c_void>());
        (*c).str = ptr::null_mut();
        (*c).len = 0;
        mls_context_destroy(c);
    }
}

pub unsafe fn context_equal(c1: *const context, c2: *const context) -> bool {
    unsafe {
        if (*c1).len != 0 && (*c2).len != 0 {
            return (*c1).len == (*c2).len && strcmp((*c1).str, (*c2).str) == 0;
        }
        if (*c1).len != 0 || (*c2).len != 0 {
            return false;
        }
        (*c1).user == (*c2).user
            && (*c1).role == (*c2).role
            && (*c1).type_ == (*c2).type_
            && mls_context_equal(c1, c2)
    }
}

unsafe extern "C" {
    pub fn context_compute_hash(c: *const context) -> u32;
}



// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
