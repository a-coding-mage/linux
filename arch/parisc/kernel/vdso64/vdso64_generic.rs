// SPDX-License-Identifier: GPL-2.0

// Dependency intent from asm/unistd.h and linux/types.h is preserved here.

#[repr(C)]
pub struct timezone {
    _private: [u8; 0],
}

#[repr(C)]
pub struct __kernel_timespec {
    _private: [u8; 0],
}

#[repr(C)]
pub struct __kernel_old_timeval {
    _private: [u8; 0],
}

pub type clockid_t = i32;

extern "C" {
    pub fn syscall2(nr: i64, arg1: i64, arg2: i64) -> i64;
}

// __NR_gettimeofday and __NR_clock_gettime are supplied by asm/unistd.h.
// Forward declarations are represented by the definitions below.

#[no_mangle]
pub unsafe extern "C" fn __vdso_gettimeofday(
    tv: *mut __kernel_old_timeval,
    tz: *mut timezone,
) -> i32 {
    syscall2(
        __NR_gettimeofday as i64,
        tv as isize as i64,
        tz as isize as i64,
    ) as i32
}

#[no_mangle]
pub unsafe extern "C" fn __vdso_clock_gettime(
    clock: clockid_t,
    ts: *mut __kernel_timespec,
) -> i32 {
    syscall2(
        __NR_clock_gettime as i64,
        clock as i64,
        ts as isize as i64,
    ) as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
