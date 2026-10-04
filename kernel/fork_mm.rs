// SPDX-License-Identifier: GPL-2.0-only
// Rust implementation of kernel/fork.c:566-808 and :1034-1826.
// Included in fork.rs with `use b::*;`. All pointee types are generated from
// the original headers; the small C field-address accessors avoid relying on
// bindgen's names for anonymous/randomized aggregates. They contain no policy.
// SAFETY: These entry points retain the original kernel locking/reference
// contracts. Newly copied task/mm objects are private until copy_process
// publishes them; concurrent fields retain the original RCU/atomic primitives.

#[no_mangle]
pub unsafe extern "C" fn dup_mm_exe_file(mm: *mut mm_struct, oldmm: *mut mm_struct) {
    let exe = get_mm_exe_file(oldmm);
    rust_fork_mm_exe_init_pointer(mm, exe);
    // oldmm must already have denied write access to its executable.
    if !exe.is_null() && rust_fork_mm_exe_deny_write_access(exe) != 0 {
        rust_fork_mm_warn_dup_exe();
    }
}

#[cfg_attr(not(CONFIG_MMU), allow(unused_variables))]
unsafe fn mm_alloc_pgd(mm: *mut mm_struct) -> i32 {
    #[cfg(CONFIG_MMU)]
    {
        let pgd = rust_fork_mm_pgd_alloc(mm);
        *rust_fork_mm_mm_pgd(mm) = pgd;
        if pgd.is_null() {
            return -(ENOMEM as i32);
        }
    }
    0
}

#[cfg_attr(not(CONFIG_MMU), allow(unused_variables))]
unsafe fn mm_free_pgd(mm: *mut mm_struct) {
    #[cfg(CONFIG_MMU)]
    rust_fork_mm_pgd_free(mm, *rust_fork_mm_mm_pgd(mm));
}

#[cfg_attr(not(CONFIG_MM_ID), allow(unused_variables))]
unsafe fn mm_alloc_id(mm: *mut mm_struct) -> i32 {
    #[cfg(CONFIG_MM_ID)]
    {
        let id = ida_alloc_range(
            rust_fork_mm_ida(),
            MM_ID_MIN as u32,
            MM_ID_MAX as u32,
            rust_fork_mm_gfp_kernel(),
        );
        if id < 0 {
            return id;
        }
        *rust_fork_mm_mm_mm_id(mm) = id as _;
    }
    0
}

#[cfg_attr(not(CONFIG_MM_ID), allow(unused_variables))]
unsafe fn mm_free_id(mm: *mut mm_struct) {
    #[cfg(CONFIG_MM_ID)]
    {
        let id = *rust_fork_mm_mm_mm_id(mm);
        *rust_fork_mm_mm_mm_id(mm) = MM_ID_DUMMY as _;
        if id == MM_ID_DUMMY as _ {
            return;
        }
        if rust_fork_mm_warn_bad_id(id < MM_ID_MIN as _ || id > MM_ID_MAX as _) {
            return;
        }
        ida_free(rust_fork_mm_ida(), id as u32);
    }
}

unsafe fn check_mm(mm: *mut mm_struct) {
    // The diagnostic strings and native counter-count assertion are Rust-owned
    // in fork_storage.rs.
    let counters = rust_fork_mm_mm_rss_stat(mm).cast::<percpu_counter>();
    for i in 0..NR_MM_COUNTERS as usize {
        let sum = rust_fork_mm_counter_sum(counters.add(i));
        if sum != 0 {
            rust_fork_mm_bad_rss(mm, resident_page_types.0[i], sum);
        }
    }
    if rust_fork_mm_pgtables_bytes(mm) != 0 {
        rust_fork_mm_bad_pgtables(rust_fork_mm_pgtables_bytes(mm));
    }
    #[cfg(all(CONFIG_TRANSPARENT_HUGEPAGE, not(CONFIG_SPLIT_PMD_PTLOCKS)))]
    rust_fork_mm_bug_huge_pte(mm);
}

unsafe fn allocate_mm() -> *mut mm_struct {
    rust_fork_mm_cache_alloc(mm_cachep, rust_fork_mm_gfp_kernel()).cast()
}

unsafe fn free_mm(mm: *mut mm_struct) {
    rust_fork_mm_cache_free(mm_cachep, mm.cast());
}

unsafe extern "C" fn do_check_lazy_tlb(arg: *mut kernel::ffi::c_void) {
    let mm = arg.cast::<mm_struct>();
    rust_fork_mm_warn_check_lazy(*rust_fork_mm_task_active_mm(current_task()) == mm);
}

