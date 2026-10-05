// SPDX-License-Identifier: GPL-2.0-only
// Rust owners for debug.c controls, iteration and directory decisions.

unsafe fn sched_feat_set(mut cmp: *mut c_char) -> c_int {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    let negative = b::lupos_debug_strncmp(cmp, cstr!("NO_"), 3) == 0;
    if negative { cmp = cmp.add(3); }
    let mut index = 0;
    while index < b::LUPOS_DEBUG_SCHED_FEAT_NR as c_int {
        if b::lupos_debug_strcmp(cmp, b::lupos_debug_feat_name(index)) == 0 { break; }
        index += 1;
    }
    if index == b::LUPOS_DEBUG_SCHED_FEAT_NR as c_int { return -(b::LUPOS_DEBUG_EINVAL as c_int); }
    let bit = (1 as c_ulong).wrapping_shl(index as u32);
    if negative {
        b::sysctl_sched_features = ((b::sysctl_sched_features as c_ulong) & !bit) as c_uint;
        #[cfg(CONFIG_JUMP_LABEL)] b::lupos_debug_feat_disable(index);
    } else {
        b::sysctl_sched_features = ((b::sysctl_sched_features as c_ulong) | bit) as c_uint;
        #[cfg(CONFIG_JUMP_LABEL)] b::lupos_debug_feat_enable(index);
    }
    0

    }
}

#[no_mangle]
/// Native scheduler debug entry point.
///
/// # Safety
/// The caller must satisfy the original debug.c entry point's object lifetime,
/// synchronization, IRQ-context and user-buffer contract, where applicable.
pub unsafe extern "C" fn lupos_debug_feat_write(filp: *mut file, ubuf: *const c_char, mut cnt: b::size_t, ppos: *mut b::loff_t) -> b::ssize_t {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    let mut buf = [0 as c_char; 64];
    cnt = core::cmp::min(cnt, 63);
    if b::lupos_debug_copy_from_user(buf.as_mut_ptr().cast(), ubuf.cast(), cnt as c_ulong) != 0 { return -(b::LUPOS_DEBUG_EFAULT as b::ssize_t); }
    buf[cnt as usize] = 0;
    let cmp = b::lupos_debug_strstrip(buf.as_mut_ptr());
    let inode = b::lupos_debug_file_inode(filp);
    b::lupos_debug_cpus_read_lock();
    b::lupos_debug_inode_lock(inode);
    let ret = sched_feat_set(cmp);
    b::lupos_debug_inode_unlock(inode);
    b::lupos_debug_cpus_read_unlock();
    if ret < 0 { return ret as b::ssize_t; }
    *ppos = (*ppos).wrapping_add(cnt as b::loff_t);
    cnt as b::ssize_t

    }
}

#[no_mangle]
/// Native scheduler debug entry point.
///
/// # Safety
/// The caller must satisfy the original debug.c entry point's object lifetime,
/// synchronization, IRQ-context and user-buffer contract, where applicable.
pub unsafe extern "C" fn lupos_debug_scaling_write(_filp: *mut file, ubuf: *const c_char, cnt: b::size_t, ppos: *mut b::loff_t) -> b::ssize_t {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    let mut scaling: c_uint = 0;
    let ret = b::lupos_debug_kstrtouint_from_user(ubuf, cnt, 10, &mut scaling);
    if ret != 0 { return ret as b::ssize_t; }
    if scaling >= b::LUPOS_DEBUG_TUNABLESCALING_END { return -(b::LUPOS_DEBUG_EINVAL as b::ssize_t); }
    b::sysctl_sched_tunable_scaling = scaling;
    if b::lupos_debug_update_scaling() != 0 { return -(b::LUPOS_DEBUG_EINVAL as b::ssize_t); }
    *ppos = (*ppos).wrapping_add(cnt as b::loff_t);
    cnt as b::ssize_t

    }
}

