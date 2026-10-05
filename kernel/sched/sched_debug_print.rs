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
    macro_rules! pn { ($site:expr, $field:ident) => {
        ns!(m, $site, "  .%-30s: %lld.%06ld\n", cstr!(concat!("se->", stringify!($field))), (*se).$field);
    }; }
    pn!(903, exec_start); pn!(904, vruntime); pn!(905, sum_exec_runtime);
    if b::lupos_debug_schedstat_enabled() {
        #[cfg(CONFIG_SCHEDSTATS)]
        let stats = b::lupos_debug_stats_from_se(se);
        macro_rules! sn { ($site:expr, $field:ident) => {
            ns!(m, $site, "  .%-30s: %lld.%06ld\n", cstr!(stringify!($field)), stat_value!(stats, $field));
        }; }
        sn!(911, wait_start); sn!(912, sleep_start); sn!(913, block_start); sn!(914, sleep_max);
        sn!(915, block_max); sn!(916, exec_max); sn!(917, slice_max); sn!(918, wait_max); sn!(919, wait_sum);
        out!(m, 920, "  .%-30s: %lld\n", cstr!("wait_count"), stat_value!(stats, wait_count) as i64);
    }
    out!(m, 923, "  .%-30s: %lld\n", cstr!("se->load.weight"), (*se).load.weight as i64);
    out!(m, 924, "  .%-30s: %lld\n", cstr!("se->avg.load_avg"), (*se).avg.load_avg as i64);
    out!(m, 925, "  .%-30s: %lld\n", cstr!("se->avg.util_avg"), (*se).avg.util_avg as i64);
    out!(m, 926, "  .%-30s: %lld\n", cstr!("se->avg.runnable_avg"), (*se).avg.runnable_avg as i64);

    }
}

