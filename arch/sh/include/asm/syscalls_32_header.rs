/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: __ASM_SH_SYSCALLS_32_H
// Dependencies: linux/compiler.h, linux/linkage.h, and linux/types.h.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong};

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

extern "C" {
    pub fn sys_sigreturn() -> c_int;
    pub fn sys_rt_sigreturn() -> c_int;
    pub fn sys_sh_pipe() -> c_int;
    pub fn sys_pread_wrapper(
        fd: c_uint,
        buf: *mut c_char,
        count: usize,
        dummy: c_long,
        pos: i64,
    ) -> isize;
    pub fn sys_pwrite_wrapper(
        fd: c_uint,
        buf: *const c_char,
        count: usize,
        dummy: c_long,
        pos: i64,
    ) -> isize;
    pub fn sys_fadvise64_64_wrapper(
        fd: c_int,
        offset0: u32,
        offset1: u32,
        len0: u32,
        len1: u32,
        advice: c_int,
    ) -> c_int;

    /* Misc syscall related bits */
    pub fn do_syscall_trace_enter(regs: *mut pt_regs) -> c_long;
    pub fn do_syscall_trace_leave(regs: *mut pt_regs);
    pub fn do_notify_resume(
        regs: *mut pt_regs,
        save_r0: c_uint,
        thread_info_flags: c_ulong,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
