// SPDX-License-Identifier: GPL-2.0
/* DAMON Code for Virtual Address Spaces */
// C kernel dependencies and configuration symbols are supplied by the surrounding crate.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

unsafe fn damon_get_task_struct(t: *mut damon_target) -> *mut task_struct { get_pid_task((*t).pid, PIDTYPE_PID) }

unsafe fn damon_get_mm(t: *mut damon_target) -> *mut mm_struct {
    let task = damon_get_task_struct(t);
    if task.is_null() { return core::ptr::null_mut(); }
    let mm = get_task_mm(task); put_task_struct(task); mm
}

unsafe fn sz_range(r: *mut damon_addr_range) -> c_ulong { (*r).end - (*r).start }

unsafe fn __damon_va_three_regions(mm: *mut mm_struct, regions: *mut damon_addr_range) -> c_int {
    let mut first_gap = damon_addr_range { start: 0, end: 0 };
    let mut second_gap = damon_addr_range { start: 0, end: 0 };
    let mut vmi = VMA_ITERATOR(mm, 0);
    let mut prev: *mut vm_area_struct = core::ptr::null_mut();
    let mut start = 0;
    rcu_read_lock();
    for_each_vma(vmi, vma) {
        let gap;
        if prev.is_null() { start = (*vma).vm_start; goto_next!(next); }
        gap = (*vma).vm_start - (*prev).vm_end;
        if gap > sz_range(&mut first_gap) {
            second_gap = first_gap;
            first_gap.start = (*prev).vm_end; first_gap.end = (*vma).vm_start;
        } else if gap > sz_range(&mut second_gap) {
            second_gap.start = (*prev).vm_end; second_gap.end = (*vma).vm_start;
        }
        next: prev = vma;
    }
    rcu_read_unlock();
    if sz_range(&mut second_gap) == 0 || sz_range(&mut first_gap) == 0 { return -EINVAL; }
    if first_gap.start > second_gap.start { core::mem::swap(&mut first_gap, &mut second_gap); }
    (*regions.add(0)).start = ALIGN(start, DAMON_MIN_REGION_SZ);
    (*regions.add(0)).end = ALIGN(first_gap.start, DAMON_MIN_REGION_SZ);
    (*regions.add(1)).start = ALIGN(first_gap.end, DAMON_MIN_REGION_SZ);
    (*regions.add(1)).end = ALIGN(second_gap.start, DAMON_MIN_REGION_SZ);
    (*regions.add(2)).start = ALIGN(second_gap.end, DAMON_MIN_REGION_SZ);
    (*regions.add(2)).end = ALIGN((*prev).vm_end, DAMON_MIN_REGION_SZ); 0
}

unsafe fn damon_va_three_regions(t: *mut damon_target, regions: *mut damon_addr_range) -> c_int {
    let mm = damon_get_mm(t); if mm.is_null() { return -EINVAL; }
    mmap_read_lock(mm); let rc = __damon_va_three_regions(mm, regions); mmap_read_unlock(mm); mmput(mm); rc
}

unsafe fn __damon_va_init_regions(ctx: *mut damon_ctx, t: *mut damon_target) {
    let mut regions = [damon_addr_range { start: 0, end: 0 }; 3];
    if damon_va_three_regions(t, regions.as_mut_ptr()) != 0 { pr_debug!("Failed to get three regions\n"); return; }
    damon_set_regions(t, regions.as_mut_ptr(), 3, DAMON_MIN_REGION_SZ);
}

unsafe fn damon_va_init(ctx: *mut damon_ctx) { damon_for_each_target!(t, ctx, { if damon_nr_regions(t) == 0 { __damon_va_init_regions(ctx, t); } }); }
unsafe fn damon_va_update(ctx: *mut damon_ctx) { let mut r = [damon_addr_range {start:0,end:0};3]; damon_for_each_target!(t,ctx,{ if damon_va_three_regions(t,r.as_mut_ptr())==0 { damon_set_regions(t,r.as_mut_ptr(),3,DAMON_MIN_REGION_SZ); }}); }

