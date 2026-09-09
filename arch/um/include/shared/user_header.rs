/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

/* Dependency supplied by generated/asm-offsets.h. */

/* The usual definition, copied here because the kernel provides its own,
 * fancier, type-safe definition. */
#[macro_export]
macro_rules! ARRAY_SIZE {
    ($x:expr) => {
        ::core::mem::size_of_val(&$x) / ::core::mem::size_of_val(&$x[0])
    };
}

/* This is to get size_t and NULL. */
pub type size_t = usize;

unsafe extern "C" {
    pub fn panic(fmt: *const ::core::ffi::c_char, ...);
}

/* Requires preincluding include/linux/kern_levels.h. */
pub const UM_KERN_EMERG: usize = KERN_EMERG;
pub const UM_KERN_ALERT: usize = KERN_ALERT;
pub const UM_KERN_CRIT: usize = KERN_CRIT;
pub const UM_KERN_ERR: usize = KERN_ERR;
pub const UM_KERN_WARNING: usize = KERN_WARNING;
pub const UM_KERN_NOTICE: usize = KERN_NOTICE;
pub const UM_KERN_INFO: usize = KERN_INFO;
pub const UM_KERN_DEBUG: usize = KERN_DEBUG;
pub const UM_KERN_CONT: usize = KERN_CONT;

/* IS_ENABLED(CONFIG_PRINTK) is a build-time condition supplied externally. */
#[cfg(feature = "CONFIG_PRINTK")]
macro_rules! printk {
    ($($args:tt)*) => { _printk($($args)*) };
}

#[cfg(feature = "CONFIG_PRINTK")]
unsafe extern "C" {
    pub fn _printk(fmt: *const ::core::ffi::c_char, ... ) -> ::core::ffi::c_int;
    pub fn print_hex_dump(
        level: *const ::core::ffi::c_char,
        prefix_str: *const ::core::ffi::c_char,
        prefix_type: ::core::ffi::c_int,
        rowsize: ::core::ffi::c_int,
        groupsize: ::core::ffi::c_int,
        buf: *const ::core::ffi::c_void,
        len: size_t,
        ascii: bool,
    );
}

#[cfg(not(feature = "CONFIG_PRINTK"))]
pub unsafe fn printk(_fmt: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_PRINTK"))]
pub unsafe fn print_hex_dump(
    _level: *const ::core::ffi::c_char,
    _prefix_str: *const ::core::ffi::c_char,
    _prefix_type: ::core::ffi::c_int,
    _rowsize: ::core::ffi::c_int,
    _groupsize: ::core::ffi::c_int,
    _buf: *const ::core::ffi::c_void,
    _len: size_t,
    _ascii: bool,
) {
}

unsafe extern "C" {
    pub fn in_aton(str_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn strlcat(
        dst: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        size: size_t,
    ) -> size_t;
    pub fn sized_strscpy(
        dst: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        size: size_t,
    ) -> size_t;
}

#[macro_export]
macro_rules! strscpy {
    ($dst:expr, $src:expr) => {
        sized_strscpy($dst, $src, ::core::mem::size_of_val(&$dst))
    };
}

/* Copied from linux/compiler-gcc.h since it cannot be included directly. */
#[macro_export]
macro_rules! barrier {
    () => {
        ::core::sync::atomic::compiler_fence(::core::sync::atomic::Ordering::SeqCst)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
