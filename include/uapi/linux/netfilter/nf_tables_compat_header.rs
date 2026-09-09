/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translated from nf_tables_compat.h.

pub const NFTA_TARGET_UNSPEC: i32 = 0;
pub const NFTA_TARGET_NAME: i32 = 1;
pub const NFTA_TARGET_REV: i32 = 2;
pub const NFTA_TARGET_INFO: i32 = 3;
pub const __NFTA_TARGET_MAX: i32 = 4;
pub const NFTA_TARGET_MAX: i32 = __NFTA_TARGET_MAX - 1;

pub const NFTA_MATCH_UNSPEC: i32 = 0;
pub const NFTA_MATCH_NAME: i32 = 1;
pub const NFTA_MATCH_REV: i32 = 2;
pub const NFTA_MATCH_INFO: i32 = 3;
pub const __NFTA_MATCH_MAX: i32 = 4;
pub const NFTA_MATCH_MAX: i32 = __NFTA_MATCH_MAX - 1;

pub const NFT_COMPAT_NAME_MAX: i32 = 32;

pub const NFNL_MSG_COMPAT_GET: i32 = 0;
pub const NFNL_MSG_COMPAT_MAX: i32 = 1;

pub const NFTA_COMPAT_UNSPEC: i32 = 0;
pub const NFTA_COMPAT_NAME: i32 = 1;
pub const NFTA_COMPAT_REV: i32 = 2;
pub const NFTA_COMPAT_TYPE: i32 = 3;
pub const __NFTA_COMPAT_MAX: i32 = 4;
pub const NFTA_COMPAT_MAX: i32 = __NFTA_COMPAT_MAX - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
