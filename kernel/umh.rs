// SPDX-License-Identifier: GPL-2.0-only
/*
 * umh - the kernel usermode helper
 */
// C header dependencies are supplied by the surrounding kernel translation.

static mut usermodehelper_bset: kernel_cap_t = CAP_FULL_SET;
static mut usermodehelper_inheritable: kernel_cap_t = CAP_FULL_SET;
static mut umh_sysctl_lock: spinlock_t = DEFINE_SPINLOCK();
static mut umhelper_sem: rw_semaphore = DECLARE_RWSEM();

unsafe fn call_usermodehelper_freeinfo(info: *mut subprocess_info) {
    if !(*info).cleanup.is_none() {
        ((*info).cleanup.unwrap())(info);
    }
    kfree(info as *mut c_void);
}

unsafe fn umh_complete(sub_info: *mut subprocess_info) {
    let comp = xchg(&mut (*sub_info).complete, ptr::null_mut());
    /*
     * See call_usermodehelper_exec(). If xchg() returns NULL
     * we own sub_info, the UMH_KILLABLE caller has gone away
     * or the caller used UMH_NO_WAIT.
     */
    if !comp.is_null() {
        complete(comp);
    } else {
        call_usermodehelper_freeinfo(sub_info);
    }
}

/*
 * This is the task which runs the usermode application
 */
unsafe fn call_usermodehelper_exec_async(data: *mut c_void) -> c_int {
    let sub_info = data as *mut subprocess_info;
    let mut new: *mut cred;
    let mut retval: c_int;

    spin_lock_irq(&mut (*current).sighand.siglock);
    flush_signal_handlers(current, 1);
    spin_unlock_irq(&mut (*current).sighand.siglock);

    /* Usermodehelper threads get a copy of userspace init's fs_struct. */
    (*current).fs.umask = 0o022;
    /* Avoid propagating elevated workqueue scheduling priority. */
    set_user_nice(current, 0);

    retval = -ENOMEM;
    new = prepare_kernel_cred(current);
    if new.is_null() { goto out; }

    spin_lock(&mut umh_sysctl_lock);
    (*new).cap_bset = cap_intersect(usermodehelper_bset, (*new).cap_bset);
    (*new).cap_inheritable = cap_intersect(usermodehelper_inheritable, (*new).cap_inheritable);
    spin_unlock(&mut umh_sysctl_lock);

    if let Some(init) = (*sub_info).init {
        retval = init(sub_info, new);
        if retval != 0 {
            abort_creds(new);
            goto out;
        }
    }

    commit_creds(new);
    wait_for_initramfs();
    retval = kernel_execve((*sub_info).path,
                           (*sub_info).argv as *const *const c_char,
                           (*sub_info).envp as *const *const c_char);
out:
    (*sub_info).retval = retval;
    /* call_usermodehelper_exec_sync() calls umh_complete for UMH_WAIT_PROC. */
    if (*sub_info).wait & UMH_WAIT_PROC == 0 { umh_complete(sub_info); }
    if retval == 0 { return 0; }
    do_exit(0);
}

/* Handles UMH_WAIT_PROC. */
unsafe fn call_usermodehelper_exec_sync(sub_info: *mut subprocess_info) {
    let mut pid: pid_t;
    kernel_sigaction(SIGCHLD, SIG_DFL);
    pid = user_mode_thread(Some(call_usermodehelper_exec_async), sub_info as *mut c_void, SIGCHLD);
    if pid < 0 { (*sub_info).retval = pid; }
    else { kernel_wait(pid, &mut (*sub_info).retval); }
    kernel_sigaction(SIGCHLD, SIG_IGN);
    umh_complete(sub_info);
}

unsafe fn call_usermodehelper_exec_work(work: *mut work_struct) {
    let sub_info = container_of!(work, subprocess_info, work);
    if (*sub_info).wait & UMH_WAIT_PROC != 0 {
        call_usermodehelper_exec_sync(sub_info);
    } else {
        let pid = user_mode_thread(Some(call_usermodehelper_exec_async), sub_info as *mut c_void,
                                   CLONE_PARENT | SIGCHLD);
        if pid < 0 { (*sub_info).retval = pid; umh_complete(sub_info); }
    }
}

