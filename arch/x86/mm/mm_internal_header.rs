/* SPDX-License-Identifier: GPL-2.0 */

extern "C" {
    pub fn alloc_low_pages(num: ::kernel::ffi::c_uint) -> *mut ::kernel::ffi::c_void;
}

#[inline]
pub unsafe fn alloc_low_page() -> *mut ::kernel::ffi::c_void {
    alloc_low_pages(1)
}

extern "C" {
    pub fn early_ioremap_page_table_range_init();

    pub fn kernel_physical_mapping_init(
        start: ::kernel::ffi::c_ulong,
        end: ::kernel::ffi::c_ulong,
        page_size_mask: ::kernel::ffi::c_ulong,
        prot: pgprot_t,
    ) -> ::kernel::ffi::c_ulong;

    pub fn kernel_physical_mapping_change(
        start: ::kernel::ffi::c_ulong,
        end: ::kernel::ffi::c_ulong,
        page_size_mask: ::kernel::ffi::c_ulong,
    ) -> ::kernel::ffi::c_ulong;

    pub static mut after_bootmem: ::kernel::ffi::c_int;

    pub fn update_cache_mode_entry(entry: ::kernel::ffi::c_uint, cache: page_cache_mode);

    pub static mut tlb_single_page_flush_ceiling: ::kernel::ffi::c_ulong;
}

// CONFIG_NUMA conditionally declares this initialization function.
#[cfg(CONFIG_NUMA)]
extern "C" {
    // C __init annotation has no direct Rust equivalent.
    pub fn x86_numa_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
