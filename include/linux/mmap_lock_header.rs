/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from linux/mmap_lock.h. Included C dependencies are external. */

extern "C" {
    pub fn rcuwait_wake_up(w: *mut rcuwait) -> c_int;
    pub fn __vma_start_write(vma: *mut vm_area_struct, state: c_int) -> c_int;
    pub fn __vma_exclude_readers_for_detach(vma: *mut vm_area_struct);
    pub fn lock_vma_under_rcu(mm: *mut mm_struct, address: c_ulong) -> *mut vm_area_struct;
    pub fn lock_next_vma(mm: *mut mm_struct, iter: *mut vma_iterator,
                         address: c_ulong) -> *mut vm_area_struct;
    pub fn __mmap_lock_do_trace_start_locking(mm: *mut mm_struct, write: bool);
    pub fn __mmap_lock_do_trace_acquire_returned(mm: *mut mm_struct, write: bool, success: bool);
    pub fn __mmap_lock_do_trace_released(mm: *mut mm_struct, write: bool);
}

/* DECLARE_TRACEPOINT(mmap_lock_start_locking); */
/* DECLARE_TRACEPOINT(mmap_lock_acquire_returned); */
/* DECLARE_TRACEPOINT(mmap_lock_released); */
#[inline] pub unsafe fn __vma_lockdep_acquire_read(vma: *mut vm_area_struct) { lock_acquire_shared(__vma_lockdep_map(vma), 0, 1, core::ptr::null_mut(), _RET_IP_); }
#[inline] pub unsafe fn __vma_lockdep_release_read(vma: *mut vm_area_struct) { lock_release(__vma_lockdep_map(vma), _RET_IP_); }
#[inline] pub unsafe fn __vma_lockdep_acquire_exclusive(vma: *mut vm_area_struct) { lock_acquire_exclusive(__vma_lockdep_map(vma), 0, 0, core::ptr::null_mut(), _RET_IP_); }
#[inline] pub unsafe fn __vma_lockdep_release_exclusive(vma: *mut vm_area_struct) { lock_release(__vma_lockdep_map(vma), _RET_IP_); }
#[inline] pub unsafe fn __vma_lockdep_stat_mark_acquired(vma: *mut vm_area_struct) { lock_acquired(__vma_lockdep_map(vma), _RET_IP_); }
#[inline] unsafe fn __vma_lockdep_map(vma: *mut vm_area_struct) -> *mut lockdep_map {
    #[cfg(CONFIG_LOCKDEP)] { &mut (*vma).vmlock_dep_map }
    #[cfg(not(CONFIG_LOCKDEP))] { core::ptr::null_mut() }
}

/* MMAP_LOCK_INITIALIZER(name): .mmap_lock = __RWSEM_INITIALIZER((name).mmap_lock) */

#[inline] pub unsafe fn __mmap_lock_trace_start_locking(mm: *mut mm_struct, write: bool) {
    #[cfg(CONFIG_TRACING)] { if tracepoint_enabled(mmap_lock_start_locking) { __mmap_lock_do_trace_start_locking(mm, write); } }
}
#[inline] pub unsafe fn __mmap_lock_trace_acquire_returned(mm: *mut mm_struct, write: bool, success: bool) {
    #[cfg(CONFIG_TRACING)] { if tracepoint_enabled(mmap_lock_acquire_returned) { __mmap_lock_do_trace_acquire_returned(mm, write, success); } }
}
#[inline] pub unsafe fn __mmap_lock_trace_released(mm: *mut mm_struct, write: bool) {
    #[cfg(CONFIG_TRACING)] { if tracepoint_enabled(mmap_lock_released) { __mmap_lock_do_trace_released(mm, write); } }
}

#[inline] pub unsafe fn mmap_assert_locked(mm: *const mm_struct) { rwsem_assert_held(&(*mm).mmap_lock); }
#[inline] pub unsafe fn mmap_assert_write_locked(mm: *const mm_struct) { rwsem_assert_held_write(&(*mm).mmap_lock); }

#[inline] pub unsafe fn mm_lock_seqcount_init(mm: *mut mm_struct) { seqcount_init(&mut (*mm).mm_lock_seq); }
#[inline] pub unsafe fn mm_lock_seqcount_begin(mm: *mut mm_struct) { do_raw_write_seqcount_begin(&mut (*mm).mm_lock_seq); }
#[inline] pub unsafe fn mm_lock_seqcount_end(mm: *mut mm_struct) { ASSERT_EXCLUSIVE_WRITER((*mm).mm_lock_seq); do_raw_write_seqcount_end(&mut (*mm).mm_lock_seq); }

