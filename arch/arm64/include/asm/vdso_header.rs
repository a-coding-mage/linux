/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 ARM Limited
 */

/* Number of pages occupied by the vDSO. */
pub const __VDSO_PAGES: usize = 4;

/*
 * The generated vDSO offsets are supplied by the build system.
 * Pass the corresponding generated `vdso_offset_<name>` item as `$offset`.
 */
#[macro_export]
macro_rules! VDSO_SYMBOL {
    ($base:expr, $offset:expr) => {{
        (($offset as usize).wrapping_add($base as usize)) as *mut core::ffi::c_void
    }};
}

extern "C" {
    pub static mut vdso_start: core::ffi::c_char;
    pub static mut vdso_end: core::ffi::c_char;
    pub static mut vdso32_start: core::ffi::c_char;
    pub static mut vdso32_end: core::ffi::c_char;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