#[cfg(CONFIG_SCHED_CACHE)]
#[no_mangle]
/// Native scheduler debug entry point.
///
/// # Safety
/// The caller must satisfy the original debug.c entry point's object lifetime,
/// synchronization, IRQ-context and user-buffer contract, where applicable.
pub unsafe extern "C" fn lupos_debug_cache_enable_write(_filp: *mut file, ubuf: *const c_char, cnt: b::size_t, ppos: *mut b::loff_t) -> b::ssize_t {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    let mut value = false;
    let ret = b::lupos_debug_kstrtobool_from_user(ubuf, cnt, &mut value);
    if ret != 0 { return ret as b::ssize_t; }
    b::sysctl_sched_cache_user = value as c_int;
    b::lupos_debug_cache_active_set();
    *ppos = (*ppos).wrapping_add(cnt as b::loff_t);
    cnt as b::ssize_t

    }
}
#[cfg(CONFIG_SCHED_CACHE)]
#[no_mangle]
/// Native scheduler debug entry point.
///
/// # Safety
/// The caller must satisfy the original debug.c entry point's object lifetime,
/// synchronization, IRQ-context and user-buffer contract, where applicable.
pub unsafe extern "C" fn lupos_debug_cache_enable_show(m: *mut seq_file, _v: *mut c_void) -> c_int {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    out!(m, "%d\n", b::sysctl_sched_cache_user);
    0

    }
}

#[cfg(CONFIG_PREEMPT_DYNAMIC)]
#[no_mangle]
/// Native scheduler debug entry point.
///
/// # Safety
/// The caller must satisfy the original debug.c entry point's object lifetime,
/// synchronization, IRQ-context and user-buffer contract, where applicable.
pub unsafe extern "C" fn lupos_debug_dynamic_write(_filp: *mut file, ubuf: *const c_char, mut cnt: b::size_t, ppos: *mut b::loff_t) -> b::ssize_t {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    let mut buf = [0 as c_char; 16];
    cnt = core::cmp::min(cnt, 15);
    if b::lupos_debug_copy_from_user(buf.as_mut_ptr().cast(), ubuf.cast(), cnt as c_ulong) != 0 { return -(b::LUPOS_DEBUG_EFAULT as b::ssize_t); }
    buf[cnt as usize] = 0;
    let mode = b::lupos_debug_dynamic_mode(b::lupos_debug_strstrip(buf.as_mut_ptr()));
    if mode < 0 { return mode as b::ssize_t; }
    b::lupos_debug_dynamic_update(mode);
    *ppos = (*ppos).wrapping_add(cnt as b::loff_t);
    cnt as b::ssize_t

    }
}
#[cfg(CONFIG_PREEMPT_DYNAMIC)]
#[no_mangle]
/// Native scheduler debug entry point.
///
/// # Safety
/// The caller must satisfy the original debug.c entry point's object lifetime,
/// synchronization, IRQ-context and user-buffer contract, where applicable.
pub unsafe extern "C" fn lupos_debug_dynamic_show(m: *mut seq_file, _v: *mut c_void) -> c_int {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    let mut i = ((cfg!(CONFIG_PREEMPT_RT) || cfg!(CONFIG_ARCH_HAS_PREEMPT_LAZY)) as c_int) * 2;
    let mode = b::lupos_debug_dynamic_read();
    let mut end = 0;
    while !b::lupos_debug_preempt_mode(end).is_null() { end += 1; }
    end -= (!cfg!(CONFIG_ARCH_HAS_PREEMPT_LAZY)) as c_int;
    while i < end {
        if mode == i { b::lupos_debug_seq_puts(m, cstr!("(")); }
        b::lupos_debug_seq_puts(m, b::lupos_debug_preempt_mode(i));
        if mode == i { b::lupos_debug_seq_puts(m, cstr!(")")); }
        b::lupos_debug_seq_puts(m, cstr!(" "));
        i += 1;
    }
    b::lupos_debug_seq_puts(m, cstr!("\n"));
    0

    }
}

