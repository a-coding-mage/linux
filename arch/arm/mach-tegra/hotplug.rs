// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Copyright (C) 2002 ARM Ltd.
 *  All Rights Reserved
 *  Copyright (c) 2010, 2012-2013, NVIDIA Corporation. All rights reserved.
 */

// Linux kernel dependencies supplied by other translation units.
extern "C" {
    fn cpu_logical_map(cpu: c_uint) -> c_uint;
    fn tegra_wait_cpu_in_reset(cpu: c_uint);
    fn tegra_disable_cpu_clock(cpu: c_uint);
    fn tegra_disable_clean_inv_dcache(level: c_int);
    fn tegra_get_chip_id() -> c_uint;
    fn soc_is_tegra() -> bool;
    fn tegra20_hotplug_shutdown();
    fn tegra30_hotplug_shutdown();
}

// Build-time configuration conditions are preserved from IS_ENABLED(...).
extern "C" {
    static CONFIG_HOTPLUG_CPU: bool;
    static CONFIG_ARCH_TEGRA_2x_SOC: bool;
    static CONFIG_ARCH_TEGRA_3x_SOC: bool;
    static CONFIG_ARCH_TEGRA_114_SOC: bool;
    static CONFIG_ARCH_TEGRA_124_SOC: bool;
}

type c_int = i32;
type c_uint = u32;

const TEGRA_FLUSH_CACHE_LOUIS: c_int = 0;
const TEGRA20: c_uint = 20;
const TEGRA30: c_uint = 30;
const TEGRA114: c_uint = 114;
const TEGRA124: c_uint = 124;

static mut tegra_hotplug_shutdown: Option<unsafe extern "C" fn()> = None;

pub unsafe extern "C" fn tegra_cpu_kill(mut cpu: c_uint) -> c_int {
    cpu = cpu_logical_map(cpu);

    /* Clock gate the CPU */
    tegra_wait_cpu_in_reset(cpu);
    tegra_disable_cpu_clock(cpu);

    1
}

/*
 * platform-specific code to shutdown a CPU
 *
 * Called with IRQs disabled
 */
pub unsafe extern "C" fn tegra_cpu_die(cpu: c_uint) {
    let _ = cpu;
    if tegra_hotplug_shutdown.is_none() {
        // WARN(1, "hotplug is not yet initialized\\n");
        return;
    }

    /* Clean L1 data cache */
    tegra_disable_clean_inv_dcache(TEGRA_FLUSH_CACHE_LOUIS);

    /* Shut down the current CPU. */
    (tegra_hotplug_shutdown.unwrap())();

    /* Should never return here. */
    // BUG();
    core::hint::unreachable_unchecked();
}

unsafe extern "C" fn tegra_hotplug_init() -> c_int {
    if !CONFIG_HOTPLUG_CPU {
        return 0;
    }

    if !soc_is_tegra() {
        return 0;
    }

    if CONFIG_ARCH_TEGRA_2x_SOC && tegra_get_chip_id() == TEGRA20 {
        tegra_hotplug_shutdown = Some(tegra20_hotplug_shutdown);
    }
    if CONFIG_ARCH_TEGRA_3x_SOC && tegra_get_chip_id() == TEGRA30 {
        tegra_hotplug_shutdown = Some(tegra30_hotplug_shutdown);
    }
    if CONFIG_ARCH_TEGRA_114_SOC && tegra_get_chip_id() == TEGRA114 {
        tegra_hotplug_shutdown = Some(tegra30_hotplug_shutdown);
    }
    if CONFIG_ARCH_TEGRA_124_SOC && tegra_get_chip_id() == TEGRA124 {
        tegra_hotplug_shutdown = Some(tegra30_hotplug_shutdown);
    }

    0
}

// pure_initcall(tegra_hotplug_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
