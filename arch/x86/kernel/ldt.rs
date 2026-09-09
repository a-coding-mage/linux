// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of ldt.c; external kernel symbols are supplied by dependencies. */

const LDT_SLOT_STRIDE: usize = LDT_ENTRIES * LDT_ENTRY_SIZE;

#[inline]
unsafe fn ldt_slot_va(slot: i32) -> *mut core::ffi::c_void {
    (LDT_BASE_ADDR + LDT_SLOT_STRIDE * slot as usize) as *mut core::ffi::c_void
}

pub unsafe fn load_mm_ldt(mm: *mut mm_struct) {
    let ldt = core::ptr::read_volatile(&(*mm).context.ldt);
    if !ldt.is_null() {
        if cpu_feature_enabled(X86_FEATURE_PTI) {
            if WARN_ON_ONCE((*ldt).slot as usize > 1) {
                clear_LDT();
                return;
            }
            set_ldt(ldt_slot_va((*ldt).slot), (*ldt).nr_entries);
        } else { set_ldt((*ldt).entries, (*ldt).nr_entries); }
    } else { clear_LDT(); }
}

pub unsafe fn switch_ldt(prev: *mut mm_struct, next: *mut mm_struct) {
    if ((*prev).context.ldt as usize) | ((*next).context.ldt as usize) != 0 { load_mm_ldt(next); }
    DEBUG_LOCKS_WARN_ON(preemptible());
}

unsafe fn refresh_ldt_segments() {
    #[cfg(CONFIG_X86_64)] {
        let mut sel: u16 = 0;
        savesegment(ds, &mut sel); if (sel & SEGMENT_TI_MASK) == SEGMENT_LDT { loadsegment(ds, sel); }
        savesegment(es, &mut sel); if (sel & SEGMENT_TI_MASK) == SEGMENT_LDT { loadsegment(es, sel); }
    }
}

unsafe fn flush_ldt(arg: *mut core::ffi::c_void) {
    let mm = arg as *mut mm_struct;
    if this_cpu_read(cpu_tlbstate.loaded_mm) != mm { return; }
    load_mm_ldt(mm); refresh_ldt_segments();
}

unsafe fn alloc_ldt_struct(num_entries: u32) -> *mut ldt_struct {
    if num_entries > LDT_ENTRIES as u32 { return core::ptr::null_mut(); }
    let new_ldt = kmalloc_obj::<ldt_struct>(GFP_KERNEL_ACCOUNT);
    if new_ldt.is_null() { return core::ptr::null_mut(); }
    let alloc_size = num_entries as usize * LDT_ENTRY_SIZE;
    (*new_ldt).entries = if alloc_size > PAGE_SIZE { __vmalloc(alloc_size, GFP_KERNEL_ACCOUNT | __GFP_ZERO) } else { get_zeroed_page(GFP_KERNEL_ACCOUNT) as *mut _ };
    if (*new_ldt).entries.is_null() { kfree(new_ldt); return core::ptr::null_mut(); }
    (*new_ldt).slot = -1; (*new_ldt).nr_entries = num_entries; new_ldt
}

#[cfg(CONFIG_MITIGATION_PAGE_TABLE_ISOLATION)]
unsafe fn do_sanity_check(mm: *mut mm_struct, had_kernel_mapping: bool, had_user_mapping: bool) {
    if !(*mm).context.ldt.is_null() { WARN_ON(!had_kernel_mapping); if boot_cpu_has(X86_FEATURE_PTI) { WARN_ON(!had_user_mapping); } }
    else { WARN_ON(had_kernel_mapping); if boot_cpu_has(X86_FEATURE_PTI) { WARN_ON(had_user_mapping); } }
}

