/*
 * Written by: Matthew Dobson, IBM Corporation
 *
 * Copyright (C) 2002, IBM Corp.
 *
 * All rights reserved.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
 * or FITNESS FOR A PARTICULAR PURPOSE, GOOD TITLE or NON INFRINGEMENT.
 */

// Dependencies supplied by the surrounding kernel translation.

#[cfg(CONFIG_NUMA)]
extern "C" {
    pub static mut x86_cpu_to_node_map: core::ffi::c_int;
    pub static mut node_to_cpumask_map: [*mut core::ffi::c_void; MAX_NUMNODES];
    pub fn __cpu_to_node(cpu: core::ffi::c_int) -> core::ffi::c_int;
    pub fn early_cpu_to_node(cpu: core::ffi::c_int) -> core::ffi::c_int;
    pub fn cpumask_of_node(node: core::ffi::c_int) -> *const core::ffi::c_void;
    pub fn setup_node_to_cpumask_map();
    pub fn __pcibus_to_node(bus: *mut core::ffi::c_void) -> core::ffi::c_int;
    pub fn __node_distance(a: core::ffi::c_int, b: core::ffi::c_int) -> core::ffi::c_int;
}

#[cfg(not(CONFIG_NUMA))]
#[inline]
pub fn numa_node_id() -> core::ffi::c_int { 0 }

#[cfg(not(CONFIG_NUMA))]
#[inline]
pub fn early_cpu_to_node(_cpu: core::ffi::c_int) -> core::ffi::c_int { 0 }

#[cfg(not(CONFIG_NUMA))]
#[inline]
pub fn setup_node_to_cpumask_map() {}

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum x86_topology_domains {
    TOPO_SMT_DOMAIN,
    TOPO_CORE_DOMAIN,
    TOPO_MODULE_DOMAIN,
    TOPO_TILE_DOMAIN,
    TOPO_DIE_DOMAIN,
    TOPO_DIEGRP_DOMAIN,
    TOPO_PKG_DOMAIN,
    TOPO_MAX_DOMAIN,
}

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum amd_cpu_type {
    AMD_CPU_TYPE_PERFORMANCE = 0,
    AMD_CPU_TYPE_EFFICIENCY = 1,
    AMD_CPU_TYPE_LOW_POWER = 2,
}

#[repr(C)]
pub struct x86_topology_system {
    pub dom_shifts: [core::ffi::c_uint; x86_topology_domains::TOPO_MAX_DOMAIN as usize],
    pub dom_size: [core::ffi::c_uint; x86_topology_domains::TOPO_MAX_DOMAIN as usize],
}

extern "C" {
    pub static mut x86_topo_system: x86_topology_system;
    pub static mut __max_dies_per_package: core::ffi::c_uint;
    pub static mut __max_logical_packages: core::ffi::c_uint;
    pub static mut __max_threads_per_core: core::ffi::c_uint;
    pub static mut __num_threads_per_package: core::ffi::c_uint;
    pub static mut __num_cores_per_package: core::ffi::c_uint;
    pub static mut __num_nodes_per_package: core::ffi::c_uint;
    pub fn get_topology_cpu_type_name(c: *mut cpuinfo_x86) -> *const core::ffi::c_char;
    pub fn cpu_coregroup_mask(cpu: core::ffi::c_int) -> *const core::ffi::c_void;
    pub fn cpu_clustergroup_mask(cpu: core::ffi::c_int) -> *const core::ffi::c_void;
}

pub struct cpuinfo_x86;

#[inline]
pub unsafe fn topology_get_domain_size(dom: x86_topology_domains) -> core::ffi::c_uint {
    x86_topo_system.dom_size[dom as usize]
}

#[inline]
pub unsafe fn topology_get_domain_shift(dom: x86_topology_domains) -> core::ffi::c_uint {
    if matches!(dom, x86_topology_domains::TOPO_SMT_DOMAIN) { 0 } else {
        x86_topo_system.dom_shifts[(dom as usize) - 1]
    }
}

#[macro_export]
macro_rules! topology_logical_package_id { ($cpu:expr) => { cpu_data($cpu).topo.logical_pkg_id }; }
#[macro_export]
macro_rules! topology_physical_package_id { ($cpu:expr) => { cpu_data($cpu).topo.pkg_id }; }
#[macro_export]
macro_rules! topology_logical_die_id { ($cpu:expr) => { cpu_data($cpu).topo.logical_die_id }; }
#[macro_export]
macro_rules! topology_logical_core_id { ($cpu:expr) => { cpu_data($cpu).topo.logical_core_id }; }
#[macro_export]
macro_rules! topology_die_id { ($cpu:expr) => { cpu_data($cpu).topo.die_id }; }
#[macro_export]
macro_rules! topology_core_id { ($cpu:expr) => { cpu_data($cpu).topo.core_id }; }
#[macro_export]
macro_rules! topology_ppin { ($cpu:expr) => { cpu_data($cpu).ppin }; }
#[macro_export]
macro_rules! topology_amd_node_id { ($cpu:expr) => { cpu_data($cpu).topo.amd_node_id }; }

#[inline] pub unsafe fn topology_max_packages() -> core::ffi::c_uint { __max_logical_packages }
#[inline] pub unsafe fn topology_max_dies_per_package() -> core::ffi::c_uint { __max_dies_per_package }
#[inline] pub unsafe fn topology_num_cores_per_package() -> core::ffi::c_uint { __num_cores_per_package }
#[inline] pub unsafe fn topology_num_threads_per_package() -> core::ffi::c_uint { __num_threads_per_package }
#[inline] pub unsafe fn topology_num_nodes_per_package() -> core::ffi::c_uint { __num_nodes_per_package }