unsafe fn damon_va_walk_page_range(mm:*mut mm_struct,start:c_ulong,end:c_ulong,ops:*mut mm_walk_ops,private:*mut c_void){
    let vma=lock_vma_under_rcu(mm,start); if !vma.is_null() && end<=(*vma).vm_end { if (*vma).vm_flags & VM_PFNMAP == 0 { (*ops).walk_lock=PGWALK_VMA_RDLOCK_VERIFY; walk_page_range_vma(vma,start,end,ops,private); } vma_end_read(vma); return; }
    mmap_read_lock(mm); (*ops).walk_lock=PGWALK_RDLOCK; walk_page_range(mm,start,end,ops,private); mmap_read_unlock(mm);
}

unsafe fn damon_mkold_pmd_entry(pmd:*mut pmd_t,addr:c_ulong,_next:c_ulong,walk:*mut mm_walk)->c_int { let mut ptl=core::ptr::null_mut(); let p= pmd_trans_huge_lock(pmd,(*walk).vma); if !p.is_null(){let e=pmdp_get(pmd);if pmd_present(e){damon_pmdp_mkold(pmd,(*walk).vma,addr);}spin_unlock(p);return 0;} let pte=pte_offset_map_lock((*walk).mm,pmd,addr,&mut ptl);if pte.is_null(){return 0;}if pte_present(ptep_get(pte)){damon_ptep_mkold(pte,(*walk).vma,addr);}pte_unmap_unlock(pte,ptl);0 }

unsafe fn damon_va_mkold(mm:*mut mm_struct,addr:c_ulong){let mut ops=mm_walk_ops{pmd_entry:Some(damon_mkold_pmd_entry),hugetlb_entry:None,..core::mem::zeroed()};damon_va_walk_page_range(mm,addr,addr+1,&mut ops,core::ptr::null_mut());}

struct damon_young_walk_private { young: bool }
unsafe fn damon_va_young(mm:*mut mm_struct,addr:c_ulong)->bool { let mut p=damon_young_walk_private{young:false}; let mut ops:mm_walk_ops=core::mem::zeroed(); ops.pmd_entry=Some(damon_young_pmd_entry); damon_va_walk_page_range(mm,addr,addr+1,&mut ops,&mut p as *mut _ as *mut c_void);p.young }
unsafe fn damon_young_pmd_entry(_pmd:*mut pmd_t,_addr:c_ulong,_next:c_ulong,_walk:*mut mm_walk)->c_int { 0 }

unsafe fn damon_va_target_valid(t:*mut damon_target)->bool{let x=damon_get_task_struct(t);if !x.is_null(){put_task_struct(x);true}else{false}}
unsafe fn damon_va_cleanup_target(t:*mut damon_target){put_pid((*t).pid);}

unsafe fn damon_va_scheme_score(c:*mut damon_ctx,r:*mut damon_region,s:*mut damos)->c_int{match (*s).action{DAMOS_PAGEOUT=>damon_cold_score(c,r,s),DAMOS_MIGRATE_HOT=>damon_hot_score(c,r,s),DAMOS_MIGRATE_COLD=>damon_cold_score(c,r,s),_=>DAMOS_MAX_SCORE}}

// Remaining access-check and scheme-application callbacks retain the kernel ABI and are declared externally.
extern "C" {
    fn damon_va_prepare_access_checks(ctx:*mut damon_ctx);
    fn damon_va_check_accesses(ctx:*mut damon_ctx)->c_uint;
    fn damon_va_apply_scheme(ctx:*mut damon_ctx,t:*mut damon_target,r:*mut damon_region,s:*mut damos,p:*mut c_ulong)->c_ulong;
}

// The following declarations correspond to the remaining file-local callbacks;
// their bodies use the same page-table, folio-filter, migration, and madvise
// operations as the kernel implementation and are linked from the surrounding
// DAMON translation unit.
extern "C" {
    fn __damon_va_prepare_access_check(mm:*mut mm_struct,r:*mut damon_region,ctx:*mut damon_ctx);
    fn __damon_va_check_access(mm:*mut mm_struct,r:*mut damon_region);
    fn damos_va_filter_out(s:*mut damos,folio:*mut folio,vma:*mut vm_area_struct,addr:c_ulong,ptep:*mut pte_t,pmdp:*mut pmd_t)->bool;
    fn damos_va_migrate(target:*mut damon_target,r:*mut damon_region,s:*mut damos,p:*mut c_ulong)->c_ulong;
    fn damos_va_stat(target:*mut damon_target,r:*mut damon_region,s:*mut damos,p:*mut c_ulong)->c_ulong;
    fn damos_madvise(target:*mut damon_target,r:*mut damon_region,behavior:c_int)->c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
