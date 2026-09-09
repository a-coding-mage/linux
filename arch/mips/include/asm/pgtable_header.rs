/* Translated from mips/include/asm/pgtable.h. */

// C includes and configuration-dependent includes are supplied by the surrounding kernel.

#[repr(C)]
pub struct mm_struct;
#[repr(C)]
pub struct vm_area_struct;

pub const PAGE_SHARED: pgprot_t = vm_get_page_prot(VM_READ | VM_WRITE | VM_SHARED);
pub const PAGE_KERNEL: pgprot_t = __pgprot(_PAGE_PRESENT | __READABLE | __WRITEABLE | _PAGE_GLOBAL | _page_cachable_default);
pub const PAGE_KERNEL_NC: pgprot_t = __pgprot(_PAGE_PRESENT | __READABLE | __WRITEABLE | _PAGE_GLOBAL | _CACHE_CACHABLE_NONCOHERENT);
pub const PAGE_KERNEL_UNCACHED: pgprot_t = __pgprot(_PAGE_PRESENT | __READABLE | __WRITEABLE | _PAGE_GLOBAL | _CACHE_UNCACHED);

// If _PAGE_NO_EXEC is unavailable, execute protection is treated as read protection.
extern "C" {
    pub static mut _page_cachable_default: ::core::ffi::c_ulong;
    pub fn __update_cache(address: ::core::ffi::c_ulong, pte: pte_t);
    pub static mut empty_zero_page: ::core::ffi::c_ulong;
    pub static mut zero_page_mask: ::core::ffi::c_ulong;
    pub fn pagetable_init();
}

#[inline]
pub unsafe fn ZERO_PAGE(vaddr: *const ::core::ffi::c_void) -> *mut page {
    virt_to_page((empty_zero_page.wrapping_add((vaddr as ::core::ffi::c_ulong) & zero_page_mask)) as *mut ::core::ffi::c_void)
}
// __HAVE_COLOR_ZERO_PAGE

#[inline]
pub unsafe fn pmd_phys(pmd: pmd_t) -> ::core::ffi::c_ulong {
    virt_to_phys(pmd_val(pmd) as *mut ::core::ffi::c_void)
}
#[inline]
pub unsafe fn pmd_pfn(pmd: pmd_t) -> ::core::ffi::c_ulong { pmd_val(pmd) >> PFN_PTE_SHIFT }
#[cfg(not(CONFIG_MIPS_HUGE_TLB_SUPPORT))]
#[inline]
pub unsafe fn pmd_page(pmd: pmd_t) -> *mut page { pfn_to_page(pmd_phys(pmd) >> PAGE_SHIFT) }
#[inline]
pub unsafe fn pmd_page_vaddr(pmd: pmd_t) -> ::core::ffi::c_ulong { pmd_val(pmd) }

#[inline]
pub unsafe fn htw_stop() {
    if cpu_has_htw {
        let mut flags: ::core::ffi::c_ulong = 0;
        local_irq_save(&mut flags);
        if raw_current_cpu_data.htw_seq == 0 { raw_current_cpu_data.htw_seq = raw_current_cpu_data.htw_seq.wrapping_add(1); write_c0_pwctl(read_c0_pwctl() & !(1 << MIPS_PWCTL_PWEN_SHIFT)); back_to_back_c0_hazard(); }
        else { raw_current_cpu_data.htw_seq = raw_current_cpu_data.htw_seq.wrapping_add(1); }
        local_irq_restore(flags);
    }
}
#[inline]
pub unsafe fn htw_start() {
    if cpu_has_htw {
        let mut flags: ::core::ffi::c_ulong = 0;
        local_irq_save(&mut flags);
        raw_current_cpu_data.htw_seq = raw_current_cpu_data.htw_seq.wrapping_sub(1);
        if raw_current_cpu_data.htw_seq == 0 { write_c0_pwctl(read_c0_pwctl() | (1 << MIPS_PWCTL_PWEN_SHIFT)); back_to_back_c0_hazard(); }
        local_irq_restore(flags);
    }
}

#[cfg(all(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_CPU_MIPS32))]
#[inline] pub fn pte_none(pte: pte_t) -> bool { #[cfg(CONFIG_XPA)] { !(pte.pte_high & !_PAGE_GLOBAL != 0) } #[cfg(not(CONFIG_XPA))] { !((pte.pte_low | pte.pte_high) & !_PAGE_GLOBAL != 0) } }
#[cfg(not(all(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_CPU_MIPS32)))]
#[inline] pub fn pte_none(pte: pte_t) -> bool { !(pte_val(pte) & !_PAGE_GLOBAL != 0) }

