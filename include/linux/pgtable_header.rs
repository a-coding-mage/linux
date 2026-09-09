/* SPDX-License-Identifier: GPL-2.0 */
/* Direct Rust translation of linux/pgtable.h.  Configuration symbols and
 * architecture-provided items are intentionally left as external dependencies. */

pub const PMD_ORDER: usize = PMD_SHIFT - PAGE_SHIFT;
pub const PUD_ORDER: usize = PUD_SHIFT - PAGE_SHIFT;

#[inline] pub fn pte_index(address: usize) -> usize { (address >> PAGE_SHIFT) & (PTRS_PER_PTE - 1) }
#[inline] pub fn pmd_index(address: usize) -> usize { (address >> PMD_SHIFT) & (PTRS_PER_PMD - 1) }
#[inline] pub fn pud_index(address: usize) -> usize { (address >> PUD_SHIFT) & (PTRS_PER_PUD - 1) }
#[inline] pub const fn pgd_index(a: usize) -> usize { (a >> PGDIR_SHIFT) & (PTRS_PER_PGD - 1) }

#[inline] pub unsafe fn kernel_pte_init(_addr: *mut core::ffi::c_void) {}
#[inline] pub unsafe fn pmd_init(_addr: *mut core::ffi::c_void) {}
#[inline] pub unsafe fn pud_init(_addr: *mut core::ffi::c_void) {}

#[inline] pub unsafe fn pte_offset_kernel(pmd: *mut pmd_t, address: usize) -> *mut pte_t {
    pmd_page_vaddr(*pmd).add(pte_index(address))
}
#[inline] pub unsafe fn __pte_map(pmd: *mut pmd_t, address: usize) -> *mut pte_t { pte_offset_kernel(pmd, address) }
#[inline] pub unsafe fn pte_unmap(_pte: *mut pte_t) { rcu_read_unlock(); }

extern "C" { pub fn pte_free_defer(mm: *mut mm_struct, pgtable: pgtable_t); }
#[inline] pub unsafe fn pmd_offset(pud: *mut pud_t, address: usize) -> *mut pmd_t { pud_pgtable(*pud).add(pmd_index(address)) }
#[inline] pub unsafe fn pud_offset(p4d: *mut p4d_t, address: usize) -> *mut pud_t { p4d_pgtable(*p4d).add(pud_index(address)) }
#[inline] pub unsafe fn pgd_offset_pgd(pgd: *mut pgd_t, _address: usize) -> *mut pgd_t { pgd }
#[inline] pub unsafe fn pgd_offset(mm: *mut mm_struct, address: usize) -> *mut pgd_t { pgd_offset_pgd((*mm).pgd, address) }
#[inline] pub unsafe fn pgd_offset_k(address: usize) -> *mut pgd_t { pgd_offset(&raw mut init_mm, address) }
#[inline] pub unsafe fn pmd_off(mm: *mut mm_struct, va: usize) -> *mut pmd_t { pmd_offset(pud_offset(p4d_offset(pgd_offset(mm,va),va),va),va) }
#[inline] pub unsafe fn pmd_off_k(va: usize) -> *mut pmd_t { pmd_offset(pud_offset(p4d_offset(pgd_offset_k(va),va),va),va) }
#[inline] pub unsafe fn virt_to_kpte(vaddr: usize) -> *mut pte_t { let p = pmd_off_k(vaddr); if pmd_none(*p) { core::ptr::null_mut() } else { pte_offset_kernel(p,vaddr) } }

#[inline] pub fn pmd_young(_pmd: pmd_t) -> i32 { 0 }
#[inline] pub fn pmd_dirty(_pmd: pmd_t) -> i32 { 0 }

#[inline] pub unsafe fn lazy_mmu_mode_enable() {}
#[inline] pub unsafe fn lazy_mmu_mode_disable() {}
#[inline] pub unsafe fn lazy_mmu_mode_pause() {}
#[inline] pub unsafe fn lazy_mmu_mode_resume() {}

