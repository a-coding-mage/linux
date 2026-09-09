/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from x86/include/asm/pgtable_types.h. */

pub const _PAGE_BIT_PRESENT: u32 = 0;
pub const _PAGE_BIT_RW: u32 = 1;
pub const _PAGE_BIT_USER: u32 = 2;
pub const _PAGE_BIT_PWT: u32 = 3;
pub const _PAGE_BIT_PCD: u32 = 4;
pub const _PAGE_BIT_ACCESSED: u32 = 5;
pub const _PAGE_BIT_DIRTY: u32 = 6;
pub const _PAGE_BIT_PSE: u32 = 7;
pub const _PAGE_BIT_PAT: u32 = 7;
pub const _PAGE_BIT_GLOBAL: u32 = 8;
pub const _PAGE_BIT_SOFTW1: u32 = 9;
pub const _PAGE_BIT_SOFTW2: u32 = 10;
pub const _PAGE_BIT_SOFTW3: u32 = 11;
pub const _PAGE_BIT_PAT_LARGE: u32 = 12;
pub const _PAGE_BIT_SOFTW4: u32 = 57;
pub const _PAGE_BIT_SOFTW5: u32 = 58;
pub const _PAGE_BIT_PKEY_BIT0: u32 = 59;
pub const _PAGE_BIT_PKEY_BIT1: u32 = 60;
pub const _PAGE_BIT_PKEY_BIT2: u32 = 61;
pub const _PAGE_BIT_PKEY_BIT3: u32 = 62;
pub const _PAGE_BIT_NX: u32 = 63;
pub const _PAGE_BIT_SPECIAL: u32 = _PAGE_BIT_SOFTW1;
pub const _PAGE_BIT_CPA_TEST: u32 = _PAGE_BIT_SOFTW1;
pub const _PAGE_BIT_UFFD: u32 = _PAGE_BIT_SOFTW2;
pub const _PAGE_BIT_SOFT_DIRTY: u32 = _PAGE_BIT_SOFTW3;
pub const _PAGE_BIT_KERNEL_4K: u32 = _PAGE_BIT_SOFTW3;

#[cfg(target_pointer_width = "64")]
pub const _PAGE_BIT_SAVED_DIRTY: u32 = _PAGE_BIT_SOFTW5;
#[cfg(target_pointer_width = "64")]
pub const _PAGE_BIT_NOPTISHADOW: u32 = _PAGE_BIT_SOFTW5;
#[cfg(not(target_pointer_width = "64"))]
pub const _PAGE_BIT_SAVED_DIRTY: u32 = _PAGE_BIT_SOFTW2;
#[cfg(not(target_pointer_width = "64"))]
pub const _PAGE_BIT_NOPTISHADOW: u32 = _PAGE_BIT_SOFTW2;
pub const _PAGE_BIT_PROTNONE: u32 = _PAGE_BIT_GLOBAL;

macro_rules! page_bit { ($name:ident, $bit:ident) => { pub const $name: pteval_t = (1 as pteval_t) << $bit; }; }
page_bit!(_PAGE_PRESENT, _PAGE_BIT_PRESENT); page_bit!(_PAGE_RW, _PAGE_BIT_RW);
page_bit!(_PAGE_USER, _PAGE_BIT_USER); page_bit!(_PAGE_PWT, _PAGE_BIT_PWT);
page_bit!(_PAGE_PCD, _PAGE_BIT_PCD); page_bit!(_PAGE_ACCESSED, _PAGE_BIT_ACCESSED);
page_bit!(_PAGE_DIRTY, _PAGE_BIT_DIRTY); page_bit!(_PAGE_PSE, _PAGE_BIT_PSE);
page_bit!(_PAGE_GLOBAL, _PAGE_BIT_GLOBAL); page_bit!(_PAGE_SOFTW1, _PAGE_BIT_SOFTW1);
page_bit!(_PAGE_SOFTW2, _PAGE_BIT_SOFTW2); page_bit!(_PAGE_SOFTW3, _PAGE_BIT_SOFTW3);
page_bit!(_PAGE_PAT, _PAGE_BIT_PAT); page_bit!(_PAGE_PAT_LARGE, _PAGE_BIT_PAT_LARGE);
page_bit!(_PAGE_SPECIAL, _PAGE_BIT_SPECIAL); page_bit!(_PAGE_CPA_TEST, _PAGE_BIT_CPA_TEST);
page_bit!(_PAGE_KERNEL_4K, _PAGE_BIT_KERNEL_4K);

