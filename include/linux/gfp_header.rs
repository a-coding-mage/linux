/* SPDX-License-Identifier: GPL-2.0 */
// Translated from linux/gfp.h. Dependencies are supplied by the surrounding kernel translation.

pub unsafe fn gfpflags_allow_blocking(gfp_flags: gfp_t) -> bool {
    (gfp_flags & __GFP_DIRECT_RECLAIM) != 0
}

pub unsafe fn gfpflags_allow_spinning(gfp_flags: gfp_t) -> bool {
    /* !__GFP_DIRECT_RECLAIM forbids direct reclaim; !__GFP_KSWAPD_RECLAIM
     * forbids waking kswapd. This permits only the reclaim modes that may spin. */
    (gfp_flags & __GFP_RECLAIM) != 0
}

#[cfg(feature = "CONFIG_HIGHMEM")]
pub const OPT_ZONE_HIGHMEM: i32 = ZONE_HIGHMEM;
#[cfg(not(feature = "CONFIG_HIGHMEM"))]
pub const OPT_ZONE_HIGHMEM: i32 = ZONE_NORMAL;
#[cfg(feature = "CONFIG_ZONE_DMA")]
pub const OPT_ZONE_DMA: i32 = ZONE_DMA;
#[cfg(not(feature = "CONFIG_ZONE_DMA"))]
pub const OPT_ZONE_DMA: i32 = ZONE_NORMAL;
#[cfg(feature = "CONFIG_ZONE_DMA32")]
pub const OPT_ZONE_DMA32: i32 = ZONE_DMA32;
#[cfg(not(feature = "CONFIG_ZONE_DMA32"))]
pub const OPT_ZONE_DMA32: i32 = ZONE_NORMAL;

// Build-time CONFIG_ZONE_DEVICE condition preserved via cfg; ZONES_SHIFT and related
// constants are supplied by the included kernel headers.
#[cfg(all(feature = "CONFIG_ZONE_DEVICE", feature = "MAX_NR_ZONES_LE_5"))]
pub const GFP_ZONES_SHIFT: i32 = 2;
#[cfg(not(all(feature = "CONFIG_ZONE_DEVICE", feature = "MAX_NR_ZONES_LE_5")))]
pub const GFP_ZONES_SHIFT: i32 = ZONES_SHIFT;

pub const GFP_ZONE_TABLE: usize =
    (ZONE_NORMAL << (0 * GFP_ZONES_SHIFT))
    | (OPT_ZONE_DMA << (___GFP_DMA * GFP_ZONES_SHIFT))
    | (OPT_ZONE_HIGHMEM << (___GFP_HIGHMEM * GFP_ZONES_SHIFT))
    | (OPT_ZONE_DMA32 << (___GFP_DMA32 * GFP_ZONES_SHIFT))
    | (ZONE_NORMAL << (___GFP_MOVABLE * GFP_ZONES_SHIFT))
    | (OPT_ZONE_DMA << ((___GFP_MOVABLE | ___GFP_DMA) * GFP_ZONES_SHIFT))
    | (ZONE_MOVABLE << ((___GFP_MOVABLE | ___GFP_HIGHMEM) * GFP_ZONES_SHIFT))
    | (OPT_ZONE_DMA32 << ((___GFP_MOVABLE | ___GFP_DMA32) * GFP_ZONES_SHIFT));

pub const GFP_ZONE_BAD: usize =
    (1 << (___GFP_DMA | ___GFP_HIGHMEM))
    | (1 << (___GFP_DMA | ___GFP_DMA32))
    | (1 << (___GFP_DMA32 | ___GFP_HIGHMEM))
    | (1 << (___GFP_DMA | ___GFP_DMA32 | ___GFP_HIGHMEM))
    | (1 << (___GFP_MOVABLE | ___GFP_HIGHMEM | ___GFP_DMA))
    | (1 << (___GFP_MOVABLE | ___GFP_DMA32 | ___GFP_DMA))
    | (1 << (___GFP_MOVABLE | ___GFP_DMA32 | ___GFP_HIGHMEM))
    | (1 << (___GFP_MOVABLE | ___GFP_DMA32 | ___GFP_DMA | ___GFP_HIGHMEM));

