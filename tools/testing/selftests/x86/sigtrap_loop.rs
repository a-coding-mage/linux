// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2025 Intel Corporation
 */

use libc::{
    c_int, c_void, err, exit, printf, sigaction, sigemptyset, siginfo_t, ucontext_t, SA_SIGINFO,
    SIGTRAP,
};
use std::arch::asm;
use std::mem;

#[cfg(target_arch = "x86_64")]
const REG_IP: usize = libc::REG_RIP as usize;
#[cfg(not(target_arch = "x86_64"))]
const REG_IP: usize = libc::REG_EIP as usize;

unsafe fn sethandler(
    sig: c_int,
    handler: unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void),
    flags: c_int,
) {
    let mut sa: sigaction = mem::zeroed();

    sa.sa_sigaction = handler as usize;
    sa.sa_flags = SA_SIGINFO | flags;
    sigemptyset(&mut sa.sa_mask);

    if sigaction(sig, &sa, std::ptr::null_mut()) != 0 {
        err(1, b"sigaction\0".as_ptr() as *const libc::c_char);
    }

    return;
}

unsafe extern "C" fn sigtrap(_sig: c_int, _info: *mut siginfo_t, ctx_void: *mut c_void) {
    let ctx: *mut ucontext_t = ctx_void as *mut ucontext_t;
    static mut LOOP_COUNT_ON_SAME_IP: libc::c_uint = 0;
    static mut LAST_TRAP_IP: libc::c_ulong = 0;

    if LAST_TRAP_IP == (*ctx).uc_mcontext.gregs[REG_IP] as libc::c_ulong {
        printf(
            b"\tTrapped at %016lx\n\0".as_ptr() as *const libc::c_char,
            LAST_TRAP_IP,
        );

        /*
         * If the same IP is hit more than 10 times in a row, it is
         * _considered_ an infinite loop.
         */
        LOOP_COUNT_ON_SAME_IP = LOOP_COUNT_ON_SAME_IP.wrapping_add(1);
        if LOOP_COUNT_ON_SAME_IP > 10 {
            printf(
                b"[FAIL]\tDetected SIGTRAP infinite loop\n\0".as_ptr() as *const libc::c_char,
            );
            exit(1);
        }

        return;
    }

    LOOP_COUNT_ON_SAME_IP = 0;
    LAST_TRAP_IP = (*ctx).uc_mcontext.gregs[REG_IP] as libc::c_ulong;
    printf(
        b"\tTrapped at %016lx\n\0".as_ptr() as *const libc::c_char,
        LAST_TRAP_IP,
    );
}

fn main() {
    unsafe {
        sethandler(SIGTRAP, sigtrap, 0);

        /*
         * Set the Trap Flag (TF) to single-step the test code, therefore to
         * trigger a SIGTRAP signal after each instruction until the TF is
         * cleared.
         *
         * Because the arithmetic flags are not significant here, the TF is
         * set by pushing 0x302 onto the stack and then popping it into the
         * flags register.
         *
         * Four instructions in the following asm code are executed with the
         * TF set, thus the SIGTRAP handler is expected to run four times.
         */
        printf(
            b"[RUN]\tSIGTRAP infinite loop detection\n\0".as_ptr() as *const libc::c_char,
        );

        #[cfg(target_arch = "x86_64")]
        asm!(
            /*
             * Avoid clobbering the redzone
             *
             * Equivalent to "sub $128, %rsp", however -128 can be encoded
             * in a single byte immediate while 128 uses 4 bytes.
             */
            "add rsp, -128",
            "push 0x302",
            "popfq",
            "nop",
            "nop",
            "push 0x202",
            "popfq",
            "sub rsp, -128",
            options(att_syntax)
        );

        #[cfg(not(target_arch = "x86_64"))]
        asm!(
            "push 0x302",
            "popf",
            "nop",
            "nop",
            "push 0x202",
            "popf",
            options(att_syntax)
        );

        printf(
            b"[OK]\tNo SIGTRAP infinite loop detected\n\0".as_ptr() as *const libc::c_char,
        );
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
