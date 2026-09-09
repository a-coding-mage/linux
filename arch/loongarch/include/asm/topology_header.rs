/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

/* C header guard: __ASM_TOPOLOGY_H */
/* Dependency: linux/smp.h */

#[cfg(CONFIG_NUMA)]
/* Dependency: asm/numa.h */
extern "C" {
    pub static mut cpus_on_node: [cpumask_t; 0];
}

#[cfg(CONFIG_NUMA)]
#[macro_export]
macro_rules! cpumask_of_node {
    ($node:expr) => {
        if ($node) == NUMA_NO_NODE {
            cpu_all_mask
        } else {
            unsafe { &cpus_on_node[$node as usize] }
        }
    };
}

#[cfg(CONFIG_NUMA)]
#[repr(C)]
pub struct pci_bus {
    _private: [u8; 0],
}

#[cfg(CONFIG_NUMA)]
extern "C" {
    pub fn pcibus_to_node(bus: *mut pci_bus) -> ::core::ffi::c_int;
}

#[cfg(CONFIG_NUMA)]
#[macro_export]
macro_rules! cpumask_of_pcibus {
    ($bus:expr) => {
        cpu_online_mask
    };
}

#[cfg(CONFIG_NUMA)]
extern "C" {
    pub fn __node_distance(from: ::core::ffi::c_int, to: ::core::ffi::c_int)
        -> ::core::ffi::c_int;
}

#[cfg(CONFIG_NUMA)]
#[macro_export]
macro_rules! node_distance {
    ($from:expr, $to:expr) => {
        unsafe { __node_distance($from, $to) }
    };
}

#[cfg(not(CONFIG_NUMA))]
#[macro_export]
macro_rules! pcibus_to_node {
    ($bus:expr) => {
        0
    };
}

#[cfg(CONFIG_SMP)]
/*
 * Return cpus that shares the last level cache.
 */
#[inline]
pub unsafe fn cpu_coregroup_mask(cpu: usize) -> *const cpumask {
    &cpu_llc_shared_map[cpu] as *const cpumask
}

#[cfg(CONFIG_SMP)]
#[macro_export]
macro_rules! topology_physical_package_id {
    ($cpu:expr) => {
        cpu_data[$cpu].package
    };
}

#[cfg(CONFIG_SMP)]
#[macro_export]
macro_rules! topology_core_id {
    ($cpu:expr) => {
        cpu_data[$cpu].core
    };
}

#[cfg(CONFIG_SMP)]
#[macro_export]
macro_rules! topology_core_cpumask {
    ($cpu:expr) => {
        &cpu_core_map[$cpu]
    };
}

#[cfg(CONFIG_SMP)]
#[macro_export]
macro_rules! topology_sibling_cpumask {
    ($cpu:expr) => {
        &cpu_sibling_map[$cpu]
    };
}

/* Dependency: asm-generic/topology.h */

#[inline]
pub fn arch_fix_phys_package_id(_num: ::core::ffi::c_int, _slot: u32) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
