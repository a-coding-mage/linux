/* SPDX-License-Identifier: GPL-2.0 */

pub const FIXADDR_START: usize = 0xffc8_0000;
pub const FIXADDR_END: usize = 0xfff0_0000;
pub const FIXADDR_TOP: usize = FIXADDR_END - PAGE_SIZE;

// Dependencies supplied by the corresponding Linux headers:
// linux/pgtable.h and asm/kmap_size.h.

#[repr(usize)]
pub enum fixed_addresses {
    FIX_EARLYCON_MEM_BASE = 0,
    __end_of_permanent_fixed_addresses,

    FIX_KMAP_BEGIN = __end_of_permanent_fixed_addresses,
    FIX_KMAP_END = FIX_KMAP_BEGIN + (KM_MAX_IDX * NR_CPUS) - 1,

    /* Support writing RO kernel text via kprobes, jump labels, etc. */
    FIX_TEXT_POKE0,
    FIX_TEXT_POKE1,

    __end_of_fixmap_region,

    /*
     * Share the kmap() region with early_ioremap(): this is guaranteed
     * not to clash since early_ioremap() is only available before
     * paging_init(), and kmap() only after.
     */
    FIX_BTMAP_END = __end_of_permanent_fixed_addresses,
    FIX_BTMAP_BEGIN = FIX_BTMAP_END + TOTAL_FIX_BTMAPS - 1,
    __end_of_early_ioremap_region,
}

pub const NR_FIX_BTMAPS: usize = 32;
pub const FIX_BTMAPS_SLOTS: usize = 7;
pub const TOTAL_FIX_BTMAPS: usize = NR_FIX_BTMAPS * FIX_BTMAPS_SLOTS;

pub const __end_of_fixed_addresses: fixed_addresses =
    if (__end_of_fixmap_region as usize) > (__end_of_early_ioremap_region as usize) {
        __end_of_fixmap_region
    } else {
        __end_of_early_ioremap_region
    };

pub const FIXMAP_PAGE_COMMON: pgprot_t =
    L_PTE_YOUNG | L_PTE_PRESENT | L_PTE_XN | L_PTE_DIRTY;

pub const FIXMAP_PAGE_NORMAL: pgprot_t = pgprot_kernel | L_PTE_XN;
pub const FIXMAP_PAGE_RO: pgprot_t = FIXMAP_PAGE_NORMAL | L_PTE_RDONLY;

/* Used by set_fixmap_(io|nocache), both meant for mapping a device */
pub const FIXMAP_PAGE_IO: pgprot_t =
    FIXMAP_PAGE_COMMON | L_PTE_MT_DEV_SHARED | L_PTE_SHARED;
pub const FIXMAP_PAGE_NOCACHE: pgprot_t = FIXMAP_PAGE_IO;

// #define __early_set_fixmap __set_fixmap
pub use __set_fixmap as __early_set_fixmap;

#[cfg(feature = "MMU")]
extern "C" {
    pub fn __set_fixmap(idx: fixed_addresses, phys: phys_addr_t, prot: pgprot_t);
    pub fn early_fixmap_init();
}

#[cfg(not(feature = "MMU"))]
#[inline]
pub fn early_fixmap_init() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
