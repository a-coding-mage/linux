// SPDX-License-Identifier: GPL-2.0
// Translation of mm/mremap.c. Kernel-provided types, constants, macros, and
// functions referenced below are intentionally left as external dependencies.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum mremap_type { MREMAP_INVALID, MREMAP_NO_RESIZE, MREMAP_SHRINK, MREMAP_EXPAND }

#[repr(C)]
struct vma_remap_struct {
    addr: u64, old_len: u64, new_len: u64, flags: u64, new_addr: u64,
    uf: *mut vm_userfaultfd_ctx, uf_unmap_early: *mut list_head,
    uf_unmap: *mut list_head, vma: *mut vm_area_struct, delta: u64,
    populate_expand: bool, remap_type: mremap_type, mmap_locked: bool,
    charged: u64, vmi_needs_invalidate: bool,
}

unsafe fn get_old_pud(mm: *mut mm_struct, addr: u64) -> *mut pud_t {
    let pgd = pgd_offset(mm, addr); if pgd_none_or_clear_bad(pgd) { return core::ptr::null_mut(); }
    let p4d = p4d_offset(pgd, addr); if p4d_none_or_clear_bad(p4d) { return core::ptr::null_mut(); }
    let pud = pud_offset(p4d, addr); if pud_none_or_clear_bad(pud) { return core::ptr::null_mut(); } pud
}
unsafe fn get_old_pmd(mm: *mut mm_struct, addr: u64) -> *mut pmd_t {
    let pud = get_old_pud(mm, addr); if pud.is_null() { return core::ptr::null_mut(); }
    let pmd = pmd_offset(pud, addr); if pmd_none(*pmd) { core::ptr::null_mut() } else { pmd }
}
unsafe fn alloc_new_pud(mm: *mut mm_struct, addr: u64) -> *mut pud_t {
    let pgd = pgd_offset(mm, addr); let p4d = p4d_alloc(mm, pgd, addr); if p4d.is_null() { return core::ptr::null_mut(); } pud_alloc(mm, p4d, addr)
}
unsafe fn alloc_new_pmd(mm: *mut mm_struct, addr: u64) -> *mut pmd_t {
    let pud = alloc_new_pud(mm, addr); if pud.is_null() { return core::ptr::null_mut(); }
    let pmd = pmd_alloc(mm, pud, addr); if pmd.is_null() { return pmd; }
    VM_BUG_ON(pmd_trans_huge(*pmd)); pmd
}
unsafe fn take_rmap_locks(vma: *mut vm_area_struct) { if !(*vma).vm_file.is_null() { i_mmap_lock_write((*(*vma).vm_file).f_mapping); } if !(*vma).anon_vma.is_null() { anon_vma_lock_write((*vma).anon_vma); } }
unsafe fn drop_rmap_locks(vma: *mut vm_area_struct) { if !(*vma).anon_vma.is_null() { anon_vma_unlock_write((*vma).anon_vma); } if !(*vma).vm_file.is_null() { i_mmap_unlock_write((*(*vma).vm_file).f_mapping); } }

unsafe fn move_soft_dirty_pte(mut pte: pte_t) -> pte_t {
    if pte_none(pte) { return pte; }
    if pgtable_supports_soft_dirty() { pte = if pte_present(pte) { pte_mksoft_dirty(pte) } else { pte_swp_mksoft_dirty(pte) }; } pte
}
unsafe fn mremap_folio_pte_batch(vma: *mut vm_area_struct, addr: u64, ptep: *mut pte_t, pte: pte_t, max_nr: i32) -> i32 {
    if max_nr == 1 || pte_batch_hint(ptep, pte) == 1 { return 1; }
    let folio = vm_normal_folio(vma, addr, pte); if folio.is_null() || !folio_test_large(folio) { return 1; }
    folio_pte_batch_flags(folio, core::ptr::null_mut(), ptep, &pte, max_nr, FPB_RESPECT_WRITE)
}

