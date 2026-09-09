// SPDX-License-Identifier: GPL-2.0
//
// Dependencies supplied by the surrounding kernel and math-emulation code are
// intentionally referenced here rather than reimplemented.

pub unsafe fn mtfsf(FM: u32, frB: *mut u32) -> i32 {
    let mask: u32;
    let mut fpscr: u32;

    if FM == 1 {
        mask = 0x0f;
    } else if FM == 0xff {
        mask = !0;
    } else {
        mask = ((FM & 1)
            | ((FM << 3) & 0x10)
            | ((FM << 6) & 0x100)
            | ((FM << 9) & 0x1000)
            | ((FM << 12) & 0x10000)
            | ((FM << 15) & 0x100000)
            | ((FM << 18) & 0x1000000)
            | ((FM << 21) & 0x10000000))
            .wrapping_mul(15);
    }

    fpscr = (__FPU_FPSCR & !mask | (*frB.add(1) & mask))
        & !(FPSCR_VX | FPSCR_FEX | 0x800);

    if fpscr
        & (FPSCR_VXSNAN
            | FPSCR_VXISI
            | FPSCR_VXIDI
            | FPSCR_VXZDZ
            | FPSCR_VXIMZ
            | FPSCR_VXVC
            | FPSCR_VXSOFT
            | FPSCR_VXSQRT
            | FPSCR_VXCVI)
        != 0
    {
        fpscr |= FPSCR_VX;
    }

    /* The bit order of exception enables and exception status
     * is the same. Simply shift and mask to check for enabled
     * exceptions.
     */
    if fpscr & (fpscr >> 22) & 0xf8 != 0 {
        fpscr |= FPSCR_FEX;
    }

    __FPU_FPSCR = fpscr;

    // #ifdef DEBUG
    // printk!("{}: {:02x} {:p}: {:08x}\n", module_path!(), FM, frB, __FPU_FPSCR);
    // #endif

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
