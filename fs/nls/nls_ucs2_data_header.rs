/* SPDX-License-Identifier: GPL-2.0-or-later */

// Translated from nls_ucs2_data.h.

#[repr(C)]
pub struct UniCaseRange {
    pub start: u32,
    pub end: u32,
    pub table: *mut i8,
}

unsafe extern "C" {
    pub static mut NlsUniUpperTable: [i8; 512];
    pub static NlsUniUpperRange: [UniCaseRange; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
