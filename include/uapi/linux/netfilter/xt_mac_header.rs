/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency supplied by <linux/if_ether.h>.
// use linux::if_ether::ETH_ALEN;

#[repr(C)]
pub struct xt_mac_info {
    pub srcaddr: [u8; ETH_ALEN],
    pub invert: ::core::ffi::c_int,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
