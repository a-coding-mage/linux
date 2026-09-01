// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int};

// C dependency intent:
//   #include <stdlib.h>
//   UNW_OBJ(dwarf_find_debug_frame)
//
// The original file relies on externally supplied libunwind types and the
// UNW_OBJ name-mangling macro.
#[allow(non_camel_case_types)]
type unw_word_t = usize;

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct unw_dyn_info_t {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn dwarf_find_debug_frame(
        found: c_int,
        di_debug: *mut unw_dyn_info_t,
        ip: unw_word_t,
        segbase: unw_word_t,
        obj_name: *const c_char,
        start: unw_word_t,
        end: unw_word_t,
    ) -> c_int;
}

fn main() {
    unsafe {
        dwarf_find_debug_frame(
            0,
            core::ptr::null_mut(),
            0,
            0,
            core::ptr::null(),
            0,
            0,
        );
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
