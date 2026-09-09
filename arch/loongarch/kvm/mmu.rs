// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2020-2023 Loongson Technology Corporation Limited */

#[inline]
unsafe fn kvm_hugepage_capable(slot: *mut kvm_memory_slot) -> bool { (*slot).arch.flags & KVM_MEM_HUGEPAGE_CAPABLE != 0 }
#[inline]
unsafe fn kvm_hugepage_incapable(slot: *mut kvm_memory_slot) -> bool { (*slot).arch.flags & KVM_MEM_HUGEPAGE_INCAPABLE != 0 }

#[inline]
unsafe fn kvm_ptw_prepare(kvm: *mut kvm, ctx: *mut kvm_ptw_ctx) {
    (*ctx).level = (*kvm).arch.root_level;
    (*ctx).invalid_ptes = (*kvm).arch.invalid_ptes;
    (*ctx).pte_shifts = (*kvm).arch.pte_shifts;
    (*ctx).pgtable_shift = (*ctx).pte_shifts[(*ctx).level as usize];
    (*ctx).invalid_entry = (*ctx).invalid_ptes[(*ctx).level as usize];
    (*ctx).opaque = kvm as *mut _;
}

unsafe fn kvm_mkold_pte(pte: *mut kvm_pte_t, _addr: phys_addr_t, _ctx: *mut kvm_ptw_ctx) -> i32 {
    if kvm_pte_young(*pte) { *pte = kvm_pte_mkold(*pte); 1 } else { 0 }
}

unsafe fn kvm_mkclean_pte(pte: *mut kvm_pte_t, addr: phys_addr_t, ctx: *mut kvm_ptw_ctx) -> i32 {
    let val = *pte;
    if ((*ctx).flag & _KVM_HAS_PGMASK) != 0 && !kvm_pte_huge(val) {
        let offset = (addr >> PAGE_SHIFT) - (*ctx).gfn;
        if (BIT(offset) & (*ctx).mask) == 0 { return 0; }
    }
    if kvm_pte_dirty(val) { *pte = kvm_pte_mkclean(val); 1 } else { 0 }
}

unsafe fn kvm_flush_pte(pte: *mut kvm_pte_t, _addr: phys_addr_t, ctx: *mut kvm_ptw_ctx) -> i32 {
    let kvm = (*ctx).opaque as *mut kvm;
    if (*ctx).level != 0 { (*kvm).stat.hugepages -= 1; } else { (*kvm).stat.pages -= 1; }
    kvm_set_pte(pte, (*ctx).invalid_entry); 1
}

pub unsafe fn kvm_pgd_alloc() -> *mut kvm_pte_t {
    let pgd = __get_free_pages(GFP_KERNEL, 0) as *mut kvm_pte_t;
    if !pgd.is_null() { pgd_init(pgd as *mut _); }
    pgd
}

unsafe fn _kvm_pte_init(addr: *mut core::ffi::c_void, val: c_ulong) {
    let mut p = addr as *mut c_ulong;
    let end = p.add(PTRS_PER_PTE as usize);
    loop {
        *p.add(0)=val; *p.add(1)=val; *p.add(2)=val; *p.add(3)=val; *p.add(4)=val;
        p = p.add(8); *p.offset(-3)=val; *p.offset(-2)=val; *p.offset(-1)=val;
        if p == end { break; }
    }
}

unsafe fn kvm_populate_gpa(kvm: *mut kvm, cache: *mut kvm_mmu_memory_cache, addr: c_ulong, level: i32) -> *mut kvm_pte_t {
    let mut ctx = core::mem::zeroed::<kvm_ptw_ctx>();
    kvm_ptw_prepare(kvm, &mut ctx);
    let mut child = (*kvm).arch.pgd;
    while ctx.level > level {
        let entry = kvm_pgtable_offset(&mut ctx, child, addr);
        if kvm_pte_none(&ctx, entry) {
            if cache.is_null() { return core::ptr::null_mut(); }
            child = kvm_mmu_memory_cache_alloc(cache);
            _kvm_pte_init(child as *mut _, ctx.invalid_ptes[(ctx.level-1) as usize]);
            smp_wmb(); kvm_set_pte(entry, __pa(child));
        } else if kvm_pte_huge(*entry) { return entry; }
        else { child = __va(PHYSADDR(*entry)) as *mut kvm_pte_t; }
        kvm_ptw_enter(&mut ctx);
    }
    kvm_pgtable_offset(&mut ctx, child, addr)
}

