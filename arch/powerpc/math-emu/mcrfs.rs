// SPDX-License-Identifier: GPL-2.0

// External dependencies supplied by the surrounding kernel/math-emulation code.
unsafe extern "C" {
    static mut __FPU_FPSCR: u32;
}

pub unsafe fn mcrfs(ccr: *mut u32, crf_d: u32, crf_s: u32) -> i32 {
    let value: u32;
    let mut clear: u32;

    // #ifdef DEBUG
    // printk("%s: %p (%08x) %d %d\n", __func__, ccr, *ccr, crfD, crfS);
    // #endif

    clear = 15 << ((7 - crf_s) << 2);
    if crf_s == 0 {
        clear = 0x9000_0000;
    }

    value = (__FPU_FPSCR >> ((7 - crf_s) << 2)) & 15;
    __FPU_FPSCR &= !clear;

    *ccr &= !(15 << ((7 - crf_d) << 2));
    *ccr |= value << ((7 - crf_d) << 2);

    // #ifdef DEBUG
    // printk("CR: %08x\n", __func__, *ccr);
    // #endif

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
