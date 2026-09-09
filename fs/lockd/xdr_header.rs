/* SPDX-License-Identifier: GPL-2.0 */
/*
 * XDR types for the NLM protocol
 *
 * Copyright (C) 1996 Olaf Kirch <okir@monad.swb.de>
 */

// Dependencies supplied by the surrounding translation unit:
// linux/fs.h, linux/filelock.h, linux/nfs.h, and linux/sunrpc/xdr.h

use core::ffi::c_char;

pub const SM_MAXSTRLEN: usize = 1024;
pub const SM_PRIV_SIZE: usize = 16;

#[repr(C)]
pub struct nsm_private {
    pub data: [u8; SM_PRIV_SIZE],
}

pub const NLM_MAXCOOKIELEN: usize = 32;
pub const NLM_MAXSTRLEN: usize = 1024;

// These macros preserve the original cpu_to_be32(NLM_...) expressions.
#[macro_export]
macro_rules! nlm_granted {
    () => { cpu_to_be32(NLM_LCK_GRANTED) };
}
#[macro_export]
macro_rules! nlm_lck_denied {
    () => { cpu_to_be32(NLM_LCK_DENIED) };
}
#[macro_export]
macro_rules! nlm_lck_denied_nolocks {
    () => { cpu_to_be32(NLM_LCK_DENIED_NOLOCKS) };
}
#[macro_export]
macro_rules! nlm_lck_blocked {
    () => { cpu_to_be32(NLM_LCK_BLOCKED) };
}
#[macro_export]
macro_rules! nlm_lck_denied_grace_period {
    () => { cpu_to_be32(NLM_LCK_DENIED_GRACE_PERIOD) };
}

/* Lock info passed via NLM */
#[repr(C)]
pub struct lockd_lock {
    pub caller: *mut c_char,
    pub len: u32, // length of "caller"
    pub fh: nfs_fh,
    pub oh: xdr_netobj,
    pub svid: u32,
    pub lock_start: u64,
    pub lock_len: u64,
    pub fl: file_lock,
}

/*
 * NLM cookies. Technically they can be 1K, but Linux only uses 8 bytes.
 * FreeBSD uses 16, Apple Mac OS X 10.3 uses 20. Therefore we set it to
 * 32 bytes.
 */
#[repr(C)]
pub struct lockd_cookie {
    pub data: [u8; NLM_MAXCOOKIELEN],
    pub len: u32,
}

/*
 * Generic lockd arguments for all but sm_notify
 */
#[repr(C)]
pub struct lockd_args {
    pub cookie: lockd_cookie,
    pub lock: lockd_lock,
    pub block: u32,
    pub reclaim: u32,
    pub state: u32,
}

/*
 * Generic lockd result
 */
#[repr(C)]
pub struct lockd_res {
    pub cookie: lockd_cookie,
    pub status: u32,
    pub lock: lockd_lock,
}

/*
 * statd callback when client has rebooted
 */
#[repr(C)]
pub struct lockd_reboot {
    pub mon: *mut c_char,
    pub len: u32,
    pub state: u32,
    pub priv_: nsm_private,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
