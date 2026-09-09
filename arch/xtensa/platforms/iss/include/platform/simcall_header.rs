/*
 * include/asm-xtensa/platform-iss/simcall.h
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 Tensilica Inc.
 * Copyright (C) 2017 - 2021 Cadence Design Systems Inc.
 */

// The C header conditionally includes platform-specific simcall definitions.
// Those symbols are supplied by the surrounding translation unit.

use core::ffi::c_void;

extern "C" {
    fn __simc(call: i32, arg1: i32, arg2: i32, arg3: i32) -> i32;
}

#[inline]
pub unsafe fn simc_exit(exit_code: i32) -> i32 {
    // C condition: defined(SYS_exit)
    #[cfg(feature = "SYS_exit")]
    {
        __simc(SYS_exit, exit_code, 0, 0)
    }
    #[cfg(not(feature = "SYS_exit"))]
    {
        // WARN_ONCE(1, "%s: not implemented\n", __func__)
        -1
    }
}

#[inline]
pub unsafe fn simc_open(file: *const i8, flags: i32, mode: i32) -> i32 {
    __simc(SYS_open, file as i32, flags, mode)
}

#[inline]
pub unsafe fn simc_close(fd: i32) -> i32 {
    __simc(SYS_close, fd, 0, 0)
}

#[inline]
pub unsafe fn simc_ioctl(fd: i32, request: i32, arg: *mut c_void) -> i32 {
    // C condition: defined(SYS_ioctl)
    #[cfg(feature = "SYS_ioctl")]
    {
        __simc(SYS_ioctl, fd, request, arg as i32)
    }
    #[cfg(not(feature = "SYS_ioctl"))]
    {
        // WARN_ONCE(1, "%s: not implemented\n", __func__)
        -1
    }
}

#[inline]
pub unsafe fn simc_read(fd: i32, buf: *mut c_void, count: usize) -> i32 {
    __simc(SYS_read, fd, buf as i32, count as i32)
}

#[inline]
pub unsafe fn simc_write(fd: i32, buf: *const c_void, count: usize) -> i32 {
    __simc(SYS_write, fd, buf as i32, count as i32)
}

#[inline]
pub unsafe fn simc_poll(fd: i32) -> i32 {
    // C condition: defined(SYS_select_one)
    #[cfg(feature = "SYS_select_one")]
    {
        let timeval: [i64; 2] = [0, 0];
        __simc(SYS_select_one, fd, XTISS_SELECT_ONE_READ, (&timeval as *const [i64; 2]) as i32)
    }
    #[cfg(not(feature = "SYS_select_one"))]
    {
        // WARN_ONCE(1, "%s: not implemented\n", __func__)
        -1
    }
}

#[inline]
pub unsafe fn simc_lseek(fd: i32, off: u32, whence: i32) -> i32 {
    __simc(SYS_lseek, fd, off as i32, whence)
}

#[inline]
pub unsafe fn simc_argc() -> i32 {
    // C condition: defined(SYS_iss_argc)
    #[cfg(feature = "SYS_iss_argc")]
    {
        __simc(SYS_iss_argc, 0, 0, 0)
    }
    #[cfg(not(feature = "SYS_iss_argc"))]
    {
        // WARN_ONCE(1, "%s: not implemented\n", __func__)
        0
    }
}

#[inline]
pub unsafe fn simc_argv_size() -> i32 {
    // C condition: defined(SYS_iss_argv_size)
    #[cfg(feature = "SYS_iss_argv_size")]
    {
        __simc(SYS_iss_argv_size, 0, 0, 0)
    }
    #[cfg(not(feature = "SYS_iss_argv_size"))]
    {
        // WARN_ONCE(1, "%s: not implemented\n", __func__)
        0
    }
}

#[inline]
pub unsafe fn simc_argv(buf: *mut c_void) {
    // C condition: defined(SYS_iss_set_argv)
    #[cfg(feature = "SYS_iss_set_argv")]
    {
        __simc(SYS_iss_set_argv, buf as i32, 0, 0);
    }
    #[cfg(not(feature = "SYS_iss_set_argv"))]
    {
        // WARN_ONCE(1, "%s: not implemented\n", __func__)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
