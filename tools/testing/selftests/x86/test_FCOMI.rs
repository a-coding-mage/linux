// SPDX-License-Identifier: GPL-2.0
// C source used _GNU_SOURCE/__USE_GNU and included libc/fenv/signal headers.

use core::arch::asm;
use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

const CF: c_long = 1 << 0;
const PF: c_long = 1 << 2;
const ZF: c_long = 1 << 6;
const ARITH: c_long = CF | PF | ZF;

const FE_INVALID: c_int = 0x01;
const FE_DIVBYZERO: c_int = 0x04;
const FE_OVERFLOW: c_int = 0x08;
const FE_UNDERFLOW: c_int = 0x10;
const FE_INEXACT: c_int = 0x20;

const SIGILL: c_int = 4;
const SIGFPE: c_int = 8;
const SIGSEGV: c_int = 11;

static mut res_fcomi_pi_1: c_long = 0;
static mut res_fcomi_1_pi: c_long = 0;
static mut res_fcomi_1_1: c_long = 0;
static mut res_fcomi_nan_1: c_long = 0;
/* sNaN is s|111 1111 1|1xx xxxx xxxx xxxx xxxx xxxx */
/* qNaN is s|111 1111 1|0xx xxxx xxxx xxxx xxxx xxxx (some x must be nonzero) */
static mut snan: c_int = 0x7fc11111;
static mut qnan: c_int = 0x7f811111;
static mut snan1: [u16; 5] = [0; 5];
/* sNaN80 is s|111 1111 1111 1111 |10xx xx...xx (some x must be nonzero) */
static mut snan80: [u16; 5] = [0x1111, 0x1111, 0x1111, 0x8111, 0x7fff];

unsafe extern "C" {
    fn feclearexcept(excepts: c_int) -> c_int;
    fn fetestexcept(excepts: c_int) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn signal(signum: c_int, handler: extern "C" fn(c_int)) -> *mut c_void;
    fn exit(status: c_int) -> !;
}

unsafe fn test(flags: c_long) -> c_int {
    unsafe {
        feclearexcept(FE_DIVBYZERO | FE_INEXACT | FE_INVALID | FE_OVERFLOW | FE_UNDERFLOW);

        asm!(
            "push {flags}",
            "popf",
            "fld1",
            "fldpi",
            "fcomi %st(1), %st",
            "ffree %st(0)",
            "ffree %st(1)",
            "pushf",
            "pop qword ptr [{res_fcomi_1_pi}]",
            "push {flags}",
            "popf",
            "fldpi",
            "fld1",
            "fcomi %st(1), %st",
            "ffree %st(0)",
            "ffree %st(1)",
            "pushf",
            "pop qword ptr [{res_fcomi_pi_1}]",
            "push {flags}",
            "popf",
            "fld1",
            "fld1",
            "fcomi %st(1), %st",
            "ffree %st(0)",
            "ffree %st(1)",
            "pushf",
            "pop qword ptr [{res_fcomi_1_1}]",
            flags = in(reg) flags,
            res_fcomi_1_pi = sym res_fcomi_1_pi,
            res_fcomi_pi_1 = sym res_fcomi_pi_1,
            res_fcomi_1_1 = sym res_fcomi_1_1,
            options(att_syntax)
        );
        if (res_fcomi_1_pi & ARITH) != 0 {
            printf(c"[BAD]\tfcomi_1_pi with flags:%lx\n".as_ptr(), flags as c_ulong);
            return 1;
        }
        if (res_fcomi_pi_1 & ARITH) != CF {
            printf(c"[BAD]\tfcomi_pi_1 with flags:%lx->%lx\n".as_ptr(), flags as c_ulong, (res_fcomi_pi_1 & ARITH) as c_ulong);
            return 1;
        }
        if (res_fcomi_1_1 & ARITH) != ZF {
            printf(c"[BAD]\tfcomi_1_1 with flags:%lx\n".as_ptr(), flags as c_ulong);
            return 1;
        }
        if fetestexcept(FE_INVALID) != 0 {
            printf(c"[BAD]\tFE_INVALID is set in %s\n".as_ptr(), c"test".as_ptr());
            return 1;
        }
        0
    }
}

unsafe fn test_qnan(flags: c_long) -> c_int {
    unsafe {
        feclearexcept(FE_DIVBYZERO | FE_INEXACT | FE_INVALID | FE_OVERFLOW | FE_UNDERFLOW);

        asm!(
            "push {flags}",
            "popf",
            "flds {qnan}",
            "fld1",
            "fnclex", // fld of a qnan raised FE_INVALID, clear it
            "fcomi %st(1), %st",
            "ffree %st(0)",
            "ffree %st(1)",
            "pushf",
            "pop qword ptr [{res_fcomi_nan_1}]",
            flags = in(reg) flags,
            qnan = sym qnan,
            res_fcomi_nan_1 = sym res_fcomi_nan_1,
            options(att_syntax)
        );
        if (res_fcomi_nan_1 & ARITH) != (ZF | CF | PF) {
            printf(c"[BAD]\tfcomi_qnan_1 with flags:%lx\n".as_ptr(), flags as c_ulong);
            return 1;
        }
        if fetestexcept(FE_INVALID) != FE_INVALID {
            printf(c"[BAD]\tFE_INVALID is not set in %s\n".as_ptr(), c"test_qnan".as_ptr());
            return 1;
        }
        0
    }
}