unsafe fn kvm_ptw_leaf(dir: *mut kvm_pte_t, mut addr: phys_addr_t, end: phys_addr_t, ctx: *mut kvm_ptw_ctx) -> i32 {
    let mut ret = 0; let start = addr;
    let child = __va(PHYSADDR(*dir)) as *mut kvm_pte_t;
    let mut entry = kvm_pgtable_offset(ctx, child, addr);
    while addr < end {
        let next = addr + (0x1 as phys_addr_t << (*ctx).pgtable_shift);
        if kvm_pte_present(ctx, entry) { ret |= ((*ctx).ops)(entry, addr, ctx); }
        entry = entry.add(1); addr = next;
    }
    if kvm_need_flush(ctx) {
        let size = 0x1 as phys_addr_t << ((*ctx).pgtable_shift + PAGE_SHIFT - 3);
        if start + size == end { list_add_tail(child as *mut list_head, &mut (*ctx).list); *dir = (*ctx).invalid_ptes[((*ctx).level+1) as usize]; }
    }
    ret
}

unsafe fn kvm_ptw_dir(dir: *mut kvm_pte_t, mut addr: phys_addr_t, end: phys_addr_t, ctx: *mut kvm_ptw_ctx) -> i32 {
    let mut ret=0; let start=addr; let child=__va(PHYSADDR(*dir)) as *mut kvm_pte_t; let mut entry=kvm_pgtable_offset(ctx,child,addr);
    while addr < end {
        let next=kvm_pgtable_addr_end(ctx,addr,end);
        if kvm_pte_present(ctx,entry) {
            if kvm_pte_huge(*entry) { ret |= ((*ctx).ops)(entry,addr,ctx); }
            else { kvm_ptw_enter(ctx); ret |= if (*ctx).level==0 { kvm_ptw_leaf(entry,addr,next,ctx) } else { kvm_ptw_dir(entry,addr,next,ctx) }; kvm_ptw_exit(ctx); }
        }
        entry=entry.add(1); addr=next;
    }
    if kvm_need_flush(ctx) { let size=0x1 as phys_addr_t << ((*ctx).pgtable_shift+PAGE_SHIFT-3); if start+size==end { list_add_tail(child as *mut list_head,&mut (*ctx).list); *dir=(*ctx).invalid_ptes[((*ctx).level+1) as usize]; } }
    ret
}

unsafe fn kvm_ptw_top(dir:*mut kvm_pte_t, mut addr:phys_addr_t, end:phys_addr_t, ctx:*mut kvm_ptw_ctx)->i32 {
    let mut ret=0; let mut entry=kvm_pgtable_offset(ctx,dir,addr);
    while addr<end { let next=kvm_pgtable_addr_end(ctx,addr,end); if kvm_pte_present(ctx,entry) { kvm_ptw_enter(ctx); ret|=kvm_ptw_dir(entry,addr,next,ctx); kvm_ptw_exit(ctx); } entry=entry.add(1); addr=next; } ret
}

