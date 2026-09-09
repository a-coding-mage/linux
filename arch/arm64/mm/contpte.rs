// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2023 ARM Ltd.
 */

// C dependencies supplied by the surrounding kernel translation unit.

#[inline]
unsafe fn mm_is_user(mm: *mut mm_struct) -> bool {
    // Kernel mappings cannot tolerate faults caused by dynamically changing CONT_PTE.
    if mm_is_efi(mm) { return false; }
    mm != &mut init_mm as *mut mm_struct
}

#[inline]
unsafe fn contpte_align_down(ptep: *mut pte_t) -> *mut pte_t {
    ((ptep as usize) & !(core::mem::size_of::<pte_t>() * CONT_PTES - 1)) as *mut pte_t
}

#[inline]
unsafe fn contpte_align_addr_ptep(start: &mut usize, end: &mut usize, mut ptep: *mut pte_t, nr: u32) -> *mut pte_t {
    if pte_cont(__ptep_get(ptep.add((nr - 1) as usize))) { *end = (*end + CONT_PTE_SIZE - 1) & !(CONT_PTE_SIZE - 1); }
    if pte_cont(__ptep_get(ptep)) {
        *start &= !(CONT_PTE_SIZE - 1);
        ptep = contpte_align_down(ptep);
    }
    ptep
}

unsafe fn contpte_try_unfold_partial(mm: *mut mm_struct, addr: usize, ptep: *mut pte_t, nr: u32) {
    if ptep != contpte_align_down(ptep) || nr < CONT_PTES as u32 { contpte_try_unfold(mm, addr, ptep, __ptep_get(ptep)); }
    if ptep.add(nr as usize) != contpte_align_down(ptep.add(nr as usize)) {
        let last_addr = addr + PAGE_SIZE * (nr as usize - 1);
        let last_ptep = ptep.add(nr as usize - 1);
        contpte_try_unfold(mm, last_addr, last_ptep, __ptep_get(last_ptep));
    }
}

unsafe fn contpte_convert(mm: *mut mm_struct, mut addr: usize, mut ptep: *mut pte_t, mut pte: pte_t) {
    let vma = TLB_FLUSH_VMA(mm, 0);
    let ptep = contpte_align_down(ptep);
    let start_addr = addr & !(CONT_PTE_SIZE - 1);
    addr = start_addr;
    pte = pfn_pte((pte_pfn(pte) / CONT_PTES as usize) * CONT_PTES as usize, pte_pgprot(pte));
    for _ in 0..CONT_PTES {
        let ptent = __ptep_get_and_clear(mm, addr, ptep);
        if pte_dirty(ptent) { pte = pte_mkdirty(pte); }
        if pte_young(ptent) { pte = pte_mkyoung(pte); }
        ptep = ptep.add(1); addr += PAGE_SIZE;
    }
    if !system_supports_bbml3() { __flush_tlb_range(&vma, start_addr, addr, PAGE_SIZE, 3, TLBF_NOWALKCACHE); }
    __set_ptes(mm, start_addr, contpte_align_down(ptep.sub(CONT_PTES)), pte, CONT_PTES as u32);
}

pub unsafe fn __contpte_try_fold(mm: *mut mm_struct, addr: usize, orig_ptep: *mut pte_t, mut pte: pte_t) {
    if !mm_is_user(mm) { return; }
    let page = pte_page(pte); let folio = page_folio(page);
    let folio_start = addr - (page.offset_from(&mut (*folio).page) as usize) * PAGE_SIZE;
    let folio_end = folio_start + folio_nr_pages(folio) * PAGE_SIZE;
    let cont_start = addr & !(CONT_PTE_SIZE - 1); let cont_end = cont_start + CONT_PTE_SIZE;
    if folio_start > cont_start || folio_end < cont_end { return; }
    let mut expected = pfn_pte((pte_pfn(pte) / CONT_PTES as usize) * CONT_PTES as usize, pte_pgprot(pte_mkold(pte_mkclean(pte))));
    let mut ptep = contpte_align_down(orig_ptep);
    for _ in 0..CONT_PTES {
        let sub = pte_mkold(pte_mkclean(__ptep_get(ptep)));
        if !pte_same(sub, expected) { return; }
        expected = pte_advance_pfn(expected, 1); ptep = ptep.add(1);
    }
    pte = pte_mkcont(pte); contpte_convert(mm, addr, orig_ptep, pte);
}

pub unsafe fn __contpte_try_unfold(mm: *mut mm_struct, addr: usize, ptep: *mut pte_t, mut pte: pte_t) {
    if !mm_is_user(mm) { return; }
    pte = pte_mknoncont(pte); contpte_convert(mm, addr, ptep, pte);
}

pub unsafe fn contpte_ptep_get(mut ptep: *mut pte_t, mut orig_pte: pte_t) -> pte_t {
    ptep = contpte_align_down(ptep);
    for i in 0..CONT_PTES { let pte = __ptep_get(ptep); if pte_dirty(pte) { orig_pte=pte_mkdirty(orig_pte); for _ in i..CONT_PTES { let p=__ptep_get(ptep); if pte_young(p) { orig_pte=pte_mkyoung(orig_pte); break; } ptep=ptep.add(1); } break; } if pte_young(pte) { orig_pte=pte_mkyoung(orig_pte); for _ in (i+1)..CONT_PTES { ptep=ptep.add(1); let p=__ptep_get(ptep); if pte_dirty(p) { orig_pte=pte_mkdirty(orig_pte); break; } } break; } ptep=ptep.add(1); }
    orig_pte
}

