/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2021 Sifive.
 */

// Dependency: <asm/alternative-macros.h>
// This header is not emitted for assembler builds.

#[cfg(CONFIG_RISCV_ALTERNATIVE)]
use core::ffi::c_void;

#[cfg(CONFIG_RISCV_ALTERNATIVE)]
#[inline]
pub const fn patch_id_cpufeature_id(p: u32) -> u16 {
    (p & 0xffff) as u16
}

#[cfg(CONFIG_RISCV_ALTERNATIVE)]
#[inline]
pub const fn patch_id_cpufeature_value(p: u32) -> u16 {
    (p >> 16) as u16
}

#[cfg(CONFIG_RISCV_ALTERNATIVE)]
pub const RISCV_ALTERNATIVES_BOOT: u32 = 0; // alternatives applied during regular boot
#[cfg(CONFIG_RISCV_ALTERNATIVE)]
pub const RISCV_ALTERNATIVES_MODULE: u32 = 1; // alternatives applied during module-init
#[cfg(CONFIG_RISCV_ALTERNATIVE)]
pub const RISCV_ALTERNATIVES_EARLY_BOOT: u32 = 2; // alternatives applied before mmu start

#[repr(C)]
#[cfg(CONFIG_RISCV_ALTERNATIVE)]
pub struct alt_entry {
    pub old_offset: i32, // offset relative to original instruction or data
    pub alt_offset: i32, // offset relative to replacement instruction or data
    pub vendor_id: u16, // CPU vendor ID
    pub alt_len: u16, // The replacement size
    pub patch_id: u32, // The patch ID (erratum ID or cpufeature ID)
}

// Add the relative offset to the address of the offset to get the absolute address.
#[cfg(CONFIG_RISCV_ALTERNATIVE)]
#[inline]
pub unsafe fn __alt_ptr(a: *const alt_entry, field_offset: *const i32) -> *mut c_void {
    (field_offset as *const u8).offset((*field_offset) as isize) as *mut c_void
}

#[cfg(CONFIG_RISCV_ALTERNATIVE)]
#[inline]
pub unsafe fn alt_old_ptr(a: *const alt_entry) -> *mut c_void {
    __alt_ptr(a, core::ptr::addr_of!((*a).old_offset))
}

#[cfg(CONFIG_RISCV_ALTERNATIVE)]
#[inline]
pub unsafe fn alt_alt_ptr(a: *const alt_entry) -> *mut c_void {
    __alt_ptr(a, core::ptr::addr_of!((*a).alt_offset))
}

#[cfg(CONFIG_RISCV_ALTERNATIVE)]
extern "C" {
    pub fn apply_boot_alternatives();
    pub fn apply_early_boot_alternatives();
    pub fn apply_module_alternatives(start: *mut c_void, length: usize);

    pub fn riscv_alternative_fix_offsets(
        alt_ptr: *mut c_void,
        len: u32,
        patch_offset: i32,
    );

    pub fn andes_errata_patch_func(
        begin: *mut alt_entry,
        end: *mut alt_entry,
        archid: usize,
        impid: usize,
        stage: u32,
    );
    pub fn mips_errata_patch_func(
        begin: *mut alt_entry,
        end: *mut alt_entry,
        archid: usize,
        impid: usize,
        stage: u32,
    );
    pub fn sifive_errata_patch_func(
        begin: *mut alt_entry,
        end: *mut alt_entry,
        archid: usize,
        impid: usize,
        stage: u32,
    );
    pub fn thead_errata_patch_func(
        begin: *mut alt_entry,
        end: *mut alt_entry,
        archid: usize,
        impid: usize,
        stage: u32,
    );

    pub fn riscv_cpufeature_patch_func(
        begin: *mut alt_entry,
        end: *mut alt_entry,
        stage: u32,
    );
}

#[cfg(not(CONFIG_RISCV_ALTERNATIVE))]
#[inline]
pub fn apply_boot_alternatives() {}

#[cfg(not(CONFIG_RISCV_ALTERNATIVE))]
#[inline]
pub fn apply_early_boot_alternatives() {}

#[cfg(not(CONFIG_RISCV_ALTERNATIVE))]
#[inline]
pub fn apply_module_alternatives(_start: *mut core::ffi::c_void, _length: usize) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
