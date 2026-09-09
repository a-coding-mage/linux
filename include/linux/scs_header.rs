/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Shadow Call Stack support.
 *
 * Copyright (C) 2019 Google LLC
 */

use core::ffi::c_void;

/* The C header includes linux/gfp.h, linux/poison.h, linux/sched.h, and
 * linux/sizes.h; their names and declarations are supplied externally. */

#[cfg(feature = "CONFIG_SHADOW_CALL_STACK")]
pub const SCS_ORDER: usize = 0;
#[cfg(feature = "CONFIG_SHADOW_CALL_STACK")]
pub const SCS_SIZE: usize = PAGE_SIZE << SCS_ORDER;
#[cfg(feature = "CONFIG_SHADOW_CALL_STACK")]
pub const GFP_SCS: usize = GFP_KERNEL | __GFP_ZERO;

/* An illegal pointer value to mark the end of the shadow stack. */
#[cfg(feature = "CONFIG_SHADOW_CALL_STACK")]
pub const SCS_END_MAGIC: usize = 0x5f6usize + POISON_POINTER_DELTA;

#[cfg(feature = "CONFIG_SHADOW_CALL_STACK")]
extern "C" {
    pub fn task_thread_info(tsk: *mut task_struct) -> *mut thread_info;
    pub fn scs_alloc(node: i32) -> *mut c_void;
    pub fn scs_free(s: *mut c_void);
    pub fn scs_init();
    pub fn scs_prepare(tsk: *mut task_struct, node: i32) -> i32;
    pub fn scs_release(tsk: *mut task_struct);
}

#[cfg(feature = "CONFIG_SHADOW_CALL_STACK")]
#[inline]
pub unsafe fn task_scs(tsk: *mut task_struct) -> *mut c_void {
    (*task_thread_info(tsk)).scs_base
}

#[cfg(feature = "CONFIG_SHADOW_CALL_STACK")]
#[inline]
pub unsafe fn task_scs_sp(tsk: *mut task_struct) -> *mut usize {
    (*task_thread_info(tsk)).scs_sp
}

#[cfg(feature = "CONFIG_SHADOW_CALL_STACK")]
#[inline]
pub unsafe fn scs_task_reset(tsk: *mut task_struct) {
    /* Reset the shadow stack to the base address in case the task is reused. */
    (*task_thread_info(tsk)).scs_sp = (*task_thread_info(tsk)).scs_base as *mut usize;
}

#[cfg(feature = "CONFIG_SHADOW_CALL_STACK")]
#[inline]
pub unsafe fn __scs_magic(s: *mut c_void) -> *mut usize {
    (s as *mut u8).add(SCS_SIZE) as *mut usize
        .sub(1)
}

#[cfg(feature = "CONFIG_SHADOW_CALL_STACK")]
#[inline]
pub unsafe fn task_scs_end_corrupted(tsk: *mut task_struct) -> bool {
    let magic = __scs_magic(task_scs(tsk));
    let sz = (task_scs_sp(tsk) as usize - task_scs(tsk) as usize) as usize;

    sz >= SCS_SIZE - 1 || core::ptr::read_volatile(magic) != SCS_END_MAGIC
}

#[cfg(feature = "CONFIG_SHADOW_CALL_STACK")]
extern "C" {
    pub static mut dynamic_scs_enabled: static_key_false;
}

#[cfg(all(feature = "CONFIG_SHADOW_CALL_STACK", not(feature = "CONFIG_DYNAMIC_SCS")))]
#[inline]
pub fn scs_is_dynamic() -> bool { false }

#[cfg(all(feature = "CONFIG_SHADOW_CALL_STACK", feature = "CONFIG_DYNAMIC_SCS"))]
#[inline]
pub unsafe fn scs_is_dynamic() -> bool {
    static_branch_likely(&dynamic_scs_enabled)
}

#[cfg(feature = "CONFIG_SHADOW_CALL_STACK")]
#[inline]
pub unsafe fn scs_is_enabled() -> bool {
    if cfg!(feature = "CONFIG_DYNAMIC_SCS") { scs_is_dynamic() } else { true }
}

#[cfg(not(feature = "CONFIG_SHADOW_CALL_STACK"))]
#[inline] pub fn scs_alloc(_node: i32) -> *mut c_void { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_SHADOW_CALL_STACK"))]
#[inline] pub fn scs_free(_s: *mut c_void) {}
#[cfg(not(feature = "CONFIG_SHADOW_CALL_STACK"))]
#[inline] pub fn scs_init() {}
#[cfg(not(feature = "CONFIG_SHADOW_CALL_STACK"))]
#[inline] pub fn scs_task_reset(_tsk: *mut task_struct) {}
#[cfg(not(feature = "CONFIG_SHADOW_CALL_STACK"))]
#[inline] pub fn scs_prepare(_tsk: *mut task_struct, _node: i32) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_SHADOW_CALL_STACK"))]
#[inline] pub fn scs_release(_tsk: *mut task_struct) {}
#[cfg(not(feature = "CONFIG_SHADOW_CALL_STACK"))]
#[inline] pub fn task_scs_end_corrupted(_tsk: *mut task_struct) -> bool { false }
#[cfg(not(feature = "CONFIG_SHADOW_CALL_STACK"))]
#[inline] pub fn scs_is_enabled() -> bool { false }
#[cfg(not(feature = "CONFIG_SHADOW_CALL_STACK"))]
#[inline] pub fn scs_is_dynamic() -> bool { false }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
