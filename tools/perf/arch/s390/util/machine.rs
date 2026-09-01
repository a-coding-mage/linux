// SPDX-License-Identifier: GPL-2.0
// C dependencies: <inttypes.h>, <unistd.h>, <stdio.h>, <string.h>,
// <internal/lib.h> (page_size), "machine.h", "api/fs/fs.h", "debug.h",
// "symbol.h"

use core::ffi::{c_char, c_int};

pub type u64 = core::primitive::u64;

pub const PATH_MAX: usize = 4096;

extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn sysfs__read_ull(entry: *const c_char, value: *mut u64) -> c_int;
    fn pr_debug2(format: *const c_char, ...);
}

#[no_mangle]
pub unsafe extern "C" fn arch__fix_module_text_start(
    start: *mut u64,
    size: *mut u64,
    name: *const c_char,
) -> c_int {
    let m_start: u64 = *start;
    let mut path: [c_char; PATH_MAX] = [0; PATH_MAX];

    snprintf(
        path.as_mut_ptr(),
        PATH_MAX,
        b"module/%.*s/sections/.text\0".as_ptr() as *const c_char,
        strlen(name) as c_int - 2,
        name.add(1),
    );
    if sysfs__read_ull(path.as_mut_ptr(), start as *mut u64) < 0 {
        pr_debug2(
            b"Using module %s start:%#lx\n\0".as_ptr() as *const c_char,
            path.as_mut_ptr(),
            m_start,
        );
        *start = m_start;
    } else {
        /* Successful read of the modules segment text start address.
         * Calculate difference between module start address
         * in memory and module text segment start address.
         * For example module load address is 0x3ff8011b000
         * (from /proc/modules) and module text segment start
         * address is 0x3ff8011b870 (from file above).
         *
         * Adjust the module size and subtract the GOT table
         * size located at the beginning of the module.
         */
        *size -= *start - m_start;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
