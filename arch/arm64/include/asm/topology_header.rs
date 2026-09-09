/* SPDX-License-Identifier: GPL-2.0 */

// Translation of <linux/cpumask.h> dependencies.

#[cfg(feature = "CONFIG_NUMA")]
#[repr(C)]
pub struct pci_bus {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_NUMA")]
extern "C" {
    pub fn pcibus_to_node(bus: *mut pci_bus) -> i32;
    pub static cpu_all_mask: *const cpumask_t;
    pub fn cpumask_of_node(node: i32) -> *const cpumask_t;
}

#[cfg(feature = "CONFIG_NUMA")]
#[repr(C)]
pub struct cpumask_t {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_NUMA")]
#[inline]
pub unsafe fn cpumask_of_pcibus(bus: *mut pci_bus) -> *const cpumask_t {
    if pcibus_to_node(bus) == -1 {
        cpu_all_mask
    } else {
        cpumask_of_node(pcibus_to_node(bus))
    }
}

// <linux/arch_topology.h>
extern "C" {
    pub fn update_freq_counters_refs();
}

// Replace task scheduler's default frequency-invariant accounting.
pub use topology_scale_freq_tick as arch_scale_freq_tick;
pub use topology_set_freq_scale as arch_set_freq_scale;
pub use topology_get_freq_scale as arch_scale_freq_capacity;
pub use topology_scale_freq_invariant as arch_scale_freq_invariant;
pub use topology_get_freq_ref as arch_scale_freq_ref;

// Replace task scheduler's default cpu-invariant accounting.
pub use topology_get_cpu_scale as arch_scale_cpu_capacity;

// Enable topology flag updates.
pub use topology_update_cpu_topology as arch_update_cpu_topology;

// Replace task scheduler's default HW pressure API.
pub use topology_get_hw_pressure as arch_scale_hw_pressure;
pub use topology_update_hw_pressure as arch_update_hw_pressure;

// <asm-generic/topology.h>

// arch_cpu_is_threaded is explicitly undefined before this replacement.
#[inline]
pub unsafe fn arch_cpu_is_threaded() -> u64 {
    read_cpuid_mpidr() & MPIDR_MT_BITMASK
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
