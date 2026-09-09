// SPDX-License-Identifier: GPL-2.0
// Translation of linux/fs/proc/array.c. Kernel includes and external symbols
// are supplied by the surrounding repository.

// use crate::kernel::*;

pub unsafe fn proc_task_name(m: *mut seq_file, p: *mut task_struct, escape: bool) {
    let mut tcomm = [0i8; 64];
    if (*p).flags & PF_WQ_WORKER != 0 {
        wq_worker_comm(tcomm.as_mut_ptr(), tcomm.len(), p);
    } else if (*p).flags & PF_KTHREAD != 0 {
        get_kthread_comm(tcomm.as_mut_ptr(), tcomm.len(), p);
    } else {
        get_task_comm(tcomm.as_mut_ptr(), p);
    }
    if escape {
        seq_escape_str(m, tcomm.as_ptr(), ESCAPE_SPACE | ESCAPE_SPECIAL, "\n\\");
    } else {
        seq_printf(m, "%.64s", tcomm.as_ptr());
    }
}

// The task state array is a bitmap of reasons to sleep; running is zero.
static TASK_STATE_ARRAY: [&[u8]; 9] = [
    b"R (running)\0", b"S (sleeping)\0", b"D (disk sleep)\0",
    b"T (stopped)\0", b"t (tracing stop)\0", b"X (dead)\0",
    b"Z (zombie)\0", b"P (parked)\0", b"I (idle)\0",
];

unsafe fn get_task_state(tsk: *mut task_struct) -> *const u8 {
    TASK_STATE_ARRAY[task_state_index(tsk)] .as_ptr()
}

unsafe fn task_state(m: *mut seq_file, ns: *mut pid_namespace, pid: *mut pid, p: *mut task_struct) {
    let user_ns = seq_user_ns(m);
    let mut group_info: *mut group_info;
    let mut g: i32;
    let mut umask: i32 = -1;
    let mut tracer: *mut task_struct;
    let cred: *const cred;
    let ppid: pid_t;
    let mut tpid: pid_t = 0;
    let tgid: pid_t;
    let ngid: pid_t;
    let mut max_fds: u32 = 0;

    rcu_read_lock();
    tracer = ptrace_parent(p);
    if !tracer.is_null() { tpid = task_pid_nr_ns(tracer, ns); }
    ppid = task_ppid_nr_ns(p, ns);
    tgid = task_tgid_nr_ns(p, ns);
    ngid = task_numa_group_id(p);
    cred = get_task_cred(p);
    task_lock(p);
    if !(*p).real_fs.is_null() { umask = (*(*p).real_fs).umask; }
    if !(*p).files.is_null() { max_fds = (*files_fdtable((*p).files)).max_fds; }
    task_unlock(p);
    rcu_read_unlock();
    if umask >= 0 { seq_printf(m, "Umask:\t%#04o\n", umask); }
    seq_puts(m, "State:\t"); seq_puts(m, get_task_state(p));
    seq_put_decimal_ull(m, "\nTgid:\t", tgid); seq_put_decimal_ull(m, "\nNgid:\t", ngid);
    seq_put_decimal_ull(m, "\nPid:\t", pid_nr_ns(pid, ns)); seq_put_decimal_ull(m, "\nPPid:\t", ppid);
    seq_put_decimal_ull(m, "\nTracerPid:\t", tpid);
    seq_put_decimal_ull(m, "\nUid:\t", from_kuid_munged(user_ns, (*cred).uid));
    seq_put_decimal_ull(m, "\t", from_kuid_munged(user_ns, (*cred).euid));
    seq_put_decimal_ull(m, "\t", from_kuid_munged(user_ns, (*cred).suid));
    seq_put_decimal_ull(m, "\t", from_kuid_munged(user_ns, (*cred).fsuid));
    seq_put_decimal_ull(m, "\nGid:\t", from_kgid_munged(user_ns, (*cred).gid));
    seq_put_decimal_ull(m, "\t", from_kgid_munged(user_ns, (*cred).egid));
    seq_put_decimal_ull(m, "\t", from_kgid_munged(user_ns, (*cred).sgid));
    seq_put_decimal_ull(m, "\t", from_kgid_munged(user_ns, (*cred).fsgid));
    seq_put_decimal_ull(m, "\nFDSize:\t", max_fds); seq_puts(m, "\nGroups:\t");
    group_info = (*cred).group_info;
    g = 0; while g < (*group_info).ngroups { seq_put_decimal_ull(m, if g != 0 { " " } else { "" }, from_kgid_munged(user_ns, (*group_info).gid[g as usize])); g += 1; }
    put_cred(cred); seq_putc(m, ' ');
    // CONFIG_PID_NS: namespace pid fields are emitted when enabled.
    #[cfg(CONFIG_PID_NS)] {
        seq_puts(m, "\nNStgid:"); g = (*ns).level; while g <= (*pid).level { seq_put_decimal_ull(m, "\t", task_tgid_nr_ns(p, (*pid).numbers[g as usize].ns)); g += 1; }
        seq_puts(m, "\nNSpid:"); g = (*ns).level; while g <= (*pid).level { seq_put_decimal_ull(m, "\t", task_pid_nr_ns(p, (*pid).numbers[g as usize].ns)); g += 1; }
        seq_puts(m, "\nNSpgid:"); g = (*ns).level; while g <= (*pid).level { seq_put_decimal_ull(m, "\t", task_pgrp_nr_ns(p, (*pid).numbers[g as usize].ns)); g += 1; }
        seq_puts(m, "\nNSsid:"); g = (*ns).level; while g <= (*pid).level { seq_put_decimal_ull(m, "\t", task_session_nr_ns(p, (*pid).numbers[g as usize].ns)); g += 1; }
    }
    seq_putc(m, '\n'); seq_printf(m, "Kthread:\t%c\n", if (*p).flags & PF_KTHREAD != 0 { '1' } else { '0' });
}

