/* SPDX-License-Identifier: GPL-2.0 */

// Dependency equivalent of <linux/skbuff.h> is supplied by the surrounding
// translation unit.

extern "C" {
    pub fn nf_dup_ipv6(
        net: *mut net,
        skb: *mut sk_buff,
        hooknum: ::core::ffi::c_uint,
        gw: *const in6_addr,
        oif: ::core::ffi::c_int,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
