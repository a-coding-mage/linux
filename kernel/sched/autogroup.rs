// SPDX-License-Identifier: GPL-2.0

/*
 * Auto-group scheduling implementation:
 */

// Dependencies supplied by the surrounding kernel translation.

pub static mut sysctl_sched_autogroup_enabled: ::core::ffi::c_uint = 1;
static mut autogroup_default: autogroup = unsafe { ::core::mem::zeroed() };
static mut autogroup_seq_nr: atomic_t = unsafe { ::core::mem::zeroed() };

// CONFIG_SYSCTL conditional declarations are preserved from the C source.
#[cfg(CONFIG_SYSCTL)]
static sched_autogroup_sysctls: [ctl_table; 1] = [ctl_table {
    procname: "sched_autogroup_enabled\0".as_ptr() as *mut ::core::ffi::c_char,
    data: unsafe { &mut sysctl_sched_autogroup_enabled as *mut _ as *mut ::core::ffi::c_void },
    maxlen: ::core::mem::size_of::<::core::ffi::c_uint>(),
    mode: 0o644,
    proc_handler: Some(proc_dointvec_minmax),
    extra1: SYSCTL_ZERO,
    extra2: SYSCTL_ONE,
    ..unsafe { ::core::mem::zeroed() }
}];

#[cfg(CONFIG_SYSCTL)]
unsafe fn sched_autogroup_sysctl_init() {
    register_sysctl_init("kernel\0".as_ptr() as *const ::core::ffi::c_char, sched_autogroup_sysctls.as_ptr());
}
#[cfg(not(CONFIG_SYSCTL))]
unsafe fn sched_autogroup_sysctl_init() {}

pub unsafe fn autogroup_init(init_task: *mut task_struct) {
    autogroup_default.tg = &mut root_task_group;
    kref_init(&mut autogroup_default.kref);
    init_rwsem(&mut autogroup_default.lock);
    (*(*init_task).signal).autogroup = &mut autogroup_default;
    sched_autogroup_sysctl_init();
}

pub unsafe fn autogroup_free(tg: *mut task_group) {
    kfree((*tg).autogroup as *mut ::core::ffi::c_void);
}

unsafe fn autogroup_destroy(kref: *mut kref) {
    let ag = container_of!(kref, autogroup, kref);
    // CONFIG_RT_GROUP_SCHED: RT tasks have been redirected to the root group.
    (*(*ag).tg).rt_se = ::core::ptr::null_mut();
    (*(*ag).tg).rt_rq = ::core::ptr::null_mut();
    sched_release_group((*ag).tg);
    sched_destroy_group((*ag).tg);
}

unsafe fn autogroup_kref_put(ag: *mut autogroup) {
    kref_put(&mut (*ag).kref, autogroup_destroy);
}

unsafe fn autogroup_kref_get(ag: *mut autogroup) -> *mut autogroup {
    kref_get(&mut (*ag).kref);
    ag
}

unsafe fn autogroup_task_get(p: *mut task_struct) -> *mut autogroup {
    let mut flags: ::core::ffi::c_ulong = 0;
    if !lock_task_sighand(p, &mut flags) {
        return autogroup_kref_get(&mut autogroup_default);
    }
    let ag = autogroup_kref_get((*(*p).signal).autogroup);
    unlock_task_sighand(p, &mut flags);
    ag
}

unsafe fn autogroup_create() -> *mut autogroup {
    let ag = kzalloc_obj::<autogroup>();
    if ag.is_null() { return autogroup_kref_get(&mut autogroup_default); }
    let tg = sched_create_group(&mut root_task_group);
    if IS_ERR(tg) {
        kfree(ag as *mut ::core::ffi::c_void);
        if printk_ratelimit() { printk(KERN_WARNING, "autogroup_create: %s failure.\n", if ag.is_null() { "kzalloc()" } else { "sched_create_group()" }); }
        return autogroup_kref_get(&mut autogroup_default);
    }
    kref_init(&mut (*ag).kref);
    init_rwsem(&mut (*ag).lock);
    (*ag).id = atomic_inc_return(&mut autogroup_seq_nr);
    (*ag).tg = tg;
    free_rt_sched_group(tg);
    (*tg).rt_se = root_task_group.rt_se;
    (*tg).rt_rq = root_task_group.rt_rq;
    (*tg).autogroup = ag;
    sched_online_group(tg, &mut root_task_group);
    ag
}

