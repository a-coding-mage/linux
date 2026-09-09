/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020 Collabora Ltd.
 */

/* Declarations supplied by the Linux scheduler, thread-info, and syscall
 * user-dispatch type definitions are external dependencies of this header. */

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_SYSCALL_USER_DISPATCH")]
extern "C" {
    pub fn syscall_user_dispatch(regs: *mut pt_regs) -> bool;

    pub fn set_syscall_user_dispatch(
        mode: ::core::ffi::c_ulong,
        offset: ::core::ffi::c_ulong,
        len: ::core::ffi::c_ulong,
        selector: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;

    pub fn syscall_user_dispatch_get_config(
        task: *mut task_struct,
        size: ::core::ffi::c_ulong,
        data: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;

    pub fn syscall_user_dispatch_set_config(
        task: *mut task_struct,
        size: ::core::ffi::c_ulong,
        data: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}

/* Supplied by the scheduler definitions included by the original header. */
#[allow(non_camel_case_types)]
pub enum task_struct {}

#[cfg(feature = "CONFIG_SYSCALL_USER_DISPATCH")]
#[inline(always)]
pub unsafe fn syscall_user_dispatch_clear_on_dispatch() -> bool {
    /* `current->syscall_dispatch.on_dispatch` is provided by the external
     * task_struct definition. */
    if !current_syscall_dispatch_on_dispatch() {
        return false;
    }

    clear_current_syscall_dispatch_on_dispatch();
    true
}

#[cfg(feature = "CONFIG_SYSCALL_USER_DISPATCH")]
extern "C" {
    fn current_syscall_dispatch_on_dispatch() -> bool;
    fn clear_current_syscall_dispatch_on_dispatch();
}

#[cfg(feature = "CONFIG_SYSCALL_USER_DISPATCH")]
#[inline(always)]
pub unsafe fn clear_syscall_work_syscall_user_dispatch(tsk: *mut task_struct) {
    clear_task_syscall_work(tsk, SYSCALL_USER_DISPATCH);
}

#[cfg(feature = "CONFIG_SYSCALL_USER_DISPATCH")]
extern "C" {
    fn clear_task_syscall_work(tsk: *mut task_struct, work: ::core::ffi::c_ulong);
}

#[cfg(feature = "CONFIG_SYSCALL_USER_DISPATCH")]
pub const SYSCALL_USER_DISPATCH: ::core::ffi::c_ulong = 0;

#[cfg(not(feature = "CONFIG_SYSCALL_USER_DISPATCH"))]
#[inline(always)]
pub unsafe fn syscall_user_dispatch(_regs: *mut pt_regs) -> bool {
    false
}

#[cfg(not(feature = "CONFIG_SYSCALL_USER_DISPATCH"))]
#[inline(always)]
pub unsafe fn syscall_user_dispatch_clear_on_dispatch() -> bool {
    false
}

#[cfg(not(feature = "CONFIG_SYSCALL_USER_DISPATCH"))]
#[inline]
pub unsafe fn set_syscall_user_dispatch(
    _mode: ::core::ffi::c_ulong,
    _offset: ::core::ffi::c_ulong,
    _len: ::core::ffi::c_ulong,
    _selector: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    -22 /* -EINVAL */
}

#[cfg(not(feature = "CONFIG_SYSCALL_USER_DISPATCH"))]
#[inline]
pub unsafe fn clear_syscall_work_syscall_user_dispatch(_tsk: *mut task_struct) {}

#[cfg(not(feature = "CONFIG_SYSCALL_USER_DISPATCH"))]
#[inline]
pub unsafe fn syscall_user_dispatch_get_config(
    _task: *mut task_struct,
    _size: ::core::ffi::c_ulong,
    _data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    -22 /* -EINVAL */
}

#[cfg(not(feature = "CONFIG_SYSCALL_USER_DISPATCH"))]
#[inline]
pub unsafe fn syscall_user_dispatch_set_config(
    _task: *mut task_struct,
    _size: ::core::ffi::c_ulong,
    _data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    -22 /* -EINVAL */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
