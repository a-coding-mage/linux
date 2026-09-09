// SPDX-License-Identifier: GPL-2.0
/* Rust translation of riscv/kvm/mmu.c. External kernel symbols are supplied by dependencies. */

static mut eager_page_split: bool = true;

unsafe fn mmu_wp_memory_region(kvm: *mut kvm, slot: i32) {
    let slots = kvm_memslots(kvm);
    let memslot = id_to_memslot(slots, slot);
    let start = (*memslot).base_gfn << PAGE_SHIFT;
    let end = ((*memslot).base_gfn + (*memslot).npages) << PAGE_SHIFT;
    let mut gstage = kvm_gstage::default();
    kvm_riscv_gstage_init(&mut gstage, kvm);
    write_lock(&mut (*kvm).mmu_lock);
    let flush = kvm_riscv_gstage_wp_range(&mut gstage, start, end);
    write_unlock(&mut (*kvm).mmu_lock);
    if flush { kvm_flush_remote_tlbs_memslot(kvm, memslot); }
}

unsafe fn kvm_riscv_mmu_ioremap(kvm: *mut kvm, gpa: gpa_t, hpa: phys_addr_t,
                                size: c_ulong, writable: bool, in_atomic: bool) -> i32 {
    let mut ret = 0;
    let prot = pgprot_noncached(PAGE_WRITE);
    let mut pfn = __phys_to_pfn(hpa);
    let end = (gpa + size + PAGE_SIZE - 1) & PAGE_MASK;
    let mut pcache = kvm_mmu_memory_cache { gfp_custom: if in_atomic { GFP_ATOMIC | __GFP_ACCOUNT } else { 0 }, gfp_zero: __GFP_ZERO, ..Default::default() };
    let mut gstage = kvm_gstage::default();
    let mut map = kvm_gstage_mapping::default();
    kvm_riscv_gstage_init(&mut gstage, kvm);
    let mut addr = gpa;
    while addr < end {
        map.addr = addr; map.pte = pfn_pte(pfn, prot); map.pte = pte_mkdirty(map.pte); map.level = 0;
        if !writable { map.pte = pte_wrprotect(map.pte); }
        ret = __kvm_mmu_topup_memory_cache(&mut pcache, (*kvm).arch.pgd_levels, (*kvm).arch.pgd_levels);
        if ret != 0 { break; }
        write_lock(&mut (*kvm).mmu_lock);
        ret = kvm_riscv_gstage_set_pte(&mut gstage, &mut pcache, &mut map);
        write_unlock(&mut (*kvm).mmu_lock);
        if ret != 0 { break; }
        pfn += 1; addr += PAGE_SIZE;
    }
    kvm_mmu_free_memory_cache(&mut pcache); ret
}

unsafe fn kvm_riscv_mmu_iounmap(kvm: *mut kvm, gpa: gpa_t, size: c_ulong) {
    let mut gstage = kvm_gstage::default(); kvm_riscv_gstage_init(&mut gstage, kvm);
    write_lock(&mut (*kvm).mmu_lock);
    let flush = kvm_riscv_gstage_unmap_range(&mut gstage, gpa, size, false);
    write_unlock(&mut (*kvm).mmu_lock);
    if flush { kvm_flush_remote_tlbs_range(kvm, gpa >> PAGE_SHIFT, size >> PAGE_SHIFT); }
}

unsafe fn need_topup_split_caches_or_resched(kvm: *mut kvm, count: i32) -> bool {
    if need_resched() || rwlock_needbreak(&(*kvm).mmu_lock) { return true; }
    kvm_mmu_memory_cache_nr_free_objects(&(*kvm).arch.pgd_split_page_cache) < count
}

unsafe fn mmu_split_huge_pages(gstage: *mut kvm_gstage, start: phys_addr_t, end: phys_addr_t) -> bool {
    let kvm = (*gstage).kvm; let pcache = &mut (*kvm).arch.pgd_split_page_cache;
    let mut addr = ALIGN_DOWN(start, PMD_SIZE); let mut last_flush_gfn = addr >> PAGE_SHIFT;
    let count = (*gstage).pgd_levels; let mut flush = false;
    lockdep_assert_held_write(&(*kvm).mmu_lock);
    while addr < end {
        if need_topup_split_caches_or_resched(kvm, count) {
            if flush { kvm_flush_remote_tlbs_range(kvm, last_flush_gfn, (addr >> PAGE_SHIFT) - last_flush_gfn); last_flush_gfn = addr >> PAGE_SHIFT; flush = false; }
            write_unlock(&mut (*kvm).mmu_lock); cond_resched();
            let ret = kvm_mmu_topup_memory_cache(pcache, count);
            if ret != 0 { kvm_err!("Failed to toup split page cache\n"); write_lock(&mut (*kvm).mmu_lock); return flush; }
            write_lock(&mut (*kvm).mmu_lock);
        }
        if (*kvm).arch.pgd.is_null() { return flush; }
        flush |= kvm_riscv_gstage_split_huge(gstage, pcache, addr, 0, false); addr += PMD_SIZE;
    } flush
}

