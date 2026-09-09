/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Dependencies supplied by the surrounding kernel translation unit:
 * linux/tracepoint.h, linux/unistd.h, linux/trace_events.h,
 * linux/thread_info.h, and asm/ptrace.h.
 */

use core::ffi::{c_char, c_int};

/*
 * A syscall entry in the ftrace syscalls array.
 *
 * @name: name of the syscall
 * @syscall_nr: number of the syscall
 * @nb_args: number of parameters it takes
 * @user_arg_is_str: set if the arg for @user_arg_size is a string
 * @user_arg_size: holds @arg that has size of the user space to read
 * @user_mask: mask of @args that will read user space
 * @types: list of types as strings
 * @args: list of args as strings (args[i] matches types[i])
 * @enter_fields: list of fields for syscall_enter trace event
 * @enter_event: associated syscall_enter trace event
 * @exit_event: associated syscall_exit trace event
 */
#[repr(C)]
pub struct syscall_metadata {
    pub name: *const c_char,
    pub syscall_nr: c_int,
    /* C bit-fields: nb_args:7 and user_arg_is_str:1 share this byte. */
    pub nb_args_and_user_arg_is_str: u8,
    pub user_arg_size: i8,
    pub user_mask: i16,
    pub types: *const *const c_char,
    pub args: *const *const c_char,
    pub enter_fields: crate::list_head,
    pub enter_event: *mut crate::trace_event_call,
    pub exit_event: *mut crate::trace_event_call,
}

impl syscall_metadata {
    #[inline]
    pub fn nb_args(&self) -> u8 {
        self.nb_args_and_user_arg_is_str & 0x7f
    }

    #[inline]
    pub fn user_arg_is_str(&self) -> u8 {
        (self.nb_args_and_user_arg_is_str >> 7) & 1
    }
}

/* CONFIG_TRACEPOINTS && CONFIG_HAVE_SYSCALL_TRACEPOINTS */
#[cfg(all(feature = "CONFIG_TRACEPOINTS", feature = "CONFIG_HAVE_SYSCALL_TRACEPOINTS"))]
#[inline]
pub unsafe fn syscall_tracepoint_update(p: *mut crate::task_struct) {
    if crate::test_syscall_work(crate::SYSCALL_TRACEPOINT) {
        crate::set_task_syscall_work(p, crate::SYSCALL_TRACEPOINT);
    } else {
        crate::clear_task_syscall_work(p, crate::SYSCALL_TRACEPOINT);
    }
}

/* !CONFIG_TRACEPOINTS || !CONFIG_HAVE_SYSCALL_TRACEPOINTS */
#[cfg(not(all(feature = "CONFIG_TRACEPOINTS", feature = "CONFIG_HAVE_SYSCALL_TRACEPOINTS")))]
#[inline]
pub unsafe fn syscall_tracepoint_update(_p: *mut crate::task_struct) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
