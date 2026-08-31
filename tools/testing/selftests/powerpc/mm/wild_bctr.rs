// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright 2018, Michael Ellerman, IBM Corp.
 *
 * Test that an out-of-bounds branch to counter behaves as expected.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;

const BAD_NIP: c_ulong = 0x788c545a18000000u64 as c_ulong;

const SIGSEGV: c_int = 11;
const SIGUSR2: c_int = 12;
const SA_SIGINFO: c_int = 4;

// Types and functions normally supplied by the C headers and "utils.h".
#[repr(C)]
pub struct pt_regs {
    pub gpr: [c_ulong; 32],
    pub nip: c_ulong,
}

#[repr(C)]
pub struct mcontext_t {
    pub regs: *mut pt_regs,
}

#[repr(C)]
pub struct ucontext_t {
    pub uc_mcontext: mcontext_t,
}

#[repr(C)]
pub struct siginfo_t {
    _private: [u8; 0],
}

type sighandler_t = unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void);

#[repr(C)]
pub struct sigaction {
    pub sa_sigaction: sighandler_t,
    pub sa_flags: c_int,
}

#[repr(C)]
pub struct jmp_buf {
    // TODO: supplied by <setjmp.h>; this preserves the dependency as an opaque
    // file-local declaration for this translation unit.
    _private: [u8; 0],
}

