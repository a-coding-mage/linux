/* SPDX-License-Identifier: GPL-2.0-only */

// C header guard omitted in Rust translation.
// Dependencies from linux/linkage.h and linux/compat.h are supplied externally.

extern "C" {
    pub fn sys_sigreturn();
    pub fn sys_rt_sigreturn();
    pub fn sysm_pipe() -> ::core::ffi::c_int;
    pub fn mipsmt_sys_sched_setaffinity(
        pid: pid_t,
        len: u32,
        user_mask_ptr: *mut ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_long;
    pub fn mipsmt_sys_sched_getaffinity(
        pid: pid_t,
        len: u32,
        user_mask_ptr: *mut ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_long;
    pub fn sys32_fallocate(
        fd: ::core::ffi::c_int,
        mode: ::core::ffi::c_int,
        offset_a2: u32,
        offset_a3: u32,
        len_a4: u32,
        len_a5: u32,
    ) -> ::core::ffi::c_long;
    pub fn sys32_fadvise64_64(
        fd: ::core::ffi::c_int,
        __pad: ::core::ffi::c_int,
        a2: ::core::ffi::c_ulong,
        a3: ::core::ffi::c_ulong,
        a4: ::core::ffi::c_ulong,
        a5: ::core::ffi::c_ulong,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    pub fn sys32_readahead(
        fd: ::core::ffi::c_int,
        pad0: u32,
        a2: u64,
        a3: u64,
        count: usize,
    ) -> isize;
    pub fn sys32_sync_file_range(
        fd: ::core::ffi::c_int,
        __pad: ::core::ffi::c_int,
        a2: ::core::ffi::c_ulong,
        a3: ::core::ffi::c_ulong,
        a4: ::core::ffi::c_ulong,
        a5: ::core::ffi::c_ulong,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    pub fn sys32_rt_sigreturn();
    pub fn sys32_sigreturn();
    pub fn sys32_sigsuspend(uset: *mut compat_sigset_t) -> ::core::ffi::c_int;
    pub fn sysn32_rt_sigreturn();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
