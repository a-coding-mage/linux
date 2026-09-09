/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * User memory access support for Hexagon
 *
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 */

/* User space memory access functions. */
/* Dependency: <asm/sections.h> supplies related external definitions. */

/*
 * When a kernel-mode page fault is taken, the faulting instruction
 * address is checked against a table of exception_table_entries.
 * Each entry is a tuple of the address of an instruction that may be
 * authorized to fault, and the address at which execution should be
 * resumed instead of the faulting instruction, so as to effect a
 * workaround.
 */

/* Assembly somewhat optimized copy routines. */
extern "C" {
    pub fn raw_copy_from_user(
        to: *mut core::ffi::c_void,
        from: *const core::ffi::c_void,
        n: usize,
    ) -> usize;

    pub fn raw_copy_to_user(
        to: *mut core::ffi::c_void,
        from: *const core::ffi::c_void,
        n: usize,
    ) -> usize;

    pub fn __clear_user_hexagon(
        dest: *mut core::ffi::c_void,
        count: usize,
    ) -> usize;
}

/* C marker macro: inline copy implementations are enabled by the header. */
pub const INLINE_COPY_USER: bool = true;

/* Equivalent of __clear_user(a, s). */
#[macro_export]
macro_rules! __clear_user {
    ($a:expr, $s:expr) => {
        unsafe { $crate::__clear_user_hexagon($a, $s) }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
