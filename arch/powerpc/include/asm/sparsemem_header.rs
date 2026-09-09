/* SPDX-License-Identifier: GPL-2.0 */

/* The original declarations are active only when __KERNEL__ is defined. */

/* CONFIG_SPARSEMEM */
/*
 * SECTION_SIZE_BITS      2^N: how big each section will be
 * MAX_PHYSMEM_BITS      2^N: how much memory we can have in that space
 */
#[cfg(feature = "CONFIG_SPARSEMEM")]
pub const SECTION_SIZE_BITS: u32 = 24;

/* CONFIG_MEMORY_HOTPLUG */
#[cfg(feature = "CONFIG_MEMORY_HOTPLUG")]
unsafe extern "C" {
    pub fn remove_section_mapping(start: ::core::ffi::c_ulong,
                                  end: ::core::ffi::c_ulong)
        -> ::core::ffi::c_int;
    pub fn memory_add_physaddr_to_nid(start: u64) -> ::core::ffi::c_int;
}

/* C macro: #define memory_add_physaddr_to_nid memory_add_physaddr_to_nid */

#[cfg(all(feature = "CONFIG_MEMORY_HOTPLUG", feature = "CONFIG_NUMA"))]
unsafe extern "C" {
    pub fn hot_add_scn_to_nid(scn_addr: ::core::ffi::c_ulong)
        -> ::core::ffi::c_int;
}

#[cfg(all(feature = "CONFIG_MEMORY_HOTPLUG", not(feature = "CONFIG_NUMA")))]
#[inline]
pub fn hot_add_scn_to_nid(_scn_addr: ::core::ffi::c_ulong) -> ::core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
