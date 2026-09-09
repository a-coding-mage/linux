/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel translation. */

#[repr(C)]
pub struct sysinfo_15_1_x;

#[repr(C)]
pub struct cpu;

/* CONFIG_SCHED_TOPOLOGY */
#[repr(C)]
pub struct cpu_topology_s390 {
    pub thread_id: u16,
    pub core_id: u16,
    pub socket_id: u16,
    pub book_id: u16,
    pub drawer_id: u16,
    /* C bit-field: dedicated : 1. */
    pub dedicated: u16,
    pub booted_cores: core::ffi::c_int,
    pub thread_mask: cpumask_t,
    pub core_mask: cpumask_t,
    pub book_mask: cpumask_t,
    pub drawer_mask: cpumask_t,
}

#[cfg(CONFIG_SCHED_TOPOLOGY)]
extern "C" {
    pub static mut cpu_topology: [cpu_topology_s390; NR_CPUS];

    pub fn topology_init_early();
    pub fn topology_cpu_init(cpu: *mut cpu) -> core::ffi::c_int;
    pub fn topology_set_cpu_management(fc: core::ffi::c_int) -> core::ffi::c_int;
    pub fn topology_schedule_update();
    pub fn store_topology(info: *mut sysinfo_15_1_x);
    pub fn update_cpu_masks();
    pub fn topology_expect_change();
    pub fn cpu_coregroup_mask(cpu: core::ffi::c_int) -> *const cpumask;
}

#[cfg(CONFIG_SCHED_TOPOLOGY)]
pub unsafe fn topology_physical_package_id(cpu: usize) -> u16 { cpu_topology[cpu].socket_id }
#[cfg(CONFIG_SCHED_TOPOLOGY)]
pub unsafe fn topology_thread_id(cpu: usize) -> u16 { cpu_topology[cpu].thread_id }
#[cfg(CONFIG_SCHED_TOPOLOGY)]
pub unsafe fn topology_sibling_cpumask(cpu: usize) -> *mut cpumask_t { &mut cpu_topology[cpu].thread_mask }
#[cfg(CONFIG_SCHED_TOPOLOGY)]
pub unsafe fn topology_core_id(cpu: usize) -> u16 { cpu_topology[cpu].core_id }
#[cfg(CONFIG_SCHED_TOPOLOGY)]
pub unsafe fn topology_core_cpumask(cpu: usize) -> *mut cpumask_t { &mut cpu_topology[cpu].core_mask }
#[cfg(CONFIG_SCHED_TOPOLOGY)]
pub unsafe fn topology_book_id(cpu: usize) -> u16 { cpu_topology[cpu].book_id }
#[cfg(CONFIG_SCHED_TOPOLOGY)]
pub unsafe fn topology_book_cpumask(cpu: usize) -> *mut cpumask_t { &mut cpu_topology[cpu].book_mask }
#[cfg(CONFIG_SCHED_TOPOLOGY)]
pub unsafe fn topology_drawer_id(cpu: usize) -> u16 { cpu_topology[cpu].drawer_id }
#[cfg(CONFIG_SCHED_TOPOLOGY)]
pub unsafe fn topology_drawer_cpumask(cpu: usize) -> *mut cpumask_t { &mut cpu_topology[cpu].drawer_mask }
#[cfg(CONFIG_SCHED_TOPOLOGY)]
pub unsafe fn topology_cpu_dedicated(cpu: usize) -> u16 { cpu_topology[cpu].dedicated }
#[cfg(CONFIG_SCHED_TOPOLOGY)]
pub unsafe fn topology_booted_cores(cpu: usize) -> core::ffi::c_int { cpu_topology[cpu].booted_cores }
#[cfg(CONFIG_SCHED_TOPOLOGY)]
pub const fn mc_capable() -> core::ffi::c_int { 1 }

#[cfg(not(CONFIG_SCHED_TOPOLOGY))]
pub unsafe fn topology_init_early() {}
#[cfg(not(CONFIG_SCHED_TOPOLOGY))]
pub unsafe fn topology_schedule_update() {}
#[cfg(not(CONFIG_SCHED_TOPOLOGY))]
pub unsafe fn topology_cpu_init(_cpu: *mut cpu) -> core::ffi::c_int { 0 }
#[cfg(not(CONFIG_SCHED_TOPOLOGY))]
pub unsafe fn topology_cpu_dedicated(_cpu_nr: core::ffi::c_int) -> core::ffi::c_int { 0 }
#[cfg(not(CONFIG_SCHED_TOPOLOGY))]
pub unsafe fn topology_booted_cores(_cpu_nr: core::ffi::c_int) -> core::ffi::c_int { 1 }
#[cfg(not(CONFIG_SCHED_TOPOLOGY))]
pub unsafe fn update_cpu_masks() {}
#[cfg(not(CONFIG_SCHED_TOPOLOGY))]
pub unsafe fn topology_expect_change() {}

pub unsafe fn topology_is_primary_thread(cpu: core::ffi::c_uint) -> bool {
    smp_get_base_cpu(cpu) == cpu
}

pub const POLARIZATION_UNKNOWN: core::ffi::c_int = -1;
pub const POLARIZATION_HRZ: core::ffi::c_int = 0;
pub const POLARIZATION_VL: core::ffi::c_int = 1;
pub const POLARIZATION_VM: core::ffi::c_int = 2;
pub const POLARIZATION_VH: core::ffi::c_int = 3;

pub const CPU_CAPACITY_HIGH: core::ffi::c_int = SCHED_CAPACITY_SCALE;
pub const CPU_CAPACITY_LOW: core::ffi::c_int = SCHED_CAPACITY_SCALE >> 3;
pub const SD_BOOK_INIT: core::ffi::c_int = SD_CPU_INIT;

#[cfg(CONFIG_NUMA)]
pub unsafe fn cpu_to_node(_cpu: core::ffi::c_int) -> core::ffi::c_int { 0 }

#[cfg(CONFIG_NUMA)]
pub unsafe fn cpumask_of_node(_node: core::ffi::c_int) -> *const cpumask {
    cpu_possible_mask
}

#[cfg(CONFIG_NUMA)]
pub unsafe fn pcibus_to_node(bus: *mut core::ffi::c_void) -> core::ffi::c_int {
    __pcibus_to_node(bus)
}

#[cfg(not(CONFIG_NUMA))]
pub unsafe fn numa_node_id() -> core::ffi::c_int { 0 }

/* The generic topology declarations are included by the surrounding translation. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