#[cfg(feature = "CONFIG_X86_INTEL_MEMORY_PROTECTION_KEYS")]
page_bit!(_PAGE_PKEY_BIT0, _PAGE_BIT_PKEY_BIT0);
#[cfg(feature = "CONFIG_X86_INTEL_MEMORY_PROTECTION_KEYS")]
page_bit!(_PAGE_PKEY_BIT1, _PAGE_BIT_PKEY_BIT1);
#[cfg(feature = "CONFIG_X86_INTEL_MEMORY_PROTECTION_KEYS")]
page_bit!(_PAGE_PKEY_BIT2, _PAGE_BIT_PKEY_BIT2);
#[cfg(feature = "CONFIG_X86_INTEL_MEMORY_PROTECTION_KEYS")]
page_bit!(_PAGE_PKEY_BIT3, _PAGE_BIT_PKEY_BIT3);
#[cfg(not(feature = "CONFIG_X86_INTEL_MEMORY_PROTECTION_KEYS"))]
pub const _PAGE_PKEY_BIT0: pteval_t = 0;
#[cfg(not(feature = "CONFIG_X86_INTEL_MEMORY_PROTECTION_KEYS"))]
pub const _PAGE_PKEY_BIT1: pteval_t = 0;
#[cfg(not(feature = "CONFIG_X86_INTEL_MEMORY_PROTECTION_KEYS"))]
pub const _PAGE_PKEY_BIT2: pteval_t = 0;
#[cfg(not(feature = "CONFIG_X86_INTEL_MEMORY_PROTECTION_KEYS"))]
pub const _PAGE_PKEY_BIT3: pteval_t = 0;

pub const _PAGE_PKEY_MASK: pteval_t = _PAGE_PKEY_BIT0 | _PAGE_PKEY_BIT1 | _PAGE_PKEY_BIT2 | _PAGE_PKEY_BIT3;
#[cfg(any(target_pointer_width = "64", feature = "CONFIG_X86_PAE"))]
pub const _PAGE_KNL_ERRATUM_MASK: pteval_t = _PAGE_DIRTY | _PAGE_ACCESSED;
#[cfg(not(any(target_pointer_width = "64", feature = "CONFIG_X86_PAE")))]
pub const _PAGE_KNL_ERRATUM_MASK: pteval_t = 0;
#[cfg(feature = "CONFIG_MEM_SOFT_DIRTY")]
pub const _PAGE_SOFT_DIRTY: pteval_t = (1 as pteval_t) << _PAGE_BIT_SOFT_DIRTY;
#[cfg(not(feature = "CONFIG_MEM_SOFT_DIRTY"))]
pub const _PAGE_SOFT_DIRTY: pteval_t = 0;
#[cfg(feature = "CONFIG_MEM_SOFT_DIRTY")]
pub const _PAGE_SWP_SOFT_DIRTY: pteval_t = _PAGE_RW;
#[cfg(not(feature = "CONFIG_MEM_SOFT_DIRTY"))]
pub const _PAGE_SWP_SOFT_DIRTY: pteval_t = 0;
#[cfg(feature = "CONFIG_HAVE_ARCH_USERFAULTFD_WP")]
pub const _PAGE_UFFD: pteval_t = (1 as pteval_t) << _PAGE_BIT_UFFD;
#[cfg(feature = "CONFIG_HAVE_ARCH_USERFAULTFD_WP")]
pub const _PAGE_SWP_UFFD: pteval_t = _PAGE_USER;
#[cfg(not(feature = "CONFIG_HAVE_ARCH_USERFAULTFD_WP"))]
pub const _PAGE_UFFD: pteval_t = 0;
#[cfg(not(feature = "CONFIG_HAVE_ARCH_USERFAULTFD_WP"))]
pub const _PAGE_SWP_UFFD: pteval_t = 0;
#[cfg(any(target_pointer_width = "64", feature = "CONFIG_X86_PAE"))]
pub const _PAGE_NX: pteval_t = (1 as pteval_t) << _PAGE_BIT_NX;
#[cfg(any(target_pointer_width = "64", feature = "CONFIG_X86_PAE"))]
pub const _PAGE_SOFTW4: pteval_t = (1 as pteval_t) << _PAGE_BIT_SOFTW4;
#[cfg(not(any(target_pointer_width = "64", feature = "CONFIG_X86_PAE")))]
pub const _PAGE_NX: pteval_t = 0;
#[cfg(not(any(target_pointer_width = "64", feature = "CONFIG_X86_PAE")))]
pub const _PAGE_SOFTW4: pteval_t = 0;
pub const _PAGE_SAVED_DIRTY: pteval_t = (1 as pteval_t) << _PAGE_BIT_SAVED_DIRTY;
pub const _PAGE_DIRTY_BITS: pteval_t = _PAGE_DIRTY | _PAGE_SAVED_DIRTY;
pub const _PAGE_PROTNONE: pteval_t = (1 as pteval_t) << _PAGE_BIT_PROTNONE;
pub const _PAGE_NOPTISHADOW: pteval_t = (1 as pteval_t) << _PAGE_BIT_NOPTISHADOW;

