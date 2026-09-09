// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2002 ARM Ltd.
 * Copyright (C) 2008 STMicroelctronics.
 * Copyright (C) 2009 ST-Ericsson.
 * Author: Srinidhi Kasagar <srinidhi.kasagar@stericsson.com>
 *
 * This file is based on arm realview platform
 */

// Dependencies are supplied by the surrounding kernel translation unit.

/* Magic triggers in backup RAM */
const UX500_CPU1_JUMPADDR_OFFSET: usize = 0x1FF4;
const UX500_CPU1_WAKEMAGIC_OFFSET: usize = 0x1FF0;

static mut backupram: *mut core::ffi::c_void = core::ptr::null_mut();

unsafe fn ux500_smp_prepare_cpus(max_cpus: u32) {
    let mut np: *mut device_node;
    static mut scu_base: *mut core::ffi::c_void = core::ptr::null_mut();
    let ncores: u32;
    let mut i: i32;

    np = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        c"ste,dbx500-backupram".as_ptr(),
    );
    if np.is_null() {
        pr_err(c"No backupram base address\n".as_ptr());
        return;
    }
    backupram = of_iomap(np, 0);
    of_node_put(np);
    if backupram.is_null() {
        pr_err(c"No backupram remap\n".as_ptr());
        return;
    }

    np = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        c"arm,cortex-a9-scu".as_ptr(),
    );
    if np.is_null() {
        pr_err(c"No SCU base address\n".as_ptr());
        return;
    }
    scu_base = of_iomap(np, 0);
    of_node_put(np);
    if scu_base.is_null() {
        pr_err(c"No SCU remap\n".as_ptr());
        return;
    }

    scu_enable(scu_base);
    ncores = scu_get_core_count(scu_base);
    i = 0;
    while i < ncores as i32 {
        set_cpu_possible(i as u32, true);
        i += 1;
    }
    iounmap(scu_base);
}

unsafe fn ux500_boot_secondary(cpu: u32, idle: *mut task_struct) -> i32 {
    /*
     * write the address of secondary startup into the backup ram register
     * at offset 0x1FF4, then write the magic number 0xA1FEED01 to the
     * backup ram register at offset 0x1FF0, which is what boot rom code
     * is waiting for. This will wake up the secondary core from WFE.
     */
    writel(
        __pa_symbol(secondary_startup),
        (backupram as *mut u8).add(UX500_CPU1_JUMPADDR_OFFSET) as *mut core::ffi::c_void,
    );
    writel(
        0xA1FEED01,
        (backupram as *mut u8).add(UX500_CPU1_WAKEMAGIC_OFFSET) as *mut core::ffi::c_void,
    );

    /* make sure write buffer is drained */
    mb();
    arch_send_wakeup_ipi_mask(cpumask_of(cpu));
    0
}

#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe fn ux500_cpu_die(cpu: u32) {
    wfi();
}

#[repr(C)]
struct smp_operations {
    smp_prepare_cpus: Option<unsafe fn(u32)>,
    smp_boot_secondary: Option<unsafe fn(u32, *mut task_struct) -> i32>,
    #[cfg(CONFIG_HOTPLUG_CPU)]
    cpu_die: Option<unsafe fn(u32)>,
}

static ux500_smp_ops: smp_operations = smp_operations {
    smp_prepare_cpus: Some(ux500_smp_prepare_cpus),
    smp_boot_secondary: Some(ux500_boot_secondary),
    #[cfg(CONFIG_HOTPLUG_CPU)]
    cpu_die: Some(ux500_cpu_die),
};

// CPU_METHOD_OF_DECLARE(ux500_smp, "ste,dbx500-smp", &ux500_smp_ops);
cpu_method_of_declare!(ux500_smp, "ste,dbx500-smp", &ux500_smp_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
