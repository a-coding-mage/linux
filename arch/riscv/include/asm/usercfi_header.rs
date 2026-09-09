/* SPDX-License-Identifier: GPL-2.0
 * Copyright (C) 2024 Rivos, Inc.
 * Deepak Gupta <debug@rivosinc.com>
 */

use core::ffi::{c_int, c_ulong};

pub const CMDLINE_DISABLE_RISCV_USERCFI_FCFI: c_int = 1;
pub const CMDLINE_DISABLE_RISCV_USERCFI_BCFI: c_int = 2;
pub const CMDLINE_DISABLE_RISCV_USERCFI: c_int = 3;

#[repr(C)]
pub struct task_struct;
#[repr(C)]
pub struct kernel_clone_args;

unsafe extern "C" {
    pub static mut riscv_nousercfi: c_ulong;
}

/* CONFIG_RISCV_USER_CFI controls the declarations and implementations below. */
#[cfg(feature = "CONFIG_RISCV_USER_CFI")]
#[repr(C)]
pub struct cfi_state {
    /* The original members are one-bit unsigned-long bitfields. */
    pub ubcfi_en: c_ulong,
    pub ubcfi_locked: c_ulong,
    pub ufcfi_en: c_ulong,
    pub ufcfi_locked: c_ulong,
    pub user_shdw_stk: c_ulong,
    pub shdw_stk_base: c_ulong,
    pub shdw_stk_size: c_ulong,
}

#[cfg(feature = "CONFIG_RISCV_USER_CFI")]
unsafe extern "C" {
    pub fn shstk_alloc_thread_stack(tsk: *mut task_struct, args: *const kernel_clone_args) -> c_ulong;
    pub fn shstk_release(tsk: *mut task_struct);
    pub fn set_shstk_base(task: *mut task_struct, shstk_addr: c_ulong, size: c_ulong);
    pub fn get_shstk_base(task: *mut task_struct, size: *mut c_ulong) -> c_ulong;
    pub fn set_active_shstk(task: *mut task_struct, shstk_addr: c_ulong);
    pub fn is_shstk_enabled(task: *mut task_struct) -> bool;
    pub fn is_shstk_locked(task: *mut task_struct) -> bool;
    pub fn is_shstk_allocated(task: *mut task_struct) -> bool;
    pub fn set_shstk_lock(task: *mut task_struct, lock: bool);
    pub fn set_shstk_status(task: *mut task_struct, enable: bool);
    pub fn get_active_shstk(task: *mut task_struct) -> c_ulong;
    pub fn restore_user_shstk(tsk: *mut task_struct, shstk_ptr: c_ulong) -> c_int;
    pub fn save_user_shstk(tsk: *mut task_struct, saved_shstk_ptr: *mut c_ulong) -> c_int;
    pub fn is_indir_lp_enabled(task: *mut task_struct) -> bool;
    pub fn is_indir_lp_locked(task: *mut task_struct) -> bool;
    pub fn set_indir_lp_status(task: *mut task_struct, enable: bool);
    pub fn set_indir_lp_lock(task: *mut task_struct, lock: bool);
}

/* These constants are supplied by linux/prctl.h in the C header. */
#[cfg(feature = "CONFIG_RISCV_USER_CFI")]
pub const PR_SHADOW_STACK_SUPPORTED_STATUS_MASK: c_ulong = PR_SHADOW_STACK_ENABLE;
#[cfg(feature = "CONFIG_RISCV_USER_CFI")]
pub const PR_CFI_SUPPORTED_STATUS_MASK: c_ulong = PR_CFI_ENABLE | PR_CFI_DISABLE | PR_CFI_LOCK;

#[cfg(not(feature = "CONFIG_RISCV_USER_CFI"))]
pub unsafe fn shstk_alloc_thread_stack(_tsk: *mut task_struct, _args: *const kernel_clone_args) -> c_ulong { 0 }
#[cfg(not(feature = "CONFIG_RISCV_USER_CFI"))]
pub unsafe fn shstk_release(_tsk: *mut task_struct) {}
#[cfg(not(feature = "CONFIG_RISCV_USER_CFI"))]
pub unsafe fn get_shstk_base(_task: *mut task_struct, _size: *mut c_ulong) -> c_ulong { 0 }
#[cfg(not(feature = "CONFIG_RISCV_USER_CFI"))]
pub unsafe fn set_shstk_base(_task: *mut task_struct, _shstk_addr: c_ulong, _size: c_ulong) {}
#[cfg(not(feature = "CONFIG_RISCV_USER_CFI"))]
pub unsafe fn set_active_shstk(_task: *mut task_struct, _shstk_addr: c_ulong) {}
#[cfg(not(feature = "CONFIG_RISCV_USER_CFI"))]
pub unsafe fn is_shstk_enabled(_task: *mut task_struct) -> bool { false }
#[cfg(not(feature = "CONFIG_RISCV_USER_CFI"))]
pub unsafe fn is_shstk_locked(_task: *mut task_struct) -> bool { false }
#[cfg(not(feature = "CONFIG_RISCV_USER_CFI"))]
pub unsafe fn is_shstk_allocated(_task: *mut task_struct) -> bool { false }
#[cfg(not(feature = "CONFIG_RISCV_USER_CFI"))]
pub unsafe fn set_shstk_lock(_task: *mut task_struct, _lock: bool) {}
#[cfg(not(feature = "CONFIG_RISCV_USER_CFI"))]
pub unsafe fn set_shstk_status(_task: *mut task_struct, _enable: bool) {}
#[cfg(not(feature = "CONFIG_RISCV_USER_CFI"))]
pub unsafe fn is_indir_lp_enabled(_task: *mut task_struct) -> bool { false }
#[cfg(not(feature = "CONFIG_RISCV_USER_CFI"))]
pub unsafe fn is_indir_lp_locked(_task: *mut task_struct) -> bool { false }
#[cfg(not(feature = "CONFIG_RISCV_USER_CFI"))]
pub unsafe fn set_indir_lp_status(_task: *mut task_struct, _enable: bool) {}
#[cfg(not(feature = "CONFIG_RISCV_USER_CFI"))]
pub unsafe fn set_indir_lp_lock(_task: *mut task_struct, _lock: bool) {}
#[cfg(not(feature = "CONFIG_RISCV_USER_CFI"))]
pub unsafe fn restore_user_shstk(_tsk: *mut task_struct, _shstk_ptr: c_ulong) -> c_int { -22 }
#[cfg(not(feature = "CONFIG_RISCV_USER_CFI"))]
pub unsafe fn save_user_shstk(_tsk: *mut task_struct, _saved_shstk_ptr: *mut c_ulong) -> c_int { -22 }
#[cfg(not(feature = "CONFIG_RISCV_USER_CFI"))]
pub unsafe fn get_active_shstk(_task: *mut task_struct) -> c_ulong { 0 }

unsafe extern "C" {
    pub fn is_user_shstk_enabled() -> bool;
    pub fn is_user_lpad_enabled() -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