#[inline] pub unsafe fn pte_batch_hint(_ptep: *mut pte_t, _pte: pte_t) -> u32 { 1 }
#[inline] pub fn pte_advance_pfn(pte: pte_t, nr: usize) -> pte_t { __pte(pte_val(pte).wrapping_add(nr << PFN_PTE_SHIFT)) }
#[inline] pub fn pte_next_pfn(pte: pte_t) -> pte_t { pte_advance_pfn(pte, 1) }
#[inline] pub unsafe fn set_ptes(mm: *mut mm_struct, addr: usize, mut ptep: *mut pte_t, mut pte: pte_t, mut nr: u32) {
    page_table_check_ptes_set(mm,addr,ptep,pte,nr);
    loop { set_pte(ptep,pte); nr -= 1; if nr == 0 { break; } ptep=ptep.add(1); pte=pte_next_pfn(pte); }
}
#[inline] pub unsafe fn set_pte_at(mm: *mut mm_struct, addr: usize, ptep: *mut pte_t, pte: pte_t) { set_ptes(mm,addr,ptep,pte,1) }

extern "C" {
 pub fn ptep_set_access_flags(vma:*mut vm_area_struct,address:usize,ptep:*mut pte_t,entry:pte_t,dirty:i32)->i32;
 pub fn pte_clear_flush_young(vma:*mut vm_area_struct,address:usize,ptep:*mut pte_t)->bool;
 pub fn pmdp_clear_flush_young(vma:*mut vm_area_struct,address:usize,pmdp:*mut pmd_t)->bool;
}
#[inline] pub unsafe fn ptep_get(p:*mut pte_t)->pte_t { core::ptr::read_volatile(p) }
#[inline] pub unsafe fn pmdp_get(p:*mut pmd_t)->pmd_t { core::ptr::read_volatile(p) }
#[inline] pub unsafe fn pudp_get(p:*mut pud_t)->pud_t { core::ptr::read_volatile(p) }
#[inline] pub unsafe fn p4dp_get(p:*mut p4d_t)->p4d_t { core::ptr::read_volatile(p) }
#[inline] pub unsafe fn pgdp_get(p:*mut pgd_t)->pgd_t { core::ptr::read_volatile(p) }

#[inline] pub unsafe fn ptep_test_and_clear_young(vma:*mut vm_area_struct,address:usize,ptep:*mut pte_t)->bool { let p=ptep_get(ptep); if !pte_young(p){false}else{set_pte_at((*vma).vm_mm,address,ptep,pte_mkold(p));true} }
#[inline] pub unsafe fn pmdp_test_and_clear_young(_vma:*mut vm_area_struct,_address:usize,_pmdp:*mut pmd_t)->bool { false }
#[inline] pub fn arch_has_hw_nonleaf_pmd_young()->bool { false }
#[inline] pub fn arch_has_hw_pte_young()->bool { false }
#[inline] pub fn exec_folio_order()->u32 { 0 }
#[inline] pub unsafe fn arch_check_zapped_pte(_vma:*mut vm_area_struct,_pte:pte_t) {}
#[inline] pub unsafe fn arch_check_zapped_pmd(_vma:*mut vm_area_struct,_pmd:pmd_t) {}
#[inline] pub unsafe fn arch_check_zapped_pud(_vma:*mut vm_area_struct,_pud:pud_t) {}

#[inline] pub unsafe fn ptep_get_and_clear(mm:*mut mm_struct,address:usize,ptep:*mut pte_t)->pte_t { let p=ptep_get(ptep); pte_clear(mm,address,ptep); page_table_check_pte_clear(mm,address,p); p }
#[inline] pub unsafe fn ptep_clear(mm:*mut mm_struct,addr:usize,ptep:*mut pte_t) { let p=ptep_get(ptep); pte_clear(mm,addr,ptep); page_table_check_pte_clear(mm,addr,p); }
#[inline] pub unsafe fn get_and_clear_full_ptes(mm:*mut mm_struct,mut addr:usize,mut ptep:*mut pte_t,mut nr:u32,full:i32)->pte_t { let mut p=ptep_get_and_clear_full(mm,addr,ptep,full); while {nr-=1;nr!=0}{ptep=ptep.add(1);addr+=PAGE_SIZE;let q=ptep_get_and_clear_full(mm,addr,ptep,full);if pte_dirty(q){p=pte_mkdirty(p)}if pte_young(q){p=pte_mkyoung(p)}} p }
#[inline] pub unsafe fn get_and_clear_ptes(mm:*mut mm_struct,addr:usize,ptep:*mut pte_t,nr:u32)->pte_t { get_and_clear_full_ptes(mm,addr,ptep,nr,0) }
#[inline] pub unsafe fn ptep_get_and_clear_full(mm:*mut mm_struct,address:usize,ptep:*mut pte_t,_full:i32)->pte_t { ptep_get_and_clear(mm,address,ptep) }
#[inline] pub unsafe fn clear_full_ptes(mm:*mut mm_struct,mut addr:usize,mut ptep:*mut pte_t,mut nr:u32,full:i32){loop{ptep_get_and_clear_full(mm,addr,ptep,full);nr-=1;if nr==0{break}ptep=ptep.add(1);addr+=PAGE_SIZE;}}
#[inline] pub unsafe fn clear_ptes(mm:*mut mm_struct,addr:usize,ptep:*mut pte_t,nr:u32){clear_full_ptes(mm,addr,ptep,nr,0)}
#[inline] pub unsafe fn update_mmu_tlb(vma:*mut vm_area_struct,address:usize,ptep:*mut pte_t){update_mmu_tlb_range(vma,address,ptep,1)}
#[inline] pub unsafe fn update_mmu_tlb_range(_vma:*mut vm_area_struct,_address:usize,_ptep:*mut pte_t,_nr:u32){}
#[inline] pub unsafe fn clear_nonpresent_ptes(mm:*mut mm_struct,mut addr:usize,mut ptep:*mut pte_t,mut nr:u32){loop{pte_clear(mm,addr,ptep);nr-=1;if nr==0{break}ptep=ptep.add(1);addr+=PAGE_SIZE;}}

