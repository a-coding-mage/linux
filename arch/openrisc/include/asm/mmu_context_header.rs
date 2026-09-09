/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * OpenRISC Linux
 *
 * Linux architectural port borrowing liberally from similar works of
 * others.  All original copyrights apply as per the original source
 * declaration.
 *
 * OpenRISC implementation:
 * Copyright (C) 2003 Matjaz Breskvar <phoenix@bsemi.com>
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 * et al.
 */

// Dependency supplied by asm-generic/mm_hooks.h.

// The self-referential C preprocessor alias for init_new_context is implicit
// in the Rust declaration below.
unsafe extern "C" {
    pub fn init_new_context(tsk: *mut task_struct, mm: *mut mm_struct) -> ::core::ffi::c_int;
}

// The self-referential C preprocessor alias for destroy_context is implicit
// in the Rust declaration below.
unsafe extern "C" {
    pub fn destroy_context(mm: *mut mm_struct);
    pub fn switch_mm(
        prev: *mut mm_struct,
        next: *mut mm_struct,
        tsk: *mut task_struct,
    );
}

#[inline]
pub unsafe fn activate_mm(prev: *mut mm_struct, next: *mut mm_struct) {
    unsafe { switch_mm(prev, next, ::core::ptr::null_mut()) };
}

/* current active pgd - this is similar to other processors pgd
 * registers like cr3 on the i386
 */

// Defined in arch/openrisc/mm/fault.c.  The zero-length array represents the
// incomplete array declaration from C; access is inherently unsafe.
unsafe extern "C" {
    pub static mut current_pgd: [*mut pgd_t; 0];
}

// Dependency supplied by asm-generic/mmu_context.h.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
