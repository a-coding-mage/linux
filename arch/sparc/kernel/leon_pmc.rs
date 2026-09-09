// SPDX-License-Identifier: GPL-2.0
/* leon_pmc.c: LEON Power-down cpu_idle() handler
 *
 * Copyright (C) 2011 Daniel Hellstrom (daniel@gaisler.com) Aeroflex Gaisler AB
 */

// C dependencies: <linux/init.h>, <linux/pm.h>, <asm/leon_amba.h>,
// <asm/cpu_type.h>, <asm/leon.h>, and <asm/processor.h>.

extern "C" {
    static mut amba_system_id: u32;
    static mut leon3_irqctrl_regs: *mut core::ffi::c_void;
    static mut sparc_cpu_model: i32;
    static sparc_leon: i32;
    static mut sparc_idle: Option<unsafe extern "C" fn()>;

    fn raw_local_irq_enable();
    fn raw_local_irq_disable();
    fn printk(format: *const core::ffi::c_char, ...);
}

/* List of Systems that need fixup instructions around power-down instruction */
static mut pmc_leon_fixup_ids: [u32; 4] = [
    AEROFLEX_UT699,
    GAISLER_GR712RC,
    LEON4_NEXTREME1,
    0,
];

unsafe fn pmc_leon_need_fixup() -> i32 {
    let systemid: u32 = amba_system_id >> 16;
    let mut id: *const u32 = pmc_leon_fixup_ids.as_ptr();

    while *id != 0 {
        if *id == systemid {
            return 1;
        }
        id = id.add(1);
    }

    0
}

/*
 * CPU idle callback function for systems that need some extra handling
 * See .../arch/sparc/kernel/process.c
 */
unsafe extern "C" fn pmc_leon_idle_fixup() {
    /* Prepare an address to a non-cachable region. APB is always
     * none-cachable. One instruction is executed after the Sleep
     * instruction, we make sure to read the bus and throw away the
     * value by accessing a non-cachable area, also we make sure the
     * MMU does not get a TLB miss here by using the MMU BYPASS ASI.
     */
    let address: u32 = leon3_irqctrl_regs as usize as u32;

    /* Interrupts need to be enabled to not hang the CPU */
    raw_local_irq_enable();

    core::arch::asm!(
        "wr %g0, %asr19",
        "lda [{address}] {asi}, %g0",
        address = in(reg) address,
        asi = const ASI_LEON_BYPASS,
        options(nostack, preserves_flags)
    );

    raw_local_irq_disable();
}

/*
 * CPU idle callback function
 * See .../arch/sparc/kernel/process.c
 */
unsafe extern "C" fn pmc_leon_idle() {
    /* Interrupts need to be enabled to not hang the CPU */
    raw_local_irq_enable();

    /* For systems without power-down, this will be no-op */
    core::arch::asm!("wr %g0, %asr19", options(nostack, preserves_flags));

    raw_local_irq_disable();
}

/* Install LEON Power Down function */
unsafe extern "C" fn leon_pmc_install() -> i32 {
    if sparc_cpu_model == sparc_leon {
        /* Assign power management IDLE handler */
        if pmc_leon_need_fixup() != 0 {
            sparc_idle = Some(pmc_leon_idle_fixup);
        } else {
            sparc_idle = Some(pmc_leon_idle);
        }

        // printk(KERN_INFO "leon: power management initialized\n");
        printk(core::ffi::c_str!("leon: power management initialized\n").as_ptr());
    }

    0
}

/* This driver is not critical to the boot process, don't care
 * if initialized late.
 */
// late_initcall(leon_pmc_install);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