#[no_mangle]
/// Native scheduler debug entry point.
///
/// # Safety
/// The caller must satisfy the original debug.c entry point's object lifetime,
/// synchronization, IRQ-context and user-buffer contract, where applicable.
pub unsafe extern "C" fn lupos_debug_verbose_write(filp: *mut file, ubuf: *const c_char, cnt: b::size_t, ppos: *mut b::loff_t) -> b::ssize_t {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    b::lupos_debug_cpus_read_lock();
    b::lupos_debug_domains_lock();
    let orig = b::sched_debug_verbose;
    let result = b::lupos_debug_write_file_bool(filp, ubuf, cnt, ppos);
    if b::sched_debug_verbose && !orig { update_sched_domain_debugfs(); }
    else if !b::sched_debug_verbose && orig {
        b::lupos_debug_remove(b::lupos_debug_sd_dentry);
        b::lupos_debug_sd_dentry = null_mut();
    }
    b::lupos_debug_domains_unlock();
    b::lupos_debug_cpus_read_unlock();
    result

    }
}

unsafe fn sched_server_write_common(filp: *mut file, ubuf: *const c_char, cnt: b::size_t, ppos: *mut b::loff_t, period_param: bool, server: *mut b::sched_dl_entity) -> b::ssize_t {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    let cpu = (*((*filp).private_data.cast::<seq_file>())).private as c_long;
    let rq = b::lupos_debug_cpu_rq(cpu as c_int);
    let mut value = 0u64;
    let err = b::lupos_debug_kstrtoull_from_user(ubuf, cnt, 10, &mut value);
    if err != 0 { return err as b::ssize_t; }
    let mut rf = core::mem::MaybeUninit::<b::rq_flags>::uninit();
    b::lupos_debug_rq_lock(rq, rf.as_mut_ptr());
    let old_runtime = (*server).dl_runtime;
    let runtime = if period_param { old_runtime } else { value };
    let period = if period_param { value } else { (*server).dl_period };
    if runtime > period || period > b::LUPOS_DEBUG_SERVER_PERIOD_MAX as u64 || period < b::LUPOS_DEBUG_SERVER_PERIOD_MIN as u64 {
        b::lupos_debug_rq_unlock(rq, rf.as_mut_ptr());
        return -(b::LUPOS_DEBUG_EINVAL as b::ssize_t);
    }
    if !b::lupos_debug_cpu_online(b::lupos_debug_cpu_of(rq)) {
        b::lupos_debug_rq_unlock(rq, rf.as_mut_ptr());
        return -(b::LUPOS_DEBUG_EBUSY as b::ssize_t);
    }
    b::lupos_debug_update_rq_clock(rq);
    b::lupos_debug_server_stop(server);
    let retval = b::lupos_debug_server_apply_params(server, runtime, period, false);
    b::lupos_debug_server_start(server);
    b::lupos_debug_rq_unlock(rq, rf.as_mut_ptr());
    if retval < 0 { return retval as b::ssize_t; }
    if (old_runtime != 0) ^ (runtime != 0) {
        b::lupos_debug_info(cstr!("%s server %sabled on CPU %d%s.\n"),
            if server == addr_of_mut!((*rq).fair_server) { cstr!("Fair") } else { cstr!("Ext") },
            if runtime != 0 { cstr!("en") } else { cstr!("dis") }, b::lupos_debug_cpu_of(rq),
            if runtime != 0 { cstr!("") } else { cstr!(", system may malfunction due to starvation") });
    }
    *ppos = (*ppos).wrapping_add(cnt as b::loff_t);
    cnt as b::ssize_t

    }
}