pub unsafe fn contpte_ptep_get_lockless(orig_ptep: *mut pte_t) -> pte_t {
    loop {
        let mut orig = __ptep_get(orig_ptep); if !pte_valid_cont(orig) { return orig; }
        let prot=pte_pgprot(pte_mkold(pte_mkclean(orig))); let base=contpte_align_down(orig_ptep); let mut pfn=pte_pfn(orig)-orig_ptep.offset_from(base) as usize; let mut p=base; let mut retry=false;
        for _ in 0..CONT_PTES { let x=__ptep_get(p); if !contpte_is_consistent(x,pfn,prot) { retry=true; break; } if pte_dirty(x) { orig=pte_mkdirty(orig); } if pte_young(x) { orig=pte_mkyoung(orig); } p=p.add(1); pfn+=1; }
        if !retry { return orig; }
    }
}

unsafe fn contpte_is_consistent(pte: pte_t, pfn: usize, orig_prot: pgprot_t) -> bool { pte_valid_cont(pte) && pte_pfn(pte)==pfn && pgprot_val(pte_pgprot(pte_mkold(pte_mkclean(pte))))==pgprot_val(orig_prot) }

pub unsafe fn contpte_set_ptes(mm:*mut mm_struct, mut addr:usize, mut ptep:*mut pte_t, mut pte:pte_t, nr:u32) { if !mm_is_user(mm) { __set_ptes(mm,addr,ptep,pte,nr); return; } let end=addr+(nr as usize*PAGE_SIZE); let mut pfn=pte_pfn(pte); let prot=pte_pgprot(pte); while addr!=end { let next=pte_cont_addr_end(addr,end); let n=((next-addr)>>PAGE_SHIFT) as u32; pte=pfn_pte(pfn,prot); pte=if ((addr|next|(pfn<<PAGE_SHIFT)) & !CONT_PTE_MASK)==0 {pte_mkcont(pte)} else {pte_mknoncont(pte)}; __set_ptes(mm,addr,ptep,pte,n); addr=next;ptep=ptep.add(n as usize);pfn+=n as usize; } }

pub unsafe fn contpte_clear_full_ptes(mm:*mut mm_struct,addr:usize,ptep:*mut pte_t,nr:u32,full:i32){contpte_try_unfold_partial(mm,addr,ptep,nr);__clear_full_ptes(mm,addr,ptep,nr,full)}
pub unsafe fn contpte_get_and_clear_full_ptes(mm:*mut mm_struct,addr:usize,ptep:*mut pte_t,nr:u32,full:i32)->pte_t{contpte_try_unfold_partial(mm,addr,ptep,nr);__get_and_clear_full_ptes(mm,addr,ptep,nr,full)}
pub unsafe fn contpte_test_and_clear_young_ptes(vma:*mut vm_area_struct,mut addr:usize,ptep:*mut pte_t,nr:u32)->bool{let end=addr+nr as usize*PAGE_SIZE;let mut e=end; ptep=contpte_align_addr_ptep(&mut addr,&mut e,ptep,nr);let mut y=false;while addr!=e{y|=__ptep_test_and_clear_young(vma,addr,ptep);addr+=PAGE_SIZE;ptep=ptep.add(1);}y}
pub unsafe fn contpte_clear_flush_young_ptes(vma:*mut vm_area_struct,addr:usize,ptep:*mut pte_t,nr:u32)->bool{let y=contpte_test_and_clear_young_ptes(vma,addr,ptep,nr);if y{let mut s=addr;let mut e=addr+nr as usize*PAGE_SIZE;contpte_align_addr_ptep(&mut s,&mut e,ptep,nr);__flush_tlb_range(vma,s,e,PAGE_SIZE,3,TLBF_NOWALKCACHE|TLBF_NOSYNC);}y}
pub unsafe fn contpte_wrprotect_ptes(mm:*mut mm_struct,addr:usize,ptep:*mut pte_t,nr:u32){contpte_try_unfold_partial(mm,addr,ptep,nr);__wrprotect_ptes(mm,addr,ptep,nr)}
pub unsafe fn contpte_clear_young_dirty_ptes(vma:*mut vm_area_struct,addr:usize,ptep:*mut pte_t,nr:u32,flags:cydp_t){let mut s=addr;let mut e=s+nr as usize*PAGE_SIZE;ptep=contpte_align_addr_ptep(&mut s,&mut e,ptep,nr);__clear_young_dirty_ptes(vma,s,ptep,((e-s)/PAGE_SIZE) as u32,flags)}

unsafe fn contpte_all_subptes_match_access_flags(ptep:*mut pte_t,entry:pte_t)->bool{let p=contpte_align_down(ptep);let mask:PTEVAL_T= PTE_RDONLY|PTE_AF|PTE_WRITE|PTE_DIRTY;let want=pte_val(entry)&mask;for i in 0..CONT_PTES{if pte_val(__ptep_get(p.add(i)))&mask != want{return false;}}true}

pub unsafe fn contpte_ptep_set_access_flags(vma:*mut vm_area_struct,addr:usize,ptep:*mut pte_t,entry:pte_t,dirty:i32)->i32{if contpte_all_subptes_match_access_flags(ptep,entry){return 0;}let orig=pte_mknoncont(__ptep_get(ptep));if pte_write(orig)==pte_write(entry){let mut p=contpte_align_down(ptep);let mut a=addr&!(CONT_PTE_SIZE-1);let start=a;for _ in 0..CONT_PTES{__ptep_set_access_flags(vma,a,p,entry,0);a+=PAGE_SIZE;p=p.add(1);}if dirty!=0{__flush_tlb_range(vma,start,start+CONT_PTE_SIZE,PAGE_SIZE,3,TLBF_NOWALKCACHE|TLBF_NOBROADCAST)}}else{__contpte_try_unfold((*vma).vm_mm,addr,ptep,orig);__ptep_set_access_flags(vma,addr,ptep,entry,dirty)}1}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