pub unsafe fn render_sigset_t(m: *mut seq_file, header: *const i8, set: *mut sigset_t) {
    seq_puts(m, header); let mut i = _NSIG; loop { let mut x = 0; i -= 4;
        if sigismember(set, i + 1) != 0 { x |= 1; } if sigismember(set, i + 2) != 0 { x |= 2; }
        if sigismember(set, i + 3) != 0 { x |= 4; } if sigismember(set, i + 4) != 0 { x |= 8; }
        seq_putc(m, hex_asc[x as usize] as char); if i < 4 { break; }
    } seq_putc(m, '\n');
}

unsafe fn collect_sigign_sigcatch(p: *mut task_struct, sigign: *mut sigset_t, sigcatch: *mut sigset_t) {
    let mut k = (*p).sighand.as_ref().unwrap().action; let mut i = 1;
    while i <= _NSIG { if (*k).sa.sa_handler == SIG_IGN { sigaddset(sigign, i); } else if (*k).sa.sa_handler != SIG_DFL { sigaddset(sigcatch, i); } k = k.add(1); i += 1; }
}

unsafe fn task_sig(m: *mut seq_file, p: *mut task_struct) {
    let mut pending = core::mem::zeroed::<sigset_t>(); let mut shpending = core::mem::zeroed::<sigset_t>();
    let mut blocked = core::mem::zeroed::<sigset_t>(); let mut ignored = core::mem::zeroed::<sigset_t>(); let mut caught = core::mem::zeroed::<sigset_t>();
    sigemptyset(&mut pending); sigemptyset(&mut shpending); sigemptyset(&mut blocked); sigemptyset(&mut ignored); sigemptyset(&mut caught);
    let mut flags = 0; let mut threads = 0; let mut qsize = 0; let mut qlim = 0;
    if lock_task_sighand(p, &mut flags) != 0 { pending = (*p).pending.signal; shpending = (*(*p).signal).shared_pending.signal; blocked = (*p).blocked; collect_sigign_sigcatch(p, &mut ignored, &mut caught); threads = get_nr_threads(p); qsize = get_rlimit_value(task_ucounts(p), UCOUNT_RLIMIT_SIGPENDING); qlim = task_rlimit(p, RLIMIT_SIGPENDING); unlock_task_sighand(p, &mut flags); }
    seq_put_decimal_ull(m, "Threads:\t", threads); seq_put_decimal_ull(m, "\nSigQ:\t", qsize); seq_put_decimal_ull(m, "/", qlim);
    render_sigset_t(m, "\nSigPnd:\t", &mut pending); render_sigset_t(m, "ShdPnd:\t", &mut shpending); render_sigset_t(m, "SigBlk:\t", &mut blocked); render_sigset_t(m, "SigIgn:\t", &mut ignored); render_sigset_t(m, "SigCgt:\t", &mut caught);
}

