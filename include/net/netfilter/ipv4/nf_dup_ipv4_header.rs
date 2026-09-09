/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding kernel networking and UAPI modules:
// linux/skbuff.h and uapi/linux/in.h

extern "C" {
    pub fn nf_dup_ipv4(
        net: *mut crate::net,
        skb: *mut crate::sk_buff,
        hooknum: core::ffi::c_uint,
        gw: *const crate::in_addr,
        oif: core::ffi::c_int,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
