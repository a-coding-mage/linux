/* SPDX-License-Identifier: GPL-2.0 */

// C header guard and __KERNEL__ conditional omitted; this file is the kernel-side translation.

use core::ffi::c_int;

pub struct device;
pub struct device_node;
pub struct drmem_lmb;
pub struct pci_bus;
pub struct cpumask;

// CONFIG_NUMA-dependent declarations.  The original preprocessor condition is preserved here.
pub const RECLAIM_DISTANCE: c_int = 10;

pub unsafe fn cpumask_of_node(node: c_int) -> *mut cpumask {
    if node == -1 { cpu_all_mask } else { node_to_cpumask_map.add(node as usize) }
}

// CONFIG_PCI selects the external implementation; otherwise the inline implementation returns -1.
#[inline]
pub unsafe fn pcibus_to_node(_bus: *mut pci_bus) -> c_int { -1 }

#[inline]
pub unsafe fn cpumask_of_pcibus(bus: *mut pci_bus) -> *mut cpumask {
    let node = pcibus_to_node(bus);
    if node == -1 { cpu_all_mask } else { cpumask_of_node(node) }
}

extern "C" {
    pub fn cpu_relative_distance(cpu1_assoc: *mut u32, cpu2_assoc: *mut u32) -> c_int;
    pub fn __node_distance(a: c_int, b: c_int) -> c_int;
    pub fn dump_numa_cpu_topology();
    pub fn sysfs_add_device_to_node(dev: *mut device, nid: c_int) -> c_int;
    pub fn sysfs_remove_device_from_node(dev: *mut device, nid: c_int);
    pub fn of_drconf_to_nid_single(lmb: *mut drmem_lmb) -> c_int;
    pub fn update_numa_distance(node: *mut device_node);
    pub fn map_cpu_to_node(cpu: c_int, node: c_int);
    pub fn find_and_update_cpu_nid(cpu: c_int);
    pub fn cpu_to_coregroup_id(cpu: c_int) -> c_int;
}

#[inline]
pub unsafe fn node_distance(a: c_int, b: c_int) -> c_int { __node_distance(a, b) }

#[inline]
pub unsafe fn update_numa_cpu_lookup_table(cpu: u32, node: c_int) {
    *numa_cpu_lookup_table.add(cpu as usize) = node;
}

#[inline]
pub unsafe fn early_cpu_to_node(cpu: c_int) -> c_int {
    let nid = *numa_cpu_lookup_table.add(cpu as usize);
    // Fall back to node 0 if nid is unset (it should be, except bugs).
    // This allows callers to safely do NODE_DATA(early_cpu_to_node(cpu)).
    if nid < 0 { 0 } else { nid }
}

// CONFIG_HOTPLUG_CPU declaration:
extern "C" { pub fn unmap_cpu_from_node(cpu: usize); }

// CONFIG_NUMA disabled inline implementations:
#[inline] pub fn early_cpu_to_node_no_numa(_cpu: c_int) -> c_int { 0 }
#[inline] pub fn dump_numa_cpu_topology_no_numa() {}
#[inline] pub fn sysfs_add_device_to_node_no_numa(_dev: *mut device, _nid: c_int) -> c_int { 0 }
#[inline] pub fn sysfs_remove_device_from_node_no_numa(_dev: *mut device, _nid: c_int) {}
#[inline] pub fn update_numa_cpu_lookup_table_no_numa(_cpu: u32, _node: c_int) {}
#[inline] pub fn cpu_relative_distance_no_numa(_a: *mut u32, _b: *mut u32) -> c_int { 0 }
#[inline] pub unsafe fn of_drconf_to_nid_single_no_numa(_lmb: *mut drmem_lmb) -> c_int { first_online_node }
#[inline] pub fn update_numa_distance_no_numa(_node: *mut device_node) {}
#[inline] pub fn map_cpu_to_node_no_numa(_cpu: c_int, _node: c_int) {}
#[inline] pub fn unmap_cpu_from_node_no_numa(_cpu: usize) {}

// CONFIG_NUMA && CONFIG_PPC_SPLPAR selects the external declarations above;
// otherwise the original inline no-op and cpu_to_core_id fallback apply.
#[inline] pub fn find_and_update_cpu_nid_no_splpar(_cpu: c_int) {}
#[inline] pub unsafe fn cpu_to_coregroup_id_no_splpar(cpu: c_int) -> c_int {
    // CONFIG_SMP selects cpu_to_core_id(cpu); otherwise the value is zero.
    cpu_to_core_id(cpu)
}

// CONFIG_SMP declarations and topology macros:
extern "C" {
    pub fn cpu_coregroup_mask(cpu: c_int) -> *mut cpumask;
    pub fn cpu_die_mask(cpu: c_int) -> *const cpumask;
    pub fn cpu_die_id(cpu: c_int) -> c_int;
}

#[inline] pub unsafe fn arch_llc_mask(cpu: c_int) -> *mut cpumask { cpu_l2_cache_mask(cpu) }

// CONFIG_PPC64 topology mappings:
#[inline] pub unsafe fn topology_physical_package_id(cpu: c_int) -> c_int { cpu_to_chip_id(cpu) }
#[inline] pub unsafe fn topology_sibling_cpumask(cpu: c_int) -> *mut cpumask { per_cpu_cpu_sibling_map(cpu) }
#[inline] pub unsafe fn topology_core_cpumask(cpu: c_int) -> *mut cpumask { per_cpu_cpu_core_map(cpu) }
#[inline] pub unsafe fn topology_core_id(cpu: c_int) -> c_int { cpu_to_core_id(cpu) }
#[inline] pub unsafe fn topology_die_id(cpu: c_int) -> c_int { cpu_die_id(cpu) }
#[inline] pub unsafe fn topology_die_cpumask(cpu: c_int) -> *const cpumask { cpu_die_mask(cpu) }

// CONFIG_HOTPLUG_SMT:
#[inline] pub unsafe fn topology_is_primary_thread(cpu: u32) -> bool { cpu == cpu_first_thread_sibling(cpu) }
#[inline] pub unsafe fn topology_smt_thread_allowed(cpu: u32) -> bool { cpu_thread_in_core(cpu) < cpu_smt_num_threads }
#[inline] pub unsafe fn topology_is_core_online(cpu: u32) -> bool {
    let first_cpu = cpu_first_thread_sibling(cpu);
    let mut i = first_cpu;
    while i < first_cpu + threads_per_core {
        if cpu_online(i) { return true; }
        i += 1;
    }
    false
}

// External symbols supplied by the other translated kernel components.
extern "C" {
    static mut cpu_all_mask: *mut cpumask;
    static mut node_to_cpumask_map: *mut *mut cpumask;
    static mut numa_cpu_lookup_table: *mut c_int;
    static first_online_node: c_int;
    static cpu_smt_num_threads: u32;
    static threads_per_core: u32;
    fn cpu_l2_cache_mask(cpu: c_int) -> *mut cpumask;
    fn cpu_to_chip_id(cpu: c_int) -> c_int;
    fn cpu_to_core_id(cpu: c_int) -> c_int;
    fn per_cpu_cpu_sibling_map(cpu: c_int) -> *mut cpumask;
    fn per_cpu_cpu_core_map(cpu: c_int) -> *mut cpumask;
    fn cpu_first_thread_sibling(cpu: u32) -> u32;
    fn cpu_thread_in_core(cpu: u32) -> u32;
    fn cpu_online(cpu: u32) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