unsafe fn testu_qnan(flags: c_long) -> c_int {
    unsafe {
        feclearexcept(FE_DIVBYZERO | FE_INEXACT | FE_INVALID | FE_OVERFLOW | FE_UNDERFLOW);

        asm!(
            "push {flags}",
            "popf",
            "flds {qnan}",
            "fld1",
            "fnclex", // fld of a qnan raised FE_INVALID, clear it
            "fucomi %st(1), %st",
            "ffree %st(0)",
            "ffree %st(1)",
            "pushf",
            "pop qword ptr [{res_fcomi_nan_1}]",
            flags = in(reg) flags,
            qnan = sym qnan,
            res_fcomi_nan_1 = sym res_fcomi_nan_1,
            options(att_syntax)
        );
        if (res_fcomi_nan_1 & ARITH) != (ZF | CF | PF) {
            printf(c"[BAD]\tfcomi_qnan_1 with flags:%lx\n".as_ptr(), flags as c_ulong);
            return 1;
        }
        if fetestexcept(FE_INVALID) != 0 {
            printf(c"[BAD]\tFE_INVALID is set in %s\n".as_ptr(), c"testu_qnan".as_ptr());
            return 1;
        }
        0
    }
}

unsafe fn testu_snan(flags: c_long) -> c_int {
    unsafe {
        feclearexcept(FE_DIVBYZERO | FE_INEXACT | FE_INVALID | FE_OVERFLOW | FE_UNDERFLOW);

        asm!(
            "push {flags}",
            "popf",
            // "flds snan"     // WRONG, this will convert 32-bit fp snan to a *qnan* in 80-bit fp register!
            // "fstpt snan1"   // if uncommented, it prints "snan1:7fff c111 1100 0000 0000" - c111, not 8111!
            // "fnclex"        // flds of a snan raised FE_INVALID, clear it
            "fldt {snan80}", // fldt never raise FE_INVALID
            "fld1",
            "fucomi %st(1), %st",
            "ffree %st(0)",
            "ffree %st(1)",
            "pushf",
            "pop qword ptr [{res_fcomi_nan_1}]",
            flags = in(reg) flags,
            snan80 = sym snan80,
            res_fcomi_nan_1 = sym res_fcomi_nan_1,
            options(att_syntax)
        );
        if (res_fcomi_nan_1 & ARITH) != (ZF | CF | PF) {
            printf(c"[BAD]\tfcomi_qnan_1 with flags:%lx\n".as_ptr(), flags as c_ulong);
            return 1;
        }
        // printf("snan:%x snan1:%04x %04x %04x %04x %04x\n", snan, snan1[4], snan1[3], snan1[2], snan1[1], snan1[0]);
        if fetestexcept(FE_INVALID) != FE_INVALID {
            printf(c"[BAD]\tFE_INVALID is not set in %s\n".as_ptr(), c"testu_snan".as_ptr());
            return 1;
        }
        0
    }
}

unsafe fn testp(flags: c_long) -> c_int {
    unsafe {
        feclearexcept(FE_DIVBYZERO | FE_INEXACT | FE_INVALID | FE_OVERFLOW | FE_UNDERFLOW);

        asm!(
            "push {flags}",
            "popf",
            "fld1",
            "fldpi",
            "fcomip %st(1), %st",
            "ffree %st(0)",
            "pushf",
            "pop qword ptr [{res_fcomi_1_pi}]",
            "push {flags}",
            "popf",
            "fldpi",
            "fld1",
            "fcomip %st(1), %st",
            "ffree %st(0)",
            "pushf",
            "pop qword ptr [{res_fcomi_pi_1}]",
            "push {flags}",
            "popf",
            "fld1",
            "fld1",
            "fcomip %st(1), %st",
            "ffree %st(0)",
            "pushf",
            "pop qword ptr [{res_fcomi_1_1}]",
            flags = in(reg) flags,
            res_fcomi_1_pi = sym res_fcomi_1_pi,
            res_fcomi_pi_1 = sym res_fcomi_pi_1,
            res_fcomi_1_1 = sym res_fcomi_1_1,
            options(att_syntax)
        );
        if (res_fcomi_1_pi & ARITH) != 0 {
            printf(c"[BAD]\tfcomi_1_pi with flags:%lx\n".as_ptr(), flags as c_ulong);
            return 1;
        }
        if (res_fcomi_pi_1 & ARITH) != CF {
            printf(c"[BAD]\tfcomi_pi_1 with flags:%lx->%lx\n".as_ptr(), flags as c_ulong, (res_fcomi_pi_1 & ARITH) as c_ulong);
            return 1;
        }
        if (res_fcomi_1_1 & ARITH) != ZF {
            printf(c"[BAD]\tfcomi_1_1 with flags:%lx\n".as_ptr(), flags as c_ulong);
            return 1;
        }
        if fetestexcept(FE_INVALID) != 0 {
            printf(c"[BAD]\tFE_INVALID is set in %s\n".as_ptr(), c"testp".as_ptr());
            return 1;
        }
        0
    }
}

