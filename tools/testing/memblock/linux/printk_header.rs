/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: <stdio.h>, <asm/bug.h>

/*
 * memblock_dbg is called with u64 arguments that don't match the "%llu"
 * specifier in printf. This results in warnings that cannot be fixed without
 * modifying memblock.c, which we wish to avoid. As these messaged are not used
 * in testing anyway, the mismatch can be ignored.
 */
// C used GCC diagnostic pragmas here to ignore -Wformat around the printk alias.

unsafe extern "C" {
    pub fn printf(format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
}

#[macro_export]
macro_rules! printk {
    ($($arg:expr),* $(,)?) => {
        unsafe { $crate::printf($($arg),*) }
    };
}

#[macro_export]
macro_rules! pr_info {
    ($($arg:expr),* $(,)?) => {
        $crate::printk!($($arg),*)
    };
}

#[macro_export]
macro_rules! pr_debug {
    ($($arg:expr),* $(,)?) => {
        $crate::printk!($($arg),*)
    };
}

#[macro_export]
macro_rules! pr_cont {
    ($($arg:expr),* $(,)?) => {
        $crate::printk!($($arg),*)
    };
}

#[macro_export]
macro_rules! pr_err {
    ($($arg:expr),* $(,)?) => {
        $crate::printk!($($arg),*)
    };
}

#[macro_export]
macro_rules! pr_warn {
    ($($arg:expr),* $(,)?) => {
        $crate::printk!($($arg),*)
    };
}