static mut usermodehelper_disabled: umh_disable_depth = UMH_DISABLED;
static mut running_helpers: atomic_t = ATOMIC_INIT(0);
static mut running_helpers_waitq: wait_queue_head = DECLARE_WAIT_QUEUE_HEAD();
static mut usermodehelper_disabled_waitq: wait_queue_head = DECLARE_WAIT_QUEUE_HEAD();

const RUNNING_HELPERS_TIMEOUT: c_long = 5 * HZ;

pub unsafe fn usermodehelper_read_trylock() -> c_int {
    let mut wait = DEFINE_WAIT();
    let mut ret = 0;
    down_read(&mut umhelper_sem);
    loop {
        prepare_to_wait(&mut usermodehelper_disabled_waitq, &mut wait, TASK_INTERRUPTIBLE);
        if usermodehelper_disabled == UMH_ENABLED { break; }
        if usermodehelper_disabled == UMH_DISABLED { ret = -EAGAIN; }
        up_read(&mut umhelper_sem);
        if ret != 0 { break; }
        schedule();
        try_to_freeze();
        down_read(&mut umhelper_sem);
    }
    finish_wait(&mut usermodehelper_disabled_waitq, &mut wait);
    ret
}

pub unsafe fn usermodehelper_read_lock_wait(mut timeout: c_long) -> c_long {
    if timeout < 0 { return -EINVAL as c_long; }
    let mut wait = DEFINE_WAIT();
    down_read(&mut umhelper_sem);
    loop {
        prepare_to_wait(&mut usermodehelper_disabled_waitq, &mut wait, TASK_UNINTERRUPTIBLE);
        if usermodehelper_disabled == UMH_ENABLED { break; }
        up_read(&mut umhelper_sem);
        timeout = schedule_timeout(timeout);
        if timeout == 0 { break; }
        down_read(&mut umhelper_sem);
    }
    finish_wait(&mut usermodehelper_disabled_waitq, &mut wait);
    timeout
}

pub unsafe fn usermodehelper_read_unlock() { up_read(&mut umhelper_sem); }

pub unsafe fn __usermodehelper_set_disable_depth(depth: umh_disable_depth) {
    down_write(&mut umhelper_sem);
    usermodehelper_disabled = depth;
    wake_up(&mut usermodehelper_disabled_waitq);
    up_write(&mut umhelper_sem);
}

pub unsafe fn __usermodehelper_disable(depth: umh_disable_depth) -> c_int {
    if depth == UMH_ENABLED { return -EINVAL; }
    down_write(&mut umhelper_sem);
    usermodehelper_disabled = depth;
    up_write(&mut umhelper_sem);
    let retval = wait_event_timeout!(&mut running_helpers_waitq,
                                     atomic_read(&running_helpers) == 0,
                                     RUNNING_HELPERS_TIMEOUT);
    if retval != 0 { return 0; }
    __usermodehelper_set_disable_depth(UMH_ENABLED);
    -EAGAIN
}

unsafe fn helper_lock() { atomic_inc(&mut running_helpers); smp_mb__after_atomic(); }
unsafe fn helper_unlock() { if atomic_dec_and_test(&mut running_helpers) { wake_up(&mut running_helpers_waitq); } }

pub unsafe fn call_usermodehelper_setup(path: *const c_char, argv: *mut *mut c_char,
    envp: *mut *mut c_char, gfp_mask: gfp_t,
    init: Option<unsafe extern "C" fn(*mut subprocess_info, *mut cred) -> c_int>,
    cleanup: Option<unsafe extern "C" fn(*mut subprocess_info)>, data: *mut c_void) -> *mut subprocess_info {
    let sub_info = kzalloc_obj::<subprocess_info>(gfp_mask);
    if sub_info.is_null() { return ptr::null_mut(); }
    INIT_WORK(&mut (*sub_info).work, Some(call_usermodehelper_exec_work));
    // CONFIG_STATIC_USERMODEHELPER selects CONFIG_STATIC_USERMODEHELPER_PATH at build time.
    (*sub_info).path = path;
    (*sub_info).argv = argv;
    (*sub_info).envp = envp;
    (*sub_info).cleanup = cleanup;
    (*sub_info).init = init;
    (*sub_info).data = data;
    sub_info
}

