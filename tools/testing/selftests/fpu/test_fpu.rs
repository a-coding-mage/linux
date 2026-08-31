// SPDX-License-Identifier: GPL-2.0+
/* This testcase operates with the test_fpu kernel driver.
 * It modifies the FPU control register in user mode and calls the kernel
 * module to perform floating point operations in the kernel. The control
 * register value should be independent between kernel and user mode.
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_void};

const O_RDONLY: c_int = 0;
const FE_ALL_EXCEPT: c_int = 0x3d;
const FE_DOWNWARD: c_int = 0x400;

static test_fpu_path: &[u8] = b"/sys/kernel/debug/selftest_helpers/test_fpu\0";

unsafe extern "C" {
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn printf(format: *const c_char, ...) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn __errno_location() -> *mut c_int;
    fn fesetround(rounding_mode: c_int) -> c_int;
    fn fegetround() -> c_int;
    fn feclearexcept(excepts: c_int) -> c_int;
    fn feenableexcept(excepts: c_int) -> c_int;
    fn fegetexcept() -> c_int;
}

fn main() {
    let mut dummy: [c_char; 1] = [0; 1];
    let fd: c_int = unsafe { open(test_fpu_path.as_ptr() as *const c_char, O_RDONLY) };

    if fd < 0 {
        unsafe {
            printf(
                b"[SKIP]\tcan't access %s: %s\n\0".as_ptr() as *const c_char,
                test_fpu_path.as_ptr() as *const c_char,
                strerror(*__errno_location()),
            );
        }
        std::process::exit(0);
    }

    if unsafe { read(fd, dummy.as_mut_ptr() as *mut c_void, 1) } < 0 {
        unsafe {
            printf(b"[FAIL]\taccess with default rounding mode failed\n\0".as_ptr() as *const c_char);
        }
        std::process::exit(1);
    }

    unsafe {
        fesetround(FE_DOWNWARD);
    }
    if unsafe { read(fd, dummy.as_mut_ptr() as *mut c_void, 1) } < 0 {
        unsafe {
            printf(b"[FAIL]\taccess with downward rounding mode failed\n\0".as_ptr() as *const c_char);
        }
        std::process::exit(2);
    }
    if unsafe { fegetround() } != FE_DOWNWARD {
        unsafe {
            printf(b"[FAIL]\tusermode rounding mode clobbered\n\0".as_ptr() as *const c_char);
        }
        std::process::exit(3);
    }

    /* Note: the tests up to this point are quite safe and will only return
     * an error. But the exception mask setting can cause misbehaving kernel
     * to crash.
     */
    unsafe {
        feclearexcept(FE_ALL_EXCEPT);
        feenableexcept(FE_ALL_EXCEPT);
    }
    if unsafe { read(fd, dummy.as_mut_ptr() as *mut c_void, 1) } < 0 {
        unsafe {
            printf(b"[FAIL]\taccess with fpu exceptions unmasked failed\n\0".as_ptr() as *const c_char);
        }
        std::process::exit(4);
    }
    if unsafe { fegetexcept() } != FE_ALL_EXCEPT {
        unsafe {
            printf(b"[FAIL]\tusermode fpu exception mask clobbered\n\0".as_ptr() as *const c_char);
        }
        std::process::exit(5);
    }

    unsafe {
        printf(b"[OK]\ttest_fpu\n\0".as_ptr() as *const c_char);
    }
    std::process::exit(0);
}
