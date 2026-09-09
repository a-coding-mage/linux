/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Define generic no-op hooks for arch_dup_mmap and arch_exit_mmap
 * to be included in asm-FOO/mmu_context.h for any arch FOO which
 * doesn't need to hook these.
 */

// Dependency types are supplied by the surrounding translation unit.
use crate::{mm_struct, vm_area_struct};

#[inline]
pub fn arch_dup_mmap(oldmm: *mut mm_struct, mm: *mut mm_struct) -> i32 {
    let _ = oldmm;
    let _ = mm;
    0
}

#[inline]
pub fn arch_exit_mmap(mm: *mut mm_struct) {
    let _ = mm;
}

#[inline]
pub fn arch_vma_access_permitted(
    vma: *mut vm_area_struct,
    write: bool,
    execute: bool,
    foreign: bool,
) -> bool {
    let _ = vma;
    let _ = write;
    let _ = execute;
    let _ = foreign;
    /* by default, allow everything */
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
