/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// asm/sn/types.h, asm/mmzone.h, and asm-generic/topology.h

#[repr(C)]
pub struct cpuinfo_ip27 {
    pub p_nasid: nasid_t,       /* my node ID in numa-as-id-space */
    pub p_speed: u16,           /* cpu speed in MHz */
    pub p_slice: u8,            /* Physical position on node board */
}

unsafe extern "C" {
    pub static mut sn_cpu_info: [cpuinfo_ip27; NR_CPUS];
}

#[macro_export]
macro_rules! cpu_to_node {
    ($cpu:expr) => {
        cputonasid($cpu)
    };
}

#[macro_export]
macro_rules! cpumask_of_node {
    ($node:expr) => {
        if ($node) == -1 {
            cpu_all_mask
        } else {
            unsafe { &(*hub_data($node)).h_cpus }
        }
    };
}

#[repr(C)]
pub struct pci_bus {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn pcibus_to_node(bus: *mut pci_bus) -> ::core::ffi::c_int;
    pub static mut __node_distances: [[u8; MAX_NUMNODES]; MAX_NUMNODES];
}

#[macro_export]
macro_rules! cpumask_of_pcibus {
    ($bus:expr) => {
        cpumask_of_node!(unsafe { pcibus_to_node($bus) })
    };
}

#[macro_export]
macro_rules! node_distance {
    ($from:expr, $to:expr) => {
        unsafe { __node_distances[($from)][($to)] }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
