/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Helpers to call into LLVM C++ code from C, for the parts that do not have
 * C APIs.
 */

use std::os::raw::c_char;

/*
 * Dependency intent from the original header:
 * - <linux/compiler.h> supplies kernel/compiler definitions.
 * - u64 and bool are supplied by surrounding perf/kernel headers.
 * - When included from C++, the declarations use extern "C".
 */

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct llvm_a2l_frame {
    pub filename: *mut c_char,
    pub funcname: *mut c_char,
    pub line: ::std::os::raw::c_uint,
}

unsafe extern "C" {
    /*
     * Implement addr2line() using libLLVM. LLVM is a C++ API, and
     * many of the linux/ headers cannot be included in a C++ compile unit,
     * so we need to make a little bridge code here. llvm_addr2line() will
     * convert the inline frame information from LLVM's internal structures
     * and put them into a flat array given in inline_frames. The caller
     * is then responsible for taking that array and convert it into perf's
     * regular inline frame structures (which depend on e.g. struct list_head).
     *
     * If the address could not be resolved, or an error occurred (e.g. OOM),
     * returns 0. Otherwise, returns the number of inline frames (which means 1
     * if the address was not part of an inlined function). If unwind_inlines
     * is set and the return code is nonzero, inline_frames will be set to
     * a newly allocated array with that length. The caller is then responsible
     * for freeing both the strings and the array itself.
     */
    pub fn llvm_addr2line(
        dso_name: *const c_char,
        addr: u64,
        file: *mut *mut c_char,
        line: *mut ::std::os::raw::c_uint,
        unwind_inlines: bool,
        inline_frames: *mut *mut llvm_a2l_frame,
    ) -> ::std::os::raw::c_int;

    /*
     * Simple symbolizers for addresses; will convert something like
     * 0x12345 to "func+0x123". Will return NULL if no symbol was found.
     *
     * The returned value must be freed by the caller, with free().
     */
    pub fn llvm_name_for_code(dso: *mut dso, dso_name: *const c_char, addr: u64) -> *mut c_char;
    pub fn llvm_name_for_data(dso: *mut dso, dso_name: *const c_char, addr: u64) -> *mut c_char;
}
