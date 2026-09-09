/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translated from the C header. The original dependency on <linux/types.h>
// supplies the fixed-width integer types represented here by Rust primitives.

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum xt_connbytes_what {
    XT_CONNBYTES_PKTS,
    XT_CONNBYTES_BYTES,
    XT_CONNBYTES_AVGPKT,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum xt_connbytes_direction {
    XT_CONNBYTES_DIR_ORIGINAL,
    XT_CONNBYTES_DIR_REPLY,
    XT_CONNBYTES_DIR_BOTH,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_connbytes_info_count {
    pub from: u64, // count to be matched
    pub to: u64,   // count to be matched
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_connbytes_info {
    pub count: xt_connbytes_info_count,
    pub what: u8,      // ipt_connbytes_what
    pub direction: u8, // ipt_connbytes_direction
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
