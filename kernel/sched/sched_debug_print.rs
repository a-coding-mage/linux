// SPDX-License-Identifier: GPL-2.0-only
// Output and traversal owners from the existing debug.c oracle.

#[cfg(CONFIG_CGROUP_SCHED)]
unsafe fn task_group_path(tg: *mut task_group, path: *mut c_char, plen: c_int) {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    if b::lupos_debug_autogroup_path(tg, path, plen) != 0 { return; }
    b::lupos_debug_cgroup_path((*tg).css.cgroup, path, plen);

    }
}

#[cfg(CONFIG_CGROUP_SCHED)]
unsafe fn with_task_group_path(tg: *mut task_group, emit: impl FnOnce(*const c_char)) {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    if b::lupos_debug_group_path_trylock() {
        let path = b::lupos_debug_group_path_buffer();
        task_group_path(tg, path, b::LUPOS_DEBUG_PATH_MAX as c_int);
        emit(path);
        b::lupos_debug_group_path_unlock();
    } else {
        let mut buf = [0 as c_char; 128];
        task_group_path(tg, buf.as_mut_ptr(), 125);
        // Original strcpy(bufend - 1, "...") includes the terminating NUL.
        buf[124] = b'.' as c_char;
        buf[125] = b'.' as c_char;
        buf[126] = b'.' as c_char;
        buf[127] = 0;
        emit(buf.as_ptr());
    }

    }
}

#[cfg(CONFIG_FAIR_GROUP_SCHED)]
unsafe fn print_cfs_group_stats(m: *mut seq_file, cpu: c_int, tg: *mut task_group) {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    let se = b::lupos_debug_tg_se(tg, cpu);
    if se.is_null() { return; }
    macro_rules! pn { ($field:ident) => {
        ns!(m, "  .%-30s: %lld.%06ld\n", cstr!(concat!("se->", stringify!($field))), (*se).$field);
    }; }
    pn!(exec_start); pn!(vruntime); pn!(sum_exec_runtime);
    if b::lupos_debug_schedstat_enabled() {
        #[cfg(CONFIG_SCHEDSTATS)]
        let stats = b::lupos_debug_stats_from_se(se);
        macro_rules! sn { ($field:ident) => {
            ns!(m, "  .%-30s: %lld.%06ld\n", cstr!(stringify!($field)), stat_value!(stats, $field));
        }; }
        sn!(wait_start); sn!(sleep_start); sn!(block_start); sn!(sleep_max);
        sn!(block_max); sn!(exec_max); sn!(slice_max); sn!(wait_max); sn!(wait_sum);
        out!(m, "  .%-30s: %lld\n", cstr!("wait_count"), stat_value!(stats, wait_count) as i64);
    }
    out!(m, "  .%-30s: %lld\n", cstr!("se->load.weight"), (*se).load.weight as i64);
    out!(m, "  .%-30s: %lld\n", cstr!("se->avg.load_avg"), (*se).avg.load_avg as i64);
    out!(m, "  .%-30s: %lld\n", cstr!("se->avg.util_avg"), (*se).avg.util_avg as i64);
    out!(m, "  .%-30s: %lld\n", cstr!("se->avg.runnable_avg"), (*se).avg.runnable_avg as i64);

    }
}

