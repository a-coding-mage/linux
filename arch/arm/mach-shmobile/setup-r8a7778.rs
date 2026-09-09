// SPDX-License-Identifier: GPL-2.0
/*
 * r8a7778 processor support
 *
 * Copyright (C) 2013  Renesas Solutions Corp.
 * Copyright (C) 2013  Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>
 * Copyright (C) 2013  Cogent Embedded, Inc.
 */

use core::ffi::c_char;

// Dependencies supplied by the kernel and other translation units.
unsafe extern "C" {
    fn ioremap(phys_addr: usize, size: usize) -> *mut u8;
    fn iounmap(addr: *mut u8);
    fn irqchip_init();
    fn writel(value: u32, addr: *mut u8);
    fn shmobile_init_delay();
    fn shmobile_init_late();
    fn BUG_ON(condition: bool);
}

const HPBREG_BASE: usize = 0xfe70_0000;

const INT2SMSKCR0: usize = 0x82288; // 0xfe782288
const INT2SMSKCR1: usize = 0x8228c; // 0xfe78228c

const INT2NTSR0: usize = 0x00018; // 0xfe700018
const INT2NTSR1: usize = 0x0002c; // 0xfe70002c

unsafe fn r8a7778_init_irq_dt() {
    let base: *mut u8 = ioremap(HPBREG_BASE, 0x0010_0000);

    BUG_ON(base.is_null());

    irqchip_init();

    /* route all interrupts to ARM */
    writel(0x73ff_ffff, base.add(INT2NTSR0));
    writel(0xffff_ffff, base.add(INT2NTSR1));

    /* unmask all known interrupts in INTCS2 */
    writel(0x0833_0773, base.add(INT2SMSKCR0));
    writel(0x0031_1110, base.add(INT2SMSKCR1));

    iounmap(base);
}

static r8a7778_compat_dt: [*const c_char; 2] = [
    c"renesas,r8a7778".as_ptr(),
    core::ptr::null(),
];

// DT_MACHINE_START(R8A7778_DT, "Generic R8A7778 (Flattened Device Tree)")
//     .init_early = shmobile_init_delay,
//     .init_irq = r8a7778_init_irq_dt,
//     .init_late = shmobile_init_late,
//     .dt_compat = r8a7778_compat_dt,
// MACHINE_END
// The machine-descriptor macro is provided by the surrounding kernel build.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
