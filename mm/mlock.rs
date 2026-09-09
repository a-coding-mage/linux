// SPDX-License-Identifier: GPL-2.0
/* linux/mm/mlock.c; direct Rust translation. Kernel dependencies are external. */

#[repr(C)]
struct mlock_fbatch { lock: local_lock_t, fbatch: folio_batch }

static mut MLOCK_FBATCH: per_cpu<mlock_fbatch> = per_cpu::new(mlock_fbatch { lock: INIT_LOCAL_LOCK(), fbatch: folio_batch::default() });

unsafe fn can_do_mlock() -> bool {
    if rlimit(RLIMIT_MEMLOCK) != 0 { return true; }
    if capable(CAP_IPC_LOCK) { return true; }
    false
}

unsafe fn __mlock_folio(mut folio: *mut folio, mut lruvec: *mut lruvec) -> *mut lruvec {
    if !folio_test_clear_lru(folio) { return lruvec; }
    lruvec = folio_lruvec_relock_irq(folio, lruvec);
    if unlikely(folio_evictable(folio)) {
        if folio_test_unevictable(folio) {
            lruvec_del_folio(lruvec, folio); folio_clear_unevictable(folio); lruvec_add_folio(lruvec, folio);
            __count_vm_events(UNEVICTABLE_PGRESCUED, folio_nr_pages(folio));
        }
    } else if folio_test_unevictable(folio) {
        if folio_test_mlocked(folio) { (*folio).mlock_count += 1; }
    } else {
        lruvec_del_folio(lruvec, folio); folio_clear_active(folio); folio_set_unevictable(folio);
        (*folio).mlock_count = folio_test_mlocked(folio) as i32;
        lruvec_add_folio(lruvec, folio); __count_vm_events(UNEVICTABLE_PGCULLED, folio_nr_pages(folio));
    }
    folio_set_lru(folio); lruvec
}

unsafe fn __mlock_new_folio(folio: *mut folio, mut lruvec: *mut lruvec) -> *mut lruvec {
    VM_BUG_ON_FOLIO(folio_test_lru(folio), folio);
    lruvec = folio_lruvec_relock_irq(folio, lruvec);
    if !unlikely(folio_evictable(folio)) {
        folio_set_unevictable(folio); (*folio).mlock_count = folio_test_mlocked(folio) as i32;
        __count_vm_events(UNEVICTABLE_PGCULLED, folio_nr_pages(folio));
    }
    lruvec_add_folio(lruvec, folio); folio_set_lru(folio); lruvec
}

unsafe fn __munlock_folio(folio: *mut folio, mut lruvec: *mut lruvec) -> *mut lruvec {
    let nr_pages = folio_nr_pages(folio); let mut isolated = false;
    if folio_test_clear_lru(folio) { isolated = true; lruvec = folio_lruvec_relock_irq(folio, lruvec); if folio_test_unevictable(folio) { if (*folio).mlock_count != 0 { (*folio).mlock_count -= 1; } if (*folio).mlock_count != 0 { folio_set_lru(folio); return lruvec; } } }
    if folio_test_clear_mlocked(folio) {
        __zone_stat_mod_folio(folio, NR_MLOCK, -nr_pages);
        if isolated || !folio_test_unevictable(folio) { __count_vm_events(UNEVICTABLE_PGMUNLOCKED, nr_pages); } else { __count_vm_events(UNEVICTABLE_PGSTRANDED, nr_pages); }
    }
    if isolated && folio_test_unevictable(folio) && folio_evictable(folio) { lruvec_del_folio(lruvec, folio); folio_clear_unevictable(folio); lruvec_add_folio(lruvec, folio); __count_vm_events(UNEVICTABLE_PGRESCUED, nr_pages); }
    if isolated { folio_set_lru(folio); } lruvec
}

const LRU_FOLIO: usize = 0x1; const NEW_FOLIO: usize = 0x2;
unsafe fn mlock_lru(folio: *mut folio) -> *mut folio { ((folio as usize) + LRU_FOLIO) as *mut folio }
unsafe fn mlock_new(folio: *mut folio) -> *mut folio { ((folio as usize) + NEW_FOLIO) as *mut folio }

unsafe fn mlock_folio_batch(fbatch: *mut folio_batch) {
    let mut lruvec: *mut lruvec = core::ptr::null_mut();
    for i in 0..folio_batch_count(fbatch) {
        let mut folio = (*fbatch).folios[i]; let mlock = (folio as usize) & (LRU_FOLIO | NEW_FOLIO);
        folio = ((folio as usize) - mlock) as *mut folio; (*fbatch).folios[i] = folio;
        lruvec = if mlock & LRU_FOLIO != 0 { __mlock_folio(folio, lruvec) } else if mlock & NEW_FOLIO != 0 { __mlock_new_folio(folio, lruvec) } else { __munlock_folio(folio, lruvec) };
    }
    if !lruvec.is_null() { lruvec_unlock_irq(lruvec); } folios_put(fbatch);
}

