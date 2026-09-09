/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Syscall support for the Hexagon architecture
 *
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 */

/* Translated from the C header; included declarations are supplied externally. */

use core::ffi::c_void;

pub type syscall_fn = unsafe extern "C" fn(
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
) -> isize;

extern "C" {
    pub static mut sys_call_table: [*mut c_void; 0];
}

#[allow(non_camel_case_types)]
pub struct task_struct;

#[repr(C)]
pub struct pt_regs {
    pub r00: usize,
    pub r01: usize,
    pub r02: usize,
    pub r03: usize,
    pub r04: usize,
    pub r05: usize,
    pub r06: usize,
}

pub unsafe fn syscall_get_nr(_task: *mut task_struct, regs: *mut pt_regs) -> isize {
    (*regs).r06 as isize
}

pub unsafe fn syscall_set_nr(_task: *mut task_struct, regs: *mut pt_regs, nr: i32) {
    (*regs).r06 = nr as usize;
}

pub unsafe fn syscall_get_arguments(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    args: *mut usize,
) {
    core::ptr::copy_nonoverlapping(
        core::ptr::addr_of!((*regs).r00),
        args,
        6,
    );
}

pub unsafe fn syscall_set_arguments(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    args: *const usize,
) {
    core::ptr::copy_nonoverlapping(
        args,
        core::ptr::addr_of_mut!((*regs).r00),
        6,
    );
}

pub unsafe fn syscall_get_error(_task: *mut task_struct, regs: *mut pt_regs) -> isize {
    let value = (*regs).r00;
    /* IS_ERR_VALUE(regs->r00), supplied by the included Linux error header. */
    if value >= (-(4095isize) as usize) {
        value as isize
    } else {
        0
    }
}

pub unsafe fn syscall_get_return_value(_task: *mut task_struct, regs: *mut pt_regs) -> isize {
    (*regs).r00 as isize
}

pub unsafe fn syscall_set_return_value(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    error: i32,
    val: isize,
) {
    (*regs).r00 = if error != 0 { error as isize } else { val } as usize;
}

pub unsafe fn syscall_get_arch(_task: *mut task_struct) -> i32 {
    /* AUDIT_ARCH_HEXAGON, supplied by the included Linux audit header. */
    AUDIT_ARCH_HEXAGON
}

extern "C" {
    pub static AUDIT_ARCH_HEXAGON: i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
