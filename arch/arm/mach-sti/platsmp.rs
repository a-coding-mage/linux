// SPDX-License-Identifier: GPL-2.0-only
/*
 *  arch/arm/mach-sti/platsmp.c
 *
 * Copyright (C) 2013 STMicroelectronics (R&D) Limited.
 *\thttp://www.st.com
 *
 * Cloned from linux/arch/arm/mach-vexpress/platsmp.c
 *
 *  Copyright (C) 2002 ARM Ltd.
 *  All Rights Reserved
 */

// Linux and architecture headers provide the referenced types, functions, and
// constants in the surrounding translation unit.

static mut cpu_strt_ptr: *mut u32 = core::ptr::null_mut();

unsafe fn sti_boot_secondary(_cpu: u32, _idle: *mut task_struct) -> i32 {
    let entry_pa: usize = __pa_symbol(secondary_startup);

    /*
     * Secondary CPU is initialised and started by a U-BOOTROM firmware.
     * Secondary CPU is spinning and waiting for a write at cpu_strt_ptr.
     * Writing secondary_startup address at cpu_strt_ptr makes it to
     * jump directly to secondary_startup().
     */
    __raw_writel(entry_pa as u32, cpu_strt_ptr);

    /* wmb so that data is actually written before cache flush is done */
    smp_wmb();
    sync_cache_w(cpu_strt_ptr);

    0
}

unsafe fn sti_smp_prepare_cpus(max_cpus: u32) {
    let mut np: *mut device_node;
    let mut scu_base: *mut core::ffi::c_void;
    let mut release_phys: u32 = 0;
    let mut cpu: i32;

    np = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        c"arm,cortex-a9-scu".as_ptr(),
    );

    if !np.is_null() {
        scu_base = of_iomap(np, 0);
        scu_enable(scu_base);
        of_node_put(np);
    }

    if max_cpus <= 1 {
        return;
    }

    // for_each_possible_cpu(cpu)
    for_each_possible_cpu!(cpu) {
        np = of_get_cpu_node(cpu, core::ptr::null_mut());

        if np.is_null() {
            continue;
        }

        if of_property_read_u32(
            np,
            c"cpu-release-addr".as_ptr(),
            &mut release_phys,
        ) != 0 {
            pr_err!(
                "CPU {}: missing or invalid cpu-release-addr property\n",
                cpu
            );
            continue;
        }

        /*
         * cpu-release-addr is usually configured in SBC DMEM but can
         * also be in RAM.
         */

        if !memblock_is_memory(release_phys) {
            cpu_strt_ptr = ioremap(
                release_phys as usize,
                core::mem::size_of::<u32>(),
            ) as *mut u32;
        } else {
            cpu_strt_ptr = phys_to_virt(release_phys) as *mut u32;
        }

        set_cpu_possible(cpu, true);
    }
}

pub static sti_smp_ops: smp_operations = smp_operations {
    .smp_prepare_cpus: Some(sti_smp_prepare_cpus),
    .smp_boot_secondary: Some(sti_boot_secondary),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
