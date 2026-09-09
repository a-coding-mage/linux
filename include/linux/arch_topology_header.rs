/* SPDX-License-Identifier: GPL-2.0 */
/*
 * include/linux/arch_topology.h - arch specific cpu topology information
 */

// Dependencies supplied by other translated headers: linux types, percpu,
// cpumask_t, cpumask, NR_CPUS, and per_cpu().

unsafe extern "C" {
    pub fn topology_normalize_cpu_scale();
    pub fn topology_update_cpu_topology() -> core::ffi::c_int;

    pub fn topology_parse_cpu_capacity(cpu_node: *mut device_node, cpu: core::ffi::c_int) -> bool;

    pub fn topology_set_freq_scale(
        cpus: *const cpumask,
        cur_freq: libc::c_ulong,
        max_freq: libc::c_ulong,
    );
    pub fn topology_scale_freq_invariant() -> bool;

    pub fn topology_scale_freq_tick();
    pub fn topology_set_scale_freq_source(
        data: *mut scale_freq_data,
        cpus: *const cpumask,
    );
    pub fn topology_clear_scale_freq_source(
        source: scale_freq_source,
        cpus: *const cpumask,
    );

    pub fn topology_update_hw_pressure(cpus: *const cpumask, capped_freq: libc::c_ulong);
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

// DECLARE_PER_CPU(unsigned long, capacity_freq_ref);
pub unsafe fn topology_get_freq_ref(cpu: core::ffi::c_int) -> libc::c_ulong {
    per_cpu_capacity_freq_ref(cpu)
}

extern "C" {
    fn per_cpu_capacity_freq_ref(cpu: core::ffi::c_int) -> libc::c_ulong;
    fn per_cpu_arch_freq_scale(cpu: core::ffi::c_int) -> libc::c_ulong;
    fn per_cpu_hw_pressure(cpu: core::ffi::c_int) -> libc::c_ulong;
}

pub unsafe fn topology_get_freq_scale(cpu: core::ffi::c_int) -> libc::c_ulong {
    per_cpu_arch_freq_scale(cpu)
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum scale_freq_source {
    SCALE_FREQ_SOURCE_CPUFREQ = 0,
    SCALE_FREQ_SOURCE_ARCH,
    SCALE_FREQ_SOURCE_CPPC,
    SCALE_FREQ_SOURCE_VIRT,
}

#[repr(C)]
pub struct scale_freq_data {
    pub source: scale_freq_source,
    pub set_freq_scale: Option<unsafe extern "C" fn()>,
}

// DECLARE_PER_CPU(unsigned long, hw_pressure);
pub unsafe fn topology_get_hw_pressure(cpu: core::ffi::c_int) -> libc::c_ulong {
    per_cpu_hw_pressure(cpu)
}

#[repr(C)]
pub struct cpu_topology {
    pub thread_id: core::ffi::c_int,
    pub core_id: core::ffi::c_int,
    pub cluster_id: core::ffi::c_int,
    pub package_id: core::ffi::c_int,
    pub thread_sibling: cpumask_t,
    pub core_sibling: cpumask_t,
    pub cluster_sibling: cpumask_t,
    pub llc_sibling: cpumask_t,
}

// CONFIG_GENERIC_ARCH_TOPOLOGY controls whether the following declarations
// and accessors are available in the build configuration.
unsafe extern "C" {
    pub static mut cpu_topology: [cpu_topology; NR_CPUS];

    pub fn init_cpu_topology();
    pub fn store_cpu_topology(cpuid: core::ffi::c_uint);
    pub fn cpu_coregroup_mask(cpu: core::ffi::c_int) -> *const cpumask;
    pub fn cpu_clustergroup_mask(cpu: core::ffi::c_int) -> *const cpumask;
    pub fn update_siblings_masks(cpu: core::ffi::c_uint);
    pub fn remove_cpu_topology(cpuid: core::ffi::c_uint);
    pub fn reset_cpu_topology();
    pub fn parse_acpi_topology() -> core::ffi::c_int;
    pub fn freq_inv_set_max_ratio(cpu: core::ffi::c_int, max_rate: u64);
}

pub unsafe fn topology_physical_package_id(cpu: usize) -> core::ffi::c_int {
    cpu_topology[cpu].package_id
}
pub unsafe fn topology_cluster_id(cpu: usize) -> core::ffi::c_int {
    cpu_topology[cpu].cluster_id
}
pub unsafe fn topology_core_id(cpu: usize) -> core::ffi::c_int {
    cpu_topology[cpu].core_id
}
pub unsafe fn topology_core_cpumask(cpu: usize) -> *mut cpumask_t {
    &mut cpu_topology[cpu].core_sibling
}
pub unsafe fn topology_sibling_cpumask(cpu: usize) -> *mut cpumask_t {
    &mut cpu_topology[cpu].thread_sibling
}
pub unsafe fn topology_cluster_cpumask(cpu: usize) -> *mut cpumask_t {
    &mut cpu_topology[cpu].cluster_sibling
}
pub unsafe fn topology_llc_cpumask(cpu: usize) -> *mut cpumask_t {
    &mut cpu_topology[cpu].llc_sibling
}

// If arch_cpu_is_threaded is not provided by the architecture, its C default
// is equivalent to this function returning zero.
pub const fn arch_cpu_is_threaded() -> core::ffi::c_int { 0 }

/*
 * Architectures like ARM64 don't have reliable architectural way to get SMT
 * information and depend on the firmware (ACPI/OF) report. Non-SMT core won't
 * initialize thread_id so we can use this to detect the SMT implementation.
 */
pub unsafe fn topology_core_has_smt(cpu: usize) -> bool {
    cpu_topology[cpu].thread_id != -1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
