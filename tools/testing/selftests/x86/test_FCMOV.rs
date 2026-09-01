// SPDX-License-Identifier: GPL-2.0
// C source included GNU/libc and signal headers.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::arch::asm;
use core::ffi::{c_char, c_int, c_long};

type long_double = f64;

const SIGILL: c_int = 4;
const SIGFPE: c_int = 8;
const SIGSEGV: c_int = 11;

const CF: c_long = 1 << 0;
const PF: c_long = 1 << 2;
const ZF: c_long = 1 << 6;

unsafe extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn signal(signum: c_int, handler: extern "C" fn(c_int)) -> usize;
}

macro_rules! TEST {
    ($insn:ident) => {
        #[inline(never)]
        unsafe fn $insn(flags: c_long) -> long_double {
            let mut out: long_double;

            unsafe {
                asm!(
                    "push {flags}",
                    "popf",
                    "fldpi",
                    "fld1",
                    concat!(stringify!($insn), " %st(1), %st"),
                    "ffree %st(1)",
                    "fstp qword ptr [{out_ptr}]",
                    flags = in(reg) flags,
                    out_ptr = in(reg) &mut out,
                    options(att_syntax)
                );
            }

            out
        }
    };
}

TEST!(fcmovb);
TEST!(fcmove);
TEST!(fcmovbe);
TEST!(fcmovu);
TEST!(fcmovnb);
TEST!(fcmovne);
TEST!(fcmovnbe);
TEST!(fcmovnu);

extern "C" fn sighandler(sig: c_int) {
    unsafe {
        printf(
            b"[FAIL]\tGot signal %d, exiting\n\0".as_ptr() as *const c_char,
            sig,
        );
        exit(1);
    }
}

fn main() {
    let mut err: c_int = 0;

    /*
     * SIGILL triggers on 32-bit kernels w/o fcomi emulation
     * when run with "no387 nofxsr". Other signals are caught
     * just in case.
     */
    unsafe {
        signal(SIGILL, sighandler);
        signal(SIGFPE, sighandler);
        signal(SIGSEGV, sighandler);

        printf(b"[RUN]\tTesting fcmovCC instructions\n\0".as_ptr() as *const c_char);
    }

    /*
     * If fcmovCC() returns 1.0, the move wasn't done
     */
    unsafe {
        err |= !(fcmovb(0) == 1.0) as c_int;
        err |= !(fcmovnb(0) != 1.0) as c_int;
        err |= !(fcmove(0) == 1.0) as c_int;
        err |= !(fcmovne(0) != 1.0) as c_int;
        err |= !(fcmovbe(0) == 1.0) as c_int;
        err |= !(fcmovnbe(0) != 1.0) as c_int;
        err |= !(fcmovu(0) == 1.0) as c_int;
        err |= !(fcmovnu(0) != 1.0) as c_int;

        err |= !(fcmovb(CF) != 1.0) as c_int;
        err |= !(fcmovnb(CF) == 1.0) as c_int;
        err |= !(fcmove(CF) == 1.0) as c_int;
        err |= !(fcmovne(CF) != 1.0) as c_int;
        err |= !(fcmovbe(CF) != 1.0) as c_int;
        err |= !(fcmovnbe(CF) == 1.0) as c_int;
        err |= !(fcmovu(CF) == 1.0) as c_int;
        err |= !(fcmovnu(CF) != 1.0) as c_int;

        err |= !(fcmovb(ZF) == 1.0) as c_int;
        err |= !(fcmovnb(ZF) != 1.0) as c_int;
        err |= !(fcmove(ZF) != 1.0) as c_int;
        err |= !(fcmovne(ZF) == 1.0) as c_int;
        err |= !(fcmovbe(ZF) != 1.0) as c_int;
        err |= !(fcmovnbe(ZF) == 1.0) as c_int;
        err |= !(fcmovu(ZF) == 1.0) as c_int;
        err |= !(fcmovnu(ZF) != 1.0) as c_int;

        err |= !(fcmovb(PF) == 1.0) as c_int;
        err |= !(fcmovnb(PF) != 1.0) as c_int;
        err |= !(fcmove(PF) == 1.0) as c_int;
        err |= !(fcmovne(PF) != 1.0) as c_int;
        err |= !(fcmovbe(PF) == 1.0) as c_int;
        err |= !(fcmovnbe(PF) != 1.0) as c_int;
        err |= !(fcmovu(PF) != 1.0) as c_int;
        err |= !(fcmovnu(PF) == 1.0) as c_int;

        if err == 0 {
            printf(b"[OK]\tfcmovCC\n\0".as_ptr() as *const c_char);
        } else {
            printf(
                b"[FAIL]\tfcmovCC errors: %d\n\0".as_ptr() as *const c_char,
                err,
            );
        }
    }

    std::process::exit(err);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
