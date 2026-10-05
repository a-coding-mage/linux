// SPDX-License-Identifier: GPL-2.0-only
// core.c debugging, task dumps, SYSRQ normalization and immutable priority data.
#[inline(always)]
unsafe fn get_preempt_disable_ip(p: *mut task_struct) -> c_ulong {
    #[cfg(CONFIG_DEBUG_PREEMPT)]
    // SAFETY: The caller keeps p live while the configured debug field is
    // read; this is stored metadata, not a new caller-PC capture implementation.
    unsafe {
        (*p).preempt_disable_ip
    }
    #[cfg(not(CONFIG_DEBUG_PREEMPT))]
    {
        let _ = p;
        0
    }
}
#[inline(never)]
unsafe fn __schedule_bug(prev: *mut task_struct) {
    // SAFETY: The scheduler keeps prev/current live through diagnostics;
    // original native panic/taint behavior and caller-IP admission gates remain.
    unsafe {
        let ip = get_preempt_disable_ip(lupos_core_current());
        if oops_in_progress != 0 {
            return;
        }
        _printk(
            b"\x013BUG: scheduling while atomic: %s/%d/0x%08x\n\0"
                .as_ptr()
                .cast(),
            (*prev).comm.as_ptr(),
            (*prev).pid,
            lupos_core_preempt_count(),
        );
        lupos_core_header_debug_show_held_locks(prev);
        lupos_core_header_print_modules();
        if lupos_core_irqs_disabled() {
            lupos_core_header_print_irqtrace_events(prev);
        }
        if cfg!(CONFIG_DEBUG_PREEMPT) {
            lupos_core_pr_err(b"Preemption disabled at:\0".as_ptr().cast());
            lupos_core_header_print_ip_sym(b"\x013\0".as_ptr().cast(), ip);
        }
        check_panic_on_warn(b"scheduling while atomic\0".as_ptr().cast());
        lupos_core_header_dump_stack();
        add_taint(LUPOS_CORE_TAINT_WARN, LUPOS_CORE_LOCKDEP_STILL_OK);
    }
}
#[inline(always)]
unsafe fn schedule_debug(prev: *mut task_struct, _preempt: bool) {
    // SAFETY: The scheduling caller keeps prev/current live through native
    // state/stack diagnostics. Profiling caller-PC and instrumentation contracts
    // remain unqualified despite these explicit source scopes.
    unsafe {
        #[cfg(CONFIG_SCHED_STACK_END_CHECK)]
        {
            if lupos_core_task_stack_end_corrupted(prev) {
                panic(
                    b"corrupted stack end detected inside scheduler\n\0"
                        .as_ptr()
                        .cast(),
                );
            }
            if lupos_core_task_scs_end_corrupted(prev) {
                panic(
                    b"corrupted shadow stack detected inside scheduler\n\0"
                        .as_ptr()
                        .cast(),
                );
            }
        }
        #[cfg(CONFIG_DEBUG_ATOMIC_SLEEP)]
        if !_preempt
            && lupos_core_read_once_uint(addr_of!((*prev).__state)) != 0
            && (*prev).non_block_count != 0
        {
            _printk(
                b"\x013BUG: scheduling in a non-blocking section: %s/%d/%i\n\0"
                    .as_ptr()
                    .cast(),
                (*prev).comm.as_ptr(),
                (*prev).pid,
                (*prev).non_block_count,
            );
            lupos_core_header_dump_stack();
            add_taint(LUPOS_CORE_TAINT_WARN, LUPOS_CORE_LOCKDEP_STILL_OK);
        }
        if lupos_core_in_atomic_preempt_off() {
            __schedule_bug(prev);
            lupos_core_preempt_count_set(LUPOS_CORE_PREEMPT_DISABLED as c_int);
        }
        lupos_core_rcu_sleep_check();
        lupos_core_warn_once_site_6121(lupos_core_ct_state() == LUPOS_CORE_CT_STATE_USER);
        // Native profiling boundary must preserve __builtin_return_address(0).
        lupos_core_profile_sched_caller();
        lupos_core_schedstat_sched_count_inc(lupos_core_this_rq());
    }
}
#[no_mangle]
pub unsafe extern "C" fn sched_show_task(p: *mut task_struct) {
    // SAFETY: The caller pins p; the acquired native stack reference spans
    // stack inspection, and RCU protects parent lookup until the final stack put.
    unsafe {
        if lupos_core_try_get_task_stack(p).is_null() {
            return;
        }
        _printk(
            b"\x016task:%-15.15s state:%c\0".as_ptr().cast(),
            (*p).comm.as_ptr(),
            lupos_core_task_state_to_char(p) as c_int,
        );
        if lupos_core_task_is_running(p) {
            _printk(b"\x01c  running task    \0".as_ptr().cast());
        }
        let free = lupos_core_stack_not_used(p);
        let mut ppid: c_int = 0;
        lupos_core_rcu_read_lock();
        if lupos_core_pid_alive(p) {
            ppid = lupos_core_task_pid_nr(lupos_core_rcu_real_parent(p));
        }
        lupos_core_rcu_read_unlock();
        _printk(
            b"\x01c stack:%-5lu pid:%-5d tgid:%-5d ppid:%-6d task_flags:0x%04x flags:0x%08lx\n\0"
                .as_ptr()
                .cast(),
            free,
            lupos_core_task_pid_nr(p),
            lupos_core_task_tgid_nr(p),
            ppid,
            (*p).flags,
            lupos_core_read_task_thread_flags(p),
        );
        print_worker_info(b"\x016\0".as_ptr().cast(), p);
        lupos_core_header_print_stop_info(b"\x016\0".as_ptr().cast(), p);
        lupos_core_header_print_scx_info(b"\x016\0".as_ptr().cast(), p);
        show_stack(p, null_mut(), b"\x016\0".as_ptr().cast());
        lupos_core_put_task_stack(p);
    }
}
unsafe fn state_filter_match(filter: c_ulong, p: *mut task_struct) -> bool {
    // SAFETY: The caller keeps p live; native READ_ONCE supplies its task
    // state snapshot for scalar filter comparisons.
    unsafe {
        let state = lupos_core_read_once_uint(addr_of!((*p).__state));
        if filter == 0 {
            return true;
        }
        if state as c_ulong & filter == 0 {
            return false;
        }
        !(filter == LUPOS_CORE_TASK_UNINTERRUPTIBLE as c_ulong && state & LUPOS_CORE_TASK_NOLOAD != 0)
    }
}
#[no_mangle]
pub unsafe extern "C" fn show_state_filter(filter: c_uint) {
    // SAFETY: RCU protects native process/thread traversal and task dumps;
    // watchdog/debug output retains the original diagnostic-context contract.
    unsafe {
        lupos_core_rcu_read_lock();
        let mut g = lupos_core_next_task(addr_of_mut!(init_task));
        while g != addr_of_mut!(init_task) {
            let mut p = lupos_core_first_thread(g);
            while !p.is_null() {
                lupos_core_header_touch_nmi_watchdog();
                lupos_core_header_touch_all_softlockup_watchdogs();
                if state_filter_match(filter as c_ulong, p) {
                    sched_show_task(p);
                }
                p = lupos_core_next_thread_in_group(g, p);
            }
            g = lupos_core_next_task(g);
        }
        if filter == 0 {
            sysrq_sched_debug_show();
        }
        lupos_core_rcu_read_unlock();
        if filter == 0 {
            lupos_core_header_debug_show_all_locks();
        }
    }
}
#[cfg(CONFIG_DEBUG_ATOMIC_SLEEP)]
#[no_mangle]
pub unsafe extern "C" fn __might_sleep(file: *const c_char, line: c_int) {
    // SAFETY: The native diagnostic caller supplies a live source filename;
    // current/task-state metadata remain live through warning and resched checks.
    unsafe {
        let current = lupos_core_current();
        let state = lupos_core_get_current_state();
        lupos_core_warn_invalid_sleep_state(
            state != LUPOS_CORE_TASK_RUNNING && (*current).task_state_change != 0,
            state,
            (*current).task_state_change,
        );
        __might_resched(file, line, 0);
    }
}
#[cfg(CONFIG_DEBUG_ATOMIC_SLEEP)]
unsafe fn print_preempt_disable_ip(offset: c_int, ip: c_ulong) {
    // SAFETY: The native diagnostic context permits preempt-state reads
    // and symbol output; ip is printed as an address, not dereferenced here.
    unsafe {
        if !cfg!(CONFIG_DEBUG_PREEMPT) || lupos_core_preempt_count() == offset {
            return;
        }
        lupos_core_pr_err(b"Preemption disabled at:\0".as_ptr().cast());
        lupos_core_header_print_ip_sym(b"\x013\0".as_ptr().cast(), ip);
    }
}
#[cfg(CONFIG_DEBUG_ATOMIC_SLEEP)]
unsafe fn resched_offsets_ok(offsets: c_uint) -> bool {
    // SAFETY: The native diagnostic context permits current CPU preempt
    // and RCU-depth snapshots; scalar offset arithmetic retains native widths.
    unsafe {
        let mut nested = lupos_core_preempt_count() as c_uint;
        nested = nested.wrapping_add(
            (lupos_core_rcu_preempt_depth() as c_uint).wrapping_shl(LUPOS_CORE_MIGHT_RESCHED_RCU_SHIFT),
        );
        nested == offsets
    }
}
#[cfg(CONFIG_DEBUG_ATOMIC_SLEEP)]
#[no_mangle]
pub unsafe extern "C" fn __might_resched(file: *const c_char, line: c_int, offsets: c_uint) {
    // SAFETY: The native diagnostic caller supplies a live filename and
    // current state; permanent rate-limit storage and native stack/lock reporting
    // retain original synchronization and instrumentation requirements.
    unsafe {
        lupos_core_rcu_sleep_check();
        let current = lupos_core_current();
        if (resched_offsets_ok(offsets)
            && !lupos_core_irqs_disabled()
            && !lupos_core_is_idle_task(current)
            && (*current).non_block_count == 0)
            || system_state == LUPOS_CORE_SYSTEM_BOOTING
            || system_state > LUPOS_CORE_SYSTEM_RUNNING
            || oops_in_progress != 0
        {
            return;
        }
        let stamp = lupos_core_might_resched_prev_jiffy();
        if lupos_core_time_before(
            lupos_core_jiffies(),
            (*stamp).wrapping_add(LUPOS_CORE_HZ as c_ulong),
        ) && *stamp != 0
        {
            return;
        }
        *stamp = lupos_core_jiffies();
        let ip = get_preempt_disable_ip(current);
        _printk(
            b"\x013BUG: sleeping function called from invalid context at %s:%d\n\0"
                .as_ptr()
                .cast(),
            file,
            line,
        );
        _printk(
            b"\x013in_atomic(): %d, irqs_disabled(): %d, non_block: %d, pid: %d, name: %s\n\0"
                .as_ptr()
                .cast(),
            lupos_core_in_atomic() as c_int,
            lupos_core_irqs_disabled() as c_int,
            (*current).non_block_count,
            (*current).pid,
            (*current).comm.as_ptr(),
        );
        _printk(
            b"\x013preempt_count: %x, expected: %x\n\0".as_ptr().cast(),
            lupos_core_preempt_count(),
            offsets & LUPOS_CORE_MIGHT_RESCHED_PREEMPT_MASK,
        );
        if cfg!(CONFIG_PREEMPT_RCU) {
            _printk(
                b"\x013RCU nest depth: %d, expected: %u\n\0".as_ptr().cast(),
                lupos_core_rcu_preempt_depth(),
                offsets >> LUPOS_CORE_MIGHT_RESCHED_RCU_SHIFT,
            );
        }
        if lupos_core_task_stack_end_corrupted(current) {
            _printk(
                b"\x010Thread overran stack, or stack corrupted\n\0"
                    .as_ptr()
                    .cast(),
            );
        }
        lupos_core_header_debug_show_held_locks(current);
        if lupos_core_irqs_disabled() {
            lupos_core_header_print_irqtrace_events(current);
        }
        print_preempt_disable_ip(
            (offsets & LUPOS_CORE_MIGHT_RESCHED_PREEMPT_MASK) as c_int,
            ip,
        );
        lupos_core_header_dump_stack();
        add_taint(LUPOS_CORE_TAINT_WARN, LUPOS_CORE_LOCKDEP_STILL_OK);
    }
}
#[cfg(CONFIG_DEBUG_ATOMIC_SLEEP)]
#[no_mangle]
pub unsafe extern "C" fn __cant_sleep(file: *const c_char, line: c_int) {
    // SAFETY: The native diagnostic caller supplies a live filename and
    // current state; checks and permanent rate-limit storage retain native rules.
    unsafe {
        if lupos_core_irqs_disabled() || !cfg!(CONFIG_PREEMPT_COUNT) || lupos_core_preempt_count() != 0
        {
            return;
        }
        let stamp = lupos_core_cant_sleep_prev_jiffy();
        if lupos_core_time_before(
            lupos_core_jiffies(),
            (*stamp).wrapping_add(LUPOS_CORE_HZ as c_ulong),
        ) && *stamp != 0
        {
            return;
        }
        *stamp = lupos_core_jiffies();
        let current = lupos_core_current();
        _printk(
            b"\x013BUG: assuming atomic context at %s:%d\n\0"
                .as_ptr()
                .cast(),
            file,
            line,
        );
        _printk(
            b"\x013in_atomic(): %d, irqs_disabled(): %d, pid: %d, name: %s\n\0"
                .as_ptr()
                .cast(),
            lupos_core_in_atomic() as c_int,
            lupos_core_irqs_disabled() as c_int,
            (*current).pid,
            (*current).comm.as_ptr(),
        );
        lupos_core_header_debug_show_held_locks(current);
        lupos_core_header_dump_stack();
        add_taint(LUPOS_CORE_TAINT_WARN, LUPOS_CORE_LOCKDEP_STILL_OK);
    }
}
#[cfg(all(CONFIG_DEBUG_ATOMIC_SLEEP, CONFIG_SMP))]
#[no_mangle]
pub unsafe extern "C" fn __cant_migrate(file: *const c_char, line: c_int) {
    // SAFETY: The native diagnostic caller supplies a live filename/current
    // task; migration/IRQ checks and rate-limited reporting preserve native context.
    unsafe {
        let current = lupos_core_current();
        if lupos_core_irqs_disabled()
            || lupos_core_is_migration_disabled(current)
            || !cfg!(CONFIG_PREEMPT_COUNT)
            || lupos_core_preempt_count() != 0
        {
            return;
        }
        let stamp = lupos_core_cant_migrate_prev_jiffy();
        if lupos_core_time_before(
            lupos_core_jiffies(),
            (*stamp).wrapping_add(LUPOS_CORE_HZ as c_ulong),
        ) && *stamp != 0
        {
            return;
        }
        *stamp = lupos_core_jiffies();
        _printk(
            b"\x013BUG: assuming non migratable context at %s:%d\n\0"
                .as_ptr()
                .cast(),
            file,
            line,
        );
        _printk(
            b"\x013in_atomic(): %d, irqs_disabled(): %d, migration_disabled() %u pid: %d, name: %s\n\0"
                .as_ptr()
                .cast(),
            lupos_core_in_atomic() as c_int,
            lupos_core_irqs_disabled() as c_int,
            lupos_core_is_migration_disabled(current) as c_uint,
            (*current).pid,
            (*current).comm.as_ptr(),
        );
        lupos_core_header_debug_show_held_locks(current);
        lupos_core_header_dump_stack();
        add_taint(LUPOS_CORE_TAINT_WARN, LUPOS_CORE_LOCKDEP_STILL_OK);
    }
}
#[cfg(CONFIG_MAGIC_SYSRQ)]
#[no_mangle]
pub unsafe extern "C" fn normalize_rt_tasks() {
    // SAFETY: The SysRq path acquires native tasklist read protection for
    // traversal; each task remains live during priority/statistics normalization.
    unsafe {
        let mut attr = MaybeUninit::<sched_attr>::zeroed();
        let attr = attr.as_mut_ptr();
        (*attr).sched_policy = LUPOS_CORE_SCHED_NORMAL;
        lupos_core_read_lock(addr_of_mut!(tasklist_lock));
        let mut g = lupos_core_next_task(addr_of_mut!(init_task));
        while g != addr_of_mut!(init_task) {
            let mut p = lupos_core_first_thread(g);
            while !p.is_null() {
                if (*p).flags & LUPOS_CORE_PF_KTHREAD == 0 {
                    (*p).se.exec_start = 0;
                    lupos_core_schedstat_wait_start_set(p, 0);
                    lupos_core_schedstat_sleep_start_set(p, 0);
                    lupos_core_schedstat_block_start_set(p, 0);
                    if !lupos_core_rt_or_dl_task(p) {
                        if lupos_core_task_nice(p) < 0 {
                            set_user_nice(p, 0);
                        }
                    } else {
                        __sched_setscheduler(p, attr, false, false);
                    }
                }
                p = lupos_core_next_thread_in_group(g, p);
            }
            g = lupos_core_next_task(g);
        }
        lupos_core_read_unlock(addr_of_mut!(tasklist_lock));
    }
}
#[cfg(CONFIG_KGDB_KDB)]
#[no_mangle]
pub unsafe extern "C" fn curr_task(cpu: c_int) -> *mut task_struct {
    // SAFETY: The KDB caller has stopped the whole system; cpu is valid and
    // its permanent runqueue's current task cannot change during this snapshot.
    unsafe {
        lupos_core_cpu_curr(cpu)
    }
}
#[no_mangle]
pub unsafe extern "C" fn dump_cpu_task(cpu: c_int) {
    // SAFETY: cpu is valid for the native diagnostic interface; current
    // IRQ registers or remote backtrace/task-stack lifetime contracts protect output.
    unsafe {
        if lupos_core_in_hardirq() && cpu == lupos_core_smp_processor_id() {
            let regs = lupos_core_get_irq_regs();
            if !regs.is_null() {
                show_regs(regs);
                return;
            }
        }
        if lupos_core_header_trigger_single_cpu_backtrace(cpu) {
            return;
        }
        _printk(b"\x016Task dump for CPU %d:\n\0".as_ptr().cast(), cpu);
        sched_show_task(lupos_core_cpu_curr(cpu));
    }
}
#[no_mangle]
pub static sched_prio_to_weight: [c_int; 40] = [
    88761, 71755, 56483, 46273, 36291, 29154, 23254, 18705, 14949, 11916, 9548, 7620, 6100, 4904,
    3906, 3121, 2501, 1991, 1586, 1277, 1024, 820, 655, 526, 423, 335, 272, 215, 172, 137, 110, 87,
    70, 56, 45, 36, 29, 23, 18, 15,
];
#[no_mangle]
pub static sched_prio_to_wmult: [u32; 40] = [
    48388, 59856, 76040, 92818, 118348, 147320, 184698, 229616, 287308, 360437, 449829, 563644,
    704093, 875809, 1099582, 1376151, 1717300, 2157191, 2708050, 3363326, 4194304, 5237765,
    6557202, 8165337, 10153587, 12820798, 15790321, 19976592, 24970740, 31350126, 39045157,
    49367440, 61356676, 76695844, 95443717, 119304647, 148102320, 186737708, 238609294, 286331153,
];
#[no_mangle]
pub unsafe extern "C" fn call_trace_sched_update_nr_running(rq: *mut rq, count: c_int) {
    // SAFETY: The caller keeps rq live in the native scheduler tracepoint
    // context; the count argument has the original int boundary.
    unsafe {
        lupos_core_trace_sched_update_nr_running(rq, count);
    }
}
