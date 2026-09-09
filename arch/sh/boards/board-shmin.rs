// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/boards/shmin/setup.c
 *
 * Copyright (C) 2006 Takashi YOSHII
 *
 * SHMIN Support.
 */

// Declarations supplied by the surrounding kernel dependencies.
extern "C" {
    fn __raw_writew(value: u16, address: usize);
    fn plat_irq_setup_pins(mode: u32);
    fn __set_io_port_base(base: usize);
}

extern "C" {
    static SHMIN_IO_BASE: usize;
    static IRQ_MODE_IRQ: u32;
}

const PFC_PHCR: usize = 0xa400010e;
const INTC_ICR1: usize = 0xa4000010;

unsafe fn init_shmin_irq() {
    __raw_writew(0x2a00, PFC_PHCR); // IRQ0-3=IRQ
    __raw_writew(0x0aaa, INTC_ICR1); // IRQ0-3=IRQ-mode,Low-active.
    plat_irq_setup_pins(IRQ_MODE_IRQ);
}

unsafe fn shmin_setup(_cmdline_p: *mut *mut i8) {
    __set_io_port_base(SHMIN_IO_BASE);
}

#[repr(C)]
struct ShMachineVector {
    mv_name: *const u8,
    mv_setup: unsafe fn(*mut *mut i8),
    mv_init_irq: unsafe fn(),
}

// __initmv
static mut mv_shmin: ShMachineVector = ShMachineVector {
    mv_name: b"SHMIN\0".as_ptr(),
    mv_setup: shmin_setup,
    mv_init_irq: init_shmin_irq,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