macro_rules! server_callbacks {
    ($show:ident, $write:ident, $field:ident, $period:expr) => {
        #[no_mangle]
        /// Native scheduler debug entry point.
        ///
        /// # Safety
        /// The caller must satisfy the original debug.c entry point's object lifetime,
        /// synchronization, IRQ-context and user-buffer contract, where applicable.
        pub unsafe extern "C" fn $show(m: *mut seq_file, _v: *mut c_void) -> c_int {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
            let rq = b::lupos_debug_cpu_rq((*m).private as c_ulong as c_int);
            let server = addr_of!((*rq).$field);
            out!(m, "%llu\n", if $period { (*server).dl_period } else { (*server).dl_runtime });
            0

    }
}
        #[no_mangle]
        /// Native scheduler debug entry point.
        ///
        /// # Safety
        /// The caller must satisfy the original debug.c entry point's object lifetime,
        /// synchronization, IRQ-context and user-buffer contract, where applicable.
        pub unsafe extern "C" fn $write(filp: *mut file, ubuf: *const c_char, cnt: b::size_t, ppos: *mut b::loff_t) -> b::ssize_t {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
            let cpu = (*((*filp).private_data.cast::<seq_file>())).private as c_long;
            let rq = b::lupos_debug_cpu_rq(cpu as c_int);
            sched_server_write_common(filp, ubuf, cnt, ppos, $period, addr_of_mut!((*rq).$field))

    }
}
    };
}
server_callbacks!(lupos_debug_fair_runtime_show, lupos_debug_fair_runtime_write, fair_server, false);
server_callbacks!(lupos_debug_fair_period_show, lupos_debug_fair_period_write, fair_server, true);
#[cfg(CONFIG_SCHED_CLASS_EXT)]
server_callbacks!(lupos_debug_ext_runtime_show, lupos_debug_ext_runtime_write, ext_server, false);
#[cfg(CONFIG_SCHED_CLASS_EXT)]
server_callbacks!(lupos_debug_ext_period_show, lupos_debug_ext_period_write, ext_server, true);

#[cfg(CONFIG_FAIR_GROUP_SCHED)]
fn cgroup_mode_name(index: c_int) -> *const c_char {
    match index { 0 => cstr!("up"), 1 => cstr!("smp"), 2 => cstr!("concur"), 3 => cstr!("max"), 4 => cstr!("tasks"), _ => core::ptr::null() }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
#[no_mangle]
/// Native scheduler debug entry point.
///
/// # Safety
/// The caller must satisfy the original debug.c entry point's object lifetime,
/// synchronization, IRQ-context and user-buffer contract, where applicable.
pub unsafe extern "C" fn lupos_debug_cgroup_write(_filp: *mut file, ubuf: *const c_char, mut cnt: b::size_t, ppos: *mut b::loff_t) -> b::ssize_t {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    let mut buf = [0 as c_char; 16];
    cnt = core::cmp::min(cnt, 15);
    if b::lupos_debug_copy_from_user(buf.as_mut_ptr().cast(), ubuf.cast(), cnt as c_ulong) != 0 { return -(b::LUPOS_DEBUG_EFAULT as b::ssize_t); }
    buf[cnt as usize] = 0;
    let value = b::lupos_debug_strstrip(buf.as_mut_ptr());
    let mut mode = 0;
    while mode < 5 && b::lupos_debug_strcmp(value, cgroup_mode_name(mode)) != 0 { mode += 1; }
    if mode == 5 { return -(b::LUPOS_DEBUG_EINVAL as b::ssize_t); }
    b::lupos_debug_cgroup_mode_update(mode);
    b::lupos_debug_cgroup_mode_write(mode);
    *ppos = (*ppos).wrapping_add(cnt as b::loff_t);
    cnt as b::ssize_t

    }
}
#[cfg(CONFIG_FAIR_GROUP_SCHED)]
#[no_mangle]
/// Native scheduler debug entry point.
///
/// # Safety
/// The caller must satisfy the original debug.c entry point's object lifetime,
/// synchronization, IRQ-context and user-buffer contract, where applicable.
pub unsafe extern "C" fn lupos_debug_cgroup_show(m: *mut seq_file, _v: *mut c_void) -> c_int {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    let mode = b::lupos_debug_cgroup_mode_read();
    for i in 0..5 {
        if mode == i { b::lupos_debug_seq_puts(m, cstr!("(")); }
        b::lupos_debug_seq_puts(m, cgroup_mode_name(i));
        if mode == i { b::lupos_debug_seq_puts(m, cstr!(")")); }
        b::lupos_debug_seq_puts(m, cstr!(" "));
    }
    b::lupos_debug_seq_puts(m, cstr!("\n"));
    0

    }
}

