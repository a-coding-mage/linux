/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * KVM/MIPS MMU handling in the KVM module.
 *
 * Copyright (C) 2012  MIPS Technologies, Inc.  All rights reserved.
 * Authors: Sanjay Lal <sanjayl@kymasys.com>
 */

// Linux and architecture headers from the C translation unit provide the
// types, constants, and functions referenced below.

#[cfg(__PAGETABLE_PMD_FOLDED)]
const KVM_MMU_CACHE_MIN_PAGES: u32 = 1;
#[cfg(not(__PAGETABLE_PMD_FOLDED))]
const KVM_MMU_CACHE_MIN_PAGES: u32 = 2;

pub unsafe fn kvm_mmu_free_memory_caches(vcpu: *mut kvm_vcpu) {
    kvm_mmu_free_memory_cache(&mut (*vcpu).arch.mmu_page_cache);
}

/* Initialise a KVM GPA page directory with pointers to the invalid table. */
unsafe fn kvm_pgd_init(page: *mut core::ffi::c_void) {
    let entry: c_ulong = {
        #[cfg(__PAGETABLE_PMD_FOLDED)]
        { invalid_pte_table as c_ulong }
        #[cfg(not(__PAGETABLE_PMD_FOLDED))]
        { invalid_pmd_table as c_ulong }
    };
    let mut p = page as *mut c_ulong;
    let end = p.add(PTRS_PER_PGD as usize);
    loop {
        *p.add(0) = entry; *p.add(1) = entry; *p.add(2) = entry;
        *p.add(3) = entry; *p.add(4) = entry;
        p = p.add(8);
        *p.sub(3) = entry; *p.sub(2) = entry; *p.sub(1) = entry;
        if p == end { break; }
    }
}

pub unsafe fn kvm_pgd_alloc() -> *mut pgd_t {
    let ret = __get_free_pages(GFP_KERNEL, PGD_TABLE_ORDER) as *mut pgd_t;
    if !ret.is_null() { kvm_pgd_init(ret as *mut core::ffi::c_void); }
    ret
}

unsafe fn kvm_mips_walk_pgd(mut pgd: *mut pgd_t, cache: *mut kvm_mmu_memory_cache,
                            addr: c_ulong) -> *mut pte_t {
    pgd = pgd.add(pgd_index(addr) as usize);
    if pgd_none(*pgd) { BUG(); return core::ptr::null_mut(); }
    let p4d = p4d_offset(pgd, addr);
    let pud = pud_offset(p4d, addr);
    if pud_none(*pud) {
        if cache.is_null() { return core::ptr::null_mut(); }
        let new_pmd = kvm_mmu_memory_cache_alloc(cache);
        pmd_init(new_pmd); pud_populate(core::ptr::null_mut(), pud, new_pmd);
    }
    let pmd = pmd_offset(pud, addr);
    if pmd_none(*pmd) {
        if cache.is_null() { return core::ptr::null_mut(); }
        let new_pte = kvm_mmu_memory_cache_alloc(cache);
        clear_page(new_pte as *mut core::ffi::c_void);
        pmd_populate_kernel(core::ptr::null_mut(), pmd, new_pte);
    }
    pte_offset_kernel(pmd, addr)
}

unsafe fn kvm_mips_pte_for_gpa(kvm: *mut kvm, cache: *mut kvm_mmu_memory_cache,
                               addr: c_ulong) -> *mut pte_t {
    kvm_mips_walk_pgd((*kvm).arch.gpa_mm.pgd, cache, addr)
}

unsafe fn kvm_mips_flush_gpa_pte(pte: *mut pte_t, start_gpa: c_ulong, end_gpa: c_ulong) -> bool {
    let i_min = pte_index(start_gpa); let i_max = pte_index(end_gpa);
    let safe = i_min == 0 && i_max == PTRS_PER_PTE - 1;
    for i in i_min..=i_max { if pte_present(*pte.add(i as usize)) { set_pte(pte.add(i as usize), __pte(0)); } }
    safe
}

