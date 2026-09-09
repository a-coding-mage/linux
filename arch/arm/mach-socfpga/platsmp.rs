// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2010-2011 Calxeda, Inc.
 * Copyright 2012 Pavel Machek <pavel@denx.de>
 * Based on platsmp.c, Copyright (C) 2002 ARM Ltd.
 * Copyright (C) 2012 Altera Corporation
 */

// Linux and architecture dependencies supplied by the surrounding tree.

unsafe fn socfpga_boot_secondary(cpu: u32, idle: *mut task_struct) -> i32 {
    let trampoline_size = secondary_trampoline_end as usize
        - secondary_trampoline as usize;

    if socfpga_cpu1start_addr != 0 {
        // This will put CPU #1 into reset.
        writel(
            RSTMGR_MPUMODRST_CPU1,
            rst_manager_base_addr.add(SOCFPGA_RSTMGR_MODMPURST as usize),
        );

        memcpy(
            phys_to_virt(0),
            secondary_trampoline,
            trampoline_size,
        );

        writel(
            __pa_symbol(secondary_startup),
            sys_manager_base_addr.add((socfpga_cpu1start_addr & 0x000000ff) as usize),
        );

        flush_cache_all();
        smp_wmb();
        outer_clean_range(0, trampoline_size);

        // This will release CPU #1 out of reset.
        writel(
            0,
            rst_manager_base_addr.add(SOCFPGA_RSTMGR_MODMPURST as usize),
        );
    }

    0
}

unsafe fn socfpga_a10_boot_secondary(cpu: u32, idle: *mut task_struct) -> i32 {
    let trampoline_size = secondary_trampoline_end as usize
        - secondary_trampoline as usize;

    if socfpga_cpu1start_addr != 0 {
        writel(
            RSTMGR_MPUMODRST_CPU1,
            rst_manager_base_addr.add(SOCFPGA_A10_RSTMGR_MODMPURST as usize),
        );
        memcpy(
            phys_to_virt(0),
            secondary_trampoline,
            trampoline_size,
        );

        writel(
            __pa_symbol(secondary_startup),
            sys_manager_base_addr.add((socfpga_cpu1start_addr & 0x00000fff) as usize),
        );

        flush_cache_all();
        smp_wmb();
        outer_clean_range(0, trampoline_size);

        // This will release CPU #1 out of reset.
        writel(
            0,
            rst_manager_base_addr.add(SOCFPGA_A10_RSTMGR_MODMPURST as usize),
        );
    }

    0
}

unsafe fn socfpga_smp_prepare_cpus(max_cpus: u32) {
    let mut np: *mut device_node;
    let mut socfpga_scu_base_addr: *mut core::ffi::c_void;

    np = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        "arm,cortex-a9-scu\0".as_ptr() as *const i8,
    );
    if np.is_null() {
        pr_err!("{}: missing scu\n", "socfpga_smp_prepare_cpus");
        return;
    }

    socfpga_scu_base_addr = of_iomap(np, 0);
    of_node_put(np);
    if socfpga_scu_base_addr.is_null() {
        return;
    }
    scu_enable(socfpga_scu_base_addr);
}

// CONFIG_HOTPLUG_CPU conditionally includes the following platform handlers.
#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe fn socfpga_cpu_die(cpu: u32) {
    // Do WFI. If we wake up early, go back into WFI.
    loop {
        cpu_do_idle();
    }
}

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe fn socfpga_cpu_kill(cpu: u32) -> i32 {
    1
}

// The surrounding kernel provides struct smp_operations and its initconst semantics.
static socfpga_smp_ops: smp_operations = smp_operations {
    smp_prepare_cpus: Some(socfpga_smp_prepare_cpus),
    smp_boot_secondary: Some(socfpga_boot_secondary),
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    cpu_die: Some(socfpga_cpu_die),
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    cpu_kill: Some(socfpga_cpu_kill),
};

static socfpga_a10_smp_ops: smp_operations = smp_operations {
    smp_prepare_cpus: Some(socfpga_smp_prepare_cpus),
    smp_boot_secondary: Some(socfpga_a10_boot_secondary),
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    cpu_die: Some(socfpga_cpu_die),
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    cpu_kill: Some(socfpga_cpu_kill),
};

// CPU_METHOD_OF_DECLARE(socfpga_smp, "altr,socfpga-smp", &socfpga_smp_ops);
// CPU_METHOD_OF_DECLARE(socfpga_a10_smp, "altr,socfpga-a10-smp", &socfpga_a10_smp_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
