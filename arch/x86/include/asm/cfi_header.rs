/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Clang Control Flow Integrity (CFI) support.
 *
 * Copyright (C) 2022 Google LLC
 *
 * Dependencies supplied by the surrounding kernel translation:
 * `bug_trap_type`, `BUG_TRAP_TYPE_NONE`, `u8`, `u32`, `bool`, `pt_regs`,
 * `IBT_NOSEAL`, and `__stringify`.
 */

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cfi_mode {
    CFI_AUTO,   /* FineIBT if hardware has IBT, otherwise kCFI */
    CFI_OFF,    /* Taditional / IBT depending on .config */
    CFI_KCFI,   /* Optionally CALL_PADDING, IBT, RETPOLINE */
    CFI_FINEIBT, /* see arch/x86/kernel/alternative.c */
}

unsafe extern "C" {
    pub static mut cfi_mode: cfi_mode;
}

// CONFIG_FINEIBT_BHI is a build-time condition from the original header.
#[cfg(CONFIG_FINEIBT_BHI)]
unsafe extern "C" {
    pub static mut cfi_bhi: bool;
}

#[cfg(not(CONFIG_FINEIBT_BHI))]
pub const cfi_bhi: i32 = 0;

pub type bhi_thunk = [u8; 32];

unsafe extern "C" {
    pub static mut __bhi_args: bhi_thunk;
    pub static mut __bhi_args_end: bhi_thunk;
}

#[repr(C)]
pub struct pt_regs;

// CONFIG_CALL_PADDING is a build-time condition from the original header.
#[cfg(CONFIG_CALL_PADDING)]
pub const CFI_OFFSET: i32 = CONFIG_FUNCTION_PADDING_CFI as i32 + 5;

#[cfg(not(CONFIG_CALL_PADDING))]
pub const CFI_OFFSET: i32 = 5;

// CONFIG_CFI is a build-time condition from the original header.
#[cfg(CONFIG_CFI)]
unsafe extern "C" {
    pub fn handle_cfi_failure(regs: *mut pt_regs) -> bug_trap_type;
    pub fn cfi_get_func_hash(func: *mut core::ffi::c_void) -> u32;
    pub fn cfi_get_func_arity(func: *mut core::ffi::c_void) -> i32;
}

#[cfg(CONFIG_CFI)]
#[inline]
pub unsafe fn cfi_get_offset() -> i32 {
    match cfi_mode {
        cfi_mode::CFI_FINEIBT => 16, // fineibt_prefix_size
        cfi_mode::CFI_KCFI => CFI_OFFSET,
        _ => 0,
    }
}

// CONFIG_FINEIBT is a build-time condition from the original header.
#[cfg(all(CONFIG_CFI, CONFIG_FINEIBT))]
unsafe extern "C" {
    pub fn decode_fineibt_insn(
        regs: *mut pt_regs,
        target: *mut usize,
        type_: *mut u32,
    ) -> bool;
}

#[cfg(all(CONFIG_CFI, not(CONFIG_FINEIBT)))]
#[inline]
pub unsafe fn decode_fineibt_insn(
    _regs: *mut pt_regs,
    _target: *mut usize,
    _type_: *mut u32,
) -> bool {
    false
}

#[cfg(not(CONFIG_CFI))]
#[inline]
pub unsafe fn handle_cfi_failure(_regs: *mut pt_regs) -> bug_trap_type {
    BUG_TRAP_TYPE_NONE
}

#[cfg(not(CONFIG_CFI))]
#[inline]
pub unsafe fn cfi_get_func_arity(_func: *mut core::ffi::c_void) -> i32 {
    0
}

// HAS_KERNEL_IBT == 1 is a build-time condition from the original header.
#[cfg(HAS_KERNEL_IBT)]
#[macro_export]
macro_rules! CFI_NOSEAL {
    ($x:tt) => {
        asm!(IBT_NOSEAL(__stringify!($x)));
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
