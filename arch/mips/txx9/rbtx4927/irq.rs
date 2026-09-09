/*
 * Toshiba RBTX4927 specific interrupt handlers
 *
 * Author: MontaVista Software, Inc.
 *	   source@mvista.com
 *
 * Copyright 2001-2002 MontaVista Software Inc.
 *
 * This file is a direct Rust translation of irq.c. Linux and platform
 * declarations supplied by the original include files are external.
 */

use core::ffi::c_int;

// Original dependencies: linux/init.h, linux/types.h, linux/interrupt.h,
// linux/irq.h, asm/io.h, asm/mipsregs.h, asm/txx9/generic.h,
// asm/txx9/rbtx4927.h.

extern "C" {
    static mut rbtx4927_imstat_addr: *mut u8;
    static mut rbtx4927_imask_addr: *mut u8;
    static mut rbtx4927_softint_addr: *mut u8;
    static mut txx9_irq_dispatch: Option<unsafe extern "C" fn(c_int) -> c_int>;

    fn readb(addr: *mut u8) -> u8;
    fn writeb(value: u8, addr: *mut u8);
    fn mmiowb();
    fn __fls8(value: u8) -> c_int;
    fn txx9_irq() -> c_int;
    fn tx4927_irq_init();
    fn irq_set_chip_and_handler(irq: c_int, chip: *const irq_chip, handler: unsafe extern "C" fn());
    fn irq_set_chained_handler(irq: c_int, handler: unsafe extern "C" fn());
    fn irq_set_irq_type(irq: c_int, flags: c_int);

    static handle_level_irq: unsafe extern "C" fn();
    static handle_simple_irq: unsafe extern "C" fn();
}

#[repr(C)]
pub struct irq_data {
    pub irq: c_int,
}

#[repr(C)]
pub struct irq_chip {
    pub name: *const u8,
    pub irq_mask: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_unmask: Option<unsafe extern "C" fn(*mut irq_data)>,
}

unsafe fn toshiba_rbtx4927_irq_nested(_sw_irq: c_int) -> c_int {
    let level3: u8 = readb(rbtx4927_imstat_addr) & 0x1f;
    if level3 == 0 {
        return -1;
    }
    RBTX4927_IRQ_IOC + __fls8(level3)
}

unsafe extern "C" fn toshiba_rbtx4927_irq_ioc_enable(d: *mut irq_data) {
    let mut v: u8 = readb(rbtx4927_imask_addr);
    v |= 1u8 << ((*d).irq - RBTX4927_IRQ_IOC);
    writeb(v, rbtx4927_imask_addr);
}

unsafe extern "C" fn toshiba_rbtx4927_irq_ioc_disable(d: *mut irq_data) {
    let mut v: u8 = readb(rbtx4927_imask_addr);
    v &= !(1u8 << ((*d).irq - RBTX4927_IRQ_IOC));
    writeb(v, rbtx4927_imask_addr);
    mmiowb();
}

pub const TOSHIBA_RBTX4927_IOC_NAME: &[u8] = b"RBTX4927-IOC\0";

static mut toshiba_rbtx4927_irq_ioc_type: irq_chip = irq_chip {
    name: TOSHIBA_RBTX4927_IOC_NAME.as_ptr(),
    irq_mask: Some(toshiba_rbtx4927_irq_ioc_disable),
    irq_unmask: Some(toshiba_rbtx4927_irq_ioc_enable),
};

unsafe fn toshiba_rbtx4927_irq_ioc_init() {
    writeb(0, rbtx4927_imask_addr);
    writeb(0, rbtx4927_softint_addr);

    let mut i = RBTX4927_IRQ_IOC;
    while i < RBTX4927_IRQ_IOC + RBTX4927_NR_IRQ_IOC {
        irq_set_chip_and_handler(i, &toshiba_rbtx4927_irq_ioc_type, handle_level_irq);
        i += 1;
    }
    irq_set_chained_handler(RBTX4927_IRQ_IOCINT, handle_simple_irq);
}

unsafe extern "C" fn rbtx4927_irq_dispatch(pending: c_int) -> c_int {
    let irq: c_int;
    if pending & STATUSF_IP7 != 0 {
        irq = MIPS_CPU_IRQ_BASE + 7;
    } else if pending & STATUSF_IP2 != 0 {
        let mut value = txx9_irq();
        if value == RBTX4927_IRQ_IOCINT {
            value = toshiba_rbtx4927_irq_nested(value);
        }
        irq = value;
    } else if pending & STATUSF_IP0 != 0 {
        irq = MIPS_CPU_IRQ_BASE;
    } else if pending & STATUSF_IP1 != 0 {
        irq = MIPS_CPU_IRQ_BASE + 1;
    } else {
        irq = -1;
    }
    irq
}

pub unsafe fn rbtx4927_irq_setup() {
    txx9_irq_dispatch = Some(rbtx4927_irq_dispatch);
    tx4927_irq_init();
    toshiba_rbtx4927_irq_ioc_init();
    irq_set_irq_type(RBTX4927_RTL_8019_IRQ, IRQF_TRIGGER_HIGH);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
