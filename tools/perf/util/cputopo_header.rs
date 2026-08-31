/* SPDX-License-Identifier: GPL-2.0 */

use std::ffi::c_char;

#[repr(C)]
pub struct cpu_topology {
    /* The number of unique package_cpus_lists below. */
    pub package_cpus_lists: u32,
    /* The number of unique die_cpu_lists below. */
    pub die_cpus_lists: u32,
    /* The number of unique core_cpu_lists below. */
    pub core_cpus_lists: u32,
    /*
     * An array of strings where each string is unique and read from
     * /sys/devices/system/cpu/cpuX/topology/package_cpus_list. From the ABI
     * each of these is a human-readable list of CPUs sharing the same
     * physical_package_id. The format is like 0-3, 8-11, 14,17.
     */
    pub package_cpus_list: *mut *const c_char,
    /*
     * An array of string where each string is unique and from
     * /sys/devices/system/cpu/cpuX/topology/die_cpus_list. From the ABI
     * each of these is a human-readable list of CPUs within the same die.
     * The format is like 0-3, 8-11, 14,17.
     */
    pub die_cpus_list: *mut *const c_char,
    /*
     * An array of string where each string is unique and from
     * /sys/devices/system/cpu/cpuX/topology/core_cpus_list. From the ABI
     * each of these is a human-readable list of CPUs within the same
     * core. The format is like 0-3, 8-11, 14,17.
     */
    pub core_cpus_list: *mut *const c_char,
}

#[repr(C)]
pub struct numa_topology_node {
    pub cpus: *mut c_char,
    pub node: u32,
    pub mem_total: u64,
    pub mem_free: u64,
}

#[repr(C)]
pub struct numa_topology {
    pub nr: u32,
    pub nodes: [numa_topology_node; 0],
}

#[repr(C)]
pub struct hybrid_topology_node {
    pub pmu_name: *mut c_char,
    pub cpus: *mut c_char,
}

#[repr(C)]
pub struct hybrid_topology {
    pub nr: u32,
    pub nodes: [hybrid_topology_node; 0],
}

unsafe extern "C" {
    /*
     * The topology for online CPUs, lazily created.
     */
    pub fn online_topology() -> *const cpu_topology;

    pub fn cpu_topology__new() -> *mut cpu_topology;
    pub fn cpu_topology__delete(tp: *mut cpu_topology);
    /* Determine from the core list whether SMT was enabled. */
    pub fn cpu_topology__smt_on(topology: *const cpu_topology) -> bool;
    /* Are the sets of SMT siblings all enabled or all disabled in user_requested_cpus. */
    pub fn cpu_topology__core_wide(
        topology: *const cpu_topology,
        user_requested_cpu_list: *const c_char,
    ) -> bool;

    pub fn numa_topology__new() -> *mut numa_topology;
    pub fn numa_topology__delete(tp: *mut numa_topology);

    pub fn hybrid_topology__new() -> *mut hybrid_topology;
    pub fn hybrid_topology__delete(tp: *mut hybrid_topology);
}
