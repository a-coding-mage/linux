/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Declarations for the Network Lock Manager protocol.
 *
 * Copyright (C) 1996, Olaf Kirch <okir@monad.swb.de>
 */

/* Maximum file offset in file_lock.fl_end */
pub const NLM_OFFSET_MAX: i32 = 0x7fff_ffffi32;
pub const NLM4_OFFSET_MAX: i64 = ((u64::MAX >> 1) as i64);

/* Return states for NLM */
pub const NLM_LCK_GRANTED: i32 = 0;
pub const NLM_LCK_DENIED: i32 = 1;
pub const NLM_LCK_DENIED_NOLOCKS: i32 = 2;
pub const NLM_LCK_BLOCKED: i32 = 3;
pub const NLM_LCK_DENIED_GRACE_PERIOD: i32 = 4;

/*
 * The following return states are present when CONFIG_LOCKD_V4 is enabled
 * in the C implementation.  This condition is preserved as a Rust cfg.
 */
#[cfg(feature = "CONFIG_LOCKD_V4")]
pub const NLM_DEADLCK: i32 = 5;
#[cfg(feature = "CONFIG_LOCKD_V4")]
pub const NLM_ROFS: i32 = 6;
#[cfg(feature = "CONFIG_LOCKD_V4")]
pub const NLM_STALE_FH: i32 = 7;
#[cfg(feature = "CONFIG_LOCKD_V4")]
pub const NLM_FBIG: i32 = 8;
#[cfg(feature = "CONFIG_LOCKD_V4")]
pub const NLM_FAILED: i32 = 9;

pub const NLM_PROGRAM: i32 = 100021;

pub const NLMPROC_NULL: i32 = 0;
pub const NLMPROC_TEST: i32 = 1;
pub const NLMPROC_LOCK: i32 = 2;
pub const NLMPROC_CANCEL: i32 = 3;
pub const NLMPROC_UNLOCK: i32 = 4;
pub const NLMPROC_GRANTED: i32 = 5;
pub const NLMPROC_TEST_MSG: i32 = 6;
pub const NLMPROC_LOCK_MSG: i32 = 7;
pub const NLMPROC_CANCEL_MSG: i32 = 8;
pub const NLMPROC_UNLOCK_MSG: i32 = 9;
pub const NLMPROC_GRANTED_MSG: i32 = 10;
pub const NLMPROC_TEST_RES: i32 = 11;
pub const NLMPROC_LOCK_RES: i32 = 12;
pub const NLMPROC_CANCEL_RES: i32 = 13;
pub const NLMPROC_UNLOCK_RES: i32 = 14;
pub const NLMPROC_GRANTED_RES: i32 = 15;
pub const NLMPROC_NSM_NOTIFY: i32 = 16; /* statd callback */
pub const NLMPROC_SHARE: i32 = 20;
pub const NLMPROC_UNSHARE: i32 = 21;
pub const NLMPROC_NM_LOCK: i32 = 22;
pub const NLMPROC_FREE_ALL: i32 = 23;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