pub unsafe fn gfp_zone(flags: gfp_t) -> zone_type {
    let bit: i32 = (flags & GFP_ZONEMASK) as i32;
    let z = (GFP_ZONE_TABLE >> (bit * GFP_ZONES_SHIFT))
        & ((1 << GFP_ZONES_SHIFT) - 1);
    VM_BUG_ON((GFP_ZONE_BAD >> bit) & 1);
    z as zone_type
}

pub unsafe fn gfp_zonelist(flags: gfp_t) -> i32 {
    #[cfg(feature = "CONFIG_NUMA")]
    if unlikely((flags & __GFP_THISNODE) != 0) { return ZONELIST_NOFALLBACK; }
    ZONELIST_FALLBACK
}

pub unsafe fn gfp_nested_mask(flags: gfp_t) -> gfp_t {
    (flags & (GFP_KERNEL | GFP_ATOMIC | __GFP_NOLOCKDEP))
        | (__GFP_NORETRY | __GFP_NOMEMALLOC | __GFP_NOWARN)
}

pub unsafe fn node_zonelist(nid: i32, flags: gfp_t) -> *mut zonelist {
    (*NODE_DATA(nid)).node_zonelists.add(gfp_zonelist(flags) as usize)
}

#[cfg(not(feature = "HAVE_ARCH_FREE_PAGE"))]
pub unsafe fn arch_free_page(_page: *mut page, _order: i32) {}
#[cfg(not(feature = "HAVE_ARCH_ALLOC_PAGE"))]
pub unsafe fn arch_alloc_page(_page: *mut page, _order: i32) {}

extern "C" {
    pub fn __folio_alloc_noprof(gfp: gfp_t, order: u32, preferred_nid: i32, nodemask: *mut nodemask_t) -> *mut folio;
    pub fn alloc_pages_bulk_noprof(gfp: gfp_t, preferred_nid: i32, nodemask: *mut nodemask_t, nr_pages: i32, page_array: *mut *mut page) -> u64;
    pub fn free_pages_bulk(page_array: *mut *mut page, nr_pages: u64);
    pub fn alloc_pages_bulk_mempolicy_noprof(gfp: gfp_t, nr_pages: u64, page_array: *mut *mut page) -> u64;
    pub fn alloc_pages_bulk_node_noprof(gfp: gfp_t, nid: i32, nr_pages: u64, page_array: *mut *mut page) -> u64;
    pub fn alloc_pages_node_noprof(nid: i32, gfp_mask: gfp_t, order: u32) -> *mut page;
    pub fn alloc_pages_nolock_noprof(gfp_flags: gfp_t, nid: i32, order: u32) -> *mut page;
    pub fn get_free_pages_noprof(gfp_mask: gfp_t, order: u32) -> u64;
    pub fn get_zeroed_page_noprof(gfp_mask: gfp_t) -> u64;
    pub fn alloc_pages_exact_noprof(size: usize, gfp_mask: gfp_t) -> *mut core::ffi::c_void;
    pub fn free_pages_exact(virt: *mut core::ffi::c_void, size: usize);
    pub fn alloc_pages_exact_nid_noprof(nid: i32, size: usize, gfp_mask: gfp_t) -> *mut core::ffi::c_void;
    pub fn __free_pages(page: *mut page, order: u32);
    pub fn free_pages_nolock(page: *mut page, order: u32);
    pub fn free_pages(addr: u64, order: u32);
    pub fn drain_local_pages(zone: *mut zone);
    pub fn page_alloc_init_late();
    pub fn setup_pcp_cacheinfo(cpu: u32);
    pub static mut gfp_allowed_mask: gfp_t;
    pub fn gfp_pfmemalloc_allowed(gfp_mask: gfp_t) -> bool;
    pub fn vma_thp_gfp_mask(vma: *mut vm_area_struct) -> gfp_t;
    pub fn __free_contig_range(pfn: u64, nr_pages: u64);
}

pub unsafe fn gfp_has_flags(gfp: gfp_t, flags: gfp_t) -> bool { (gfp & flags) == flags }
pub unsafe fn gfp_has_io_fs(gfp: gfp_t) -> bool { gfp_has_flags(gfp, __GFP_IO | __GFP_FS) }
pub unsafe fn gfp_compaction_allowed(gfp_mask: gfp_t) -> bool {
    IS_ENABLED(CONFIG_COMPACTION) && (gfp_mask & __GFP_IO) != 0
}

