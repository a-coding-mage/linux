/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * Sysmacro definitions for NOLIBC
 * Copyright (C) 2017-2021 Willy Tarreau <w@1wt.eu>
 */

/* make sure to include all global symbols */
/* C dependency: ../nolibc.h */

/* C dependency: ../std.h */

pub type dev_t = crate::dev_t;

#[inline]
pub fn __nolibc_makedev(maj: ::core::ffi::c_uint, min: ::core::ffi::c_uint) -> dev_t {
    (((maj as dev_t) & !(0xfff as dev_t)) << 32)
        | (((maj & 0xfff) as dev_t) << 8)
        | (((min as dev_t) & !(0xff as dev_t)) << 12)
        | ((min & 0xff) as dev_t)
}

/* #define makedev(maj, min) __nolibc_makedev(maj, min) */

#[inline]
pub fn makedev(maj: ::core::ffi::c_uint, min: ::core::ffi::c_uint) -> dev_t {
    __nolibc_makedev(maj, min)
}

#[inline]
pub fn __nolibc_major(dev: dev_t) -> ::core::ffi::c_uint {
    (((dev >> 32) & !(0xfff as dev_t)) | ((dev >> 8) & (0xfff as dev_t))) as ::core::ffi::c_uint
}

/* #define major(dev) __nolibc_major(dev) */

#[inline]
pub fn major(dev: dev_t) -> ::core::ffi::c_uint {
    __nolibc_major(dev)
}

#[inline]
pub fn __nolibc_minor(dev: dev_t) -> ::core::ffi::c_uint {
    (((dev >> 12) & !(0xff as dev_t)) | (dev & (0xff as dev_t))) as ::core::ffi::c_uint
}

/* #define minor(dev) __nolibc_minor(dev) */

#[inline]
pub fn minor(dev: dev_t) -> ::core::ffi::c_uint {
    __nolibc_minor(dev)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
