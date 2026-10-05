// SPDX-License-Identifier: GPL-2.0
// psi.c task-state owner, including IRQ accounting and cgroup lifecycle.

/// # Safety
/// The count array has COUNTS initialized entries under the native runqueue lock.
unsafe fn test_states(tasks: *const c_uint, mut mask: u32) -> u32 {
    unsafe {
        let oncpu = mask & ONCPU != 0;
        let running = *tasks.add(b::RUST_PSI_NR_RUNNING as usize);
        let iowait = *tasks.add(b::RUST_PSI_NR_IOWAIT as usize);
        let memstall = *tasks.add(b::RUST_PSI_NR_MEMSTALL as usize);
        if iowait != 0 {
            mask |= 1 << b::RUST_PSI_IO_SOME;
            if running == 0 { mask |= 1 << b::RUST_PSI_IO_FULL; }
        }
        if memstall != 0 {
            mask |= 1 << b::RUST_PSI_MEM_SOME;
            if running == *tasks.add(b::RUST_PSI_NR_MEMSTALL_RUNNING as usize) {
                mask |= 1 << b::RUST_PSI_MEM_FULL;
            }
        }
        if running > oncpu as c_uint { mask |= 1 << b::RUST_PSI_CPU_SOME; }
        if running != 0 && !oncpu { mask |= 1 << b::RUST_PSI_CPU_FULL; }
        if iowait != 0 || memstall != 0 || running != 0 { mask |= 1 << NONIDLE; }
        mask
    }
}

/// # Safety
/// Caller holds this CPU's rq lock and psi_seq write section.
unsafe fn record_times(groupc: *mut b::psi_group_cpu, now: u64) {
    unsafe {
        let delta = now.wrapping_sub((*groupc).state_start) as u32;
        (*groupc).state_start = now;
        for (some, full) in [(b::RUST_PSI_IO_SOME, b::RUST_PSI_IO_FULL),
                             (b::RUST_PSI_MEM_SOME, b::RUST_PSI_MEM_FULL),
                             (b::RUST_PSI_CPU_SOME, b::RUST_PSI_CPU_FULL)] {
            if (*groupc).state_mask & (1u32 << some) != 0 {
                (*groupc).times[some as usize] = (*groupc).times[some as usize].wrapping_add(delta);
                if (*groupc).state_mask & (1u32 << full) != 0 {
                    (*groupc).times[full as usize] = (*groupc).times[full as usize].wrapping_add(delta);
                }
            }
        }
        if (*groupc).state_mask & (1u32 << NONIDLE) != 0 {
            (*groupc).times[NONIDLE] = (*groupc).times[NONIDLE].wrapping_add(delta);
        }
    }
}

/// # Safety
/// Native rq lock and psi_seq write section cover all per-CPU state changes.
/// Masks must use only native TSK_* bits and group storage must remain live.
unsafe fn psi_group_change(group: *mut b::psi_group, cpu: c_int, mut clear: c_uint,
                           mut set: c_uint, now: u64, wake_clock: bool) {
    unsafe {
        b::rust_psi_assert_rq_held(b::rust_psi_cpu_rq(cpu));
        let groupc = b::rust_psi_group_cpu(group, cpu);
        let mut mask;
        if clear & TSK_ONCPU != 0 {
            mask = 0;
            clear &= !TSK_ONCPU;
        } else if set & TSK_ONCPU != 0 {
            mask = ONCPU;
            set &= !TSK_ONCPU;
        } else {
            mask = (*groupc).state_mask & ONCPU;
        }
        let mut remaining = clear;
        let mut t = 0usize;
        while remaining != 0 {
            if remaining & (1u32 << t) != 0 {
                if (*groupc).tasks[t] != 0 {
                    (*groupc).tasks[t] -= 1;
                } else if b::rust_psi_bug == 0 {
                    b::rust_psi_report_underflow(cpu, t as c_uint, groupc, clear, set);
                    b::rust_psi_bug = 1;
                }
            }
            remaining &= !(1u32 << t);
            t += 1;
        }
        t = 0;
        while set != 0 {
            if set & (1u32 << t) != 0 {
                (*groupc).tasks[t] = (*groupc).tasks[t].wrapping_add(1);
            }
            set &= !(1u32 << t);
            t += 1;
        }
        if !(*group).enabled {
            if (*groupc).state_mask & (1u32 << NONIDLE) != 0 { record_times(groupc, now); }
            (*groupc).state_mask = mask;
            return;
        }
        mask = test_states(addr_of!((*groupc).tasks).cast(), mask);
        if mask & ONCPU != 0 && b::rust_psi_in_memstall(b::rust_psi_cpu_curr(cpu)) {
            mask |= 1 << b::RUST_PSI_MEM_FULL;
        }
        record_times(groupc, now);
        (*groupc).state_mask = mask;
        if mask & (*group).rtpoll_states != 0 { psi_schedule_rtpoll_work(group, 1, false); }
        if wake_clock && !b::rust_psi_delayed_pending(addr_of_mut!((*group).avgs_work)) {
            b::rust_psi_schedule_delayed(addr_of_mut!((*group).avgs_work), FREQ);
        }
    }
}

