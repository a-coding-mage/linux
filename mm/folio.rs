// SPDX-License-Identifier: GPL-2.0-only
/* Translated from linux/mm/folio.c. Kernel-provided types and functions are external dependencies. */

#[repr(C)]
pub struct cpu_fbatches {
    pub lock: local_lock_t,
    pub lru_add: folio_batch,
    pub lru_deactivate_file: folio_batch,
    pub lru_deactivate: folio_batch,
    pub lru_lazyfree: folio_batch,
    #[cfg(feature = "CONFIG_SMP")]
    pub lru_activate: folio_batch,
    pub lock_irq: local_lock_t,
    pub lru_move_tail: folio_batch,
}

static mut cpu_fbatches: cpu_fbatches = cpu_fbatches {
    lock: INIT_LOCAL_LOCK!(lock), lru_add: folio_batch::default(),
    lru_deactivate_file: folio_batch::default(), lru_deactivate: folio_batch::default(),
    lru_lazyfree: folio_batch::default(),
    #[cfg(feature = "CONFIG_SMP")] lru_activate: folio_batch::default(),
    lock_irq: INIT_LOCAL_LOCK!(lock_irq), lru_move_tail: folio_batch::default(),
};

unsafe fn __page_cache_release(folio: *mut folio, lruvecp: *mut *mut lruvec, flagsp: *mut c_ulong) {
    if folio_test_lru(folio) { folio_lruvec_relock_irqsave(folio, lruvecp, flagsp); lruvec_del_folio(*lruvecp, folio); __folio_clear_lru_flags(folio); }
}
unsafe fn page_cache_release(folio: *mut folio) { let mut lruvec = core::ptr::null_mut(); let mut flags = 0; __page_cache_release(folio, &mut lruvec, &mut flags); if !lruvec.is_null() { lruvec_unlock_irqrestore(lruvec, flags); } }

#[no_mangle] pub unsafe extern "C" fn __folio_put(folio: *mut folio) {
    if unlikely(folio_is_zone_device(folio)) { free_zone_device_folio(folio); return; }
    if folio_test_hugetlb(folio) { free_huge_folio(folio); return; }
    page_cache_release(folio); folio_unqueue_deferred_split(folio); mem_cgroup_uncharge(folio); free_frozen_pages(&mut (*folio).page, folio_order(folio));
}

pub type move_fn_t = unsafe fn(*mut lruvec, *mut folio);
unsafe fn lru_add(lruvec: *mut lruvec, folio: *mut folio) {
    let was_unevictable = folio_test_clear_unevictable(folio); let nr_pages = folio_nr_pages(folio);
    VM_BUG_ON_FOLIO!(folio_test_lru(folio), folio);
    if folio_evictable(folio) { if was_unevictable { __count_vm_events!(UNEVICTABLE_PGRESCUED, nr_pages); } }
    else { folio_clear_active(folio); folio_set_unevictable(folio); (*folio).mlock_count = 0; if !was_unevictable { __count_vm_events!(UNEVICTABLE_PGCULLED, nr_pages); } }
    lruvec_add_folio(lruvec, folio); trace_mm_lru_insertion(folio);
}
unsafe fn folio_batch_move_lru(fbatch: *mut folio_batch, move_fn: move_fn_t) {
    let mut lruvec = core::ptr::null_mut(); let mut flags = 0; let is_lru_add = move_fn as usize == lru_add as usize; let mut free_fbatch = folio_batch::default();
    if is_lru_add { folio_batch_init(&mut free_fbatch); }
    for i in 0..folio_batch_count(fbatch) { let folio = (*fbatch).folios[i]; if !is_lru_add && !folio_test_clear_lru(folio) { continue; }
        if is_lru_add && folio_ref_freeze(folio, 1) { __folio_clear_active(folio); __folio_clear_unevictable(folio); folio_unqueue_deferred_split(folio); (*fbatch).folios[i] = core::ptr::null_mut(); folio_batch_add(&mut free_fbatch, folio); continue; }
        folio_lruvec_relock_irqsave(folio, &mut lruvec, &mut flags); move_fn(lruvec, folio); folio_set_lru(folio);
    }
    if !lruvec.is_null() { lruvec_unlock_irqrestore(lruvec, flags); } if is_lru_add { mem_cgroup_uncharge_folios(&mut free_fbatch); free_unref_folios(&mut free_fbatch); } folios_put(fbatch);
}
unsafe fn __folio_batch_add_and_move(fbatch: *mut folio_batch, folio: *mut folio, op: move_fn_t, disable_irq: bool) {
    let mut flags = 0; folio_get(folio); if disable_irq { local_lock_irqsave!((*core::ptr::addr_of_mut!(cpu_fbatches)).lock_irq, flags); } else { local_lock!((*core::ptr::addr_of_mut!(cpu_fbatches)).lock); }
    if folio_batch_add(this_cpu_ptr(fbatch), folio) == 0 || !folio_may_be_lru_cached(folio) || lru_cache_disabled() { folio_batch_move_lru(this_cpu_ptr(fbatch), op); }
    if disable_irq { local_unlock_irqrestore!((*core::ptr::addr_of_mut!(cpu_fbatches)).lock_irq, flags); } else { local_unlock!((*core::ptr::addr_of_mut!(cpu_fbatches)).lock); }
}

