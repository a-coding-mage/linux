/* SPDX-License-Identifier: GPL-2.0 */

/*
 * C header dependencies removed from executable Rust:
 * "../../include/linux/kernel.h", <string.h>, <stdio.h>, <limits.h>,
 * <linux/compiler.h>, <linux/err.h>, <linux/bitops.h>, <linux/log2.h>,
 * "../../../include/linux/kconfig.h".
 */

unsafe extern "C" {
    pub fn printf(format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
}

#[macro_export]
macro_rules! printk {
    ($($arg:tt)*) => {
        unsafe { $crate::printf($($arg)*) }
    };
}

#[macro_export]
macro_rules! pr_err {
    ($($arg:tt)*) => {
        $crate::printk!($($arg)*)
    };
}

#[macro_export]
macro_rules! pr_info {
    ($($arg:tt)*) => {
        $crate::printk!($($arg)*)
    };
}

#[macro_export]
macro_rules! pr_debug {
    ($($arg:tt)*) => {
        $crate::printk!($($arg)*)
    };
}

#[macro_export]
macro_rules! pr_cont {
    ($($arg:tt)*) => {
        $crate::printk!($($arg)*)
    };
}

#[macro_export]
macro_rules! schedule {
    () => {};
}

pub const PAGE_SHIFT: ::core::ffi::c_int = 12;

#[macro_export]
macro_rules! EXPORT_PER_CPU_SYMBOL_GPL {
    ($x:tt) => {};
}