/// # Safety
/// Task and its default cgroup hierarchy remain protected by native scheduler/RCU.
unsafe fn task_psi_group(task: *mut b::task_struct) -> *mut b::psi_group {
    unsafe {
        #[cfg(CONFIG_CGROUPS)]
        if b::rust_psi_cgroups_enabled() {
            return b::rust_psi_cgroup_psi(b::rust_psi_task_dfl_cgroup(task));
        }
        #[cfg(not(CONFIG_CGROUPS))]
        let _ = task;
        addr_of_mut!(b::psi_system)
    }
}

/// # Safety
/// Task psi_flags are stabilized by the task's runqueue lock.
unsafe fn psi_flags_change(task: *mut b::task_struct, clear: c_int, set: c_int) {
    unsafe {
        if (((*task).psi_flags & set as c_uint) != 0 ||
            ((*task).psi_flags & clear as c_uint) != clear as c_uint) && b::rust_psi_bug == 0 {
            b::rust_psi_report_flags(task, clear, set);
            b::rust_psi_bug = 1;
        }
        (*task).psi_flags &= !(clear as c_uint);
        (*task).psi_flags |= set as c_uint;
    }
}

/// # Safety
/// Caller holds the task's native rq lock, IRQ discipline and cgroup lifetime.
#[no_mangle]
pub unsafe extern "C" fn psi_task_change(task: *mut b::task_struct, clear: c_int, set: c_int) {
    unsafe {
        let cpu = b::rust_psi_task_cpu(task);
        if (*task).pid == 0 { return; }
        psi_flags_change(task, clear, set);
        b::rust_psi_write_begin(cpu);
        let now = b::rust_psi_cpu_clock(cpu);
        let mut group = task_psi_group(task);
        while !group.is_null() {
            psi_group_change(group, cpu, clear as c_uint, set as c_uint, now, true);
            group = (*group).parent;
        }
        b::rust_psi_write_end(cpu);
    }
}

