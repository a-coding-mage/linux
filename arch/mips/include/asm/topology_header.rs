/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2007 by Ralf Baechle
 */

// Dependency intent from <topology.h> and <linux/smp.h> is supplied externally.

// CONFIG_SMP conditional: these items are present only for SMP builds.
#[cfg(feature = "CONFIG_SMP")]
macro_rules! topology_physical_package_id {
    ($cpu:expr) => {
        cpu_data[$cpu].package
    };
}

#[cfg(feature = "CONFIG_SMP")]
macro_rules! topology_core_id {
    ($cpu:expr) => {
        cpu_core(&cpu_data[$cpu])
    };
}

#[cfg(feature = "CONFIG_SMP")]
macro_rules! topology_core_cpumask {
    ($cpu:expr) => {
        &cpu_core_map[$cpu]
    };
}

#[cfg(feature = "CONFIG_SMP")]
macro_rules! topology_sibling_cpumask {
    ($cpu:expr) => {
        &cpu_sibling_map[$cpu]
    };
}

#[cfg(feature = "CONFIG_SMP")]
extern "C" {
    pub static __cpu_primary_thread_mask: cpumask;
}

#[cfg(feature = "CONFIG_SMP")]
macro_rules! cpu_primary_thread_mask {
    () => {
        &__cpu_primary_thread_mask as *const cpumask
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
