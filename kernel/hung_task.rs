// SPDX-License-Identifier: GPL-2.0-only
/* Detect Hung Task -- direct Rust translation of kernel/hung_task.c. */

// Kernel headers and symbols are supplied by the surrounding translation.

static mut SYSCTL_HUNG_TASK_CHECK_COUNT: i32 = PID_MAX_LIMIT;
static mut SYSCTL_HUNG_TASK_DETECT_COUNT: atomic_long_t = ATOMIC_LONG_INIT(0);

const HUNG_TASK_LOCK_BREAK: usize = HZ / 10;
static mut sysctl_hung_task_timeout_secs: c_ulong = CONFIG_DEFAULT_HUNG_TASK_TIMEOUT;
static mut sysctl_hung_task_check_interval_secs: c_ulong = 0;
static mut sysctl_hung_task_warnings: i32 = 10;
static mut did_panic: i32 = 0;
static mut hung_task_call_panic: bool = false;
static mut watchdog_task: *mut task_struct = core::ptr::null_mut();
static mut hung_task_si_mask: c_ulong = 0;

#[cfg(CONFIG_SMP)]
static mut sysctl_hung_task_all_cpu_backtrace: c_uint = 0;
#[cfg(not(CONFIG_SMP))]
const sysctl_hung_task_all_cpu_backtrace: c_uint = 0;

static mut sysctl_hung_task_panic: c_uint = CONFIG_BOOTPARAM_HUNG_TASK_PANIC;

unsafe extern "C" fn hung_task_panic(_this: *mut notifier_block, _event: c_ulong, _ptr: *mut c_void) -> c_int {
    did_panic = 1;
    NOTIFY_DONE
}

static mut panic_block: notifier_block = notifier_block { notifier_call: Some(hung_task_panic) };

unsafe fn task_is_hung(t: *mut task_struct, timeout: c_ulong) -> bool {
    let switch_count = (*t).nvcsw + (*t).nivcsw;
    let state = READ_ONCE((*t).__state);
    if state & TASK_UNINTERRUPTIBLE == 0 || state & (TASK_WAKEKILL | TASK_NOLOAD | TASK_FROZEN) != 0 { return false; }
    if unlikely(switch_count == 0) { return false; }
    if switch_count != (*t).last_switch_count {
        (*t).last_switch_count = switch_count;
        (*t).last_switch_time = jiffies;
        return false;
    }
    if time_is_after_jiffies((*t).last_switch_time + timeout * HZ) { return false; }
    true
}

#[cfg(CONFIG_DETECT_HUNG_TASK_BLOCKER)]
unsafe fn debug_show_blocker(task: *mut task_struct, timeout: c_ulong) {
    let blocker = READ_ONCE((*task).blocker);
    if blocker == 0 { return; }
    let blocker_type = hung_task_get_blocker_type(blocker);
    let owner = match blocker_type {
        BLOCKER_TYPE_MUTEX => mutex_get_owner(hung_task_blocker_to_lock(blocker)),
        BLOCKER_TYPE_SEM => sem_last_holder(hung_task_blocker_to_lock(blocker)),
        BLOCKER_TYPE_RWSEM_READER | BLOCKER_TYPE_RWSEM_WRITER => rwsem_owner(hung_task_blocker_to_lock(blocker)) as c_ulong,
        _ => { WARN_ON_ONCE(1); return; }
    };
    if unlikely(owner == 0) { return; }
    let mut g: *mut task_struct = core::ptr::null_mut();
    let mut t: *mut task_struct = core::ptr::null_mut();
    for_each_process_thread!(g, t) {
        if t as c_ulong != owner { continue; }
        sched_show_task(t);
        if !task_is_hung(t, timeout) { sched_show_task(t); }
        return;
    }
}
#[cfg(not(CONFIG_DETECT_HUNG_TASK_BLOCKER))]
unsafe fn debug_show_blocker(_task: *mut task_struct, _timeout: c_ulong) {}

unsafe fn hung_task_info(t: *mut task_struct, timeout: c_ulong, this_round_count: c_ulong) {
    trace_sched_process_hang(t);
    if sysctl_hung_task_panic != 0 && this_round_count >= sysctl_hung_task_panic as c_ulong {
        console_verbose(); hung_task_call_panic = true;
    }
    if sysctl_hung_task_warnings != 0 || hung_task_call_panic {
        if sysctl_hung_task_warnings > 0 { sysctl_hung_task_warnings -= 1; }
        pr_err!("INFO: task %s:%d blocked%s for more than %ld seconds.\n", (*t).comm, (*t).pid, if (*t).in_iowait { " in I/O wait" } else { "" }, (jiffies - (*t).last_switch_time) / HZ);
        pr_err!("      %s %s %.*s\n", print_tainted(), init_utsname().release, strcspn(init_utsname().version, " "), init_utsname().version);
        if (*t).flags & PF_POSTCOREDUMP != 0 { pr_err!("      Blocked by coredump.\n"); }
        pr_err!("\"echo 0 > /proc/sys/kernel/hung_task_timeout_secs\" disables this message.\n");
        sched_show_task(t); debug_show_blocker(t, timeout);
        if sysctl_hung_task_warnings == 0 { pr_info!("Future hung task reports are suppressed, see sysctl kernel.hung_task_warnings\n"); }
    }
    touch_nmi_watchdog();
}

unsafe fn rcu_lock_break(g: *mut task_struct, t: *mut task_struct) -> bool {
    get_task_struct(g); get_task_struct(t); rcu_read_unlock(); cond_resched(); rcu_read_lock();
    let can_cont = pid_alive(g) && pid_alive(t); put_task_struct(t); put_task_struct(g); can_cont
}