unsafe fn debugfs_server_init(ext: bool) {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    let dir = b::lupos_debug_create_dir(if ext { cstr!("ext_server") } else { cstr!("fair_server") }, b::lupos_debug_root);
    if dir.is_null() { return; }
    each_configured_cpu(b::lupos_debug_possible_mask(), |cpu| {
        let mut buf = [0 as c_char; 32];
        b::lupos_debug_snprintf(buf.as_mut_ptr(), buf.len() as b::size_t, cstr!("cpu%lu"), cpu as c_ulong);
        let cpu_dir = b::lupos_debug_create_dir(buf.as_ptr(), dir);
        let (runtime, period) = if ext {
            (b::LUPOS_DEBUG_EXT_RUNTIME, b::LUPOS_DEBUG_EXT_PERIOD)
        } else { (b::LUPOS_DEBUG_FAIR_RUNTIME, b::LUPOS_DEBUG_FAIR_PERIOD) };
        b::lupos_debug_create_file(cstr!("runtime"), 0o644, cpu_dir, cpu as usize as *mut c_void, runtime);
        b::lupos_debug_create_file(cstr!("period"), 0o644, cpu_dir, cpu as usize as *mut c_void, period);
    });

    }
}

#[no_mangle]
/// Native scheduler debug entry point.
///
/// # Safety
/// The caller must satisfy the original debug.c entry point's object lifetime,
/// synchronization, IRQ-context and user-buffer contract, where applicable.
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn lupos_debug_init() -> c_int {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    b::lupos_debug_root = b::lupos_debug_create_dir(cstr!("sched"), null_mut());
    let root = b::lupos_debug_root;
    b::lupos_debug_create_file(cstr!("features"), 0o644, root, null_mut(), b::LUPOS_DEBUG_FEATURES);
    b::lupos_debug_create_verbose(root);
    #[cfg(CONFIG_PREEMPT_DYNAMIC)]
    b::lupos_debug_create_file(cstr!("preempt"), 0o644, root, null_mut(), b::LUPOS_DEBUG_PREEMPT);
    macro_rules! u32_file { ($name:literal, $var:ident, $parent:expr) => {
        b::lupos_debug_create_u32(cstr!($name), 0o644, $parent, addr_of_mut!(b::$var).cast());
    }; }
    u32_file!("base_slice_ns", sysctl_sched_base_slice, root);
    u32_file!("latency_warn_ms", sysctl_resched_latency_warn_ms, root);
    u32_file!("latency_warn_once", sysctl_resched_latency_warn_once, root);
    b::lupos_debug_create_file(cstr!("tunable_scaling"), 0o644, root, null_mut(), b::LUPOS_DEBUG_SCALING);
    u32_file!("migration_cost_ns", sysctl_sched_migration_cost, root);
    u32_file!("nr_migrate", sysctl_sched_nr_migrate, root);
    b::lupos_debug_domains_lock();
    update_sched_domain_debugfs();
    b::lupos_debug_domains_unlock();
    #[cfg(CONFIG_NUMA_BALANCING)] {
        let numa = b::lupos_debug_create_dir(cstr!("numa_balancing"), root);
        u32_file!("scan_delay_ms", sysctl_numa_balancing_scan_delay, numa);
        u32_file!("scan_period_min_ms", sysctl_numa_balancing_scan_period_min, numa);
        u32_file!("scan_period_max_ms", sysctl_numa_balancing_scan_period_max, numa);
        u32_file!("scan_size_mb", sysctl_numa_balancing_scan_size, numa);
        u32_file!("hot_threshold_ms", sysctl_numa_balancing_hot_threshold, numa);
    }
    #[cfg(CONFIG_SCHED_CACHE)] {
        let llc = b::lupos_debug_create_dir(cstr!("llc_balancing"), root);
        b::lupos_debug_create_file(cstr!("enabled"), 0o644, llc, null_mut(), b::LUPOS_DEBUG_CACHE);
        u32_file!("aggr_tolerance", llc_aggr_tolerance, llc);
        u32_file!("epoch_period", llc_epoch_period, llc);
        u32_file!("epoch_affinity_timeout", llc_epoch_affinity_timeout, llc);
        u32_file!("overaggr_pct", llc_overaggr_pct, llc);
        u32_file!("imb_pct", llc_imb_pct, llc);
    }
    b::lupos_debug_create_file(cstr!("debug"), 0o444, root, null_mut(), b::LUPOS_DEBUG_DUMP);
    #[cfg(CONFIG_FAIR_GROUP_SCHED)]
    b::lupos_debug_create_file(cstr!("cgroup_mode"), 0o644, root, null_mut(), b::LUPOS_DEBUG_CGROUP);
    debugfs_server_init(false);
    #[cfg(CONFIG_SCHED_CLASS_EXT)] debugfs_server_init(true);
    0

    }
}

