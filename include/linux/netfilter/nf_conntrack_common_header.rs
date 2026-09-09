/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux Rust translations:
// linux/refcount.h
// uapi/linux/netfilter/nf_conntrack_common.h

#[repr(C)]
pub struct ip_conntrack_stat {
    pub found: ::core::ffi::c_uint,
    pub invalid: ::core::ffi::c_uint,
    pub insert: ::core::ffi::c_uint,
    pub insert_failed: ::core::ffi::c_uint,
    pub clash_resolve: ::core::ffi::c_uint,
    pub drop: ::core::ffi::c_uint,
    pub early_drop: ::core::ffi::c_uint,
    pub error: ::core::ffi::c_uint,
    pub expect_new: ::core::ffi::c_uint,
    pub expect_create: ::core::ffi::c_uint,
    pub expect_delete: ::core::ffi::c_uint,
    pub search_restart: ::core::ffi::c_uint,
    pub chaintoolong: ::core::ffi::c_uint,
}

pub const NFCT_INFOMASK: ::core::ffi::c_ulong = 7;
pub const NFCT_PTRMASK: ::core::ffi::c_ulong = !NFCT_INFOMASK;

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