extern "C" {
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn printf(format: *const c_char, ...) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn setjmp(env: *mut jmp_buf) -> c_int;
    fn longjmp(env: *mut jmp_buf, val: c_int) -> !;
    fn kill(pid: c_int, sig: c_int) -> c_int;
    fn getpid() -> c_int;

    fn test_harness(test_function: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

static mut signal_regs: pt_regs = pt_regs {
    gpr: [0; 32],
    nip: 0,
};
static mut setjmp_env: jmp_buf = jmp_buf { _private: [] };

unsafe fn bzero(s: *mut c_void, n: usize) {
    memset(s, 0, n);
}

unsafe fn FAIL_IF(cond: bool) {
    if cond {
        // The original FAIL_IF macro is supplied by utils.h.
        core::arch::asm!("trap", options(noreturn));
    }
}

unsafe fn save_regs(ctxt: *mut ucontext_t) {
    let regs: *mut pt_regs = (*ctxt).uc_mcontext.regs;

    memcpy(
        &mut signal_regs as *mut pt_regs as *mut c_void,
        regs as *const c_void,
        size_of::<pt_regs>(),
    );
}

unsafe extern "C" fn segv_handler(
    _signum: c_int,
    _info: *mut siginfo_t,
    ctxt_v: *mut c_void,
) {
    save_regs(ctxt_v as *mut ucontext_t);
    longjmp(&mut setjmp_env as *mut jmp_buf, 1);
}

unsafe extern "C" fn usr2_handler(
    _signum: c_int,
    _info: *mut siginfo_t,
    ctxt_v: *mut c_void,
) {
    save_regs(ctxt_v as *mut ucontext_t);
}

unsafe extern "C" fn ok() -> c_int {
    printf(b"Everything is OK in here.\n\0".as_ptr() as *const c_char);
    0
}

const REG_POISON: c_ulong = 0x5a5a;

fn POISONED_REG(n: c_ulong) -> c_ulong {
    (REG_POISON << 48) | (n << 32) | (REG_POISON << 16) | n
}

#[inline]
unsafe fn poison_regs() {
    core::arch::asm!(
        "lis  15, {reg_poison}",
        "addi 15, 15, 15",
        "sldi 15, 15, 32",
        "oris 15, 15, {reg_poison}",
        "addi 15, 15, 15",
        "lis  16, {reg_poison}",
        "addi 16, 16, 16",
        "sldi 16, 16, 32",
        "oris 16, 16, {reg_poison}",
        "addi 16, 16, 16",
        "lis  17, {reg_poison}",
        "addi 17, 17, 17",
        "sldi 17, 17, 32",
        "oris 17, 17, {reg_poison}",
        "addi 17, 17, 17",
        "lis  18, {reg_poison}",
        "addi 18, 18, 18",
        "sldi 18, 18, 32",
        "oris 18, 18, {reg_poison}",
        "addi 18, 18, 18",
        "lis  19, {reg_poison}",
        "addi 19, 19, 19",
        "sldi 19, 19, 32",
        "oris 19, 19, {reg_poison}",
        "addi 19, 19, 19",
        "lis  20, {reg_poison}",
        "addi 20, 20, 20",
        "sldi 20, 20, 32",
        "oris 20, 20, {reg_poison}",
        "addi 20, 20, 20",
        "lis  21, {reg_poison}",
        "addi 21, 21, 21",
        "sldi 21, 21, 32",
        "oris 21, 21, {reg_poison}",
        "addi 21, 21, 21",
        "lis  22, {reg_poison}",
        "addi 22, 22, 22",
        "sldi 22, 22, 32",
        "oris 22, 22, {reg_poison}",
        "addi 22, 22, 22",
        "lis  23, {reg_poison}",
        "addi 23, 23, 23",
        "sldi 23, 23, 32",
        "oris 23, 23, {reg_poison}",
        "addi 23, 23, 23",
        "lis  24, {reg_poison}",
        "addi 24, 24, 24",
        "sldi 24, 24, 32",
        "oris 24, 24, {reg_poison}",
        "addi 24, 24, 24",
        "lis  25, {reg_poison}",
        "addi 25, 25, 25",
        "sldi 25, 25, 32",
        "oris 25, 25, {reg_poison}",
        "addi 25, 25, 25",
        "lis  26, {reg_poison}",
        "addi 26, 26, 26",
        "sldi 26, 26, 32",
        "oris 26, 26, {reg_poison}",
        "addi 26, 26, 26",
        "lis  27, {reg_poison}",
        "addi 27, 27, 27",
        "sldi 27, 27, 32",
        "oris 27, 27, {reg_poison}",
        "addi 27, 27, 27",
        "lis  28, {reg_poison}",
        "addi 28, 28, 28",
        "sldi 28, 28, 32",
        "oris 28, 28, {reg_poison}",
        "addi 28, 28, 28",
        "lis  29, {reg_poison}",
        "addi 29, 29, 29",
        "sldi 29, 29, 32",
        "oris 29, 29, {reg_poison}",
        "addi 29, 29, 29",
        reg_poison = const 0x5a5a,
        out("r15") _,
        out("r16") _,
        out("r17") _,
        out("r18") _,
        out("r19") _,
        out("r20") _,
        out("r21") _,
        out("r22") _,
        out("r23") _,
        out("r24") _,
        out("r25") _,
        out("r26") _,
        out("r27") _,
        out("r28") _,
        out("r29") _,
    );
}

unsafe fn check_regs() -> c_int {
    let mut i: c_ulong;

    i = 15;
    while i <= 29 {
        FAIL_IF(signal_regs.gpr[i as usize] != POISONED_REG(i));
        i += 1;
    }

    printf(b"Regs OK\n\0".as_ptr() as *const c_char);
    0
}

unsafe fn dump_regs() {
    let mut i: c_int = 0;

    while i < 32 {
        printf(
            b"r%02d 0x%016lx  r%02d 0x%016lx  r%02d 0x%016lx  r%02d 0x%016lx\n\0"
                .as_ptr() as *const c_char,
            i,
            signal_regs.gpr[i as usize],
            i + 1,
            signal_regs.gpr[(i + 1) as usize],
            i + 2,
            signal_regs.gpr[(i + 2) as usize],
            i + 3,
            signal_regs.gpr[(i + 3) as usize],
        );
        i += 4;
    }
}

// C conditional:
// #ifdef _CALL_AIXDESC
#[cfg(_CALL_AIXDESC)]
#[repr(C)]
struct opd {
    ip: c_ulong,
    toc: c_ulong,
    env: c_ulong,
}

#[cfg(_CALL_AIXDESC)]
static mut bad_opd: opd = opd {
    ip: BAD_NIP,
    toc: 0,
    env: 0,
};

unsafe fn BAD_FUNC() -> *mut c_void {
    #[cfg(_CALL_AIXDESC)]
    {
        &mut bad_opd as *mut opd as *mut c_void
    }

    #[cfg(not(_CALL_AIXDESC))]
    {
        BAD_NIP as *mut c_void
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_wild_bctr() -> c_int {
    let mut func_ptr: unsafe extern "C" fn() -> c_int;
    let segv: sigaction = sigaction {
        sa_sigaction: segv_handler,
        sa_flags: SA_SIGINFO,
    };
    let usr2: sigaction = sigaction {
        sa_sigaction: usr2_handler,
        sa_flags: SA_SIGINFO,
    };

    FAIL_IF(sigaction(SIGSEGV, &segv as *const sigaction, core::ptr::null_mut()) != 0);
    FAIL_IF(sigaction(SIGUSR2, &usr2 as *const sigaction, core::ptr::null_mut()) != 0);

    bzero(
        &mut signal_regs as *mut pt_regs as *mut c_void,
        size_of::<pt_regs>(),
    );

    if setjmp(&mut setjmp_env as *mut jmp_buf) == 0 {
        func_ptr = ok;
        func_ptr();

        kill(getpid(), SIGUSR2);
        printf(b"Regs before:\n\0".as_ptr() as *const c_char);
        dump_regs();
        bzero(
            &mut signal_regs as *mut pt_regs as *mut c_void,
            size_of::<pt_regs>(),
        );

        poison_regs();

        func_ptr = core::mem::transmute::<*mut c_void, unsafe extern "C" fn() -> c_int>(BAD_FUNC());
        func_ptr();

        FAIL_IF(true); /* we didn't segv? */
    }

    FAIL_IF(signal_regs.nip != BAD_NIP);

    printf(
        b"All good - took SEGV as expected branching to 0x%llx\n\0".as_ptr() as *const c_char,
        BAD_NIP,
    );

    dump_regs();
    FAIL_IF(check_regs() != 0);

    0
}

fn main() {
    unsafe {
        std::process::exit(test_harness(
            test_wild_bctr,
            b"wild_bctr\0".as_ptr() as *const c_char,
        ));
    }
}
