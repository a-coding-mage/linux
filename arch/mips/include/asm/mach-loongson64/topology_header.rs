/* SPDX-License-Identifier: GPL-2.0 */

/* Preserved from the C header: these declarations are enabled by CONFIG_NUMA. */
#[cfg(CONFIG_NUMA)]
pub unsafe fn cpu_to_node(cpu: ::core::ffi::c_int) -> ::core::ffi::c_int {
    cpu_logical_map(cpu) >> 2
}

#[cfg(CONFIG_NUMA)]
extern "C" {
    pub static mut __node_cpumask: [cpumask_t; 0];
    pub fn cpu_logical_map(cpu: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub static cpu_all_mask: *const cpumask_t;
    pub static cpu_online_mask: *const cpumask_t;
    pub fn pcibus_to_node(bus: *mut pci_bus) -> ::core::ffi::c_int;
    pub static mut __node_distances: [[::core::ffi::c_uchar; MAX_NUMNODES]; MAX_NUMNODES];
}

#[cfg(CONFIG_NUMA)]
#[repr(C)]
pub struct pci_bus {
    _private: [u8; 0],
}

#[cfg(CONFIG_NUMA)]
pub unsafe fn cpumask_of_node(node: ::core::ffi::c_int) -> *const cpumask_t {
    if node == NUMA_NO_NODE {
        cpu_all_mask
    } else {
        __node_cpumask.as_ptr().add(node as usize)
    }
}

#[cfg(CONFIG_NUMA)]
pub unsafe fn cpumask_of_pcibus(_bus: *mut pci_bus) -> *const cpumask_t {
    cpu_online_mask
}

#[cfg(CONFIG_NUMA)]
pub unsafe fn node_distance(
    from: ::core::ffi::c_int,
    to: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    __node_distances[from as usize][to as usize]
}

/* The C header includes <asm-generic/topology.h>; its dependency declarations
 * are supplied by the surrounding translation unit. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
