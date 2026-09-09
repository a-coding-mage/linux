/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

#[repr(C)]
pub struct syscall_info {
    pub sp: __u64,
    pub data: seccomp_data,
}

extern "C" {
    pub fn ptracer_access_allowed(tsk: *mut task_struct) -> bool;
    pub fn ptrace_access_vm(tsk: *mut task_struct, addr: c_ulong, buf: *mut c_void,
                            len: c_int, gup_flags: c_uint) -> c_int;
    pub fn arch_ptrace(child: *mut task_struct, request: c_long, addr: c_ulong,
                       data: c_ulong) -> c_long;
    pub fn ptrace_readdata(tsk: *mut task_struct, src: c_ulong, dst: *mut c_char,
                           len: c_int) -> c_int;
    pub fn ptrace_writedata(tsk: *mut task_struct, src: *mut c_char, dst: c_ulong,
                            len: c_int) -> c_int;
    pub fn ptrace_disable(task: *mut task_struct);
    pub fn ptrace_request(child: *mut task_struct, request: c_long, addr: c_ulong,
                          data: c_ulong) -> c_int;
    pub fn ptrace_notify(exit_code: c_int, message: c_ulong) -> c_int;
    pub fn __ptrace_link(child: *mut task_struct, new_parent: *mut task_struct,
                         ptracer_cred: *const cred);
    pub fn __ptrace_unlink(child: *mut task_struct);
    pub fn exit_ptrace(tracer: *mut task_struct, dead: *mut list_head);
    pub fn ptrace_may_access(task: *mut task_struct, mode: c_uint) -> bool;
    pub fn generic_ptrace_peekdata(tsk: *mut task_struct, addr: c_ulong,
                                   data: c_ulong) -> c_int;
    pub fn generic_ptrace_pokedata(tsk: *mut task_struct, addr: c_ulong,
                                   data: c_ulong) -> c_int;
    pub fn task_current_syscall(target: *mut task_struct, info: *mut syscall_info) -> c_int;
    pub fn sigaction_compat_abi(act: *mut k_sigaction, oact: *mut k_sigaction);
}

pub const PT_SEIZED: c_ulong = 0x0001_0000;
pub const PT_PTRACED: c_ulong = 0x0000_0001;
pub const PT_OPT_FLAG_SHIFT: c_int = 3;
pub const PT_TRACESYSGOOD: c_ulong = 1 << PT_OPT_FLAG_SHIFT;
pub const PT_TRACE_FORK: c_ulong = 1 << (PT_OPT_FLAG_SHIFT + PTRACE_EVENT_FORK);
pub const PT_TRACE_VFORK: c_ulong = 1 << (PT_OPT_FLAG_SHIFT + PTRACE_EVENT_VFORK);
pub const PT_TRACE_CLONE: c_ulong = 1 << (PT_OPT_FLAG_SHIFT + PTRACE_EVENT_CLONE);
pub const PT_TRACE_EXEC: c_ulong = 1 << (PT_OPT_FLAG_SHIFT + PTRACE_EVENT_EXEC);
pub const PT_TRACE_VFORK_DONE: c_ulong = 1 << (PT_OPT_FLAG_SHIFT + PTRACE_EVENT_VFORK_DONE);
pub const PT_TRACE_EXIT: c_ulong = 1 << (PT_OPT_FLAG_SHIFT + PTRACE_EVENT_EXIT);
pub const PT_TRACE_SECCOMP: c_ulong = 1 << (PT_OPT_FLAG_SHIFT + PTRACE_EVENT_SECCOMP);
pub const PT_EXITKILL: c_ulong = PTRACE_O_EXITKILL << PT_OPT_FLAG_SHIFT;
pub const PT_SUSPEND_SECCOMP: c_ulong = PTRACE_O_SUSPEND_SECCOMP << PT_OPT_FLAG_SHIFT;

pub const PTRACE_MODE_READ: c_uint = 0x01;
pub const PTRACE_MODE_ATTACH: c_uint = 0x02;
pub const PTRACE_MODE_NOAUDIT: c_uint = 0x04;
pub const PTRACE_MODE_FSCREDS: c_uint = 0x08;
pub const PTRACE_MODE_REALCREDS: c_uint = 0x10;
pub const PTRACE_MODE_READ_FSCREDS: c_uint = PTRACE_MODE_READ | PTRACE_MODE_FSCREDS;
pub const PTRACE_MODE_READ_REALCREDS: c_uint = PTRACE_MODE_READ | PTRACE_MODE_REALCREDS;
pub const PTRACE_MODE_ATTACH_FSCREDS: c_uint = PTRACE_MODE_ATTACH | PTRACE_MODE_FSCREDS;
pub const PTRACE_MODE_ATTACH_REALCREDS: c_uint = PTRACE_MODE_ATTACH | PTRACE_MODE_REALCREDS;

