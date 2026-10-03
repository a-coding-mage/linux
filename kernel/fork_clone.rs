// SPDX-License-Identifier: GPL-2.0-only
// kernel/fork.c:2649-3071, clone entry points and UAPI validation.
unsafe fn init_idle_pids(idle: *mut task_struct) {
    for kind in PIDTYPE_PID..PIDTYPE_MAX {
        init_hlist(addr_of_mut!((*idle).pid_links[kind as usize]));
        init_task_pid(idle, kind, addr_of_mut!(init_struct_pid));
    }
}
unsafe extern "C" fn idle_dummy(_dummy: *mut c_void) -> c_int {
    0
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn fork_idle(cpu: c_int) -> *mut task_struct {
    let mut args: kernel_clone_args = zeroed();
    args.flags = CLONE_VM as u64;
    args.fn_ = Some(idle_dummy);
    args.fn_arg = null_mut();
    args.set_kthread(1);
    args.idle = 1;
    let task = copy_process(
        addr_of_mut!(init_struct_pid),
        0,
        rust_fork_cpu_to_node(cpu),
        &mut args,
    );
    if !is_err(task) {
        init_idle_pids(task);
        init_idle(task, cpu);
    }
    task
}
#[no_mangle]
pub unsafe extern "C" fn create_io_thread(
    func: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    arg: *mut c_void,
    node: c_int,
) -> *mut task_struct {
    let mut args: kernel_clone_args = zeroed();
    args.flags = (CLONE_FS
        | CLONE_FILES
        | CLONE_SIGHAND
        | CLONE_THREAD
        | CLONE_IO
        | CLONE_VM
        | CLONE_UNTRACED) as u64;
    args.fn_ = func;
    args.fn_arg = arg;
    args.set_io_thread(1);
    args.set_user_worker(1);
    copy_process(null_mut(), 0, node, &mut args)
}
#[no_mangle]
pub unsafe extern "C" fn kernel_clone(args: *mut kernel_clone_args) -> pid_t {
    let mut flags = (*args).flags;
    let mut vfork: completion = zeroed();
    if flags & CLONE_EMPTY_MNTNS as u64 != 0 {
        flags |= CLONE_NEWNS as u64;
        (*args).flags = flags;
    }
    if flags & CLONE_PIDFD as u64 != 0
        && flags & CLONE_PARENT_SETTID as u64 != 0
        && (*args).pidfd == (*args).parent_tid
    {
        return -(EINVAL as c_int);
    }
    if !rust_fork_valid_signal((*args).exit_signal as c_ulong) {
        return -(EINVAL as c_int);
    }
    let mut trace = 0;
    if flags & CLONE_UNTRACED as u64 == 0 {
        trace = if flags & CLONE_VFORK as u64 != 0 {
            PTRACE_EVENT_VFORK
        } else if (*args).exit_signal != SIGCHLD as c_int {
            PTRACE_EVENT_CLONE
        } else {
            PTRACE_EVENT_FORK
        };
        if !rust_fork_ptrace_event_enabled(current_task(), trace as c_int) {
            trace = 0;
        }
    }
    let p = copy_process(null_mut(), trace as c_int, NUMA_NO_NODE, args);
    rust_fork_add_latent_entropy();
    if is_err(p) {
        return ptr_err(p);
    }
    rust_fork_trace_sched_process_fork(current_task(), p);
    let pid = get_task_pid(p, PIDTYPE_PID);
    let nr = pid_vnr(pid);
    if flags & CLONE_PARENT_SETTID as u64 != 0 {
        rust_fork_put_user_int(nr, (*args).parent_tid);
    }
    if flags & CLONE_VFORK as u64 != 0 {
        (*p).vfork_done = &mut vfork;
        rust_fork_init_completion(&mut vfork);
        rust_fork_get_task_struct(p);
    }
    #[cfg(CONFIG_LRU_GEN_WALKS_MMU)]
    if flags & CLONE_VM as u64 == 0 {
        rust_fork_task_lock(p);
        rust_fork_lru_gen_add_mm((*p).mm);
        rust_fork_task_unlock(p);
    }
    wake_up_new_task(p);
    if trace != 0 {
        rust_fork_ptrace_event_pid(trace as c_int, pid);
    }
    if flags & CLONE_VFORK as u64 != 0 && wait_for_vfork_done(p, &mut vfork) == 0 {
        rust_fork_ptrace_event_pid(PTRACE_EVENT_VFORK_DONE as c_int, pid);
    }
    put_pid(pid);
    nr
}
#[no_mangle]
pub unsafe extern "C" fn kernel_thread(
    func: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    arg: *mut c_void,
    name: *const c_char,
    flags: c_ulong,
) -> pid_t {
    let mut args: kernel_clone_args = zeroed();
    args.flags =
        ((flags | CLONE_VM as c_ulong | CLONE_UNTRACED as c_ulong) & !(CSIGNAL as c_ulong)) as u64;
    args.exit_signal = (flags & CSIGNAL as c_ulong) as c_int;
    args.fn_ = func;
    args.fn_arg = arg;
    args.name = name;
    args.set_kthread(1);
    kernel_clone(&mut args)
}
#[no_mangle]
pub unsafe extern "C" fn user_mode_thread(
    func: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    arg: *mut c_void,
    flags: c_ulong,
) -> pid_t {
    let mut args: kernel_clone_args = zeroed();
    args.flags =
        ((flags | CLONE_VM as c_ulong | CLONE_UNTRACED as c_ulong) & !(CSIGNAL as c_ulong)) as u64;
    args.exit_signal = (flags & CSIGNAL as c_ulong) as c_int;
    args.fn_ = func;
    args.fn_arg = arg;
    args.set_umh(1);
    kernel_clone(&mut args)
}
#[no_mangle]
pub unsafe extern "C" fn rust_fork_sys_fork() -> c_long {
    #[cfg(CONFIG_MMU)]
    {
        let mut args: kernel_clone_args = zeroed();
        args.exit_signal = SIGCHLD as c_int;
        kernel_clone(&mut args) as c_long
    }
    #[cfg(not(CONFIG_MMU))]
    {
        -(EINVAL as c_long)
    }
}
#[no_mangle]
pub unsafe extern "C" fn rust_fork_sys_vfork() -> c_long {
    let mut args: kernel_clone_args = zeroed();
    args.flags = (CLONE_VFORK | CLONE_VM) as u64;
    args.exit_signal = SIGCHLD as c_int;
    kernel_clone(&mut args) as c_long
}
#[no_mangle]
pub unsafe extern "C" fn rust_fork_sys_clone(
    flags: c_ulong,
    newsp: c_ulong,
    parent_tid: *mut c_int,
    child_tid: *mut c_int,
    tls: c_ulong,
) -> c_long {
    let mut args: kernel_clone_args = zeroed();
    args.flags = ((flags as u32) & !CSIGNAL) as u64;
    args.pidfd = parent_tid;
    args.parent_tid = parent_tid;
    args.child_tid = child_tid;
    args.exit_signal = ((flags as u32) & CSIGNAL) as c_int;
    args.stack = newsp;
    args.tls = tls;
    kernel_clone(&mut args) as c_long
}
#[inline(never)]
unsafe fn copy_clone_args_from_user(
    kargs: *mut kernel_clone_args,
    uargs: *mut clone_args,
    usize_: usize,
) -> c_int {
    let mut args: clone_args = zeroed();
    let kset_tid = (*kargs).set_tid;
    const {
        assert!(offset_of!(clone_args, tls) + size_of::<u64>() == CLONE_ARGS_SIZE_VER0 as usize);
    }
    const {
        assert!(
            offset_of!(clone_args, set_tid_size) + size_of::<u64>()
                == CLONE_ARGS_SIZE_VER1 as usize
        );
    }
    const {
        assert!(offset_of!(clone_args, cgroup) + size_of::<u64>() == CLONE_ARGS_SIZE_VER2 as usize);
    }
    const {
        assert!(size_of::<clone_args>() == CLONE_ARGS_SIZE_VER2 as usize);
    }
    if usize_ > RUST_FORK_PAGE_SIZE as usize {
        return -(E2BIG as c_int);
    }
    if usize_ < CLONE_ARGS_SIZE_VER0 as usize {
        return -(EINVAL as c_int);
    }
    let err = fork_copy_struct_from_user(
        (&mut args as *mut clone_args).cast(),
        size_of::<clone_args>(),
        uargs.cast(),
        usize_,
        size_of::<clone_args>(),
    );
    if err != 0 {
        return err;
    }
    if args.set_tid_size > MAX_PID_NS_LEVEL as u64
        || (args.set_tid == 0 && args.set_tid_size > 0)
        || (args.set_tid != 0 && args.set_tid_size == 0)
        || args.exit_signal & !(CSIGNAL as u64) != 0
    {
        return -(EINVAL as c_int);
    }
    if args.flags & CLONE_INTO_CGROUP as u64 != 0
        && (args.cgroup > c_int::MAX as u64 || usize_ < CLONE_ARGS_SIZE_VER2 as usize)
    {
        return -(EINVAL as c_int);
    }
    *kargs = zeroed();
    (*kargs).flags = args.flags;
    (*kargs).pidfd = args.pidfd as usize as *mut c_int;
    (*kargs).child_tid = args.child_tid as usize as *mut c_int;
    (*kargs).parent_tid = args.parent_tid as usize as *mut c_int;
    (*kargs).exit_signal = args.exit_signal as c_int;
    (*kargs).stack = args.stack as c_ulong;
    (*kargs).stack_size = args.stack_size as c_ulong;
    (*kargs).tls = args.tls as c_ulong;
    (*kargs).set_tid_size = args.set_tid_size as usize;
    (*kargs).cgroup = args.cgroup as c_int;
    if args.set_tid != 0
        && rust_fork_copy_from_user(
            kset_tid.cast(),
            args.set_tid as usize as *const c_void,
            (*kargs).set_tid_size * size_of::<pid_t>(),
        ) != 0
    {
        return -(EFAULT as c_int);
    }
    (*kargs).set_tid = kset_tid;
    0
}
unsafe fn clone3_stack_valid(kargs: *mut kernel_clone_args) -> bool {
    if (*kargs).stack == 0 {
        if (*kargs).stack_size > 0 {
            return false;
        }
    } else {
        if (*kargs).stack_size == 0 {
            return false;
        }
        if !rust_fork_access_ok(
            (*kargs).stack as *const c_void,
            (*kargs).stack_size as usize,
        ) {
            return false;
        }
        #[cfg(not(CONFIG_STACK_GROWSUP))]
        {
            (*kargs).stack = (*kargs).stack.wrapping_add((*kargs).stack_size);
        }
    }
    true
}
unsafe fn clone3_args_valid(kargs: *mut kernel_clone_args) -> bool {
    let flags = (*kargs).flags;
    if flags
        & !(CLONE_LEGACY_FLAGS as u64
            | CLONE_CLEAR_SIGHAND
            | CLONE_INTO_CGROUP
            | CLONE_AUTOREAP
            | CLONE_NNP
            | CLONE_PIDFD_AUTOKILL
            | CLONE_EMPTY_MNTNS)
        != 0
    {
        return false;
    }
    if flags & (CLONE_DETACHED | (CSIGNAL & !CLONE_NEWTIME)) as u64 != 0 {
        return false;
    }
    if flags & (CLONE_SIGHAND as u64 | CLONE_CLEAR_SIGHAND)
        == CLONE_SIGHAND as u64 | CLONE_CLEAR_SIGHAND
    {
        return false;
    }
    if flags & (CLONE_THREAD | CLONE_PARENT) as u64 != 0 && (*kargs).exit_signal != 0 {
        return false;
    }
    clone3_stack_valid(kargs)
}
#[no_mangle]
pub unsafe extern "C" fn rust_fork_sys_clone3(uargs: *mut clone_args, size: usize) -> c_long {
    if RUST_FORK_ARCH_BROKEN_SYS_CLONE3 != 0 {
        return -(ENOSYS as c_long);
    }
    let mut kargs: kernel_clone_args = zeroed();
    let mut set_tid = [0 as pid_t; MAX_PID_NS_LEVEL as usize];
    kargs.set_tid = set_tid.as_mut_ptr();
    let err = copy_clone_args_from_user(&mut kargs, uargs, size);
    if err != 0 {
        return err as c_long;
    }
    if !clone3_args_valid(&mut kargs) {
        return -(EINVAL as c_long);
    }
    kernel_clone(&mut kargs) as c_long
}
