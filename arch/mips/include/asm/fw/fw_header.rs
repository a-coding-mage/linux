/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2012 MIPS Technologies, Inc.
 */

// Dependency intent: declarations from <asm/bootinfo.h> are supplied externally.

use std::ffi::{c_char, c_int, c_ulong};

extern "C" {
    pub static mut fw_argc: c_int;
    pub static mut _fw_argv: *mut c_int;
    pub static mut _fw_envp: *mut c_int;

    pub fn fw_init_cmdline();
    pub fn fw_getcmdline() -> *mut c_char;
    pub fn fw_meminit();
    pub fn fw_getenv(name: *mut c_char) -> *mut c_char;
    pub fn fw_getenvl(name: *mut c_char) -> c_ulong;
    pub fn fw_init_early_console();
}

/*
 * Most firmware like YAMON, PMON, etc. pass arguments and environment
 * variables as 32-bit pointers. These take care of sign extension.
 */
#[inline]
pub unsafe fn fw_argv(index: usize) -> *mut c_char {
    (_fw_argv.add(index).read() as isize) as *mut c_char
}

#[inline]
pub unsafe fn fw_envp(index: usize) -> *mut c_char {
    (_fw_envp.add(index).read() as isize) as *mut c_char
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