#[cfg(not(feature = "__ASSEMBLER__"))]
#[repr(C)]
#[derive(Copy, Clone)]
pub enum page_cache_mode { _PAGE_CACHE_MODE_WB = 0, _PAGE_CACHE_MODE_WC = 1, _PAGE_CACHE_MODE_UC_MINUS = 2, _PAGE_CACHE_MODE_UC = 3, _PAGE_CACHE_MODE_WT = 4, _PAGE_CACHE_MODE_WP = 5, _PAGE_CACHE_MODE_NUM = 8 }

pub const _PAGE_CACHE_MASK: pteval_t = _PAGE_PWT | _PAGE_PCD | _PAGE_PAT;
pub const _PAGE_LARGE_CACHE_MASK: pteval_t = _PAGE_PWT | _PAGE_PCD | _PAGE_PAT_LARGE;
pub const _PAGE_CC: pteval_t = cc_get_mask() as pteval_t;
pub const _PAGE_ENC: pteval_t = sme_me_mask as pteval_t;
pub const _PAGE_SPECIAL_MASK: pteval_t = _PAGE_SPECIAL;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pgprot_t { pub pgprot: pgprotval_t }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pgd_t { pub pgd: pgdval_t }

#[inline]
pub fn pgprot_val(x: pgprot_t) -> pgprotval_t { x.pgprot }
#[inline]
pub fn __pgprot(x: pgprotval_t) -> pgprot_t { pgprot_t { pgprot: x } }
#[inline]
pub fn __pg(x: pgprotval_t) -> pgprot_t { __pgprot(x) }
#[inline]
pub fn pgprot_nx(prot: pgprot_t) -> pgprot_t { __pgprot(pgprot_val(prot) | _PAGE_NX) }

pub const PGD_PAE_PAGE_MASK: i64 = PAGE_MASK as i64;
#[cfg(feature = "CONFIG_X86_PAE")]
pub const PGD_ALLOWED_BITS: u64 = PGD_PAE_PHYS_MASK | _PAGE_PRESENT as u64 | _PAGE_PWT as u64 | _PAGE_PCD as u64 | _PAGE_SOFTW1 as u64 | _PAGE_SOFTW2 as u64 | _PAGE_SOFTW3 as u64;
#[cfg(not(feature = "CONFIG_X86_PAE"))]
pub const PGD_ALLOWED_BITS: u64 = !0u64;
#[inline]
pub fn native_make_pgd(val: pgdval_t) -> pgd_t { pgd_t { pgd: val & PGD_ALLOWED_BITS as pgdval_t } }
#[inline]
pub fn native_pgd_val(pgd: pgd_t) -> pgdval_t { pgd.pgd & PGD_ALLOWED_BITS as pgdval_t }
#[inline]
pub fn pgd_flags(pgd: pgd_t) -> pgdval_t { native_pgd_val(pgd) & PTE_FLAGS_MASK as pgdval_t }

extern "C" {
    pub fn cc_get_mask() -> pteval_t;
    pub static mut sme_me_mask: pteval_t;
    pub static mut __supported_pte_mask: pteval_t;
    pub static mut __default_kernel_pte_mask: pteval_t;
    pub fn cachemode2protval(pcm: page_cache_mode) -> unsigned_long;
    pub fn pgprot_writecombine(prot: pgprot_t) -> pgprot_t;
    pub fn pgprot_writethrough(prot: pgprot_t) -> pgprot_t;
    pub fn phys_mem_access_prot(file: *mut file, pfn: unsigned_long, size: unsigned_long, vma_prot: pgprot_t) -> pgprot_t;
    pub fn set_pte_vaddr(vaddr: unsigned_long, pte: pte_t);
}

pub struct file;
pub struct page;
pub type pgtable_t = *mut page;
pub enum pg_level { PG_LEVEL_NONE, PG_LEVEL_4K, PG_LEVEL_2M, PG_LEVEL_1G, PG_LEVEL_512G, PG_LEVEL_256T, PG_LEVEL_NUM }

#[inline]
pub fn protval_4k_2_large(val: pgprotval_t) -> pgprotval_t { (val & !(_PAGE_PAT | _PAGE_PAT_LARGE) as pgprotval_t) | ((val & _PAGE_PAT) << (_PAGE_BIT_PAT_LARGE - _PAGE_BIT_PAT)) }
#[inline]
pub fn pgprot_4k_2_large(prot: pgprot_t) -> pgprot_t { __pgprot(protval_4k_2_large(pgprot_val(prot))) }
#[inline]
pub fn protval_large_2_4k(val: pgprotval_t) -> pgprotval_t { (val & !(_PAGE_PAT | _PAGE_PAT_LARGE) as pgprotval_t) | ((val & _PAGE_PAT_LARGE) >> (_PAGE_BIT_PAT_LARGE - _PAGE_BIT_PAT)) }
#[inline]
pub fn pgprot_large_2_4k(prot: pgprot_t) -> pgprot_t { __pgprot(protval_large_2_4k(pgprot_val(prot))) }