unsafe fn print_task(m: *mut seq_file, rq: *mut rq, p: *mut task_struct) {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    if b::lupos_debug_task_current(rq, p) != 0 { out!(m, ">R"); }
    else { out!(m, " %c", b::lupos_debug_task_state_to_char(p) as c_int); }
    out!(m, " %15s %5d %10ld %9Ld.%06ld   %c   %9Ld.%06ld %c %9Ld.%06ld %9Ld.%06ld %9Ld   %5d ",
        (*p).comm.as_ptr(), b::lupos_debug_task_pid_nr(p), (*p).se.h_load.weight as c_long,
        nsec_high((*p).se.vruntime), nsec_low((*p).se.vruntime),
        if b::lupos_debug_entity_eligible(addr_of_mut!((*rq).cfs), addr_of_mut!((*p).se)) != 0 { b'E' as c_int } else { b'N' as c_int },
        nsec_high((*p).se.deadline), nsec_low((*p).se.deadline),
        if b::lupos_debug_custom_slice(addr_of!((*p).se)) { b'S' as c_int } else { b' ' as c_int },
        nsec_high((*p).se.slice), nsec_low((*p).se.slice),
        nsec_high((*p).se.sum_exec_runtime), nsec_low((*p).se.sum_exec_runtime),
        (*p).nvcsw.wrapping_add((*p).nivcsw) as i64, (*p).prio);
    macro_rules! stat_or_zero { ($field:ident) => {{
        if b::lupos_debug_schedstat_enabled() { stat_value!(addr_of!((*p).stats), $field) as u64 } else { 0u64 }
    }}; }
    let wait = stat_or_zero!(wait_sum);
    let sleep = stat_or_zero!(sum_sleep_runtime);
    let block = stat_or_zero!(sum_block_runtime);
    out!(m, "%9lld.%06ld %9lld.%06ld %9lld.%06ld",
        nsec_high(wait), nsec_low(wait), nsec_high(sleep), nsec_low(sleep), nsec_high(block), nsec_low(block));
    #[cfg(CONFIG_NUMA_BALANCING)]
    out!(m, "   %d      %d", b::lupos_debug_task_node(p), b::lupos_debug_task_numa_group_id(p));
    #[cfg(CONFIG_CGROUP_SCHED)]
    with_task_group_path(b::lupos_debug_task_group(p), |path| { out!(m, "        %s", path); });
    out!(m, "\n");

    }
}

unsafe fn print_rq(m: *mut seq_file, rq: *mut rq, rq_cpu: c_int) {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    #[cfg(CONFIG_NUMA_BALANCING)] macro_rules! numa_columns { () => { "  node   group-id" }; }
    #[cfg(not(CONFIG_NUMA_BALANCING))] macro_rules! numa_columns { () => { "" }; }
    #[cfg(CONFIG_CGROUP_SCHED)] macro_rules! cgroup_columns { () => { "  group-path" }; }
    #[cfg(not(CONFIG_CGROUP_SCHED))] macro_rules! cgroup_columns { () => { "" }; }
    #[cfg(CONFIG_NUMA_BALANCING)] macro_rules! numa_separator { () => { "--------------" }; }
    #[cfg(not(CONFIG_NUMA_BALANCING))] macro_rules! numa_separator { () => { "" }; }
    #[cfg(CONFIG_CGROUP_SCHED)] macro_rules! cgroup_separator { () => { "--------------" }; }
    #[cfg(not(CONFIG_CGROUP_SCHED))] macro_rules! cgroup_separator { () => { "" }; }
    out!(m, "\n");
    out!(m, "runnable tasks:\n");
    out!(m, concat!(" S            task   PID     weight       vruntime   eligible    ",
        "deadline             slice          sum-exec      switches  ",
        "prio         wait-time        sum-sleep       sum-block",
        numa_columns!(), cgroup_columns!(), "\n"));
    out!(m, concat!("-------------------------------------------------------",
        "------------------------------------------------------",
        "------------------------------------------------------",
        numa_separator!(), cgroup_separator!(), "\n"));
    b::lupos_debug_rcu_read_lock();
    let init = b::lupos_debug_init_task();
    let mut group = b::lupos_debug_next_task(init);
    while group != init {
        let head = b::lupos_debug_thread_head(group);
        b::lupos_debug_thread_check_rcu();
        let mut node = b::lupos_debug_list_next_rcu(head);
        while node != head {
            let p = b::lupos_debug_task_from_thread_node(node);
            if b::lupos_debug_task_cpu(p) == rq_cpu { print_task(m, rq, p); }
            node = b::lupos_debug_list_next_rcu(node);
        }
        group = b::lupos_debug_next_task(group);
    }
    b::lupos_debug_rcu_read_unlock();

    }
}

