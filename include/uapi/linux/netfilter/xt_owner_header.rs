/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translated from the Linux UAPI header xt_owner.h.

pub const XT_OWNER_UID: u32 = 1 << 0;
pub const XT_OWNER_GID: u32 = 1 << 1;
pub const XT_OWNER_SOCKET: u32 = 1 << 2;
pub const XT_OWNER_SUPPL_GROUPS: u32 = 1 << 3;

pub const XT_OWNER_MASK: u32 = XT_OWNER_UID
    | XT_OWNER_GID
    | XT_OWNER_SOCKET
    | XT_OWNER_SUPPL_GROUPS;

#[repr(C)]
pub struct xt_owner_match_info {
    pub uid_min: u32,
    pub uid_max: u32,
    pub gid_min: u32,
    pub gid_max: u32,
    pub match_: u8,
    pub invert: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
