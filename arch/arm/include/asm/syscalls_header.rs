/* SPDX-License-Identifier: GPL-2.0-only */

// C dependencies: <linux/linkage.h> and <linux/types.h>.
// `asmlinkage` and the `__user` annotation have no direct Rust syntax;
// declarations retain the corresponding C ABI and raw-pointer behavior.

use kernel::ffi::{c_char, c_void};

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
    pub fn sys_sigreturn(regs: *mut pt_regs) -> kernel::ffi::c_int;
    pub fn sys_rt_sigreturn(regs: *mut pt_regs) -> kernel::ffi::c_int;
    pub fn sys_arm_fadvise64_64(
        fd: kernel::ffi::c_int,
        advice: kernel::ffi::c_int,
        offset: i64,
        len: i64,
    ) -> kernel::ffi::c_long;

    pub fn sys_oabi_stat64(
        filename: *const c_char,
        statbuf: *mut oldabi_stat64,
    ) -> kernel::ffi::c_long;
    pub fn sys_oabi_lstat64(
        filename: *const c_char,
        statbuf: *mut oldabi_stat64,
    ) -> kernel::ffi::c_long;
    pub fn sys_oabi_fstat64(
        fd: kernel::ffi::c_ulong,
        statbuf: *mut oldabi_stat64,
    ) -> kernel::ffi::c_long;
    pub fn sys_oabi_fstatat64(
        dfd: kernel::ffi::c_int,
        filename: *const c_char,
        statbuf: *mut oldabi_stat64,
        flag: kernel::ffi::c_int,
    ) -> kernel::ffi::c_long;
    pub fn sys_oabi_fcntl64(
        fd: kernel::ffi::c_uint,
        cmd: kernel::ffi::c_uint,
        arg: kernel::ffi::c_ulong,
    ) -> kernel::ffi::c_long;
    pub fn sys_oabi_epoll_ctl(
        epfd: kernel::ffi::c_int,
        op: kernel::ffi::c_int,
        fd: kernel::ffi::c_int,
        event: *mut oabi_epoll_event,
    ) -> kernel::ffi::c_long;
    pub fn sys_oabi_semtimedop(
        semid: kernel::ffi::c_int,
        tsops: *mut oabi_sembuf,
        nsops: kernel::ffi::c_uint,
        timeout: *const old_timespec32,
    ) -> kernel::ffi::c_long;
    pub fn sys_oabi_semop(
        semid: kernel::ffi::c_int,
        tsops: *mut oabi_sembuf,
        nsops: kernel::ffi::c_uint,
    ) -> kernel::ffi::c_long;
    pub fn sys_oabi_ipc(
        call: kernel::ffi::c_uint,
        first: kernel::ffi::c_int,
        second: kernel::ffi::c_int,
        third: kernel::ffi::c_int,
        ptr: *mut c_void,
        fifth: kernel::ffi::c_long,
    ) -> kernel::ffi::c_int;
    pub fn sys_oabi_bind(
        fd: kernel::ffi::c_int,
        addr: *mut sockaddr,
        addrlen: kernel::ffi::c_int,
    ) -> kernel::ffi::c_long;
    pub fn sys_oabi_connect(
        fd: kernel::ffi::c_int,
        addr: *mut sockaddr,
        addrlen: kernel::ffi::c_int,
    ) -> kernel::ffi::c_long;
    pub fn sys_oabi_sendto(
        fd: kernel::ffi::c_int,
        buff: *mut c_void,
        len: usize,
        flags: kernel::ffi::c_uint,
        addr: *mut sockaddr,
        addrlen: kernel::ffi::c_int,
    ) -> kernel::ffi::c_long;
    pub fn sys_oabi_sendmsg(
        fd: kernel::ffi::c_int,
        msg: *mut user_msghdr,
        flags: kernel::ffi::c_uint,
    ) -> kernel::ffi::c_long;
    pub fn sys_oabi_socketcall(
        call: kernel::ffi::c_int,
        args: *mut kernel::ffi::c_ulong,
    ) -> kernel::ffi::c_long;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