#[no_mangle]
/// Native scheduler debug entry point.
///
/// # Safety
/// The caller must satisfy the original debug.c entry point's object lifetime,
/// synchronization, IRQ-context and user-buffer contract, where applicable.
pub unsafe extern "C" fn print_cfs_rq(m: *mut seq_file, cpu: c_int, cfs_rq: *mut cfs_rq) {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    let mut left_vruntime = -1i64;
    let mut right_vruntime = -1i64;
    let mut left_deadline = -1i64;
    let rq = b::lupos_debug_cpu_rq(cpu);
    out!(m, "\n");
    #[cfg(CONFIG_FAIR_GROUP_SCHED)]
    with_task_group_path((*cfs_rq).tg, |path| { out!(m, "cfs_rq[%d]:%s\n", cpu, path); });
    #[cfg(not(CONFIG_FAIR_GROUP_SCHED))]
    out!(m, "cfs_rq[%d]:\n", cpu);
    let flags = b::lupos_debug_raw_rq_lock_irqsave(rq);
    let root = b::lupos_debug_pick_root_entity(cfs_rq);
    if !root.is_null() { left_vruntime = (*root).min_vruntime as i64; }
    let first = b::lupos_debug_pick_first_entity(cfs_rq);
    if !first.is_null() { left_deadline = (*first).deadline as i64; }
    let last = b::lupos_debug_pick_last_entity(cfs_rq);
    if !last.is_null() { right_vruntime = (*last).vruntime as i64; }
    let zero_vruntime = (*cfs_rq).zero_vruntime as i64;
    let sum_w_vruntime = (*cfs_rq).sum_w_vruntime;
    let sum_weight = (*cfs_rq).sum_weight;
    let sum_shift = (*cfs_rq).sum_shift;
    let avruntime = b::lupos_debug_avg_vruntime(cfs_rq);
    b::lupos_debug_raw_rq_unlock_irqrestore(rq, flags);
    macro_rules! pn { ($name:literal, $value:expr) => { ns!(m, "  .%-30s: %Ld.%06ld\n", cstr!($name), $value); }; }
    pn!("left_deadline", left_deadline); pn!("left_vruntime", left_vruntime); pn!("zero_vruntime", zero_vruntime);
    out!(m, "  .%-30s: %Ld (%d bits)\n", cstr!("sum_w_vruntime"), sum_w_vruntime, b::lupos_debug_ilog2_abs(sum_w_vruntime));
    out!(m, "  .%-30s: %Lu\n", cstr!("sum_weight"), sum_weight);
    out!(m, "  .%-30s: %u\n", cstr!("sum_shift"), sum_shift);
    pn!("avg_vruntime", avruntime); pn!("right_vruntime", right_vruntime);
    pn!("spread", right_vruntime.wrapping_sub(left_vruntime));
    macro_rules! pi { ($field:ident) => { out!(m, "  .%-30s: %d\n", cstr!(stringify!($field)), (*cfs_rq).$field as c_int); }; }
    pi!(nr_queued); pi!(h_nr_runnable); pi!(h_nr_queued); pi!(h_nr_idle);
    out!(m, "  .%-30s: %ld\n", cstr!("load"), (*cfs_rq).load.weight as c_long);
    out!(m, "  .%-30s: %lu\n", cstr!("load_avg"), (*cfs_rq).avg.load_avg as c_ulong);
    out!(m, "  .%-30s: %lu\n", cstr!("runnable_avg"), (*cfs_rq).avg.runnable_avg as c_ulong);
    out!(m, "  .%-30s: %lu\n", cstr!("util_avg"), (*cfs_rq).avg.util_avg as c_ulong);
    out!(m, "  .%-30s: %u\n", cstr!("util_est"), (*cfs_rq).avg.util_est);
    out!(m, "  .%-30s: %ld\n", cstr!("removed.load_avg"), (*cfs_rq).removed.load_avg as c_long);
    out!(m, "  .%-30s: %ld\n", cstr!("removed.util_avg"), (*cfs_rq).removed.util_avg as c_long);
    out!(m, "  .%-30s: %ld\n", cstr!("removed.runnable_avg"), (*cfs_rq).removed.runnable_avg as c_long);
    #[cfg(CONFIG_FAIR_GROUP_SCHED)] {
        out!(m, "  .%-30s: %lu\n", cstr!("tg_load_avg_contrib"), (*cfs_rq).tg_load_avg_contrib);
        out!(m, "  .%-30s: %ld\n", cstr!("tg_load_avg"), b::lupos_debug_atomic_long_read(addr_of!((*(*cfs_rq).tg).load_avg)));
        out!(m, "  .%-30s: %lu\n", cstr!("h_load"), (*cfs_rq).h_load);
    }
    #[cfg(CONFIG_CFS_BANDWIDTH)] {
        out!(m, "  .%-30s: %d\n", cstr!("throttled"), b::lupos_debug_cfs_throttled(cfs_rq) as c_int);
        pi!(throttle_count);
    }
    #[cfg(CONFIG_FAIR_GROUP_SCHED)] print_cfs_group_stats(m, cpu, (*cfs_rq).tg);

    }
}