pub unsafe fn update_page_count(_level: i32, _pages: unsigned_long) {}

pub const __PP: pteval_t = _PAGE_PRESENT;
pub const __RW: pteval_t = _PAGE_RW;
pub const _USR: pteval_t = _PAGE_USER;
pub const ___A: pteval_t = _PAGE_ACCESSED;
pub const ___D: pteval_t = _PAGE_DIRTY;
pub const ___G: pteval_t = _PAGE_GLOBAL;
pub const __NX: pteval_t = _PAGE_NX;
pub const _ENC: pteval_t = _PAGE_ENC;
pub const _PSE: pteval_t = _PAGE_PSE;

pub const PTE_PFN_MASK: pteval_t = PHYSICAL_PAGE_MASK as pteval_t;
pub const PTE_FLAGS_MASK: pteval_t = !PTE_PFN_MASK;

#[inline]
pub fn native_make_p4d(val: pudval_t) -> p4d_t { p4d_t { p4d: val as p4dval_t } }
#[inline]
pub fn native_p4d_val(p4d: p4d_t) -> p4dval_t { p4d.p4d }
#[inline]
pub fn native_make_pud(val: pmdval_t) -> pud_t { pud_t { pud: val as pudval_t } }
#[inline]
pub fn native_pud_val(pud: pud_t) -> pudval_t { pud.pud }
#[inline]
pub fn native_make_pmd(val: pmdval_t) -> pmd_t { pmd_t { pmd: val } }
#[inline]
pub fn native_pmd_val(pmd: pmd_t) -> pmdval_t { pmd.pmd }
#[inline]
pub fn p4d_pfn_mask(_p4d: p4d_t) -> p4dval_t { PTE_PFN_MASK as p4dval_t }
#[inline]
pub fn p4d_flags_mask(p4d: p4d_t) -> p4dval_t { !p4d_pfn_mask(p4d) }
#[inline]
pub fn p4d_flags(p4d: p4d_t) -> p4dval_t { native_p4d_val(p4d) & p4d_flags_mask(p4d) }
#[inline]
pub fn pud_pfn_mask(pud: pud_t) -> pudval_t { if native_pud_val(pud) & _PAGE_PSE as pudval_t != 0 { PHYSICAL_PUD_PAGE_MASK as pudval_t } else { PTE_PFN_MASK as pudval_t } }
#[inline]
pub fn pud_flags_mask(pud: pud_t) -> pudval_t { !pud_pfn_mask(pud) }
#[inline]
pub fn pud_flags(pud: pud_t) -> pudval_t { native_pud_val(pud) & pud_flags_mask(pud) }
#[inline]
pub fn pmd_pfn_mask(pmd: pmd_t) -> pmdval_t { if native_pmd_val(pmd) & _PAGE_PSE as pmdval_t != 0 { PHYSICAL_PMD_PAGE_MASK as pmdval_t } else { PTE_PFN_MASK as pmdval_t } }
#[inline]
pub fn pmd_flags_mask(pmd: pmd_t) -> pmdval_t { !pmd_pfn_mask(pmd) }
#[inline]
pub fn pmd_flags(pmd: pmd_t) -> pmdval_t { native_pmd_val(pmd) & pmd_flags_mask(pmd) }
#[inline]
pub fn native_make_pte(val: pteval_t) -> pte_t { pte_t { pte: val } }
#[inline]
pub fn native_pte_val(pte: pte_t) -> pteval_t { pte.pte }
#[inline]
pub fn pte_flags(pte: pte_t) -> pteval_t { native_pte_val(pte) & PTE_FLAGS_MASK }

extern "C" {
    pub fn lookup_address(address: unsigned_long, level: *mut u32) -> *mut pte_t;
    pub fn lookup_address_in_pgd(pgd: *mut pgd_t, address: unsigned_long, level: *mut u32) -> *mut pte_t;
    pub fn lookup_address_in_pgd_attr(pgd: *mut pgd_t, address: unsigned_long, level: *mut u32, nx: *mut bool, rw: *mut bool) -> *mut pte_t;
    pub fn lookup_pmd_address(address: unsigned_long) -> *mut pmd_t;
    pub fn slow_virt_to_phys(address: *mut core::ffi::c_void) -> phys_addr_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
