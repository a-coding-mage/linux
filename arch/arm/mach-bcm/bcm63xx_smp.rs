// SPDX-License-Identifier: GPL-2.0-only
/*
 * Broadcom BCM63138 DSL SoCs SMP support code
 *
 * Copyright (C) 2015, Broadcom Corporation
 */

// Linux dependencies supplied by the surrounding kernel translation.

/* Size of mapped Cortex A9 SCU address space */
const CORTEX_A9_SCU_SIZE: usize = 0x58;

/*
 * Enable the Cortex A9 Snoop Control Unit
 *
 * By the time this is called we already know there are multiple
 * cores present.  We assume we're running on a Cortex A9 processor,
 * so any trouble getting the base address register or getting the
 * SCU base is a problem.
 *
 * Return 0 if successful or an error code otherwise.
 */
unsafe fn scu_a9_enable() -> i32 {
    let mut config_base: usize;
    let scu_base: *mut core::ffi::c_void;
    let mut i: u32;
    let mut ncores: u32;

    if !scu_a9_has_base() {
        pr_err!("no configuration base address register!\n");
        return -ENXIO;
    }

    /* Config base address register value is zero for uniprocessor */
    config_base = scu_a9_get_base();
    if config_base == 0 {
        pr_err!("hardware reports only one core\n");
        return -ENOENT;
    }

    scu_base = ioremap(config_base as phys_addr_t, CORTEX_A9_SCU_SIZE);
    if scu_base.is_null() {
        pr_err!("failed to remap config base ({}/{}) for SCU\n", config_base, CORTEX_A9_SCU_SIZE);
        return -ENOMEM;
    }

    scu_enable(scu_base);

    ncores = if !scu_base.is_null() { scu_get_core_count(scu_base) } else { 1 };

    if ncores > nr_cpu_ids {
        pr_warn!("SMP: {} cores greater than maximum ({}), clipping\n", ncores, nr_cpu_ids);
        ncores = nr_cpu_ids;
    }

    /* The BCM63138 SoC has two Cortex-A9 CPUs, CPU0 features a complete
     * and fully functional VFP unit that can be used, but CPU1 does not.
     * Since we will not be able to trap kernel-mode NEON to force
     * migration to CPU0, just do not advertise VFP support at all.
     *
     * This will make vfp_init bail out and do not attempt to use VFP at
     * all, for kernel-mode NEON, we do not want to introduce any
     * conditionals in hot-paths, so we just restrict the system to UP.
     */
    // CONFIG_VFP is a build-time condition from the original source.
    #[cfg(CONFIG_VFP)]
    if ncores > 1 {
        pr_warn!("SMP: secondary CPUs lack VFP unit, disabling VFP\n");
        vfp_disable();

        // CONFIG_KERNEL_MODE_NEON is a build-time condition from the original source.
        #[cfg(CONFIG_KERNEL_MODE_NEON)]
        {
            WARN!(true, "SMP: kernel-mode NEON enabled, restricting to UP\n");
            ncores = 1;
        }
    }

    i = 0;
    while i < ncores {
        set_cpu_possible(i, true);
        i += 1;
    }

    iounmap(scu_base); /* That's the last we'll need of this */

    0
}

static BCM63138_BOOTLUT_IDS: [of_device_id; 2] = [
    of_device_id { compatible: "brcm,bcm63138-bootlut", },
    of_device_id { /* sentinel */ },
];

const BOOTLUT_RESET_VECT: usize = 0x20;

unsafe fn bcm63138_smp_boot_secondary(cpu: u32, _idle: *mut task_struct) -> i32 {
    let bootlut_base: *mut core::ffi::c_void;
    let mut dn: *mut device_node;
    let mut ret: i32 = 0;
    let val: u32;

    dn = of_find_matching_node(core::ptr::null_mut(), BCM63138_BOOTLUT_IDS.as_ptr());
    if dn.is_null() {
        pr_err!("SMP: unable to find bcm63138 boot LUT node\n");
        return -ENODEV;
    }

    bootlut_base = of_iomap(dn, 0);
    of_node_put(dn);

    if bootlut_base.is_null() {
        pr_err!("SMP: unable to remap boot LUT base register\n");
        return -ENOMEM;
    }

    /* Locate the secondary CPU node */
    dn = of_get_cpu_node(cpu, core::ptr::null_mut());
    if dn.is_null() {
        pr_err!("SMP: failed to locate secondary CPU{} node\n", cpu);
        ret = -ENODEV;
        iounmap(bootlut_base);
        return ret;
    }

    /* Write the secondary init routine to the BootLUT reset vector */
    val = __pa_symbol(secondary_startup);
    writel_relaxed(val, (bootlut_base as *mut u8).add(BOOTLUT_RESET_VECT) as *mut core::ffi::c_void);

    /* Power up the core, will jump straight to its reset vector when we
     * return
     */
    ret = bcm63xx_pmb_power_on_cpu(dn);
    of_node_put(dn);

    iounmap(bootlut_base);
    ret
}

unsafe fn bcm63138_smp_prepare_cpus(_max_cpus: u32) {
    let ret = scu_a9_enable();
    if ret != 0 {
        pr_warn!("SMP: Cortex-A9 SCU setup failed\n");
        return;
    }
}

static BCM63138_SMP_OPS: smp_operations = smp_operations {
    smp_prepare_cpus: Some(bcm63138_smp_prepare_cpus),
    smp_boot_secondary: Some(bcm63138_smp_boot_secondary),
};

CPU_METHOD_OF_DECLARE!(bcm63138_smp, "brcm,bcm63138", &BCM63138_SMP_OPS);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