#[no_mangle]
/// Native scheduler debug entry point.
///
/// # Safety
/// The caller must satisfy the original debug.c entry point's object lifetime,
/// synchronization, IRQ-context and user-buffer contract, where applicable.
pub unsafe extern "C" fn print_rt_rq(m: *mut seq_file, cpu: c_int, rt_rq: *mut rt_rq) {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    out!(m, "\n");
    #[cfg(CONFIG_RT_GROUP_SCHED)]
    with_task_group_path((*rt_rq).tg, |path| { out!(m, "rt_rq[%d]:%s\n", cpu, path); });
    #[cfg(not(CONFIG_RT_GROUP_SCHED))] out!(m, "rt_rq[%d]:\n", cpu);
    out!(m, "  .%-30s: %lu\n", cstr!("rt_nr_running"), (*rt_rq).rt_nr_running as c_ulong);
    #[cfg(CONFIG_RT_GROUP_SCHED)] {
        out!(m, "  .%-30s: %Ld\n", cstr!("rt_throttled"), (*rt_rq).rt_throttled as i64);
        ns!(m, "  .%-30s: %Ld.%06ld\n", cstr!("rt_time"), (*rt_rq).rt_time);
        ns!(m, "  .%-30s: %Ld.%06ld\n", cstr!("rt_runtime"), (*rt_rq).rt_runtime);
    }

    }
}

#[no_mangle]
/// Native scheduler debug entry point.
///
/// # Safety
/// The caller must satisfy the original debug.c entry point's object lifetime,
/// synchronization, IRQ-context and user-buffer contract, where applicable.
pub unsafe extern "C" fn print_dl_rq(m: *mut seq_file, cpu: c_int, dl_rq: *mut dl_rq) {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    out!(m, "\n"); out!(m, "dl_rq[%d]:\n", cpu);
    out!(m, "  .%-30s: %lu\n", cstr!("dl_nr_running"), (*dl_rq).dl_nr_running as c_ulong);
    let bw = addr_of!((*(*b::lupos_debug_cpu_rq(cpu)).rd).dl_bw);
    out!(m, "  .%-30s: %lld\n", cstr!("dl_bw->bw"), (*bw).bw as i64);
    out!(m, "  .%-30s: %lld\n", cstr!("dl_bw->total_bw"), (*bw).total_bw as i64);

    }
}

