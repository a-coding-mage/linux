/* SPDX-License-Identifier: GPL-2.0 */

pub const H_PTE_INDEX_SIZE: usize = 8;
pub const H_PMD_INDEX_SIZE: usize = 10;
pub const H_PUD_INDEX_SIZE: usize = 10;
pub const H_PGD_INDEX_SIZE: usize = 8;

/* CONFIG_SPARSEMEM_VMEMMAP && CONFIG_SPARSEMEM_EXTREME selects 51; otherwise 46. */
pub const H_MAX_PHYSMEM_BITS: usize = 46;
pub const MAX_EA_BITS_PER_CONTEXT: usize = 49;
pub const REGION_SHIFT: usize = MAX_EA_BITS_PER_CONTEXT;
pub const H_KERN_MAP_SIZE: usize = 1usize << MAX_EA_BITS_PER_CONTEXT;
pub const H_KERN_VIRT_START: u64 = 0xc008_0000_0000_0000;

pub const H_PAGE_COMBO: _RPAGE_RPN0_TYPE = _RPAGE_RPN0;
pub const H_PAGE_4K_PFN: _RPAGE_RPN1_TYPE = _RPAGE_RPN1;
pub const H_PAGE_BUSY: _RPAGE_RSV1_TYPE = _RPAGE_RSV1;
pub const H_PAGE_HASHPTE: _RPAGE_RPN43_TYPE = _RPAGE_RPN43;
pub const H_PTE_PKEY_BIT4: _RPAGE_PKEY_BIT4_TYPE = _RPAGE_PKEY_BIT4;
pub const H_PTE_PKEY_BIT3: _RPAGE_PKEY_BIT3_TYPE = _RPAGE_PKEY_BIT3;
pub const H_PTE_PKEY_BIT2: _RPAGE_PKEY_BIT2_TYPE = _RPAGE_PKEY_BIT2;
pub const H_PTE_PKEY_BIT1: _RPAGE_PKEY_BIT1_TYPE = _RPAGE_PKEY_BIT1;
pub const H_PTE_PKEY_BIT0: _RPAGE_PKEY_BIT0_TYPE = _RPAGE_PKEY_BIT0;
pub const H_PAGE_THP_HUGE: _RPAGE_RPN1_TYPE = H_PAGE_4K_PFN;
pub const _PAGE_HPTEFLAGS: u64 = H_PAGE_BUSY as u64 | H_PAGE_HASHPTE as u64 | H_PAGE_COMBO as u64;
pub const H_PTE_FRAG_SIZE_SHIFT: usize = H_PTE_INDEX_SIZE + 3 + 1;
pub const H_PTE_FRAG_NR: usize = PAGE_SIZE >> H_PTE_FRAG_SIZE_SHIFT;
/* CONFIG_TRANSPARENT_HUGEPAGE || CONFIG_HUGETLB_PAGE adds the extra slot-details half. */
pub const H_PMD_FRAG_SIZE_SHIFT: usize = H_PMD_INDEX_SIZE + 3;
pub const H_PMD_FRAG_NR: usize = PAGE_SIZE >> H_PMD_FRAG_SIZE_SHIFT;

pub const INVALID_RPTE_HIDX: usize = 0x0;

#[inline]
pub unsafe fn __real_pte(pte: pte_t, ptep: *mut pte_t, offset: isize) -> real_pte_t {
    let mut rpte: real_pte_t = core::mem::zeroed();
    rpte.pte = pte;
    smp_rmb();
    let hidxp = (ptep.offset(offset) as *mut usize);
    rpte.hidx = core::ptr::read(hidxp);
    rpte
}

#[inline]
pub fn hidx_unshift_by_one(x: usize) -> usize { x.wrapping_add(0xf) & 0xf }
#[inline]
pub fn hidx_shift_by_one(x: usize) -> usize { x.wrapping_add(1) & 0xf }
#[inline]
pub fn hidx_bits(x: usize, index: usize) -> usize { x << (index << 2) }
#[inline]
pub fn bits_to_hidx(x: usize, index: usize) -> usize { (x >> (index << 2)) & 0xf }

#[inline]
pub fn __rpte_to_hidx(rpte: real_pte_t, index: usize) -> usize {
    hidx_unshift_by_one(bits_to_hidx(rpte.hidx, index))
}

#[inline]
pub unsafe fn pte_set_hidx(ptep: *mut pte_t, mut rpte: real_pte_t, subpg_index: usize, hidx: usize, offset: isize) -> usize {
    let hidxp = ptep.offset(offset) as *mut usize;
    rpte.hidx &= !hidx_bits(0xf, subpg_index);
    core::ptr::write(hidxp, rpte.hidx | hidx_bits(hidx_shift_by_one(hidx), subpg_index));
    smp_wmb();
    0
}

