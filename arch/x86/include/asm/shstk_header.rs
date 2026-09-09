/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: _ASM_X86_SHSTK_H
// The C declarations below are intended for non-assembler consumers.

use core::ffi::c_int;

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ksignal {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_X86_USER_SHADOW_STACK")]
#[repr(C)]
pub struct thread_shstk {
    pub base: u64,
    pub size: u64,
}

#[cfg(feature = "CONFIG_X86_USER_SHADOW_STACK")]
extern "C" {
    pub fn shstk_prctl(task: *mut task_struct, option: c_int, arg2: usize) -> isize;
    pub fn reset_thread_features();
    pub fn shstk_alloc_thread_stack(
        p: *mut task_struct,
        clone_flags: u64,
        stack_size: usize,
    ) -> usize;
    pub fn shstk_free(p: *mut task_struct);
    pub fn setup_signal_shadow_stack(ksig: *mut ksignal) -> c_int;
    pub fn restore_signal_shadow_stack() -> c_int;
    pub fn shstk_update_last_frame(val: usize) -> c_int;
    pub fn shstk_is_enabled() -> bool;
    pub fn shstk_pop(val: *mut u64) -> c_int;
    pub fn shstk_push(val: u64) -> c_int;
}

#[cfg(not(feature = "CONFIG_X86_USER_SHADOW_STACK"))]
#[inline]
pub unsafe fn shstk_prctl(_task: *mut task_struct, _option: c_int, _arg2: usize) -> isize {
    -EINVAL as isize
}

#[cfg(not(feature = "CONFIG_X86_USER_SHADOW_STACK"))]
#[inline]
pub unsafe fn reset_thread_features() {}

#[cfg(not(feature = "CONFIG_X86_USER_SHADOW_STACK"))]
#[inline]
pub unsafe fn shstk_alloc_thread_stack(
    _p: *mut task_struct,
    _clone_flags: u64,
    _stack_size: usize,
) -> usize {
    0
}

#[cfg(not(feature = "CONFIG_X86_USER_SHADOW_STACK"))]
#[inline]
pub unsafe fn shstk_free(_p: *mut task_struct) {}

#[cfg(not(feature = "CONFIG_X86_USER_SHADOW_STACK"))]
#[inline]
pub unsafe fn setup_signal_shadow_stack(_ksig: *mut ksignal) -> c_int {
    0
}

#[cfg(not(feature = "CONFIG_X86_USER_SHADOW_STACK"))]
#[inline]
pub unsafe fn restore_signal_shadow_stack() -> c_int {
    0
}

#[cfg(not(feature = "CONFIG_X86_USER_SHADOW_STACK"))]
#[inline]
pub unsafe fn shstk_update_last_frame(_val: usize) -> c_int {
    0
}

#[cfg(not(feature = "CONFIG_X86_USER_SHADOW_STACK"))]
#[inline]
pub unsafe fn shstk_is_enabled() -> bool {
    false
}

#[cfg(not(feature = "CONFIG_X86_USER_SHADOW_STACK"))]
#[inline]
pub unsafe fn shstk_pop(_val: *mut u64) -> c_int {
    -ENOTSUPP as c_int
}

#[cfg(not(feature = "CONFIG_X86_USER_SHADOW_STACK"))]
#[inline]
pub unsafe fn shstk_push(_val: u64) -> c_int {
    -ENOTSUPP as c_int
}

// EINVAL and ENOTSUPP are supplied by the kernel error-code dependency.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
