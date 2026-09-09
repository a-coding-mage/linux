/*
 * Translation of include/linux/topology.h.
 * C includes and preprocessor guards are retained below as conditional intent.
 */

// #include <linux/arch_topology.h>
// #include <linux/cpumask.h>
// #include <linux/nodemask.h>
// #include <linux/bitops.h>
// #include <linux/mmzone.h>
// #include <linux/smp.h>
// #include <linux/percpu.h>
// #include <asm/topology.h>

#[allow(non_camel_case_types)]
pub type c_int = i32;
pub type c_uint = u32;
pub type c_ulong = usize;

pub const LOCAL_DISTANCE: c_int = 10;
pub const REMOTE_DISTANCE: c_int = 20;
pub const DISTANCE_BITS: c_int = 8;
pub const RECLAIM_DISTANCE: c_int = 30;
pub const PENALTY_FOR_NODE_WITH_CPUS: c_int = 1;

#[repr(C)]
pub struct cpumask {
    _private: [u8; 0],
}

extern "C" {
    pub fn arch_update_cpu_topology() -> c_int;
    pub static mut node_reclaim_distance: c_int;
    pub fn cpumask_of_node(node: c_int) -> *const cpumask;
    pub fn cpumask_of(cpu: c_int) -> *const cpumask;
    pub fn cpumask_weight(mask: *const cpumask) -> c_uint;
    pub fn cpumask_first(mask: *const cpumask) -> c_int;
    pub fn cpumask_nth_and(n: c_int, cpus: *const cpumask, online: *const cpumask) -> c_int;
    pub fn cpu_to_node(cpu: c_int) -> c_int;
    pub static cpu_online_mask: *const cpumask;
    pub fn raw_smp_processor_id() -> c_int;
    pub fn nearest_node_nodemask(start: c_int, unvisited: *mut core::ffi::c_void) -> c_int;
    pub fn node_clear(node: c_int, mask: *mut core::ffi::c_void);
    pub fn topology_set_cpu_scale(cpu: c_uint, capacity: c_ulong);
}

pub const MAX_NUMNODES: c_int = 0; // supplied by the nodemask dependency
pub const NUMA_NO_NODE: c_uint = !0;

#[inline]
pub unsafe fn nr_cpus_node(node: c_int) -> c_uint {
    cpumask_weight(cpumask_of_node(node))
}

#[inline]
pub const fn node_distance(from: c_int, to: c_int) -> c_int {
    if from == to { LOCAL_DISTANCE } else { REMOTE_DISTANCE }
}

// CONFIG_USE_PERCPU_NUMA_NODE_ID selects the following per-CPU implementation.
#[cfg(feature = "CONFIG_USE_PERCPU_NUMA_NODE_ID")]
extern "C" {
    pub static mut numa_node: c_int;
    pub fn raw_cpu_read_numa_node() -> c_int;
    pub fn per_cpu_numa_node(cpu: c_int) -> c_int;
    pub fn this_cpu_write_numa_node(node: c_int);
    pub fn set_per_cpu_numa_node(cpu: c_int, node: c_int);
}

#[cfg(feature = "CONFIG_USE_PERCPU_NUMA_NODE_ID")]
#[inline]
pub unsafe fn numa_node_id() -> c_int { raw_cpu_read_numa_node() }

#[cfg(feature = "CONFIG_USE_PERCPU_NUMA_NODE_ID")]
#[inline]
pub unsafe fn cpu_to_node_percpu(cpu: c_int) -> c_int { per_cpu_numa_node(cpu) }

#[cfg(feature = "CONFIG_USE_PERCPU_NUMA_NODE_ID")]
#[inline]
pub unsafe fn set_numa_node(node: c_int) { this_cpu_write_numa_node(node) }

#[cfg(feature = "CONFIG_USE_PERCPU_NUMA_NODE_ID")]
#[inline]
pub unsafe fn set_cpu_numa_node(cpu: c_int, node: c_int) { set_per_cpu_numa_node(cpu, node) }

#[cfg(not(feature = "CONFIG_USE_PERCPU_NUMA_NODE_ID"))]
#[inline]
pub unsafe fn numa_node_id() -> c_int { cpu_to_node(raw_smp_processor_id()) }

// CONFIG_HAVE_MEMORYLESS_NODES selects the per-CPU memory-node accessors.
#[cfg(feature = "CONFIG_HAVE_MEMORYLESS_NODES")]
extern "C" {
    pub static mut _numa_mem_: c_int;
    pub fn this_cpu_write_numa_mem(node: c_int);
    pub fn raw_cpu_read_numa_mem() -> c_int;
    pub fn per_cpu_numa_mem(cpu: c_int) -> c_int;
    pub fn set_per_cpu_numa_mem(cpu: c_int, node: c_int);
}

