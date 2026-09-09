/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This header is used to share core functionality between the
 * standalone connection tracking module, and the compatibility layer's use
 * of connection tracking.
 *
 * 16 Dec 2003: Yasuyuki Kozakai @USAGI <yasuyuki.kozakai@toshiba.co.jp>
 *	- generalize L3 protocol dependent part.
 *
 * Derived from include/linux/netfiter_ipv4/ip_conntrack_core.h
 */

// C header guard: _NF_CONNTRACK_CORE_H
// Dependencies are supplied by the surrounding translation unit:
// linux/netfilter.h, nf_conntrack.h, nf_conntrack_ecache.h, and
// nf_conntrack_l4proto.h.

/* This header is used to share core functionality between the
   standalone connection tracking module, and the compatibility layer's use
   of connection tracking. */

extern "C" {
    pub fn nf_conntrack_in(
        skb: *mut sk_buff,
        state: *const nf_hook_state,
    ) -> ::core::ffi::c_uint;

    pub fn nf_conntrack_init_net(net: *mut net) -> ::core::ffi::c_int;
    pub fn nf_conntrack_cleanup_net(net: *mut net);
    pub fn nf_conntrack_cleanup_net_list(net_exit_list: *mut list_head);

    pub fn nf_conntrack_proto_pernet_init(net: *mut net);

    pub fn nf_conntrack_proto_init() -> ::core::ffi::c_int;
    pub fn nf_conntrack_proto_fini();

    pub fn nf_conntrack_init_start() -> ::core::ffi::c_int;
    pub fn nf_conntrack_cleanup_start();

    pub fn nf_conntrack_init_end();
    pub fn nf_conntrack_cleanup_end();

    pub fn nf_ct_invert_tuple(
        inverse: *mut nf_conntrack_tuple,
        orig: *const nf_conntrack_tuple,
    ) -> bool;

    /* Find a connection corresponding to a tuple. */
    pub fn nf_conntrack_find_get(
        net: *mut net,
        zone: *const nf_conntrack_zone,
        tuple: *const nf_conntrack_tuple,
    ) -> *mut nf_conntrack_tuple_hash;

    pub fn __nf_conntrack_confirm(skb: *mut sk_buff) -> ::core::ffi::c_int;

    pub fn nf_confirm(
        priv_: *mut ::core::ffi::c_void,
        skb: *mut sk_buff,
        state: *const nf_hook_state,
    ) -> ::core::ffi::c_uint;

    pub fn print_tuple(
        s: *mut seq_file,
        tuple: *const nf_conntrack_tuple,
        proto: *const nf_conntrack_l4proto,
    );

    pub static mut nf_conntrack_locks: [spinlock_t; CONNTRACK_LOCKS];
    pub fn nf_conntrack_lock(lock: *mut spinlock_t);

    pub static mut nf_conntrack_expect_lock: spinlock_t;

    pub fn lockdep_assert_held(lock: *const spinlock_t);

    pub fn __nf_ct_change_timeout(
        ct: *mut nf_conn,
        cta_timeout: u64,
    ) -> ::core::ffi::c_int;
    pub fn __nf_ct_change_status(
        ct: *mut nf_conn,
        on: ::core::ffi::c_ulong,
        off: ::core::ffi::c_ulong,
    );
    pub fn nf_ct_change_status_common(
        ct: *mut nf_conn,
        status: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
}

pub const CONNTRACK_LOCKS: usize = 1024;

/* Confirm a connection: returns NF_DROP if packet must be dropped. */
#[inline]
pub unsafe fn nf_conntrack_confirm(skb: *mut sk_buff) -> ::core::ffi::c_int {
    let mut ct: *mut nf_conn = skb_nfct(skb) as *mut nf_conn;
    let mut ret: ::core::ffi::c_int = NF_ACCEPT;

    if !ct.is_null() {
        if !nf_ct_is_confirmed(ct) {
            ret = __nf_conntrack_confirm(skb);

            if ret == NF_ACCEPT {
                ct = skb_nfct(skb) as *mut nf_conn;
            }
        }

        if ret == NF_ACCEPT && nf_ct_ecache_exist(ct) {
            nf_ct_deliver_cached_events(ct);
        }
    }
    ret
}

#[inline]
pub unsafe fn lockdep_nfct_expect_lock_held() {
    lockdep_assert_held(&raw const nf_conntrack_expect_lock);
}

/* ctnetlink code shared by both ctnetlink and nf_conntrack_bpf */

#[inline]
pub unsafe fn __nf_ct_set_timeout(ct: *mut nf_conn, mut timeout: u64) {
    if timeout > INT_MAX as u64 {
        timeout = INT_MAX as u64;
    }

    if nf_ct_is_confirmed(ct) {
        (*ct).timeout = nfct_time_stamp + timeout as u32;
    } else {
        (*ct).timeout = timeout as u32;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
