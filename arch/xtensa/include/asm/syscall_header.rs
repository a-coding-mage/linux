/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2007 Tensilica Inc.
 * Copyright (C) 2018 Cadence Design Systems Inc.
 */

// C header guard: _ASM_SYSCALL_H
// Dependencies: linux/err.h, asm/ptrace.h, and uapi/linux/audit.h.

#[inline]
pub unsafe fn syscall_get_arch(_task: *mut task_struct) -> i32 {
    AUDIT_ARCH_XTENSA
}

pub type syscall_t = unsafe extern "C" fn();
extern "C" {
    pub static mut sys_call_table: *mut syscall_t;
}

#[inline]
pub unsafe fn syscall_get_nr(_task: *mut task_struct, regs: *mut pt_regs) -> i64 {
    (*regs).syscall as i64
}

#[inline]
pub unsafe fn syscall_set_nr(_task: *mut task_struct, regs: *mut pt_regs, nr: i32) {
    (*regs).syscall = nr;
}

#[inline]
pub unsafe fn syscall_rollback(_task: *mut task_struct, _regs: *mut pt_regs) {
    /* Do nothing. */
}

#[inline]
pub unsafe fn syscall_get_error(_task: *mut task_struct, regs: *mut pt_regs) -> i64 {
    /* 0 if syscall succeeded, otherwise -Errorcode */
    if IS_ERR_VALUE((*regs).areg[2]) {
        (*regs).areg[2]
    } else {
        0
    }
}

#[inline]
pub unsafe fn syscall_get_return_value(_task: *mut task_struct, regs: *mut pt_regs) -> i64 {
    (*regs).areg[2]
}

#[inline]
pub unsafe fn syscall_set_return_value(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    error: i32,
    val: i64,
) {
    (*regs).areg[2] = if (error as i64) != 0 { error as i64 } else { val };
}

pub const XTENSA_SYSCALL_ARGUMENT_REGS: [u32; 6] = [6, 3, 4, 5, 8, 9];

#[inline]
pub unsafe fn syscall_get_arguments(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    args: *mut u64,
) {
    let reg: [u32; 6] = XTENSA_SYSCALL_ARGUMENT_REGS;
    let mut i: u32 = 0;
    while i < 6 {
        *args.add(i as usize) = (*regs).areg[reg[i as usize] as usize] as u64;
        i = i.wrapping_add(1);
    }
}

#[inline]
pub unsafe fn syscall_set_arguments(
    _task: *mut task_struct,
    regs: *mut pt_regs,
    args: *const u64,
) {
    let reg: [u32; 6] = XTENSA_SYSCALL_ARGUMENT_REGS;
    let mut i: u32 = 0;
    while i < 6 {
        (*regs).areg[reg[i as usize] as usize] = *args.add(i as usize) as i64;
        i = i.wrapping_add(1);
    }
}

// `asmlinkage` declarations from the C header.
extern "C" {
    pub fn xtensa_rt_sigreturn() -> i64;
    pub fn xtensa_shmat(arg1: i32, arg2: *mut core::ffi::c_char, arg3: i32) -> i64;
    pub fn xtensa_fadvise64_64(arg1: i32, arg2: i32, arg3: u64, arg4: u64) -> i64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