unsafe fn testp_qnan(flags: c_long) -> c_int {
    unsafe {
        feclearexcept(FE_DIVBYZERO | FE_INEXACT | FE_INVALID | FE_OVERFLOW | FE_UNDERFLOW);

        asm!(
            "push {flags}",
            "popf",
            "flds {qnan}",
            "fld1",
            "fnclex", // fld of a qnan raised FE_INVALID, clear it
            "fcomip %st(1), %st",
            "ffree %st(0)",
            "pushf",
            "pop qword ptr [{res_fcomi_nan_1}]",
            flags = in(reg) flags,
            qnan = sym qnan,
            res_fcomi_nan_1 = sym res_fcomi_nan_1,
            options(att_syntax)
        );
        if (res_fcomi_nan_1 & ARITH) != (ZF | CF | PF) {
            printf(c"[BAD]\tfcomi_qnan_1 with flags:%lx\n".as_ptr(), flags as c_ulong);
            return 1;
        }
        if fetestexcept(FE_INVALID) != FE_INVALID {
            printf(c"[BAD]\tFE_INVALID is not set in %s\n".as_ptr(), c"testp_qnan".as_ptr());
            return 1;
        }
        0
    }
}

unsafe fn testup_qnan(flags: c_long) -> c_int {
    unsafe {
        feclearexcept(FE_DIVBYZERO | FE_INEXACT | FE_INVALID | FE_OVERFLOW | FE_UNDERFLOW);

        asm!(
            "push {flags}",
            "popf",
            "flds {qnan}",
            "fld1",
            "fnclex", // fld of a qnan raised FE_INVALID, clear it
            "fucomip %st(1), %st",
            "ffree %st(0)",
            "pushf",
            "pop qword ptr [{res_fcomi_nan_1}]",
            flags = in(reg) flags,
            qnan = sym qnan,
            res_fcomi_nan_1 = sym res_fcomi_nan_1,
            options(att_syntax)
        );
        if (res_fcomi_nan_1 & ARITH) != (ZF | CF | PF) {
            printf(c"[BAD]\tfcomi_qnan_1 with flags:%lx\n".as_ptr(), flags as c_ulong);
            return 1;
        }
        if fetestexcept(FE_INVALID) != 0 {
            printf(c"[BAD]\tFE_INVALID is set in %s\n".as_ptr(), c"testup_qnan".as_ptr());
            return 1;
        }
        0
    }
}

extern "C" fn sighandler(sig: c_int) {
    unsafe {
        printf(c"[FAIL]\tGot signal %d, exiting\n".as_ptr(), sig);
        exit(1);
    }
}

unsafe fn c_main(_argc: c_int, _argv: *mut *mut c_char, _envp: *mut *mut c_char) -> c_int {
    unsafe {
        let mut err: c_int = 0;

        /* SIGILL triggers on 32-bit kernels w/o fcomi emulation
         * when run with "no387 nofxsr". Other signals are caught
         * just in case.
         */
        signal(SIGILL, sighandler);
        signal(SIGFPE, sighandler);
        signal(SIGSEGV, sighandler);

        printf(c"[RUN]\tTesting f[u]comi[p] instructions\n".as_ptr());
        err |= test(0);
        err |= test_qnan(0);
        err |= testu_qnan(0);
        err |= testu_snan(0);
        err |= test(CF | ZF | PF);
        err |= test_qnan(CF | ZF | PF);
        err |= testu_qnan(CF | ZF | PF);
        err |= testu_snan(CF | ZF | PF);
        err |= testp(0);
        err |= testp_qnan(0);
        err |= testup_qnan(0);
        err |= testp(CF | ZF | PF);
        err |= testp_qnan(CF | ZF | PF);
        err |= testup_qnan(CF | ZF | PF);
        if err == 0 {
            printf(c"[OK]\tf[u]comi[p]\n".as_ptr());
        } else {
            printf(c"[FAIL]\tf[u]comi[p] errors: %d\n".as_ptr(), err);
        }

        err
    }
}

fn main() {
    unsafe {
        let status = c_main(0, core::ptr::null_mut(), core::ptr::null_mut());
        if status != 0 {
            std::process::exit(status);
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