unsafe fn kvm_flush_range(kvm:*mut kvm,start_gfn:gfn_t,end_gfn:gfn_t,lock:i32) {
    let mut ctx=core::mem::zeroed::<kvm_ptw_ctx>(); ctx.ops=Some(kvm_flush_pte); ctx.flag=_KVM_FLUSH_PGTABLE; kvm_ptw_prepare(kvm,&mut ctx); INIT_LIST_HEAD(&mut ctx.list);
    if lock!=0 { spin_lock(&mut (*kvm).mmu_lock); kvm_ptw_top((*kvm).arch.pgd,start_gfn<<PAGE_SHIFT,end_gfn<<PAGE_SHIFT,&mut ctx); spin_unlock(&mut (*kvm).mmu_lock); } else { kvm_ptw_top((*kvm).arch.pgd,start_gfn<<PAGE_SHIFT,end_gfn<<PAGE_SHIFT,&mut ctx); }
    kvm_flush_remote_tlbs(kvm); list_for_each_safe!(pos,temp,&mut ctx.list) { list_del(pos); free_page(pos as c_ulong); }
}

unsafe fn kvm_mkclean_gpa_pt(kvm:*mut kvm,start:gfn_t,end:gfn_t)->i32 { let mut c=core::mem::zeroed::<kvm_ptw_ctx>(); c.ops=Some(kvm_mkclean_pte); kvm_ptw_prepare(kvm,&mut c); kvm_ptw_top((*kvm).arch.pgd,start<<PAGE_SHIFT,end<<PAGE_SHIFT,&mut c) }

pub unsafe fn kvm_arch_mmu_enable_log_dirty_pt_masked(kvm:*mut kvm,slot:*mut kvm_memory_slot,gfn_offset:gfn_t,mask:c_ulong) { let base=(*slot).base_gfn+gfn_offset; let start=base+__ffs(mask); let end=base+__fls(mask)+1; let mut c=core::mem::zeroed::<kvm_ptw_ctx>(); c.ops=Some(kvm_mkclean_pte); c.flag=_KVM_HAS_PGMASK; c.mask=mask; c.gfn=base; kvm_ptw_prepare(kvm,&mut c); kvm_ptw_top((*kvm).arch.pgd,start<<PAGE_SHIFT,end<<PAGE_SHIFT,&mut c); }

pub unsafe fn kvm_arch_prepare_memory_region(kvm:*mut kvm, _old:*const kvm_memory_slot,new:*mut kvm_memory_slot,change:kvm_mr_change)->i32 {
    if change!=KVM_MR_MOVE && change!=KVM_MR_CREATE { return 0; }
    if (*new).base_gfn+(*new).npages > (*kvm).arch.gpa_size>>PAGE_SHIFT { return -ENOMEM; }
    (*new).arch.flags=0; let size=(*new).npages*PAGE_SIZE; let gpa=(*new).base_gfn<<PAGE_SHIFT; let hva=(*new).userspace_addr;
    if IS_ALIGNED(size,PMD_SIZE)&&IS_ALIGNED(gpa,PMD_SIZE)&&IS_ALIGNED(hva,PMD_SIZE) { (*new).arch.flags|=KVM_MEM_HUGEPAGE_CAPABLE; }
    else { let go=gpa&(PMD_SIZE-1); let ho=hva&(PMD_SIZE-1); if go!=ho || { let x=if go==0 {PMD_SIZE}else{go}; size+x<PMD_SIZE*2 } { (*new).arch.flags|=KVM_MEM_HUGEPAGE_INCAPABLE; } } 0
}

pub unsafe fn kvm_arch_commit_memory_region(kvm:*mut kvm,old:*mut kvm_memory_slot,new:*const kvm_memory_slot,change:kvm_mr_change) { if change!=KVM_MR_FLAGS_ONLY{return;} let of=if old.is_null(){0}else{(*old).flags}; let nf=if new.is_null(){0}else{(*new).flags}; if (of&nf)&KVM_MEM_READONLY!=0{return;} if of&KVM_MEM_LOG_DIRTY_PAGES==0 && nf&KVM_MEM_LOG_DIRTY_PAGES!=0 { if kvm_dirty_log_manual_protect_and_init_set(kvm){return;} spin_lock(&mut (*kvm).mmu_lock); let n=kvm_mkclean_gpa_pt(kvm,(*new).base_gfn,(*new).base_gfn+(*new).npages); spin_unlock(&mut (*kvm).mmu_lock); if n!=0{kvm_flush_remote_tlbs(kvm);} } }

