// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2007 Lemote Inc. & Institute of Computing Technology
 * Author: Fuxin Zhang, zhangfx@lemote.com
 */

// Dependencies supplied by the surrounding kernel and platform code.
unsafe extern "C" {
    static mut LOONGSON_INTISR: u32;
    static mut LOONGSON_INTEN: u32;
    static mut LOONGSON_INTSTEER: u32;
    static mut LOONGSON_INTENCLR: u32;

    static LOONGSON_IRQ_BASE: u32;
    static ST0_IM: u32;
    static ST0_BEV: u32;

    fn udelay(usecs: u32);
    fn __ffs(word: u32) -> i32;
    fn do_IRQ(irq: u32);
    fn read_c0_cause() -> u32;
    fn read_c0_status() -> u32;
    fn clear_c0_status(bits: u32);
    fn mach_irq_dispatch(pending: u32);
    fn mach_init_irq();
}

/*
 * the first level int-handler will jump here if it is a bonito irq
 */
pub unsafe extern "C" fn bonito_irqdispatch() {
    let mut int_status: u32;
    let mut i: i32;

    /* workaround the IO dma problem: let cpu looping to allow DMA finish */
    int_status = core::ptr::read_volatile(&raw const LOONGSON_INTISR);
    while (int_status & (1u32 << 10)) != 0 {
        udelay(1);
        int_status = core::ptr::read_volatile(&raw const LOONGSON_INTISR);
    }

    /* Get pending sources, masked by current enables */
    int_status = core::ptr::read_volatile(&raw const LOONGSON_INTISR)
        & core::ptr::read_volatile(&raw const LOONGSON_INTEN);

    if int_status != 0 {
        i = __ffs(int_status);
        do_IRQ(LOONGSON_IRQ_BASE + i as u32);
    }
}

pub unsafe extern "C" fn plat_irq_dispatch() {
    let pending: u32;

    pending = read_c0_cause() & read_c0_status() & ST0_IM;

    /* machine-specific plat_irq_dispatch */
    mach_irq_dispatch(pending);
}

pub unsafe extern "C" fn arch_init_irq() {
    /*
     * Clear all of the interrupts while we change the able around a bit.
     * int-handler is not on bootstrap
     */
    clear_c0_status(ST0_IM | ST0_BEV);

    /* no steer */
    core::ptr::write_volatile(&raw mut LOONGSON_INTSTEER, 0);

    /*
     * Mask out all interrupt by writing "1" to all bit position in
     * the interrupt reset reg.
     */
    core::ptr::write_volatile(&raw mut LOONGSON_INTENCLR, !0u32);

    /* machine specific irq init */
    mach_init_irq();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
