/* SPDX-License-Identifier: GPL-2.0 */

/* CONFIG_NUMA conditional preserved from the original header. */
#[cfg(CONFIG_NUMA)]
pub const NR_NODE_MEMBLKS: usize = MAX_NUMNODES * 2;

#[cfg(CONFIG_NUMA)]
extern "C" {
    pub fn __node_distance(from: ::core::ffi::c_int, to: ::core::ffi::c_int) -> ::core::ffi::c_int;

    pub static mut numa_nodes_parsed: nodemask_t;
    pub static mut numa_off: bool;
    pub static mut node_to_cpumask_map: [cpumask_var_t; MAX_NUMNODES];

    pub fn numa_clear_node(cpu: ::core::ffi::c_uint);

    #[cfg(CONFIG_DEBUG_PER_CPU_MAPS)]
    pub fn cpumask_of_node(node: ::core::ffi::c_int) -> *const cpumask;

    pub fn arch_numa_init();
    pub fn numa_add_memblk(
        nodeid: ::core::ffi::c_int,
        start: u64,
        end: u64,
    ) -> ::core::ffi::c_int;
    pub fn early_map_cpu_to_node(cpu: ::core::ffi::c_uint, nid: ::core::ffi::c_int);
    pub fn early_cpu_to_node(cpu: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn numa_store_cpu_info(cpu: ::core::ffi::c_uint);
    pub fn numa_add_cpu(cpu: ::core::ffi::c_uint);
    pub fn numa_remove_cpu(cpu: ::core::ffi::c_uint);
}

#[cfg(all(CONFIG_NUMA, not(CONFIG_DEBUG_PER_CPU_MAPS)))]
#[inline]
pub unsafe fn cpumask_of_node(node: ::core::ffi::c_int) -> *const cpumask {
    if node == NUMA_NO_NODE {
        return cpu_all_mask;
    }

    node_to_cpumask_map[node as usize]
}

#[cfg(not(CONFIG_NUMA))]
#[inline]
pub unsafe fn numa_store_cpu_info(_cpu: ::core::ffi::c_uint) {}

#[cfg(not(CONFIG_NUMA))]
#[inline]
pub unsafe fn numa_add_cpu(_cpu: ::core::ffi::c_uint) {}

#[cfg(not(CONFIG_NUMA))]
#[inline]
pub unsafe fn numa_remove_cpu(_cpu: ::core::ffi::c_uint) {}

#[cfg(not(CONFIG_NUMA))]
#[inline]
pub unsafe fn arch_numa_init() {}

#[cfg(not(CONFIG_NUMA))]
#[inline]
pub unsafe fn early_map_cpu_to_node(_cpu: ::core::ffi::c_uint, _nid: ::core::ffi::c_int) {}

#[cfg(not(CONFIG_NUMA))]
#[inline]
pub unsafe fn early_cpu_to_node(_cpu: ::core::ffi::c_int) -> ::core::ffi::c_int {
    0
}

#[cfg(CONFIG_NUMA_EMU)]
extern "C" {
    pub fn debug_cpumask_set_cpu(
        cpu: ::core::ffi::c_uint,
        node: ::core::ffi::c_int,
        enable: bool,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
