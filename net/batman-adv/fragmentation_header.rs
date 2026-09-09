/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) B.A.T.M.A.N. contributors:
 *
 * Martin Hundebøll <martin@hundeboll.net>
 */

/* Dependencies supplied by the surrounding kernel translation unit. */

use core::ffi::c_int;

extern "C" {
    pub fn batadv_frag_purge_orig(
        orig: *mut batadv_orig_node,
        check_cb: Option<unsafe extern "C" fn(*mut batadv_frag_table_entry) -> bool>,
    );
    pub fn batadv_frag_skb_fwd(
        skb: *mut sk_buff,
        recv_if: *mut batadv_hard_iface,
        orig_node_src: *mut batadv_orig_node,
        rx_result: *mut c_int,
    ) -> bool;
    pub fn batadv_frag_skb_buffer(
        skb: *mut *mut sk_buff,
        orig_node: *mut batadv_orig_node,
    ) -> bool;
    pub fn batadv_frag_send_packet(
        skb: *mut sk_buff,
        orig_node: *mut batadv_orig_node,
        neigh_node: *mut batadv_neigh_node,
    ) -> c_int;

    /* C macros/helpers supplied by the included kernel headers. */
    pub fn hlist_empty(head: *const hlist_head) -> bool;
    pub fn batadv_has_timed_out(timestamp: u32, timeout: u32) -> bool;
}

/* Types supplied by main.h and the kernel headers. */
#[repr(C)]
pub struct batadv_orig_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct batadv_hard_iface {
    _private: [u8; 0],
}
#[repr(C)]
pub struct batadv_neigh_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}
#[repr(C)]
pub struct hlist_head {
    _private: [u8; 0],
}
#[repr(C)]
pub struct batadv_frag_table_entry {
    pub fragment_list: hlist_head,
    pub timestamp: u32,
}

/* BATADV_FRAG_TIMEOUT is supplied by the surrounding build configuration. */
pub const BATADV_FRAG_TIMEOUT: u32 = 0;

/**
 * batadv_frag_check_entry() - check if a list of fragments has timed out
 * @frags_entry: table entry to check
 *
 * Return: true if the frags entry has timed out, false otherwise.
 */
#[inline]
pub unsafe fn batadv_frag_check_entry(
    frags_entry: *mut batadv_frag_table_entry,
) -> bool {
    if !hlist_empty(core::ptr::addr_of!((*frags_entry).fragment_list))
        && batadv_has_timed_out((*frags_entry).timestamp, BATADV_FRAG_TIMEOUT)
    {
        return true;
    }
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