unsafe fn kvm_arch_mmu_enable_log_dirty_pt_masked(kvm: *mut kvm, slot: *mut kvm_memory_slot, gfn_offset: gfn_t, mask: c_ulong) {
    let base_gfn = (*slot).base_gfn + gfn_offset; let start = (base_gfn + __ffs(mask)) << PAGE_SHIFT; let end = (base_gfn + __fls(mask) + 1) << PAGE_SHIFT;
    let mut gstage = kvm_gstage::default(); kvm_riscv_gstage_init(&mut gstage, kvm); kvm_riscv_gstage_wp_pt_masked(&mut gstage, base_gfn, mask);
    if kvm_dirty_log_manual_protect_and_init_set(kvm) && READ_ONCE(eager_page_split) { mmu_split_huge_pages(&mut gstage, start, end); }
}
unsafe fn kvm_arch_sync_dirty_log(_kvm: *mut kvm, _memslot: *mut kvm_memory_slot) {}
unsafe fn kvm_arch_free_memslot(_kvm: *mut kvm, _free: *mut kvm_memory_slot) {}
unsafe fn kvm_arch_memslots_updated(_kvm: *mut kvm, _gen: u64) {}
unsafe fn kvm_arch_flush_shadow_all(kvm: *mut kvm) { kvm_riscv_mmu_free_pgd(kvm); }

unsafe fn kvm_arch_flush_shadow_memslot(kvm: *mut kvm, slot: *mut kvm_memory_slot) {
    let gpa = (*slot).base_gfn << PAGE_SHIFT; let size = (*slot).npages << PAGE_SHIFT; let mut gs = kvm_gstage::default(); kvm_riscv_gstage_init(&mut gs, kvm);
    write_lock(&mut (*kvm).mmu_lock); let flush = kvm_riscv_gstage_unmap_range(&mut gs, gpa, size, false); write_unlock(&mut (*kvm).mmu_lock);
    if flush { kvm_flush_remote_tlbs_range(kvm, gpa >> PAGE_SHIFT, size >> PAGE_SHIFT); }
}

unsafe fn mmu_split_memory_region(kvm: *mut kvm, slot: i32) {
    let ms = id_to_memslot(kvm_memslots(kvm), slot); let start = (*ms).base_gfn << PAGE_SHIFT; let end = ((*ms).base_gfn + (*ms).npages) << PAGE_SHIFT; let mut gs = kvm_gstage::default(); kvm_riscv_gstage_init(&mut gs, kvm);
    write_lock(&mut (*kvm).mmu_lock); let flush = mmu_split_huge_pages(&mut gs, start, end); write_unlock(&mut (*kvm).mmu_lock); if flush { kvm_flush_remote_tlbs_memslot(kvm, ms); }
}

unsafe fn kvm_arch_commit_memory_region(kvm: *mut kvm, _old: *const kvm_memory_slot, new: *const kvm_memory_slot, change: kvm_mr_change) {
    if change != KVM_MR_DELETE && (*new).flags & KVM_MEM_LOG_DIRTY_PAGES != 0 {
        if kvm_dirty_log_manual_protect_and_init_set(kvm) { return; }
        mmu_wp_memory_region(kvm, (*new).id); if READ_ONCE(eager_page_split) { mmu_split_memory_region(kvm, (*new).id); }
    }
}

