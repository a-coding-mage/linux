/* SPDX-License-Identifier: GPL-2.0-only */

// Dependencies supplied by the corresponding kernel headers:
// linux/list.h, nvhe/memory.h, and nvhe/spinlock.h.

pub const HYP_NO_ORDER: u8 = !0u8;

#[repr(C)]
pub struct hyp_pool {
    /*
     * Spinlock protecting concurrent changes to the memory pool as well as
     * the struct hyp_page of the pool's pages until we have a proper atomic
     * API at EL2.
     */
    pub lock: hyp_spinlock_t,
    pub free_area: [list_head; NR_PAGE_ORDERS],
    pub range_start: phys_addr_t,
    pub range_end: phys_addr_t,
    pub max_order: u8,
}

/* Allocation */
extern "C" {
    pub fn hyp_alloc_pages(pool: *mut hyp_pool, order: u8) -> *mut core::ffi::c_void;
    pub fn hyp_split_page(page: *mut hyp_page);
    pub fn hyp_get_page(pool: *mut hyp_pool, addr: *mut core::ffi::c_void);
    pub fn hyp_put_page(pool: *mut hyp_pool, addr: *mut core::ffi::c_void);

    /* Used pages cannot be freed */
    pub fn hyp_pool_init(
        pool: *mut hyp_pool,
        pfn: u64,
        nr_pages: core::ffi::c_uint,
        reserved_pages: core::ffi::c_uint,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