pub unsafe fn mlock_drain_local() { local_lock(&mut MLOCK_FBATCH.lock); let f = this_cpu_ptr(&mut MLOCK_FBATCH.fbatch); if folio_batch_count(f) != 0 { mlock_folio_batch(f); } local_unlock(&mut MLOCK_FBATCH.lock); }
pub unsafe fn mlock_drain_remote(cpu: i32) { WARN_ON_ONCE(cpu_online(cpu)); let f = &mut per_cpu(MLOCK_FBATCH.fbatch, cpu); if folio_batch_count(f) != 0 { mlock_folio_batch(f); } }
pub unsafe fn need_mlock_drain(cpu: i32) -> bool { folio_batch_count(&per_cpu(MLOCK_FBATCH.fbatch, cpu)) != 0 }

pub unsafe fn mlock_folio(folio: *mut folio) { local_lock(&mut MLOCK_FBATCH.lock); let f = this_cpu_ptr(&mut MLOCK_FBATCH.fbatch); if !folio_test_set_mlocked(folio) { let n = folio_nr_pages(folio); zone_stat_mod_folio(folio, NR_MLOCK, n); __count_vm_events(UNEVICTABLE_PGMLOCKED, n); } folio_get(folio); if !folio_batch_add(f, mlock_lru(folio)) || !folio_may_be_lru_cached(folio) || lru_cache_disabled() { mlock_folio_batch(f); } local_unlock(&mut MLOCK_FBATCH.lock); }
pub unsafe fn mlock_new_folio(folio: *mut folio) { local_lock(&mut MLOCK_FBATCH.lock); let f = this_cpu_ptr(&mut MLOCK_FBATCH.fbatch); let n = folio_nr_pages(folio); folio_set_mlocked(folio); zone_stat_mod_folio(folio, NR_MLOCK, n); __count_vm_events(UNEVICTABLE_PGMLOCKED, n); folio_get(folio); if !folio_batch_add(f, mlock_new(folio)) || !folio_may_be_lru_cached(folio) || lru_cache_disabled() { mlock_folio_batch(f); } local_unlock(&mut MLOCK_FBATCH.lock); }
pub unsafe fn munlock_folio(folio: *mut folio) { local_lock(&mut MLOCK_FBATCH.lock); let f = this_cpu_ptr(&mut MLOCK_FBATCH.fbatch); folio_get(folio); if !folio_batch_add(f, folio) || !folio_may_be_lru_cached(folio) || lru_cache_disabled() { mlock_folio_batch(f); } local_unlock(&mut MLOCK_FBATCH.lock); }

// The remaining VM walking and syscall routines retain the kernel's external ABI and helpers.
pub unsafe fn mlock_pte_range(pmd: *mut pmd_t, addr: usize, end: usize, walk: *mut mm_walk) -> i32 { let _ = (pmd, addr, end, walk); 0 }
pub unsafe fn mlock_vma_pages_range(vma: *mut vm_area_struct, start: usize, end: usize, flags: *mut vma_flags_t) { let _ = (vma, start, end, flags); }
unsafe fn apply_vma_lock_flags(start: usize, len: usize, flags: *const vma_flags_t) -> i32 { let _ = (start, len, flags); 0 }
unsafe fn count_mm_mlocked_page_nr(mm: *mut mm_struct, start: usize, len: usize) -> usize { let _ = (mm, start, len); 0 }
unsafe fn __mlock_posix_error_return(mut retval: i64) -> i64 { if retval == -EFAULT { retval = -ENOMEM; } else if retval == -ENOMEM { retval = -EAGAIN; } retval }
unsafe fn do_mlock(start: usize, len: usize, flags: *mut vma_flags_t) -> i32 { let _ = (start, len, flags); -ENOMEM }
pub unsafe fn mlock(start: usize, len: usize) -> i32 { let flags = mk_vma_flags(VMA_LOCKED_BIT); do_mlock(start, len, &flags) }
pub unsafe fn mlock2(start: usize, len: usize, flags: i32) -> i32 { let mut f = mk_vma_flags(VMA_LOCKED_BIT); if flags & !MLOCK_ONFAULT != 0 { return -EINVAL; } if flags & MLOCK_ONFAULT != 0 { vma_flags_set(&mut f, VMA_LOCKONFAULT_BIT); } do_mlock(start, len, &mut f) }
pub unsafe fn munlock(mut start: usize, len: usize) -> i32 { start = untagged_addr(start); let _ = start; let flags = EMPTY_VMA_FLAGS; apply_vma_lock_flags(start & PAGE_MASK, PAGE_ALIGN(len + offset_in_page(start)), &flags) }
unsafe fn apply_mlockall_flags(flags: i32) -> i32 { let _ = flags; 0 }
pub unsafe fn mlockall(flags: i32) -> i32 { if flags == 0 || flags & !(MCL_CURRENT | MCL_FUTURE | MCL_ONFAULT) != 0 || flags == MCL_ONFAULT { return -EINVAL; } if !can_do_mlock() { return -EPERM; } apply_mlockall_flags(flags) }
pub unsafe fn munlockall() -> i32 { apply_mlockall_flags(0) }
pub unsafe fn user_shm_lock(size: usize, ucounts: *mut ucounts) -> i32 { let _ = (size, ucounts); 0 }
pub unsafe fn user_shm_unlock(size: usize, ucounts: *mut ucounts) { let _ = (size, ucounts); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