unsafe fn kvm_arch_prepare_memory_region(kvm: *mut kvm, _old: *const kvm_memory_slot, new: *mut kvm_memory_slot, change: kvm_mr_change) -> i32 {
    if change != KVM_MR_CREATE && change != KVM_MR_MOVE && change != KVM_MR_FLAGS_ONLY { return 0; }
    if (*new).base_gfn + (*new).npages > kvm_riscv_gstage_gpa_size((*kvm).arch.pgd_levels) >> PAGE_SHIFT { return -EFAULT; }
    let mut hva = (*new).userspace_addr; let size = (*new).npages << PAGE_SHIFT; let reg_end = hva + size; let writable = (*new).flags & KVM_MEM_READONLY == 0; let mut ret = 0;
    mmap_read_lock((*current()).mm);
    loop { let vma = find_vma_intersection((*current()).mm, hva, reg_end); if vma.is_null() { break; }
        if writable && (*vma).vm_flags & VM_WRITE == 0 { ret = -EPERM; break; }
        let vm_end = min(reg_end, (*vma).vm_end); if (*vma).vm_flags & VM_PFNMAP != 0 && (*new).flags & KVM_MEM_LOG_DIRTY_PAGES != 0 { ret = -EINVAL; break; } hva = vm_end; if hva >= reg_end { break; }
    } mmap_read_unlock((*current()).mm); ret
}

unsafe fn kvm_unmap_gfn_range(kvm: *mut kvm, range: *mut kvm_gfn_range) -> bool {
    if (*kvm).arch.pgd.is_null() { return false; } lockdep_assert_held_write(&(*kvm).mmu_lock); let mut gs=kvm_gstage::default(); kvm_riscv_gstage_init(&mut gs,kvm);
    let flush=kvm_riscv_gstage_unmap_range(&mut gs,(*range).start<<PAGE_SHIFT,((*range).end-(*range).start)<<PAGE_SHIFT,(*range).may_block); if flush { kvm_flush_remote_tlbs_range(kvm,(*range).start,(*range).end-(*range).start); } false
}

unsafe fn kvm_age_gfn(kvm:*mut kvm, range:*mut kvm_gfn_range)->bool { if (*kvm).arch.pgd.is_null(){return false;} let size=((*range).end-(*range).start)<<PAGE_SHIFT; WARN_ON(size!=PAGE_SIZE&&size!=PMD_SIZE&&size!=PUD_SIZE); let mut gs=kvm_gstage::default(); let mut ptep=core::ptr::null_mut(); let mut level=0; kvm_riscv_gstage_init(&mut gs,kvm); if !kvm_riscv_gstage_get_leaf(&mut gs,(*range).start<<PAGE_SHIFT,&mut ptep,&mut level){return false;} ptep_test_and_clear_young(core::ptr::null_mut(),0,ptep) }
unsafe fn kvm_test_age_gfn(kvm:*mut kvm, range:*mut kvm_gfn_range)->bool { if (*kvm).arch.pgd.is_null(){return false;} let size=((*range).end-(*range).start)<<PAGE_SHIFT; WARN_ON(size!=PAGE_SIZE&&size!=PMD_SIZE&&size!=PUD_SIZE); let mut gs=kvm_gstage::default(); let mut ptep=core::ptr::null_mut(); let mut level=0; kvm_riscv_gstage_init(&mut gs,kvm); if !kvm_riscv_gstage_get_leaf(&mut gs,(*range).start<<PAGE_SHIFT,&mut ptep,&mut level){return false;} pte_young(ptep_get(ptep)) }

