// SPDX-License-Identifier: GPL-2.0
/*
 * R8A7740 processor support
 *
 * Copyright (C) 2011  Renesas Solutions Corp.
 * Copyright (C) 2011  Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>
 */

// C dependencies supplied by the surrounding kernel translation.
use core::ffi::c_void;

extern "C" {
    fn ioremap(addr: usize, size: usize) -> *mut c_void;
    fn iounmap(addr: *mut c_void);
    fn iowrite32(value: u32, addr: *mut c_void);
    fn iowrite8(value: u8, addr: *mut c_void);
    fn irqchip_init();
    fn shmobile_init_delay();
    fn shmobile_init_late();
}

/*
 * r8a7740 chip has lasting errata on MERAM buffer.
 * this is work-around for it.
 * see
 *     "Media RAM (MERAM)" on r8a7740 documentation
 */
const MEBUFCNTR: usize = 0xFE950098;

unsafe fn r8a7740_meram_workaround() {
    let reg = ioremap(MEBUFCNTR, 4);
    if !reg.is_null() {
        iowrite32(0x01600164, reg);
        iounmap(reg);
    }
}

unsafe fn r8a7740_init_irq_of() {
    let intc_prio_base = ioremap(0xe6900010, 0x10);
    let intc_msk_base = ioremap(0xe6900040, 0x10);
    let pfc_inta_ctrl = ioremap(0xe605807c, 0x4);

    irqchip_init();

    /* route signals to GIC */
    iowrite32(0x0, pfc_inta_ctrl);

    /*
     * To mask the shared interrupt to SPI 149 we must ensure to set
     * PRIO *and* MASK. Else we run into IRQ floods when registering
     * the intc_irqpin devices
     */
    iowrite32(0x0, intc_prio_base.add(0x0));
    iowrite32(0x0, intc_prio_base.add(0x4));
    iowrite32(0x0, intc_prio_base.add(0x8));
    iowrite32(0x0, intc_prio_base.add(0xc));
    iowrite8(0xff, intc_msk_base.add(0x0));
    iowrite8(0xff, intc_msk_base.add(0x4));
    iowrite8(0xff, intc_msk_base.add(0x8));
    iowrite8(0xff, intc_msk_base.add(0xc));

    iounmap(intc_prio_base);
    iounmap(intc_msk_base);
    iounmap(pfc_inta_ctrl);
}

unsafe fn r8a7740_generic_init() {
    r8a7740_meram_workaround();
}

static R8A7740_BOARDS_COMPAT_DT: &[Option<&str>] = &[
    Some("renesas,r8a7740"),
    None,
];

/* DT_MACHINE_START(R8A7740_DT, "Generic R8A7740 (Flattened Device Tree)") */
#[allow(dead_code)]
const R8A7740_DT: (&str, u32, usize, usize, usize, usize, usize, &'static [Option<&'static str>]) = (
    "Generic R8A7740 (Flattened Device Tree)",
    0,
    0,
    shmobile_init_delay as usize,
    r8a7740_init_irq_of as usize,
    r8a7740_generic_init as usize,
    shmobile_init_late as usize,
    R8A7740_BOARDS_COMPAT_DT,
);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