#[no_mangle]
/// Native scheduler debug entry point.
///
/// # Safety
/// The caller must satisfy the original debug.c entry point's object lifetime,
/// synchronization, IRQ-context and user-buffer contract, where applicable.
pub unsafe extern "C" fn lupos_debug_sd_flags_show(m: *mut seq_file, _v: *mut c_void) -> c_int {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    let flags = *((*m).private.cast::<c_uint>()) as c_ulong;
    for i in 0..b::LUPOS_DEBUG_SD_FLAG_CNT {
        if flags & (1 as c_ulong).wrapping_shl(i) != 0 {
            b::lupos_debug_seq_puts(m, b::lupos_debug_sd_flag_name(i));
            b::lupos_debug_seq_puts(m, cstr!(" "));
        }
    }
    b::lupos_debug_seq_puts(m, cstr!("\n"));
    0

    }
}

unsafe fn register_sd(sd: *mut sched_domain, parent: *mut b::dentry) {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    macro_rules! sd_ulong { ($field:ident) => {
        b::lupos_debug_create_ulong(cstr!(stringify!($field)), 0o644, parent, addr_of_mut!((*sd).$field));
    }; }
    macro_rules! sd_u32 { ($field:ident) => {
        b::lupos_debug_create_u32(cstr!(stringify!($field)), 0o644, parent, addr_of_mut!((*sd).$field));
    }; }
    sd_ulong!(min_interval); sd_ulong!(max_interval);
    b::lupos_debug_create_u64(cstr!("max_newidle_lb_cost"), 0o644, parent, addr_of_mut!((*sd).max_newidle_lb_cost));
    sd_u32!(busy_factor); sd_u32!(imbalance_pct); sd_u32!(cache_nice_tries);
    b::lupos_debug_create_str(cstr!("name"), 0o444, parent, addr_of_mut!((*sd).name));
    b::lupos_debug_create_file(cstr!("flags"), 0o444, parent, addr_of_mut!((*sd).flags).cast(), b::LUPOS_DEBUG_SD_FLAGS);
    b::lupos_debug_create_file(cstr!("groups_flags"), 0o444, parent, addr_of_mut!((*(*sd).groups).flags).cast(), b::LUPOS_DEBUG_SD_FLAGS);
    b::lupos_debug_create_u32(cstr!("level"), 0o444, parent, addr_of_mut!((*sd).level).cast());
    if ((*sd).flags as c_uint) & b::LUPOS_DEBUG_SD_ASYM_PACKING != 0 {
        b::lupos_debug_create_u32(cstr!("group_asym_prefer_cpu"), 0o444, parent, addr_of_mut!((*(*sd).groups).asym_prefer_cpu).cast());
    }

    }
}

