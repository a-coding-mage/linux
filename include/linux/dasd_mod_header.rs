/* SPDX-License-Identifier: GPL-2.0 */

// The C header guard is omitted from executable Rust syntax.
// Dependency supplied by <asm/dasd.h>:
// `dasd_information2_t` is referenced through the surrounding crate.

use core::ffi::c_int;

#[repr(C)]
pub struct gendisk {
    _private: [u8; 0],
}

extern "C" {
    pub fn dasd_biodasdinfo(
        disk: *mut gendisk,
        info: *mut crate::dasd_information2_t,
    ) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
