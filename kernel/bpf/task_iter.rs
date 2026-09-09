// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2020 Facebook */

// Linux kernel dependencies supplied by other translation units.

static ITER_TASK_TYPE_NAMES: [&[u8]; 3] = [b"ALL\0", b"TID\0", b"PID\0"];

#[repr(C)]
pub struct BpfIterSeqTaskCommon {
    pub ns: *mut pid_namespace,
    pub type_: bpf_iter_task_type,
    pub pid: u32,
    pub pid_visiting: u32,
}

#[repr(C)]
pub struct BpfIterSeqTaskInfo {
    // The first field must be struct bpf_iter_seq_task_common.
    // this is assumed by {init, fini}_seq_pidns() callback functions.
    pub common: BpfIterSeqTaskCommon,
    pub tid: u32,
}

unsafe fn task_group_seq_get_next(
    common: *mut BpfIterSeqTaskCommon,
    tid: *mut u32,
    skip_if_dup_files: bool,
) -> *mut task_struct {
    let mut task: *mut task_struct;
    let mut pid: *mut pid;
    let mut next_tid: u32;

    if unsafe { *tid } == 0 {
        // The first time, the iterator calls this function.
        pid = unsafe { find_pid_ns((*common).pid, (*common).ns) };
        task = unsafe { get_pid_task(pid, PIDTYPE_TGID) };
        if task.is_null() {
            return core::ptr::null_mut();
        }

        unsafe {
            *tid = (*common).pid;
            (*common).pid_visiting = (*common).pid;
        }
        return task;
    }

    // If the control returns to user space and comes back to the
    // kernel again, *tid and common->pid_visiting should be the
    // same for task_seq_start() to pick up the correct task.
    if unsafe { *tid == (*common).pid_visiting } {
        pid = unsafe { find_pid_ns((*common).pid_visiting, (*common).ns) };
        task = unsafe { get_pid_task(pid, PIDTYPE_PID) };
        return task;
    }

    task = unsafe { find_task_by_pid_ns((*common).pid_visiting, (*common).ns) };
    if task.is_null() {
        return core::ptr::null_mut();
    }

    loop {
        task = unsafe { __next_thread(task) };
        if task.is_null() {
            return core::ptr::null_mut();
        }

        next_tid = unsafe { __task_pid_nr_ns(task, PIDTYPE_PID, (*common).ns) };
        if next_tid == 0 {
            continue;
        }

        if skip_if_dup_files && unsafe { (*task).files == (*(*task).group_leader).files } {
            continue;
        }
        unsafe {
            *tid = next_tid;
            (*common).pid_visiting = next_tid;
            get_task_struct(task);
        }
        return task;
    }
}

unsafe fn task_seq_get_next(
    common: *mut BpfIterSeqTaskCommon,
    tid: *mut u32,
    skip_if_dup_files: bool,
) -> *mut task_struct {
    let mut task: *mut task_struct = core::ptr::null_mut();
    let mut pid: *mut pid;

    if unsafe { (*common).type_ == BPF_TASK_ITER_TID } {
        if unsafe { *tid != 0 && *tid != (*common).pid } {
            return core::ptr::null_mut();
        }
        unsafe { rcu_read_lock(); }
        pid = unsafe { find_pid_ns((*common).pid, (*common).ns) };
        if !pid.is_null() {
            task = unsafe { get_pid_task(pid, PIDTYPE_PID) };
            unsafe { *tid = (*common).pid; }
        }
        unsafe { rcu_read_unlock(); }
        return task;
    }

    if unsafe { (*common).type_ == BPF_TASK_ITER_TGID } {
        unsafe { rcu_read_lock(); }
        task = unsafe { task_group_seq_get_next(common, tid, skip_if_dup_files) };
        unsafe { rcu_read_unlock(); }
        return task;
    }

    unsafe { rcu_read_lock(); }
    loop {
        pid = unsafe { find_ge_pid(*tid, (*common).ns) };
        if pid.is_null() {
            break;
        }
        unsafe { *tid = pid_nr_ns(pid, (*common).ns); }
        task = unsafe { get_pid_task(pid, PIDTYPE_PID) };
        if task.is_null() {
            unsafe { *tid = (*tid).wrapping_add(1); }
            continue;
        }
        if skip_if_dup_files
            && unsafe { !thread_group_leader(task) && (*task).files == (*(*task).group_leader).files }
        {
            unsafe { put_task_struct(task); }
            task = core::ptr::null_mut();
            unsafe { *tid = (*tid).wrapping_add(1); }
            continue;
        }
        break;
    }
    unsafe { rcu_read_unlock(); }
    task
}

