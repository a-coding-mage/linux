// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/sh/boards/se/7780/irq.c
 *
 * Copyright (C) 2006,2007  Nobuhiro Iwamatsu
 *
 * Hitachi UL SolutionEngine 7780 Support.
 */

// Linux kernel dependencies supplied by other translation units.
unsafe extern "C" {
    fn __raw_writew(value: u16, address: usize);
    fn __raw_readw(address: usize) -> u16;
    fn __raw_writel(value: u32, address: usize);
    fn plat_irq_setup_pins(mode: u32);

    static FPGA_INTMSK1: usize;
    static FPGA_INTMSK2: usize;
    static FPGA_INTSEL1: usize;
    static FPGA_INTSEL2: usize;
    static FPGA_INTSEL3: usize;
    static FPGA_PCI_INTSEL1: usize;
    static FPGA_PCI_INTSEL2: usize;
    static IRQPIN_SM501: u16;
    static IRQPOS_SM501: u32;
    static IRQPIN_SMC91CX: u16;
    static IRQPOS_SMC91CX: u32;
    static IRQPIN_EXTINT4: u16;
    static IRQPOS_EXTINT4: u32;
    static IRQPIN_EXTINT3: u16;
    static IRQPOS_EXTINT3: u32;
    static IRQPIN_EXTINT2: u16;
    static IRQPOS_EXTINT2: u32;
    static IRQPIN_EXTINT1: u16;
    static IRQPOS_EXTINT1: u32;
    static IRQPIN_PCCPW: u16;
    static IRQPOS_PCCPW: u32;
    static IRQ_MODE_IRQ: u32;
}

const INTC_BASE: usize = 0xffd00000;
const INTC_ICR1: usize = INTC_BASE + 0x1c;

/*
 * Initialize IRQ setting
 */
pub unsafe fn init_se7780_IRQ() {
    /* enable all interrupt at FPGA */
    unsafe { __raw_writew(0, FPGA_INTMSK1) };
    /* mask SM501 interrupt */
    unsafe {
        __raw_writew(
            __raw_readw(FPGA_INTMSK1) | 0x0002,
            FPGA_INTMSK1,
        )
    };
    /* enable all interrupt at FPGA */
    unsafe { __raw_writew(0, FPGA_INTMSK2) };

    /* set FPGA INTSEL register */
    /* FPGA + 0x06 */
    unsafe {
        __raw_writew(
            ((*IRQPIN_SM501 << *IRQPOS_SM501)
                | (*IRQPIN_SMC91CX << *IRQPOS_SMC91CX)),
            FPGA_INTSEL1,
        )
    };

    /* FPGA + 0x08 */
    unsafe {
        __raw_writew(
            ((*IRQPIN_EXTINT4 << *IRQPOS_EXTINT4)
                | (*IRQPIN_EXTINT3 << *IRQPOS_EXTINT3)
                | (*IRQPIN_EXTINT2 << *IRQPOS_EXTINT2)
                | (*IRQPIN_EXTINT1 << *IRQPOS_EXTINT1)),
            FPGA_INTSEL2,
        )
    };

    /* FPGA + 0x0A */
    unsafe { __raw_writew(*IRQPIN_PCCPW << *IRQPOS_PCCPW, FPGA_INTSEL3) };

    unsafe { plat_irq_setup_pins(*IRQ_MODE_IRQ) }; /* install handlers for IRQ0-7 */

    /* ICR1: detect low level(for 2ndcut) */
    unsafe { __raw_writel(0xAAAA0000, INTC_ICR1) };

    /*
     * FPGA PCISEL register initialize
     *
     *  CPU  || SLOT1 | SLOT2 | S-ATA | USB
     *  -------------------------------------
     *  INTA || INTA  | INTD  |  --   | INTB
     *  -------------------------------------
     *  INTB || INTB  | INTA  |  --   | INTC
     *  -------------------------------------
     *  INTC || INTC  | INTB  | INTA  |  --
     *  -------------------------------------
     *  INTD || INTD  | INTC  |  --   | INTA
     *  -------------------------------------
     */
    unsafe { __raw_writew(0x0013, FPGA_PCI_INTSEL1) };
    unsafe { __raw_writew(0xE402, FPGA_PCI_INTSEL2) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