unsafe fn print_cpu(m: *mut seq_file, cpu: c_int) {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    let rq = b::lupos_debug_cpu_rq(cpu);
    #[cfg(CONFIG_X86)] {
        let freq = if b::cpu_khz != 0 { b::cpu_khz } else { 1 };
        out!(m, "cpu#%d, %u.%03u MHz\n", cpu, freq / 1000, freq % 1000);
    }
    #[cfg(not(CONFIG_X86))] out!(m, "cpu#%d\n", cpu);
    macro_rules! p { ($field:ident, $size:ident) => {{
        if b::$size == 4 {
            out!(m, "  .%-30s: %d\n", cstr!(stringify!($field)), (*rq).$field as c_int);
        } else {
            out!(m, "  .%-30s: %Ld\n", cstr!(stringify!($field)), (*rq).$field as i64);
        }
    }}; }
    macro_rules! pn { ($field:ident) => { ns!(m, "  .%-30s: %Ld.%06ld\n", cstr!(stringify!($field)), (*rq).$field); }; }
    p!(nr_running, LUPOS_DEBUG_RQ_NR_RUNNING_SIZE);
    p!(nr_switches, LUPOS_DEBUG_RQ_NR_SWITCHES_SIZE);
    p!(nr_uninterruptible, LUPOS_DEBUG_RQ_NR_UNINTERRUPTIBLE_SIZE);
    pn!(next_balance);
    out!(m, "  .%-30s: %ld\n", cstr!("curr->pid"), b::lupos_debug_task_pid_nr(b::lupos_debug_rq_curr(rq)) as c_long);
    pn!(clock); pn!(clock_task);
    out!(m, "  .%-30s: %Ld\n", cstr!("avg_idle"), (*rq).avg_idle as i64);
    out!(m, "  .%-30s: %Ld\n", cstr!("max_idle_balance_cost"), (*rq).max_idle_balance_cost as i64);
    if b::lupos_debug_schedstat_enabled() {
        macro_rules! ps { ($field:ident) => {
            out!(m, "  .%-30s: %d\n", cstr!(stringify!($field)), stat_value!(rq, $field) as c_int);
        }; }
        ps!(yld_count); ps!(sched_count); ps!(sched_goidle); ps!(ttwu_count); ps!(ttwu_local);
    }
    // These are other scheduler owners (fair.c/rt.c/deadline.c), not debug.c fallbacks.
    b::lupos_debug_print_cfs_stats(m, cpu);
    b::lupos_debug_print_rt_stats(m, cpu);
    b::lupos_debug_print_dl_stats(m, cpu);
    print_rq(m, rq, cpu);
    out!(m, "\n");

    }
}

unsafe fn sched_debug_header(m: *mut seq_file) {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    let flags = b::lupos_debug_local_irq_save();
    let ktime = b::lupos_debug_ktime_get_ns();
    let sched_clk = b::lupos_debug_sched_clock();
    let cpu_clk = b::lupos_debug_local_clock();
    b::lupos_debug_local_irq_restore(flags);
    let uts = b::lupos_debug_init_utsname();
    out!(m, "Sched Debug Version: v0.11, %s %.*s\n", (*uts).release.as_ptr(),
        b::lupos_debug_strcspn((*uts).version.as_ptr(), cstr!(" ")) as c_int, (*uts).version.as_ptr());
    ns!(m, "%-40s: %Ld.%06ld\n", cstr!("ktime"), ktime);
    ns!(m, "%-40s: %Ld.%06ld\n", cstr!("sched_clk"), sched_clk);
    ns!(m, "%-40s: %Ld.%06ld\n", cstr!("cpu_clk"), cpu_clk);
    out!(m, "%-40s: %Ld\n", cstr!("jiffies"), b::lupos_debug_jiffies() as i64);
    #[cfg(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK)]
    out!(m, "%-40s: %Ld\n", cstr!("sched_clock_stable()"), b::lupos_debug_sched_clock_stable() as i64);
    out!(m, "\n"); out!(m, "sysctl_sched\n");
    ns!(m, "  .%-40s: %Ld.%06ld\n", cstr!("sysctl_sched_base_slice"), b::sysctl_sched_base_slice);
    out!(m, "  .%-40s: %Ld\n", cstr!("sysctl_sched_features"), b::sysctl_sched_features as i64);
    // Native enum validated by sched_scaling_write; names preserve original order.
    let scaling = b::sysctl_sched_tunable_scaling;
    let names = [cstr!("none"), cstr!("logarithmic"), cstr!("linear")];
    out!(m, "  .%-40s: %d (%s)\n", cstr!("sysctl_sched_tunable_scaling"), scaling as c_int, *names.as_ptr().add(scaling as usize));
    out!(m, "\n");

    }
}

