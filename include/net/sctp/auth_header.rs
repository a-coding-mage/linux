/* SPDX-License-Identifier: GPL-2.0-or-later */
/* SCTP kernel implementation
 * (C) Copyright 2007 Hewlett-Packard Development Company, L.P.
 *
 * This file is part of the SCTP kernel implementation
 *
 * Please send any bug reports or fixes you make to the
 * email address(es):
 *    lksctp developers <linux-sctp@vger.kernel.org>
 *
 * Written or modified by:
 *   Vlad Yasevich     <vladislav.yasevich@hp.com>
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than redefined.

#[repr(C)]
pub struct sctp_hmac {
    pub hmac_id: __u16, // one of SCTP_AUTH_HMAC_ID_*
    pub hmac_len: __u16, // length of the HMAC value in bytes
}

// Generic structure containing authentication bytes used as keying material.
#[repr(C)]
pub struct sctp_auth_bytes {
    pub refcnt: refcount_t,
    pub len: __u32,
    pub data: [__u8; 0],
}

// Definition for a shared key, whether endpoint or association.
#[repr(C)]
pub struct sctp_shared_key {
    pub key_list: list_head,
    pub key: *mut sctp_auth_bytes,
    pub refcnt: refcount_t,
    pub key_id: __u16,
    pub deactivated: __u8,
}

// C macros retained as Rust macro equivalents; list traversal is supplied by
// the surrounding kernel translation.
#[macro_export]
macro_rules! key_for_each {
    ($key:expr, $list_head:expr) => {
        list_for_each_entry!($key, $list_head, key_list)
    };
}

#[macro_export]
macro_rules! key_for_each_safe {
    ($key:expr, $tmp:expr, $list_head:expr) => {
        list_for_each_entry_safe!($key, $tmp, $list_head, key_list)
    };
}

#[inline]
pub unsafe fn sctp_auth_key_hold(key: *mut sctp_auth_bytes) {
    if key.is_null() {
        return;
    }

    refcount_inc!(&mut (*key).refcnt);
}

extern "C" {
    pub fn sctp_auth_key_put(key: *mut sctp_auth_bytes);
    pub fn sctp_auth_shkey_create(key_id: __u16, gfp: gfp_t) -> *mut sctp_shared_key;
    pub fn sctp_auth_destroy_keys(keys: *mut list_head);
    pub fn sctp_auth_asoc_init_active_key(asoc: *mut sctp_association, gfp: gfp_t) -> c_int;
    pub fn sctp_auth_get_shkey(asoc: *const sctp_association, key_id: __u16) -> *mut sctp_shared_key;
    pub fn sctp_auth_asoc_copy_shkeys(ep: *const sctp_endpoint, asoc: *mut sctp_association, gfp: gfp_t) -> c_int;
    pub fn sctp_auth_verify_cookie_params(ep: *const sctp_endpoint, cookie: *const sctp_cookie) -> bool;
    pub fn sctp_auth_get_hmac(hmac_id: __u16) -> *const sctp_hmac;
    pub fn sctp_auth_asoc_get_hmac(asoc: *const sctp_association) -> *const sctp_hmac;
    pub fn sctp_auth_asoc_set_default_hmac(asoc: *mut sctp_association, hmacs: *mut sctp_hmac_algo_param);
    pub fn sctp_auth_asoc_verify_hmac_id(asoc: *const sctp_association, hmac_id: __be16) -> c_int;
    pub fn sctp_auth_send_cid(chunk: sctp_cid, asoc: *const sctp_association) -> c_int;
    pub fn sctp_auth_recv_cid(chunk: sctp_cid, asoc: *const sctp_association) -> c_int;
    pub fn sctp_auth_calculate_hmac(asoc: *const sctp_association, skb: *mut sk_buff,
        auth: *mut sctp_auth_chunk, ep_key: *mut sctp_shared_key, gfp: gfp_t) -> c_int;
    pub fn sctp_auth_shkey_release(sh_key: *mut sctp_shared_key);
    pub fn sctp_auth_shkey_hold(sh_key: *mut sctp_shared_key);

    // API Helpers
    pub fn sctp_auth_ep_add_chunkid(ep: *mut sctp_endpoint, chunk_id: __u8) -> c_int;
    pub fn sctp_auth_ep_set_hmacs(ep: *mut sctp_endpoint, hmacs: *mut sctp_hmacalgo) -> c_int;
    pub fn sctp_auth_set_key(ep: *mut sctp_endpoint, asoc: *mut sctp_association,
        auth_key: *mut sctp_authkey) -> c_int;
    pub fn sctp_auth_set_active_key(ep: *mut sctp_endpoint, asoc: *mut sctp_association,
        key_id: __u16) -> c_int;
    pub fn sctp_auth_del_key_id(ep: *mut sctp_endpoint, asoc: *mut sctp_association,
        key_id: __u16) -> c_int;
    pub fn sctp_auth_deact_key_id(ep: *mut sctp_endpoint, asoc: *mut sctp_association,
        key_id: __u16) -> c_int;
    pub fn sctp_auth_init(ep: *mut sctp_endpoint, gfp: gfp_t) -> c_int;
    pub fn sctp_auth_free(ep: *mut sctp_endpoint);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
