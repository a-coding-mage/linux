/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// C header guard: _UAPI_NFNL_ACCT_H_

// The C definition is conditional on NFACCT_NAME_MAX not already being
// defined by the surrounding build.
pub const NFACCT_NAME_MAX: i32 = 32;

pub const NFNL_MSG_ACCT_NEW: i32 = 0;
pub const NFNL_MSG_ACCT_GET: i32 = 1;
pub const NFNL_MSG_ACCT_GET_CTRZERO: i32 = 2;
pub const NFNL_MSG_ACCT_DEL: i32 = 3;
pub const NFNL_MSG_ACCT_OVERQUOTA: i32 = 4;
pub const NFNL_MSG_ACCT_MAX: i32 = 5;

pub const NFACCT_F_QUOTA_PKTS: i32 = 1 << 0;
pub const NFACCT_F_QUOTA_BYTES: i32 = 1 << 1;
// Can't be set from userspace.
pub const NFACCT_F_OVERQUOTA: i32 = 1 << 2;

pub const NFACCT_UNSPEC: i32 = 0;
pub const NFACCT_NAME: i32 = 1;
pub const NFACCT_PKTS: i32 = 2;
pub const NFACCT_BYTES: i32 = 3;
pub const NFACCT_USE: i32 = 4;
pub const NFACCT_FLAGS: i32 = 5;
pub const NFACCT_QUOTA: i32 = 6;
pub const NFACCT_FILTER: i32 = 7;
pub const NFACCT_PAD: i32 = 8;
pub const __NFACCT_MAX: i32 = 9;
pub const NFACCT_MAX: i32 = __NFACCT_MAX - 1;

pub const NFACCT_FILTER_UNSPEC: i32 = 0;
pub const NFACCT_FILTER_MASK: i32 = 1;
pub const NFACCT_FILTER_VALUE: i32 = 2;
pub const __NFACCT_FILTER_MAX: i32 = 3;
pub const NFACCT_FILTER_MAX: i32 = __NFACCT_FILTER_MAX - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
