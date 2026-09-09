// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/sh/boards/se/7721/irq.c
 *
 * Copyright (C) 2008  Renesas Solutions Corp.
 */

// Linux kernel dependencies supplied by the surrounding build.

enum {
    UNUSED = 0,

    /* board specific interrupt sources */
    MRSHPC,
}

static mut VECTORS: [intc_vect; 1] = [INTC_IRQ!(MRSHPC, MRSHPC_IRQ0)];

static mut PRIO_REGISTERS: [intc_prio_reg; 1] = [intc_prio_reg {
    fpga_ilsr: FPGA_ILSR6,
    pos: 0,
    width: 8,
    shift: 4,
    // IRLMSK
    values: [0, MRSHPC],
}];

static mut INTC_DESC: intc_desc = DECLARE_INTC_DESC!(
    "SE7721",
    VECTORS,
    core::ptr::null_mut(),
    core::ptr::null_mut(),
    PRIO_REGISTERS,
    core::ptr::null_mut(),
);

extern "C" {
    static FPGA_ILSR6: u32;
    static MRSHPC_IRQ0: u32;

    fn __raw_readw(address: usize) -> u16;
    fn __raw_writew(value: u16, address: usize);
    fn register_intc_controller(desc: *mut intc_desc);
    fn intc_set_priority(irq: u32, priority: u32);
}

/*
 * Initialize IRQ setting
 */
#[no_mangle]
pub unsafe extern "C" fn init_se7721_IRQ() {
    /* PPCR */
    __raw_writew(__raw_readw(0xa4050118) & !0x00ff, 0xa4050118);

    register_intc_controller(&raw mut INTC_DESC);
    intc_set_priority(MRSHPC_IRQ0, 0xf - MRSHPC_IRQ0);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
