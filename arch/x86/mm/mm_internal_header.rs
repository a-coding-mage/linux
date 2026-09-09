/* SPDX-License-Identifier: GPL-2.0 */

extern "C" {
    pub fn alloc_low_pages(num: ::core::ffi::c_uint) -> *mut ::core::ffi::c_void;
}

#[inline]
pub unsafe fn alloc_low_page() -> *mut ::core::ffi::c_void {
    alloc_low_pages(1)
}

extern "C" {
    pub fn early_ioremap_page_table_range_init();

    pub fn kernel_physical_mapping_init(
        start: ::core::ffi::c_ulong,
        end: ::core::ffi::c_ulong,
        page_size_mask: ::core::ffi::c_ulong,
        prot: pgprot_t,
    ) -> ::core::ffi::c_ulong;

    pub fn kernel_physical_mapping_change(
        start: ::core::ffi::c_ulong,
        end: ::core::ffi::c_ulong,
        page_size_mask: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong;

    pub static mut after_bootmem: ::core::ffi::c_int;

    pub fn update_cache_mode_entry(entry: ::core::ffi::c_uint, cache: page_cache_mode);

    pub static mut tlb_single_page_flush_ceiling: ::core::ffi::c_ulong;
}

// CONFIG_NUMA conditionally declares this initialization function.
#[cfg(CONFIG_NUMA)]
extern "C" {
    // C __init annotation has no direct Rust equivalent.
    pub fn x86_numa_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
