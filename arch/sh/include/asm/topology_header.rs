/* SPDX-License-Identifier: GPL-2.0 */

/* CONFIG_NUMA conditional declarations and macros. */
#[cfg(feature = "CONFIG_NUMA")]
macro_rules! cpu_to_node {
    ($cpu:expr) => {{
        let _ = $cpu;
        0
    }};
}

#[cfg(feature = "CONFIG_NUMA")]
macro_rules! cpumask_of_node {
    ($node:expr) => {{
        let _ = $node;
        cpu_online_mask
    }};
}

#[cfg(feature = "CONFIG_NUMA")]
macro_rules! pcibus_to_node {
    ($bus:expr) => {{
        let _ = $bus;
        -1
    }};
}

#[cfg(feature = "CONFIG_NUMA")]
macro_rules! cpumask_of_pcibus {
    ($bus:expr) => {{
        if pcibus_to_node!($bus) == -1 {
            cpu_all_mask
        } else {
            cpumask_of_node!(pcibus_to_node!($bus))
        }
    }};
}

macro_rules! mc_capable {
    () => { 1 };
}

pub unsafe extern "C" {
    pub fn cpu_coregroup_mask(cpu: core::ffi::c_int) -> *const cpumask;
    pub static mut cpu_core_map: [cpumask_t; NR_CPUS];
}

macro_rules! topology_core_cpumask {
    ($cpu:expr) => {
        unsafe { &cpu_core_map[$cpu] as *const cpumask_t }
    };
}

/* Declarations supplied by <asm-generic/topology.h> remain external dependencies. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