unsafe fn lru_move_tail(lruvec: *mut lruvec, folio: *mut folio) { if folio_test_unevictable(folio) { return; } lruvec_del_folio(lruvec, folio); folio_clear_active(folio); lruvec_add_folio_tail(lruvec, folio); __count_vm_events!(PGROTATED, folio_nr_pages(folio)); }
#[no_mangle] pub unsafe extern "C" fn folio_rotate_reclaimable(folio: *mut folio) { if folio_test_locked(folio)||folio_test_dirty(folio)||folio_test_unevictable(folio)||!folio_test_lru(folio){return;} __folio_batch_add_and_move(core::ptr::addr_of_mut!((*core::ptr::addr_of_mut!(cpu_fbatches)).lru_move_tail),folio,lru_move_tail,true); }

unsafe fn lru_activate(lruvec: *mut lruvec, folio: *mut folio) { let n=folio_nr_pages(folio); if folio_test_active(folio)||folio_test_unevictable(folio){return;} lruvec_del_folio(lruvec,folio); folio_set_active(folio); lruvec_add_folio(lruvec,folio); trace_mm_lru_activate(folio); __count_vm_events!(PGACTIVATE,n); count_memcg_events(lruvec_memcg(lruvec),PGACTIVATE,n); }

#[no_mangle] pub unsafe extern "C" fn folio_mark_accessed(folio:*mut folio){ if folio_test_dropbehind(folio){return;} if lru_gen_enabled(){lru_gen_inc_refs(folio);return;} if !folio_test_referenced(folio){folio_set_referenced(folio);}else if folio_test_unevictable(folio){}else if !folio_test_active(folio){if folio_test_lru(folio){folio_activate(folio);}else{__lru_cache_activate_folio(folio);}folio_clear_referenced(folio);workingset_activation(folio);}if folio_test_idle(folio){folio_clear_idle(folio);} }

unsafe fn __lru_cache_activate_folio(folio:*mut folio){local_lock!((*core::ptr::addr_of_mut!(cpu_fbatches)).lock);let b=this_cpu_ptr(core::ptr::addr_of_mut!((*core::ptr::addr_of_mut!(cpu_fbatches)).lru_add));for i in (0..folio_batch_count(b)).rev(){if (*b).folios[i]==folio{folio_set_active(folio);break;}}local_unlock!((*core::ptr::addr_of_mut!(cpu_fbatches)).lock);}

#[no_mangle] pub unsafe extern "C" fn folio_add_lru(folio:*mut folio){VM_BUG_ON_FOLIO!(folio_test_active(folio)&&folio_test_unevictable(folio),folio);VM_BUG_ON_FOLIO!(folio_test_lru(folio),folio);if lru_gen_enabled()&&!folio_test_unevictable(folio)&&lru_gen_in_fault()&&((*current).flags&PF_MEMALLOC)==0{if folio_test_workingset(folio){folio_set_active(folio);}else if !folio_test_referenced(folio){folio_mark_accessed(folio);}}__folio_batch_add_and_move(core::ptr::addr_of_mut!((*core::ptr::addr_of_mut!(cpu_fbatches)).lru_add),folio,lru_add,false);}

#[no_mangle] pub unsafe extern "C" fn folio_add_lru_vma(folio:*mut folio,vma:*mut vm_area_struct){VM_BUG_ON_FOLIO!(folio_test_lru(folio),folio);if unlikely(((*vma).vm_flags&(VM_LOCKED|VM_SPECIAL))==VM_LOCKED){mlock_new_folio(folio);}else{folio_add_lru(folio);}}

#[no_mangle] pub unsafe extern "C" fn lru_add_drain(){local_lock!((*core::ptr::addr_of_mut!(cpu_fbatches)).lock);lru_add_drain_cpu(smp_processor_id());local_unlock!((*core::ptr::addr_of_mut!(cpu_fbatches)).lock);mlock_drain_local();}

#[no_mangle] pub unsafe extern "C" fn folio_deactivate(folio:*mut folio){if folio_test_unevictable(folio)||!folio_test_lru(folio){return;}if lru_gen_enabled(){if lru_gen_clear_refs(folio){return;}}else if !folio_test_active(folio){return;}__folio_batch_add_and_move(core::ptr::addr_of_mut!((*core::ptr::addr_of_mut!(cpu_fbatches)).lru_deactivate),folio,lru_deactivate,false);}
#[no_mangle] pub unsafe extern "C" fn folio_mark_lazyfree(folio:*mut folio){if !folio_test_anon(folio)||!folio_test_swapbacked(folio)||!folio_test_lru(folio)||folio_test_swapcache(folio)||folio_test_unevictable(folio){return;}__folio_batch_add_and_move(core::ptr::addr_of_mut!((*core::ptr::addr_of_mut!(cpu_fbatches)).lru_lazyfree),folio,lru_lazyfree,false);}