pub unsafe fn call_usermodehelper_exec(sub_info: *mut subprocess_info, wait: c_int) -> c_int {
    let mut state = TASK_UNINTERRUPTIBLE;
    let mut done = DECLARE_COMPLETION_ONSTACK();
    let mut retval = 0;
    if (*sub_info).path.is_null() { call_usermodehelper_freeinfo(sub_info); return -EINVAL; }
    helper_lock();
    if usermodehelper_disabled != UMH_ENABLED { retval = -EBUSY; goto_out!(out); }
    if strlen((*sub_info).path) == 0 { goto_out!(out); }
    (*sub_info).complete = if wait == UMH_NO_WAIT { ptr::null_mut() } else { &mut done };
    (*sub_info).wait = wait;
    queue_work(system_dfl_wq, &mut (*sub_info).work);
    if wait == UMH_NO_WAIT { helper_unlock(); return retval; }
    if wait & UMH_FREEZABLE != 0 { state |= TASK_FREEZABLE; }
    if wait & UMH_KILLABLE != 0 {
        retval = wait_for_completion_state(&mut done, state | TASK_KILLABLE);
        if retval == 0 { goto_wait_done!(wait_done); }
        if !xchg(&mut (*sub_info).complete, ptr::null_mut()).is_null() { helper_unlock(); return retval; }
    }
    wait_for_completion_state(&mut done, state);
wait_done:
    retval = (*sub_info).retval;
out:
    call_usermodehelper_freeinfo(sub_info);
    helper_unlock();
    retval
}

pub unsafe fn call_usermodehelper(path: *const c_char, argv: *mut *mut c_char,
    envp: *mut *mut c_char, wait: c_int) -> c_int {
    let gfp_mask = if wait == UMH_NO_WAIT { GFP_ATOMIC } else { GFP_KERNEL };
    let info = call_usermodehelper_setup(path, argv, envp, gfp_mask, None, None, ptr::null_mut());
    if info.is_null() { return -ENOMEM; }
    call_usermodehelper_exec(info, wait)
}

// CONFIG_SYSCTL section: preserved as a direct translation of the kernel sysctl handler.
#[cfg(CONFIG_SYSCTL)]
unsafe fn proc_cap_handler(table: *const ctl_table, write: c_int, buffer: *mut c_void,
                           lenp: *mut usize, ppos: *mut loff_t) -> c_int {
    let mut t = *table;
    let mut cap_array = [0_ulong; 2];
    let mut new_cap: kernel_cap_t;
    let cap = (*table).data as *mut kernel_cap_t;
    if write != 0 && (!capable(CAP_SETPCAP) || !capable(CAP_SYS_MODULE)) { return -EPERM; }
    spin_lock(&mut umh_sysctl_lock);
    cap_array[0] = (*cap).val as u32 as ulong;
    cap_array[1] = ((*cap).val >> 32) as ulong;
    spin_unlock(&mut umh_sysctl_lock);
    t.data = cap_array.as_mut_ptr() as *mut c_void;
    let err = proc_doulongvec_minmax(&mut t, write, buffer, lenp, ppos);
    if err < 0 { return err; }
    new_cap.val = cap_array[0] as u32 as u64;
    new_cap.val += (cap_array[1] as u64) << 32;
    if write != 0 {
        spin_lock(&mut umh_sysctl_lock);
        *cap = cap_intersect(*cap, new_cap);
        spin_unlock(&mut umh_sysctl_lock);
    }
    0
}

#[cfg(CONFIG_SYSCTL)]
static usermodehelper_table: [ctl_table; 3] = [
    ctl_table { procname: cstr!("bset"), data: unsafe { &mut usermodehelper_bset as *mut _ as *mut c_void },
        maxlen: 2 * size_of::<ulong>(), mode: 0o600, proc_handler: Some(proc_cap_handler) },
    ctl_table { procname: cstr!("inheritable"), data: unsafe { &mut usermodehelper_inheritable as *mut _ as *mut c_void },
        maxlen: 2 * size_of::<ulong>(), mode: 0o600, proc_handler: Some(proc_cap_handler) },
    ctl_table::default(),
];

#[cfg(CONFIG_SYSCTL)]
unsafe fn init_umh_sysctls() -> c_int {
    register_sysctl_init(cstr!("kernel/usermodehelper"), usermodehelper_table.as_ptr());
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
