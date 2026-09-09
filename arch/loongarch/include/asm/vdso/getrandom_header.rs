// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2024 Xi Ruoyao <xry111@xry111.site>. All Rights Reserved.
 */

// C header guard: __ASM_VDSO_GETRANDOM_H
// C-only assembler exclusion: !__ASSEMBLER__
// Dependencies: asm/unistd.h and asm/vdso/vdso.h

/// Direct translation of the LoongArch getrandom system-call wrapper.
#[inline(always)]
pub unsafe fn getrandom_syscall(
    _buffer: *mut core::ffi::c_void,
    _len: usize,
    _flags: u32,
) -> isize {
    let mut ret: isize;
    let nr: isize = __NR_getrandom as isize;
    let buffer = _buffer;
    let len = _len;
    let flags = _flags;

    core::arch::asm!(
        "syscall 0",
        inout("a0") buffer => ret,
        in("a7") nr,
        in("a1") len,
        in("a2") flags,
        clobber_abi("C"),
        options(nostack),
    );

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
