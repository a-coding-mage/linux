// SPDX-License-Identifier: GPL-2.0
//
// Linux kernel headers and architecture headers included by the C source are
// supplied by the surrounding translation unit.

use core::ffi::c_void;

pub type ssize_t = isize;
pub type size_t = usize;
pub type loff_t = i64;
pub type u32 = core::ffi::c_uint;
pub type u64 = core::ffi::c_ulonglong;

#[repr(C)]
pub struct pt_regs {
    pub regs: [u32; 16],
}

extern "C" {
    fn do_pipe_flags(fd: *mut i32, flags: i32) -> i32;
    fn current_pt_regs() -> *mut pt_regs;
    fn ksys_pread64(fd: u32, buf: *mut c_void, count: size_t, pos: loff_t) -> ssize_t;
    fn ksys_pwrite64(fd: u32, buf: *const c_void, count: size_t, pos: loff_t) -> ssize_t;
    fn ksys_fadvise64_64(fd: i32, offset: u64, len: u64, advice: i32) -> i32;
    fn ksys_sync_file_range(fd: i32, offset: loff_t, nbytes: loff_t, flags: u32) -> i32;
}

/*
 * sys_pipe() is the normal C calling standard for creating
 * a pipe. It's not the way Unix traditionally does this, though.
 */
#[no_mangle]
pub unsafe extern "C" fn sys_sh_pipe() -> i32 {
    let mut fd = [0i32; 2];
    let error: i32;

    error = do_pipe_flags(fd.as_mut_ptr(), 0);
    if error == 0 {
        (*current_pt_regs()).regs[1] = fd[1] as u32;
        return fd[0];
    }
    error
}

#[no_mangle]
pub unsafe extern "C" fn sys_pread_wrapper(
    fd: u32,
    buf: *mut c_void,
    count: size_t,
    _dummy: i64,
    pos: loff_t,
) -> ssize_t {
    ksys_pread64(fd, buf, count, pos)
}

#[no_mangle]
pub unsafe extern "C" fn sys_pwrite_wrapper(
    fd: u32,
    buf: *const c_void,
    count: size_t,
    _dummy: i64,
    pos: loff_t,
) -> ssize_t {
    ksys_pwrite64(fd, buf, count, pos)
}

#[no_mangle]
pub unsafe extern "C" fn sys_fadvise64_64_wrapper(
    fd: i32,
    offset0: u32,
    offset1: u32,
    len0: u32,
    len1: u32,
    advice: i32,
) -> i32 {
    // __LITTLE_ENDIAN__ in the original build selects this ordering.
    ksys_fadvise64_64(
        fd,
        ((offset1 as u64) << 32) | offset0 as u64,
        ((len1 as u64) << 32) | len0 as u64,
        advice,
    )
}

/*
 * Swap the arguments the way that libc wants them instead of
 * moving flags ahead of the 64-bit nbytes argument.
 *
 * SC_ARG64 expands to the architecture-specific pair of 32-bit arguments;
 * this is the corresponding little-endian SH32 representation.
 */
#[no_mangle]
pub unsafe extern "C" fn sh_sync_file_range6(
    fd: i32,
    offset0: u32,
    offset1: u32,
    nbytes0: u32,
    nbytes1: u32,
    flags: u32,
) -> i32 {
    ksys_sync_file_range(
        fd,
        (((offset1 as u64) << 32) | offset0 as u64) as loff_t,
        (((nbytes1 as u64) << 32) | nbytes0 as u64) as loff_t,
        flags,
    )
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
