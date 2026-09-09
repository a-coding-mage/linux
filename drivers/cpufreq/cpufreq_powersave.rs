// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/drivers/cpufreq/cpufreq_powersave.c
 *
 * Copyright (C) 2002 - 2003 Dominik Brodowski <linux@brodo.de>
 */

// #define pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// Dependencies supplied by the surrounding kernel translation.

unsafe fn cpufreq_gov_powersave_limits(policy: *mut cpufreq_policy) {
    pr_debug!("setting to %u kHz\n", (*policy).min);
    __cpufreq_driver_target(policy, (*policy).min, CPUFREQ_RELATION_L);
}

static mut cpufreq_gov_powersave: cpufreq_governor = cpufreq_governor {
    name: b"powersave\0".as_ptr() as *const _,
    limits: Some(cpufreq_gov_powersave_limits),
    owner: THIS_MODULE,
    flags: CPUFREQ_GOV_STRICT_TARGET,
};

MODULE_AUTHOR!("Dominik Brodowski <linux@brodo.de>");
MODULE_DESCRIPTION!("CPUfreq policy governor 'powersave'");
MODULE_LICENSE!("GPL");

// Preserve the build-time condition from CONFIG_CPU_FREQ_DEFAULT_GOV_POWERSAVE.
#[cfg(CONFIG_CPU_FREQ_DEFAULT_GOV_POWERSAVE)]
unsafe fn cpufreq_default_governor() -> *mut cpufreq_governor {
    &raw mut cpufreq_gov_powersave
}

cpufreq_governor_init!(cpufreq_gov_powersave);
cpufreq_governor_exit!(cpufreq_gov_powersave);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
