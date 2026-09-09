// SPDX-License-Identifier: GPL-2.0
/*
 * r8a7779 processor support
 *
 * Copyright (C) 2011, 2013  Renesas Solutions Corp.
 * Copyright (C) 2011  Magnus Damm
 * Copyright (C) 2013  Cogent Embedded, Inc.
 */

// Linux kernel and architecture headers provide these declarations.

const HPBREG_BASE: usize = 0xfe700000;

/* IRQ */
const INT2SMSKCR0: usize = 0x822a0; /* Interrupt Submask Clear Register 0 */
const INT2SMSKCR1: usize = 0x822a4; /* Interrupt Submask Clear Register 1 */
const INT2SMSKCR2: usize = 0x822a8; /* Interrupt Submask Clear Register 2 */
const INT2SMSKCR3: usize = 0x822ac; /* Interrupt Submask Clear Register 3 */
const INT2SMSKCR4: usize = 0x822b0; /* Interrupt Submask Clear Register 4 */

const INT2NTSR0: usize = 0x00060; /* Interrupt Notification Select Register 0 */
const INT2NTSR1: usize = 0x00064; /* Interrupt Notification Select Register 1 */

extern "C" {
    fn ioremap(phys_addr: usize, size: usize) -> *mut core::ffi::c_void;
    fn irqchip_init();
    fn writel(value: u32, address: *mut core::ffi::c_void);
    fn iounmap(address: *mut core::ffi::c_void);

    static r8a7779_smp_ops: core::ffi::c_void;
    fn shmobile_init_late();
}

unsafe fn r8a7779_init_irq_dt() {
    let base = ioremap(HPBREG_BASE, 0x00100000);

    irqchip_init();

    /* route all interrupts to ARM */
    writel(0xffffffff, base.add(INT2NTSR0));
    writel(0x3fffffff, base.add(INT2NTSR1));

    /* unmask all known interrupts in INTCS2 */
    writel(0xfffffff0, base.add(INT2SMSKCR0));
    writel(0xfff7ffff, base.add(INT2SMSKCR1));
    writel(0xfffbffdf, base.add(INT2SMSKCR2));
    writel(0xbffffffc, base.add(INT2SMSKCR3));
    writel(0x003fee3f, base.add(INT2SMSKCR4));

    iounmap(base);
}

static R8A7779_COMPAT_DT: &[Option<&str>] = &[
    Some("renesas,r8a7779"),
    None,
];

// Corresponds to DT_MACHINE_START(R8A7779_DT,
// "Generic R8A7779 (Flattened Device Tree)") ... MACHINE_END.
#[repr(C)]
pub struct MachineDesc {
    pub smp: *const core::ffi::c_void,
    pub init_irq: unsafe fn(),
    pub init_late: unsafe extern "C" fn(),
    pub dt_compat: &'static [Option<&'static str>],
}

#[no_mangle]
pub static R8A7779_DT: MachineDesc = MachineDesc {
    smp: unsafe { &r8a7779_smp_ops },
    init_irq: r8a7779_init_irq_dt,
    init_late: shmobile_init_late,
    dt_compat: R8A7779_COMPAT_DT,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
