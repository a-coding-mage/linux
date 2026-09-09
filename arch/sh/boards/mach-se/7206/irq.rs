// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/sh/boards/se/7206/irq.c
 *
 * Copyright (C) 2005,2006 Yoshinori Sato
 *
 * Hitachi SolutionEngine Support.
 *
 */

// C headers and mach-se/mach/se7206.h provide the following types and
// functions in the surrounding kernel translation.

const INTSTS0: usize = 0x31800000;
const INTSTS1: usize = 0x31800002;
const INTMSK0: usize = 0x31800004;
const INTMSK1: usize = 0x31800006;
const INTSEL: usize = 0x31800008;

const IRQ0_IRQ: u32 = 64;
const IRQ1_IRQ: u32 = 65;
const IRQ3_IRQ: u32 = 67;

const INTC_IPR01: usize = 0xfffe0818;
const INTC_ICR1: usize = 0xfffe0802;

unsafe fn disable_se7206_irq(data: *mut irq_data) {
    let irq: u32 = (*data).irq;
    let mut val: u16;
    let mask: u16 = 0xffff ^ (0x0f << (4 * (3 - (IRQ0_IRQ - irq))));
    let mut msk0: u16;
    let mut msk1: u16;

    /* Set the priority in IPR to 0 */
    val = __raw_readw(INTC_IPR01);
    val &= mask;
    __raw_writew(val, INTC_IPR01);
    /* FPGA mask set */
    msk0 = __raw_readw(INTMSK0);
    msk1 = __raw_readw(INTMSK1);

    match irq {
        IRQ0_IRQ => msk0 |= 0x0010,
        IRQ1_IRQ => msk0 |= 0x000f,
        IRQ3_IRQ => {
            msk0 |= 0x0f00;
            msk1 |= 0x00ff;
        }
        _ => {}
    }
    __raw_writew(msk0, INTMSK0);
    __raw_writew(msk1, INTMSK1);
}

unsafe fn enable_se7206_irq(data: *mut irq_data) {
    let irq: u32 = (*data).irq;
    let mut val: u16;
    let value: u16 = 0x0001 << (4 * (3 - (IRQ0_IRQ - irq)));
    let mut msk0: u16;
    let mut msk1: u16;

    /* Set priority in IPR back to original value */
    val = __raw_readw(INTC_IPR01);
    val |= value;
    __raw_writew(val, INTC_IPR01);

    /* FPGA mask reset */
    msk0 = __raw_readw(INTMSK0);
    msk1 = __raw_readw(INTMSK1);

    match irq {
        IRQ0_IRQ => msk0 &= !0x0010,
        IRQ1_IRQ => msk0 &= !0x000f,
        IRQ3_IRQ => {
            msk0 &= !0x0f00;
            msk1 &= !0x00ff;
        }
        _ => {}
    }
    __raw_writew(msk0, INTMSK0);
    __raw_writew(msk1, INTMSK1);
}

unsafe fn eoi_se7206_irq(data: *mut irq_data) {
    let mut sts0: u16;
    let mut sts1: u16;
    let irq: u32 = (*data).irq;

    if !irqd_irq_disabled(data) && !irqd_irq_inprogress(data) {
        enable_se7206_irq(data);
    }
    /* FPGA isr clear */
    sts0 = __raw_readw(INTSTS0);
    sts1 = __raw_readw(INTSTS1);

    match irq {
        IRQ0_IRQ => sts0 &= !0x0010,
        IRQ1_IRQ => sts0 &= !0x000f,
        IRQ3_IRQ => {
            sts0 &= !0x0f00;
            sts1 &= !0x00ff;
        }
        _ => {}
    }
    __raw_writew(sts0, INTSTS0);
    __raw_writew(sts1, INTSTS1);
}

static mut se7206_irq_chip: irq_chip = irq_chip {
    name: "SE7206-FPGA",
    irq_mask: Some(disable_se7206_irq),
    irq_unmask: Some(enable_se7206_irq),
    irq_eoi: Some(eoi_se7206_irq),
};

unsafe fn make_se7206_irq(irq: u32) {
    disable_irq_nosync(irq);
    irq_set_chip_and_handler_name(
        irq,
        &raw mut se7206_irq_chip,
        handle_level_irq,
        "level",
    );
    disable_se7206_irq(irq_get_irq_data(irq));
}

/*
 * Initialize IRQ setting
 */
pub unsafe fn init_se7206_IRQ() {
    make_se7206_irq(IRQ0_IRQ); /* SMC91C111 */
    make_se7206_irq(IRQ1_IRQ); /* ATA */
    make_se7206_irq(IRQ3_IRQ); /* SLOT / PCM */

    __raw_writew(__raw_readw(INTC_ICR1) | 0x000b, INTC_ICR1); /* ICR1 */

    /* FPGA System register setup*/
    __raw_writew(0x0000, INTSTS0); /* Clear INTSTS0 */
    __raw_writew(0x0000, INTSTS1); /* Clear INTSTS1 */

    /* IRQ0=LAN, IRQ1=ATA, IRQ3=SLT,PCM */
    __raw_writew(0x0001, INTSEL);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