unsafe fn check_hung_uninterruptible_tasks(timeout: c_ulong) {
    let mut max_count = SYSCTL_HUNG_TASK_CHECK_COUNT; let mut last_break = jiffies;
    let mut this_round_count = 0; let need_warning = sysctl_hung_task_warnings; let mut si_mask = hung_task_si_mask;
    if test_taint(TAINT_DIE) || did_panic != 0 { return; }
    rcu_read_lock();
    let mut g: *mut task_struct = core::ptr::null_mut(); let mut t: *mut task_struct = core::ptr::null_mut();
    for_each_process_thread!(g, t) {
        if max_count <= 0 { break; } max_count -= 1;
        if time_after(jiffies, last_break + HUNG_TASK_LOCK_BREAK) { if !rcu_lock_break(g, t) { break; } last_break = jiffies; }
        if task_is_hung(t, timeout) { atomic_long_inc(&raw mut SYSCTL_HUNG_TASK_DETECT_COUNT); this_round_count += 1; hung_task_info(t, timeout, this_round_count); }
    }
    rcu_read_unlock(); if this_round_count == 0 { return; }
    if need_warning != 0 || hung_task_call_panic { si_mask |= SYS_INFO_LOCKS; if sysctl_hung_task_all_cpu_backtrace != 0 { si_mask |= SYS_INFO_ALL_BT; } }
    sys_info(si_mask); if hung_task_call_panic { panic!("hung_task: blocked tasks"); }
}

unsafe fn hung_timeout_jiffies(last_checked: c_ulong, timeout: c_ulong) -> c_long { if timeout != 0 { (last_checked - jiffies + timeout * HZ) as c_long } else { MAX_SCHEDULE_TIMEOUT } }

#[cfg(CONFIG_SYSCTL)]
unsafe extern "C" fn proc_dohung_task_detect_count(table: *const ctl_table, dir: c_int, buffer: *mut c_void, lenp: *mut usize, ppos: *mut loff_t) -> c_int {
    let mut detect_count: c_ulong = 0; let mut proxy_table = *table; proxy_table.data = &mut detect_count;
    if SYSCTL_KERN_TO_USER(dir) { detect_count = atomic_long_read(&raw mut SYSCTL_HUNG_TASK_DETECT_COUNT); }
    let err = proc_doulongvec_minmax(&mut proxy_table, dir, buffer, lenp, ppos); if err < 0 { return err; }
    if SYSCTL_USER_TO_KERN(dir) { if detect_count != 0 { return -EINVAL; } atomic_long_set(&raw mut SYSCTL_HUNG_TASK_DETECT_COUNT, 0); } 0
}

#[cfg(CONFIG_SYSCTL)]
unsafe extern "C" fn proc_dohung_task_timeout_secs(table: *const ctl_table, write: c_int, buffer: *mut c_void, lenp: *mut usize, ppos: *mut loff_t) -> c_int {
    let ret = proc_doulongvec_minmax(table, write, buffer, lenp, ppos); if ret == 0 && write != 0 { wake_up_process(watchdog_task); } ret
}
#[cfg(CONFIG_SYSCTL)]
static hung_task_timeout_max: c_ulong = LONG_MAX as c_ulong / HZ;
#[cfg(CONFIG_SYSCTL)]
unsafe fn hung_task_sysctl_init() { register_sysctl_init(c"kernel".as_ptr(), core::ptr::null()); }
#[cfg(not(CONFIG_SYSCTL))]
unsafe fn hung_task_sysctl_init() {}

static mut reset_hung_task: atomic_t = ATOMIC_INIT(0);
pub unsafe extern "C" fn reset_hung_task_detector() { atomic_set(&raw mut reset_hung_task, 1); }
static mut hung_detector_suspended: bool = false;

unsafe extern "C" fn hungtask_pm_notify(_self: *mut notifier_block, action: c_ulong, _hcpu: *mut c_void) -> c_int {
    match action { PM_SUSPEND_PREPARE | PM_HIBERNATION_PREPARE | PM_RESTORE_PREPARE => hung_detector_suspended = true, PM_POST_SUSPEND | PM_POST_HIBERNATION | PM_POST_RESTORE => hung_detector_suspended = false, _ => {} } NOTIFY_OK
}

unsafe extern "C" fn watchdog(_dummy: *mut c_void) -> c_int {
    let mut hung_last_checked = jiffies; set_user_nice(current, 0);
    loop {
        let timeout = sysctl_hung_task_timeout_secs; let mut interval = sysctl_hung_task_check_interval_secs;
        if interval == 0 { interval = timeout; } interval = min_t!(c_ulong, interval, timeout); let t = hung_timeout_jiffies(hung_last_checked, interval);
        if t <= 0 { if atomic_xchg(&raw mut reset_hung_task, 0) == 0 && !hung_detector_suspended { check_hung_uninterruptible_tasks(timeout); } hung_last_checked = jiffies; continue; }
        schedule_timeout_interruptible(t);
    }
}

unsafe extern "C" fn hung_task_init() -> c_int {
    atomic_notifier_chain_register(&raw mut panic_notifier_list, &raw mut panic_block);
    pm_notifier(hungtask_pm_notify, 0);
    watchdog_task = kthread_run(watchdog, core::ptr::null_mut(), c"khungtaskd".as_ptr());
    hung_task_sysctl_init();
    0
}

// subsys_initcall(hung_task_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
