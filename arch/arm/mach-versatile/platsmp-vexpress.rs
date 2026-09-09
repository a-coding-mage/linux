// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Copyright (C) 2002 ARM Ltd.
 *  All Rights Reserved
 */

// Dependencies supplied by the surrounding kernel translation.

pub unsafe fn vexpress_smp_init_ops() -> bool {
    // CONFIG_MCPM
    #[cfg(CONFIG_MCPM)]
    {
        let mut cpu: ::core::ffi::c_int;
        let mut cpu_node: *mut device_node;
        let mut cci_node: *mut device_node;

        /*
         * The best way to detect a multi-cluster configuration
         * is to detect if the kernel can take over CCI ports
         * control. Loop over possible CPUs and check if CCI
         * port control is available.
         * Override the default vexpress_smp_ops if so.
         */
        for_each_possible_cpu!(cpu) {
            let available: bool;

            cpu_node = of_get_cpu_node(cpu, ::core::ptr::null_mut());
            if WARN!(!cpu_node.is_null(), "Missing cpu device node!") {
                return false;
            }

            cci_node = of_parse_phandle(cpu_node, "cci-control-port", 0);
            available = !cci_node.is_null() && of_device_is_available(cci_node);
            of_node_put(cci_node);
            of_node_put(cpu_node);

            if !available {
                return false;
            }
        }

        mcpm_smp_set_ops();
        return true;
    }

    // CONFIG_MCPM is not enabled.
    #[cfg(not(CONFIG_MCPM))]
    {
        false
    }
}

static vexpress_smp_dt_scu_match: [of_device_id; 3] = [
    of_device_id {
        compatible: "arm,cortex-a5-scu",
    },
    of_device_id {
        compatible: "arm,cortex-a9-scu",
    },
    of_device_id {
        ..unsafe { ::core::mem::zeroed() }
    },
];

unsafe fn vexpress_smp_dt_prepare_cpus(_max_cpus: u32) {
    let scu = of_find_matching_node(
        ::core::ptr::null_mut(),
        vexpress_smp_dt_scu_match.as_ptr(),
    );

    if !scu.is_null() {
        scu_enable(of_iomap(scu, 0));
    }

    /*
     * Write the address of secondary startup into the
     * system-wide flags register. The boot monitor waits
     * until it receives a soft interrupt, and then the
     * secondary CPU branches to this address.
     */
    vexpress_flags_set(__pa_symbol!(versatile_secondary_startup));
}

// CONFIG_HOTPLUG_CPU
#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe fn vexpress_cpu_die(cpu: u32) {
    versatile_immitation_cpu_die(cpu, 0x40);
}

static vexpress_smp_dt_ops: smp_operations = smp_operations {
    smp_prepare_cpus: Some(vexpress_smp_dt_prepare_cpus),
    smp_secondary_init: Some(versatile_secondary_init),
    smp_boot_secondary: Some(versatile_boot_secondary),
    #[cfg(CONFIG_HOTPLUG_CPU)]
    cpu_die: Some(vexpress_cpu_die),
    ..unsafe { ::core::mem::zeroed() }
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