// The remaining allocation APIs are declaration-only in the source header.
extern "C" {
    pub fn alloc_pages_noprof(gfp: gfp_t, order: u32) -> *mut page;
    pub fn folio_alloc_noprof(gfp: gfp_t, order: u32) -> *mut folio;
    pub fn folio_alloc_mpol_noprof(gfp: gfp_t, order: u32, mpol: *mut mempolicy, ilx: pgoff_t, nid: i32) -> *mut folio;
    pub fn vma_alloc_folio_noprof(gfp: gfp_t, order: i32, vma: *mut vm_area_struct, addr: u64) -> *mut folio;
    pub fn alloc_contig_frozen_range_noprof(start: u64, end: u64, alloc_flags: acr_flags_t, gfp_mask: gfp_t) -> i32;
    pub fn alloc_contig_range_noprof(start: u64, end: u64, alloc_flags: acr_flags_t, gfp_mask: gfp_t) -> i32;
    pub fn alloc_contig_frozen_pages_noprof(nr_pages: u64, gfp_mask: gfp_t, nid: i32, nodemask: *mut nodemask_t) -> *mut page;
    pub fn alloc_contig_pages_noprof(nr_pages: u64, gfp_mask: gfp_t, nid: i32, nodemask: *mut nodemask_t) -> *mut page;
    pub fn free_contig_frozen_range(pfn: u64, nr_pages: u64);
    pub fn free_contig_range(pfn: u64, nr_pages: u64);
}

pub type acr_flags_t = u32;
pub const ACR_FLAGS_NONE: acr_flags_t = 0;
pub const ACR_FLAGS_CMA: acr_flags_t = BIT(0);

pub unsafe fn alloc_pages_bulk_node_noprof_inline(gfp: gfp_t, mut nid: i32, nr_pages: u64, page_array: *mut *mut page) -> u64 {
    if nid == NUMA_NO_NODE { nid = numa_mem_id(); }
    alloc_pages_bulk_noprof(gfp, nid, core::ptr::null_mut(), nr_pages as i32, page_array)
}

pub unsafe fn warn_if_node_offline(this_node: i32, gfp_mask: gfp_t) {
    let warn_gfp = gfp_mask & (__GFP_THISNODE | __GFP_NOWARN);
    if warn_gfp != (__GFP_THISNODE | __GFP_NOWARN) || node_online(this_node) { return; }
    pr_warn("%pGg allocation from offline node %d\n", &gfp_mask, this_node);
    dump_stack();
}

pub unsafe fn __folio_alloc_node_noprof_inline(gfp: gfp_t, order: u32, nid: i32) -> *mut folio {
    warn_if_node_offline(nid, gfp);
    __folio_alloc_noprof(gfp, order, nid, core::ptr::null_mut())
}

pub unsafe fn alloc_page_vma_noprof(gfp: gfp_t, vma: *mut vm_area_struct, addr: u64) -> *mut page {
    let folio = vma_alloc_folio_noprof(gfp, 0, vma, addr);
    &mut (*folio).page
}

#[cfg(not(feature = "CONFIG_NUMA"))]
pub unsafe fn alloc_pages_noprof_numa_fallback(gfp_mask: gfp_t, order: u32) -> *mut page {
    alloc_pages_node_noprof(numa_node_id(), gfp_mask, order)
}
#[cfg(not(feature = "CONFIG_NUMA"))]
pub unsafe fn folio_alloc_noprof_numa_fallback(gfp: gfp_t, order: u32) -> *mut folio {
    __folio_alloc_node_noprof_inline(gfp, order, numa_node_id())
}
#[cfg(not(feature = "CONFIG_NUMA"))]
pub unsafe fn folio_alloc_mpol_noprof_numa_fallback(gfp: gfp_t, order: u32, _mpol: *mut mempolicy, _ilx: pgoff_t, _nid: i32) -> *mut folio {
    folio_alloc_noprof_numa_fallback(gfp, order)
}
#[cfg(not(feature = "CONFIG_NUMA"))]
pub unsafe fn vma_alloc_folio_noprof_numa_fallback(gfp: gfp_t, order: i32, _vma: *mut vm_area_struct, _addr: u64) -> *mut folio {
    folio_alloc_noprof_numa_fallback(gfp, order as u32)
}

pub const fn __get_free_page(gfp_mask: gfp_t) -> u64 { 0 /* macro expansion requires alloc_hooks */ }
pub const fn __free_page(_page: *mut page) {}
pub const fn free_page(_addr: u64) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
