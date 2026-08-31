// SPDX-License-Identifier: GPL-2.0-only
/*
 * vdso_test_gettimeofday.c: Sample code to test parse_vdso.c and
 *                           vDSO gettimeofday()
 * Copyright (c) 2014 Andy Lutomirski
 *
 * Compile with:
 * gcc -std=gnu99 vdso_test_gettimeofday.c parse_vdso_gettimeofday.c
 *
 * Tested on x86, 32-bit and 64-bit.  It may work on other architectures, too.
 */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

// Constants and declarations supplied by kselftest.h, parse_vdso.h,
// vdso_config.h, vdso_call.h, and system headers in the original C source.
const AT_SYSINFO_EHDR: c_ulong = 33;

unsafe extern "C" {
    static versions: [*const c_char; 0];
    static names: [*const c_char; 0];
    static VDSO_VERSION: usize;
    static VDSO_NAMES: usize;

    fn getauxval(type_: c_ulong) -> c_ulong;
    fn printf(format: *const c_char, ...) -> c_int;
    fn vdso_init_from_sysinfo_ehdr(base: c_ulong);
    fn vdso_sym(version: *const c_char, name: *const c_char) -> *mut c_void;
}

const KSFT_SKIP: c_int = 4;
const KSFT_FAIL: c_int = 1;

#[repr(C)]
struct timeval {
    tv_sec: c_long,
    tv_usec: c_long,
}

#[repr(C)]
struct timezone {
    tz_minuteswest: c_int,
    tz_dsttime: c_int,
}

type gtod_t = unsafe extern "C" fn(tv: *mut timeval, tz: *mut timezone) -> c_long;

unsafe fn VDSO_CALL(
    gtod: gtod_t,
    _nr: c_int,
    tv: *mut timeval,
    tz: *mut timezone,
) -> c_long {
    unsafe { gtod(tv, tz) }
}

fn main() {
    unsafe {
        let version = versions[VDSO_VERSION];
        let name = (&raw const names[VDSO_NAMES]) as *const *const c_char;

        let sysinfo_ehdr = getauxval(AT_SYSINFO_EHDR);
        if sysinfo_ehdr == 0 {
            printf(c"AT_SYSINFO_EHDR is not present!\n".as_ptr());
            std::process::exit(KSFT_SKIP);
        }

        vdso_init_from_sysinfo_ehdr(getauxval(AT_SYSINFO_EHDR));

        /* Find gettimeofday. */
        let gtod_sym = vdso_sym(version, *name.add(0));
        let gtod: Option<gtod_t> = core::mem::transmute(gtod_sym);

        if gtod.is_none() {
            printf(c"Could not find %s\n".as_ptr(), *name.add(0));
            std::process::exit(KSFT_SKIP);
        }

        let gtod = gtod.unwrap_unchecked();
        let mut tv = core::mem::MaybeUninit::<timeval>::uninit();
        let ret = VDSO_CALL(gtod, 2, tv.as_mut_ptr(), core::ptr::null_mut());

        if ret == 0 {
            let tv = tv.assume_init();
            printf(
                c"The time is %lld.%06lld\n".as_ptr(),
                tv.tv_sec as i64,
                tv.tv_usec as i64,
            );
        } else {
            printf(c"%s failed\n".as_ptr(), *name.add(0));
            std::process::exit(KSFT_FAIL);
        }

        std::process::exit(0);
    }
}
