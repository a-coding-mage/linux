/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Written by Kanoj Sarcar (kanoj@sgi.com) Aug 99
 *
 * PowerPC64 port:
 * Copyright (C) 2002 Anton Blanchard, IBM Corp.
 */

/* This header is active only when __KERNEL__ is defined in the C build. */

/* Dependency: linux/cpumask.h */

/*
 * generic non-linear memory support:
 *
 * 1) we will not split memory into more chunks than will fit into the
 *    flags field of the struct page
 */

/* Following declarations are specific to this NUMA platform. */
#[cfg(CONFIG_NUMA)]
extern "C" {
    pub static mut numa_cpu_lookup_table: *mut ::core::ffi::c_int;
    pub static mut node_to_cpumask_map: *mut cpumask_var_t;
}

#[cfg(all(CONFIG_NUMA, CONFIG_MEMORY_HOTPLUG))]
extern "C" {
    pub static mut max_pfn: ::core::ffi::c_ulong;
    pub fn memory_hotplug_max() -> u64;
    pub fn hot_add_drconf_memory_max() -> u64;
}

/* CONFIG_MEMORY_HOTPLUG and CONFIG_NUMA determine the C declaration below. */
#[cfg(any(not(CONFIG_NUMA), all(CONFIG_NUMA, not(CONFIG_MEMORY_HOTPLUG))))]
pub unsafe fn memory_hotplug_max() -> u64 {
    memblock_end_of_DRAM()
}

/* External dependency referenced by the C macro memory_hotplug_max(). */
unsafe extern "C" {
    pub fn memblock_end_of_DRAM() -> u64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