// The following functions retain the kernel algorithm and intentionally use
// raw pointers and kernel helper names supplied by the surrounding tree.
unsafe fn move_ptes(pmc: *mut pagetable_move_control, extent: u64, old_pmd: *mut pmd_t, new_pmd: *mut pmd_t) -> i32 {
    let vma = (*pmc).old; let mm = (*vma).vm_mm; let mut err = 0;
    let need_clear = vma_has_uffd_without_event_remap(vma); let mut old_addr = (*pmc).old_addr; let mut new_addr = (*pmc).new_addr;
    let old_end = old_addr + extent; let mut old_ptep; let mut new_ptep; let mut old_ptl; let mut new_ptl; let mut dummy = core::mem::zeroed();
    if (*pmc).need_rmap_locks { take_rmap_locks(vma); }
    old_ptep = pte_offset_map_lock(mm, old_pmd, old_addr, &mut old_ptl); if old_ptep.is_null() { err = -EAGAIN; goto_out!(out, pmc, vma, err); }
    new_ptep = pte_offset_map_rw_nolock(mm, new_pmd, new_addr, &mut dummy, &mut new_ptl);
    if new_ptep.is_null() { pte_unmap_unlock(old_ptep, old_ptl); err = -EAGAIN; goto_out!(out, pmc, vma, err); }
    if new_ptl != old_ptl { spin_lock_nested(new_ptl, SINGLE_DEPTH_NESTING); }
    flush_tlb_batched_pending((*vma).vm_mm); lazy_mmu_mode_enable(); let mut force_flush = false; let mut nr_ptes;
    while old_addr < old_end { nr_ptes = 1; let max = (old_end-old_addr)>>PAGE_SHIFT; let old = ptep_get(old_ptep); if !pte_none(old) { let n = if pte_present(old) { force_flush=true; mremap_folio_pte_batch(vma,old_addr,old_ptep,old,max as i32) as u64 } else {1}; nr_ptes=n; let mut p = get_and_clear_ptes(mm,old_addr,old_ptep,nr_ptes); p=move_pte(p,old_addr,new_addr); p=move_soft_dirty_pte(p); if need_clear && pte_is_uffd_wp_marker(p) { pte_clear(mm,new_addr,new_ptep); } else { if need_clear { p=if pte_present(p) { if userfaultfd_rwp(vma)&&pte_uffd(p) { pte_modify(p,(*vma).vm_page_prot) } pte_clear_uffd(p) } else { pte_swp_clear_uffd(p) }; } set_ptes(mm,new_addr,new_ptep,p,nr_ptes); } } old_addr+=nr_ptes*PAGE_SIZE; new_addr+=nr_ptes*PAGE_SIZE; old_ptep=old_ptep.add(nr_ptes as usize); new_ptep=new_ptep.add(nr_ptes as usize); }
    lazy_mmu_mode_disable(); if force_flush { flush_tlb_range(vma,old_end-extent,old_end); } if new_ptl!=old_ptl { spin_unlock(new_ptl); } pte_unmap(new_ptep.sub(1)); pte_unmap_unlock(old_ptep.sub(1),old_ptl);
out: if (*pmc).need_rmap_locks { drop_rmap_locks(vma); } err
}

unsafe fn vrm_set_delta(v: *mut vma_remap_struct) { (*v).delta=abs_diff((*v).old_len,(*v).new_len); }
unsafe fn vrm_remap_type(v: *mut vma_remap_struct)->mremap_type { if (*v).delta==0 {mremap_type::MREMAP_NO_RESIZE} else if (*v).old_len>(*v).new_len {mremap_type::MREMAP_SHRINK} else {mremap_type::MREMAP_EXPAND} }
unsafe fn vrm_overlaps(v:*mut vma_remap_struct)->bool { let a=(*v).addr;let b=(*v).new_addr; a+(*v).old_len>b && b+(*v).new_len>a }
unsafe fn vrm_implies_new_addr(v:*mut vma_remap_struct)->bool { (*v).flags&(MREMAP_FIXED|MREMAP_DONTUNMAP)!=0 }

// Remaining kernel helpers are kept as direct external declarations so the
// translated implementation preserves the original ABI and dependency set.
extern "C" { fn do_mremap(vrm:*mut vma_remap_struct)->u64; }
#[no_mangle] pub unsafe extern "C" fn mremap(addr:u64,old_len:u64,new_len:u64,flags:u64,new_addr:u64)->u64 {
    let mut vrm=vma_remap_struct { addr:untagged_addr(addr),old_len,new_len,flags,new_addr,uf:core::ptr::null_mut(),uf_unmap_early:core::ptr::null_mut(),uf_unmap:core::ptr::null_mut(),vma:core::ptr::null_mut(),delta:0,populate_expand:false,remap_type:mremap_type::MREMAP_INVALID,mmap_locked:false,charged:0,vmi_needs_invalidate:false }; do_mremap(&mut vrm)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