unsafe fn task_seq_start(seq: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void {
    let info = unsafe { (*seq).private as *mut BpfIterSeqTaskInfo };
    let task = unsafe { task_seq_get_next(&mut (*info).common, &mut (*info).tid, false) };
    if task.is_null() {
        return core::ptr::null_mut();
    }
    if unsafe { *pos == 0 } {
        unsafe { *pos += 1; }
    }
    task.cast()
}

unsafe fn task_seq_next(
    seq: *mut seq_file,
    v: *mut core::ffi::c_void,
    pos: *mut loff_t,
) -> *mut core::ffi::c_void {
    let info = unsafe { (*seq).private as *mut BpfIterSeqTaskInfo };
    unsafe {
        *pos += 1;
        (*info).tid += 1;
        put_task_struct(v.cast::<task_struct>());
    }
    unsafe { task_seq_get_next(&mut (*info).common, &mut (*info).tid, false).cast() }
}

#[repr(C)]
pub struct BpfIterTask {
    pub meta: *mut bpf_iter_meta,
    pub task: *mut task_struct,
}

// DEFINE_BPF_ITER_FUNC(task, struct bpf_iter_meta *meta, struct task_struct *task)

unsafe fn __task_seq_show(
    seq: *mut seq_file,
    task: *mut task_struct,
    in_stop: bool,
) -> i32 {
    let mut meta: bpf_iter_meta = core::mem::zeroed();
    let mut ctx: BpfIterTask = core::mem::zeroed();
    meta.seq = seq;
    let prog = bpf_iter_get_info(&mut meta, in_stop);
    if prog.is_null() {
        return 0;
    }
    ctx.meta = &mut meta;
    ctx.task = task;
    bpf_iter_run_prog(prog, &mut ctx)
}

unsafe fn task_seq_show(seq: *mut seq_file, v: *mut core::ffi::c_void) -> i32 {
    __task_seq_show(seq, v.cast(), false)
}

unsafe fn task_seq_stop(seq: *mut seq_file, v: *mut core::ffi::c_void) {
    if v.is_null() {
        let _ = __task_seq_show(seq, v.cast(), true);
    } else {
        put_task_struct(v.cast::<task_struct>());
    }
}

unsafe fn bpf_iter_attach_task(
    _prog: *mut bpf_prog,
    linfo: *mut bpf_iter_link_info,
    aux: *mut bpf_iter_aux_info,
) -> i32 {
    let mut flags: u32;
    let mut pid: *mut pid;
    let tgid: pid_t;

    if unsafe {
        (((*linfo).task.tid != 0) as i32
            + ((*linfo).task.pid != 0) as i32
            + ((*linfo).task.pid_fd != 0) as i32)
            > 1
    } {
        return -EINVAL;
    }

    unsafe {
        (*aux).task.type_ = BPF_TASK_ITER_ALL;
        if (*linfo).task.tid != 0 {
            (*aux).task.type_ = BPF_TASK_ITER_TID;
            (*aux).task.pid = (*linfo).task.tid;
        }
        if (*linfo).task.pid != 0 {
            (*aux).task.type_ = BPF_TASK_ITER_TGID;
            (*aux).task.pid = (*linfo).task.pid;
        }
        if (*linfo).task.pid_fd != 0 {
            (*aux).task.type_ = BPF_TASK_ITER_TGID;
            pid = pidfd_get_pid((*linfo).task.pid_fd, &mut flags);
            if IS_ERR(pid) {
                return PTR_ERR(pid);
            }
            tgid = pid_nr_ns(pid, task_active_pid_ns(current));
            (*aux).task.pid = tgid;
            put_pid(pid);
        }
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
