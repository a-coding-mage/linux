// SPDX-License-Identifier: GPL-2.0
// C preprocessor feature-test macros from the source:
// _GNU_SOURCE=1 and __USE_GNU=1.
// C includes translated as external C declarations below.

use core::arch::asm;
use core::ffi::{c_char, c_int};

const FE_INVALID: c_int = 0x01;
const FE_DIVBYZERO: c_int = 0x04;
const FE_OVERFLOW: c_int = 0x08;
const FE_UNDERFLOW: c_int = 0x10;
const FE_INEXACT: c_int = 0x20;

const SIGILL: c_int = 4;
const SIGFPE: c_int = 8;
const SIGSEGV: c_int = 11;

const FE_TEST_EXCEPTS: c_int =
    FE_DIVBYZERO | FE_INEXACT | FE_INVALID | FE_OVERFLOW | FE_UNDERFLOW;

unsafe extern "C" {
    fn feclearexcept(excepts: c_int) -> c_int;
    fn fetestexcept(excepts: c_int) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn signal(signum: c_int, handler: extern "C" fn(c_int)) -> usize;
    fn exit(status: c_int) -> !;
}

#[no_mangle]
pub static mut res64: u64 = -1i32 as u64;
#[no_mangle]
pub static mut res32: u32 = -1i32 as u32;
#[no_mangle]
pub static mut res16: u16 = -1i32 as u16;

pub unsafe fn test() -> c_int {
    let mut ex: c_int;

    unsafe {
        feclearexcept(FE_TEST_EXCEPTS);
        asm!(
            "\n",
            "	fld1",
            "	fisttps res16",
            "	fld1",
            "	fisttpl res32",
            "	fld1",
            "	fisttpll res64",
            options(att_syntax),
        );
    }
    if unsafe { res16 != 1 || res32 != 1 || res64 != 1 } {
        unsafe {
            printf(c"[BAD]\tfisttp 1\n".as_ptr());
        }
        return 1;
    }
    ex = unsafe { fetestexcept(FE_TEST_EXCEPTS) };
    if ex != 0 {
        unsafe {
            printf(c"[BAD]\tfisttp 1: wrong exception state\n".as_ptr());
        }
        return 1;
    }

    unsafe {
        feclearexcept(FE_TEST_EXCEPTS);
        asm!(
            "\n",
            "	fldpi",
            "	fisttps res16",
            "	fldpi",
            "	fisttpl res32",
            "	fldpi",
            "	fisttpll res64",
            options(att_syntax),
        );
    }
    if unsafe { res16 != 3 || res32 != 3 || res64 != 3 } {
        unsafe {
            printf(c"[BAD]\tfisttp pi\n".as_ptr());
        }
        return 1;
    }
    ex = unsafe { fetestexcept(FE_TEST_EXCEPTS) };
    if ex != FE_INEXACT {
        unsafe {
            printf(c"[BAD]\tfisttp pi: wrong exception state\n".as_ptr());
        }
        return 1;
    }

    unsafe {
        feclearexcept(FE_TEST_EXCEPTS);
        asm!(
            "\n",
            "	fldpi",
            "	fchs",
            "	fisttps res16",
            "	fldpi",
            "	fchs",
            "	fisttpl res32",
            "	fldpi",
            "	fchs",
            "	fisttpll res64",
            options(att_syntax),
        );
    }
    if unsafe { res16 != 0xfffd || res32 != 0xfffffffd || res64 != 0xfffffffffffffffd_u64 } {
        unsafe {
            printf(c"[BAD]\tfisttp -pi\n".as_ptr());
        }
        return 1;
    }
    ex = unsafe { fetestexcept(FE_TEST_EXCEPTS) };
    if ex != FE_INEXACT {
        unsafe {
            printf(c"[BAD]\tfisttp -pi: wrong exception state\n".as_ptr());
        }
        return 1;
    }

    unsafe {
        feclearexcept(FE_TEST_EXCEPTS);
        asm!(
            "\n",
            "	fldln2",
            "	fisttps res16",
            "	fldln2",
            "	fisttpl res32",
            "	fldln2",
            "	fisttpll res64",
            options(att_syntax),
        );
    }
    /* Test truncation to zero (round-to-nearest would give 1 here) */
    if unsafe { res16 != 0 || res32 != 0 || res64 != 0 } {
        unsafe {
            printf(c"[BAD]\tfisttp ln2\n".as_ptr());
        }
        return 1;
    }
    ex = unsafe { fetestexcept(FE_TEST_EXCEPTS) };
    if ex != FE_INEXACT {
        unsafe {
            printf(c"[BAD]\tfisttp ln2: wrong exception state\n".as_ptr());
        }
        return 1;
    }

    0
}

extern "C" fn sighandler(sig: c_int) {
    unsafe {
        printf(c"[FAIL]\tGot signal %d, exiting\n".as_ptr(), sig);
        exit(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn main(
    _argc: c_int,
    _argv: *mut *mut c_char,
    _envp: *mut *mut c_char,
) -> c_int {
    let mut err: c_int = 0;

    /* SIGILL triggers on 32-bit kernels w/o fisttp emulation
     * when run with "no387 nofxsr". Other signals are caught
     * just in case.
     */
    unsafe {
        signal(SIGILL, sighandler);
        signal(SIGFPE, sighandler);
        signal(SIGSEGV, sighandler);

        printf(c"[RUN]\tTesting fisttp instructions\n".as_ptr());
        err |= test();
        if err == 0 {
            printf(c"[OK]\tfisttp\n".as_ptr());
        } else {
            printf(c"[FAIL]\tfisttp errors: %d\n".as_ptr(), err);
        }
    }

    err
}