#[cfg(feature = "CONFIG_HAVE_MEMORYLESS_NODES")]
#[inline]
pub unsafe fn set_numa_mem(node: c_int) { this_cpu_write_numa_mem(node) }
#[cfg(feature = "CONFIG_HAVE_MEMORYLESS_NODES")]
#[inline]
pub unsafe fn numa_mem_id() -> c_int { raw_cpu_read_numa_mem() }
#[cfg(feature = "CONFIG_HAVE_MEMORYLESS_NODES")]
#[inline]
pub unsafe fn cpu_to_mem(cpu: c_int) -> c_int { per_cpu_numa_mem(cpu) }
#[cfg(feature = "CONFIG_HAVE_MEMORYLESS_NODES")]
#[inline]
pub unsafe fn set_cpu_numa_mem(cpu: c_int, node: c_int) { set_per_cpu_numa_mem(cpu, node) }

#[cfg(not(feature = "CONFIG_HAVE_MEMORYLESS_NODES"))]
#[inline]
pub unsafe fn numa_mem_id() -> c_int { numa_node_id() }
#[cfg(not(feature = "CONFIG_HAVE_MEMORYLESS_NODES"))]
#[inline]
pub unsafe fn cpu_to_mem(cpu: c_int) -> c_int { cpu_to_node(cpu) }

#[inline]
pub unsafe fn topology_physical_package_id(_cpu: c_int) -> c_int { -1 }
#[inline]
pub unsafe fn topology_die_id(_cpu: c_int) -> c_int { -1 }
#[inline]
pub unsafe fn topology_cluster_id(_cpu: c_int) -> c_int { -1 }
#[inline]
pub unsafe fn topology_core_id(_cpu: c_int) -> c_int { 0 }
#[inline]
pub unsafe fn topology_book_id(_cpu: c_int) -> c_int { -1 }
#[inline]
pub unsafe fn topology_drawer_id(_cpu: c_int) -> c_int { -1 }
#[inline]
pub unsafe fn topology_ppin(_cpu: c_int) -> u64 { 0 }
#[inline]
pub unsafe fn topology_sibling_cpumask(cpu: c_int) -> *const cpumask { cpumask_of(cpu) }
#[inline]
pub unsafe fn topology_core_cpumask(cpu: c_int) -> *const cpumask { cpumask_of(cpu) }
#[inline]
pub unsafe fn topology_cluster_cpumask(cpu: c_int) -> *const cpumask { cpumask_of(cpu) }
#[inline]
pub unsafe fn topology_die_cpumask(cpu: c_int) -> *const cpumask { cpumask_of(cpu) }
#[inline]
pub unsafe fn topology_book_cpumask(cpu: c_int) -> *const cpumask { cpumask_of(cpu) }
#[inline]
pub unsafe fn topology_drawer_cpumask(cpu: c_int) -> *const cpumask { cpumask_of(cpu) }

#[inline]
pub unsafe fn cpu_smt_mask(cpu: c_int) -> *const cpumask { topology_sibling_cpumask(cpu) }

#[inline]
pub unsafe fn topology_is_primary_thread(cpu: c_uint) -> bool {
    cpu as c_int == cpumask_first(topology_sibling_cpumask(cpu as c_int))
}

#[inline]
pub unsafe fn cpu_node_mask(cpu: c_int) -> *const cpumask {
    cpumask_of_node(cpu_to_node(cpu))
}

// CONFIG_NUMA provides architecture-specific implementations.
#[cfg(feature = "CONFIG_NUMA")]
extern "C" {
    pub fn sched_numa_find_nth_cpu(cpus: *const cpumask, cpu: c_int, node: c_int) -> c_int;
    pub fn sched_numa_hop_mask(node: c_uint, hops: c_uint) -> *const cpumask;
}

#[cfg(not(feature = "CONFIG_NUMA"))]
#[inline]
pub unsafe fn sched_numa_find_nth_cpu(cpus: *const cpumask, cpu: c_int, _node: c_int) -> c_int {
    cpumask_nth_and(cpu, cpus, cpu_online_mask)
}

#[cfg(not(feature = "CONFIG_NUMA"))]
#[inline]
pub unsafe fn sched_numa_hop_mask(_node: c_uint, _hops: c_uint) -> *const cpumask {
    core::ptr::null()
}

// for_each_node_numadist and for_each_numa_hop_mask are C iteration macros;
// callers should preserve their loop state and ordering when expanding them.

extern "C" {
    pub static mut cpu_scale: c_ulong;
    pub fn per_cpu_cpu_scale(cpu: c_int) -> c_ulong;
}

#[inline]
pub unsafe fn topology_get_cpu_scale(cpu: c_int) -> c_ulong {
    per_cpu_cpu_scale(cpu)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
