/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2014-2015 The Linux Foundation. All rights reserved.
 *
 * A call to __dcc_getchar() or __dcc_putchar() is typically followed by
 * a call to __dcc_getstatus().  We want to make sure that the CPU does
 * not speculative read the DCC status before executing the read or write
 * instruction.  That's what the ISBs are for.
 *
 * The 'volatile' ensures that the compiler does not cache the status bits,
 * and instead reads the DCC register every time.
 */

// C dependencies supplied by asm/barrier.h and asm/sysreg.h are expected
// to provide `read_sysreg`, `write_sysreg`, and `isb`.

pub unsafe fn __dcc_getstatus() -> u32 {
    read_sysreg(mdccsr_el0)
}

pub unsafe fn __dcc_getchar() -> i8 {
    let c: i8 = read_sysreg(dbgdtrrx_el0);
    isb();

    c
}

pub unsafe fn __dcc_putchar(c: i8) {
    /*
     * The typecast is to make absolutely certain that 'c' is
     * zero-extended.
     */
    write_sysreg(c as u8, dbgdtrtx_el0);
    isb();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