unsafe fn kvm_mips_flush_gpa_pmd(pmd: *mut pmd_t, mut start: c_ulong, end_gpa: c_ulong) -> bool {
    let mut safe = pmd_index(start) == 0 && pmd_index(end_gpa) == PTRS_PER_PMD - 1;
    let i_max = pmd_index(end_gpa); let mut cur_end = !0 as c_ulong;
    for i in pmd_index(start)..=i_max {
        if pmd_present(*pmd.add(i as usize)) {
            let pte = pte_offset_kernel(pmd.add(i as usize), 0);
            if i == i_max { cur_end = end_gpa; }
            if kvm_mips_flush_gpa_pte(pte, start, cur_end) { pmd_clear(pmd.add(i as usize)); pte_free_kernel(core::ptr::null_mut(), pte); } else { safe = false; }
        }
        start = 0;
    } safe
}

unsafe fn kvm_mips_flush_gpa_pud(pud: *mut pud_t, mut start: c_ulong, end_gpa: c_ulong) -> bool {
    let mut safe = pud_index(start) == 0 && pud_index(end_gpa) == PTRS_PER_PUD - 1;
    let i_max = pud_index(end_gpa); let mut cur_end = !0 as c_ulong;
    for i in pud_index(start)..=i_max {
        if pud_present(*pud.add(i as usize)) {
            let pmd = pmd_offset(pud.add(i as usize), 0); if i == i_max { cur_end = end_gpa; }
            if kvm_mips_flush_gpa_pmd(pmd, start, cur_end) { pud_clear(pud.add(i as usize)); pmd_free(core::ptr::null_mut(), pmd); } else { safe = false; }
        } start = 0;
    } safe
}

unsafe fn kvm_mips_flush_gpa_pgd(pgd: *mut pgd_t, mut start: c_ulong, end_gpa: c_ulong) -> bool {
    let mut safe = pgd_index(start) == 0 && pgd_index(end_gpa) == PTRS_PER_PGD - 1;
    let i_max = pgd_index(end_gpa); let mut cur_end = !0 as c_ulong;
    for i in pgd_index(start)..=i_max {
        if pgd_present(*pgd.add(i as usize)) {
            let p4d = p4d_offset(pgd, 0); let pud = pud_offset(p4d.add(i as usize), 0);
            if i == i_max { cur_end = end_gpa; }
            if kvm_mips_flush_gpa_pud(pud, start, cur_end) { pgd_clear(pgd.add(i as usize)); pud_free(core::ptr::null_mut(), pud); } else { safe = false; }
        } start = 0;
    } safe
}

pub unsafe fn kvm_mips_flush_gpa_pt(kvm: *mut kvm, start_gfn: gfn_t, end_gfn: gfn_t) -> bool {
    kvm_mips_flush_gpa_pgd((*kvm).arch.gpa_mm.pgd, start_gfn << PAGE_SHIFT, end_gfn << PAGE_SHIFT)
}

unsafe fn kvm_mips_range_pte(pte: *mut pte_t, start: c_ulong, end: c_ulong, op: unsafe fn(pte_t) -> pte_t) -> c_int {
    let mut ret = 0; for i in pte_index(start)..=pte_index(end) {
        let p = pte.add(i as usize); if !pte_present(*p) { continue; }
        let old = *p; let new = op(old); if pte_val(new) != pte_val(old) { set_pte(p, new); ret = 1; }
    } ret
}
unsafe fn kvm_mips_range_pmd(pmd: *mut pmd_t, mut start: c_ulong, end: c_ulong, op: unsafe fn(pte_t)->pte_t) -> c_int {
    let mut ret=0; let max=pmd_index(end); let mut ce=!0 as c_ulong; for i in pmd_index(start)..=max { if pmd_present(*pmd.add(i as usize)) { let p=pte_offset_kernel(pmd.add(i as usize),0); if i==max {ce=end;} ret |= kvm_mips_range_pte(p,start,ce,op); } start=0; } ret
}
unsafe fn kvm_mips_range_pud(pud: *mut pud_t, mut start: c_ulong, end: c_ulong, op: unsafe fn(pte_t)->pte_t) -> c_int {
    let mut ret=0; let max=pud_index(end); let mut ce=!0 as c_ulong; for i in pud_index(start)..=max { if pud_present(*pud.add(i as usize)) { let p=pmd_offset(pud.add(i as usize),0); if i==max {ce=end;} ret |= kvm_mips_range_pmd(p,start,ce,op); } start=0; } ret
}
unsafe fn kvm_mips_range_pgd(pgd: *mut pgd_t, mut start: c_ulong, end: c_ulong, op: unsafe fn(pte_t)->pte_t) -> c_int {
    let mut ret=0; let max=pgd_index(end); let mut ce=!0 as c_ulong; for i in pgd_index(start)..=max { if pgd_present(*pgd.add(i as usize)) { let p4=p4d_offset(pgd,0); let p=pud_offset(p4.add(i as usize),0); if i==max {ce=end;} ret |= kvm_mips_range_pud(p,start,ce,op); } start=0; } ret
}