#[no_mangle]
/// Native scheduler debug entry point.
///
/// # Safety
/// The caller must satisfy the original debug.c entry point's object lifetime,
/// synchronization, IRQ-context and user-buffer contract, where applicable.
pub unsafe extern "C" fn update_sched_domain_debugfs() {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    if b::lupos_debug_root.is_null() || !b::sched_debug_verbose { return; }
    if !b::lupos_debug_sd_mask_available() {
        if !b::lupos_debug_sd_mask_alloc() { return; }
        b::lupos_debug_mask_copy(b::lupos_debug_sd_mask(), b::lupos_debug_possible_mask());
    }
    if b::lupos_debug_sd_dentry.is_null() {
        b::lupos_debug_sd_dentry = b::lupos_debug_create_dir(cstr!("domains"), b::lupos_debug_root);
        if b::lupos_debug_mask_empty(b::lupos_debug_sd_mask()) {
            b::lupos_debug_mask_copy(b::lupos_debug_sd_mask(), b::lupos_debug_online_mask());
        }
    }
    let mask = b::lupos_debug_sd_mask();
    let mut cpu = b::lupos_debug_mask_first(mask);
    while cpu < b::lupos_debug_mask_bound() {
        let mut buf = [0 as c_char; 32];
        b::lupos_debug_snprintf(buf.as_mut_ptr(), buf.len() as b::size_t, cstr!("cpu%d"), cpu as c_int);
        b::lupos_debug_lookup_and_remove(buf.as_ptr(), b::lupos_debug_sd_dentry);
        let d_cpu = b::lupos_debug_create_dir(buf.as_ptr(), b::lupos_debug_sd_dentry);
        let mut sd = b::lupos_debug_first_domain(cpu as c_int);
        let mut i: c_int = 0;
        while !sd.is_null() {
            b::lupos_debug_snprintf(buf.as_mut_ptr(), buf.len() as b::size_t, cstr!("domain%d"), i);
            let d_sd = b::lupos_debug_create_dir(buf.as_ptr(), d_cpu);
            register_sd(sd, d_sd);
            i += 1;
            sd = (*sd).parent;
        }
        b::lupos_debug_mask_clear_cpu(cpu, mask);
        cpu = b::lupos_debug_mask_next(cpu as c_int, mask);
    }

    }
}

#[no_mangle]
/// Native scheduler debug entry point.
///
/// # Safety
/// The caller must satisfy the original debug.c entry point's object lifetime,
/// synchronization, IRQ-context and user-buffer contract, where applicable.
pub unsafe extern "C" fn dirty_sched_domain_sysctl(cpu: c_int) {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    if b::lupos_debug_sd_mask_available() { b::lupos_debug_mask_set_cpu(cpu as c_uint, b::lupos_debug_sd_mask()); }

    }
}

#[no_mangle]
/// Native scheduler debug entry point.
///
/// # Safety
/// The caller must satisfy the original debug.c entry point's object lifetime,
/// synchronization, IRQ-context and user-buffer contract, where applicable.
pub unsafe extern "C" fn lupos_debug_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    let cpu = (v as usize).wrapping_sub(2) as c_int;
    if cpu != -1 { print_cpu(m, cpu); } else { sched_debug_header(m); }
    0

    }
}
#[no_mangle]
/// Native scheduler debug entry point.
///
/// # Safety
/// The caller must satisfy the original debug.c entry point's object lifetime,
/// synchronization, IRQ-context and user-buffer contract, where applicable.
pub unsafe extern "C" fn lupos_debug_start(_file: *mut seq_file, offset: *mut b::loff_t) -> *mut c_void {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    let mut n = *offset as c_ulong;
    if n == 0 { return 1usize as *mut c_void; }
    n = n.wrapping_sub(1);
    n = if n > 0 { b::lupos_debug_mask_next(n.wrapping_sub(1) as c_int, b::lupos_debug_online_mask()) as c_ulong }
        else { b::lupos_debug_mask_first(b::lupos_debug_online_mask()) as c_ulong };
    *offset = n.wrapping_add(1) as b::loff_t;
    if n < b::lupos_debug_nr_cpu_ids() as c_ulong { n.wrapping_add(2) as *mut c_void } else { null_mut() }

    }
}
#[no_mangle]
/// Native scheduler debug entry point.
///
/// # Safety
/// The caller must satisfy the original debug.c entry point's object lifetime,
/// synchronization, IRQ-context and user-buffer contract, where applicable.
pub unsafe extern "C" fn lupos_debug_next(file: *mut seq_file, _data: *mut c_void, offset: *mut b::loff_t) -> *mut c_void {
    // SAFETY: The caller supplies the live native objects and original entry
    // point's locking/IRQ/VFS context; native primitives retain their checks.
    unsafe {
    *offset = (*offset).wrapping_add(1);
    lupos_debug_start(file, offset)

    }
}
#[no_mangle]
/// Native scheduler debug entry point.
///
/// # Safety
/// The caller must satisfy the original debug.c entry point's object lifetime,
/// synchronization, IRQ-context and user-buffer contract, where applicable.
pub unsafe extern "C" fn lupos_debug_stop(_file: *mut seq_file, _data: *mut c_void) {
    // The original stop callback intentionally has no work or owned resources.
}