#[cfg(all(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_CPU_MIPS32))]
#[inline] pub fn pte_present(pte: pte_t) -> bool { pte.pte_low & _PAGE_PRESENT != 0 }
#[cfg(not(all(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_CPU_MIPS32)))]
#[inline] pub fn pte_present(pte: pte_t) -> bool { pte_val(pte) & _PAGE_PRESENT != 0 }
#[inline] pub fn pte_no_exec(pte: pte_t) -> bool { #[cfg(all(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_CPU_MIPS32))] { pte.pte_low & _PAGE_NO_EXEC != 0 } #[cfg(not(all(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_CPU_MIPS32)))] { pte_val(pte) & _PAGE_NO_EXEC != 0 } }

#[cfg(all(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_CPU_MIPS32))]
pub unsafe fn set_pte(ptep: *mut pte_t, pte: pte_t) { (*ptep).pte_high = pte.pte_high; smp_wmb(); (*ptep).pte_low = pte.pte_low; let buddy = ptep_buddy(ptep); if (if cfg!(CONFIG_XPA) { pte.pte_high } else { pte.pte_low }) & _PAGE_GLOBAL != 0 && pte_none(*buddy) { if !cfg!(CONFIG_XPA) { (*buddy).pte_low |= _PAGE_GLOBAL; } (*buddy).pte_high |= _PAGE_GLOBAL; } }
#[cfg(all(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_CPU_MIPS32))]
pub unsafe fn pte_clear(_mm: *mut mm_struct, _addr: ::core::ffi::c_ulong, ptep: *mut pte_t) { let mut null = __pte(0); htw_stop(); if cfg!(CONFIG_XPA) { if (*ptep_buddy(ptep)).pte_high & _PAGE_GLOBAL != 0 { null.pte_high = _PAGE_GLOBAL; } } else if (*ptep_buddy(ptep)).pte_low & _PAGE_GLOBAL != 0 { null.pte_low = _PAGE_GLOBAL; null.pte_high = _PAGE_GLOBAL; } set_pte(ptep, null); htw_start(); }

