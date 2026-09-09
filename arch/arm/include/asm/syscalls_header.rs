/* SPDX-License-Identifier: GPL-2.0-only */

// C dependencies: <linux/linkage.h> and <linux/types.h>.
// `asmlinkage` and the `__user` annotation have no direct Rust syntax;
// declarations retain the corresponding C ABI and raw-pointer behavior.

use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct oldabi_stat64 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct oabi_epoll_event {
    _private: [u8; 0],
}

#[repr(C)]
pub struct oabi_sembuf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct old_timespec32 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sockaddr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct user_msghdr {
    _private: [u8; 0],
}

extern "C" {
    pub fn sys_sigreturn(regs: *mut pt_regs) -> core::ffi::c_int;
    pub fn sys_rt_sigreturn(regs: *mut pt_regs) -> core::ffi::c_int;
    pub fn sys_arm_fadvise64_64(
        fd: core::ffi::c_int,
        advice: core::ffi::c_int,
        offset: i64,
        len: i64,
    ) -> core::ffi::c_long;

    pub fn sys_oabi_stat64(
        filename: *const c_char,
        statbuf: *mut oldabi_stat64,
    ) -> core::ffi::c_long;
    pub fn sys_oabi_lstat64(
        filename: *const c_char,
        statbuf: *mut oldabi_stat64,
    ) -> core::ffi::c_long;
    pub fn sys_oabi_fstat64(
        fd: core::ffi::c_ulong,
        statbuf: *mut oldabi_stat64,
    ) -> core::ffi::c_long;
    pub fn sys_oabi_fstatat64(
        dfd: core::ffi::c_int,
        filename: *const c_char,
        statbuf: *mut oldabi_stat64,
        flag: core::ffi::c_int,
    ) -> core::ffi::c_long;
    pub fn sys_oabi_fcntl64(
        fd: core::ffi::c_uint,
        cmd: core::ffi::c_uint,
        arg: core::ffi::c_ulong,
    ) -> core::ffi::c_long;
    pub fn sys_oabi_epoll_ctl(
        epfd: core::ffi::c_int,
        op: core::ffi::c_int,
        fd: core::ffi::c_int,
        event: *mut oabi_epoll_event,
    ) -> core::ffi::c_long;
    pub fn sys_oabi_semtimedop(
        semid: core::ffi::c_int,
        tsops: *mut oabi_sembuf,
        nsops: core::ffi::c_uint,
        timeout: *const old_timespec32,
    ) -> core::ffi::c_long;
    pub fn sys_oabi_semop(
        semid: core::ffi::c_int,
        tsops: *mut oabi_sembuf,
        nsops: core::ffi::c_uint,
    ) -> core::ffi::c_long;
    pub fn sys_oabi_ipc(
        call: core::ffi::c_uint,
        first: core::ffi::c_int,
        second: core::ffi::c_int,
        third: core::ffi::c_int,
        ptr: *mut c_void,
        fifth: core::ffi::c_long,
    ) -> core::ffi::c_int;
    pub fn sys_oabi_bind(
        fd: core::ffi::c_int,
        addr: *mut sockaddr,
        addrlen: core::ffi::c_int,
    ) -> core::ffi::c_long;
    pub fn sys_oabi_connect(
        fd: core::ffi::c_int,
        addr: *mut sockaddr,
        addrlen: core::ffi::c_int,
    ) -> core::ffi::c_long;
    pub fn sys_oabi_sendto(
        fd: core::ffi::c_int,
        buff: *mut c_void,
        len: usize,
        flags: core::ffi::c_uint,
        addr: *mut sockaddr,
        addrlen: core::ffi::c_int,
    ) -> core::ffi::c_long;
    pub fn sys_oabi_sendmsg(
        fd: core::ffi::c_int,
        msg: *mut user_msghdr,
        flags: core::ffi::c_uint,
    ) -> core::ffi::c_long;
    pub fn sys_oabi_socketcall(
        call: core::ffi::c_int,
        args: *mut core::ffi::c_ulong,
    ) -> core::ffi::c_long;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
