/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by net/netfilter/nf_conntrack_tuple.h.

#[repr(C)]
pub struct nf_ct_gre {
    pub stream_timeout: ::core::ffi::c_uint,
    pub timeout: ::core::ffi::c_uint,
}

// Forward declaration of struct nf_conn.
#[repr(C)]
pub struct nf_conn {
    _unused: [u8; 0],
}

// structure for original <-> reply keymap
#[repr(C)]
pub struct nf_ct_gre_keymap {
    pub list: list_head,
    pub tuple: nf_conntrack_tuple,
    pub rcu: rcu_head,
}

// add tuple->key_reply pairs to keymap
unsafe extern "C" {
    pub fn nf_ct_gre_keymap_add(
        ct: *mut nf_conn,
        orig: *const nf_conntrack_tuple,
        repl: *const nf_conntrack_tuple,
    ) -> bool;
}

// delete keymap entries
unsafe extern "C" {
    pub fn nf_ct_gre_keymap_destroy(ct: *mut nf_conn);
}

unsafe extern "C" {
    pub fn gre_pkt_to_tuple(
        skb: *const sk_buff,
        dataoff: ::core::ffi::c_uint,
        net: *mut net,
        tuple: *mut nf_conntrack_tuple,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