#[cfg(not(all(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_CPU_MIPS32)))]
pub unsafe fn set_pte(ptep: *mut pte_t, pteval: pte_t) { *ptep = pteval; #[cfg(not(CONFIG_CPU_R3K_TLB))] if pte_val(pteval) & _PAGE_GLOBAL != 0 { let buddy = ptep_buddy(ptep); #[cfg(all(CONFIG_PHYS_ADDR_T_64BIT, not(CONFIG_CPU_MIPS32)))] { cmpxchg64(&mut (*buddy).pte, 0, _PAGE_GLOBAL); } #[cfg(not(all(CONFIG_PHYS_ADDR_T_64BIT, not(CONFIG_CPU_MIPS32))))] { cmpxchg(&mut (*buddy).pte, 0, _PAGE_GLOBAL); } } }
#[cfg(not(all(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_CPU_MIPS32)))]
pub unsafe fn pte_clear(_mm: *mut mm_struct, _addr: ::core::ffi::c_ulong, ptep: *mut pte_t) { htw_stop(); #[cfg(not(CONFIG_CPU_R3K_TLB))] { if pte_val(*ptep_buddy(ptep)) & _PAGE_GLOBAL != 0 { set_pte(ptep, __pte(_PAGE_GLOBAL)); } else { set_pte(ptep, __pte(0)); } } #[cfg(CONFIG_CPU_R3K_TLB)] set_pte(ptep, __pte(0)); htw_start(); }

pub unsafe fn set_ptes(_mm: *mut mm_struct, addr: ::core::ffi::c_ulong, mut ptep: *mut pte_t, mut pte: pte_t, mut nr: ::core::ffi::c_uint) { let mut do_sync = false; for i in 0..nr { let cur = *ptep.add(i as usize); if !pte_present(pte) || (pte_present(cur) && pte_pfn(cur) == pte_pfn(pte)) { continue; } do_sync = true; } if do_sync { __update_cache(addr, pte); } loop { set_pte(ptep, pte); nr -= 1; if nr == 0 { break; } ptep = ptep.add(1); pte = __pte(pte_val(pte).wrapping_add(1usize.wrapping_shl(PFN_PTE_SHIFT as u32) as _)); } }
// set_ptes set_ptes

#[inline] pub unsafe fn set_pmd(pmdptr: *mut pmd_t, pmdval: pmd_t) { *pmdptr = pmdval; }
#[cfg(not(__PAGETABLE_PMD_FOLDED))]
#[inline] pub unsafe fn set_pud(pudptr: *mut pud_t, pudval: pud_t) { *pudptr = pudval; }
pub const PGD_T_LOG2: usize = core::mem::size_of::<pgd_t>().trailing_zeros() as usize;
pub const PMD_T_LOG2: usize = core::mem::size_of::<pmd_t>().trailing_zeros() as usize;
pub const PTE_T_LOG2: usize = core::mem::size_of::<pte_t>().trailing_zeros() as usize;
extern "C" { pub static mut swapper_pg_dir: [pgd_t; 0]; }

#[cfg(CONFIG_ARCH_HAS_PTE_SPECIAL)]
#[inline] pub fn pte_special(pte: pte_t) -> bool { #[cfg(all(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_CPU_MIPS32))] { pte.pte_low & _PAGE_SPECIAL != 0 } #[cfg(not(all(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_CPU_MIPS32)))] { pte_val(pte) & _PAGE_SPECIAL != 0 } }
#[cfg(CONFIG_ARCH_HAS_PTE_SPECIAL)]
#[inline] pub fn pte_mkspecial(mut pte: pte_t) -> pte_t { #[cfg(all(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_CPU_MIPS32))] { pte.pte_low |= _PAGE_SPECIAL; } #[cfg(not(all(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_CPU_MIPS32)))] { pte_val(pte) |= _PAGE_SPECIAL; } pte }

#[cfg(all(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_CPU_MIPS32))]
mod pte_ops_64 { use super::*; #[inline] pub fn pte_write(p: pte_t)->bool{p.pte_low&_PAGE_WRITE!=0} #[inline] pub fn pte_dirty(p: pte_t)->bool{p.pte_low&_PAGE_MODIFIED!=0} #[inline] pub fn pte_young(p: pte_t)->bool{p.pte_low&_PAGE_ACCESSED!=0} pub fn pte_wrprotect(mut p:pte_t)->pte_t{p.pte_low&=!_PAGE_WRITE;if !cfg!(CONFIG_XPA){p.pte_low&=!_PAGE_SILENT_WRITE;}p.pte_high&=!_PAGE_SILENT_WRITE;p} pub fn pte_mkclean(mut p:pte_t)->pte_t{p.pte_low&=!_PAGE_MODIFIED;if !cfg!(CONFIG_XPA){p.pte_low&=!_PAGE_SILENT_WRITE;}p.pte_high&=!_PAGE_SILENT_WRITE;p} pub fn pte_mkold(mut p:pte_t)->pte_t{p.pte_low&=!_PAGE_ACCESSED;if !cfg!(CONFIG_XPA){p.pte_low&=!_PAGE_SILENT_READ;}p.pte_high&=!_PAGE_SILENT_READ;p} pub fn pte_mkwrite_novma(mut p:pte_t)->pte_t{p.pte_low|=_PAGE_WRITE;if p.pte_low&_PAGE_MODIFIED!=0{if !cfg!(CONFIG_XPA){p.pte_low|=_PAGE_SILENT_WRITE;}p.pte_high|=_PAGE_SILENT_WRITE;}p} pub fn pte_mkdirty(mut p:pte_t)->pte_t{p.pte_low|=_PAGE_MODIFIED;if p.pte_low&_PAGE_WRITE!=0{if !cfg!(CONFIG_XPA){p.pte_low|=_PAGE_SILENT_WRITE;}p.pte_high|=_PAGE_SILENT_WRITE;}p} pub fn pte_mkyoung(mut p:pte_t)->pte_t{p.pte_low|=_PAGE_ACCESSED;if p.pte_low&_PAGE_NO_READ==0{if !cfg!(CONFIG_XPA){p.pte_low|=_PAGE_SILENT_READ;}p.pte_high|=_PAGE_SILENT_READ;}p} }
#[cfg(not(all(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_CPU_MIPS32)))]
#[inline] pub fn pte_write(p:pte_t)->bool{pte_val(p)&_PAGE_WRITE!=0}
#[cfg(not(all(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_CPU_MIPS32)))]
#[inline] pub fn pte_dirty(p:pte_t)->bool{pte_val(p)&_PAGE_MODIFIED!=0}
#[cfg(not(all(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_CPU_MIPS32)))]
#[inline] pub fn pte_young(p:pte_t)->bool{pte_val(p)&_PAGE_ACCESSED!=0}
#[cfg(not(all(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_CPU_MIPS32)))]
pub fn pte_wrprotect(mut p:pte_t)->pte_t{pte_val(p)&=!(_PAGE_WRITE|_PAGE_SILENT_WRITE);p}
#[cfg(not(all(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_CPU_MIPS32)))]
pub fn pte_mkclean(mut p:pte_t)->pte_t{pte_val(p)&=!(_PAGE_MODIFIED|_PAGE_SILENT_WRITE);p}
#[cfg(not(all(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_CPU_MIPS32)))]
pub fn pte_mkold(mut p:pte_t)->pte_t{pte_val(p)&=!(_PAGE_ACCESSED|_PAGE_SILENT_READ);p}
#[cfg(not(all(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_CPU_MIPS32)))]
pub fn pte_mkwrite_novma(mut p:pte_t)->pte_t{pte_val(p)|=_PAGE_WRITE;if pte_val(p)&_PAGE_MODIFIED!=0{pte_val(p)|=_PAGE_SILENT_WRITE;}p}
#[cfg(not(all(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_CPU_MIPS32)))]
pub fn pte_mkdirty(mut p:pte_t)->pte_t{pte_val(p)|=_PAGE_MODIFIED|_PAGE_SOFT_DIRTY;if pte_val(p)&_PAGE_WRITE!=0{pte_val(p)|=_PAGE_SILENT_WRITE;}p}
#[cfg(not(all(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_CPU_MIPS32)))]
pub fn pte_mkyoung(mut p:pte_t)->pte_t{pte_val(p)|=_PAGE_ACCESSED;if pte_val(p)&_PAGE_NO_READ==0{pte_val(p)|=_PAGE_SILENT_READ;}p}

#[cfg(not(all(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_CPU_MIPS32)))]
#[inline] pub fn pte_sw_mkyoung(p: pte_t) -> pte_t { pte_mkyoung(p) }
#[cfg(all(CONFIG_MIPS_HUGE_TLB_SUPPORT, not(all(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_CPU_MIPS32))))]
#[inline] pub fn pte_huge(p: pte_t) -> bool { pte_val(p) & _PAGE_HUGE != 0 }
#[cfg(all(CONFIG_MIPS_HUGE_TLB_SUPPORT, not(all(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_CPU_MIPS32))))]
#[inline] pub fn pte_mkhuge(mut p: pte_t) -> pte_t { pte_val(p) |= _PAGE_HUGE; p }
#[cfg(all(CONFIG_MIPS_HUGE_TLB_SUPPORT, not(all(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_CPU_MIPS32))))]
#[inline] pub fn pmd_write(p: pmd_t) -> bool { pmd_val(p) & _PAGE_WRITE != 0 }
#[cfg(all(CONFIG_MIPS_HUGE_TLB_SUPPORT, not(all(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_CPU_MIPS32))))]
pub unsafe fn pmd_page_huge(pmd: pmd_t) -> *mut page { if pmd_val(pmd) & _PAGE_HUGE != 0 { pfn_to_page(pmd_pfn(pmd)) } else { pfn_to_page(pmd_phys(pmd) >> PAGE_SHIFT) } }

#[cfg(all(CONFIG_HAVE_ARCH_SOFT_DIRTY, not(all(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_CPU_MIPS32))))]
#[inline] pub fn pte_soft_dirty(p: pte_t) -> bool { pte_val(p) & _PAGE_SOFT_DIRTY != 0 }
#[cfg(all(CONFIG_HAVE_ARCH_SOFT_DIRTY, not(all(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_CPU_MIPS32))))]
#[inline] pub fn pte_swp_soft_dirty(p: pte_t) -> bool { pte_soft_dirty(p) }
#[cfg(all(CONFIG_HAVE_ARCH_SOFT_DIRTY, not(all(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_CPU_MIPS32))))]
#[inline] pub fn pte_mksoft_dirty(mut p: pte_t) -> pte_t { pte_val(p) |= _PAGE_SOFT_DIRTY; p }
#[cfg(all(CONFIG_HAVE_ARCH_SOFT_DIRTY, not(all(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_CPU_MIPS32))))]
#[inline] pub fn pte_swp_mksoft_dirty(p: pte_t) -> pte_t { pte_mksoft_dirty(p) }
#[cfg(all(CONFIG_HAVE_ARCH_SOFT_DIRTY, not(all(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_CPU_MIPS32))))]
#[inline] pub fn pte_clear_soft_dirty(mut p: pte_t) -> pte_t { pte_val(p) &= !_PAGE_SOFT_DIRTY; p }
#[cfg(all(CONFIG_HAVE_ARCH_SOFT_DIRTY, not(all(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_CPU_MIPS32))))]
#[inline] pub fn pte_swp_clear_soft_dirty(p: pte_t) -> pte_t { pte_clear_soft_dirty(p) }

#[inline] pub fn pgprot_noncached(p: pgprot_t) -> pgprot_t { __pgprot((pgprot_val(p) & !_CACHE_MASK) | _CACHE_UNCACHED) }
#[inline] pub fn pgprot_writecombine(p: pgprot_t) -> pgprot_t { __pgprot((pgprot_val(p) & !_CACHE_MASK) | cpu_data[0].writecombine) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