#[cfg(CONFIG_X86_LOCAL_APIC)]
extern "C" { pub fn topology_get_logical_id(apicid: u32, at_level: x86_topology_domains) -> core::ffi::c_int; }
#[cfg(not(CONFIG_X86_LOCAL_APIC))]
#[inline] pub fn topology_get_logical_id(_apicid: u32, _at_level: x86_topology_domains) -> core::ffi::c_int { 0 }

#[cfg(CONFIG_SMP)]
extern "C" {
    pub static mut __max_smt_threads: core::ffi::c_int;
    pub static mut __amd_nodes_per_pkg: core::ffi::c_uint;
}
#[cfg(CONFIG_SMP)]
#[inline] pub unsafe fn topology_max_smt_threads() -> core::ffi::c_int { __max_smt_threads }
#[cfg(CONFIG_SMP)]
#[inline] pub unsafe fn topology_amd_nodes_per_pkg() -> core::ffi::c_uint { __amd_nodes_per_pkg }
#[cfg(not(CONFIG_SMP))]
#[inline] pub fn topology_max_smt_threads() -> core::ffi::c_int { 1 }
#[cfg(not(CONFIG_SMP))]
#[inline] pub fn topology_amd_nodes_per_pkg() -> core::ffi::c_uint { 1 }

extern "C" {
    pub static mut __cpu_primary_thread_mask: core::ffi::c_void;
    pub fn cpumask_test_cpu(cpu: core::ffi::c_uint, mask: *const core::ffi::c_void) -> bool;
    pub fn topology_get_primary_thread(cpu: core::ffi::c_uint) -> core::ffi::c_int;
    pub fn cpu_online(cpu: core::ffi::c_int) -> bool;
    pub fn x86_pci_root_bus_node(bus: core::ffi::c_int) -> core::ffi::c_int;
    pub fn x86_pci_root_bus_resources(bus: core::ffi::c_int, resources: *mut core::ffi::c_void);
    pub static mut x86_topology_update: bool;
    pub fn arch_scale_freq_tick();
    pub fn arch_sched_node_distance(from: core::ffi::c_int, to: core::ffi::c_int) -> core::ffi::c_int;
}

#[inline]
pub unsafe fn topology_is_primary_thread(cpu: core::ffi::c_uint) -> bool {
    cpumask_test_cpu(cpu, &__cpu_primary_thread_mask as *const _ as *const core::ffi::c_void)
}

#[inline]
pub unsafe fn topology_is_core_online(cpu: core::ffi::c_uint) -> bool {
    let pcpu = topology_get_primary_thread(cpu);
    if pcpu >= 0 { cpu_online(pcpu) } else { false }
}

#[inline] pub fn arch_fix_phys_package_id(_num: core::ffi::c_int, _slot: u32) {}

#[cfg(CONFIG_SCHED_MC_PRIO)]
extern "C" {
    pub static mut sysctl_sched_itmt_enabled: bool;
    pub fn sched_set_itmt_core_prio(prio: core::ffi::c_int, core_cpu: core::ffi::c_int);
    pub fn sched_set_itmt_support() -> core::ffi::c_int;
    pub fn sched_clear_itmt_support();
}
#[cfg(not(CONFIG_SCHED_MC_PRIO))]
pub const sysctl_sched_itmt_enabled: bool = false;
#[cfg(not(CONFIG_SCHED_MC_PRIO))]
#[inline] pub fn sched_set_itmt_core_prio(_prio: core::ffi::c_int, _core_cpu: core::ffi::c_int) {}
#[cfg(not(CONFIG_SCHED_MC_PRIO))]
#[inline] pub fn sched_set_itmt_support() -> core::ffi::c_int { 0 }
#[cfg(not(CONFIG_SCHED_MC_PRIO))]
#[inline] pub fn sched_clear_itmt_support() {}

#[cfg(all(CONFIG_SMP, CONFIG_X86_64))]
extern "C" {
    pub static mut arch_freq_scale: usize;
    pub fn arch_enable_hybrid_capacity_scale() -> bool;
    pub fn arch_set_cpu_capacity(cpu: core::ffi::c_int, cap: usize, max_cap: usize, cap_freq: usize, base_freq: usize);
    pub fn arch_scale_cpu_capacity(cpu: core::ffi::c_int) -> usize;
    pub fn arch_set_max_freq_ratio(turbo_disabled: bool);
    pub fn freq_invariance_set_perf_ratio(ratio: u64, turbo_disabled: bool);
}
#[cfg(not(all(CONFIG_SMP, CONFIG_X86_64)))]
#[inline] pub fn arch_enable_hybrid_capacity_scale() -> bool { false }
#[cfg(not(all(CONFIG_SMP, CONFIG_X86_64)))]
#[inline] pub fn arch_set_cpu_capacity(_cpu: core::ffi::c_int, _cap: usize, _max_cap: usize, _cap_freq: usize, _base_freq: usize) {}
#[cfg(not(all(CONFIG_SMP, CONFIG_X86_64)))]
#[inline] pub fn arch_set_max_freq_ratio(_turbo_disabled: bool) {}
#[cfg(not(all(CONFIG_SMP, CONFIG_X86_64)))]
#[inline] pub fn freq_invariance_set_perf_ratio(_ratio: u64, _turbo_disabled: bool) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
