/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * CALIPSO - Common Architecture Label IPv6 Security Option
 *
 * This is an implementation of the CALIPSO protocol as specified in
 * RFC 5570.
 *
 * Authors: Paul Moore <paul@paul-moore.com>
 *          Huw Davies <huw@codeweavers.com>
 */

/*
 * (c) Copyright Hewlett-Packard Development Company, L.P., 2006
 * (c) Copyright Huw Davies <huw@codeweavers.com>, 2015
 */

/* C header dependencies: linux/types.h, linux/rcupdate.h, linux/list.h,
 * linux/net.h, linux/skbuff.h, net/netlabel.h, net/request_sock.h,
 * linux/refcount.h, and linux/unaligned.h.
 */

/* known doi values */
pub const CALIPSO_DOI_UNKNOWN: u32 = 0x00000000;

/* doi mapping types */
pub const CALIPSO_MAP_UNKNOWN: u32 = 0;
pub const CALIPSO_MAP_PASS: u32 = 2;

/*
 * CALIPSO DOI definitions
 */

/* DOI definition struct */
#[repr(C)]
pub struct calipso_doi {
    pub doi: u32,
    pub type_: u32,

    pub refcount: refcount_t,
    pub list: list_head,
    pub rcu: rcu_head,
}

/*
 * Sysctl Variables
 */
extern "C" {
    pub static mut calipso_cache_enabled: ::core::ffi::c_int;
    pub static mut calipso_cache_bucketsize: ::core::ffi::c_int;
}

/* CONFIG_NETLABEL is a build-time condition; declarations and inline
 * fallbacks are preserved below using Rust cfg conditional compilation.
 */
#[cfg(CONFIG_NETLABEL)]
extern "C" {
    pub fn calipso_init() -> ::core::ffi::c_int;
    pub fn calipso_exit();
    pub fn calipso_validate(
        skb: *const sk_buff,
        option: *const ::core::ffi::c_uchar,
    ) -> bool;
}

#[cfg(not(CONFIG_NETLABEL))]
#[inline]
pub fn calipso_init() -> ::core::ffi::c_int {
    0
}

#[cfg(not(CONFIG_NETLABEL))]
#[inline]
pub fn calipso_exit() {}

#[cfg(not(CONFIG_NETLABEL))]
#[inline]
pub fn calipso_validate(
    _skb: *const sk_buff,
    _option: *const ::core::ffi::c_uchar,
) -> bool {
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
