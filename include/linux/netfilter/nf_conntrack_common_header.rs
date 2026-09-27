/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux Rust translations:
// linux/refcount.h
// uapi/linux/netfilter/nf_conntrack_common.h

#[repr(C)]
pub struct ip_conntrack_stat {
    pub found: ::kernel::ffi::c_uint,
    pub invalid: ::kernel::ffi::c_uint,
    pub insert: ::kernel::ffi::c_uint,
    pub insert_failed: ::kernel::ffi::c_uint,
    pub clash_resolve: ::kernel::ffi::c_uint,
    pub drop: ::kernel::ffi::c_uint,
    pub early_drop: ::kernel::ffi::c_uint,
    pub error: ::kernel::ffi::c_uint,
    pub expect_new: ::kernel::ffi::c_uint,
    pub expect_create: ::kernel::ffi::c_uint,
    pub expect_delete: ::kernel::ffi::c_uint,
    pub search_restart: ::kernel::ffi::c_uint,
    pub chaintoolong: ::kernel::ffi::c_uint,
}

pub const NFCT_INFOMASK: ::kernel::ffi::c_ulong = 7;
pub const NFCT_PTRMASK: ::kernel::ffi::c_ulong = !NFCT_INFOMASK;

#[repr(C)]
pub struct nf_conntrack {
    pub r#use: refcount_t,
}

unsafe extern "C" {
    pub fn nf_conntrack_destroy(nfct: *mut nf_conntrack);
}

/* like nf_ct_put, but without module dependency on nf_conntrack */
#[inline]
pub unsafe fn nf_conntrack_put(nfct: *mut nf_conntrack) {
    if !nfct.is_null() && refcount_dec_and_test(&mut (*nfct).r#use) {
        nf_conntrack_destroy(nfct);
    }
}

#[inline]
pub unsafe fn nf_conntrack_get(nfct: *mut nf_conntrack) {
    if !nfct.is_null() {
        refcount_inc(&mut (*nfct).r#use);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
