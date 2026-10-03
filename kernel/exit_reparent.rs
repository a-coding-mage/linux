// SPDX-License-Identifier: GPL-2.0-only
unsafe fn coredump_task_exit(tsk: *mut task_struct, core_state: *mut core_state) {
    let mut this: core_thread = zeroed();
    this.task = tsk;
    if (*tsk).flags & PF_SIGNALED != 0 {
        this.next = b::rust_exit_core_exchange(addr_of_mut!((*core_state).dumper.next), addr_of_mut!(this));
    } else { this.task = null_mut(); }
    // atomic_dec_and_test implies the full barrier publishing the stack node.
    if b::rust_exit_atomic_dec(addr_of_mut!((*core_state).nr_threads)) {
        b::complete(addr_of_mut!((*core_state).startup));
    }
    loop {
        b::rust_exit_set_state((TASK_IDLE | TASK_FREEZABLE) as _);
        // coredump_finish() writes this field while we sleep.
        if read_volatile(addr_of!(this.task)).is_null() { break; }
        b::schedule();
    }
    b::rust_exit_set_state_unbarriered(TASK_RUNNING as _);
}
#[cfg(CONFIG_MEMCG)]
unsafe fn __try_to_set_owner(tsk: *mut task_struct, mm: *mut mm_struct) -> bool {
    let mut ret = false;
    b::rust_exit_spin_lock(addr_of_mut!((*tsk).alloc_lock));
    if (*tsk).mm == mm {
        b::rust_exit_tasklist_read_unlock();
        write_volatile(b::rust_exit_mm_owner(mm), tsk);
        #[cfg(CONFIG_LRU_GEN_WALKS_MMU)] b::lru_gen_migrate_mm(mm);
        ret = true;
    }
    b::rust_exit_spin_unlock(addr_of_mut!((*tsk).alloc_lock));
    ret
}
#[cfg(CONFIG_MEMCG)]
unsafe fn try_to_set_owner(g: *mut task_struct, mm: *mut mm_struct) -> bool {
    b::rust_exit_rcu_list_check(true);
    let head = addr_of_mut!((*(*g).signal).thread_head);
    let mut node = b::rust_exit_list_next(head);
    while node != head {
        let t = task_from_thread(node);
        let t_mm = read_volatile(addr_of!((*t).mm));
        if t_mm == mm {
            if __try_to_set_owner(t, mm) { return true; }
        } else if !t_mm.is_null() { break; }
        node = b::rust_exit_list_next(node);
    }
    false
}
#[cfg(CONFIG_MEMCG)]
#[no_mangle]
pub unsafe extern "C" fn mm_update_next_owner(mm: *mut mm_struct) {
    let p = current_task();
    if *b::rust_exit_mm_owner(mm) != p { return; }
    if b::rust_exit_atomic_read(b::rust_exit_mm_users(mm)) <= 1 {
        write_volatile(b::rust_exit_mm_owner(mm), null_mut());
        return;
    }
    b::rust_exit_tasklist_read_lock();
    let head = addr_of_mut!((*p).children);
    let mut node = (*head).next;
    while node != head {
        if try_to_set_owner(task_from_sibling(node), mm) { return; }
        node = (*node).next;
    }
    let head = addr_of_mut!((*(*p).real_parent).children);
    let mut node = (*head).next;
    while node != head {
        if try_to_set_owner(task_from_sibling(node), mm) { return; }
        node = (*node).next;
    }
    let head = addr_of_mut!(b::init_task.tasks);
    let mut node = b::rust_exit_list_next(head);
    while node != head {
        if b::rust_exit_atomic_read(b::rust_exit_mm_users(mm)) <= 1 { break; }
        let g = node.cast::<u8>().sub(offset_of!(task_struct, tasks)).cast::<task_struct>();
        if (*g).flags & PF_KTHREAD == 0 && try_to_set_owner(g, mm) { return; }
        node = b::rust_exit_list_next(node);
    }
    b::rust_exit_tasklist_read_unlock();
    write_volatile(b::rust_exit_mm_owner(mm), null_mut());
}
unsafe fn exit_mm_sched_cache(mm: *mut mm_struct) {
    #[cfg(all(CONFIG_SCHED_CACHE, CONFIG_NUMA_BALANCING))] {
        let faults = (*current_task()).total_numa_faults;
        if faults == 0 { return; }
        let footprint = b::rust_exit_mm_footprint(mm);
        let fp = read_volatile(footprint);
        write_volatile(footprint, fp - core::cmp::min(fp, faults));
    }
}
unsafe fn exit_mm() {
    let tsk = current_task();
    let mm = (*tsk).mm;
    b::mm_exit_exec_release(tsk, mm);
    if mm.is_null() { return; }
    exit_mm_sched_cache(mm);
    b::rust_exit_mmap_read_lock(mm);
    #[cfg(CONFIG_MMU_LAZY_TLB_REFCOUNT)] b::rust_exit_atomic_inc(b::rust_exit_mm_count(mm));
    b::rust_exit_bug_on(mm != (*tsk).active_mm);
    b::rust_exit_spin_lock(addr_of_mut!((*tsk).alloc_lock));
    b::rust_exit_mb_after_spinlock();
    local_irq_disable();
    (*tsk).mm = null_mut();
    #[cfg(CONFIG_MEMBARRIER)] b::membarrier_update_current_mm(null_mut());
    b::rust_exit_enter_lazy_tlb(mm, tsk);
    local_irq_enable();
    b::rust_exit_spin_unlock(addr_of_mut!((*tsk).alloc_lock));
    b::rust_exit_mmap_read_unlock(mm);
    #[cfg(CONFIG_MEMCG)] mm_update_next_owner(mm);
    b::mmput(mm);
    if b::rust_exit_memdie() { b::exit_oom_victim(); }
}
unsafe fn find_alive_thread(p: *mut task_struct) -> *mut task_struct {
    b::rust_exit_rcu_list_check(true);
    let head = addr_of_mut!((*(*p).signal).thread_head);
    let mut node = b::rust_exit_list_next(head);
    while node != head {
        let t = task_from_thread(node);
        if (*t).flags & PF_EXITING == 0 { return t; }
        node = b::rust_exit_list_next(node);
    }
    null_mut()
}
unsafe fn release_dead_list(dead: *mut list_head) {
    // Advance before release_task: it may release the current task's storage.
    let mut node = (*dead).next;
    while node != dead {
        let next = (*node).next;
        let p = task_from_ptrace(node);
        list_del_init(node);
        release_task(p);
        node = next;
    }
}
unsafe fn find_child_reaper(father: *mut task_struct, dead: *mut list_head) -> *mut task_struct {
    let pid_ns = b::task_active_pid_ns(father);
    let reaper = (*pid_ns).child_reaper;
    if reaper != father { return reaper; }
    let reaper = find_alive_thread(father);
    if !reaper.is_null() {
        b::rust_exit_reaper_assert_exclusive(pid_ns);
        write_volatile(addr_of_mut!((*pid_ns).child_reaper), reaper);
        return reaper;
    }
    b::rust_exit_tasklist_write_unlock_irq();
    release_dead_list(dead);
    b::zap_pid_ns_processes(pid_ns);
    b::rust_exit_tasklist_write_lock_irq();
    father
}
unsafe fn find_new_reaper(father: *mut task_struct, child_reaper: *mut task_struct) -> *mut task_struct {
    let thread = find_alive_thread(father);
    if !thread.is_null() { return thread; }
    if (*(*father).signal).has_child_subreaper() != 0 {
        let ns_level = (*task_pid(father)).level;
        let mut reaper = (*father).real_parent;
        while (*task_pid(reaper)).level == ns_level {
            if reaper == addr_of_mut!(b::init_task) { break; }
            if (*(*reaper).signal).is_child_subreaper() != 0 {
                let thread = find_alive_thread(reaper);
                if !thread.is_null() { return thread; }
            }
            reaper = (*reaper).real_parent;
        }
    }
    child_reaper
}
unsafe fn reparent_leader(father: *mut task_struct, p: *mut task_struct, dead: *mut list_head) {
    if (*p).exit_state == EXIT_DEAD as c_int { return; }
    (*p).exit_signal = SIGCHLD as c_int;
    if (*p).ptrace == 0 && (*p).exit_state == EXIT_ZOMBIE as c_int && thread_group_empty(p) {
        if b::do_notify_parent(p, (*p).exit_signal) {
            (*p).exit_state = EXIT_DEAD as c_int;
            list_add(addr_of_mut!((*p).ptrace_entry), dead);
        }
    }
    kill_orphaned_pgrp(p, father);
}
unsafe fn forget_original_parent(father: *mut task_struct, dead: *mut list_head) {
    if !list_empty(addr_of!((*father).ptraced)) { b::exit_ptrace(father, dead); }
    let reaper = find_child_reaper(father, dead);
    if list_empty(addr_of!((*father).children)) { return; }
    let reaper = find_new_reaper(father, reaper);
    let head = addr_of_mut!((*father).children);
    let mut node = (*head).next;
    while node != head {
        let p = task_from_sibling(node);
        b::rust_exit_rcu_list_check(true);
        let threads = addr_of_mut!((*(*p).signal).thread_head);
        let mut thread_node = b::rust_exit_list_next(threads);
        while thread_node != threads {
            let t = task_from_thread(thread_node);
            write_volatile(addr_of_mut!((*t).real_parent), reaper); // RCU_INIT_POINTER
            b::rust_exit_bug_on(((*t).ptrace == 0) != (read_volatile(addr_of!((*t).parent)) == father));
            if (*t).ptrace == 0 { (*t).parent = (*t).real_parent; }
            if (*t).pdeath_signal != 0 {
                b::group_send_sig_info((*t).pdeath_signal, null_mut(), t, PIDTYPE_TGID);
            }
            thread_node = b::rust_exit_list_next(thread_node);
        }
        if !same_thread_group(reaper, father) { reparent_leader(father, p, dead); }
        node = (*node).next;
    }
    list_splice_tail_init(head, addr_of_mut!((*reaper).children));
}
unsafe fn exit_notify(tsk: *mut task_struct, group_dead: bool) {
    let mut dead: list_head = zeroed();
    list_init(addr_of_mut!(dead));
    b::rust_exit_tasklist_write_lock_irq();
    forget_original_parent(tsk, addr_of_mut!(dead));
    if group_dead { kill_orphaned_pgrp((*tsk).group_leader, null_mut()); }
    (*tsk).exit_state = EXIT_ZOMBIE as c_int;
    let autoreap = if (*tsk).ptrace != 0 {
        let sig = if thread_group_empty(tsk) && !ptrace_reparented(tsk) { (*tsk).exit_signal } else { SIGCHLD as c_int };
        b::do_notify_parent(tsk, sig)
    } else if thread_group_leader(tsk) {
        thread_group_empty(tsk) && b::do_notify_parent(tsk, (*tsk).exit_signal)
    } else {
        b::do_notify_pidfd(tsk);
        true
    };
    if autoreap {
        (*tsk).exit_state = EXIT_DEAD as c_int;
        list_add(addr_of_mut!((*tsk).ptrace_entry), addr_of_mut!(dead));
    }
    if (*(*tsk).signal).notify_count < 0 { b::wake_up_process((*(*tsk).signal).group_exec_task); }
    b::rust_exit_tasklist_write_unlock_irq();
    release_dead_list(addr_of_mut!(dead));
}