pub unsafe fn kvm_arch_flush_shadow_all(kvm:*mut kvm){kvm_flush_range(kvm,0,(*kvm).arch.gpa_size>>PAGE_SHIFT,0)}
pub unsafe fn kvm_arch_flush_shadow_memslot(kvm:*mut kvm,slot:*mut kvm_memory_slot){kvm_flush_range(kvm,(*slot).base_gfn,(*slot).base_gfn+(*slot).npages,1)}
pub unsafe fn kvm_unmap_gfn_range(kvm:*mut kvm,range:*mut kvm_gfn_range)->bool { let mut c=core::mem::zeroed::<kvm_ptw_ctx>(); c.ops=Some(kvm_flush_pte); kvm_ptw_prepare(kvm,&mut c); INIT_LIST_HEAD(&mut c.list); kvm_ptw_top((*kvm).arch.pgd,(*range).start<<PAGE_SHIFT,(*range).end<<PAGE_SHIFT,&mut c)!=0 }
pub unsafe fn kvm_age_gfn(kvm:*mut kvm,range:*mut kvm_gfn_range)->bool { let mut c=core::mem::zeroed::<kvm_ptw_ctx>(); c.ops=Some(kvm_mkold_pte); kvm_ptw_prepare(kvm,&mut c); kvm_ptw_top((*kvm).arch.pgd,(*range).start<<PAGE_SHIFT,(*range).end<<PAGE_SHIFT,&mut c)!=0 }
pub unsafe fn kvm_test_age_gfn(kvm:*mut kvm,range:*mut kvm_gfn_range)->bool { let p=kvm_populate_gpa(kvm,core::ptr::null_mut(),(*range).start<<PAGE_SHIFT,0); !p.is_null()&&kvm_pte_present(core::ptr::null(),p)&&kvm_pte_young(*p) }

unsafe fn kvm_map_page_fast(vcpu:*mut kvm_vcpu,gpa:c_ulong,write:bool)->i32 { let kvm=(*vcpu).kvm; let gfn=gpa>>PAGE_SHIFT; spin_lock(&mut (*kvm).mmu_lock); let p=kvm_populate_gpa(kvm,core::ptr::null_mut(),gpa,0); if p.is_null()||!kvm_pte_present(core::ptr::null(),p){spin_unlock(&mut (*kvm).mmu_lock);return -EFAULT;} let mut n=kvm_pte_mkyoung(*p); if write&&!kvm_pte_dirty(n){if !kvm_pte_writeable(n){spin_unlock(&mut (*kvm).mmu_lock);return -EFAULT;} if kvm_pte_huge(n)&&kvm_slot_dirty_track_enabled(gfn_to_memslot(kvm,gfn)){spin_unlock(&mut (*kvm).mmu_lock);return -EFAULT;} n=kvm_pte_mkdirty(n);} let changed=n^*p;if changed!=0{kvm_set_pte(p,n);} spin_unlock(&mut (*kvm).mmu_lock);if kvm_pte_dirty(changed){mark_page_dirty(kvm,gfn);}0 }

unsafe fn fault_supports_huge_mapping(slot:*mut kvm_memory_slot,hva:c_ulong,write:bool)->bool { if kvm_slot_dirty_track_enabled(slot)&&write{return false;} if kvm_hugepage_capable(slot){return true;} if kvm_hugepage_incapable(slot){return false;} let start=(*slot).userspace_addr;let end=start+(*slot).npages*PAGE_SIZE; hva>=ALIGN(start,PMD_SIZE)&&hva<ALIGN_DOWN(end,PMD_SIZE) }

