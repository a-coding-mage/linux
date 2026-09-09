// SPDX-License-Identifier: GPL-2.0
// Translated from pagewalk.c. Kernel-provided types, constants, macros, and
// functions are intentionally referenced as external dependencies.

unsafe fn real_depth(mut depth: i32) -> i32 {
    if depth == 3 && PTRS_PER_PMD == 1 { depth = 2; }
    if depth == 2 && PTRS_PER_PUD == 1 { depth = 1; }
    if depth == 1 && PTRS_PER_P4D == 1 { depth = 0; }
    depth
}

unsafe fn walk_pte_range_inner(mut pte: *mut pte_t, mut addr: c_ulong, end: c_ulong, walk: *mut mm_walk) -> c_int {
    let ops = (*walk).ops;
    let mut err = 0;
    loop {
        if !(*ops).install_pte.is_none() && pte_none(ptep_get(pte)) {
            let mut new_pte: pte_t = core::mem::zeroed();
            err = ((*ops).install_pte.unwrap())(addr, addr + PAGE_SIZE, &mut new_pte, walk);
            if err != 0 { break; }
            set_pte_at((*walk).mm, addr, pte, new_pte);
            if !WARN_ON_ONCE((*walk).no_vma) { update_mmu_cache((*walk).vma, addr, pte); }
        } else {
            err = ((*ops).pte_entry.unwrap())(pte, addr, addr + PAGE_SIZE, walk);
            if err != 0 { break; }
        }
        if addr >= end - PAGE_SIZE { break; }
        addr += PAGE_SIZE;
        pte = pte.add(1);
    }
    err
}

unsafe fn walk_pte_range(pmd: *mut pmd_t, mut addr: c_ulong, end: c_ulong, walk: *mut mm_walk) -> c_int {
    let mut pte: *mut pte_t = core::ptr::null_mut();
    let mut err = 0;
    let mut ptl: *mut spinlock_t = core::ptr::null_mut();
    if (*walk).no_vma {
        if (*walk).mm == &mut init_mm || addr >= TASK_SIZE { pte = pte_offset_kernel(pmd, addr); }
        else { pte = pte_offset_map(pmd, addr); }
        if !pte.is_null() {
            err = walk_pte_range_inner(pte, addr, end, walk);
            if (*walk).mm != &mut init_mm && addr < TASK_SIZE { pte_unmap(pte); }
        }
    } else {
        pte = pte_offset_map_lock((*walk).mm, pmd, addr, &mut ptl);
        if !pte.is_null() { err = walk_pte_range_inner(pte, addr, end, walk); pte_unmap_unlock(pte, ptl); }
    }
    if pte.is_null() { (*walk).action = ACTION_AGAIN; }
    err
}

