/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency: ETH_ALEN is supplied by <linux/if_ether.h>.

#[repr(C)]
pub struct ebt_arpreply_info {
    pub mac: [u8; ETH_ALEN],
    pub target: core::ffi::c_int,
}

pub const EBT_ARPREPLY_TARGET: &str = "arpreply";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