unsafe extern "C" fn do_shoot_lazy_tlb(arg: *mut kernel::ffi::c_void) {
    let mm = arg.cast::<mm_struct>();
    let task = current_task();
    if *rust_fork_mm_task_active_mm(task) == mm {
        rust_fork_mm_warn_shoot_lazy(!(*rust_fork_mm_task_mm(task)).is_null());
        let initial = core::ptr::addr_of_mut!(b::init_mm);
        *rust_fork_mm_task_active_mm(task) = initial;
        rust_fork_mm_switch_mm(mm, initial, task);
    }
}

#[cfg_attr(not(CONFIG_MMU_LAZY_TLB_SHOOTDOWN), allow(unused_variables))]
unsafe fn cleanup_lazy_tlbs(mm: *mut mm_struct) {
    #[cfg(CONFIG_MMU_LAZY_TLB_SHOOTDOWN)]
    {
        // Wait for every target CPU to relinquish its uncounted lazy-mm use
        // before freeing pgd/context/mm. Callback lifetime ends at return.
        rust_fork_mm_on_each_cpu_mask(
            rust_fork_mm_cpumask(mm),
            Some(do_shoot_lazy_tlb),
            mm.cast(),
            true,
        );
        #[cfg(CONFIG_DEBUG_VM_SHOOT_LAZIES)]
        rust_fork_mm_on_each_cpu(Some(do_check_lazy_tlb), mm.cast(), true);
    }
}

#[no_mangle]
pub unsafe extern "C" fn __mmdrop(mm: *mut mm_struct) {
    rust_fork_mm_bug_init_mm(mm == core::ptr::addr_of_mut!(b::init_mm));
    rust_fork_mm_warn_drop_current_mm(mm == *rust_fork_mm_task_mm(current_task()));
    cleanup_lazy_tlbs(mm);
    rust_fork_mm_warn_drop_active_mm(mm == *rust_fork_mm_task_active_mm(current_task()));
    fork_mm_destroy_sched(mm);
    mm_free_pgd(mm);
    mm_free_id(mm);
    fork_destroy_context(mm);
    rust_fork_mm_notifier_destroy(mm);
    check_mm(mm);
    rust_fork_mm_pasid_drop(mm);
    fork_mm_destroy_cid(mm);
    rust_fork_mm_counter_destroy_many(rust_fork_mm_mm_rss_stat(mm).cast(), NR_MM_COUNTERS as u32);
    free_mm(mm);
}

unsafe extern "C" fn mmdrop_async_fn(work: *mut work_struct) {
    __mmdrop(rust_fork_mm_work_to_mm(work));
}

unsafe fn mmdrop_async(mm: *mut mm_struct) {
    if rust_fork_mm_atomic_dec_and_test(rust_fork_mm_mm_mm_count(mm)) {
        let work = rust_fork_mm_mm_async_put_work(mm);
        rust_fork_mm_init_drop_work(work, Some(mmdrop_async_fn));
        rust_fork_mm_schedule_work(work);
    }
}

unsafe fn free_signal_struct(sig: *mut signal_struct) {
    rust_fork_mm_taskstats_tgid_free(sig);
    rust_fork_mm_autogroup_exit(sig);
    let oom_mm = *rust_fork_mm_sig_oom_mm(sig);
    // x86 pgd_dtor cannot run in softirq context; the final drop is queued.
    if !oom_mm.is_null() {
        mmdrop_async(oom_mm);
    }
    rust_fork_mm_cache_free(signal_cachep, sig.cast());
}

unsafe fn put_signal_struct(sig: *mut signal_struct) {
    if rust_fork_mm_refcount_dec_and_test(rust_fork_mm_sig_sigcnt(sig)) {
        free_signal_struct(sig);
    }
}

#[no_mangle]
pub unsafe extern "C" fn __put_task_struct(tsk: *mut task_struct) {
    rust_fork_mm_warn_task_exit(*rust_fork_mm_task_exit_state(tsk) == 0);
    rust_fork_mm_warn_task_usage(rust_fork_mm_refcount_read(rust_fork_mm_task_usage(tsk)) != 0);
    rust_fork_mm_warn_task_current(tsk == current_task());
    rust_fork_mm_unwind_task_free(tsk);
    rust_fork_mm_io_uring_free(tsk);
    rust_fork_mm_cgroup_task_free(tsk);
    rust_fork_mm_task_numa_free(tsk, true);
    rust_fork_mm_security_task_free(tsk);
    exit_creds(tsk);
    rust_fork_mm_delayacct_tsk_free(tsk);
    put_signal_struct(*rust_fork_mm_task_signal(tsk));
    rust_fork_mm_sched_core_free(tsk);
    free_task(tsk);
}

#[no_mangle]
pub unsafe extern "C" fn __put_task_struct_rcu_cb(rhp: *mut callback_head) {
    __put_task_struct(rust_fork_mm_rcu_to_task(rhp));
}

