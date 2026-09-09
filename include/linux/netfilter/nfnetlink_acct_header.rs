/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: declarations from <uapi/linux/netfilter/nfnetlink_acct.h>
// and <net/net_namespace.h> are supplied by other translated files.

pub const NFACCT_NO_QUOTA: i32 = -1;
pub const NFACCT_UNDERQUOTA: i32 = 0;
pub const NFACCT_OVERQUOTA: i32 = 1;

#[repr(C)]
pub struct nf_acct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn nfnl_acct_find_get(
        net: *mut net,
        filter_name: *const core::ffi::c_char,
    ) -> *mut nf_acct;
    pub fn nfnl_acct_put(acct: *mut nf_acct);
    pub fn nfnl_acct_update(skb: *const sk_buff, nfacct: *mut nf_acct);
    pub fn nfnl_acct_overquota(net: *mut net, nfacct: *mut nf_acct) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
