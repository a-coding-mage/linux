/* SPDX-License-Identifier: GPL-2.0 */

// Translated from nf_conntrack_snmp.h.
// C dependencies: <linux/netfilter.h> and <linux/skbuff.h>.

use core::ffi::c_int;

/// Opaque declaration supplied by the Linux networking dependencies.
#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

/// Opaque declaration supplied by the Linux conntrack dependencies.
#[repr(C)]
pub struct nf_conn {
    _private: [u8; 0],
}

/// `enum ip_conntrack_info` supplied by the Linux netfilter dependencies.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_conntrack_info(pub c_int);

pub type nf_nat_snmp_hook_fn = unsafe extern "C" fn(
    skb: *mut sk_buff,
    protoff: u32,
    ct: *mut nf_conn,
    ctinfo: ip_conntrack_info,
) -> c_int;

/// External RCU-protected SNMP NAT hook.
extern "C" {
    pub static mut nf_nat_snmp_hook: *mut nf_nat_snmp_hook_fn;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