unsafe fn fault_supports_gstage_huge_mapping(ms:*mut kvm_memory_slot,hva:usize,map_size:usize)->bool { let size=(*ms).npages*PAGE_SIZE; let us=(*ms).userspace_addr; let ue=us+size; let gs=(*ms).base_gfn<<PAGE_SHIFT; if (gs&(map_size-1))!=(us&(map_size-1)){return false;} hva>=ALIGN(us,map_size)&&hva<ALIGN_DOWN(ue,map_size) }
unsafe fn get_hva_mapping_size(kvm:*mut kvm,hva:usize)->i32 { let mut size=PAGE_SIZE; let mut flags=0; local_irq_save(&mut flags); let pgd=pgdp_get(pgd_offset((*kvm).mm,hva)); if pgd_none(pgd){local_irq_restore(flags);return size;} let p4d=p4dp_get(p4d_offset(&pgd,hva)); if p4d_none(p4d)||!p4d_present(p4d){local_irq_restore(flags);return size;} let pud=pudp_get(pud_offset(&p4d,hva)); if pud_none(pud)||!pud_present(pud){local_irq_restore(flags);return size;} if pud_leaf(pud){size=PUD_SIZE;local_irq_restore(flags);return size;} let pmd=pmdp_get(pmd_offset(&pud,hva)); if !pmd_none(pmd)&&pmd_present(pmd)&&pmd_leaf(pmd){size=PMD_SIZE;} local_irq_restore(flags);size }
unsafe fn transparent_hugepage_adjust(kvm:*mut kvm,ms:*mut kvm_memory_slot,hva:usize,hfnp:*mut kvm_pfn_t,gpa:*mut gpa_t)->usize { if fault_supports_gstage_huge_mapping(ms,hva,PMD_SIZE){let sz=get_hva_mapping_size(kvm,hva);if sz<PMD_SIZE{return sz as usize;} *gpa&=PMD_MASK;*hfnp&=!(PTRS_PER_PMD-1);return PMD_SIZE;} PAGE_SIZE }
unsafe fn hugetlb_mapping_size(ms:*mut kvm_memory_slot,hva:usize,map_size:usize)->usize { if map_size==PUD_SIZE&&!cfg!(target_pointer_width="32")&&fault_supports_gstage_huge_mapping(ms,hva,PUD_SIZE){return PUD_SIZE;} if map_size>=PMD_SIZE&&fault_supports_gstage_huge_mapping(ms,hva,PMD_SIZE){return PMD_SIZE;} if map_size==PAGE_SIZE||map_size>=PMD_SIZE{return PAGE_SIZE;} map_size }

unsafe fn kvm_riscv_mmu_dirty_log_write_fault_fast(kvm:*mut kvm,memslot:*mut kvm_memory_slot,gpa:gpa_t,out:*mut kvm_gstage_mapping)->bool { let mut gs=kvm_gstage::default(); let seq=(*kvm).mmu_invalidate_seq; let gfn=gpa>>PAGE_SHIFT; let mut ptep=core::ptr::null_mut();let mut level=0; kvm_riscv_gstage_init(&mut gs,kvm);read_lock(&(*kvm).mmu_lock); if mmu_invalidate_retry_gfn(kvm,seq,gfn)||!kvm_riscv_gstage_get_leaf(&mut gs,gpa,&mut ptep,&mut level)||level!=0 {read_unlock(&(*kvm).mmu_lock);return false;} let mut dirty=false;let mut new_pte=Default::default(); loop {let old=ptep_get(ptep);if pte_val(old)&_PAGE_LEAF==0{read_unlock(&(*kvm).mmu_lock);return false;}if !dirty{mark_page_dirty_in_slot(kvm,memslot,gfn);dirty=true;}if pte_val(old)&(_PAGE_WRITE|_PAGE_DIRTY)==(_PAGE_WRITE|_PAGE_DIRTY){new_pte=old;break;}new_pte=pte_mkdirty(pte_mkwrite_novma(old));if kvm_riscv_gstage_try_update_pte(&mut gs,level,gpa,ptep,old,new_pte){break;}cpu_relax();}read_unlock(&(*kvm).mmu_lock);(*out).addr=gpa&PAGE_MASK;(*out).level=0;(*out).pte=new_pte;true }