#[no_mangle] pub unsafe extern "C" fn lru_cache_disable(){atomic_inc(&mut lru_disable_count);synchronize_rcu_expedited();#[cfg(feature="CONFIG_SMP")]__lru_add_drain_all(true);#[cfg(not(feature="CONFIG_SMP"))]lru_add_and_bh_lrus_drain();}
#[no_mangle] pub unsafe extern "C" fn lru_cache_drain_for_folio(folio:*const folio,extra_refs:u32,drained:*mut enum_lru_cache_drained){if !folio_may_be_lru_cached(folio){return;}if drained.is_null()||*drained==LRU_CACHE_NOT_DRAINED{if folio_ref_count(folio)==folio_expected_ref_count(folio)+extra_refs{return;}lru_add_drain();if !drained.is_null(){*drained=LRU_CACHE_DRAINED;}}if drained.is_null()||*drained==LRU_CACHE_DRAINED{if folio_ref_count(folio)==folio_expected_ref_count(folio)+extra_refs{return;}lru_add_drain_all();if !drained.is_null(){*drained=LRU_CACHE_DRAINED_ALL;}}}

pub static mut lru_disable_count: atomic_t = ATOMIC_INIT!(0);

unsafe fn lru_deactivate_file(lruvec:*mut lruvec,folio:*mut folio){let active=folio_test_active(folio)||lru_gen_enabled();let n=folio_nr_pages(folio);if folio_test_unevictable(folio)||folio_mapped(folio){return;}lruvec_del_folio(lruvec,folio);folio_clear_active(folio);folio_clear_referenced(folio);if folio_test_writeback(folio)||folio_test_dirty(folio){lruvec_add_folio(lruvec,folio);folio_set_reclaim(folio);}else{lruvec_add_folio_tail(lruvec,folio);__count_vm_events!(PGROTATED,n);}if active{__count_vm_events!(PGDEACTIVATE,n);count_memcg_events(lruvec_memcg(lruvec),PGDEACTIVATE,n);}}
unsafe fn lru_deactivate(lruvec:*mut lruvec,folio:*mut folio){let n=folio_nr_pages(folio);if folio_test_unevictable(folio)||!(folio_test_active(folio)||lru_gen_enabled()){return;}lruvec_del_folio(lruvec,folio);folio_clear_active(folio);folio_clear_referenced(folio);lruvec_add_folio(lruvec,folio);__count_vm_events!(PGDEACTIVATE,n);count_memcg_events(lruvec_memcg(lruvec),PGDEACTIVATE,n);}
unsafe fn lru_lazyfree(lruvec:*mut lruvec,folio:*mut folio){let n=folio_nr_pages(folio);if !folio_test_anon(folio)||!folio_test_swapbacked(folio)||folio_test_swapcache(folio)||folio_test_unevictable(folio){return;}lruvec_del_folio(lruvec,folio);folio_clear_active(folio);if lru_gen_enabled(){lru_gen_clear_refs(folio);}else{folio_clear_referenced(folio);}folio_clear_swapbacked(folio);lruvec_add_folio(lruvec,folio);__count_vm_events!(PGLAZYFREE,n);count_memcg_events(lruvec_memcg(lruvec),PGLAZYFREE,n);}
#[no_mangle] pub unsafe extern "C" fn deactivate_file_folio(folio:*mut folio){if folio_test_unevictable(folio)||!folio_test_lru(folio){return;}if lru_gen_enabled()&&lru_gen_clear_refs(folio){return;}__folio_batch_add_and_move(core::ptr::addr_of_mut!((*core::ptr::addr_of_mut!(cpu_fbatches)).lru_deactivate_file),folio,lru_deactivate_file,false);}
#[no_mangle] pub unsafe extern "C" fn lru_add_drain_cpu(cpu:i32){let b=&mut per_cpu!(cpu_fbatches,cpu).lru_add;if folio_batch_count(b)!=0{folio_batch_move_lru(b,lru_add);}let b=&mut per_cpu!(cpu_fbatches,cpu).lru_move_tail;if data_race!(folio_batch_count(b))!=0{let mut f=0;local_lock_irqsave!((*core::ptr::addr_of_mut!(cpu_fbatches)).lock_irq,f);folio_batch_move_lru(b,lru_move_tail);local_unlock_irqrestore!((*core::ptr::addr_of_mut!(cpu_fbatches)).lock_irq,f);} }
#[no_mangle] pub unsafe extern "C" fn lru_add_drain_all(){lru_add_drain();}
#[no_mangle] pub unsafe extern "C" fn __folio_batch_release(b:*mut folio_batch){if !(*b).percpu_pvec_drained{lru_add_drain();(*b).percpu_pvec_drained=true;}folios_put(b);}
#[no_mangle] pub unsafe extern "C" fn folio_batch_remove_exceptionals(b:*mut folio_batch){let mut j=0;for i in 0..folio_batch_count(b){let f=(*b).folios[i];if !xa_is_value(f){(*b).folios[j]=f;j+=1;}}(*b).nr=j;}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