/// # Safety
/// Scheduler passes live previous/next tasks on the same locked runqueue.
#[no_mangle]
pub unsafe extern "C" fn psi_task_switch(prev: *mut b::task_struct, next: *mut b::task_struct, sleep: bool) {
    unsafe {
        let mut common = null_mut();
        let cpu = b::rust_psi_task_cpu(prev);
        b::rust_psi_write_begin(cpu);
        let now = b::rust_psi_cpu_clock(cpu);
        if (*next).pid != 0 {
            psi_flags_change(next, 0, TSK_ONCPU as c_int);
            let mut group = task_psi_group(next);
            while !group.is_null() {
                if (*b::rust_psi_group_cpu(group, cpu)).state_mask & ONCPU != 0 {
                    common = group;
                    break;
                }
                psi_group_change(group, cpu, 0, TSK_ONCPU, now, true);
                group = (*group).parent;
            }
        }
        if (*prev).pid != 0 {
            let mut clear = TSK_ONCPU;
            let mut set = 0;
            let mut wake_clock = true;
            if sleep {
                clear |= b::RUST_PSI_TSK_RUNNING as c_uint;
                if b::rust_psi_in_memstall(prev) { clear |= b::RUST_PSI_TSK_MEMSTALL_RUNNING as c_uint; }
                if b::rust_psi_in_iowait(prev) { set |= b::RUST_PSI_TSK_IOWAIT as c_uint; }
                if ((*prev).flags & b::RUST_PSI_PF_WQ_WORKER as c_uint != 0) && b::rust_psi_worker_last_is_avgs(prev) { wake_clock = false; }
            }
            psi_flags_change(prev, clear as c_int, set as c_int);
            let mut group = task_psi_group(prev);
            while !group.is_null() && group != common {
                psi_group_change(group, cpu, clear, set, now, wake_clock);
                group = (*group).parent;
            }
            if ((*prev).psi_flags ^ (*next).psi_flags) & !TSK_ONCPU != 0 {
                clear &= !TSK_ONCPU;
                group = common;
                while !group.is_null() {
                    psi_group_change(group, cpu, clear, set, now, wake_clock);
                    group = (*group).parent;
                }
            }
        }
        b::rust_psi_write_end(cpu);
    }
}

/// # Safety
/// Caller holds rq; curr/prev and their cgroup ancestors are live and stabilized.
#[cfg(CONFIG_IRQ_TIME_ACCOUNTING)]
#[no_mangle]
pub unsafe extern "C" fn psi_account_irqtime(rq: *mut b::rq, curr: *mut b::task_struct, prev: *mut b::task_struct) {
    unsafe {
        let cpu = b::rust_psi_task_cpu(curr);
        if b::rust_psi_disabled() || !b::rust_psi_irqtime_enabled() || (*curr).pid == 0 { return; }
        b::rust_psi_assert_rq_held(rq);
        if !prev.is_null() && task_psi_group(prev) == task_psi_group(curr) { return; }
        let irq = b::rust_psi_irq_time_read(cpu);
        let delta = irq.wrapping_sub((*rq).psi_irq_time) as i64;
        if delta <= 0 { return; }
        (*rq).psi_irq_time = irq;
        b::rust_psi_write_begin(cpu);
        let now = b::rust_psi_cpu_clock(cpu);
        let mut group = task_psi_group(curr);
        while !group.is_null() {
            if (*group).enabled {
                let groupc = b::rust_psi_group_cpu(group, cpu);
                record_times(groupc, now);
                let state = b::RUST_PSI_IRQ_FULL as usize;
                (*groupc).times[state] = (*groupc).times[state].wrapping_add(delta as u32);
                if (*group).rtpoll_states & (1u32 << state) != 0 { psi_schedule_rtpoll_work(group, 1, false); }
            }
            group = (*group).parent;
        }
        b::rust_psi_write_end(cpu);
    }
}

/// # Safety
/// Flags points to caller-owned nesting storage; called in a native task context.
#[no_mangle]
pub unsafe extern "C" fn psi_memstall_enter(flags: *mut c_ulong) {
    unsafe {
        if b::rust_psi_disabled() { return; }
        let current = b::rust_psi_current();
        *flags = b::rust_psi_in_memstall(current) as c_ulong;
        if *flags != 0 { return; }
        let mut rf = MaybeUninit::<b::rq_flags>::uninit();
        let rq = b::rust_psi_this_rq_lock_irq(rf.as_mut_ptr());
        b::rust_psi_set_memstall(current, true);
        psi_task_change(current, 0, (b::RUST_PSI_TSK_MEMSTALL | b::RUST_PSI_TSK_MEMSTALL_RUNNING) as c_int);
        b::rust_psi_rq_unlock_irq(rq, rf.as_mut_ptr());
    }
}