#[inline] pub fn pte_same(a:pte_t,b:pte_t)->i32 {(pte_val(a)==pte_val(b)) as i32}
#[inline] pub fn pmd_same(a:pmd_t,b:pmd_t)->i32 {(pmd_val(a)==pmd_val(b)) as i32}
#[inline] pub fn pud_same(a:pud_t,b:pud_t)->i32 {(pud_val(a)==pud_val(b)) as i32}
#[inline] pub fn p4d_same(a:p4d_t,b:p4d_t)->i32 {(p4d_val(a)==p4d_val(b)) as i32}
#[inline] pub fn pgd_same(a:pgd_t,b:pgd_t)->i32 {(pgd_val(a)==pgd_val(b)) as i32}
#[inline] pub fn pte_unused(_pte:pte_t)->i32{0}
#[inline] pub fn pte_sw_mkyoung(pte:pte_t)->pte_t{pte}
#[inline] pub fn arch_needs_pgtable_deposit()->bool{false}

pub const ARCH_PAGE_TABLE_SYNC_MASK:u32=0;
pub type pgtbl_mod_mask=u32;
pub const __PGTBL_PGD_MODIFIED:u32=0; pub const __PGTBL_P4D_MODIFIED:u32=1; pub const __PGTBL_PUD_MODIFIED:u32=2; pub const __PGTBL_PMD_MODIFIED:u32=3; pub const __PGTBL_PTE_MODIFIED:u32=4;
#[repr(C)] #[derive(Copy,Clone)] pub enum pgtable_level { PGTABLE_LEVEL_PTE=0, PGTABLE_LEVEL_PMD, PGTABLE_LEVEL_PUD, PGTABLE_LEVEL_P4D, PGTABLE_LEVEL_PGD }
pub fn pgtable_level_to_str(level:pgtable_level)->&'static core::ffi::CStr { match level { pgtable_level::PGTABLE_LEVEL_PTE=>c"pte",pgtable_level::PGTABLE_LEVEL_PMD=>c"pmd",pgtable_level::PGTABLE_LEVEL_PUD=>c"pud",pgtable_level::PGTABLE_LEVEL_P4D=>c"p4d",pgtable_level::PGTABLE_LEVEL_PGD=>c"pgd" } }

pub const USER_PGTABLES_CEILING:usize=0; pub const FIRST_USER_ADDRESS:usize=0;
#[inline] pub fn pgprot_nx<T>(p:T)->T{p} #[inline] pub fn pgprot_mhp<T>(p:T)->T{p}
#[inline] pub fn has_transparent_hugepage()->bool{false} #[inline] pub fn has_transparent_pud_hugepage()->bool{false}
#[inline] pub fn pgd_leaf<T>(_x:T)->bool{false} #[inline] pub fn p4d_leaf<T>(_x:T)->bool{false} #[inline] pub fn pud_leaf<T>(_x:T)->bool{false} #[inline] pub fn pmd_leaf<T>(_x:T)->bool{false}
pub const MAX_POSSIBLE_PHYSMEM_BITS:u32=32;

/* External types, constants, and architecture operations referenced above are
 * supplied by the corresponding translated Linux headers. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
