/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * C header guard: _XT_TEE_TARGET_H
 * Dependency: linux/netfilter.h supplies nf_inet_addr.
 */

#[repr(C)]
pub struct nf_inet_addr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xt_tee_priv {
    _private: [u8; 0],
}

#[repr(C, align(8))]
pub struct xt_tee_tginfo {
    pub gw: nf_inet_addr,
    pub oif: [core::ffi::c_char; 16],

    /* used internally by the kernel */
    pub priv_: *mut xt_tee_priv,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
