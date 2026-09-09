// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2015 Linus Walleij
 */

// Dependencies supplied by the corresponding kernel headers are intentionally
// left as external names.

const REALVIEW_SYS_FLAGSSET_OFFSET: u32 = 0x30;

static REALVIEW_SCU_MATCH: [of_device_id; 4] = [
    /*
     * The ARM11MP SCU compatible is only provided as fallback for
     * old RealView EB Cortex-A9 device trees that were using this
     * compatible by mistake.
     */
    of_device_id { compatible: b"arm,arm11mp-scu\0".as_ptr() as *const i8 },
    of_device_id { compatible: b"arm,cortex-a9-scu\0".as_ptr() as *const i8 },
    of_device_id { compatible: b"arm,cortex-a5-scu\0".as_ptr() as *const i8 },
    of_device_id { compatible: core::ptr::null() },
];

static REALVIEW_SYSCON_MATCH: [of_device_id; 4] = [
    of_device_id { compatible: b"arm,core-module-integrator\0".as_ptr() as *const i8 },
    of_device_id { compatible: b"arm,realview-eb-syscon\0".as_ptr() as *const i8 },
    of_device_id { compatible: b"arm,realview-pbx-syscon\0".as_ptr() as *const i8 },
    of_device_id { compatible: core::ptr::null() },
];

unsafe fn realview_smp_prepare_cpus(max_cpus: u32) {
    let mut np: *mut device_node;
    let mut scu_base: *mut core::ffi::c_void;
    let mut map: *mut regmap;
    let ncores: u32;
    let mut i: i32;

    np = of_find_matching_node(core::ptr::null_mut(), REALVIEW_SCU_MATCH.as_ptr());
    if np.is_null() {
        pr_err!("PLATSMP: No SCU base address");
        return;
    }
    scu_base = of_iomap(np, 0);
    of_node_put(np);
    if scu_base.is_null() {
        pr_err!("PLATSMP: No SCU remap");
        return;
    }

    scu_enable(scu_base);
    ncores = scu_get_core_count(scu_base);
    pr_info!("SCU: %d cores detected\\n", ncores);
    i = 0;
    while i < ncores as i32 {
        set_cpu_possible(i as u32, true);
        i += 1;
    }
    iounmap(scu_base);

    /* The syscon contains the magic SMP start address registers */
    np = of_find_matching_node(core::ptr::null_mut(), REALVIEW_SYSCON_MATCH.as_ptr());
    if np.is_null() {
        pr_err!("PLATSMP: No syscon match");
        return;
    }
    map = syscon_node_to_regmap(np);
    of_node_put(np);
    if IS_ERR(map) {
        pr_err!("PLATSMP: No syscon regmap");
        return;
    }
    /* Put the boot address in this magic register */
    regmap_write(map, REALVIEW_SYS_FLAGSSET_OFFSET, __pa_symbol(versatile_secondary_startup));
}

// #ifdef CONFIG_HOTPLUG_CPU
unsafe fn realview_cpu_die(cpu: u32) {
    return versatile_immitation_cpu_die(cpu, 0x20);
}
// #endif

static REALVIEW_DT_SMP_OPS: smp_operations = smp_operations {
    smp_prepare_cpus: Some(realview_smp_prepare_cpus),
    smp_secondary_init: Some(versatile_secondary_init),
    smp_boot_secondary: Some(versatile_boot_secondary),
    // #ifdef CONFIG_HOTPLUG_CPU
    cpu_die: Some(realview_cpu_die),
    // #endif
};

CPU_METHOD_OF_DECLARE!(realview_smp, b"arm,realview-smp\0", &REALVIEW_DT_SMP_OPS);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