// The remaining mapping/allocation routines retain the C control flow and use external kernel types.
unsafe fn kvm_riscv_mmu_map(vcpu:*mut kvm_vcpu,memslot:*mut kvm_memory_slot,mut gpa:gpa_t,hva:usize,is_write:bool,out:*mut kvm_gstage_mapping)->i32 {
    let kvm=(*vcpu).kvm; let cache=&mut (*vcpu).arch.mmu_page_cache; let logging=kvm_slot_dirty_track_enabled(memslot)&&(*memslot).flags&KVM_MEM_READONLY==0; (*out)=Default::default(); if is_write&&logging&&kvm_riscv_mmu_dirty_log_write_fault_fast(kvm,memslot,gpa,out){return 0;}
    let mut ret=kvm_mmu_topup_memory_cache(cache,(*kvm).arch.pgd_levels);if ret!=0{return ret;} mmap_read_lock((*current()).mm);let vma=vma_lookup((*current()).mm,hva);if vma.is_null(){mmap_read_unlock((*current()).mm);return -EFAULT;}let hugetlb=is_vm_hugetlb_page(vma);let shift=if hugetlb{huge_page_shift(hstate_vma(vma))}else{PAGE_SHIFT};let mut pagesize=1usize<<shift;if logging||(*vma).vm_flags&VM_PFNMAP!=0{pagesize=PAGE_SIZE;}else if hugetlb{pagesize=hugetlb_mapping_size(memslot,hva,pagesize);}if pagesize==PMD_SIZE||pagesize==PUD_SIZE{gpa=ALIGN_DOWN(gpa,pagesize);}let seq=(*kvm).mmu_invalidate_seq;mmap_read_unlock((*current()).mm);if pagesize!=PUD_SIZE&&pagesize!=PMD_SIZE&&pagesize!=PAGE_SIZE{return -EFAULT;}let mut writable=false;let mut page=core::ptr::null_mut();let hfn=__kvm_faultin_pfn(memslot,gpa>>PAGE_SHIFT,if is_write{FOLL_WRITE}else{0},&mut writable,&mut page);if hfn==KVM_PFN_ERR_HWPOISON{return 0;}if is_error_noslot_pfn(hfn){return -EFAULT;}if logging&&!is_write{writable=false;}let mut gs=kvm_gstage::default();kvm_riscv_gstage_init(&mut gs,kvm);write_lock(&mut (*kvm).mmu_lock);if mmu_invalidate_retry(kvm,seq){write_unlock(&mut (*kvm).mmu_lock);return 0;}if !logging&&!hugetlb&&pagesize==PAGE_SIZE{pagesize=transparent_hugepage_adjust(kvm,memslot,hva,&mut (hfn as kvm_pfn_t),&mut gpa);}if writable{mark_page_dirty_in_slot(kvm,memslot,gpa>>PAGE_SHIFT);ret=kvm_riscv_gstage_map_page(&mut gs,cache,gpa,hfn<<PAGE_SHIFT,pagesize,false,true,out);}else{ret=kvm_riscv_gstage_map_page(&mut gs,cache,gpa,hfn<<PAGE_SHIFT,pagesize,true,true,out);}kvm_release_faultin_page(kvm,page,ret!=0&&ret!=-EEXIST,writable);write_unlock(&mut (*kvm).mmu_lock);ret
}

unsafe fn kvm_riscv_mmu_alloc_pgd(kvm:*mut kvm)->i32 { if !(*kvm).arch.pgd.is_null(){return -EINVAL;}let p=alloc_pages(GFP_KERNEL_ACCOUNT|__GFP_ZERO,get_order(kvm_riscv_gstage_pgd_size()));if p.is_null(){return -ENOMEM;}(*kvm).arch.pgd=page_to_virt(p);(*kvm).arch.pgd_phys=page_to_phys(p);(*kvm).arch.pgd_levels=kvm_riscv_gstage_max_pgd_levels;(*kvm).arch.pgd_split_page_cache.gfp_zero=__GFP_ZERO;0 }
unsafe fn kvm_riscv_mmu_free_pgd(kvm:*mut kvm){let mut gs=kvm_gstage::default();let mut pgd=core::ptr::null_mut();let mut flush=false;write_lock(&mut (*kvm).mmu_lock);if !(*kvm).arch.pgd.is_null(){kvm_riscv_gstage_init(&mut gs,kvm);flush=kvm_riscv_gstage_unmap_range(&mut gs,0,kvm_riscv_gstage_gpa_size((*kvm).arch.pgd_levels),false);pgd=READ_ONCE((*kvm).arch.pgd);(*kvm).arch.pgd=core::ptr::null_mut();(*kvm).arch.pgd_phys=0;(*kvm).arch.pgd_levels=0;}write_unlock(&mut (*kvm).mmu_lock);if flush{kvm_flush_remote_tlbs(kvm);}if !pgd.is_null(){free_pages(pgd as usize,get_order(kvm_riscv_gstage_pgd_size()));}kvm_mmu_free_memory_cache(&mut (*kvm).arch.pgd_split_page_cache);}
unsafe fn kvm_riscv_mmu_update_hgatp(vcpu:*mut kvm_vcpu){let ka=&mut (*(*vcpu).kvm).arch;let mut hgatp=kvm_riscv_gstage_mode(ka.pgd_levels)<<HGATP_MODE_SHIFT;hgatp|=(READ_ONCE(ka.vmid.vmid)<<HGATP_VMID_SHIFT)&HGATP_VMID;hgatp|=(ka.pgd_phys>>PAGE_SHIFT)&HGATP_PPN;ncsr_write(CSR_HGATP,hgatp);if kvm_riscv_gstage_vmid_bits()==0{kvm_riscv_local_hfence_gvma_all();}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
