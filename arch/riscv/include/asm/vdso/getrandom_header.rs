/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2025 Xi Ruoyao <xry111@xry111.site>. All Rights Reserved.
 */

// Dependency intent: __NR_getrandom is supplied by the architecture syscall definitions.

/// Invoke the RISC-V getrandom system call.
#[inline(always)]
pub unsafe fn getrandom_syscall(
    _buffer: *mut core::ffi::c_void,
    _len: usize,
    _flags: u32,
) -> isize {
    let ret: isize;
    let nr: isize = __NR_getrandom as isize;

    core::arch::asm!(
        "ecall",
        inlateout("a0") _buffer => ret,
        in("a7") nr,
        in("a1") _len,
        in("a2") _flags,
        options(nostack),
    );

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
