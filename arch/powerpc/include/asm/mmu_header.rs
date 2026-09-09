/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from the PowerPC MMU header. Kernel configuration gates are
 * represented with Rust cfg attributes where applicable. */

/* MMU features bit definitions. */
pub const MMU_FTR_HPTE_TABLE: u64 = 0x0000_0001;
pub const MMU_FTR_TYPE_8xx: u64 = 0x0000_0002;
pub const MMU_FTR_TYPE_44x: u64 = 0x0000_0008;
pub const MMU_FTR_TYPE_FSL_E: u64 = 0x0000_0010;
pub const MMU_FTR_TYPE_47x: u64 = 0x0000_0020;
pub const MMU_FTR_TYPE_RADIX: u64 = 0x0000_0040;
pub const MMU_FTR_KUAP: u64 = 0x0000_0200;
pub const MMU_FTR_BOOK3S_KUEP: u64 = 0x0000_0400;
pub const MMU_FTR_PKEY: u64 = 0x0000_0800;
pub const MMU_FTR_GTSE: u64 = 0x0000_1000;
pub const MMU_FTR_68_BIT_VA: u64 = 0x0000_2000;
pub const MMU_FTR_KERNEL_RO: u64 = 0x0000_4000;
pub const MMU_FTR_TLBIE_CROP_VA: u64 = 0x0000_8000;
pub const MMU_FTR_USE_HIGH_BATS: u64 = 0x0001_0000;
pub const MMU_FTR_BIG_PHYS: u64 = 0x0002_0000;
pub const MMU_FTR_USE_TLBIVAX_BCAST: u64 = 0x0004_0000;
pub const MMU_FTR_USE_TLBILX: u64 = 0x0008_0000;
pub const MMU_FTR_LOCK_BCAST_INVAL: u64 = 0x0010_0000;
pub const MMU_FTR_NEED_DTLB_SW_LRU: u64 = 0x0020_0000;
pub const MMU_FTR_NO_SLBIE_B: u64 = 0x0200_0000;
pub const MMU_FTR_16M_PAGE: u64 = 0x0400_0000;
pub const MMU_FTR_TLBIEL: u64 = 0x0800_0000;
pub const MMU_FTR_LOCKLESS_TLBIE: u64 = 0x1000_0000;
pub const MMU_FTR_CI_LARGE_PAGE: u64 = 0x2000_0000;
pub const MMU_FTR_1T_SEGMENT: u64 = 0x4000_0000;
pub const MMU_FTR_NX_DSI: u64 = 0x8000_0000;

pub const MMU_FTRS_DEFAULT_HPTE_ARCH_V2: u64 = MMU_FTR_HPTE_TABLE | MMU_FTR_TLBIEL | MMU_FTR_16M_PAGE;
pub const MMU_FTRS_POWER: u64 = MMU_FTRS_DEFAULT_HPTE_ARCH_V2;
pub const MMU_FTRS_PPC970: u64 = MMU_FTRS_POWER | MMU_FTR_TLBIE_CROP_VA;
pub const MMU_FTRS_POWER5: u64 = MMU_FTRS_POWER | MMU_FTR_LOCKLESS_TLBIE;
pub const MMU_FTRS_POWER6: u64 = MMU_FTRS_POWER5 | MMU_FTR_KERNEL_RO | MMU_FTR_68_BIT_VA;
pub const MMU_FTRS_POWER7: u64 = MMU_FTRS_POWER6;
pub const MMU_FTRS_POWER8: u64 = MMU_FTRS_POWER6;
pub const MMU_FTRS_POWER9: u64 = MMU_FTRS_POWER6;
pub const MMU_FTRS_POWER10: u64 = MMU_FTRS_POWER6;
pub const MMU_FTRS_POWER11: u64 = MMU_FTRS_POWER6;
pub const MMU_FTRS_POWER12: u64 = MMU_FTRS_POWER6;
pub const MMU_FTRS_CELL: u64 = MMU_FTRS_DEFAULT_HPTE_ARCH_V2 | MMU_FTR_CI_LARGE_PAGE;
pub const MMU_FTRS_PA6T: u64 = MMU_FTRS_DEFAULT_HPTE_ARCH_V2 | MMU_FTR_CI_LARGE_PAGE | MMU_FTR_NO_SLBIE_B;

/* External kernel dependencies. */
pub type Pgtable = *mut Pte;
pub type Pte = core::ffi::c_void;