#[inline] pub unsafe fn mmap_lock_speculate_try_begin(mm: *mut mm_struct, seq: *mut c_uint) -> bool { raw_seqcount_try_begin(&(*mm).mm_lock_seq, *seq) }
#[inline] pub unsafe fn mmap_lock_speculate_retry(mm: *mut mm_struct, seq: c_uint) -> bool { read_seqcount_retry(&(*mm).mm_lock_seq, seq) }

#[inline] pub unsafe fn vma_lock_init(vma: *mut vm_area_struct, reset_refcnt: bool) {
    #[cfg(CONFIG_DEBUG_LOCK_ALLOC)] { static mut lockdep_key: lock_class_key = lock_class_key {}; lockdep_init_map(__vma_lockdep_map(vma), b"vm_lock\0".as_ptr() as *const _, &mut lockdep_key, 0); }
    if reset_refcnt { refcount_set(&mut (*vma).vm_refcnt, 0); }
    (*vma).vm_lock_seq = UINT_MAX;
}
#[inline] pub unsafe fn __vma_are_readers_excluded(refcnt: c_int) -> bool { (refcnt & VM_REFCNT_EXCLUDE_READERS_FLAG) != 0 && refcnt <= VM_REFCNT_EXCLUDE_READERS_FLAG + 1 }
#[inline] pub unsafe fn __vma_refcount_put_return(vma: *mut vm_area_struct) -> c_uint { let mut oldcnt = 0; if __refcount_dec_and_test(&mut (*vma).vm_refcnt, &mut oldcnt) { 0 } else { (oldcnt - 1) as c_uint } }
#[inline] pub unsafe fn vma_refcount_put(vma: *mut vm_area_struct) { let mm = (*vma).vm_mm; __vma_lockdep_release_read(vma); let newcnt = __vma_refcount_put_return(vma); if newcnt != 0 && __vma_are_readers_excluded(newcnt as c_int) { rcuwait_wake_up(&mut (*mm).vma_writer_wait); } }

#[inline] pub unsafe fn vma_start_read_locked_nested(vma: *mut vm_area_struct, _subclass: c_int) -> bool { mmap_assert_locked((*vma).vm_mm); let mut oldcnt=0; if !__refcount_inc_not_zero_limited_acquire(&mut (*vma).vm_refcnt, &mut oldcnt, VM_REFCNT_LIMIT) { return false; } __vma_lockdep_acquire_read(vma); true }
#[inline] pub unsafe fn vma_start_read_locked(vma: *mut vm_area_struct) -> bool { vma_start_read_locked_nested(vma, 0) }
#[inline] pub unsafe fn vma_end_read(vma: *mut vm_area_struct) { vma_refcount_put(vma); }
#[inline] pub unsafe fn __vma_raw_mm_seqnum(vma: *mut vm_area_struct) -> c_uint { mmap_assert_write_locked((*vma).vm_mm); (*(*vma).vm_mm).mm_lock_seq.sequence }
#[inline] pub unsafe fn __is_vma_write_locked(vma: *mut vm_area_struct) -> bool { (*vma).vm_lock_seq == __vma_raw_mm_seqnum(vma) }
#[inline] pub unsafe fn vma_start_write(vma: *mut vm_area_struct) { if !__is_vma_write_locked(vma) { __vma_start_write(vma, TASK_UNINTERRUPTIBLE); } }
#[inline] pub unsafe fn vma_start_write_killable(vma: *mut vm_area_struct) -> c_int { if __is_vma_write_locked(vma) { 0 } else { __vma_start_write(vma, TASK_KILLABLE) } }
#[inline] pub unsafe fn vma_assert_write_locked(vma: *mut vm_area_struct) { VM_WARN_ON_ONCE_VMA(!__is_vma_write_locked(vma), vma); }
#[inline] pub unsafe fn vma_is_attached(vma: *mut vm_area_struct) -> bool { refcount_read(&(*vma).vm_refcnt) != 0 }
#[inline] pub unsafe fn vma_assert_attached(vma: *mut vm_area_struct) { WARN_ON_ONCE(!vma_is_attached(vma)); }
#[inline] pub unsafe fn vma_assert_detached(vma: *mut vm_area_struct) { WARN_ON_ONCE(vma_is_attached(vma)); }
#[inline] pub unsafe fn vma_mark_attached(vma: *mut vm_area_struct) { vma_assert_write_locked(vma); vma_assert_detached(vma); refcount_set_release(&mut (*vma).vm_refcnt, 1); }
#[inline] pub unsafe fn vma_mark_detached(vma: *mut vm_area_struct) { vma_assert_write_locked(vma); vma_assert_attached(vma); if __vma_refcount_put_return(vma) == 0 { return; } __vma_exclude_readers_for_detach(vma); }