pub unsafe fn ptrace_reparented(child: *mut task_struct) -> c_int {
    (!same_thread_group((*child).real_parent, (*child).parent)) as c_int
}

pub unsafe fn ptrace_unlink(child: *mut task_struct) {
    if (*child).ptrace != 0 { __ptrace_unlink(child); }
}

pub unsafe fn ptrace_parent(task: *mut task_struct) -> *mut task_struct {
    if (*task).ptrace != 0 { rcu_dereference((*task).parent) } else { core::ptr::null_mut() }
}

pub unsafe fn ptrace_event_enabled(task: *mut task_struct, event: c_int) -> bool {
    ((*task).ptrace & (1 << (PT_OPT_FLAG_SHIFT + event))) != 0
}

pub unsafe fn ptrace_event(event: c_int, message: c_ulong) {
    if ptrace_event_enabled(current, event) {
        ptrace_notify((event << 8) | SIGTRAP, message);
    } else if event == PTRACE_EVENT_EXEC && ((*current).ptrace & (PT_PTRACED | PT_SEIZED)) == PT_PTRACED {
        send_sig(SIGTRAP, current, 0);
    }
}

pub unsafe fn ptrace_event_pid(event: c_int, pid: *mut pid) {
    let mut message: c_ulong = 0;
    rcu_read_lock();
    let ns = task_active_pid_ns(rcu_dereference((*current).parent));
    if !ns.is_null() { message = pid_nr_ns(pid, ns); }
    rcu_read_unlock();
    ptrace_event(event, message);
}

pub unsafe fn ptrace_init_task(child: *mut task_struct, ptrace: bool) {
    INIT_LIST_HEAD(&mut (*child).ptrace_entry);
    INIT_LIST_HEAD(&mut (*child).ptraced);
    (*child).jobctl = 0;
    (*child).ptrace = 0;
    (*child).parent = (*child).real_parent;
    if ptrace && (*current).ptrace != 0 {
        (*child).ptrace = (*current).ptrace;
        __ptrace_link(child, (*current).parent, (*current).ptracer_cred);
        if (*child).ptrace & PT_SEIZED != 0 {
            task_set_jobctl_pending(child, JOBCTL_TRAP_STOP);
        } else {
            sigaddset(&mut (*child).pending.signal, SIGSTOP);
        }
    } else { (*child).ptracer_cred = core::ptr::null_mut(); }
}

pub unsafe fn ptrace_release_task(task: *mut task_struct) {
    BUG_ON(!list_empty(&(*task).ptraced));
    ptrace_unlink(task);
    BUG_ON(!list_empty(&(*task).ptrace_entry));
}

pub const fn arch_has_single_step() -> c_int { 0 }
pub unsafe fn user_enable_single_step(_task: *mut task_struct) { BUG!(); }
pub unsafe fn user_disable_single_step(_task: *mut task_struct) {}
pub const fn arch_has_block_step() -> c_int { 0 }
pub unsafe fn user_enable_block_step(_task: *mut task_struct) { BUG!(); }

pub unsafe fn user_single_step_report(_regs: *mut pt_regs) {
    let mut info: kernel_siginfo_t = core::mem::zeroed();
    clear_siginfo(&mut info);
    info.si_signo = SIGTRAP; info.si_errno = 0; info.si_code = SI_USER;
    info.si_pid = 0; info.si_uid = 0;
    force_sig_info(&mut info);
}

pub const fn arch_ptrace_stop_needed() -> c_int { 0 }
pub unsafe fn arch_ptrace_stop() {}

pub unsafe fn ptrace_report_syscall(message: c_ulong) -> bool {
    let ptrace = (*current).ptrace;
    if ptrace & PT_PTRACED == 0 { return true; }
    let signr = ptrace_notify(SIGTRAP | if ptrace & PT_TRACESYSGOOD != 0 { 0x80 } else { 0 }, message);
    if signr != 0 { send_sig(signr, current, 1); }
    !fatal_signal_pending(current)
}

pub unsafe fn ptrace_report_syscall_permit_entry(_regs: *mut pt_regs) -> bool {
    ptrace_report_syscall(PTRACE_EVENTMSG_SYSCALL_ENTRY)
}
pub unsafe fn ptrace_report_syscall_exit(regs: *mut pt_regs, step: c_int) {
    if step != 0 { user_single_step_report(regs); }
    else { ptrace_report_syscall(PTRACE_EVENTMSG_SYSCALL_EXIT); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