unsafe fn print_task(m: *mut seq_file, rq: *mut rq, p: *mut task_struct) {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    if b::lupos_debug_task_current(rq, p) != 0 { out!(m, 974, ">R"); }
    else { out!(m, 976, " %c", b::lupos_debug_task_state_to_char(p) as c_int); }
    out!(m, 978, " %15s %5d %10ld %9Ld.%06ld   %c   %9Ld.%06ld %c %9Ld.%06ld %9Ld.%06ld %9Ld   %5d ",
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
    out!(m, 990, "%9lld.%06ld %9lld.%06ld %9lld.%06ld",
        nsec_high(wait), nsec_low(wait), nsec_high(sleep), nsec_low(sleep), nsec_high(block), nsec_low(block));
    #[cfg(CONFIG_NUMA_BALANCING)]
    out!(m, 996, "   %d      %d", b::lupos_debug_task_node(p), b::lupos_debug_task_numa_group_id(p));
    #[cfg(CONFIG_CGROUP_SCHED)]
    with_task_group_path(b::lupos_debug_task_group(p), |path| { out!(m, 999, "        %s", path); });
    out!(m, 1002, "\n");

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
    out!(m, 1009, "\n");
    out!(m, 1010, "runnable tasks:\n");
    out!(m, 1011, concat!(" S            task   PID     weight       vruntime   eligible    ",
        "deadline             slice          sum-exec      switches  ",
        "prio         wait-time        sum-sleep       sum-block",
        numa_columns!(), cgroup_columns!(), "\n"));
    out!(m, 1021, concat!("-------------------------------------------------------",
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
    #[cfg(CONFIG_FAIR_GROUP_SCHED)] out!(m, 1054, "\n");
    #[cfg(not(CONFIG_FAIR_GROUP_SCHED))] out!(m, 1057, "\n");
    #[cfg(CONFIG_FAIR_GROUP_SCHED)]
    with_task_group_path((*cfs_rq).tg, |path| { out!(m, 1055, "cfs_rq[%d]:%s\n", cpu, path); });
    #[cfg(not(CONFIG_FAIR_GROUP_SCHED))]
    out!(m, 1058, "cfs_rq[%d]:\n", cpu);
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
    macro_rules! pn { ($site:expr, $name:literal, $value:expr) => { ns!(m, $site, "  .%-30s: %Ld.%06ld\n", cstr!($name), $value); }; }
    pn!(1078, "left_deadline", left_deadline); pn!(1080, "left_vruntime", left_vruntime); pn!(1082, "zero_vruntime", zero_vruntime);
    out!(m, 1084, "  .%-30s: %Ld (%d bits)\n", cstr!("sum_w_vruntime"), sum_w_vruntime, b::lupos_debug_ilog2_abs(sum_w_vruntime));
    out!(m, 1086, "  .%-30s: %Lu\n", cstr!("sum_weight"), sum_weight);
    out!(m, 1088, "  .%-30s: %u\n", cstr!("sum_shift"), sum_shift);
    pn!(1089, "avg_vruntime", avruntime); pn!(1091, "right_vruntime", right_vruntime);
    pn!(1094, "spread", right_vruntime.wrapping_sub(left_vruntime));
    macro_rules! pi { ($site:expr, $field:ident) => { out!(m, $site, "  .%-30s: %d\n", cstr!(stringify!($field)), (*cfs_rq).$field as c_int); }; }
    pi!(1095, nr_queued); pi!(1096, h_nr_runnable); pi!(1097, h_nr_queued); pi!(1098, h_nr_idle);
    out!(m, 1099, "  .%-30s: %ld\n", cstr!("load"), (*cfs_rq).load.weight as c_long);
    out!(m, 1100, "  .%-30s: %lu\n", cstr!("load_avg"), (*cfs_rq).avg.load_avg as c_ulong);
    out!(m, 1102, "  .%-30s: %lu\n", cstr!("runnable_avg"), (*cfs_rq).avg.runnable_avg as c_ulong);
    out!(m, 1104, "  .%-30s: %lu\n", cstr!("util_avg"), (*cfs_rq).avg.util_avg as c_ulong);
    out!(m, 1106, "  .%-30s: %u\n", cstr!("util_est"), (*cfs_rq).avg.util_est);
    out!(m, 1108, "  .%-30s: %ld\n", cstr!("removed.load_avg"), (*cfs_rq).removed.load_avg as c_long);
    out!(m, 1110, "  .%-30s: %ld\n", cstr!("removed.util_avg"), (*cfs_rq).removed.util_avg as c_long);
    out!(m, 1112, "  .%-30s: %ld\n", cstr!("removed.runnable_avg"), (*cfs_rq).removed.runnable_avg as c_long);
    #[cfg(CONFIG_FAIR_GROUP_SCHED)] {
        out!(m, 1115, "  .%-30s: %lu\n", cstr!("tg_load_avg_contrib"), (*cfs_rq).tg_load_avg_contrib);
        out!(m, 1117, "  .%-30s: %ld\n", cstr!("tg_load_avg"), b::lupos_debug_atomic_long_read(addr_of!((*(*cfs_rq).tg).load_avg)));
        out!(m, 1119, "  .%-30s: %lu\n", cstr!("h_load"), (*cfs_rq).h_load);
    }
    #[cfg(CONFIG_CFS_BANDWIDTH)] {
        out!(m, 1123, "  .%-30s: %d\n", cstr!("throttled"), b::lupos_debug_cfs_throttled(cfs_rq) as c_int);
        pi!(1125, throttle_count);
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
    #[cfg(CONFIG_RT_GROUP_SCHED)] out!(m, 1137, "\n");
    #[cfg(not(CONFIG_RT_GROUP_SCHED))] out!(m, 1140, "\n");
    #[cfg(CONFIG_RT_GROUP_SCHED)]
    with_task_group_path((*rt_rq).tg, |path| { out!(m, 1138, "rt_rq[%d]:%s\n", cpu, path); });
    #[cfg(not(CONFIG_RT_GROUP_SCHED))] out!(m, 1141, "rt_rq[%d]:\n", cpu);
    out!(m, 1151, "  .%-30s: %lu\n", cstr!("rt_nr_running"), (*rt_rq).rt_nr_running as c_ulong);
    #[cfg(CONFIG_RT_GROUP_SCHED)] {
        out!(m, 1154, "  .%-30s: %Ld\n", cstr!("rt_throttled"), (*rt_rq).rt_throttled as i64);
        ns!(m, 1155, "  .%-30s: %Ld.%06ld\n", cstr!("rt_time"), (*rt_rq).rt_time);
        ns!(m, 1156, "  .%-30s: %Ld.%06ld\n", cstr!("rt_runtime"), (*rt_rq).rt_runtime);
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
    out!(m, 1168, "\n"); out!(m, 1169, "dl_rq[%d]:\n", cpu);
    out!(m, 1174, "  .%-30s: %lu\n", cstr!("dl_nr_running"), (*dl_rq).dl_nr_running as c_ulong);
    let bw = addr_of!((*(*b::lupos_debug_cpu_rq(cpu)).rd).dl_bw);
    out!(m, 1176, "  .%-30s: %lld\n", cstr!("dl_bw->bw"), (*bw).bw as i64);
    out!(m, 1177, "  .%-30s: %lld\n", cstr!("dl_bw->total_bw"), (*bw).total_bw as i64);

    }
}

unsafe fn print_cpu(m: *mut seq_file, cpu: c_int) {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    let rq = b::lupos_debug_cpu_rq(cpu);
    #[cfg(CONFIG_X86)] {
        let freq = if b::cpu_khz != 0 { b::cpu_khz } else { 1 };
        out!(m, 1190, "cpu#%d, %u.%03u MHz\n", cpu, freq / 1000, freq % 1000);
    }
    #[cfg(not(CONFIG_X86))] out!(m, 1194, "cpu#%d\n", cpu);
    macro_rules! p { ($site:expr, $field:ident, $size:ident) => {{
        if b::$size == 4 {
            out!(m, $site, "  .%-30s: %d\n", cstr!(stringify!($field)), (*rq).$field as c_int);
        } else {
            out!(m, $site, "  .%-30s: %Ld\n", cstr!(stringify!($field)), (*rq).$field as i64);
        }
    }}; }
    macro_rules! pn { ($site:expr, $field:ident) => { ns!(m, $site, "  .%-30s: %Ld.%06ld\n", cstr!(stringify!($field)), (*rq).$field); }; }
    p!(1208, nr_running, LUPOS_DEBUG_RQ_NR_RUNNING_SIZE);
    p!(1209, nr_switches, LUPOS_DEBUG_RQ_NR_SWITCHES_SIZE);
    p!(1210, nr_uninterruptible, LUPOS_DEBUG_RQ_NR_UNINTERRUPTIBLE_SIZE);
    pn!(1211, next_balance);
    out!(m, 1212, "  .%-30s: %ld\n", cstr!("curr->pid"), b::lupos_debug_task_pid_nr(b::lupos_debug_rq_curr(rq)) as c_long);
    pn!(1213, clock); pn!(1214, clock_task);
    out!(m, 1219, "  .%-30s: %Ld\n", cstr!("avg_idle"), (*rq).avg_idle as i64);
    out!(m, 1220, "  .%-30s: %Ld\n", cstr!("max_idle_balance_cost"), (*rq).max_idle_balance_cost as i64);
    if b::lupos_debug_schedstat_enabled() {
        macro_rules! ps { ($site:expr, $field:ident) => {
            out!(m, $site, "  .%-30s: %d\n", cstr!(stringify!($field)), stat_value!(rq, $field) as c_int);
        }; }
        ps!(1225, yld_count); ps!(1226, sched_count); ps!(1227, sched_goidle); ps!(1228, ttwu_count); ps!(1229, ttwu_local);
    }
    // These are other scheduler owners (fair.c/rt.c/deadline.c), not debug.c fallbacks.
    b::lupos_debug_print_cfs_stats(m, cpu);
    b::lupos_debug_print_rt_stats(m, cpu);
    b::lupos_debug_print_dl_stats(m, cpu);
    print_rq(m, rq, cpu);
    out!(m, 1238, "\n");

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
    out!(m, 1258, "Sched Debug Version: v0.11, %s %.*s\n", (*uts).release.as_ptr(),
        b::lupos_debug_strcspn((*uts).version.as_ptr(), cstr!(" ")) as c_int, (*uts).version.as_ptr());
    ns!(m, 1267, "%-40s: %Ld.%06ld\n", cstr!("ktime"), ktime);
    ns!(m, 1268, "%-40s: %Ld.%06ld\n", cstr!("sched_clk"), sched_clk);
    ns!(m, 1269, "%-40s: %Ld.%06ld\n", cstr!("cpu_clk"), cpu_clk);
    out!(m, 1270, "%-40s: %Ld\n", cstr!("jiffies"), b::lupos_debug_jiffies() as i64);
    #[cfg(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK)]
    out!(m, 1272, "%-40s: %Ld\n", cstr!("sched_clock_stable()"), b::lupos_debug_sched_clock_stable() as i64);
    out!(m, 1277, "\n"); out!(m, 1278, "sysctl_sched\n");
    ns!(m, 1284, "  .%-40s: %Ld.%06ld\n", cstr!("sysctl_sched_base_slice"), b::sysctl_sched_base_slice);
    out!(m, 1285, "  .%-40s: %Ld\n", cstr!("sysctl_sched_features"), b::sysctl_sched_features as i64);
    // Native enum validated by sched_scaling_write; names preserve original order.
    let scaling = b::sysctl_sched_tunable_scaling;
    let names = [cstr!("none"), cstr!("logarithmic"), cstr!("linear")];
    out!(m, 1289, "  .%-40s: %d (%s)\n", cstr!("sysctl_sched_tunable_scaling"), scaling as c_int, *names.as_ptr().add(scaling as usize));
    out!(m, 1293, "\n");

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
    out!(m, 1384, "numa_faults node=%d ", node);
    out!(m, 1385, "task_private=%lu task_shared=%lu ", tpf, tsf);
    out!(m, 1386, "group_private=%lu group_shared=%lu\n", gpf, gsf);

    }
}

