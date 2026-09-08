/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: declarations from <uapi/asm/console.h> are supplied by
// the surrounding translation unit.

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

// The declarations below are excluded when compiling for the assembler in C.
// Rust translation of the non-assembler interface follows.
unsafe extern "C" {
    pub fn callback_puts(unit: c_long, s: *const c_char, length: c_long) -> c_long;
    pub fn callback_getc(unit: c_long) -> c_long;
    pub fn callback_open_console() -> c_long;
    pub fn callback_close_console() -> c_long;
    pub fn callback_open(device: *const c_char, length: c_long) -> c_long;
    pub fn callback_close(unit: c_long) -> c_long;
    pub fn callback_read(
        channel: c_long,
        count: c_long,
        buf: *const c_char,
        lbn: c_long,
    ) -> c_long;
    pub fn callback_getenv(id: c_long, buf: *const c_char, buf_size: c_ulong) -> c_long;
    pub fn callback_setenv(id: c_long, buf: *const c_char, buf_size: c_ulong) -> c_long;
    pub fn callback_save_env() -> c_long;

    pub fn srm_fixup(new_callback_addr: c_ulong, new_hwrpb_addr: c_ulong) -> c_int;
    pub fn srm_puts(s: *const c_char, arg1: c_long) -> c_long;
    pub fn srm_printk(s: *const c_char, ...) -> c_long;

    pub fn callback_init(arg1: *mut c_void) -> *mut c_void;
    pub static mut callback_init_done: c_int;
}

#[repr(C)]
pub struct crb_struct {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct hwrpb_struct {
    _opaque: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
