/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translated from <linux/types.h> and <linux/netfilter.h> dependencies.

pub const XT_RECENT_CHECK: u32 = 1 << 0;
pub const XT_RECENT_SET: u32 = 1 << 1;
pub const XT_RECENT_UPDATE: u32 = 1 << 2;
pub const XT_RECENT_REMOVE: u32 = 1 << 3;
pub const XT_RECENT_TTL: u32 = 1 << 4;
pub const XT_RECENT_REAP: u32 = 1 << 5;

pub const XT_RECENT_SOURCE: u32 = 0;
pub const XT_RECENT_DEST: u32 = 1;

pub const XT_RECENT_NAME_LEN: usize = 200;

// Only allowed with --rcheck and --update
pub const XT_RECENT_MODIFIERS: u32 = XT_RECENT_TTL | XT_RECENT_REAP;

pub const XT_RECENT_VALID_FLAGS: u32 = XT_RECENT_CHECK
    | XT_RECENT_SET
    | XT_RECENT_UPDATE
    | XT_RECENT_REMOVE
    | XT_RECENT_TTL
    | XT_RECENT_REAP;

#[repr(C)]
pub struct xt_recent_mtinfo {
    pub seconds: u32,
    pub hit_count: u32,
    pub check_set: u8,
    pub invert: u8,
    pub name: [i8; XT_RECENT_NAME_LEN],
    pub side: u8,
}

#[repr(C)]
pub struct xt_recent_mtinfo_v1 {
    pub seconds: u32,
    pub hit_count: u32,
    pub check_set: u8,
    pub invert: u8,
    pub name: [i8; XT_RECENT_NAME_LEN],
    pub side: u8,
    // Supplied by the linux/netfilter.h dependency.
    pub mask: nf_inet_addr,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