unsafe fn kvm_split_huge(vcpu:*mut kvm_vcpu,ptep:*mut kvm_pte_t,gfn:gfn_t)->*mut kvm_pte_t { let child=kvm_mmu_memory_cache_alloc(&mut (*vcpu).arch.mmu_page_cache);let mut val=kvm_pte_mksmall(*ptep);for i in 0..PTRS_PER_PTE {kvm_set_pte(child.add(i as usize),val);val+=PAGE_SIZE;}smp_wmb();kvm_set_pte(ptep,__pa(child));(*(*vcpu).kvm).stat.hugepages-=1;(*(*vcpu).kvm).stat.pages+=PTRS_PER_PTE;child.add((gfn&(PTRS_PER_PTE-1)) as usize)}

unsafe fn kvm_map_page(vcpu:*mut kvm_vcpu,gpa:c_ulong,write:bool)->i32 { let kvm=(*vcpu).kvm;let gfn=gpa>>PAGE_SHIFT;let fast=kvm_map_page_fast(vcpu,gpa,write);if fast==0{return 0;}let slot=gfn_to_memslot(kvm,gfn);let mut writable=false;let hva=gfn_to_hva_memslot_prot(slot,gfn,&mut writable);if kvm_is_error_hva(hva)||(write&&!writable){return -EFAULT;}let cache=&mut (*vcpu).arch.mmu_page_cache;if kvm_mmu_topup_memory_cache(cache,KVM_MMU_CACHE_MIN_PAGES)!=0{return -EFAULT;}let mut page=core::ptr::null_mut();let pfn=kvm_faultin_pfn(vcpu,gfn,write,&mut writable,&mut page);if is_error_noslot_pfn(pfn){return -EFAULT;}spin_lock(&mut (*kvm).mmu_lock);let prot=_PAGE_PRESENT|__READABLE|if pfn_valid(pfn){_CACHE_CC}else{_CACHE_SUC};let mut pp=prot;if writable{pp=kvm_pte_mkwriteable(pp);if write||!kvm_slot_dirty_track_enabled(slot){pp=kvm_pte_mkdirty(pp);}}let mut level=0;if fault_supports_huge_mapping(slot,hva,write){level=host_pfn_mapping_level(kvm,gfn,slot);}let p=kvm_populate_gpa(kvm,cache,gpa,level);let mut np=kvm_pfn_pte(pfn,__pgprot(pp));if level==1{np=kvm_pte_mkhuge(np);kvm_make_request(KVM_REQ_TLB_FLUSH,vcpu);(*kvm).stat.hugepages+=1;}else if kvm_pte_huge(*p)&&write{let q=kvm_split_huge(vcpu,p,gfn);kvm_set_pte(q,np);spin_unlock(&mut (*kvm).mmu_lock);return 0;}else{(*kvm).stat.pages+=1;}kvm_set_pte(p,np);kvm_release_faultin_page(kvm,page,false,writable);spin_unlock(&mut (*kvm).mmu_lock);if kvm_pte_dirty(pp){mark_page_dirty_in_slot(kvm,slot,gfn);}0 }

unsafe fn host_pfn_mapping_level(_kvm:*mut kvm,_gfn:gfn_t,_slot:*const kvm_memory_slot)->i32 { 0 }

pub unsafe fn kvm_handle_mm_fault(vcpu:*mut kvm_vcpu,gpa:c_ulong,write:bool,ecode:i32)->i32 { let ret=kvm_map_page(vcpu,gpa,write); if ret!=0{return ret;} if !cpu_has_ptw || ecode==EXCCODE_TLBM {(*vcpu).arch.flush_gpa=gpa;kvm_make_request(KVM_REQ_TLB_FLUSH_GPA,vcpu);} 0 }
pub unsafe fn kvm_arch_sync_dirty_log(_kvm:*mut kvm,_memslot:*mut kvm_memory_slot) {}
pub unsafe fn kvm_arch_flush_remote_tlbs_memslot(kvm:*mut kvm,_memslot:*const kvm_memory_slot){kvm_flush_remote_tlbs(kvm)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