#[cfg(feature = "ppc_book3s_604")]
pub const MMU_FTRS_POSSIBLE_BOOK3S_604: u64 = MMU_FTR_HPTE_TABLE;
#[cfg(feature = "ppc_8xx")]
pub const MMU_FTRS_POSSIBLE_8XX: u64 = MMU_FTR_TYPE_8xx;
#[cfg(feature = "ppc_47x")]
pub const MMU_FTRS_POSSIBLE_47X: u64 = MMU_FTR_TYPE_47x | MMU_FTR_USE_TLBIVAX_BCAST | MMU_FTR_LOCK_BCAST_INVAL;
#[cfg(feature = "config_44x")]
pub const MMU_FTRS_POSSIBLE_44X: u64 = MMU_FTR_TYPE_44x;
#[cfg(feature = "ppc_e500")]
pub const MMU_FTRS_POSSIBLE_E500: u64 = MMU_FTR_TYPE_FSL_E | MMU_FTR_BIG_PHYS | MMU_FTR_USE_TLBILX;
#[cfg(feature = "ppc_book3s_32")]
pub const MMU_FTRS_POSSIBLE_BOOK3S_32: u64 = MMU_FTR_USE_HIGH_BATS;
#[cfg(feature = "ppc_83xx")]
pub const MMU_FTRS_POSSIBLE_83XX: u64 = MMU_FTR_NEED_DTLB_SW_LRU;

/* The C conditional accumulation is configuration-specific; this preserves
 * its intent as a single feature mask assembled from enabled options. */
pub const MMU_FTRS_POSSIBLE: u64 = 0;

pub const MMU_FTRS_ALWAYS: u64 = 0;

pub const MMU_PAGE_4K: usize = 0;
pub const MMU_PAGE_16K: usize = 1;
pub const MMU_PAGE_64K: usize = 2;
pub const MMU_PAGE_64K_AP: usize = 3; /* "Admixed pages" (hash64 only) */
pub const MMU_PAGE_256K: usize = 4;
pub const MMU_PAGE_512K: usize = 5;
pub const MMU_PAGE_1M: usize = 6;
pub const MMU_PAGE_2M: usize = 7;
pub const MMU_PAGE_4M: usize = 8;
pub const MMU_PAGE_8M: usize = 9;
pub const MMU_PAGE_16M: usize = 10;
pub const MMU_PAGE_64M: usize = 11;
pub const MMU_PAGE_256M: usize = 12;
pub const MMU_PAGE_1G: usize = 13;
pub const MMU_PAGE_16G: usize = 14;
pub const MMU_PAGE_64G: usize = 15;
pub const MMU_PAGE_COUNT: usize = 16;

extern "C" {
    pub static mut cur_cpu_spec: CpuSpec;
    pub static mut rodata_enabled: bool;
    pub static mut ppc64_rma_size: u64;
    pub static mut __start___mmu_ftr_fixup: u32;
    pub static mut __stop___mmu_ftr_fixup: u32;
    pub fn early_init_mmu();
    pub fn early_init_mmu_secondary();
    pub fn setup_initial_memory_limit(first_memblock_base: usize, first_memblock_size: usize);
    pub fn mmu_cleanup_all();
    pub fn radix__mmu_cleanup_all();
    pub fn mmu_partition_table_init();
    pub fn mmu_partition_table_set_entry(lpid: u32, dw0: usize, dw1: usize, flush: bool);
}

#[repr(C)]
pub struct CpuSpec {
    pub mmu_features: u64,
}

#[inline(always)]
pub unsafe fn early_mmu_has_feature(feature: u64) -> bool {
    if MMU_FTRS_ALWAYS & feature != 0 {
        return true;
    }
    (MMU_FTRS_POSSIBLE & (*core::ptr::addr_of!(cur_cpu_spec)).mmu_features & feature) != 0
}

#[inline(always)]
pub unsafe fn mmu_feature_keys_init() {}

#[inline(always)]
pub unsafe fn mmu_has_feature(feature: u64) -> bool {
    early_mmu_has_feature(feature)
}

#[inline]
pub unsafe fn mmu_clear_feature(feature: u64) {
    (*core::ptr::addr_of_mut!(cur_cpu_spec)).mmu_features &= !feature;
}

pub fn radix_enabled() -> bool { unsafe { mmu_has_feature(MMU_FTR_TYPE_RADIX) } }
pub fn early_radix_enabled() -> bool { unsafe { early_mmu_has_feature(MMU_FTR_TYPE_RADIX) } }

pub fn strict_kernel_rwx_enabled() -> bool {
    cfg!(feature = "strict_kernel_rwx") && unsafe { rodata_enabled }
}

pub fn strict_module_rwx_enabled() -> bool {
    cfg!(feature = "strict_module_rwx") && strict_kernel_rwx_enabled()
}

pub fn mmu_early_init_devtree() {}
pub fn pkey_early_init_devtree() {}
pub fn assert_pte_locked(_mm: *mut MmStruct, _addr: usize) {}

#[repr(C)]
pub struct MmStruct {
    _private: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
