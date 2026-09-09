// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/sh/boards/lboxre2/irq.c
 *
 * Copyright (C) 2007 Nobuhiro Iwamatsu
 *
 * NTT COMWARE L-BOX RE2 Support.
 */

// Dependencies supplied by the surrounding kernel environment:
// linux/init.h, linux/interrupt.h, linux/irq.h, asm/irq.h, asm/io.h,
// and mach/lboxre2.h.

extern "C" {
    fn make_imask_irq(irq: i32);
}

// IRQ_CF1, IRQ_CF0, IRQ_INTD, IRQ_ETH1, IRQ_ETH0, and IRQ_INTA are supplied
// by mach/lboxre2.h.
extern "C" {
    static IRQ_CF1: i32;
    static IRQ_CF0: i32;
    static IRQ_INTD: i32;
    static IRQ_ETH1: i32;
    static IRQ_ETH0: i32;
    static IRQ_INTA: i32;
}

/*
 * Initialize IRQ setting
 */
pub unsafe fn init_lboxre2_IRQ() {
    unsafe {
        make_imask_irq(IRQ_CF1);
        make_imask_irq(IRQ_CF0);
        make_imask_irq(IRQ_INTD);
        make_imask_irq(IRQ_ETH1);
        make_imask_irq(IRQ_ETH0);
        make_imask_irq(IRQ_INTA);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