pub unsafe fn task_wants_autogroup(p: *mut task_struct, tg: *mut task_group) -> bool {
    if tg != &mut root_task_group || ((*p).flags & PF_EXITING) != 0 { return false; }
    true
}

pub unsafe fn sched_autogroup_exit_task(p: *mut task_struct) { sched_move_task(p, true); }

unsafe fn autogroup_move_group(p: *mut task_struct, ag: *mut autogroup) {
    let mut flags = 0 as ::core::ffi::c_ulong;
    if WARN_ON_ONCE(!lock_task_sighand(p, &mut flags)) { return; }
    let prev = (*(*p).signal).autogroup;
    if prev == ag { unlock_task_sighand(p, &mut flags); return; }
    (*(*p).signal).autogroup = autogroup_kref_get(ag);
    for_each_thread!(p, t, { sched_move_task(t, true); });
    unlock_task_sighand(p, &mut flags);
    autogroup_kref_put(prev);
}

pub unsafe fn sched_autogroup_create_attach(p: *mut task_struct) {
    let ag = autogroup_create(); autogroup_move_group(p, ag); autogroup_kref_put(ag);
}
pub unsafe fn sched_autogroup_detach(p: *mut task_struct) { autogroup_move_group(p, &mut autogroup_default); }
pub unsafe fn sched_autogroup_fork(sig: *mut signal_struct) { (*sig).autogroup = autogroup_task_get(current); }
pub unsafe fn sched_autogroup_exit(sig: *mut signal_struct) { autogroup_kref_put((*sig).autogroup); }

unsafe fn setup_autogroup(_str: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    sysctl_sched_autogroup_enabled = 0; 1
}

// __setup("noautogroup", setup_autogroup);

#[cfg(CONFIG_PROC_FS)]
pub unsafe fn proc_sched_autogroup_set_nice(p: *mut task_struct, nice: ::core::ffi::c_int) -> ::core::ffi::c_int {
    static mut next: ::core::ffi::c_ulong = INITIAL_JIFFIES;
    if nice < MIN_NICE || nice > MAX_NICE { return -EINVAL; }
    let mut err = security_task_setnice(current, nice); if err != 0 { return err; }
    if nice < 0 && !can_nice(current, nice) { return -EPERM; }
    if !capable(CAP_SYS_ADMIN) && time_before(jiffies, next) { return -EAGAIN; }
    next = HZ / 10 + jiffies;
    let ag = autogroup_task_get(p);
    let idx = array_index_nospec((nice + 20) as usize, 40);
    let shares = scale_load(sched_prio_to_weight[idx]);
    down_write(&mut (*ag).lock); err = sched_group_set_shares((*ag).tg, shares);
    if err == 0 { (*ag).nice = nice; } up_write(&mut (*ag).lock); autogroup_kref_put(ag); err
}

#[cfg(CONFIG_PROC_FS)]
pub unsafe fn proc_sched_autogroup_show_task(p: *mut task_struct, m: *mut seq_file) {
    let ag = autogroup_task_get(p);
    if task_group_is_autogroup((*ag).tg) { down_read(&(*ag).lock); seq_printf(m, "/autogroup-%ld nice %d\n", (*ag).id, (*ag).nice); up_read(&(*ag).lock); }
    autogroup_kref_put(ag);
}

pub unsafe fn autogroup_path(tg: *mut task_group, buf: *mut ::core::ffi::c_char, buflen: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if !task_group_is_autogroup(tg) { return 0; }
    snprintf(buf, buflen, "%s-%ld\0".as_ptr() as *const _, "/autogroup\0".as_ptr() as *const _, (*(*tg).autogroup).id)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
