/* SPDX-License-Identifier: GPL-2.0 */
/*
 *	Variables and functions used by the code in sleep.c
 */

// Dependency intent from <linux/linkage.h> is supplied by the surrounding build.

use core::ffi::{c_int, c_long, c_ulong};

extern "C" {
    pub static mut saved_video_mode: c_ulong;
    pub static mut saved_magic: c_long;

    pub static mut wakeup_pmode_return: c_int;

    pub static mut wake_sleep_flags: u8;

    pub fn wakeup_long64();

    pub fn do_suspend_lowlevel();

    pub fn x86_acpi_suspend_lowlevel() -> c_int;

    // `asmlinkage` is a platform-specific calling-convention annotation in C;
    // the surrounding Rust target supplies the corresponding ABI for acpi_status.
    pub fn x86_acpi_enter_sleep_state(state: u8) -> acpi_status;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