unsafe fn kvm_mips_mkclean_pgd(pgd:*mut pgd_t,s:c_ulong,e:c_ulong)->c_int { kvm_mips_range_pgd(pgd,s,e,pte_mkclean) }
unsafe fn kvm_mips_mkold_pgd(pgd:*mut pgd_t,s:c_ulong,e:c_ulong)->c_int { kvm_mips_range_pgd(pgd,s,e,pte_mkold) }

pub unsafe fn kvm_mips_mkclean_gpa_pt(kvm:*mut kvm,s:gfn_t,e:gfn_t)->c_int { kvm_mips_mkclean_pgd((*kvm).arch.gpa_mm.pgd,s<<PAGE_SHIFT,e<<PAGE_SHIFT) }
pub unsafe fn kvm_arch_mmu_enable_log_dirty_pt_masked(kvm:*mut kvm,slot:*mut kvm_memory_slot,off:gfn_t,mask:c_ulong) { let b=(*slot).base_gfn+off; kvm_mips_mkclean_gpa_pt(kvm,b+__ffs(mask),b+__fls(mask)); }
unsafe fn kvm_mips_mkold_gpa_pt(kvm:*mut kvm,s:gfn_t,e:gfn_t)->c_int { kvm_mips_mkold_pgd((*kvm).arch.gpa_mm.pgd,s<<PAGE_SHIFT,e<<PAGE_SHIFT) }
pub unsafe fn kvm_unmap_gfn_range(kvm:*mut kvm,r:*mut kvm_gfn_range)->bool { kvm_mips_flush_gpa_pt(kvm,(*r).start,(*r).end); true }
pub unsafe fn kvm_age_gfn(kvm:*mut kvm,r:*mut kvm_gfn_range)->bool { kvm_mips_mkold_gpa_pt(kvm,(*r).start,(*r).end) != 0 }
pub unsafe fn kvm_test_age_gfn(kvm:*mut kvm,r:*mut kvm_gfn_range)->bool { let p=kvm_mips_pte_for_gpa(kvm,core::ptr::null_mut(),(*r).start<<PAGE_SHIFT); !p.is_null() && pte_young(*p) }

unsafe fn _kvm_mips_map_page_fast(vcpu:*mut kvm_vcpu,gpa:c_ulong,write_fault:bool,out_entry:*mut pte_t,out_buddy:*mut pte_t)->c_int { let k=(*vcpu).kvm; let gfn=gpa>>PAGE_SHIFT; spin_lock(&mut (*k).mmu_lock); let p=kvm_mips_pte_for_gpa(k,core::ptr::null_mut(),gpa); let mut ret=0; if p.is_null()||!pte_present(*p){ret=-EFAULT;} else { if !pte_young(*p){set_pte(p,pte_mkyoung(*p));} if write_fault&&!pte_dirty(*p){if !pte_write(*p){ret=-EFAULT;}else{set_pte(p,pte_mkdirty(*p));mark_page_dirty(k,gfn);}} if ret==0&&!out_entry.is_null(){*out_entry=*p;} if ret==0&&!out_buddy.is_null(){*out_buddy=*ptep_buddy(p);} } spin_unlock(&mut (*k).mmu_lock); ret }

