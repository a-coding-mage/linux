// SPDX-License-Identifier: GPL-2.0-only
// Original unit lines 2195-3687: VM registration, context lifetime and queues.
unsafe fn vma_can_userfault(v: *mut vm_area_struct, mut f: vm_flags_t, wp_async: bool) -> bool {
    let ops = vma_uffd_ops(v);
    if vflags(v) & (RUST_UFFD_VM_DROPPABLE | RUST_UFFD_VM_SHADOW_STACK) != 0 {
        return false;
    }
    if !is_vm_hugetlb_page(v) && vflags(v) & RUST_UFFD_VM_SPECIAL != 0 {
        return false;
    }
    f &= RUST_UFFD___VM_UFFD_FLAGS;
    if wp_async && f == RUST_UFFD_VM_UFFD_WP {
        return true;
    }
    if ops.is_null() {
        return false;
    }
    if !uffd_supports_wp_marker() && f & RUST_UFFD_VM_UFFD_WP != 0 && !vma_is_anonymous(v) {
        return false;
    }
    (*ops).can_userfault.unwrap()(v, f)
}
unsafe fn userfaultfd_set_vm_flags(v: *mut vm_area_struct, f: vm_flags_t) {
    let changed = (vflags(v) ^ f) & RUST_UFFD_VM_UFFD_WP != 0;
    vm_flags_reset(v, f);
    if vflags(v) & RUST_UFFD_VM_SHARED != 0 && changed {
        vma_set_page_prot(v);
    }
}
unsafe fn userfaultfd_set_ctx(v: *mut vm_area_struct, c: *mut userfaultfd_ctx, f: vm_flags_t) {
    vma_start_write(v);
    (*v).vm_userfaultfd_ctx.ctx = c;
    userfaultfd_set_vm_flags(v, (vflags(v) & !RUST_UFFD___VM_UFFD_FLAGS) | f);
}
unsafe fn userfaultfd_reset_ctx(v: *mut vm_area_struct) {
    userfaultfd_set_ctx(v, null_mut(), 0);
}
unsafe fn userfaultfd_clear_vma(
    i: *mut vma_iterator,
    prev: *mut vm_area_struct,
    v: *mut vm_area_struct,
    start: c_ulong,
    end: c_ulong,
) -> *mut vm_area_struct {
    let mut f = vma_native_flags(v);
    vma_flags_clear_mask(&mut f, all_uffd_vma_flags());
    let give_up = start == vstart(v) && end == vend(v);
    if userfaultfd_protected(v) {
        let mut flags = 0;
        let mut tlb = zeroed();
        if userfaultfd_wp(v) {
            flags |= RUST_UFFD_MM_CP_UFFD_WP_RESOLVE;
        }
        if userfaultfd_rwp(v) {
            flags |= RUST_UFFD_MM_CP_UFFD_RWP_RESOLVE;
        }
        if vma_wants_manual_pte_write_upgrade(v) {
            flags |= RUST_UFFD_MM_CP_TRY_CHANGE_WRITABLE;
        }
        tlb_gather_mmu(&mut tlb, (*v).vm_mm);
        change_protection(&mut tlb, v, start, end, flags);
        tlb_finish_mmu(&mut tlb);
    }
    let ret = vma_modify_flags_uffd(
        i,
        prev,
        v,
        start,
        end,
        &f,
        vm_userfaultfd_ctx { ctx: null_mut() },
        give_up,
    );
    if !is_err(ret) {
        userfaultfd_reset_ctx(ret);
    }
    ret
}
unsafe fn userfaultfd_register_range(
    c: *mut userfaultfd_ctx,
    initial: *mut vm_area_struct,
    flags: vm_flags_t,
    mut start: c_ulong,
    end: c_ulong,
    wp_async: bool,
) -> c_int {
    let mask = legacy_to_vma_flags(flags);
    let mut i = iterator((*c).mm, start);
    let mut prev = vma_prev(&mut i);
    if vstart(initial) < start {
        prev = initial;
    }
    loop {
        let mut v = vma_find(&mut i, end);
        if v.is_null() {
            break;
        }
        cond_resched();
        vm_warn!(warn_029, !vma_can_userfault(v, flags, wp_async));
        vm_warn!(warn_030, !vctx(v).is_null() && vctx(v) != c);
        vm_warn!(warn_031, !vma_test(v, VMA_MAYWRITE_BIT));
        if !(vctx(v) == c && vma_test_all_mask(v, mask)) {
            vm_warn!(
                warn_032,
                vctx(v) == c
                    && vflags(v) & (RUST_UFFD_VM_UFFD_WP | RUST_UFFD_VM_UFFD_RWP) & !flags != 0,
            );
            start = core::cmp::max(start, vstart(v));
            let end = core::cmp::min(end, vend(v));
            let mut f = vma_native_flags(v);
            vma_flags_clear_mask(&mut f, all_uffd_vma_flags());
            vma_flags_set_mask(&mut f, mask);
            v = vma_modify_flags_uffd(
                &mut i,
                prev,
                v,
                start,
                end,
                &f,
                vm_userfaultfd_ctx { ctx: c },
                false,
            );
            if is_err(v) {
                return ptr_err(v);
            }
            userfaultfd_set_ctx(v, c, flags);
            if is_vm_hugetlb_page(v) && uffd_disable_huge_pmd_share(v) {
                hugetlb_unshare_all_pmds(v);
            }
        }
        prev = v;
        start = vend(v);
    }
    0
}
unsafe fn userfaultfd_release_new(c: *mut userfaultfd_ctx) {
    let mm = (*c).mm;
    let mut i = iterator(mm, 0);
    mmap_write_lock(mm);
    loop {
        let v = vma_next(&mut i);
        if v.is_null() {
            break;
        }
        if vctx(v) == c {
            userfaultfd_reset_ctx(v);
        }
    }
    mmap_write_unlock(mm);
}
unsafe fn userfaultfd_release_all(mm: *mut mm_struct, c: *mut userfaultfd_ctx) {
    if !mmget_not_zero(mm) {
        return;
    }
    let mut i = iterator(mm, 0);
    let mut prev = null_mut();
    mmap_write_lock(mm);
    loop {
        let v = vma_next(&mut i);
        if v.is_null() {
            break;
        }
        cond_resched();
        vm_warn!(
            warn_033,
            (!vctx(v).is_null()) ^ (vflags(v) & RUST_UFFD___VM_UFFD_FLAGS != 0)
        );
        if vctx(v) != c {
            prev = v;
            continue;
        }
        prev = userfaultfd_clear_vma(&mut i, prev, v, vstart(v), vend(v));
    }
    mmap_write_unlock(mm);
    mmput(mm);
}
#[no_mangle]
#[link_section = ".data..read_mostly"]
static mut rust_uffd_sysctl_unprivileged_userfaultfd: c_int = 0;
#[link_section = ".data..ro_after_init"]
static mut userfaultfd_ctx_cachep: *mut kmem_cache = null_mut();
unsafe fn userfaultfd_features(c: *mut userfaultfd_ctx) -> u32 {
    read_features(c)
}
unsafe fn userfaultfd_is_initialized(c: *mut userfaultfd_ctx) -> bool {
    userfaultfd_features(c) & UFFD_FEATURE_INITIALIZED != 0
}
unsafe fn userfaultfd_wp_async_ctx(c: *mut userfaultfd_ctx) -> bool {
    !c.is_null() && userfaultfd_features(c) & RUST_UFFD_UFFD_FEATURE_WP_ASYNC != 0
}
unsafe fn userfaultfd_rwp_async_ctx(c: *mut userfaultfd_ctx) -> bool {
    !c.is_null() && userfaultfd_features(c) & RUST_UFFD_UFFD_FEATURE_RWP_ASYNC != 0
}
#[no_mangle]
pub unsafe extern "C" fn userfaultfd_wp_unpopulated(v: *mut vm_area_struct) -> bool {
    let c = vctx(v);
    !c.is_null() && userfaultfd_features(c) & RUST_UFFD_UFFD_FEATURE_WP_UNPOPULATED != 0
}
unsafe extern "C" fn userfaultfd_wake_function(
    w: *mut wait_queue_entry_t,
    mode: u32,
    wake_flags: c_int,
    key: *mut c_void,
) -> c_int {
    let r = key as *mut userfaultfd_wake_range;
    let u = container!(w, userfaultfd_wait_queue, wq);
    let start = (*r).start;
    let len = (*r).len;
    let a = msg_address(addr_of!((*u).msg));
    if len != 0 && (start > a || start.wrapping_add(len) <= a) {
        return 0;
    }
    write_waken(u, true);
    let ret = wake_up_state((*w).private as *mut task_struct, mode);
    if ret != 0 {
        list_del_init(addr_of_mut!((*w).entry));
    }
    ret
}
unsafe fn userfaultfd_ctx_get(c: *mut userfaultfd_ctx) {
    refcount_inc(addr_of_mut!((*c).refcount));
}
unsafe fn userfaultfd_ctx_put(c: *mut userfaultfd_ctx) {
    if refcount_dec_and_test(addr_of_mut!((*c).refcount)) {
        vm_warn!(
            warn_ctx_pending_lock,
            spin_is_locked(addr_of!((*c).fault_pending_wqh.lock))
        );
        vm_warn!(
            warn_ctx_pending_queue,
            waitqueue_active(addr_of_mut!((*c).fault_pending_wqh))
        );
        vm_warn!(
            warn_ctx_fault_lock,
            spin_is_locked(addr_of!((*c).fault_wqh.lock))
        );
        vm_warn!(
            warn_ctx_fault_queue,
            waitqueue_active(addr_of_mut!((*c).fault_wqh))
        );
        vm_warn!(
            warn_ctx_event_lock,
            spin_is_locked(addr_of!((*c).event_wqh.lock))
        );
        vm_warn!(
            warn_ctx_event_queue,
            waitqueue_active(addr_of_mut!((*c).event_wqh))
        );
        vm_warn!(warn_ctx_fd_lock, spin_is_locked(addr_of!((*c).fd_wqh.lock)));
        vm_warn!(
            warn_ctx_fd_queue,
            waitqueue_active(addr_of_mut!((*c).fd_wqh))
        );
        mmdrop((*c).mm);
        kmem_cache_free(userfaultfd_ctx_cachep, c as *mut c_void);
    }
}
unsafe fn msg_init(m: *mut uffd_msg) {
    const {
        assert!(size_of::<uffd_msg>() == 32);
    }
    core::ptr::write_bytes(m, 0, 1);
}
unsafe fn userfault_msg(
    address: c_ulong,
    real: c_ulong,
    flags: u32,
    reason: c_ulong,
    features: u32,
) -> uffd_msg {
    let mut m: uffd_msg = zeroed();
    m.event = UFFD_EVENT_PAGEFAULT as u8;
    let address = if features & RUST_UFFD_UFFD_FEATURE_EXACT_ADDRESS != 0 {
        real
    } else {
        address
    };
    let mut f = 0;
    if flags & RUST_UFFD_FAULT_FLAG_WRITE != 0 {
        f |= RUST_UFFD_UFFD_PAGEFAULT_FLAG_WRITE;
    }
    if reason & RUST_UFFD_VM_UFFD_WP != 0 {
        f |= RUST_UFFD_UFFD_PAGEFAULT_FLAG_WP;
    }
    if reason & RUST_UFFD_VM_UFFD_RWP != 0 {
        f |= RUST_UFFD_UFFD_PAGEFAULT_FLAG_RWP;
    }
    if reason & RUST_UFFD_VM_UFFD_MINOR != 0 {
        f |= RUST_UFFD_UFFD_PAGEFAULT_FLAG_MINOR;
    }
    let tid = if features & RUST_UFFD_UFFD_FEATURE_THREAD_ID != 0 {
        task_pid_vnr(current())
    } else {
        0
    };
    msg_set_pagefault(&mut m, address, f, tid);
    m
}
unsafe fn userfaultfd_huge_must_wait(
    c: *mut userfaultfd_ctx,
    vmf: *mut vm_fault,
    reason: c_ulong,
) -> bool {
    #[cfg(CONFIG_HUGETLB_PAGE)]
    {
        let v = vmf_vma(vmf);
        assert_fault_locked(vmf);
        let p = hugetlb_walk(v, vmf_address(vmf), vma_mmu_pagesize(v));
        if p.is_null() {
            return true;
        }
        let e = huge_ptep_get((*v).vm_mm, vmf_address(vmf), p);
        if huge_pte_none(e) || pte_is_uffd_marker(e) {
            return true;
        }
        if !pte_present(e) {
            return false;
        }
        return !huge_pte_write(e) && reason & RUST_UFFD_VM_UFFD_WP != 0
            || pte_protnone(e) && huge_pte_uffd(e) && reason & RUST_UFFD_VM_UFFD_RWP != 0;
    }
    #[cfg(not(CONFIG_HUGETLB_PAGE))]
    {
        vm_warn!(warn_036, true);
        false
    }
}
unsafe fn userfaultfd_must_wait(c: *mut userfaultfd_ctx, f: *mut vm_fault, r: c_ulong) -> bool {
    assert_fault_locked(f);
    let a = vmf_address(f);
    let pgd = pgd_offset((*c).mm, a);
    if !pgd_present(*pgd) {
        return true;
    }
    let p4d = p4d_offset(pgd, a);
    if !p4d_present(*p4d) {
        return true;
    }
    let pud = pud_offset(p4d, a);
    if !pud_present(*pud) {
        return true;
    }
    let pmd = pmd_offset(pud, a);
    loop {
        let e = pmdp_get_lockless(pmd);
        if pmd_none(e) {
            return true;
        }
        if !pmd_present(e) {
            return false;
        }
        if pmd_trans_huge(e) {
            return !pmd_write(e) && r & RUST_UFFD_VM_UFFD_WP != 0
                || pmd_protnone(e) && pmd_uffd(e) && r & RUST_UFFD_VM_UFFD_RWP != 0;
        }
        let p = pte_offset_map(pmd, a);
        if p.is_null() {
            continue;
        }
        let e = ptep_get(p);
        let ret = pte_none(e)
            || pte_is_uffd_marker(e)
            || pte_present(e)
                && (!pte_write(e) && r & RUST_UFFD_VM_UFFD_WP != 0
                    || pte_protnone(e) && pte_uffd(e) && r & RUST_UFFD_VM_UFFD_RWP != 0);
        pte_unmap(p);
        return ret;
    }
}
fn userfaultfd_get_blocking_state(f: u32) -> u32 {
    if f & RUST_UFFD_FAULT_FLAG_INTERRUPTIBLE != 0 {
        RUST_UFFD_TASK_INTERRUPTIBLE
    } else if f & RUST_UFFD_FAULT_FLAG_KILLABLE != 0 {
        RUST_UFFD_TASK_KILLABLE
    } else {
        RUST_UFFD_TASK_UNINTERRUPTIBLE
    }
}
#[no_mangle]
pub unsafe extern "C" fn handle_userfault(f: *mut vm_fault, reason: c_ulong) -> vm_fault_t {
    let v = vmf_vma(f);
    let mm = (*v).vm_mm;
    let sigbus = RUST_UFFD_VM_FAULT_SIGBUS as vm_fault_t;
    if current_flags() & (RUST_UFFD_PF_EXITING | RUST_UFFD_PF_DUMPCORE) != 0 {
        return sigbus;
    }
    assert_fault_locked(f);
    let c = vctx(v);
    if c.is_null() {
        return sigbus;
    }
    vm_warn!(warn_037, (*c).mm != mm);
    vm_warn!(warn_038, reason & !RUST_UFFD___VM_UFFD_FLAGS != 0);
    vm_warn!(
        warn_039,
        reason == 0 || reason & reason.wrapping_sub(1) != 0
    );
    if (*c).features & RUST_UFFD_UFFD_FEATURE_SIGBUS != 0
        || vmf_flags(f) & RUST_UFFD_FAULT_FLAG_USER == 0
            && (*c).flags & RUST_UFFD_UFFD_USER_MODE_ONLY != 0
    {
        return sigbus;
    }
    if vmf_flags(f) & RUST_UFFD_FAULT_FLAG_ALLOW_RETRY == 0 {
        vm_warn!(
            warn_040,
            vmf_flags(f) & RUST_UFFD_FAULT_FLAG_RETRY_NOWAIT != 0
        );
        #[cfg(CONFIG_DEBUG_VM)]
        if printk_ratelimit() {
            debug_missing_allow_retry(vmf_flags(f));
            dump_stack();
        }
        return sigbus;
    }
    let retry = RUST_UFFD_VM_FAULT_RETRY as vm_fault_t;
    if vmf_flags(f) & RUST_UFFD_FAULT_FLAG_RETRY_NOWAIT != 0 {
        return retry;
    }
    if read_released(c) {
        release_fault_lock(f);
        return retry;
    }
    userfaultfd_ctx_get(c);
    let mut u: userfaultfd_wait_queue = zeroed();
    init_waitqueue_func_entry(&mut u.wq, Some(userfaultfd_wake_function));
    u.wq.private = current() as *mut c_void;
    u.msg = userfault_msg(
        vmf_address(f),
        vmf_real_address(f),
        vmf_flags(f),
        reason,
        (*c).features,
    );
    u.ctx = c;
    u.waken = false;
    let state = userfaultfd_get_blocking_state(vmf_flags(f));
    let huge = is_vm_hugetlb_page(v);
    if huge {
        hugetlb_vma_lock_read(v);
    }
    spin_lock_irq(addr_of_mut!((*c).fault_pending_wqh.lock));
    __add_wait_queue(addr_of_mut!((*c).fault_pending_wqh), &mut u.wq);
    set_current_state(state);
    spin_unlock_irq(addr_of_mut!((*c).fault_pending_wqh.lock));
    let wait = if huge {
        let w = userfaultfd_huge_must_wait(c, f, reason);
        hugetlb_vma_unlock_read(v);
        w
    } else {
        userfaultfd_must_wait(c, f, reason)
    };
    release_fault_lock(f);
    if wait && !read_released(c) {
        wake_up_poll(addr_of_mut!((*c).fd_wqh), RUST_UFFD_EPOLLIN);
        schedule();
    }
    __set_current_state(RUST_UFFD_TASK_RUNNING);
    if !list_empty_careful(&u.wq.entry) {
        spin_lock_irq(addr_of_mut!((*c).fault_pending_wqh.lock));
        list_del(&mut u.wq.entry);
        spin_unlock_irq(addr_of_mut!((*c).fault_pending_wqh.lock));
    }
    userfaultfd_ctx_put(c);
    retry
}
unsafe fn userfaultfd_event_wait_completion(
    c: *mut userfaultfd_ctx,
    e: *mut userfaultfd_wait_queue,
) {
    if !warn_exit_event(current_flags() & RUST_UFFD_PF_EXITING != 0) {
        (*e).ctx = c;
        init_waitqueue_entry(addr_of_mut!((*e).wq), current());
        let mut release = null_mut();
        spin_lock_irq(addr_of_mut!((*c).event_wqh.lock));
        __add_wait_queue(addr_of_mut!((*c).event_wqh), addr_of_mut!((*e).wq));
        loop {
            set_current_state(RUST_UFFD_TASK_KILLABLE);
            if (*e).msg.event == 0 {
                break;
            }
            if read_released(c) || fatal_signal_pending(current()) {
                __remove_wait_queue(addr_of_mut!((*c).event_wqh), addr_of_mut!((*e).wq));
                if (*e).msg.event == UFFD_EVENT_FORK as u8 {
                    release = msg_fork_ctx(addr_of!((*e).msg));
                }
                break;
            }
            spin_unlock_irq(addr_of_mut!((*c).event_wqh.lock));
            wake_up_poll(addr_of_mut!((*c).fd_wqh), RUST_UFFD_EPOLLIN);
            schedule();
            spin_lock_irq(addr_of_mut!((*c).event_wqh.lock));
        }
        __set_current_state(RUST_UFFD_TASK_RUNNING);
        spin_unlock_irq(addr_of_mut!((*c).event_wqh.lock));
        if !release.is_null() {
            userfaultfd_release_new(release);
            userfaultfd_ctx_put(release);
        }
    }
    atomic_dec(addr_of_mut!((*c).mmap_changing));
    vm_warn!(warn_041, atomic_read(addr_of!((*c).mmap_changing)) < 0);
    userfaultfd_ctx_put(c);
}
unsafe fn userfaultfd_event_complete(c: *mut userfaultfd_ctx, e: *mut userfaultfd_wait_queue) {
    (*e).msg.event = 0;
    wake_up_locked(addr_of_mut!((*c).event_wqh));
    __remove_wait_queue(addr_of_mut!((*c).event_wqh), addr_of_mut!((*e).wq));
}
unsafe fn acquire_map_change(c: *mut userfaultfd_ctx) {
    userfaultfd_ctx_get(c);
    down_write(addr_of_mut!((*c).map_changing_lock));
    atomic_inc(addr_of_mut!((*c).mmap_changing));
    up_write(addr_of_mut!((*c).map_changing_lock));
}
#[no_mangle]
pub unsafe extern "C" fn dup_userfaultfd(v: *mut vm_area_struct, fcs: *mut list_head) -> c_int {
    let old = vctx(v);
    if old.is_null() {
        return 0;
    }
    if (*old).features & RUST_UFFD_UFFD_FEATURE_EVENT_FORK == 0 {
        userfaultfd_reset_ctx(v);
        return 0;
    }
    let mut c = null_mut();
    let mut n = (*fcs).next;
    while n != fcs {
        let f = container!(n, userfaultfd_fork_ctx, list);
        if (*f).orig == old {
            c = (*f).new;
            break;
        }
        n = (*n).next;
    }
    if c.is_null() {
        let f = kmalloc(
            size_of::<userfaultfd_fork_ctx>() as _,
            RUST_UFFD_GFP_KERNEL as _,
        ) as *mut userfaultfd_fork_ctx;
        if f.is_null() {
            return error(ENOMEM);
        }
        c = kmem_cache_alloc(userfaultfd_ctx_cachep, RUST_UFFD_GFP_KERNEL as _)
            as *mut userfaultfd_ctx;
        if c.is_null() {
            kfree(f as _);
            return error(ENOMEM);
        }
        refcount_set(addr_of_mut!((*c).refcount), 1);
        (*c).flags = (*old).flags;
        (*c).features = (*old).features;
        (*c).released = false;
        init_rwsem_fork(addr_of_mut!((*c).map_changing_lock));
        atomic_set(addr_of_mut!((*c).mmap_changing), 0);
        (*c).mm = (*v).vm_mm;
        mmgrab((*c).mm);
        acquire_map_change(old);
        (*f).orig = old;
        (*f).new = c;
        list_add_tail(addr_of_mut!((*f).list), fcs);
    }
    (*v).vm_userfaultfd_ctx.ctx = c;
    0
}
unsafe fn dup_fctx(f: *mut userfaultfd_fork_ctx) {
    let mut e: userfaultfd_wait_queue = zeroed();
    msg_init(&mut e.msg);
    e.msg.event = UFFD_EVENT_FORK as u8;
    msg_set_fork_ctx(&mut e.msg, (*f).new);
    userfaultfd_event_wait_completion((*f).orig, &mut e);
}
#[no_mangle]
pub unsafe extern "C" fn dup_userfaultfd_complete(h: *mut list_head) {
    while !list_empty(h) {
        let f = container!((*h).next, userfaultfd_fork_ctx, list);
        dup_fctx(f);
        list_del(addr_of_mut!((*f).list));
        kfree(f as _);
    }
}
#[no_mangle]
pub unsafe extern "C" fn dup_userfaultfd_fail(h: *mut list_head) {
    while !list_empty(h) {
        let f = container!((*h).next, userfaultfd_fork_ctx, list);
        let old = (*f).orig;
        atomic_dec(addr_of_mut!((*old).mmap_changing));
        vm_warn!(warn_042, atomic_read(addr_of!((*old).mmap_changing)) < 0);
        userfaultfd_ctx_put(old);
        userfaultfd_ctx_put((*f).new);
        list_del(addr_of_mut!((*f).list));
        kfree(f as _);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mremap_userfaultfd_prep(
    v: *mut vm_area_struct,
    vc: *mut vm_userfaultfd_ctx,
) {
    let c = vctx(v);
    if c.is_null() {
        return;
    }
    if (*c).features & RUST_UFFD_UFFD_FEATURE_EVENT_REMAP != 0 {
        (*vc).ctx = c;
        acquire_map_change(c);
    } else {
        userfaultfd_reset_ctx(v);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mremap_userfaultfd_complete(
    vc: *mut vm_userfaultfd_ctx,
    from: c_ulong,
    to: c_ulong,
    len: c_ulong,
) {
    let c = (*vc).ctx;
    if c.is_null() {
        return;
    }
    let mut e: userfaultfd_wait_queue = zeroed();
    msg_init(&mut e.msg);
    e.msg.event = UFFD_EVENT_REMAP as u8;
    msg_set_remap(&mut e.msg, from, to, len);
    userfaultfd_event_wait_completion(c, &mut e);
}
#[no_mangle]
pub unsafe extern "C" fn mremap_userfaultfd_fail(vc: *mut vm_userfaultfd_ctx) {
    let c = (*vc).ctx;
    if c.is_null() {
        return;
    }
    atomic_dec(addr_of_mut!((*c).mmap_changing));
    vm_warn!(warn_043, atomic_read(addr_of!((*c).mmap_changing)) < 0);
    userfaultfd_ctx_put(c);
}
#[no_mangle]
pub unsafe extern "C" fn userfaultfd_remove(
    v: *mut vm_area_struct,
    start: c_ulong,
    end: c_ulong,
) -> bool {
    let c = vctx(v);
    if c.is_null() || (*c).features & RUST_UFFD_UFFD_FEATURE_EVENT_REMOVE == 0 {
        return true;
    }
    acquire_map_change(c);
    mmap_read_unlock((*v).vm_mm);
    let mut e: userfaultfd_wait_queue = zeroed();
    msg_init(&mut e.msg);
    e.msg.event = UFFD_EVENT_REMOVE as u8;
    msg_set_remove(&mut e.msg, start, end);
    userfaultfd_event_wait_completion(c, &mut e);
    false
}
unsafe fn has_unmap_ctx(
    c: *mut userfaultfd_ctx,
    h: *mut list_head,
    start: c_ulong,
    end: c_ulong,
) -> bool {
    let mut n = (*h).next;
    while n != h {
        let u = container!(n, userfaultfd_unmap_ctx, list);
        if (*u).ctx == c && (*u).start == start && (*u).end == end {
            return true;
        }
        n = (*n).next;
    }
    false
}
#[no_mangle]
pub unsafe extern "C" fn userfaultfd_unmap_prep(
    v: *mut vm_area_struct,
    start: c_ulong,
    end: c_ulong,
    h: *mut list_head,
) -> c_int {
    let c = vctx(v);
    if c.is_null()
        || (*c).features & RUST_UFFD_UFFD_FEATURE_EVENT_UNMAP == 0
        || has_unmap_ctx(c, h, start, end)
    {
        return 0;
    }
    let u = kzalloc(
        size_of::<userfaultfd_unmap_ctx>() as _,
        RUST_UFFD_GFP_KERNEL as _,
    ) as *mut userfaultfd_unmap_ctx;
    if u.is_null() {
        return error(ENOMEM);
    }
    acquire_map_change(c);
    (*u).ctx = c;
    (*u).start = start;
    (*u).end = end;
    list_add_tail(addr_of_mut!((*u).list), h);
    0
}
#[no_mangle]
pub unsafe extern "C" fn userfaultfd_unmap_complete(mm: *mut mm_struct, h: *mut list_head) {
    while !list_empty(h) {
        let u = container!((*h).next, userfaultfd_unmap_ctx, list);
        let mut e: userfaultfd_wait_queue = zeroed();
        msg_init(&mut e.msg);
        e.msg.event = UFFD_EVENT_UNMAP as u8;
        msg_set_remove(&mut e.msg, (*u).start, (*u).end);
        userfaultfd_event_wait_completion((*u).ctx, &mut e);
        list_del(addr_of_mut!((*u).list));
        kfree(u as _);
    }
}
unsafe extern "C" fn userfaultfd_release(inode: *mut inode, file: *mut file) -> c_int {
    let c = (*file).private_data as *mut userfaultfd_ctx;
    let mut r: userfaultfd_wake_range = zeroed();
    write_released(c, true);
    userfaultfd_release_all((*c).mm, c);
    spin_lock_irq(addr_of_mut!((*c).fault_pending_wqh.lock));
    __wake_up_locked_key(
        addr_of_mut!((*c).fault_pending_wqh),
        RUST_UFFD_TASK_NORMAL,
        &mut r as *mut _ as *mut c_void,
    );
    __wake_up(
        addr_of_mut!((*c).fault_wqh),
        RUST_UFFD_TASK_NORMAL,
        1,
        &mut r as *mut _ as *mut c_void,
    );
    spin_unlock_irq(addr_of_mut!((*c).fault_pending_wqh.lock));
    wake_up_all(addr_of_mut!((*c).event_wqh));
    wake_up_poll(addr_of_mut!((*c).fd_wqh), RUST_UFFD_EPOLLHUP);
    userfaultfd_ctx_put(c);
    0
}
unsafe fn find_userfault_in(q: *mut wait_queue_head_t) -> *mut userfaultfd_wait_queue {
    lockdep_assert_held(addr_of!((*q).lock));
    if !waitqueue_active(q) {
        return null_mut();
    }
    let w = container!((*q).head.prev, wait_queue_entry_t, entry);
    container!(w, userfaultfd_wait_queue, wq)
}
unsafe fn find_userfault(c: *mut userfaultfd_ctx) -> *mut userfaultfd_wait_queue {
    find_userfault_in(addr_of_mut!((*c).fault_pending_wqh))
}
unsafe fn find_userfault_evt(c: *mut userfaultfd_ctx) -> *mut userfaultfd_wait_queue {
    find_userfault_in(addr_of_mut!((*c).event_wqh))
}
unsafe extern "C" fn userfaultfd_poll(file: *mut file, wait: *mut poll_table) -> __poll_t {
    let c = (*file).private_data as *mut userfaultfd_ctx;
    poll_wait(file, addr_of_mut!((*c).fd_wqh), wait);
    if !userfaultfd_is_initialized(c) || (*file).f_flags & RUST_UFFD_O_NONBLOCK == 0 {
        return RUST_UFFD_EPOLLERR;
    }
    smp_mb();
    if waitqueue_active(addr_of_mut!((*c).fault_pending_wqh))
        || waitqueue_active(addr_of_mut!((*c).event_wqh))
    {
        RUST_UFFD_EPOLLIN
    } else {
        0
    }
}
unsafe fn resolve_userfault_fork(
    new: *mut userfaultfd_ctx,
    inode: *mut inode,
    msg: *mut uffd_msg,
) -> c_int {
    let fd = anon_inode_create_getfd(
        c"[userfaultfd]".as_ptr(),
        addr_of!(userfaultfd_fops),
        new as _,
        (RUST_UFFD_O_RDONLY | ((*new).flags & RUST_UFFD_UFFD_SHARED_FCNTL_FLAGS)) as c_int,
        inode,
    );
    if fd < 0 {
        return fd;
    }
    msg_set_fork_ctx(msg, null_mut());
    msg_set_fork_fd(msg, fd);
    0
}
unsafe fn userfaultfd_ctx_read(
    c: *mut userfaultfd_ctx,
    no_wait: bool,
    msg: *mut uffd_msg,
    inode: *mut inode,
) -> c_long {
    let mut wait: wait_queue_entry_t = zeroed();
    declare_waitqueue(&mut wait, current());
    let mut fork_event: list_head = zeroed();
    list_init(&mut fork_event);
    let mut new = null_mut();
    spin_lock_irq(addr_of_mut!((*c).fd_wqh.lock));
    __add_wait_queue(addr_of_mut!((*c).fd_wqh), &mut wait);
    let mut ret = loop {
        set_current_state(RUST_UFFD_TASK_INTERRUPTIBLE);
        spin_lock(addr_of_mut!((*c).fault_pending_wqh.lock));
        let mut u = find_userfault(c);
        if !u.is_null() {
            write_seqcount_begin(addr_of_mut!((*c).refile_seq));
            list_del(addr_of_mut!((*u).wq.entry));
            add_wait_queue(addr_of_mut!((*c).fault_wqh), addr_of_mut!((*u).wq));
            write_seqcount_end(addr_of_mut!((*c).refile_seq));
            *msg = (*u).msg;
            spin_unlock(addr_of_mut!((*c).fault_pending_wqh.lock));
            break 0;
        }
        spin_unlock(addr_of_mut!((*c).fault_pending_wqh.lock));
        spin_lock(addr_of_mut!((*c).event_wqh.lock));
        u = find_userfault_evt(c);
        if !u.is_null() {
            *msg = (*u).msg;
            if (*u).msg.event == UFFD_EVENT_FORK as u8 {
                new = msg_fork_ctx(addr_of!((*u).msg));
                list_move(addr_of_mut!((*u).wq.entry), &mut fork_event);
                userfaultfd_ctx_get(new);
            } else {
                userfaultfd_event_complete(c, u);
            }
            spin_unlock(addr_of_mut!((*c).event_wqh.lock));
            break 0;
        }
        spin_unlock(addr_of_mut!((*c).event_wqh.lock));
        if signal_pending(current()) {
            break error(ERESTARTSYS) as c_long;
        }
        if no_wait {
            break error(EAGAIN) as c_long;
        }
        spin_unlock_irq(addr_of_mut!((*c).fd_wqh.lock));
        schedule();
        spin_lock_irq(addr_of_mut!((*c).fd_wqh.lock));
    };
    __remove_wait_queue(addr_of_mut!((*c).fd_wqh), &mut wait);
    __set_current_state(RUST_UFFD_TASK_RUNNING);
    spin_unlock_irq(addr_of_mut!((*c).fd_wqh.lock));
    if ret == 0 && (*msg).event == UFFD_EVENT_FORK as u8 {
        ret = resolve_userfault_fork(new, inode, msg) as c_long;
        spin_lock_irq(addr_of_mut!((*c).event_wqh.lock));
        if !list_empty(&fork_event) {
            userfaultfd_ctx_put(new);
            let w = container!(fork_event.next, wait_queue_entry_t, entry);
            let u = container!(w, userfaultfd_wait_queue, wq);
            list_del(addr_of_mut!((*u).wq.entry));
            __add_wait_queue(addr_of_mut!((*c).event_wqh), addr_of_mut!((*u).wq));
            if ret == 0 {
                userfaultfd_event_complete(c, u);
            }
        } else if ret != 0 {
            userfaultfd_ctx_put(new);
        }
        spin_unlock_irq(addr_of_mut!((*c).event_wqh.lock));
    }
    ret
}
unsafe extern "C" fn userfaultfd_read_iter(iocb: *mut kiocb, to: *mut iov_iter) -> c_long {
    let file = (*iocb).ki_filp;
    let c = (*file).private_data as *mut userfaultfd_ctx;
    let inode = file_inode(file);
    let mut ret = 0;
    if !userfaultfd_is_initialized(c) {
        return error(EINVAL) as c_long;
    }
    let mut no_wait = (*file).f_flags & RUST_UFFD_O_NONBLOCK != 0
        || (*iocb).ki_flags & RUST_UFFD_IOCB_NOWAIT != 0;
    loop {
        if iov_iter_count(to) < size_of::<uffd_msg>() {
            return if ret != 0 {
                ret
            } else {
                error(EINVAL) as c_long
            };
        }
        let mut msg = zeroed();
        let r = userfaultfd_ctx_read(c, no_wait, &mut msg, inode);
        if r < 0 {
            return if ret != 0 { ret } else { r };
        }
        if !copy_to_iter_full(&msg as *const _ as _, size_of::<uffd_msg>(), to) {
            return if ret != 0 {
                ret
            } else {
                error(EFAULT) as c_long
            };
        }
        ret += size_of::<uffd_msg>() as c_long;
        no_wait = true;
    }
}
unsafe fn __wake_userfault(c: *mut userfaultfd_ctx, r: *mut userfaultfd_wake_range) {
    spin_lock_irq(addr_of_mut!((*c).fault_pending_wqh.lock));
    if waitqueue_active(addr_of_mut!((*c).fault_pending_wqh)) {
        __wake_up_locked_key(
            addr_of_mut!((*c).fault_pending_wqh),
            RUST_UFFD_TASK_NORMAL,
            r as _,
        );
    }
    if waitqueue_active(addr_of_mut!((*c).fault_wqh)) {
        __wake_up(
            addr_of_mut!((*c).fault_wqh),
            RUST_UFFD_TASK_NORMAL,
            1,
            r as _,
        );
    }
    spin_unlock_irq(addr_of_mut!((*c).fault_pending_wqh.lock));
}
unsafe fn wake_userfault(c: *mut userfaultfd_ctx, r: *mut userfaultfd_wake_range) {
    smp_mb();
    let need = loop {
        let seq = read_seqcount_begin(addr_of!((*c).refile_seq));
        let need = waitqueue_active(addr_of_mut!((*c).fault_pending_wqh))
            || waitqueue_active(addr_of_mut!((*c).fault_wqh));
        cond_resched();
        if !read_seqcount_retry(addr_of!((*c).refile_seq), seq) {
            break need;
        }
    };
    if need {
        __wake_userfault(c, r);
    }
}