#[cfg(all(CONFIG_MITIGATION_PAGE_TABLE_ISOLATION, CONFIG_X86_PAE))]
unsafe fn pgd_to_pmd_walk(pgd: *mut pgd_t, va: usize) -> *mut pmd_t {
    if (*pgd).pgd == 0 { return core::ptr::null_mut(); }
    let p4d = p4d_offset(pgd, va); if p4d_none(*p4d) { return core::ptr::null_mut(); }
    let pud = pud_offset(p4d, va); if pud_none(*pud) { return core::ptr::null_mut(); }
    pmd_offset(pud, va)
}

#[cfg(all(CONFIG_MITIGATION_PAGE_TABLE_ISOLATION, CONFIG_X86_PAE))]
unsafe fn map_ldt_struct_to_user(mm: *mut mm_struct) { let k = pgd_offset(mm, LDT_BASE_ADDR); let u = kernel_to_user_pgdp(k); let kp = pgd_to_pmd_walk(k, LDT_BASE_ADDR); let up = pgd_to_pmd_walk(u, LDT_BASE_ADDR); if boot_cpu_has(X86_FEATURE_PTI) && (*mm).context.ldt.is_null() { set_pmd(up, *kp); } }
#[cfg(all(CONFIG_MITIGATION_PAGE_TABLE_ISOLATION, CONFIG_X86_PAE))]
unsafe fn sanity_check_ldt_mapping(mm: *mut mm_struct) { let k = pgd_offset(mm, LDT_BASE_ADDR); let u = kernel_to_user_pgdp(k); let kp = pgd_to_pmd_walk(k, LDT_BASE_ADDR); let up = pgd_to_pmd_walk(u, LDT_BASE_ADDR); do_sanity_check(mm, (*kp).pmd != 0, (*up).pmd != 0); }
#[cfg(all(CONFIG_MITIGATION_PAGE_TABLE_ISOLATION, not(CONFIG_X86_PAE)))]
unsafe fn map_ldt_struct_to_user(mm: *mut mm_struct) { let p = pgd_offset(mm, LDT_BASE_ADDR); if boot_cpu_has(X86_FEATURE_PTI) && (*mm).context.ldt.is_null() { set_pgd(kernel_to_user_pgdp(p), *p); } }
#[cfg(all(CONFIG_MITIGATION_PAGE_TABLE_ISOLATION, not(CONFIG_X86_PAE)))]
unsafe fn sanity_check_ldt_mapping(mm: *mut mm_struct) { let p = pgd_offset(mm, LDT_BASE_ADDR); do_sanity_check(mm, (*p).pgd != 0, (*kernel_to_user_pgdp(p)).pgd != 0); }

#[cfg(CONFIG_MITIGATION_PAGE_TABLE_ISOLATION)]
unsafe fn map_ldt_struct(mm: *mut mm_struct, ldt: *mut ldt_struct, slot: i32) -> i32 {
    if !boot_cpu_has(X86_FEATURE_PTI) { return 0; } WARN_ON((*ldt).slot != -1); sanity_check_ldt_mapping(mm);
    let pages = ((*ldt).nr_entries as usize * LDT_ENTRY_SIZE + PAGE_SIZE - 1) / PAGE_SIZE;
    for i in 0..pages { let off = i << PAGE_SHIFT; let src = ((*ldt).entries as *mut u8).add(off); let pfn = if is_vmalloc_addr(src) { vmalloc_to_pfn(src) } else { page_to_pfn(virt_to_page(src)) }; let va = ldt_slot_va(slot) as usize + off; let mut ptl = core::ptr::null_mut(); let ptep = get_locked_pte(mm, va, &mut ptl); if ptep.is_null() { return -ENOMEM; } let mut prot = __pgprot(__PAGE_KERNEL_RO & !_PAGE_GLOBAL); pgprot_val(&mut prot) &= __supported_pte_mask; set_pte_at(mm, va, ptep, pfn_pte(pfn, prot)); pte_unmap_unlock(ptep, ptl); }
    map_ldt_struct_to_user(mm); (*ldt).slot = slot; 0
}
#[cfg(not(CONFIG_MITIGATION_PAGE_TABLE_ISOLATION))] unsafe fn map_ldt_struct(_: *mut mm_struct, _: *mut ldt_struct, _: i32) -> i32 { 0 }

