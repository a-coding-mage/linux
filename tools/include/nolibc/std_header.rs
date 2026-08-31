/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * Standard definitions and types for NOLIBC
 * Copyright (C) 2017-2021 Willy Tarreau <w@1wt.eu>
 */

/* Declare a few quite common macros and types that usually are in stdlib.h,
 * stdint.h, ctype.h, unistd.h and a few other common locations. Please place
 * integer type definitions and generic macros here, but avoid OS-specific and
 * syscall-specific stuff, as this file is expected to be included very early.
 */

/* C dependencies: "stdint.h", "stddef.h", and <linux/types.h>. */

/* those are commonly provided by sys/types.h */
pub type dev_t = u64;
pub type ino_t = u64;
pub type mode_t = ::core::ffi::c_uint;
pub type pid_t = ::core::ffi::c_int;
pub type uid_t = ::core::ffi::c_uint;
pub type gid_t = ::core::ffi::c_uint;
pub type nlink_t = ::core::ffi::c_ulong;
pub type off_t = i64;
pub type blksize_t = ::core::ffi::c_long;
pub type blkcnt_t = ::core::ffi::c_long;
pub type time_t = __kernel_time64_t;
