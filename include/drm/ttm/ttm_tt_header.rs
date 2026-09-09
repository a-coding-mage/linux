/*
 * Translation of ttm_tt.h. Linux header dependencies and externally defined
 * types are intentionally left as external Rust dependencies.
 */

use core::ffi::c_ulong;

#[repr(C)]
pub struct ttm_tt {
    /** @pages: Array of pages backing the data. */
    pub pages: *mut *mut page,
    /** @page_flags: The page flags. */
    pub page_flags: u32,
    /** @num_pages: Number of pages in the page array. */
    pub num_pages: u32,
    /** @sg: for SG objects via dma-buf. */
    pub sg: *mut sg_table,
    /** @dma_address: The DMA (bus) addresses of the pages. */
    pub dma_address: *mut dma_addr_t,
    /** @swap_storage: Pointer to shmem struct file for swap storage. */
    pub swap_storage: *mut file,
    /** @backup: Pointer to backup struct for backed up tts. */
    pub backup: *mut file,
    /** @caching: The current caching state of the pages. */
    pub caching: ttm_caching,
    /** @restore: Partial restoration from backup state. TTM private */
    pub restore: *mut ttm_pool_tt_restore,
}

pub const TTM_TT_FLAG_SWAPPED: u32 = 1 << 0;
pub const TTM_TT_FLAG_ZERO_ALLOC: u32 = 1 << 1;
pub const TTM_TT_FLAG_EXTERNAL: u32 = 1 << 2;
pub const TTM_TT_FLAG_EXTERNAL_MAPPABLE: u32 = 1 << 3;
pub const TTM_TT_FLAG_DECRYPTED: u32 = 1 << 4;
pub const TTM_TT_FLAG_BACKED_UP: u32 = 1 << 5;
pub const TTM_TT_FLAG_PRIV_POPULATED: u32 = 1 << 6;

#[repr(C)]
pub struct ttm_kmap_iter_tt {
    pub base: ttm_kmap_iter,
    pub tt: *mut ttm_tt,
    pub prot: pgprot_t,
}

pub unsafe fn ttm_tt_is_populated(tt: *mut ttm_tt) -> bool {
    (*tt).page_flags & TTM_TT_FLAG_PRIV_POPULATED != 0
}

pub unsafe fn ttm_tt_is_swapped(tt: *const ttm_tt) -> bool {
    (*tt).page_flags & (TTM_TT_FLAG_SWAPPED | TTM_TT_FLAG_BACKED_UP) != 0
}

pub unsafe fn ttm_tt_is_backed_up(tt: *const ttm_tt) -> bool {
    (*tt).page_flags & TTM_TT_FLAG_BACKED_UP != 0
}

pub unsafe fn ttm_tt_clear_backed_up(tt: *mut ttm_tt) {
    (*tt).page_flags &= !TTM_TT_FLAG_BACKED_UP;
}

extern "C" {
    pub fn ttm_tt_create(bo: *mut ttm_buffer_object, zero_alloc: bool) -> i32;
    pub fn ttm_tt_init(
        ttm: *mut ttm_tt, bo: *mut ttm_buffer_object, page_flags: u32,
        caching: ttm_caching, extra_pages: c_ulong,
    ) -> i32;
    pub fn ttm_sg_tt_init(
        ttm_dma: *mut ttm_tt, bo: *mut ttm_buffer_object, page_flags: u32,
        caching: ttm_caching,
    ) -> i32;
    pub fn ttm_tt_fini(ttm: *mut ttm_tt);
    pub fn ttm_tt_destroy(bdev: *mut ttm_device, ttm: *mut ttm_tt);
    pub fn ttm_tt_swapin(ttm: *mut ttm_tt) -> i32;
    pub fn ttm_tt_swapout(bdev: *mut ttm_device, ttm: *mut ttm_tt, gfp_flags: gfp_t) -> i32;
    pub fn ttm_tt_populate(
        bdev: *mut ttm_device, ttm: *mut ttm_tt, ctx: *mut ttm_operation_ctx,
    ) -> i32;
    pub fn ttm_tt_unpopulate(bdev: *mut ttm_device, ttm: *mut ttm_tt);
    pub fn ttm_tt_mgr_init(num_pages: c_ulong, num_dma32_pages: c_ulong);
    pub fn ttm_kmap_iter_tt_init(iter_tt: *mut ttm_kmap_iter_tt, tt: *mut ttm_tt)
        -> *mut ttm_kmap_iter;
    pub fn ttm_tt_pages_limit() -> c_ulong;
    pub fn ttm_tt_backup(
        bdev: *mut ttm_device, tt: *mut ttm_tt, flags: ttm_backup_flags,
    ) -> isize;
    pub fn ttm_tt_restore(
        bdev: *mut ttm_device, tt: *mut ttm_tt, ctx: *const ttm_operation_ctx,
    ) -> i32;
    pub fn ttm_tt_setup_backup(tt: *mut ttm_tt) -> i32;
}

pub unsafe fn ttm_tt_mark_for_clear(ttm: *mut ttm_tt) {
    (*ttm).page_flags |= TTM_TT_FLAG_ZERO_ALLOC;
}

#[repr(C)]
pub struct ttm_backup_flags {
    /* C bit-fields: purge:1, writeback:1. */
    pub purge: u32,
    pub writeback: u32,
}

/* CONFIG_AGP conditional declarations are preserved as an intentional build-time dependency. */
#[cfg(feature = "CONFIG_AGP")]
extern "C" {
    pub fn ttm_agp_tt_create(
        bo: *mut ttm_buffer_object, bridge: *mut agp_bridge_data, page_flags: u32,
    ) -> *mut ttm_tt;
    pub fn ttm_agp_bind(ttm: *mut ttm_tt, bo_mem: *mut ttm_resource) -> i32;
    pub fn ttm_agp_unbind(ttm: *mut ttm_tt);
    pub fn ttm_agp_destroy(ttm: *mut ttm_tt);
    pub fn ttm_agp_is_bound(ttm: *mut ttm_tt) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