unsafe fn task_cap(m: *mut seq_file, p: *mut task_struct) { let c = __task_cred(p); render_cap_t(m, "CapInh:\t", &(*c).cap_inheritable); render_cap_t(m, "CapPrm:\t", &(*c).cap_permitted); render_cap_t(m, "CapEff:\t", &(*c).cap_effective); render_cap_t(m, "CapBnd:\t", &(*c).cap_bset); render_cap_t(m, "CapAmb:\t", &(*c).cap_ambient); }
unsafe fn render_cap_t(m: *mut seq_file, h: *const i8, a: *const kernel_cap_t) { seq_puts(m, h); seq_put_hex_ll(m, core::ptr::null(), (*a).val, 16); seq_putc(m, '\n'); }

pub unsafe fn proc_pid_status(m: *mut seq_file, ns: *mut pid_namespace, pid: *mut pid, task: *mut task_struct) -> i32 {
    let mm = get_task_mm(task); seq_puts(m, "Name:\t"); proc_task_name(m, task, true); seq_putc(m, '\n'); task_state(m, ns, pid, task);
    if !mm.is_null() { task_mem(m, mm); task_core_dumping(m, task); task_thp_status(m, mm); task_untag_mask(m, mm); mmput(mm); }
    task_sig(m, task); task_cap(m, task); task_seccomp(m, task); task_cpus_allowed(m, task); cpuset_task_status_allowed(m, task); task_context_switch_counts(m, task); arch_proc_pid_thread_features(m, task); 0
}

pub unsafe fn proc_pid_statm(m: *mut seq_file, _ns: *mut pid_namespace, _pid: *mut pid, task: *mut task_struct) -> i32 { let mm = get_task_mm(task); if !mm.is_null() { let mut shared=0; let mut text=0; let mut data=0; let mut resident=0; let size=task_statm(mm,&mut shared,&mut text,&mut data,&mut resident); mmput(mm); for (s,v) in [("",size),(" ",resident),(" ",shared),(" ",text),(" ",0),(" ",data),(" ",0)] { seq_put_decimal_ull(m,s,v); } seq_putc(m,'\n'); } else { seq_write(m,"0 0 0 0 0 0 0\n",14); } 0 }

pub unsafe fn proc_tid_stat(m:*mut seq_file,ns:*mut pid_namespace,pid:*mut pid,task:*mut task_struct)->i32 { do_task_stat(m,ns,pid,task,0) }
pub unsafe fn proc_tgid_stat(m:*mut seq_file,ns:*mut pid_namespace,pid:*mut pid,task:*mut task_struct)->i32 { do_task_stat(m,ns,pid,task,1) }
extern "C" { fn do_task_stat(m:*mut seq_file,ns:*mut pid_namespace,pid:*mut pid,task:*mut task_struct,whole:i32)->i32; fn task_seccomp(_: *mut seq_file,_:*mut task_struct); fn task_cpus_allowed(_: *mut seq_file,_:*mut task_struct); fn task_context_switch_counts(_: *mut seq_file,_:*mut task_struct); fn task_core_dumping(_: *mut seq_file,_:*mut task_struct); fn task_thp_status(_: *mut seq_file,_:*mut mm_struct); fn task_untag_mask(_: *mut seq_file,_:*mut mm_struct); fn arch_proc_pid_thread_features(_: *mut seq_file,_:*mut task_struct); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
