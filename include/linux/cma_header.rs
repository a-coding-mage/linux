/* SPDX-License-Identifier: GPL-2.0 */

// CONFIG_CMA_AREAS: when configured, MAX_CMA_AREAS is defined to its value.

pub const CMA_MAX_NAME: usize = 64;

/*
 * the buddy -- especially pageblock merging and alloc_contig_range()
 * -- can deal with only some pageblocks of a higher-order page being
 * MIGRATE_CMA, we can use pageblock_nr_pages.
 */
pub const CMA_MIN_ALIGNMENT_PAGES: usize = pageblock_nr_pages;
pub const CMA_MIN_ALIGNMENT_BYTES: usize = PAGE_SIZE * CMA_MIN_ALIGNMENT_PAGES;

#[repr(C)]
pub struct cma {
    _private: [u8; 0],
}

extern "C" {
    pub static mut totalcma_pages: ::core::ffi::c_ulong;
    pub fn cma_get_base(cma: *const cma) -> phys_addr_t;
    pub fn cma_get_size(cma: *const cma) -> ::core::ffi::c_ulong;
    pub fn cma_get_name(cma: *const cma) -> *const ::core::ffi::c_char;

    pub fn cma_declare_contiguous_nid(
        base: phys_addr_t,
        size: phys_addr_t,
        limit: phys_addr_t,
        alignment: phys_addr_t,
        order_per_bit: ::core::ffi::c_uint,
        fixed: bool,
        name: *const ::core::ffi::c_char,
        res_cma: *mut *mut cma,
        nid: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;

    pub fn cma_declare_contiguous_multi(
        size: phys_addr_t,
        align: phys_addr_t,
        order_per_bit: ::core::ffi::c_uint,
        name: *const ::core::ffi::c_char,
        res_cma: *mut *mut cma,
        nid: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;

    pub fn cma_init_reserved_mem(
        base: phys_addr_t,
        size: phys_addr_t,
        order_per_bit: ::core::ffi::c_uint,
        name: *const ::core::ffi::c_char,
        res_cma: *mut *mut cma,
    ) -> ::core::ffi::c_int;

    pub fn cma_alloc(
        cma: *mut cma,
        count: ::core::ffi::c_ulong,
        align: ::core::ffi::c_uint,
        no_warn: bool,
    ) -> *mut page;
    pub fn cma_release(
        cma: *mut cma,
        pages: *const page,
        count: ::core::ffi::c_ulong,
    ) -> bool;

    pub fn cma_alloc_frozen(
        cma: *mut cma,
        count: ::core::ffi::c_ulong,
        align: ::core::ffi::c_uint,
        no_warn: bool,
    ) -> *mut page;
    pub fn cma_alloc_frozen_compound(cma: *mut cma, order: ::core::ffi::c_uint) -> *mut page;
    pub fn cma_release_frozen(
        cma: *mut cma,
        pages: *const page,
        count: ::core::ffi::c_ulong,
    ) -> bool;

    pub fn cma_for_each_area(
        it: Option<unsafe extern "C" fn(cma: *mut cma, data: *mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
        data: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    pub fn cma_intersects(cma: *mut cma, start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong) -> bool;
    pub fn cma_reserve_pages_on_error(cma: *mut cma);
}

#[inline]
pub unsafe fn cma_declare_contiguous(
    base: phys_addr_t,
    size: phys_addr_t,
    limit: phys_addr_t,
    alignment: phys_addr_t,
    order_per_bit: ::core::ffi::c_uint,
    fixed: bool,
    name: *const ::core::ffi::c_char,
    res_cma: *mut *mut cma,
) -> ::core::ffi::c_int {
    cma_declare_contiguous_nid(
        base,
        size,
        limit,
        alignment,
        order_per_bit,
        fixed,
        name,
        res_cma,
        NUMA_NO_NODE,
    )
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
