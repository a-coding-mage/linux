/* SPDX-License-Identifier: GPL-2.0 */

pub const SECTION_SIZE_BITS: u32 = 27;
pub const MAX_PHYSMEM_BITS: u32 = CONFIG_MAX_PHYSMEM_BITS;

/* CONFIG_NUMA: these declarations are present when NUMA support is enabled. */
#[inline]
pub fn memory_add_physaddr_to_nid(_addr: u64) -> i32 {
    0
}

/* C macro alias: memory_add_physaddr_to_nid memory_add_physaddr_to_nid */

#[inline]
pub fn phys_to_target_node(_start: u64) -> i32 {
    0
}

/* C macro alias: phys_to_target_node phys_to_target_node */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