#[cfg(CONFIG_NUMA_BALANCING)]
#[no_mangle]
/// Native scheduler debug entry point.
///
/// # Safety
/// The caller must satisfy the original debug.c entry point's object lifetime,
/// synchronization, IRQ-context and user-buffer contract, where applicable.
pub unsafe extern "C" fn print_numa_stats(m: *mut seq_file, node: c_int, tsf: c_ulong, tpf: c_ulong, gsf: c_ulong, gpf: c_ulong) {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    out!(m, "numa_faults node=%d ", node);
    out!(m, "task_private=%lu task_shared=%lu ", tpf, tsf);
    out!(m, "group_private=%lu group_shared=%lu\n", gpf, gsf);

    }
}

#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn sched_show_numa(p: *mut task_struct, m: *mut seq_file) {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    if !(*p).mm.is_null() { out!(m, "%-45s:%21Ld\n", cstr!("mm->numa_scan_seq"), (*(*p).mm).numa_scan_seq as i64); }
    out!(m, "%-45s:%21Ld\n", cstr!("numa_pages_migrated"), (*p).numa_pages_migrated as i64);
    out!(m, "%-45s:%21Ld\n", cstr!("numa_preferred_nid"), (*p).numa_preferred_nid as i64);
    out!(m, "%-45s:%21Ld\n", cstr!("total_numa_faults"), (*p).total_numa_faults as i64);
    out!(m, "current_node=%d, numa_group_id=%d\n", b::lupos_debug_task_node(p), b::lupos_debug_task_numa_group_id(p));
    b::lupos_debug_show_numa_stats(p, m);

    }
}