#[inline] pub unsafe fn vma_assert_locked(vma: *mut vm_area_struct) { mmap_assert_locked((*vma).vm_mm); }
#[inline] pub unsafe fn vma_assert_stabilised(vma: *mut vm_area_struct) { mmap_assert_locked((*vma).vm_mm); }
#[inline] pub unsafe fn vma_assert_can_modify(vma: *mut vm_area_struct) { if vma_is_attached(vma) { vma_assert_write_locked(vma); } }

#[inline] pub unsafe fn mmap_write_lock(mm: *mut mm_struct) { __mmap_lock_trace_start_locking(mm,true); down_write(&mut (*mm).mmap_lock); mm_lock_seqcount_begin(mm); __mmap_lock_trace_acquire_returned(mm,true,true); }
#[inline] pub unsafe fn mmap_write_lock_nested(mm: *mut mm_struct, subclass: c_int) { __mmap_lock_trace_start_locking(mm,true); down_write_nested(&mut (*mm).mmap_lock,subclass); mm_lock_seqcount_begin(mm); __mmap_lock_trace_acquire_returned(mm,true,true); }
#[inline] pub unsafe fn mmap_write_lock_killable(mm: *mut mm_struct) -> c_int { __mmap_lock_trace_start_locking(mm,true); let ret=down_write_killable(&mut (*mm).mmap_lock); if ret==0 { mm_lock_seqcount_begin(mm); } __mmap_lock_trace_acquire_returned(mm,true,ret==0); ret }
#[inline] pub unsafe fn vma_end_write_all(mm: *mut mm_struct) { mmap_assert_write_locked(mm); mm_lock_seqcount_end(mm); }
#[inline] pub unsafe fn mmap_write_unlock(mm: *mut mm_struct) { __mmap_lock_trace_released(mm,true); vma_end_write_all(mm); up_write(&mut (*mm).mmap_lock); }
#[inline] pub unsafe fn mmap_write_downgrade(mm: *mut mm_struct) { __mmap_lock_trace_acquire_returned(mm,false,true); vma_end_write_all(mm); downgrade_write(&mut (*mm).mmap_lock); }
#[inline] pub unsafe fn mmap_read_lock(mm: *mut mm_struct) { __mmap_lock_trace_start_locking(mm,false); down_read(&mut (*mm).mmap_lock); __mmap_lock_trace_acquire_returned(mm,false,true); }
#[inline] pub unsafe fn mmap_read_lock_killable(mm: *mut mm_struct) -> c_int { __mmap_lock_trace_start_locking(mm,false); let ret=down_read_killable(&mut (*mm).mmap_lock); __mmap_lock_trace_acquire_returned(mm,false,ret==0); ret }
#[inline] pub unsafe fn mmap_read_trylock(mm: *mut mm_struct) -> bool { __mmap_lock_trace_start_locking(mm,false); let ret=down_read_trylock(&mut (*mm).mmap_lock)!=0; __mmap_lock_trace_acquire_returned(mm,false,ret); ret }
#[inline] pub unsafe fn mmap_read_unlock(mm: *mut mm_struct) { __mmap_lock_trace_released(mm,false); up_read(&mut (*mm).mmap_lock); }
#[inline] pub unsafe fn mmap_read_unlock_non_owner(mm: *mut mm_struct) { __mmap_lock_trace_released(mm,false); up_read_non_owner(&mut (*mm).mmap_lock); }
#[inline] pub unsafe fn mmap_lock_is_contended(mm: *mut mm_struct) -> c_int { rwsem_is_contended(&(*mm).mmap_lock) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
