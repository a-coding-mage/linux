/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/const.h, asm/page.h, and asm/hypervisor.h.

pub const CTX_NR_BITS: u32 = 13;
pub const TAG_CONTEXT_BITS: u64 = (1u64 << CTX_NR_BITS) - 1u64;

/* UltraSPARC-III+ and later have a feature whereby you can
 * select what page size the various Data-TLB instances in the
 * chip.  In order to gracefully support this, we put the version
 * field in a spot outside of the areas of the context register
 * where this parameter is specified.
 */
pub const CTX_VERSION_SHIFT: u32 = 22;
pub const CTX_VERSION_MASK: u64 = (!0u64) << CTX_VERSION_SHIFT;

pub const CTX_PGSZ_8KB: u64 = 0x0;
pub const CTX_PGSZ_64KB: u64 = 0x1;
pub const CTX_PGSZ_512KB: u64 = 0x2;
pub const CTX_PGSZ_4MB: u64 = 0x3;
pub const CTX_PGSZ_BITS: u64 = 0x7;
pub const CTX_PGSZ0_NUC_SHIFT: u32 = 61;
pub const CTX_PGSZ1_NUC_SHIFT: u32 = 58;
pub const CTX_PGSZ0_SHIFT: u32 = 16;
pub const CTX_PGSZ1_SHIFT: u32 = 19;
pub const CTX_PGSZ_MASK: u64 =
    (CTX_PGSZ_BITS << CTX_PGSZ0_SHIFT) | (CTX_PGSZ_BITS << CTX_PGSZ1_SHIFT);

pub const CTX_PGSZ_BASE: u64 = CTX_PGSZ_8KB;
pub const CTX_PGSZ_HUGE: u64 = CTX_PGSZ_4MB;
pub const CTX_PGSZ_KERN: u64 = CTX_PGSZ_4MB;

/* Thus, when running on UltraSPARC-III+ and later, we use the following
 * PRIMARY_CONTEXT register values for the kernel context.
 */
pub const CTX_CHEETAH_PLUS_NUC: u64 =
    (CTX_PGSZ_KERN << CTX_PGSZ0_NUC_SHIFT) | (CTX_PGSZ_BASE << CTX_PGSZ1_NUC_SHIFT);

pub const CTX_CHEETAH_PLUS_CTX0: u64 =
    (CTX_PGSZ_KERN << CTX_PGSZ0_SHIFT) | (CTX_PGSZ_BASE << CTX_PGSZ1_SHIFT);

/* If you want "the TLB context number" use CTX_NR_MASK.  If you
 * want "the bits I program into the context registers" use
 * CTX_HW_MASK.
 */
pub const CTX_NR_MASK: u64 = TAG_CONTEXT_BITS;
pub const CTX_HW_MASK: u64 = CTX_NR_MASK | CTX_PGSZ_MASK;
pub const CTX_FIRST_VERSION: u64 = 1u64 << CTX_VERSION_SHIFT;

#[macro_export]
macro_rules! CTX_VALID {
    ($ctx:expr, $tlb_context_cache:expr) => {
        !((($ctx.sparc64_ctx_val ^ $tlb_context_cache) & $crate::CTX_VERSION_MASK) != 0)
    };
}

#[macro_export]
macro_rules! CTX_HWBITS {
    ($ctx:expr) => { $ctx.sparc64_ctx_val & $crate::CTX_HW_MASK };
}

#[macro_export]
macro_rules! CTX_NRBITS {
    ($ctx:expr) => { $ctx.sparc64_ctx_val & $crate::CTX_NR_MASK };
}

pub const TSB_ENTRY_ALIGNMENT: usize = 16;

#[repr(C, align(16))]
pub struct tsb {
    pub tag: u64,
    pub pte: u64,
}

extern "C" {
    pub fn __tsb_insert(ent: u64, tag: u64, pte: u64);
    pub fn tsb_flush(ent: u64, tag: u64);
    pub fn tsb_init(tsb: *mut tsb, size: u64);
}

#[repr(C)]
pub struct tsb_config {
    pub tsb: *mut tsb,
    pub tsb_rss_limit: u64,
    pub tsb_nentries: u64,
    pub tsb_reg_val: u64,
    pub tsb_map_vaddr: u64,
    pub tsb_map_pte: u64,
}

pub const MM_TSB_BASE: usize = 0;

#[cfg(any(feature = "CONFIG_HUGETLB_PAGE", feature = "CONFIG_TRANSPARENT_HUGEPAGE"))]
pub const MM_TSB_HUGE: usize = 1;
#[cfg(any(feature = "CONFIG_HUGETLB_PAGE", feature = "CONFIG_TRANSPARENT_HUGEPAGE"))]
pub const MM_NUM_TSBS: usize = 2;
#[cfg(not(any(feature = "CONFIG_HUGETLB_PAGE", feature = "CONFIG_TRANSPARENT_HUGEPAGE")))]
pub const MM_NUM_TSBS: usize = 1;

/* ADI tags are stored when a page is swapped out and the storage for
 * tags is allocated dynamically. There is a tag storage descriptor
 * associated with each set of tag storage pages. Tag storage descriptors
 * are allocated dynamically. Since kernel will allocate a full page for
 * each tag storage descriptor, we can store up to
 * PAGE_SIZE/sizeof(tag storage descriptor) descriptors on that page.
 */
#[repr(C)]
pub struct tag_storage_desc_t {
    pub start: u64,                 /* Start address for this tag storage */
    pub end: u64,                   /* Last address for tag storage */
    pub tags: *mut u8,              /* Where the tags are */
    pub tag_users: u64,             /* number of references to descriptor */
}

#[repr(C)]
pub struct mm_context_t {
    pub lock: spinlock_t,
    pub sparc64_ctx_val: u64,
    pub hugetlb_pte_count: u64,
    pub thp_pte_count: u64,
    pub tsb_block: [tsb_config; MM_NUM_TSBS],
    pub tsb_descr: [hv_tsb_descr; MM_NUM_TSBS],
    pub vdso: *mut core::ffi::c_void,
    pub adi: bool,
    pub tag_store: *mut tag_storage_desc_t,
    pub tag_lock: spinlock_t,
}

pub const TSB_CONFIG_TSB: usize = 0x00;
pub const TSB_CONFIG_RSS_LIMIT: usize = 0x08;
pub const TSB_CONFIG_NENTRIES: usize = 0x10;
pub const TSB_CONFIG_REG_VAL: usize = 0x18;
pub const TSB_CONFIG_MAP_VADDR: usize = 0x20;
pub const TSB_CONFIG_MAP_PTE: usize = 0x28;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
