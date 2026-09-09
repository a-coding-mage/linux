/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency: __u8 is provided by <linux/types.h> in the C source.
#[repr(C)]
pub struct ebt_pkttype_info {
    pub pkt_type: u8,
    pub invert: u8,
}

pub const EBT_PKTTYPE_MATCH: &str = "pkttype";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