#[inline]
pub fn __rpte_to_pte(r: real_pte_t) -> pte_t { r.pte }
extern "C" { pub fn __rpte_sub_valid(rpte: real_pte_t, index: usize) -> bool; }

/* The C iterator macro expands to a loop over hashed subpages. */
#[macro_export]
macro_rules! pte_iterate_hashed_subpages {
    ($rpte:expr, $psize:expr, $vpn:ident, $index:ident, $shift:ident, $body:block) => {{
        let __end = $vpn + (1usize << (PAGE_SHIFT - VPN_SHIFT));
        let __split = $psize == MMU_PAGE_4K || $psize == MMU_PAGE_64K_AP;
        $shift = mmu_psize_defs[$psize].shift;
        $index = 0;
        while $vpn < __end {
            if !__split || unsafe { __rpte_sub_valid($rpte, $index) } $body
            $index += 1;
            $vpn += 1usize << ($shift - VPN_SHIFT);
        }
    }};
}

#[inline]
pub fn pte_pagesize_index(_mm: *mut mm_struct, _addr: usize, pte: usize) -> usize {
    if (pte & H_PAGE_COMBO as usize) != 0 { MMU_PAGE_4K } else { MMU_PAGE_64K }
}

#[inline]
pub unsafe fn hash__remap_4k_pfn(vma: *mut vm_area_struct, addr: usize, pfn: usize, prot: pgprot_t) -> i32 {
    if pfn > (PTE_RPN_MASK >> PAGE_SHIFT) {
        WARN(1, "remap_4k_pfn called with wrong pfn value\n");
        return -EINVAL;
    }
    remap_pfn_range(vma, addr, pfn, PAGE_SIZE, __pgprot(pgprot_val(prot) | H_PAGE_4K_PFN as _))
}

pub const H_PTE_TABLE_SIZE: usize = PTE_FRAG_SIZE;
/* H_PMD_TABLE_SIZE/H_PUD_TABLE_SIZE include a second unsigned-long array when their respective huge-page option is enabled. */
pub const H_PGD_TABLE_SIZE: usize = core::mem::size_of::<pgd_t>() << PGD_INDEX_SIZE;

#[cfg(feature = "transparent_hugepage")]
#[inline]
pub unsafe fn get_hpte_slot_array(pmdp: *mut pmd_t) -> *mut u8 {
    smp_rmb();
    *(pmdp.add(PTRS_PER_PMD) as *mut *mut u8)
}

#[cfg(feature = "transparent_hugepage")]
#[inline]
pub unsafe fn hpte_valid(hpte_slot_array: *mut u8, index: isize) -> u32 { *hpte_slot_array.offset(index) as u32 & 0x1 }
#[cfg(feature = "transparent_hugepage")]
#[inline]
pub unsafe fn hpte_hash_index(hpte_slot_array: *mut u8, index: isize) -> u32 { (*hpte_slot_array.offset(index) as u32) >> 1 }
#[cfg(feature = "transparent_hugepage")]
#[inline]
pub unsafe fn mark_hpte_slot_valid(hpte_slot_array: *mut u8, index: usize, hidx: u32) { *hpte_slot_array.add(index) = ((hidx << 1) | 1) as u8; }

#[cfg(feature = "transparent_hugepage")]
#[inline]
pub fn hash__pmd_trans_huge(pmd: pmd_t) -> i32 { ((pmd_val(pmd) & (_PAGE_PTE | H_PAGE_THP_HUGE)) == (_PAGE_PTE | H_PAGE_THP_HUGE)) as i32 }
#[cfg(feature = "transparent_hugepage")]
#[inline]
pub fn hash__pmd_mkhuge(pmd: pmd_t) -> pmd_t { __pmd(pmd_val(pmd) | (_PAGE_PTE | H_PAGE_THP_HUGE)) }

extern "C" {
    pub fn remap_pfn_range(vma: *mut vm_area_struct, addr: usize, pfn: usize, size: usize, prot: pgprot_t) -> i32;
    pub fn hash__pmd_hugepage_update(mm: *mut mm_struct, addr: usize, pmdp: *mut pmd_t, clr: usize, set: usize) -> usize;
    pub fn hash__pmdp_collapse_flush(vma: *mut vm_area_struct, address: usize, pmdp: *mut pmd_t) -> pmd_t;
    pub fn hash__pgtable_trans_huge_deposit(mm: *mut mm_struct, pmdp: *mut pmd_t, pgtable: pgtable_t);
    pub fn hash__pgtable_trans_huge_withdraw(mm: *mut mm_struct, pmdp: *mut pmd_t) -> pgtable_t;
    pub fn hash__pmdp_huge_get_and_clear(mm: *mut mm_struct, addr: usize, pmdp: *mut pmd_t) -> pmd_t;
    pub fn hash__has_transparent_hugepage() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
