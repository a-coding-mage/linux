/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Common syscall restarting data
 */

// C dependencies:
// #include <linux/compiler.h>
// #include <linux/time64.h>
// #include <linux/types.h>

// Forward declarations from the C header/dependent headers.
#[repr(C)]
pub struct __kernel_timespec {
    _private: [u8; 0],
}

#[repr(C)]
pub struct timespec {
    _private: [u8; 0],
}

#[repr(C)]
pub struct old_timespec32 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pollfd {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum timespec_type {
    TT_NONE = 0,
    TT_NATIVE = 1,
    TT_COMPAT = 2,
}

/*
 * System call restart block.
 */
#[repr(C)]
pub struct restart_block {
    pub arch_data: ::core::ffi::c_ulong,
    pub fn_: Option<unsafe extern "C" fn(*mut restart_block) -> ::core::ffi::c_long>,
    pub data: restart_block_data,
}

#[repr(C)]
pub union restart_block_data {
    /* For futex_wait() */
    pub futex: restart_block_futex,
    /* For nanosleep */
    pub nanosleep: restart_block_nanosleep,
    /* For poll */
    pub poll: restart_block_poll,
}

#[repr(C)]
pub struct restart_block_futex {
    pub uaddr: *mut u32,
    pub val: u32,
    pub flags: u32,
    pub bitset: u32,
    pub time: ktime_t,
    pub uaddr2: *mut u32,
}

#[repr(C)]
pub union restart_block_nanosleep_rmtp {
    pub rmtp: *mut __kernel_timespec,
    pub compat_rmtp: *mut old_timespec32,
}

#[repr(C)]
pub struct restart_block_nanosleep {
    pub clockid: clockid_t,
    pub type_: timespec_type,
    pub rmtp: restart_block_nanosleep_rmtp,
    pub expires: ktime_t,
}

#[repr(C)]
pub struct restart_block_poll {
    pub ufds: *mut pollfd,
    pub nfds: ::core::ffi::c_int,
    pub has_timeout: ::core::ffi::c_int,
    pub end_time: timespec64,
}

extern "C" {
    pub fn do_no_restart_syscall(parm: *mut restart_block) -> ::core::ffi::c_long;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
