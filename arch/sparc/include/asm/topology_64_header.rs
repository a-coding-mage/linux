/* SPDX-License-Identifier: GPL-2.0 */

/* The original header guard and include directives are omitted. */

/* CONFIG_NUMA */
#[cfg(CONFIG_NUMA)]
pub unsafe fn cpu_to_node(cpu: ::core::ffi::c_int) -> ::core::ffi::c_int {
    numa_cpu_lookup_table[cpu as usize]
}

#[cfg(CONFIG_NUMA)]
#[macro_export]
macro_rules! cpumask_of_node {
    ($node:expr) => {
        if ($node) == -1 {
            cpu_all_mask
        } else {
            &numa_cpumask_lookup_table[$node as usize]
        }
    };
}

#[repr(C)]
pub struct pci_bus {
    _private: [u8; 0],
}

#[cfg(all(CONFIG_NUMA, CONFIG_PCI))]
unsafe extern "C" {
    pub fn pcibus_to_node(pbus: *mut pci_bus) -> ::core::ffi::c_int;
}

#[cfg(all(CONFIG_NUMA, not(CONFIG_PCI)))]
pub unsafe fn pcibus_to_node(_pbus: *mut pci_bus) -> ::core::ffi::c_int {
    -1
}

#[cfg(CONFIG_NUMA)]
#[macro_export]
macro_rules! cpumask_of_pcibus {
    ($bus:expr) => {
        if unsafe { pcibus_to_node($bus) } == -1 {
            cpu_all_mask
        } else {
            cpumask_of_node!(unsafe { pcibus_to_node($bus) })
        }
    };
}

#[cfg(CONFIG_NUMA)]
unsafe extern "C" {
    pub fn __node_distance(
        a: ::core::ffi::c_int,
        b: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

#[cfg(CONFIG_NUMA)]
#[inline]
pub unsafe fn node_distance(
    a: ::core::ffi::c_int,
    b: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    __node_distance(a, b)
}

/* When CONFIG_NUMA is disabled, the declarations come from asm-generic/topology.h. */

/* CONFIG_SMP */
#[cfg(CONFIG_SMP)]
#[macro_export]
macro_rules! topology_physical_package_id {
    ($cpu:expr) => {
        cpu_data($cpu).proc_id
    };
}

#[cfg(CONFIG_SMP)]
#[macro_export]
macro_rules! topology_core_id {
    ($cpu:expr) => {
        cpu_data($cpu).core_id
    };
}

#[cfg(CONFIG_SMP)]
#[macro_export]
macro_rules! topology_core_cpumask {
    ($cpu:expr) => {
        &cpu_core_sib_map[$cpu as usize]
    };
}

#[cfg(CONFIG_SMP)]
#[macro_export]
macro_rules! topology_core_cache_cpumask {
    ($cpu:expr) => {
        &cpu_core_sib_cache_map[$cpu as usize]
    };
}

#[cfg(CONFIG_SMP)]
#[macro_export]
macro_rules! topology_sibling_cpumask {
    ($cpu:expr) => {
        &per_cpu!(cpu_sibling_map, $cpu)
    };
}

unsafe extern "C" {
    pub static mut cpu_core_map: [cpumask_t; NR_CPUS];
    pub static mut cpu_core_sib_map: [cpumask_t; NR_CPUS];
    pub static mut cpu_core_sib_cache_map: [cpumask_t; NR_CPUS];
}

/// Return cores that shares the last level cache.
#[inline]
pub unsafe fn cpu_coregroup_mask(cpu: ::core::ffi::c_int) -> *const cpumask {
    &cpu_core_sib_cache_map[cpu as usize] as *const cpumask_t as *const cpumask
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