/// # Safety
/// Flags is the live nesting value returned by the matching enter on this task.
#[no_mangle]
pub unsafe extern "C" fn psi_memstall_leave(flags: *mut c_ulong) {
    unsafe {
        if b::rust_psi_disabled() || *flags != 0 { return; }
        let mut rf = MaybeUninit::<b::rq_flags>::uninit();
        let rq = b::rust_psi_this_rq_lock_irq(rf.as_mut_ptr());
        let current = b::rust_psi_current();
        b::rust_psi_set_memstall(current, false);
        psi_task_change(current, (b::RUST_PSI_TSK_MEMSTALL | b::RUST_PSI_TSK_MEMSTALL_RUNNING) as c_int, 0);
        b::rust_psi_rq_unlock_irq(rq, rf.as_mut_ptr());
    }
}

/// # Safety
/// Cgroup creation serializes this unpublished cgroup and keeps its parent live.
#[cfg(CONFIG_CGROUPS)]
#[no_mangle]
pub unsafe extern "C" fn psi_cgroup_alloc(cgroup: *mut b::cgroup) -> c_int {
    unsafe {
        if !b::rust_psi_cgroups_enabled() { return 0; }
        (*cgroup).psi = b::rust_psi_alloc_group();
        let group = (*cgroup).psi;
        if group.is_null() { return -(b::RUST_PSI_ENOMEM as c_int); }
        (*group).pcpu = b::rust_psi_alloc_pcpu();
        if (*group).pcpu.is_null() {
            b::rust_psi_free(group.cast());
            return -(b::RUST_PSI_ENOMEM as c_int);
        }
        group_init(group);
        (*group).parent = b::rust_psi_cgroup_psi(b::rust_psi_cgroup_parent(cgroup));
        0
    }
}

/// # Safety
/// Cgroup teardown has removed every trigger and quiesced task accounting.
#[cfg(CONFIG_CGROUPS)]
#[no_mangle]
pub unsafe extern "C" fn psi_cgroup_free(cgroup: *mut b::cgroup) {
    unsafe {
        if !b::rust_psi_cgroups_enabled() { return; }
        let group = (*cgroup).psi;
        b::rust_psi_cancel_delayed_sync(addr_of_mut!((*group).avgs_work));
        b::rust_psi_timer_shutdown_sync(addr_of_mut!((*group).rtpoll_timer));
        b::rust_psi_free_pcpu((*group).pcpu);
        b::rust_psi_warn_trigger_leak((*group).rtpoll_states != 0);
        b::rust_psi_free(group.cast());
    }
}

/// # Safety
/// Caller holds cgroup migration lifetime protection for task and css_set.
#[cfg(CONFIG_CGROUPS)]
#[no_mangle]
pub unsafe extern "C" fn cgroup_move_task(task: *mut b::task_struct, to: *mut b::css_set) {
    unsafe {
        if !b::rust_psi_cgroups_enabled() {
            b::rust_psi_assign_cgroups(task, to);
            return;
        }
        let mut rf = MaybeUninit::<b::rq_flags>::uninit();
        let rq = b::rust_psi_task_rq_lock(task, rf.as_mut_ptr());
        let flags = (*task).psi_flags;
        if flags != 0 { psi_task_change(task, flags as c_int, 0); }
        b::rust_psi_assign_cgroups(task, to);
        if flags != 0 { psi_task_change(task, 0, flags as c_int); }
        b::rust_psi_task_rq_unlock(rq, task, rf.as_mut_ptr());
    }
}

/// # Safety
/// Caller serializes enable-state changes and keeps the group allocated.
#[cfg(CONFIG_CGROUPS)]
#[no_mangle]
pub unsafe extern "C" fn psi_cgroup_restart(group: *mut b::psi_group) {
    unsafe {
        if !(*group).enabled { return; }
        each_possible_cpu(|cpu| {
            let rq = b::rust_psi_cpu_rq(cpu);
            let mut rf = MaybeUninit::<b::rq_flags>::uninit();
            b::rust_psi_rq_lock_irq(rq, rf.as_mut_ptr());
            b::rust_psi_write_begin(cpu);
            let now = b::rust_psi_cpu_clock(cpu);
            psi_group_change(group, cpu, 0, 0, now, true);
            b::rust_psi_write_end(cpu);
            b::rust_psi_rq_unlock_irq(rq, rf.as_mut_ptr());
        });
    }
}
