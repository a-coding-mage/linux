/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/spinlock_types.h, linux/device.h, and asm/page.h.

pub const IOMMU_POOL_HASHBITS: usize = 4;
pub const IOMMU_NR_POOLS: usize = 1usize << IOMMU_POOL_HASHBITS;
pub const IOMMU_ERROR_CODE: usize = !0usize;

#[repr(C)]
pub struct iommu_pool {
    pub start: usize,
    pub end: usize,
    pub hint: usize,
    pub lock: spinlock_t,
}

#[repr(C)]
pub struct iommu_map_table {
    pub table_map_base: usize,
    pub table_shift: usize,
    pub nr_pools: usize,
    pub lazy_flush: Option<unsafe extern "C" fn(*mut iommu_map_table)>,
    pub poolsize: usize,
    pub pools: [iommu_pool; IOMMU_NR_POOLS],
    pub flags: u32,
    pub large_pool: iommu_pool,
    pub map: *mut usize,
}

pub const IOMMU_HAS_LARGE_POOL: u32 = 0x00000001;
pub const IOMMU_NO_SPAN_BOUND: u32 = 0x00000002;
pub const IOMMU_NEED_FLUSH: u32 = 0x00000004;

unsafe extern "C" {
    pub fn iommu_tbl_pool_init(
        iommu: *mut iommu_map_table,
        num_entries: usize,
        table_shift: u32,
        lazy_flush: Option<unsafe extern "C" fn(*mut iommu_map_table)>,
        large_pool: bool,
        npools: u32,
        skip_span_boundary_check: bool,
    );

    pub fn iommu_tbl_range_alloc(
        dev: *mut device,
        iommu: *mut iommu_map_table,
        npages: usize,
        handle: *mut usize,
        mask: usize,
        align_order: u32,
    ) -> usize;

    pub fn iommu_tbl_range_free(
        iommu: *mut iommu_map_table,
        dma_addr: u64,
        npages: usize,
        entry: usize,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
