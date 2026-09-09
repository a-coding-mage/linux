/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) 2016 Namjae Jeon <linkinjeon@kernel.org>
 *   Copyright (C) 2018 Samsung Electronics Co., Ltd.
 */

// Dependencies supplied by the corresponding kernel/Rust translation units:
// `unicode.h`, `vfs_cache.h`, and Linux ctype facilities.

extern "C" {
    pub static mut ksmbd_debug_types: ::core::ffi::c_int;
}

pub const KSMBD_DEBUG_SMB: ::core::ffi::c_uint = 1 << 0;
pub const KSMBD_DEBUG_AUTH: ::core::ffi::c_uint = 1 << 1;
pub const KSMBD_DEBUG_VFS: ::core::ffi::c_uint = 1 << 2;
pub const KSMBD_DEBUG_OPLOCK: ::core::ffi::c_uint = 1 << 3;
pub const KSMBD_DEBUG_IPC: ::core::ffi::c_uint = 1 << 4;
pub const KSMBD_DEBUG_CONN: ::core::ffi::c_uint = 1 << 5;
pub const KSMBD_DEBUG_RDMA: ::core::ffi::c_uint = 1 << 6;
pub const KSMBD_DEBUG_ALL: ::core::ffi::c_uint = KSMBD_DEBUG_SMB
    | KSMBD_DEBUG_AUTH
    | KSMBD_DEBUG_VFS
    | KSMBD_DEBUG_OPLOCK
    | KSMBD_DEBUG_IPC
    | KSMBD_DEBUG_CONN
    | KSMBD_DEBUG_RDMA;

// The C header conditionally redefines the kernel logging prefix using
// `SUBMOD_NAME`; preserve that build-time intent for dependent translations.
#[cfg(submod_name)]
#[macro_export]
macro_rules! pr_fmt {
    ($fmt:expr) => { concat!("ksmbd: ", env!("SUBMOD_NAME"), ": ", $fmt) };
}

#[cfg(not(submod_name))]
#[macro_export]
macro_rules! pr_fmt {
    ($fmt:expr) => { concat!("ksmbd: ", $fmt) };
}

// `pr_info` is supplied externally, as in the original kernel header.
#[macro_export]
macro_rules! ksmbd_debug {
    (SMB, $fmt:expr $(, $arg:expr)*) => { $crate::ksmbd_debug!(KSMBD_DEBUG_SMB, $fmt $(, $arg)*); };
    (AUTH, $fmt:expr $(, $arg:expr)*) => { $crate::ksmbd_debug!(KSMBD_DEBUG_AUTH, $fmt $(, $arg)*); };
    (VFS, $fmt:expr $(, $arg:expr)*) => { $crate::ksmbd_debug!(KSMBD_DEBUG_VFS, $fmt $(, $arg)*); };
    (OPLOCK, $fmt:expr $(, $arg:expr)*) => { $crate::ksmbd_debug!(KSMBD_DEBUG_OPLOCK, $fmt $(, $arg)*); };
    (IPC, $fmt:expr $(, $arg:expr)*) => { $crate::ksmbd_debug!(KSMBD_DEBUG_IPC, $fmt $(, $arg)*); };
    (CONN, $fmt:expr $(, $arg:expr)*) => { $crate::ksmbd_debug!(KSMBD_DEBUG_CONN, $fmt $(, $arg)*); };
    (RDMA, $fmt:expr $(, $arg:expr)*) => { $crate::ksmbd_debug!(KSMBD_DEBUG_RDMA, $fmt $(, $arg)*); };
    ($mask:expr, $fmt:expr $(, $arg:expr)*) => {
        if unsafe { $crate::ksmbd_debug_types } & $mask as ::core::ffi::c_int != 0 {
            pr_info!($fmt $(, $arg)*);
        }
    };
}

#[macro_export]
macro_rules! UNICODE_LEN {
    ($x:expr) => { ($x) * 2 };
}

// GFP_KERNEL and __GFP_RETRY_MAYFAIL are supplied by the kernel dependency.
pub const KSMBD_DEFAULT_GFP: usize = GFP_KERNEL | __GFP_RETRY_MAYFAIL;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