unsafe fn walk_pmd_range(pud: *mut pud_t, mut addr: c_ulong, end: c_ulong, walk: *mut mm_walk) -> c_int {
    let pudval = pudp_get(pud); let mut pmd: *mut pmd_t; let mut next;
    let ops = (*walk).ops; let has_handler = !(*ops).pte_entry.is_none(); let has_install = !(*ops).install_pte.is_none();
    let mut err = 0; let depth = real_depth(3);
    if !pud_present(pudval) || pud_leaf(pudval) { (*walk).action = ACTION_AGAIN; return 0; }
    pmd = pmd_offset(pud, addr);
    loop {
        'again: {
            (*walk).action = ACTION_SUBTREE; next = pmd_addr_end(addr, end);
            if pmd_none(*pmd) {
                if has_install { err = __pte_alloc((*walk).mm, pmd); }
                else if !(*ops).pte_hole.is_none() { err = ((*ops).pte_hole.unwrap())(addr, next, depth, walk); }
                if err != 0 { break; } if !has_install { addr = next; continue; }
            }
            if !(*ops).pmd_entry.is_none() { err = ((*ops).pmd_entry.unwrap())(pmd, addr, next, walk); }
            if err != 0 { break; }
            if (*walk).action == ACTION_AGAIN { continue 'again; }
            if (*walk).action == ACTION_CONTINUE { addr = next; continue; }
            if !has_handler { if !has_install { addr = next; continue; } if pmd_present(*pmd) && pmd_trans_huge(*pmd) { addr = next; continue; } }
            if !(*walk).vma.is_null() { split_huge_pmd((*walk).vma, pmd, addr); }
            else if pmd_leaf(*pmd) || !pmd_present(*pmd) { addr = next; continue; }
            err = walk_pte_range(pmd, addr, next, walk); if err != 0 { break; }
            if (*walk).action == ACTION_AGAIN { continue 'again; }
        }
        if err != 0 || next == end { break; }
        pmd = pmd.add(1); addr = next;
    }
    err
}

unsafe fn walk_pud_range(p4d: *mut p4d_t, mut addr: c_ulong, end: c_ulong, walk: *mut mm_walk) -> c_int {
    let mut pud = pud_offset(p4d, addr); let mut next; let ops=(*walk).ops;
    let has_handler=!(*ops).pmd_entry.is_none() || !(*ops).pte_entry.is_none(); let has_install=!(*ops).install_pte.is_none(); let mut err=0; let depth=real_depth(2);
    loop { (*walk).action=ACTION_SUBTREE; next=pud_addr_end(addr,end);
        if pud_none(*pud) { if has_install { err=__pmd_alloc((*walk).mm,pud,addr); } else if !(*ops).pte_hole.is_none() { err=((*ops).pte_hole.unwrap())(addr,next,depth,walk); } if err!=0 {break;} if !has_install {addr=next;continue;} }
        if !(*ops).pud_entry.is_none() {err=((*ops).pud_entry.unwrap())(pud,addr,next,walk);} if err!=0 {break;}
        if (*walk).action==ACTION_AGAIN {continue;} if (*walk).action==ACTION_CONTINUE {addr=next;continue;}
        if !has_handler {if !has_install {addr=next;continue;} if pud_present(*pud)&&pud_trans_huge(*pud){addr=next;continue;}}
        if !(*walk).vma.is_null(){split_huge_pud((*walk).vma,pud,addr);} else if pud_leaf(*pud)||!pud_present(*pud){addr=next;continue;}
        err=walk_pmd_range(pud,addr,next,walk); if err!=0{break;} if (*walk).action==ACTION_AGAIN{continue;} if next==end{break;} pud=pud.add(1);addr=next;
    } err
}

unsafe fn walk_p4d_range(pgd:*mut pgd_t,mut addr:c_ulong,end:c_ulong,walk:*mut mm_walk)->c_int{let mut p4d=p4d_offset(pgd,addr);let ops=(*walk).ops;let has_handler=!(*ops).pud_entry.is_none()||!(*ops).pmd_entry.is_none()||!(*ops).pte_entry.is_none();let has_install=!(*ops).install_pte.is_none();let mut err=0;let depth=real_depth(1);loop{let next=p4d_addr_end(addr,end);if p4d_none_or_clear_bad(p4d){if has_install{err=__pud_alloc((*walk).mm,p4d,addr)}else if !(*ops).pte_hole.is_none(){err=((*ops).pte_hole.unwrap())(addr,next,depth,walk)}if err!=0{break}if !has_install{addr=next;continue}}if !(*ops).p4d_entry.is_none(){err=((*ops).p4d_entry.unwrap())(p4d,addr,next,walk);if err!=0{break}}if has_handler||has_install{err=walk_pud_range(p4d,addr,next,walk)}if err!=0||next==end{break}p4d=p4d.add(1);addr=next}err}

unsafe fn walk_pgd_range(mut addr:c_ulong,end:c_ulong,walk:*mut mm_walk)->c_int{let mut pgd=if !(*walk).pgd.is_null(){(*walk).pgd.add(pgd_index(addr))}else{pgd_offset((*walk).mm,addr)};let ops=(*walk).ops;let has_handler=!(*ops).p4d_entry.is_none()||!(*ops).pud_entry.is_none()||!(*ops).pmd_entry.is_none()||!(*ops).pte_entry.is_none();let has_install=!(*ops).install_pte.is_none();let mut err=0;loop{let next=pgd_addr_end(addr,end);if pgd_none_or_clear_bad(pgd){if has_install{err=__p4d_alloc((*walk).mm,pgd,addr)}else if !(*ops).pte_hole.is_none(){err=((*ops).pte_hole.unwrap())(addr,next,0,walk)}if err!=0{break}if !has_install{addr=next;continue}}if !(*ops).pgd_entry.is_none(){err=((*ops).pgd_entry.unwrap())(pgd,addr,next,walk);if err!=0{break}}if has_handler||has_install{err=walk_p4d_range(pgd,addr,next,walk)}if err!=0||next==end{break}pgd=pgd.add(1);addr=next}err}

#[cfg(feature="CONFIG_HUGETLB_PAGE")] unsafe fn hugetlb_entry_end(h:*mut hstate,addr:c_ulong,end:c_ulong)->c_ulong{min((addr&huge_page_mask(h))+huge_page_size(h),end)}
#[cfg(feature="CONFIG_HUGETLB_PAGE")] unsafe fn walk_hugetlb_range(mut addr:c_ulong,end:c_ulong,walk:*mut mm_walk)->c_int{let vma=(*walk).vma;let h=hstate_vma(vma);let mask=huge_page_mask(h);let sz=huge_page_size(h);let ops=(*walk).ops;let mut err=0;hugetlb_vma_lock_read(vma);loop{let next=hugetlb_entry_end(h,addr,end);let pte=hugetlb_walk(vma,addr&mask,sz);if !pte.is_null(){err=((*ops).hugetlb_entry.unwrap())(pte,mask,addr,next,walk)}else if !(*ops).pte_hole.is_none(){err=((*ops).pte_hole.unwrap())(addr,next,-1,walk)}if err!=0||next==end{break}addr=next}hugetlb_vma_unlock_read(vma);err}
#[cfg(not(feature="CONFIG_HUGETLB_PAGE"))] unsafe fn walk_hugetlb_range(_addr:c_ulong,_end:c_ulong,_walk:*mut mm_walk)->c_int{0}

unsafe fn walk_page_test(start:c_ulong,end:c_ulong,walk:*mut mm_walk)->c_int{let vma=(*walk).vma;let ops=(*walk).ops;if !(*ops).test_walk.is_none(){return ((*ops).test_walk.unwrap())(start,end,walk)}if (*vma).vm_flags&VM_PFNMAP!=0{let mut err=1;if !(*ops).pte_hole.is_none(){err=((*ops).pte_hole.unwrap())(start,end,-1,walk)}return if err!=0{err}else{1}}0}

unsafe fn __walk_page_range(start:c_ulong,end:c_ulong,walk:*mut mm_walk)->c_int{let vma=(*walk).vma;let ops=(*walk).ops;let is_hugetlb=is_vm_hugetlb_page(vma);if !(*ops).install_pte.is_none()&&is_hugetlb{return -EINVAL}let mut err=0;if !(*ops).pre_vma.is_none(){err=((*ops).pre_vma.unwrap())(start,end,walk);if err!=0{return err}}if is_hugetlb{if !(*ops).hugetlb_entry.is_none(){err=walk_hugetlb_range(start,end,walk)}}else{err=walk_pgd_range(start,end,walk)}if !(*ops).post_vma.is_none(){((*ops).post_vma.unwrap())(walk)}err}

unsafe fn process_mm_walk_lock(mm:*mut mm_struct,lock:page_walk_lock){if lock==PGWALK_RDLOCK{mmap_assert_locked(mm)}else if lock!=PGWALK_VMA_RDLOCK_VERIFY{mmap_assert_write_locked(mm)}}
unsafe fn process_vma_walk_lock(vma:*mut vm_area_struct,lock:page_walk_lock){#[cfg(feature="CONFIG_PER_VMA_LOCK")]match lock{PGWALK_WRLOCK=>vma_start_write(vma),PGWALK_WRLOCK_VERIFY=>vma_assert_write_locked(vma),PGWALK_VMA_RDLOCK_VERIFY=>vma_assert_locked(vma),PGWALK_RDLOCK=>(),}}

// Public entry points below retain the C ABI-facing names and semantics.
unsafe fn check_ops_safe(ops:*const mm_walk_ops)->bool{(*ops).install_pte.is_none()}

unsafe fn walk_page_range_mm_unsafe(mm:*mut mm_struct,mut start:c_ulong,end:c_ulong,ops:*const mm_walk_ops,private:*mut c_void)->c_int{let mut walk:mm_walk=core::mem::zeroed();walk.ops=ops;walk.mm=mm;walk.private=private;if start>=end||mm.is_null(){return -EINVAL}process_mm_walk_lock(mm,(*ops).walk_lock);let mut vma=find_vma(mm,start);let mut err=0;loop{let next;if vma.is_null(){walk.vma=core::ptr::null_mut();next=end;if !(*ops).pte_hole.is_none(){err=((*ops).pte_hole.unwrap())(start,next,-1,&mut walk)}}else if start<(*vma).vm_start{walk.vma=core::ptr::null_mut();next=min(end,(*vma).vm_start);if !(*ops).pte_hole.is_none(){err=((*ops).pte_hole.unwrap())(start,next,-1,&mut walk)}}else{process_vma_walk_lock(vma,(*ops).walk_lock);walk.vma=vma;next=min(end,(*vma).vm_end);vma=find_vma(mm,(*walk).vma.as_ref().unwrap().vm_end);err=walk_page_test(start,next,&mut walk);if err>0{err=0;start=next;continue}if err<0{break}err=__walk_page_range(start,next,&mut walk)}if err!=0{break}start=next;if start>=end{break}}err}

unsafe fn walk_page_range(mm:*mut mm_struct,start:c_ulong,end:c_ulong,ops:*const mm_walk_ops,private:*mut c_void)->c_int{if !check_ops_safe(ops){-EINVAL}else{walk_page_range_mm_unsafe(mm,start,end,ops,private)}}

unsafe fn walk_kernel_page_table_range_lockless(start:c_ulong,end:c_ulong,ops:*const mm_walk_ops,pgd:*mut pgd_t,private:*mut c_void)->c_int{let mut walk:mm_walk=core::mem::zeroed();walk.ops=ops;walk.mm=&mut init_mm;walk.pgd=pgd;walk.private=private;walk.no_vma=true;if start>=end||!check_ops_safe(ops){return -EINVAL}walk_pgd_range(start,end,&mut walk)}
unsafe fn walk_kernel_page_table_range(start:c_ulong,end:c_ulong,ops:*const mm_walk_ops,pgd:*mut pgd_t,private:*mut c_void)->c_int{mmap_assert_locked(&mut init_mm);walk_kernel_page_table_range_lockless(start,end,ops,pgd,private)}

unsafe fn walk_page_range_vma_unsafe(vma:*mut vm_area_struct,start:c_ulong,end:c_ulong,ops:*const mm_walk_ops,private:*mut c_void)->c_int{let mut walk:mm_walk=core::mem::zeroed();walk.ops=ops;walk.mm=(*vma).vm_mm;walk.vma=vma;walk.private=private;if start>=end||walk.mm.is_null()||start<(*vma).vm_start||end>(*vma).vm_end{return -EINVAL}process_mm_walk_lock(walk.mm,(*ops).walk_lock);process_vma_walk_lock(vma,(*ops).walk_lock);__walk_page_range(start,end,&mut walk)}
unsafe fn walk_page_range_vma(vma:*mut vm_area_struct,start:c_ulong,end:c_ulong,ops:*const mm_walk_ops,private:*mut c_void)->c_int{if !check_ops_safe(ops){-EINVAL}else{walk_page_range_vma_unsafe(vma,start,end,ops,private)}}
unsafe fn walk_page_vma(vma:*mut vm_area_struct,ops:*const mm_walk_ops,private:*mut c_void)->c_int{if (*vma).vm_mm.is_null()||!check_ops_safe(ops){return -EINVAL}walk_page_range_vma_unsafe(vma,(*vma).vm_start,(*vma).vm_end,ops,private)}

unsafe fn walk_page_mapping(mapping:*mut address_space,first_index:pgoff_t,nr:pgoff_t,ops:*const mm_walk_ops,private:*mut c_void)->c_int{
    if !check_ops_safe(ops){return -EINVAL} lockdep_assert_held(&mut (*mapping).i_mmap_rwsem);
    let mut walk:mm_walk=core::mem::zeroed();walk.ops=ops;walk.private=private;let mut err=0;
    mapping_rmap_tree_foreach!(vma,mapping,first_index,first_index+nr-1,{let vba=vma_start_pgoff(vma);let vea=vba+vma_pages(vma);let cba=max(first_index,vba);let cea=min(first_index+nr,vea);let start_addr=((cba-vba)<<PAGE_SHIFT)+(*vma).vm_start;let end_addr=((cea-vba)<<PAGE_SHIFT)+(*vma).vm_start;if start_addr<end_addr{walk.vma=vma;walk.mm=(*vma).vm_mm;err=walk_page_test((*vma).vm_start,(*vma).vm_end,&mut walk);if err>0{err=0;break}else if err<0{break}err=__walk_page_range(start_addr,end_addr,&mut walk);if err!=0{break}}});err
}

unsafe fn folio_walk_start(fw:*mut folio_walk,vma:*mut vm_area_struct,addr:c_ulong,flags:folio_walk_flags_t)->*mut folio{
    let mut entry_size:c_ulong;let mut zeropage=false;let mut page:*mut page=core::ptr::null_mut();let mut ptl:*mut spinlock_t;let mut pudp:*mut pud_t;let mut pud:pud_t;let mut pmdp:*mut pmd_t;let mut pmd:pmd_t;let mut ptep:*mut pte_t;let mut pte:pte_t;let pgdp:*mut pgd_t;let p4dp:*mut p4d_t;
    mmap_assert_locked((*vma).vm_mm);vma_pgtable_walk_begin(vma);if WARN_ON_ONCE(addr<(*vma).vm_start||addr>=(*vma).vm_end){vma_pgtable_walk_end(vma);return core::ptr::null_mut()}
    pgdp=pgd_offset((*vma).vm_mm,addr);if pgd_none_or_clear_bad(pgdp){vma_pgtable_walk_end(vma);return core::ptr::null_mut()}p4dp=p4d_offset(pgdp,addr);if p4d_none_or_clear_bad(p4dp){vma_pgtable_walk_end(vma);return core::ptr::null_mut()}pudp=pud_offset(p4dp,addr);pud=pudp_get(pudp);if pud_none(pud){vma_pgtable_walk_end(vma);return core::ptr::null_mut()}
    if IS_ENABLED(CONFIG_PGTABLE_HAS_HUGE_LEAVES)&&(!pud_present(pud)||pud_leaf(pud)){ptl=pud_lock((*vma).vm_mm,pudp);pud=pudp_get(pudp);entry_size=PUD_SIZE;(*fw).level=FW_LEVEL_PUD;(*fw).pudp=pudp;(*fw).pud=pud;if pud_none(pud){spin_unlock(ptl);vma_pgtable_walk_end(vma);return core::ptr::null_mut()}else if pud_present(pud)&&!pud_leaf(pud){spin_unlock(ptl)}else if pud_present(pud){page=vm_normal_page_pud(vma,addr,pud);if !page.is_null(){(*fw).ptl=ptl;(*fw).page=page.add(((addr&(entry_size-1))>>PAGE_SHIFT) as usize);return page_folio(page)}spin_unlock(ptl)}else{spin_unlock(ptl)}vma_pgtable_walk_end(vma);return core::ptr::null_mut()}
    pmdp=pmd_offset(pudp,addr);pmd=pmdp_get_lockless(pmdp);if pmd_none(pmd){vma_pgtable_walk_end(vma);return core::ptr::null_mut()}
    if IS_ENABLED(CONFIG_PGTABLE_HAS_HUGE_LEAVES)&&(!pmd_present(pmd)||pmd_leaf(pmd)){ptl=pmd_lock((*vma).vm_mm,pmdp);pmd=pmdp_get(pmdp);entry_size=PMD_SIZE;(*fw).level=FW_LEVEL_PMD;(*fw).pmdp=pmdp;(*fw).pmd=pmd;if pmd_none(pmd){spin_unlock(ptl);vma_pgtable_walk_end(vma);return core::ptr::null_mut()}else if pmd_present(pmd)&&!pmd_leaf(pmd){spin_unlock(ptl)}else if pmd_present(pmd){page=vm_normal_page_pmd(vma,addr,pmd);if page.is_null()&&(flags&FW_ZEROPAGE)!=0&&is_huge_zero_pmd(pmd){page=pfn_to_page(pmd_pfn(pmd));zeropage=true}if !page.is_null(){(*fw).ptl=ptl;if zeropage{(*fw).page=core::ptr::null_mut()}else{(*fw).page=page.add(((addr&(entry_size-1))>>PAGE_SHIFT) as usize)}return page_folio(page)}spin_unlock(ptl)}else{spin_unlock(ptl)}vma_pgtable_walk_end(vma);return core::ptr::null_mut()}
    ptep=pte_offset_map_lock((*vma).vm_mm,pmdp,addr,&mut ptl);if ptep.is_null(){vma_pgtable_walk_end(vma);return core::ptr::null_mut()}pte=ptep_get(ptep);entry_size=PAGE_SIZE;(*fw).level=FW_LEVEL_PTE;(*fw).ptep=ptep;(*fw).pte=pte;if pte_present(pte){page=vm_normal_page(vma,addr,pte);if page.is_null()&&(flags&FW_ZEROPAGE)!=0&&is_zero_pfn(pte_pfn(pte)){page=pfn_to_page(pte_pfn(pte));zeropage=true}if !page.is_null(){(*fw).ptl=ptl;if zeropage{(*fw).page=core::ptr::null_mut()}else{(*fw).page=page.add(((addr&(entry_size-1))>>PAGE_SHIFT) as usize)}return page_folio(page)}}pte_unmap_unlock(ptep,ptl);vma_pgtable_walk_end(vma);core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
