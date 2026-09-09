// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the kernel headers are intentionally external.

#[cfg(feature = "CONFIG_TRACING")]
pub unsafe fn __mmap_lock_do_trace_start_locking(mm: *mut mm_struct, write: bool) {
    trace_mmap_lock_start_locking(mm, write);
}

#[cfg(feature = "CONFIG_TRACING")]
pub unsafe fn __mmap_lock_do_trace_acquire_returned(
    mm: *mut mm_struct,
    write: bool,
    success: bool,
) {
    trace_mmap_lock_acquire_returned(mm, write, success);
}

#[cfg(feature = "CONFIG_TRACING")]
pub unsafe fn __mmap_lock_do_trace_released(mm: *mut mm_struct, write: bool) {
    trace_mmap_lock_released(mm, write);
}

#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_PER_VMA_LOCK"))]
#[repr(C)]
struct vma_exclude_readers_state {
    vma: *mut vm_area_struct,
    state: i32,
    detaching: bool,
    detached: bool,
    exclusive: bool,
}

#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_PER_VMA_LOCK"))]
unsafe fn __vma_end_exclude_readers(ves: *mut vma_exclude_readers_state) {
    let vma = (*ves).vma;
    vm_warn_on_once((*ves).detached);
    (*ves).detached = refcount_sub_and_test(VM_REFCNT_EXCLUDE_READERS_FLAG, &mut (*vma).vm_refcnt);
    __vma_lockdep_release_exclusive(vma);
}

#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_PER_VMA_LOCK"))]
unsafe fn get_target_refcnt(ves: *mut vma_exclude_readers_state) -> u32 {
    let tgt: u32 = if (*ves).detaching { 0 } else { 1 };
    tgt | VM_REFCNT_EXCLUDE_READERS_FLAG
}

#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_PER_VMA_LOCK"))]
unsafe fn __vma_start_exclude_readers(ves: *mut vma_exclude_readers_state) -> i32 {
    let vma = (*ves).vma;
    let tgt_refcnt = get_target_refcnt(ves);
    let mut err = 0;
    mmap_assert_write_locked((*vma).vm_mm);
    if !refcount_add_not_zero(VM_REFCNT_EXCLUDE_READERS_FLAG, &mut (*vma).vm_refcnt) {
        (*ves).detached = true;
        return 0;
    }
    __vma_lockdep_acquire_exclusive(vma);
    err = rcuwait_wait_event(
        &mut (*(*vma).vm_mm).vma_writer_wait,
        refcount_read(&(*vma).vm_refcnt) == tgt_refcnt,
        (*ves).state,
    );
    if err != 0 {
        __vma_end_exclude_readers(ves);
        return err;
    }
    __vma_lockdep_stat_mark_acquired(vma);
    (*ves).exclusive = true;
    0
}

#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_PER_VMA_LOCK"))]
pub unsafe fn __vma_start_write(vma: *mut vm_area_struct, state: i32) -> i32 {
    let mm_lock_seq = __vma_raw_mm_seqnum(vma);
    let mut ves = vma_exclude_readers_state {
        vma,
        state,
        detaching: false,
        detached: false,
        exclusive: false,
    };
    let err = __vma_start_exclude_readers(&mut ves);
    if err != 0 {
        warn_on_once(ves.detached);
        return err;
    }
    write_once(&mut (*vma).vm_lock_seq, mm_lock_seq);
    if ves.exclusive {
        __vma_end_exclude_readers(&mut ves);
        warn_on_once(ves.detached);
    }
    0
}

#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_PER_VMA_LOCK"))]
pub unsafe fn __vma_exclude_readers_for_detach(vma: *mut vm_area_struct) {
    let mut ves = vma_exclude_readers_state {
        vma,
        state: TASK_UNINTERRUPTIBLE,
        detaching: true,
        detached: false,
        exclusive: false,
    };
    let err = __vma_start_exclude_readers(&mut ves);
    if err == 0 && ves.exclusive {
        __vma_end_exclude_readers(&mut ves);
    }
    warn_on_once(!ves.detached);
}