// mmlist_lock's cache-line alignment and the boot-option registration are C
// section/initializer primitives; all parsing and masked updates are Rust.
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn rust_fork_coredump_filter_setup(s: *mut kernel::ffi::c_char) -> i32 {
    let filter = addr_of_mut!(coredump_filter);
    if rust_fork_mm_kstrtoul(s, 0, filter) != 0 {
        return 0;
    }
    *filter <<= MMF_DUMP_FILTER_SHIFT;
    *filter &= RUST_FORK_MMF_DUMP_FILTER_MASK as kernel::ffi::c_ulong;
    1
}

#[cfg_attr(not(CONFIG_AIO), allow(unused_variables))]
unsafe fn mm_init_aio(mm: *mut mm_struct) {
    #[cfg(CONFIG_AIO)]
    {
        rust_fork_mm_init_ioctx_lock(rust_fork_mm_mm_ioctx_lock(mm));
        *rust_fork_mm_mm_ioctx_table(mm) = core::ptr::null_mut();
    }
}

#[cfg_attr(not(CONFIG_MEMCG), allow(unused_variables))]
unsafe fn mm_clear_owner(mm: *mut mm_struct, task: *mut task_struct) {
    #[cfg(CONFIG_MEMCG)]
    if *rust_fork_mm_mm_owner(mm) == task {
        rust_fork_mm_owner_write_once(mm, core::ptr::null_mut());
    }
}

#[cfg_attr(not(CONFIG_MEMCG), allow(unused_variables))]
unsafe fn mm_init_owner(mm: *mut mm_struct, task: *mut task_struct) {
    #[cfg(CONFIG_MEMCG)]
    {
        *rust_fork_mm_mm_owner(mm) = task;
    }
}

#[cfg_attr(not(CONFIG_UPROBES), allow(unused_variables))]
unsafe fn mm_init_uprobes_state(mm: *mut mm_struct) {
    #[cfg(CONFIG_UPROBES)]
    {
        *rust_fork_mm_xol_area(mm) = core::ptr::null_mut();
    }
}

unsafe fn mmap_init_lock(mm: *mut mm_struct) {
    rust_fork_mm_init_mmap_lock(rust_fork_mm_mm_mmap_lock(mm));
    rust_fork_mm_lock_seqcount_init(mm);
    #[cfg(CONFIG_PER_VMA_LOCK)]
    rust_fork_mm_rcuwait_init(rust_fork_mm_mm_vma_writer_wait(mm));
}

