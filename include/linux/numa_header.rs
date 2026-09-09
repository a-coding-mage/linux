/* SPDX-License-Identifier: GPL-2.0 */

// Translated from linux/numa.h.
// Dependencies supplied by the surrounding kernel translation are referenced
// here but are not defined in this header.

pub const NUMA_NO_MEMBLK: i32 = -1;

#[inline]
pub const fn numa_valid_node(nid: i32) -> bool {
    nid >= 0 && nid < MAX_NUMNODES
}

/* optionally keep NUMA memory info available post init */
// CONFIG_NUMA_KEEP_MEMINFO selects an empty annotation; otherwise __initdata
// is used by the original kernel build.

#[cfg(feature = "CONFIG_NUMA")]
extern "C" {
    pub static mut node_data: [*mut pglist_data; 0];
}

#[cfg(feature = "CONFIG_NUMA")]
#[inline]
pub unsafe fn NODE_DATA(nid: usize) -> *mut pglist_data {
    node_data.as_ptr().add(nid).read()
}

#[cfg(feature = "CONFIG_NUMA")]
extern "C" {
    pub fn alloc_node_data(nid: i32);
    pub fn alloc_offline_node_data(nid: i32);

    /* Generic implementation available */
    pub fn numa_nearest_node(node: i32, state: u32) -> i32;
    pub fn nearest_node_nodemask(node: i32, mask: *mut nodemask_t) -> i32;

    #[cfg(not(feature = "memory_add_physaddr_to_nid"))]
    pub fn memory_add_physaddr_to_nid(start: u64) -> i32;

    #[cfg(not(feature = "phys_to_target_node"))]
    pub fn phys_to_target_node(start: u64) -> i32;

    pub fn numa_fill_memblks(start: u64, end: u64) -> i32;
}

#[cfg(not(feature = "CONFIG_NUMA"))]
#[inline]
pub const fn numa_nearest_node(_node: i32, _state: u32) -> i32 {
    NUMA_NO_NODE
}

#[cfg(not(feature = "CONFIG_NUMA"))]
#[inline]
pub const fn nearest_node_nodemask(_node: i32, _mask: *mut nodemask_t) -> i32 {
    NUMA_NO_NODE
}

#[cfg(not(feature = "CONFIG_NUMA"))]
#[inline]
pub const fn memory_add_physaddr_to_nid(_start: u64) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_NUMA"))]
#[inline]
pub const fn phys_to_target_node(_start: u64) -> i32 {
    0
}

// #define numa_map_to_online_node(node) numa_nearest_node(node, N_ONLINE)
#[inline]
pub fn numa_map_to_online_node(node: i32) -> i32 {
    numa_nearest_node(node, N_ONLINE)
}

#[cfg(feature = "CONFIG_HAVE_ARCH_NODE_DEV_GROUP")]
extern "C" {
    pub static arch_node_dev_group: attribute_group;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