#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_PER_VMA_LOCK"))]
unsafe fn vma_start_read(mm: *mut mm_struct, mut vma: *mut vm_area_struct) -> *mut vm_area_struct {
    let mut other_mm: *mut mm_struct;
    let mut oldcnt = 0;
    rcu_lockdep_warn(!rcu_read_lock_held(), "no rcu lock held");
    if read_once((*vma).vm_lock_seq) == read_once((*mm).mm_lock_seq.sequence) {
        vma = core::ptr::null_mut();
        goto_err: {
            rcu_read_unlock();
            return vma;
        }
    }
    if !__refcount_inc_not_zero_limited_acquire(&mut (*vma).vm_refcnt, &mut oldcnt, VM_REFCNT_LIMIT) {
        vma = if oldcnt != 0 { core::ptr::null_mut() } else { err_ptr(-EAGAIN) };
        rcu_read_unlock();
        return vma;
    }
    __vma_lockdep_acquire_read(vma);
    if (*vma).vm_mm != mm {
        other_mm = (*vma).vm_mm;
        rcu_read_unlock();
        mmgrab(other_mm);
        vma_refcount_put(vma);
        mmdrop(other_mm);
        return core::ptr::null_mut();
    }
    if (*vma).vm_lock_seq == raw_read_seqcount(&(*mm).mm_lock_seq) {
        vma_refcount_put(vma);
        vma = core::ptr::null_mut();
        rcu_read_unlock();
        return vma;
    }
    vma
}

#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_PER_VMA_LOCK"))]
pub unsafe fn lock_vma_under_rcu(mm: *mut mm_struct, address: u64) -> *mut vm_area_struct {
    let mut mas = ma_state_new(&mut (*mm).mm_mt, address, address);
    loop {
        rcu_read_lock();
        let mut vma = mas_walk(&mut mas);
        if vma.is_null() { rcu_read_unlock(); count_vm_vma_lock_event(VMA_LOCK_ABORT); return core::ptr::null_mut(); }
        vma = vma_start_read(mm, vma);
        if is_err_or_null(vma) {
            if ptr_err(vma) == -EAGAIN { count_vm_vma_lock_event(VMA_LOCK_MISS); mas_set(&mut mas, address); continue; }
            count_vm_vma_lock_event(VMA_LOCK_ABORT); return core::ptr::null_mut();
        }
        rcu_read_unlock();
        if address < (*vma).vm_start || address >= (*vma).vm_end { vma_end_read(vma); count_vm_vma_lock_event(VMA_LOCK_ABORT); return core::ptr::null_mut(); }
        return vma;
    }
}

#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_PER_VMA_LOCK"))]
unsafe fn lock_next_vma_under_mmap_lock(mm: *mut mm_struct, vmi: *mut vma_iterator, from_addr: u64) -> *mut vm_area_struct {
    let ret = mmap_read_lock_killable(mm);
    if ret != 0 { return err_ptr(ret); }
    vma_iter_set(vmi, from_addr);
    let mut vma = vma_next(vmi);
    if !vma.is_null() && !vma_start_read_locked(vma) { vma = err_ptr(-EAGAIN); }
    mmap_read_unlock(mm);
    vma
}