unsafe fn mm_init(mm: *mut mm_struct, task: *mut task_struct) -> *mut mm_struct {
    rust_fork_mm_mt_init(rust_fork_mm_mm_mm_mt(mm), MM_MT_FLAGS as u32);
    rust_fork_mm_mt_set_external_lock(rust_fork_mm_mm_mm_mt(mm), rust_fork_mm_mm_mmap_lock(mm));
    rust_fork_mm_atomic_set(rust_fork_mm_mm_mm_users(mm), 1);
    rust_fork_mm_atomic_set(rust_fork_mm_mm_mm_count(mm), 1);
    rust_fork_mm_init_write_protect_seq(rust_fork_mm_mm_write_protect_seq(mm));
    mmap_init_lock(mm);
    rust_fork_mm_init_list_head(rust_fork_mm_mm_mmlist(mm));
    rust_fork_mm_pgtables_bytes_init(mm);
    *rust_fork_mm_mm_map_count(mm) = 0;
    *rust_fork_mm_mm_locked_vm(mm) = 0;
    rust_fork_mm_atomic64_set(rust_fork_mm_mm_pinned_vm(mm), 0);
    core::ptr::write_bytes(rust_fork_mm_mm_rss_stat(mm), 0, 1);
    rust_fork_mm_init_page_table_lock(rust_fork_mm_mm_page_table_lock(mm));
    rust_fork_mm_init_arg_lock(rust_fork_mm_mm_arg_lock(mm));
    rust_fork_mm_init_cpumask(mm);
    mm_init_aio(mm);
    mm_init_owner(mm, task);
    rust_fork_mm_pasid_init(mm);
    rust_fork_mm_exe_init_pointer(mm, core::ptr::null_mut());
    rust_fork_mm_notifier_init(mm);
    rust_fork_mm_init_tlb_flush_pending(mm);
    #[cfg(all(CONFIG_TRANSPARENT_HUGEPAGE, not(CONFIG_SPLIT_PMD_PTLOCKS)))]
    {
        *rust_fork_mm_mm_pmd_huge_pte(mm) = core::ptr::null_mut();
    }
    mm_init_uprobes_state(mm);
    rust_fork_mm_hugetlb_count_init(mm);
    rust_fork_mm_futex_mm_init(mm);

    rust_fork_mm_flags_clear_all(mm);
    let oldmm = *rust_fork_mm_task_mm(current_task());
    if !oldmm.is_null() {
        let flags = rust_fork_mm_flags_get(oldmm);
        rust_fork_mm_flags_overwrite(mm, rust_fork_mm_flags_initial(flags));
        *rust_fork_mm_mm_def_flags(mm) =
            *rust_fork_mm_mm_def_flags(oldmm) & RUST_FORK_VM_INIT_DEF_MASK as kernel::ffi::c_ulong;
    } else {
        rust_fork_mm_flags_overwrite(mm, coredump_filter);
        *rust_fork_mm_mm_def_flags(mm) = 0;
    }

    // A failed stage never owns that stage's resource. The completed-stage
    // count gives exactly the original fail_* fall-through cleanup order.
    let completed = if mm_alloc_pgd(mm) != 0 {
        0
    } else if mm_alloc_id(mm) != 0 {
        1
    } else if fork_init_new_context(task, mm) != 0 {
        2
    } else if fork_mm_alloc_cid(mm, task) != 0 {
        3
    } else if fork_mm_alloc_sched(mm) != 0 {
        4
    } else if rust_fork_mm_counter_init_many(
        rust_fork_mm_mm_rss_stat(mm).cast(),
        0,
        rust_fork_mm_gfp_kernel_account(),
        NR_MM_COUNTERS as u32,
    ) != 0
    {
        5
    } else {
        rust_fork_mm_lru_gen_init(mm);
        return mm;
    };
    if completed >= 5 {
        fork_mm_destroy_sched(mm);
    }
    if completed >= 4 {
        fork_mm_destroy_cid(mm);
    }
    if completed >= 3 {
        fork_destroy_context(mm);
    }
    if completed >= 2 {
        mm_free_id(mm);
    }
    if completed >= 1 {
        mm_free_pgd(mm);
    }
    free_mm(mm);
    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn mm_alloc() -> *mut mm_struct {
    let mm = allocate_mm();
    if mm.is_null() {
        return core::ptr::null_mut();
    }
    core::ptr::write_bytes(mm, 0, 1);
    mm_init(mm, current_task())
}

unsafe fn __mmput(mm: *mut mm_struct) {
    #[cfg(CONFIG_DEBUG_VM)]
    rust_fork_mm_bug_mm_users(rust_fork_mm_atomic_read(rust_fork_mm_mm_mm_users(mm)) != 0);
    rust_fork_mm_uprobe_clear_state(mm);
    rust_fork_mm_exit_aio(mm);
    rust_fork_mm_ksm_exit(mm);
    rust_fork_mm_khugepaged_exit(mm); // Must precede exit_mmap.
    exit_mmap(mm);
    rust_fork_mm_put_huge_zero_folio(mm);
    set_mm_exe_file(mm, core::ptr::null_mut());
    let link = rust_fork_mm_mm_mmlist(mm);
    if !rust_fork_mm_list_empty(link) {
        let lock = rust_fork_mm_mmlist_lock();
        rust_fork_mm_spin_lock(lock);
        rust_fork_mm_list_del(link);
        rust_fork_mm_spin_unlock(lock);
    }
    let binfmt = *rust_fork_mm_mm_binfmt(mm);
    if !binfmt.is_null() {
        rust_fork_mm_module_put(*rust_fork_mm_binfmt_module(binfmt));
    }
    rust_fork_mm_lru_gen_del(mm);
    rust_fork_mm_futex_hash_free(mm);
    fork_mmdrop(mm);
}

#[no_mangle]
pub unsafe extern "C" fn mmput(mm: *mut mm_struct) {
    rust_fork_mm_might_sleep();
    if rust_fork_mm_atomic_dec_and_test(rust_fork_mm_mm_mm_users(mm)) {
        __mmput(mm);
    }
}

#[cfg(any(CONFIG_MMU, CONFIG_FUTEX_PRIVATE_HASH))]
unsafe extern "C" fn mmput_async_fn(work: *mut work_struct) {
    __mmput(rust_fork_mm_work_to_mm(work));
}

#[cfg(any(CONFIG_MMU, CONFIG_FUTEX_PRIVATE_HASH))]
#[no_mangle]
pub unsafe extern "C" fn mmput_async(mm: *mut mm_struct) {
    if rust_fork_mm_atomic_dec_and_test(rust_fork_mm_mm_mm_users(mm)) {
        let work = rust_fork_mm_mm_async_put_work(mm);
        rust_fork_mm_init_put_work(work, Some(mmput_async_fn));
        rust_fork_mm_schedule_work(work);
    }
}

#[no_mangle]
pub unsafe extern "C" fn set_mm_exe_file(mm: *mut mm_struct, new_exe: *mut file) -> i32 {
    // mmput/exec exclusion permits this raw dereference without an RCU read
    // section. Get the new reference before publishing or dropping the old one.
    let old_exe = rust_fork_mm_exe_dereference_raw(mm);
    if !new_exe.is_null() {
        if rust_fork_mm_exe_deny_write_access(new_exe) != 0 {
            return -(EACCES as i32);
        }
        rust_fork_mm_get_file(new_exe);
    }
    rust_fork_mm_exe_assign_pointer(mm, new_exe);
    if !old_exe.is_null() {
        rust_fork_mm_exe_allow_write_access(old_exe);
        fput(old_exe);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn replace_mm_exe_file(mm: *mut mm_struct, new_exe: *mut file) -> i32 {
    let mut old_exe = get_mm_exe_file(mm);
    if !old_exe.is_null() {
        let mut iterator: vma_iterator = core::mem::zeroed();
        iterator.mas.tree = rust_fork_mm_mm_mm_mt(mm);
        iterator.mas.index = 0;
        iterator.mas.node = core::ptr::null_mut();
        iterator.mas.status = ma_start;
        let mut busy = false;
        rust_fork_mm_mmap_read_lock(mm);
        loop {
            let vma = rust_fork_mm_vma_next(&mut iterator);
            if vma.is_null() {
                break;
            }
            let mapped = *rust_fork_mm_vma_vm_file(vma);
            if mapped.is_null() {
                continue;
            }
            if rust_fork_mm_path_equal(
                rust_fork_mm_file_f_path(mapped),
                rust_fork_mm_file_f_path(old_exe),
            ) {
                busy = true;
                break;
            }
        }
        rust_fork_mm_mmap_read_unlock(mm);
        fput(old_exe);
        if busy {
            return -(EBUSY as i32);
        }
    }
    if rust_fork_mm_exe_deny_write_access(new_exe) != 0 {
        return -(EACCES as i32);
    }
    rust_fork_mm_get_file(new_exe);
    rust_fork_mm_mmap_write_lock(mm);
    old_exe = rust_fork_mm_exe_dereference_raw(mm);
    rust_fork_mm_exe_assign_pointer(mm, new_exe);
    rust_fork_mm_mmap_write_unlock(mm);
    if !old_exe.is_null() {
        rust_fork_mm_exe_allow_write_access(old_exe);
        fput(old_exe);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn get_mm_exe_file(mm: *mut mm_struct) -> *mut file {
    rust_fork_mm_rcu_read_lock();
    let exe = get_file_rcu(rust_fork_mm_mm_exe_file(mm));
    rust_fork_mm_rcu_read_unlock();
    exe
}

#[no_mangle]
pub unsafe extern "C" fn get_task_exe_file(task: *mut task_struct) -> *mut file {
    if *rust_fork_mm_task_flags(task) & PF_KTHREAD as u32 != 0 {
        return core::ptr::null_mut();
    }
    rust_fork_mm_task_lock(task);
    let mm = *rust_fork_mm_task_mm(task);
    let exe = if mm.is_null() {
        core::ptr::null_mut()
    } else {
        get_mm_exe_file(mm)
    };
    rust_fork_mm_task_unlock(task);
    exe
}

#[no_mangle]
pub unsafe extern "C" fn get_task_mm(task: *mut task_struct) -> *mut mm_struct {
    if *rust_fork_mm_task_flags(task) & PF_KTHREAD as u32 != 0 {
        return core::ptr::null_mut();
    }
    rust_fork_mm_task_lock(task);
    let mm = *rust_fork_mm_task_mm(task);
    if !mm.is_null() {
        rust_fork_mm_mmget(mm);
    }
    rust_fork_mm_task_unlock(task);
    mm
}

unsafe fn may_access_mm(mm: *mut mm_struct, task: *mut task_struct, mode: u32) -> bool {
    if mm == *rust_fork_mm_task_mm(current_task()) {
        return true;
    }
    if ptrace_may_access(task, mode) {
        return true;
    }
    if mode & PTRACE_MODE_READ as u32 != 0 && rust_fork_mm_perfmon_capable() {
        return true;
    }
    false
}

#[no_mangle]
pub unsafe extern "C" fn mm_access(task: *mut task_struct, mode: u32) -> *mut mm_struct {
    let sig = *rust_fork_mm_task_signal(task);
    let lock = rust_fork_mm_sig_exec_update_lock(sig);
    let error = down_read_killable(lock);
    if error != 0 {
        return rust_fork_mm_err_ptr(error as kernel::ffi::c_long).cast();
    }
    let mut mm = get_task_mm(task);
    if mm.is_null() {
        mm = rust_fork_mm_err_ptr(-(ESRCH as kernel::ffi::c_long)).cast();
    } else if !may_access_mm(mm, task, mode) {
        mmput(mm);
        mm = rust_fork_mm_err_ptr(-(EACCES as kernel::ffi::c_long)).cast();
    }
    up_read(lock);
    mm
}

unsafe fn complete_vfork_done(tsk: *mut task_struct) {
    rust_fork_mm_task_lock(tsk);
    let vfork = *rust_fork_mm_task_vfork_done(tsk);
    if !vfork.is_null() {
        *rust_fork_mm_task_vfork_done(tsk) = core::ptr::null_mut();
        complete(vfork);
    }
    rust_fork_mm_task_unlock(tsk);
}

unsafe fn wait_for_vfork_done(child: *mut task_struct, vfork: *mut completion) -> i32 {
    let state = (TASK_KILLABLE | TASK_FREEZABLE) as u32;
    rust_fork_mm_cgroup_enter_frozen();
    let killed = wait_for_completion_state(vfork, state);
    rust_fork_mm_cgroup_leave_frozen(false);
    if killed != 0 {
        rust_fork_mm_task_lock(child);
        *rust_fork_mm_task_vfork_done(child) = core::ptr::null_mut();
        rust_fork_mm_task_unlock(child);
    }
    fork_put_task_struct(child);
    killed
}

unsafe fn mm_release(tsk: *mut task_struct, mm: *mut mm_struct) {
    rust_fork_mm_uprobe_free_utask(tsk);
    rust_fork_mm_deactivate_mm(tsk, mm);
    let clear_tid = *rust_fork_mm_task_clear_child_tid(tsk);
    if !clear_tid.is_null() {
        if rust_fork_mm_atomic_read(rust_fork_mm_mm_mm_users(mm)) > 1 {
            // The original deliberately ignores put_user failure, then wakes.
            rust_fork_mm_put_user_zero(clear_tid);
            rust_fork_mm_do_futex(
                clear_tid.cast(),
                FUTEX_WAKE as i32,
                1,
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                0,
                0,
            );
        }
        *rust_fork_mm_task_clear_child_tid(tsk) = core::ptr::null_mut();
    }
    if !(*rust_fork_mm_task_vfork_done(tsk)).is_null() {
        complete_vfork_done(tsk);
    }
}

#[no_mangle]
pub unsafe extern "C" fn mm_exit_exec_release(tsk: *mut task_struct, mm: *mut mm_struct) {
    rust_fork_mm_futex_exit_exec_release(tsk);
    mm_release(tsk, mm);
}

unsafe fn dup_mm(tsk: *mut task_struct, oldmm: *mut mm_struct) -> *mut mm_struct {
    let mm = allocate_mm();
    if mm.is_null() {
        return core::ptr::null_mut();
    }
    core::ptr::copy_nonoverlapping(oldmm, mm, 1);
    if mm_init(mm, tsk).is_null() {
        return core::ptr::null_mut();
    }
    rust_fork_mm_uprobe_start_dup_mmap();
    let error = dup_mmap(mm, oldmm);
    if error == 0 {
        rust_fork_mm_uprobe_end_dup_mmap();
        *rust_fork_mm_mm_hiwater_rss(mm) = rust_fork_mm_get_rss(mm);
        *rust_fork_mm_mm_hiwater_vm(mm) = *rust_fork_mm_mm_total_vm(mm);
        let binfmt = *rust_fork_mm_mm_binfmt(mm);
        if binfmt.is_null() || rust_fork_mm_try_module_get(*rust_fork_mm_binfmt_module(binfmt)) {
            return mm;
        }
    }
    // No module reference was acquired on either failure path. Null binfmt
    // before mmput so cleanup does not release an unowned module reference.
    *rust_fork_mm_mm_binfmt(mm) = core::ptr::null_mut();
    mm_init_owner(mm, core::ptr::null_mut());
    mmput(mm);
    if error != 0 {
        rust_fork_mm_uprobe_end_dup_mmap();
    }
    core::ptr::null_mut()
}

unsafe fn copy_mm(clone_flags: u64, tsk: *mut task_struct) -> i32 {
    *rust_fork_mm_task_min_flt(tsk) = 0;
    *rust_fork_mm_task_maj_flt(tsk) = 0;
    *rust_fork_mm_task_nvcsw(tsk) = 0;
    *rust_fork_mm_task_nivcsw(tsk) = 0;
    #[cfg(CONFIG_DETECT_HUNG_TASK)]
    {
        *rust_fork_mm_task_last_switch_count(tsk) =
            (*rust_fork_mm_task_nvcsw(tsk)).wrapping_add(*rust_fork_mm_task_nivcsw(tsk));
        *rust_fork_mm_task_last_switch_time(tsk) = 0;
    }
    *rust_fork_mm_task_mm(tsk) = core::ptr::null_mut();
    *rust_fork_mm_task_active_mm(tsk) = core::ptr::null_mut();
    let oldmm = *rust_fork_mm_task_mm(current_task());
    if oldmm.is_null() {
        return 0;
    }
    let mm = if clone_flags & CLONE_VM as u64 != 0 {
        rust_fork_mm_mmget(oldmm);
        oldmm
    } else {
        let copied = dup_mm(tsk, *rust_fork_mm_task_mm(current_task()));
        if copied.is_null() {
            return -(ENOMEM as i32);
        }
        copied
    };
    *rust_fork_mm_task_mm(tsk) = mm;
    *rust_fork_mm_task_active_mm(tsk) = mm;
    0
}

unsafe fn copy_exec_state(clone_flags: u64, tsk: *mut task_struct) -> i32 {
    if clone_flags & CLONE_VM as u64 != 0 {
        let state = rust_fork_mm_exec_dereference_protected(current_task());
        rust_fork_mm_refcount_inc(rust_fork_mm_exec_count(state));
        rust_fork_mm_exec_assign_pointer(tsk, state);
        return 0;
    }
    task_exec_state_copy(tsk)
}

unsafe fn copy_fs(clone_flags: u64, tsk: *mut task_struct, umh: bool) -> i32 {
    let current = current_task();
    let fs = if umh {
        if clone_flags & (CLONE_NEWNS as u64 | CLONE_FS as u64) != 0 {
            return -(EINVAL as i32);
        }
        let nsproxy = *rust_fork_mm_task_nsproxy(current);
        if *rust_fork_mm_nsproxy_mnt_ns(nsproxy) != core::ptr::addr_of_mut!(b::init_mnt_ns) {
            return -(EINVAL as i32);
        }
        b::userspace_init_fs
    } else {
        let inherited = *rust_fork_mm_task_fs(current);
        rust_fork_mm_warn_fs(inherited != *rust_fork_mm_task_real_fs(current));
        inherited
    };
    if clone_flags & CLONE_FS as u64 != 0 {
        let lock = rust_fork_mm_fs_seq(fs);
        rust_fork_mm_read_seqlock_excl(lock);
        if *rust_fork_mm_fs_in_exec(fs) != 0 {
            rust_fork_mm_read_sequnlock_excl(lock);
            return -(EAGAIN as i32);
        }
        let users = rust_fork_mm_fs_users(fs);
        *users = (*users).wrapping_add(1);
        rust_fork_mm_read_sequnlock_excl(lock);
        return 0;
    }
    let copied = copy_fs_struct(fs);
    *rust_fork_mm_task_fs(tsk) = copied;
    *rust_fork_mm_task_real_fs(tsk) = copied;
    if copied.is_null() {
        return -(ENOMEM as i32);
    }
    0
}

unsafe fn copy_files(clone_flags: u64, tsk: *mut task_struct, no_files: i32) -> i32 {
    let old = *rust_fork_mm_task_files(current_task());
    if old.is_null() {
        return 0;
    }
    if no_files != 0 {
        *rust_fork_mm_task_files(tsk) = core::ptr::null_mut();
        return 0;
    }
    if clone_flags & CLONE_FILES as u64 != 0 {
        rust_fork_mm_atomic_inc(rust_fork_mm_files_count(old));
        return 0;
    }
    let copied = dup_fd(old, core::ptr::null_mut());
    if rust_fork_mm_is_err(copied.cast()) {
        return rust_fork_mm_ptr_err(copied.cast()) as i32;
    }
    *rust_fork_mm_task_files(tsk) = copied;
    0
}

unsafe fn copy_sighand(clone_flags: u64, tsk: *mut task_struct) -> i32 {
    let parent_sig = *rust_fork_mm_task_sighand(current_task());
    if clone_flags & CLONE_SIGHAND as u64 != 0 {
        rust_fork_mm_refcount_inc(rust_fork_mm_sh_count(parent_sig));
        return 0;
    }
    let sig = rust_fork_mm_cache_alloc(sighand_cachep, rust_fork_mm_gfp_kernel())
        .cast::<sighand_struct>();
    rust_fork_mm_sighand_init_pointer(tsk, sig);
    if sig.is_null() {
        return -(ENOMEM as i32);
    }
    rust_fork_mm_refcount_set(rust_fork_mm_sh_count(sig), 1);
    let lock = rust_fork_mm_sh_siglock(parent_sig);
    rust_fork_mm_spin_lock_irq(lock);
    core::ptr::copy_nonoverlapping(
        rust_fork_mm_sh_action(parent_sig),
        rust_fork_mm_sh_action(sig),
        1,
    );
    rust_fork_mm_spin_unlock_irq(lock);
    if clone_flags & CLONE_CLEAR_SIGHAND as u64 != 0 {
        flush_signal_handlers(tsk, 0);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn __cleanup_sighand(sighand: *mut sighand_struct) {
    if rust_fork_mm_refcount_dec_and_test(rust_fork_mm_sh_count(sighand)) {
        rust_fork_mm_signalfd_cleanup(sighand);
        // SLAB_TYPESAFE_BY_RCU permits freeing without an RCU grace period.
        rust_fork_mm_cache_free(sighand_cachep, sighand.cast());
    }
}

unsafe fn posix_cpu_timers_init_group(sig: *mut signal_struct) {
    let limit = rust_fork_mm_cpu_limit_read_once(sig);
    rust_fork_mm_posix_cputimers_group_init(rust_fork_mm_sig_posix_cputimers(sig), limit);
}

unsafe fn copy_signal(clone_flags: u64, tsk: *mut task_struct) -> i32 {
    if clone_flags & CLONE_THREAD as u64 != 0 {
        return 0;
    }
    let sig =
        rust_fork_mm_cache_zalloc(signal_cachep, rust_fork_mm_gfp_kernel()).cast::<signal_struct>();
    *rust_fork_mm_task_signal(tsk) = sig;
    if sig.is_null() {
        return -(ENOMEM as i32);
    }
    *rust_fork_mm_sig_nr_threads(sig) = 1;
    *rust_fork_mm_sig_quick_threads(sig) = 1;
    rust_fork_mm_atomic_set(rust_fork_mm_sig_live(sig), 1);
    rust_fork_mm_refcount_set(rust_fork_mm_sig_sigcnt(sig), 1);

    // Both links are initialized directly, equivalent to the two original
    // LIST_HEAD_INIT compound literals (no premature list_add on zero links).
    let head = rust_fork_mm_sig_thread_head(sig);
    let node = rust_fork_mm_task_thread_node(tsk);
    (*head).next = node;
    (*head).prev = node;
    (*node).next = head;
    (*node).prev = head;

    rust_fork_mm_init_wait_chldexit(rust_fork_mm_sig_wait_chldexit(sig));
    *rust_fork_mm_sig_curr_target(sig) = tsk;
    rust_fork_mm_init_sigpending(rust_fork_mm_sig_shared_pending(sig));
    rust_fork_mm_init_hlist_head(rust_fork_mm_sig_multiprocess(sig));
    rust_fork_mm_init_stats_lock(rust_fork_mm_sig_stats_lock(sig));
    rust_fork_mm_prev_cputime_init(rust_fork_mm_sig_prev_cputime(sig));
    #[cfg(CONFIG_POSIX_TIMERS)]
    {
        rust_fork_mm_init_hlist_head(rust_fork_mm_sig_posix_timers(sig));
        rust_fork_mm_init_hlist_head(rust_fork_mm_sig_ignored_posix_timers(sig));
        rust_fork_mm_setup_real_timer(rust_fork_mm_sig_real_timer(sig));
    }
    let current = current_task();
    let leader = *rust_fork_mm_task_group_leader(current);
    let parent_sig = *rust_fork_mm_task_signal(current);
    rust_fork_mm_task_lock(leader);
    core::ptr::copy_nonoverlapping(
        rust_fork_mm_sig_rlim(parent_sig),
        rust_fork_mm_sig_rlim(sig),
        1,
    );
    rust_fork_mm_task_unlock(leader);
    posix_cpu_timers_init_group(sig);
    rust_fork_mm_tty_audit_fork(sig);
    rust_fork_mm_autogroup_fork(sig);
    #[cfg(CONFIG_CGROUPS)]
    rust_fork_mm_init_cgroup_rwsem(rust_fork_mm_sig_cgroup_threadgroup_rwsem(sig));
    *rust_fork_mm_sig_oom_score_adj(sig) = *rust_fork_mm_sig_oom_score_adj(parent_sig);
    *rust_fork_mm_sig_oom_score_adj_min(sig) = *rust_fork_mm_sig_oom_score_adj_min(parent_sig);
    rust_fork_mm_init_cred_guard_mutex(rust_fork_mm_sig_cred_guard_mutex(sig));
    rust_fork_mm_init_exec_update_lock(rust_fork_mm_sig_exec_update_lock(sig));
    0
}

#[cfg_attr(not(CONFIG_SECCOMP), allow(unused_variables))]
unsafe fn copy_seccomp(task: *mut task_struct) {
    #[cfg(CONFIG_SECCOMP)]
    {
        let current = current_task();
        let sighand = *rust_fork_mm_task_sighand(current);
        rust_fork_mm_assert_spin_locked(rust_fork_mm_sh_siglock(sighand));
        rust_fork_mm_get_seccomp_filter(current);
        core::ptr::copy_nonoverlapping(
            rust_fork_mm_task_seccomp(current),
            rust_fork_mm_task_seccomp(task),
            1,
        );
        // Recheck after acquiring sighand: no_new_privs/seccomp may have changed
        // since task/thread flags were copied, and both must remain in sync.
        if rust_fork_mm_task_no_new_privs(current) {
            rust_fork_mm_task_set_no_new_privs(task);
        }
        if rust_fork_mm_seccomp_mode(task) != SECCOMP_MODE_DISABLED as i32 {
            rust_fork_mm_set_seccomp_syscall_work(task);
        }
    }
}