unsafe fn kvm_mips_map_page(vcpu:*mut kvm_vcpu,gpa:c_ulong,write_fault:bool,out_entry:*mut pte_t,out_buddy:*mut pte_t)->c_int {
    let k=(*vcpu).kvm; let gfn=gpa>>PAGE_SHIFT; let mut idx=srcu_read_lock(&mut (*k).srcu); let mut err=_kvm_mips_map_page_fast(vcpu,gpa,write_fault,out_entry,out_buddy); if err==0 {srcu_read_unlock(&mut (*k).srcu,idx);return 0;}
    let cache=&mut (*vcpu).arch.mmu_page_cache; err=kvm_mmu_topup_memory_cache(cache,KVM_MMU_CACHE_MIN_PAGES); if err!=0 {srcu_read_unlock(&mut (*k).srcu,idx);return err;}
    let seq=(*k).mmu_invalidate_seq; smp_rmb(); let mut writeable=false; let mut page=core::ptr::null_mut(); let pfn=kvm_faultin_pfn(vcpu,gfn,write_fault,&mut writeable,&mut page); if is_error_noslot_pfn(pfn){err=-EFAULT;srcu_read_unlock(&mut (*k).srcu,idx);return err;}
    spin_lock(&mut (*k).mmu_lock); if mmu_invalidate_retry(k,seq){spin_unlock(&mut (*k).mmu_lock);kvm_release_page_unused(page);srcu_read_unlock(&mut (*k).srcu,idx);return kvm_mips_map_page(vcpu,gpa,write_fault,out_entry,out_buddy);}
    let p=kvm_mips_pte_for_gpa(k,cache,gpa); let mut prot=_PAGE_PRESENT|__READABLE|_page_cachable_default; if writeable {prot|=_PAGE_WRITE;if write_fault{prot|=__WRITEABLE;mark_page_dirty(k,gfn);}} let entry=pfn_pte(pfn,__pgprot(prot)); set_pte(p,entry); if !out_entry.is_null(){*out_entry=*p;} if !out_buddy.is_null(){*out_buddy=*ptep_buddy(p);} kvm_release_faultin_page(k,page,false,writeable);spin_unlock(&mut (*k).mmu_lock);srcu_read_unlock(&mut (*k).srcu,idx);0
}

pub unsafe fn kvm_mips_handle_vz_root_tlb_fault(addr:c_ulong,vcpu:*mut kvm_vcpu,write:bool)->c_int { let r=kvm_mips_map_page(vcpu,addr,write,core::ptr::null_mut(),core::ptr::null_mut()); if r!=0 {r} else {kvm_vz_host_tlb_inv(vcpu,addr)} }
unsafe fn kvm_mips_migrate_count(vcpu:*mut kvm_vcpu){if hrtimer_cancel(&mut (*vcpu).arch.comparecount_timer)!=0{hrtimer_restart(&mut (*vcpu).arch.comparecount_timer);}}
pub unsafe fn kvm_arch_vcpu_load(vcpu:*mut kvm_vcpu,cpu:c_int){let mut flags=0;kvm_debug!("%s: vcpu %p, cpu: %d\n",__func__,vcpu,cpu);local_irq_save(&mut flags);(*vcpu).cpu=cpu;if (*vcpu).arch.last_sched_cpu!=cpu{kvm_mips_migrate_count(vcpu);}kvm_mips_callbacks.vcpu_load(vcpu,cpu);local_irq_restore(flags);}
pub unsafe fn kvm_arch_vcpu_put(vcpu:*mut kvm_vcpu){let mut flags=0;local_irq_save(&mut flags);let cpu=smp_processor_id();(*vcpu).arch.last_sched_cpu=cpu;(*vcpu).cpu=-1;kvm_mips_callbacks.vcpu_put(vcpu,cpu);local_irq_restore(flags);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