#[no_mangle]
/// Native scheduler debug entry point.
///
/// # Safety
/// The caller must satisfy the original debug.c entry point's object lifetime,
/// synchronization, IRQ-context and user-buffer contract, where applicable.
pub unsafe extern "C" fn proc_sched_show_task(p: *mut task_struct, ns: *mut pid_namespace, m: *mut seq_file) {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    out!(m, "%s (%d, #threads: %d)\n", (*p).comm.as_ptr(), b::lupos_debug_task_pid_nr_ns(p, ns), b::lupos_debug_get_nr_threads(p));
    out!(m, "-------------------------------------------------------------------\n");
    macro_rules! pfield { ($first:ident $(.$field:ident)*) => {
        out!(m, "%-45s:%21Ld\n", cstr!(concat!(stringify!($first) $(, ".", stringify!($field))*)), (*p).$first $(.$field)* as i64);
    }; }
    macro_rules! pnfield { ($first:ident $(.$field:ident)*) => {
        ns!(m, "%-45s:%14Ld.%06ld\n", cstr!(concat!(stringify!($first) $(, ".", stringify!($field))*)), (*p).$first $(.$field)*);
    }; }
    pnfield!(se.exec_start); pnfield!(se.vruntime); pnfield!(se.sum_exec_runtime);
    let nr_switches = (*p).nvcsw.wrapping_add((*p).nivcsw);
    pfield!(se.nr_migrations);
    if b::lupos_debug_schedstat_enabled() {
        macro_rules! sn { ($field:ident) => { ns!(m, "%-45s:%14Ld.%06ld\n", cstr!(stringify!($field)), stat_value!(addr_of!((*p).stats), $field)); }; }
        macro_rules! sp { ($field:ident) => { out!(m, "%-45s:%21Ld\n", cstr!(stringify!($field)), stat_value!(addr_of!((*p).stats), $field) as i64); }; }
        sn!(sum_sleep_runtime); sn!(sum_block_runtime); sn!(wait_start); sn!(sleep_start); sn!(block_start);
        sn!(sleep_max); sn!(block_max); sn!(exec_max); sn!(slice_max); sn!(wait_max); sn!(wait_sum);
        sp!(wait_count); sn!(iowait_sum); sp!(iowait_count);
        sp!(nr_failed_migrations_affine); sp!(nr_failed_migrations_running); sp!(nr_failed_migrations_hot);
        sp!(nr_forced_migrations); sp!(nr_wakeups); sp!(nr_wakeups_sync); sp!(nr_wakeups_migrate);
        sp!(nr_wakeups_local); sp!(nr_wakeups_remote); sp!(nr_wakeups_affine); sp!(nr_wakeups_affine_attempts);
        let avg_atom = if nr_switches != 0 { (*p).se.sum_exec_runtime / nr_switches as u64 } else { u64::MAX };
        let avg_per_cpu = if (*p).se.nr_migrations != 0 { (*p).se.sum_exec_runtime / (*p).se.nr_migrations } else { u64::MAX };
        ns!(m, "%-45s:%14Ld.%06ld\n", cstr!("avg_atom"), avg_atom);
        ns!(m, "%-45s:%14Ld.%06ld\n", cstr!("avg_per_cpu"), avg_per_cpu);
        #[cfg(CONFIG_SCHED_CORE)] sn!(core_forceidle_sum);
    }
    out!(m, "%-45s:%21Ld\n", cstr!("nr_switches"), nr_switches as i64);
    out!(m, "%-45s:%21Ld\n", cstr!("nr_voluntary_switches"), (*p).nvcsw as i64);
    out!(m, "%-45s:%21Ld\n", cstr!("nr_involuntary_switches"), (*p).nivcsw as i64);
    pfield!(se.load.weight); pfield!(se.avg.load_sum); pfield!(se.avg.runnable_sum); pfield!(se.avg.util_sum);
    pfield!(se.avg.load_avg); pfield!(se.avg.runnable_avg); pfield!(se.avg.util_avg); pfield!(se.avg.last_update_time);
    out!(m, "%-45s:%21Ld\n", cstr!("se.avg.util_est"), ((*p).se.avg.util_est & !b::LUPOS_DEBUG_UTIL_AVG_UNCHANGED) as i64);
    #[cfg(CONFIG_UCLAMP_TASK)] {
        out!(m, "%-45s:%21Ld\n", cstr!("uclamp.min"), b::lupos_debug_uclamp_req(p, b::LUPOS_DEBUG_UCLAMP_MIN) as i64);
        out!(m, "%-45s:%21Ld\n", cstr!("uclamp.max"), b::lupos_debug_uclamp_req(p, b::LUPOS_DEBUG_UCLAMP_MAX) as i64);
        out!(m, "%-45s:%21Ld\n", cstr!("effective uclamp.min"), b::lupos_debug_uclamp_eff(p, b::LUPOS_DEBUG_UCLAMP_MIN) as i64);
        out!(m, "%-45s:%21Ld\n", cstr!("effective uclamp.max"), b::lupos_debug_uclamp_eff(p, b::LUPOS_DEBUG_UCLAMP_MAX) as i64);
    }
    pfield!(policy); pfield!(prio);
    if b::lupos_debug_task_has_dl_policy(p) { pfield!(dl.runtime); pfield!(dl.deadline); }
    else if b::lupos_debug_fair_policy((*p).policy as c_int) { pfield!(se.slice); }
    #[cfg(CONFIG_SCHED_CLASS_EXT)] out!(m, "%-45s:%21Ld\n", cstr!("ext.enabled"), b::lupos_debug_task_on_scx(p) as i64);
    let this_cpu = b::lupos_debug_raw_smp_processor_id();
    let t0 = b::lupos_debug_cpu_clock(this_cpu as c_int);
    let t1 = b::lupos_debug_cpu_clock(this_cpu as c_int);
    out!(m, "%-45s:%21Ld\n", cstr!("clock-delta"), t1.wrapping_sub(t0) as i64);
    #[cfg(CONFIG_NUMA_BALANCING)] sched_show_numa(p, m);

    }
}
