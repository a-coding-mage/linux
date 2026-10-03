// SPDX-License-Identifier: GPL-2.0-only
// Algorithms from the genuine configured headers. C only supplies individual
// synchronization, architecture, instrumentation and checked-usercopy actions.
#[inline] unsafe fn current_task() -> *mut task_struct { b::rust_exit_current() }
#[inline] fn size_of_val_raw<T>(_p: *const T) -> usize { size_of::<T>() }
#[inline] unsafe fn wait_flags(wo: *const wait_opts, mask: c_uint) -> bool { ((*wo).wo_flags as c_uint) & mask != 0 }
#[inline] fn is_err<T>(p: *const T) -> bool { p as usize >= (-(MAX_ERRNO as isize)) as usize }
#[inline] unsafe fn task_pid(p: *mut task_struct) -> *mut pid { (*p).thread_pid }
#[inline] unsafe fn task_pid_type(p: *mut task_struct, kind: pid_type) -> *mut pid {
    if kind == PIDTYPE_PID { task_pid(p) } else { (*(*p).signal).pids[kind.0 as usize] }
}
#[inline] unsafe fn task_pgrp(p: *mut task_struct) -> *mut pid { task_pid_type(p, PIDTYPE_PGID) }
#[inline] unsafe fn task_session(p: *mut task_struct) -> *mut pid { task_pid_type(p, PIDTYPE_SID) }
#[inline] unsafe fn is_global_init(p: *mut task_struct) -> bool { (*p).tgid == 1 }
#[inline] unsafe fn thread_group_leader(p: *mut task_struct) -> bool { (*p).exit_signal >= 0 }
#[inline] unsafe fn same_thread_group(p: *mut task_struct, q: *mut task_struct) -> bool { (*p).signal == (*q).signal }
#[inline] unsafe fn thread_group_empty(p: *mut task_struct) -> bool {
    thread_group_leader(p) && (*p).thread_node.next == addr_of_mut!((*(*p).signal).thread_head)
}
#[inline] unsafe fn task_from_thread(p: *mut list_head) -> *mut task_struct { p.cast::<u8>().sub(offset_of!(task_struct, thread_node)).cast() }
#[inline] unsafe fn task_from_sibling(p: *mut list_head) -> *mut task_struct { p.cast::<u8>().sub(offset_of!(task_struct, sibling)).cast() }
#[inline] unsafe fn task_from_ptrace(p: *mut list_head) -> *mut task_struct { p.cast::<u8>().sub(offset_of!(task_struct, ptrace_entry)).cast() }
#[inline] unsafe fn next_thread(p: *mut task_struct) -> *mut task_struct {
    let next = b::rust_exit_list_next(addr_of_mut!((*p).thread_node));
    if next == addr_of_mut!((*(*p).signal).thread_head) { (*p).group_leader } else { task_from_thread(next) }
}
#[inline] unsafe fn pid_first(p: *mut pid, kind: pid_type) -> *mut hlist_node {
    b::rust_exit_rcu_list_check(false);
    b::rust_exit_hlist_first(addr_of_mut!((*p).tasks[kind.0 as usize]))
}
#[inline] unsafe fn pid_has_task(p: *mut pid, kind: pid_type) -> bool { !read_volatile(addr_of!((*p).tasks[kind.0 as usize].first)).is_null() }
#[inline] unsafe fn task_from_pid_link(p: *mut hlist_node, kind: pid_type) -> *mut task_struct {
    p.cast::<u8>().sub(offset_of!(task_struct, pid_links) + kind.0 as usize * size_of::<hlist_node>()).cast()
}
#[inline] unsafe fn list_empty(p: *const list_head) -> bool { read_volatile(addr_of!((*p).next)) == p.cast_mut() }
unsafe fn list_init(p: *mut list_head) { write_volatile(addr_of_mut!((*p).next), p); write_volatile(addr_of_mut!((*p).prev), p); }
unsafe fn list_add_valid(new: *mut list_head, prev: *mut list_head, next: *mut list_head) -> bool {
    #[cfg(CONFIG_LIST_HARDENED)] {
        #[cfg(not(CONFIG_DEBUG_LIST))] {
            if (*next).prev == prev && (*prev).next == next && new != prev && new != next { return true; }
            b::rust_exit_list_add_report(new, prev, next);
            return false;
        }
        #[cfg(CONFIG_DEBUG_LIST)] { return b::rust_exit_list_add_report(new, prev, next); }
    }
    #[cfg(not(CONFIG_LIST_HARDENED))] true
}
unsafe fn list_del_valid(entry: *mut list_head) -> bool {
    #[cfg(CONFIG_LIST_HARDENED)] {
        #[cfg(not(CONFIG_DEBUG_LIST))] {
            if (*(*entry).prev).next == entry && (*(*entry).next).prev == entry { return true; }
            b::rust_exit_list_del_report(entry);
            return false;
        }
        #[cfg(CONFIG_DEBUG_LIST)] { return b::rust_exit_list_del_report(entry); }
    }
    #[cfg(not(CONFIG_LIST_HARDENED))] true
}
unsafe fn list_del_entry(entry: *mut list_head) {
    if !list_del_valid(entry) { return; }
    let prev = (*entry).prev;
    let next = (*entry).next;
    (*next).prev = prev;
    write_volatile(addr_of_mut!((*prev).next), next);
}
unsafe fn list_del_rcu(entry: *mut list_head) { list_del_entry(entry); (*entry).prev = RUST_EXIT_LIST_POISON2 as *mut list_head; }
unsafe fn list_del_init(entry: *mut list_head) { list_del_entry(entry); list_init(entry); }
unsafe fn list_add(new: *mut list_head, head: *mut list_head) {
    let next = (*head).next;
    if !list_add_valid(new, head, next) { return; }
    (*next).prev = new; (*new).next = next; (*new).prev = head;
    write_volatile(addr_of_mut!((*head).next), new);
}
unsafe fn list_splice_tail_init(list: *mut list_head, head: *mut list_head) {
    if list_empty(list) { return; }
    let first = (*list).next;
    let last = (*list).prev;
    let prev = (*head).prev;
    (*first).prev = prev; (*prev).next = first; (*last).next = head; (*head).prev = last;
    list_init(list);
}
unsafe fn ptrace_reparented(p: *mut task_struct) -> bool { !same_thread_group((*p).real_parent, (*p).parent) }
unsafe fn ptrace_unlink(p: *mut task_struct) { if (*p).ptrace != 0 { b::__ptrace_unlink(p); } }
unsafe fn ptrace_release_task(p: *mut task_struct) {
    b::rust_exit_bug_on(!list_empty(addr_of!((*p).ptraced)));
    ptrace_unlink(p);
    b::rust_exit_bug_on(!list_empty(addr_of!((*p).ptrace_entry)));
}
unsafe fn get_task_struct(p: *mut task_struct) { b::rust_exit_ref_inc(addr_of_mut!((*p).usage)); }
unsafe fn put_task_struct(p: *mut task_struct) {
    if b::rust_exit_ref_dec(addr_of_mut!((*p).usage)) {
        b::call_rcu(addr_of_mut!((*p).rcu), Some(b::__put_task_struct_rcu_cb));
    }
}
unsafe fn task_ucounts(p: *mut task_struct) -> *mut ucounts {
    b::rust_exit_rcu_lock();
    let cred = b::rust_exit_real_cred(p);
    let ret = (*cred).ucounts;
    b::rust_exit_rcu_unlock();
    ret
}
unsafe fn task_wait_uid(p: *mut task_struct) -> uid_t {
    b::rust_exit_rcu_lock();
    let cred = b::rust_exit_real_cred(p);
    let uid = (*cred).uid;
    b::rust_exit_rcu_unlock();
    #[cfg(CONFIG_USER_NS)] { b::from_kuid_munged(b::rust_exit_current_user_ns(), uid) }
    #[cfg(not(CONFIG_USER_NS))] {
        #[cfg(CONFIG_MULTIUSER)] let value = uid.val;
        #[cfg(not(CONFIG_MULTIUSER))] let value: uid_t = 0;
        if value == uid_t::MAX { b::overflowuid as uid_t } else { value }
    }
}
unsafe fn task_cputime(p: *mut task_struct, utime: *mut u64, stime: *mut u64) {
    #[cfg(CONFIG_VIRT_CPU_ACCOUNTING_GEN)] { b::task_cputime(p, utime, stime); }
    #[cfg(not(CONFIG_VIRT_CPU_ACCOUNTING_GEN))] { *utime = (*p).utime; *stime = (*p).stime; }
}
unsafe fn task_gtime(p: *mut task_struct) -> u64 {
    #[cfg(CONFIG_VIRT_CPU_ACCOUNTING_GEN)] { b::task_gtime(p) }
    #[cfg(not(CONFIG_VIRT_CPU_ACCOUNTING_GEN))] { (*p).gtime }
}
unsafe fn task_io_get_inblock(p: *mut task_struct) -> c_ulong {
    #[cfg(CONFIG_TASK_IO_ACCOUNTING)] { ((*p).ioac.read_bytes >> 9) as c_ulong }
    #[cfg(not(CONFIG_TASK_IO_ACCOUNTING))] { 0 }
}
unsafe fn task_io_get_oublock(p: *mut task_struct) -> c_ulong {
    #[cfg(CONFIG_TASK_IO_ACCOUNTING)] { ((*p).ioac.write_bytes >> 9) as c_ulong }
    #[cfg(not(CONFIG_TASK_IO_ACCOUNTING))] { 0 }
}
unsafe fn task_io_accounting_add(dst: *mut task_io_accounting, src: *mut task_io_accounting) {
    #[cfg(CONFIG_TASK_XACCT)] {
        (*dst).rchar = (*dst).rchar.wrapping_add((*src).rchar);
        (*dst).wchar = (*dst).wchar.wrapping_add((*src).wchar);
        (*dst).syscr = (*dst).syscr.wrapping_add((*src).syscr);
        (*dst).syscw = (*dst).syscw.wrapping_add((*src).syscw);
    }
    #[cfg(CONFIG_TASK_IO_ACCOUNTING)] {
        (*dst).read_bytes = (*dst).read_bytes.wrapping_add((*src).read_bytes);
        (*dst).write_bytes = (*dst).write_bytes.wrapping_add((*src).write_bytes);
        (*dst).cancelled_write_bytes = (*dst).cancelled_write_bytes.wrapping_add((*src).cancelled_write_bytes);
    }
}
unsafe fn setmax_mm_hiwater_rss(maxrss: *mut c_ulong, mm: *mut mm_struct) {
    let counters = b::rust_exit_mm_rss_stat(mm);
    let mut rss: c_ulong = 0;
    for i in [MM_FILEPAGES, MM_ANONPAGES, MM_SHMEMPAGES] {
        let counter = counters.add(i as usize);
        #[cfg(CONFIG_SMP)] let count = core::cmp::max(read_volatile(addr_of!((*counter).count)), 0);
        #[cfg(not(CONFIG_SMP))] let count = (*counter).count;
        rss = rss.wrapping_add(count as c_ulong);
    }
    let hiwater = core::cmp::max(*b::rust_exit_mm_hiwater_rss(mm), rss);
    if *maxrss < hiwater { *maxrss = hiwater; }
}
unsafe fn signal_pending(p: *mut task_struct) -> bool {
    b::rust_exit_test_tsk_flag(p, RUST_EXIT_TIF_NOTIFY_SIGNAL) || b::rust_exit_test_tsk_flag(p, RUST_EXIT_TIF_SIGPENDING)
}
unsafe fn task_exit_status(p: *mut task_struct) -> c_int {
    if (*(*p).signal).flags & SIGNAL_GROUP_EXIT != 0 { (*(*p).signal).group_exit_code } else { (*p).exit_code }
}
unsafe fn put_page(page: *mut page) {
    let info = read_volatile(b::rust_exit_page_compound_info(page));
    let head = if cfg!(CONFIG_HUGETLB_PAGE_OPTIMIZE_VMEMMAP) && size_of::<page>().is_power_of_two() {
        (page as c_ulong) & ((info & 1).wrapping_sub(1) | info)
    } else if info & 1 != 0 { info - 1 } else { page as c_ulong };
    let folio = head as *mut folio;
    let page_type = b::rust_exit_page_type_read(folio.cast());
    if page_type >> 24 == RUST_EXIT_PGTY_SLAB || page_type >> 24 == RUST_EXIT_PGTY_LARGE_KMALLOC { return; }
    let page = folio.cast::<page>();
    b::rust_exit_page_ref_bug(page, b::rust_exit_atomic_read(b::rust_exit_page_refcount(page)) == 0);
    let ret = b::rust_exit_atomic_dec(b::rust_exit_page_refcount(page));
    if b::rust_exit_page_ref_trace_active() { b::rust_exit_page_ref_trace(page, ret); }
    if ret { b::__folio_put(folio); }
}
#[cfg(CONFIG_DEBUG_STACK_USAGE)]
unsafe fn end_of_stack(p: *mut task_struct) -> *mut c_ulong {
    #[cfg(CONFIG_THREAD_INFO_IN_TASK)] let base = (*p).stack.cast::<u8>();
    #[cfg(not(CONFIG_THREAD_INFO_IN_TASK))] let base = b::rust_exit_task_thread_info(p).cast::<u8>();
    #[cfg(CONFIG_STACK_GROWSUP)] { base.add(RUST_EXIT_THREAD_SIZE as usize).cast::<c_ulong>().sub(1) }
    #[cfg(all(not(CONFIG_STACK_GROWSUP), CONFIG_THREAD_INFO_IN_TASK))] { base.cast() }
    #[cfg(all(not(CONFIG_STACK_GROWSUP), not(CONFIG_THREAD_INFO_IN_TASK)))] { base.add(size_of::<thread_info>()).cast() }
}
// Keep the scheduler/IRQ trace callsite in the owning Rust body.
#[inline(always)]
unsafe fn local_irq_disable() {
    #[cfg(CONFIG_TRACE_IRQFLAGS)] let was_disabled = b::rust_exit_raw_irqs_disabled();
    b::rust_exit_raw_irq_disable();
    #[cfg(CONFIG_TRACE_IRQFLAGS)] if !was_disabled { b::trace_hardirqs_off(); }
}
#[inline(always)]
unsafe fn local_irq_enable() {
    #[cfg(CONFIG_TRACE_IRQFLAGS)] b::trace_hardirqs_on();
    b::rust_exit_raw_irq_enable();
}
#[inline(always)]
unsafe fn preempt_disable() {
    #[cfg(CONFIG_PREEMPT_COUNT)] {
        #[cfg(any(CONFIG_DEBUG_PREEMPT, CONFIG_TRACE_PREEMPT_TOGGLE))] b::preempt_count_add(1);
        #[cfg(not(any(CONFIG_DEBUG_PREEMPT, CONFIG_TRACE_PREEMPT_TOGGLE)))] b::rust_exit_preempt_add_raw(1);
    }
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}