#[cfg(CONFIG_NUMA_BALANCING)]
unsafe fn sched_show_numa(p: *mut task_struct, m: *mut seq_file) {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    if !(*p).mm.is_null() { out!(m, 1395, "%-45s:%21Ld\n", cstr!("mm->numa_scan_seq"), (*(*p).mm).numa_scan_seq as i64); }
    out!(m, 1397, "%-45s:%21Ld\n", cstr!("numa_pages_migrated"), (*p).numa_pages_migrated as i64);
    out!(m, 1398, "%-45s:%21Ld\n", cstr!("numa_preferred_nid"), (*p).numa_preferred_nid as i64);
    out!(m, 1399, "%-45s:%21Ld\n", cstr!("total_numa_faults"), (*p).total_numa_faults as i64);
    out!(m, 1400, "current_node=%d, numa_group_id=%d\n", b::lupos_debug_task_node(p), b::lupos_debug_task_numa_group_id(p));
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
    out!(m, 1411, "%s (%d, #threads: %d)\n", (*p).comm.as_ptr(), b::lupos_debug_task_pid_nr_ns(p, ns), b::lupos_debug_get_nr_threads(p));
    out!(m, 1413, "-------------------------------------------------------------------\n");
    macro_rules! pfield { ($site:expr, $first:ident $(.$field:ident)*) => {
        out!(m, $site, "%-45s:%21Ld\n", cstr!(concat!(stringify!($first) $(, ".", stringify!($field))*)), (*p).$first $(.$field)* as i64);
    }; }
    macro_rules! pnfield { ($site:expr, $first:ident $(.$field:ident)*) => {
        ns!(m, $site, "%-45s:%14Ld.%06ld\n", cstr!(concat!(stringify!($first) $(, ".", stringify!($field))*)), (*p).$first $(.$field)*);
    }; }
    pnfield!(1420, se.exec_start); pnfield!(1421, se.vruntime); pnfield!(1422, se.sum_exec_runtime);
    let nr_switches = (*p).nvcsw.wrapping_add((*p).nivcsw);
    pfield!(1426, se.nr_migrations);
    if b::lupos_debug_schedstat_enabled() {
        macro_rules! sn { ($site:expr, $field:ident) => { ns!(m, $site, "%-45s:%14Ld.%06ld\n", cstr!(stringify!($field)), stat_value!(addr_of!((*p).stats), $field)); }; }
        macro_rules! sp { ($site:expr, $field:ident) => { out!(m, $site, "%-45s:%21Ld\n", cstr!(stringify!($field)), stat_value!(addr_of!((*p).stats), $field) as i64); }; }
        sn!(1431, sum_sleep_runtime); sn!(1432, sum_block_runtime); sn!(1433, wait_start); sn!(1434, sleep_start); sn!(1435, block_start);
        sn!(1436, sleep_max); sn!(1437, block_max); sn!(1438, exec_max); sn!(1439, slice_max); sn!(1440, wait_max); sn!(1441, wait_sum);
        sp!(1442, wait_count); sn!(1443, iowait_sum); sp!(1444, iowait_count);
        sp!(1445, nr_failed_migrations_affine); sp!(1446, nr_failed_migrations_running); sp!(1447, nr_failed_migrations_hot);
        sp!(1448, nr_forced_migrations); sp!(1449, nr_wakeups); sp!(1450, nr_wakeups_sync); sp!(1451, nr_wakeups_migrate);
        sp!(1452, nr_wakeups_local); sp!(1453, nr_wakeups_remote); sp!(1454, nr_wakeups_affine); sp!(1455, nr_wakeups_affine_attempts);
        let avg_atom = if nr_switches != 0 { (*p).se.sum_exec_runtime / nr_switches as u64 } else { u64::MAX };
        let avg_per_cpu = if (*p).se.nr_migrations != 0 { (*p).se.sum_exec_runtime / (*p).se.nr_migrations } else { u64::MAX };
        ns!(m, 1471, "%-45s:%14Ld.%06ld\n", cstr!("avg_atom"), avg_atom);
        ns!(m, 1472, "%-45s:%14Ld.%06ld\n", cstr!("avg_per_cpu"), avg_per_cpu);
        #[cfg(CONFIG_SCHED_CORE)] sn!(1475, core_forceidle_sum);
    }
    out!(m, 1479, "%-45s:%21Ld\n", cstr!("nr_switches"), nr_switches as i64);
    out!(m, 1480, "%-45s:%21Ld\n", cstr!("nr_voluntary_switches"), (*p).nvcsw as i64);
    out!(m, 1481, "%-45s:%21Ld\n", cstr!("nr_involuntary_switches"), (*p).nivcsw as i64);
    pfield!(1483, se.load.weight); pfield!(1484, se.avg.load_sum); pfield!(1485, se.avg.runnable_sum); pfield!(1486, se.avg.util_sum);
    pfield!(1487, se.avg.load_avg); pfield!(1488, se.avg.runnable_avg); pfield!(1489, se.avg.util_avg); pfield!(1490, se.avg.last_update_time);
    out!(m, 1491, "%-45s:%21Ld\n", cstr!("se.avg.util_est"), ((*p).se.avg.util_est & !b::LUPOS_DEBUG_UTIL_AVG_UNCHANGED) as i64);
    #[cfg(CONFIG_UCLAMP_TASK)] {
        out!(m, 1493, "%-45s:%21Ld\n", cstr!("uclamp.min"), b::lupos_debug_uclamp_req(p, b::LUPOS_DEBUG_UCLAMP_MIN) as i64);
        out!(m, 1494, "%-45s:%21Ld\n", cstr!("uclamp.max"), b::lupos_debug_uclamp_req(p, b::LUPOS_DEBUG_UCLAMP_MAX) as i64);
        out!(m, 1495, "%-45s:%21Ld\n", cstr!("effective uclamp.min"), b::lupos_debug_uclamp_eff(p, b::LUPOS_DEBUG_UCLAMP_MIN) as i64);
        out!(m, 1496, "%-45s:%21Ld\n", cstr!("effective uclamp.max"), b::lupos_debug_uclamp_eff(p, b::LUPOS_DEBUG_UCLAMP_MAX) as i64);
    }
    pfield!(1498, policy); pfield!(1499, prio);
    if b::lupos_debug_task_has_dl_policy(p) { pfield!(1501, dl.runtime); pfield!(1502, dl.deadline); }
    else if b::lupos_debug_fair_policy((*p).policy as c_int) { pfield!(1504, se.slice); }
    #[cfg(CONFIG_SCHED_CLASS_EXT)] out!(m, 1507, "%-45s:%21Ld\n", cstr!("ext.enabled"), b::lupos_debug_task_on_scx(p) as i64);
    let this_cpu = b::lupos_debug_raw_smp_processor_id();
    let t0 = b::lupos_debug_cpu_clock(this_cpu as c_int);
    let t1 = b::lupos_debug_cpu_clock(this_cpu as c_int);
    out!(m, 1518, "%-45s:%21Ld\n", cstr!("clock-delta"), t1.wrapping_sub(t0) as i64);
    #[cfg(CONFIG_NUMA_BALANCING)] sched_show_numa(p, m);

    }
}
