/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// C dependencies removed from executable Rust:
// - <sys/types.h>
// - <stdint.h>
// - <jvmti.h>, which supplies jmethodID.

#[repr(C)]
pub struct jvmti_line_info_t {
    pub pc: c_ulong,
    pub line_number: c_int,
    pub discrim: c_int, /* discriminator -- 0 for now */
    pub methodID: jmethodID,
}

unsafe extern "C" {
    pub fn jvmti_open() -> *mut c_void;
    pub fn jvmti_close(agent: *mut c_void) -> c_int;
    pub fn jvmti_write_code(
        agent: *mut c_void,
        symbol_name: *const c_char,
        vma: u64,
        code: *const c_void,
        code_size: c_uint,
    ) -> c_int;

    pub fn jvmti_write_debug_info(
        agent: *mut c_void,
        code: u64,
        nr_lines: c_int,
        li: *mut jvmti_line_info_t,
        file_names: *const *const c_char,
    ) -> c_int;
}