unsafe fn unmap_ldt_struct(mm: *mut mm_struct, ldt: *mut ldt_struct) { if ldt.is_null() || !boot_cpu_has(X86_FEATURE_PTI) { return; } let pages = ((*ldt).nr_entries as usize * LDT_ENTRY_SIZE + PAGE_SIZE - 1) / PAGE_SIZE; for i in 0..pages { let va = ldt_slot_va((*ldt).slot) as usize + (i << PAGE_SHIFT); let mut ptl = core::ptr::null_mut(); let ptep = get_locked_pte(mm, va, &mut ptl); if !WARN_ON_ONCE(ptep.is_null()) { pte_clear(mm, va, ptep); pte_unmap_unlock(ptep, ptl); } } let va = ldt_slot_va((*ldt).slot) as usize; flush_tlb_mm_range(mm, va, va + pages * PAGE_SIZE, PAGE_SHIFT, false); }

unsafe fn free_ldt_pgtables(mm: *mut mm_struct) { #[cfg(CONFIG_MITIGATION_PAGE_TABLE_ISOLATION)] { if !boot_cpu_has(X86_FEATURE_PTI) { return; } let mut tlb = core::mem::zeroed(); tlb_gather_mmu_fullmm(&mut tlb, mm); free_pgd_range(&mut tlb, LDT_BASE_ADDR, LDT_END_ADDR, LDT_BASE_ADDR, LDT_END_ADDR); tlb_finish_mmu(&mut tlb); } }
unsafe fn finalize_ldt_struct(ldt: *mut ldt_struct) { paravirt_alloc_ldt((*ldt).entries, (*ldt).nr_entries); }
unsafe fn install_ldt(mm: *mut mm_struct, ldt: *mut ldt_struct) { mutex_lock(&mut (*mm).context.lock); smp_store_release(&mut (*mm).context.ldt, ldt); on_each_cpu_mask(mm_cpumask(mm), flush_ldt, mm as *mut _, true); mutex_unlock(&mut (*mm).context.lock); }
unsafe fn free_ldt_struct(ldt: *mut ldt_struct) { if ldt.is_null() { return; } paravirt_free_ldt((*ldt).entries, (*ldt).nr_entries); if (*ldt).nr_entries as usize * LDT_ENTRY_SIZE > PAGE_SIZE { vfree_atomic((*ldt).entries); } else { free_page((*ldt).entries as usize); } kfree(ldt); }

pub unsafe fn ldt_dup_context(old_mm: *mut mm_struct, mm: *mut mm_struct) -> i32 { if old_mm.is_null() { return 0; } mutex_lock(&mut (*old_mm).context.lock); let old = (*old_mm).context.ldt; if old.is_null() { mutex_unlock(&mut (*old_mm).context.lock); return 0; } let new = alloc_ldt_struct((*old).nr_entries); if new.is_null() { mutex_unlock(&mut (*old_mm).context.lock); return -ENOMEM; } core::ptr::copy_nonoverlapping((*old).entries, (*new).entries, (*new).nr_entries as usize * LDT_ENTRY_SIZE); finalize_ldt_struct(new); let ret = map_ldt_struct(mm, new, 0); if ret != 0 { free_ldt_pgtables(mm); free_ldt_struct(new); mutex_unlock(&mut (*old_mm).context.lock); return ret; } (*mm).context.ldt = new; mutex_unlock(&mut (*old_mm).context.lock); 0 }
pub unsafe fn destroy_context_ldt(mm: *mut mm_struct) { free_ldt_struct((*mm).context.ldt); (*mm).context.ldt = core::ptr::null_mut(); }
pub unsafe fn ldt_arch_exit_mmap(mm: *mut mm_struct) { free_ldt_pgtables(mm); }

unsafe fn read_ldt(ptr: *mut core::ffi::c_void, mut bytecount: usize) -> i32 { let mm = current->mm; down_read(&mut (*mm).context.ldt_usr_sem); let ldt = (*mm).context.ldt; if ldt.is_null() { up_read(&mut (*mm).context.ldt_usr_sem); return 0; } bytecount = bytecount.min(LDT_ENTRY_SIZE * LDT_ENTRIES); let size = ( (*ldt).nr_entries as usize * LDT_ENTRY_SIZE).min(bytecount); if copy_to_user(ptr, (*ldt).entries, size) != 0 { up_read(&mut (*mm).context.ldt_usr_sem); return -EFAULT; } if size != bytecount && clear_user(ptr.add(size), bytecount-size) != 0 { up_read(&mut (*mm).context.ldt_usr_sem); return -EFAULT; } up_read(&mut (*mm).context.ldt_usr_sem); bytecount as i32 }
unsafe fn read_default_ldt(ptr: *mut core::ffi::c_void, mut bytecount: usize) -> i32 { let size = if cfg!(CONFIG_X86_32) { 5 * core::mem::size_of::<desc_struct>() } else { 128 }; bytecount = bytecount.min(size); if clear_user(ptr, bytecount) != 0 { -EFAULT } else { bytecount as i32 } }
unsafe fn allow_16bit_segments() -> bool { if !cfg!(CONFIG_X86_16BIT) { return false; } #[cfg(CONFIG_XEN_PV)] { if xen_pv_domain() { pr_info_once!("Warning: 16-bit segments do not work correctly in a Xen PV guest\n"); return false; } } true }

unsafe fn write_ldt(ptr: *mut core::ffi::c_void, bytecount: usize, oldmode: i32) -> i32 { let mm=current->mm; let mut info: user_desc=core::mem::zeroed(); if bytecount != core::mem::size_of::<user_desc>() { return -EINVAL; } if copy_from_user(&mut info,ptr,core::mem::size_of::<user_desc>()) != 0 { return -EFAULT; } if info.entry_number >= LDT_ENTRIES { return -EINVAL; } if info.contents == 3 && (oldmode != 0 || info.seg_not_present == 0) { return -EINVAL; } let mut ldt: desc_struct=core::mem::zeroed(); if (oldmode != 0 && info.base_addr == 0 && info.limit == 0) || LDT_empty(&info) { } else { if !info.seg_32bit && !allow_16bit_segments() { return -EINVAL; } fill_ldt(&mut ldt,&info); if oldmode != 0 { ldt.avl=0; } } if down_write_killable(&mut (*mm).context.ldt_usr_sem) != 0 { return -EINTR; } let old=(*mm).context.ldt; let oldn=if old.is_null(){0}else{(*old).nr_entries}; let newn=(info.entry_number+1).max(oldn); let new=alloc_ldt_struct(newn); if new.is_null(){up_write(&mut (*mm).context.ldt_usr_sem);return -ENOMEM;} if !old.is_null(){core::ptr::copy_nonoverlapping((*old).entries,(*new).entries,oldn as usize*LDT_ENTRY_SIZE);} *((*new).entries as *mut desc_struct).add(info.entry_number as usize)=ldt; finalize_ldt_struct(new); let e=map_ldt_struct(mm,new,if old.is_null(){0}else{(!(*old).slot) as i32}); if e!=0 { if old.is_null(){free_ldt_pgtables(mm);} free_ldt_struct(new); up_write(&mut (*mm).context.ldt_usr_sem); return e; } install_ldt(mm,new); unmap_ldt_struct(mm,old); free_ldt_struct(old); up_write(&mut (*mm).context.ldt_usr_sem); 0 }

pub unsafe fn sys_modify_ldt(func: i32, ptr: *mut core::ffi::c_void, bytecount: usize) -> u32 { let ret=match func { 0=>read_ldt(ptr,bytecount), 1=>write_ldt(ptr,bytecount,1), 2=>read_default_ldt(ptr,bytecount), 0x11=>write_ldt(ptr,bytecount,0), _=>-ENOSYS }; ret as u32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
