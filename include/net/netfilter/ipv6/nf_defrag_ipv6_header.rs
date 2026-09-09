/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_int;

// Opaque types supplied by the corresponding kernel dependencies.
#[repr(C)]
pub struct net {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ctl_table_header {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fqdir {
    _private: [u8; 0],
}

extern "C" {
    pub fn nf_defrag_ipv6_enable(net: *mut net) -> c_int;
    pub fn nf_defrag_ipv6_disable(net: *mut net);

    pub fn nf_ct_frag6_init() -> c_int;
    pub fn nf_ct_frag6_cleanup();
    pub fn nf_ct_frag6_gather(net: *mut net, skb: *mut sk_buff, user: u32) -> c_int;
}

#[repr(C)]
pub struct nft_ct_frag6_pernet {
    pub nf_frag_frags_hdr: *mut ctl_table_header,
    pub fqdir: *mut fqdir,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
