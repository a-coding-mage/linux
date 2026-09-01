// SPDX-License-Identifier: GPL-2.0+

/*
 * Part of fork context switch microbenchmark.
 *
 * Copyright 2018, Anton Blanchard, IBM Corp.
 */

// C source defined _GNU_SOURCE and included <sys/syscall.h> for SYS_exit.
const SYS_exit: i32 = 1;

#[no_mangle]
pub unsafe extern "C" fn _start() {
    core::arch::asm!(
        "li 0, {sys_exit}",
        "li 3, 0",
        "sc",
        sys_exit = const SYS_exit,
        /*
         * "sc" will clobber r0, r3-r13, cr0, ctr, xer and memory.
         * Even though sys_exit never returns, handle clobber
         * registers.
         */
        out("r0") _,
        out("r3") _,
        out("r4") _,
        out("r5") _,
        out("r6") _,
        out("r7") _,
        out("r8") _,
        out("r9") _,
        out("r10") _,
        out("r11") _,
        out("r12") _,
        out("r13") _,
        out("cr0") _,
        out("ctr") _,
        out("xer") _,
        options(noreturn),
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
