// SPDX-License-Identifier: GPL-2.0
// C dependencies translated by reference:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::arch::asm;

#[no_mangle]
pub static mut call_happened: i32 = 0;

/*
 * 32765 is the exact minimum number of padding instructions needed to
 * trigger the verifier failure, because:
 * 1. Counting the wrapper instructions around the padding block (one
 *    "r0=0" and two "exit" instructions), the actual jump distance
 *    evaluates to N + 3.
 * 2. To overflow the s16 max bound (32767), we need N + 3 > 32767.
 * Thus, N = 32765 is the exact minimum padding size required.
 */
#[inline(never)]
unsafe fn padding_subprog() {
    asm!(
        "r0 = 0;",
        ".rept 32765;",
        "r0 += 0;",
        ".endr;",
        options(nostack, preserves_flags),
    );
}

#[inline(never)]
unsafe fn target_subprog() -> i32 {
    /* Use volatile variable here to prevent optimization. */
    let magic_ret: i32 = 3;
    core::ptr::read_volatile(&magic_ret)
}

// SEC("syscall")
// __success __retval(3)
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn call_large_imm_test(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;

    /*
     * Landing pad to handle call error on kernel without the fix,
     * preventing kernel panic.
     */
    asm!(
        "r0 = 0;",
        ".rept 32768;",
        "r0 += 0;",
        ".endr;",
        options(nostack, preserves_flags),
    );

    /*
     * The call_happened variable is 1 only when the call insn wrongly
     * go back to the landing pad above.
     */
    if call_happened == 1 {
        /* Use volatile variable here to prevent optimization. */
        let flag: i32 = -1;
        return core::ptr::read_volatile(&flag);
    }

    call_happened = 1;

    padding_subprog();

    target_subprog()
}

#[no_mangle]
#[link_section = "license"]
pub static LICENSE: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
