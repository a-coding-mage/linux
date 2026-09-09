// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2011 Richard Weinberger <richrd@nod.at>
 * Mostly copied from arch/x86/lib/delay.c
 */

// Symbols supplied by the surrounding kernel build.
unsafe extern "C" {
    static loops_per_jiffy: u32;
    static HZ: u32;
}

#[no_mangle]
pub unsafe extern "C" fn __delay(mut loops: u32) {
    core::arch::asm!(
        "test {0:e}, {0:e}",
        "jz 3f",
        "jmp 1f",
        ".align 16",
        "1: jmp 2f",
        ".align 16",
        "2: dec {0:e}",
        "jnz 2b",
        "3: dec {0:e}",
        inout(reg) loops,
        options(nostack, preserves_flags),
    );
}

#[no_mangle]
pub unsafe extern "C" fn __const_udelay(mut xloops: u32) {
    let mut d0: u32;

    xloops = xloops.wrapping_mul(4);
    let mut mult = (*loops_per_jiffy).wrapping_mul(*HZ / 4);
    core::arch::asm!(
        "mull %edx",
        inout("eax") xloops => d0,
        inout("edx") mult => xloops,
        options(nostack),
    );

    __delay(xloops.wrapping_add(1));
}

#[no_mangle]
pub unsafe extern "C" fn __udelay(usecs: u32) {
    __const_udelay(usecs.wrapping_mul(0x0000_10c7));
}

#[no_mangle]
pub unsafe extern "C" fn __ndelay(nsecs: u32) {
    __const_udelay(nsecs.wrapping_mul(0x0000_0005));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