#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_PER_VMA_LOCK"))]
pub unsafe fn lock_next_vma(mm: *mut mm_struct, vmi: *mut vma_iterator, from_addr: u64) -> *mut vm_area_struct {
    rcu_lockdep_warn(!rcu_read_lock_held(), "no rcu read lock held");
    let mut mm_wr_seq = 0;
    let mut mmap_unlocked = mmap_lock_speculate_try_begin(mm, &mut mm_wr_seq);
    loop {
        let mut vma = vma_next(vmi);
        if vma.is_null() { return core::ptr::null_mut(); }
        vma = vma_start_read(mm, vma);
        if is_err_or_null(vma) {
            if ptr_err(vma) == -EAGAIN { rcu_read_lock(); vma_iter_set(vmi, from_addr); continue; }
            rcu_read_unlock();
            vma = lock_next_vma_under_mmap_lock(mm, vmi, from_addr);
            rcu_read_lock();
            vma_iter_set(vmi, if is_err_or_null(vma) { from_addr } else { (*vma).vm_end });
            return vma;
        }
        if from_addr >= (*vma).vm_end { rcu_read_unlock(); vma_end_read(vma); }
        else {
            if from_addr < (*vma).vm_start && (!mmap_unlocked || mmap_lock_speculate_retry(mm, mm_wr_seq)) {
                vma_iter_set(vmi, from_addr);
                if vma != vma_next(vmi) { rcu_read_unlock(); vma_end_read(vma); }
                else { return vma; }
            } else { return vma; }
        }
        vma = lock_next_vma_under_mmap_lock(mm, vmi, from_addr);
        rcu_read_lock();
        vma_iter_set(vmi, if is_err_or_null(vma) { from_addr } else { (*vma).vm_end });
        return vma;
    }
}

#[cfg(feature = "CONFIG_LOCK_MM_AND_FIND_VMA")]
unsafe fn get_mmap_lock_carefully(mm: *mut mm_struct, regs: *mut pt_regs) -> bool {
    if mmap_read_trylock(mm) { return true; }
    if !regs.is_null() && !user_mode(regs) {
        let ip = exception_ip(regs);
        if !search_exception_tables(ip) { return false; }
    }
    mmap_read_lock_killable(mm) == 0
}

#[cfg(feature = "CONFIG_LOCK_MM_AND_FIND_VMA")]
unsafe fn mmap_upgrade_trylock(_mm: *mut mm_struct) -> bool { false }

#[cfg(feature = "CONFIG_LOCK_MM_AND_FIND_VMA")]
unsafe fn upgrade_mmap_lock_carefully(mm: *mut mm_struct, regs: *mut pt_regs) -> bool {
    mmap_read_unlock(mm);
    if !regs.is_null() && !user_mode(regs) && !search_exception_tables(exception_ip(regs)) { return false; }
    mmap_write_lock_killable(mm) == 0
}

#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_LOCK_MM_AND_FIND_VMA"))]
pub unsafe fn lock_mm_and_find_vma(mm: *mut mm_struct, addr: u64, regs: *mut pt_regs) -> *mut vm_area_struct {
    if !get_mmap_lock_carefully(mm, regs) { return core::ptr::null_mut(); }
    let mut vma = find_vma(mm, addr);
    if !vma.is_null() && (*vma).vm_start <= addr { return mmap_write_downgrade(mm, vma); }
    if vma.is_null() || (*vma).vm_flags & VM_GROWSDOWN == 0 { mmap_read_unlock(mm); return core::ptr::null_mut(); }
    if !mmap_upgrade_trylock(mm) {
        if !upgrade_mmap_lock_carefully(mm, regs) { return core::ptr::null_mut(); }
        vma = find_vma(mm, addr);
        if vma.is_null() || ((*vma).vm_start > addr && (*vma).vm_flags & VM_GROWSDOWN == 0) { mmap_write_unlock(mm); return core::ptr::null_mut(); }
    }
    if expand_stack_locked(vma, addr) != 0 { mmap_write_unlock(mm); return core::ptr::null_mut(); }
    mmap_write_downgrade(mm, vma)
}

#[cfg(not(feature = "CONFIG_MMU"))]
pub unsafe fn lock_mm_and_find_vma(mm: *mut mm_struct, addr: u64, _regs: *mut pt_regs) -> *mut vm_area_struct {
    mmap_read_lock(mm);
    let vma = vma_lookup(mm, addr);
    if vma.is_null() { mmap_read_unlock(mm); }
    vma
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
