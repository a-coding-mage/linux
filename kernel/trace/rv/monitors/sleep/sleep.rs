// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel/RV implementation are intentionally not reproduced here.

const MODULE_NAME: &str = "sleep";

unsafe fn ltl_atoms_fetch(task: *mut task_struct, mon: *mut ltl_monitor) {
    /*
     * This includes "actual" real-time tasks and also PI-boosted
     * tasks. A task being PI-boosted means it is blocking an "actual"
     * real-task, therefore it should also obey the monitor's rule,
     * otherwise the "actual" real-task may be delayed.
     */
    ltl_atom_set(mon, LTL_RT, rt_or_dl_task(task));
}

unsafe fn ltl_atoms_init(
    task: *mut task_struct,
    mon: *mut ltl_monitor,
    task_creation: bool,
) {
    ltl_atom_set(mon, LTL_SLEEP, false);
    ltl_atom_set(mon, LTL_SCHEDULE_IN, false);
    ltl_atom_set(mon, LTL_ABORT_SLEEP, false);
    ltl_atom_set(mon, LTL_WOKEN_BY_HARDIRQ, false);
    ltl_atom_set(mon, LTL_WOKEN_BY_NMI, false);
    ltl_atom_set(mon, LTL_WOKEN_BY_EQUAL_OR_HIGHER_PRIO, false);

    if task_creation {
        ltl_atom_set(mon, LTL_NANOSLEEP_CLOCK_REALTIME, false);
        ltl_atom_set(mon, LTL_NANOSLEEP_TIMER_ABSTIME, false);
        ltl_atom_set(mon, LTL_CLOCK_NANOSLEEP, false);
        ltl_atom_set(mon, LTL_FUTEX_WAIT, false);
        ltl_atom_set(mon, LTL_EPOLL_WAIT, false);
        ltl_atom_set(mon, LTL_FUTEX_LOCK_PI, false);
        ltl_atom_set(mon, LTL_BLOCK_ON_RT_MUTEX, false);
    }

    ltl_atom_set(mon, LTL_USER_THREAD, !((*task).flags & PF_KTHREAD) != 0);
}

unsafe fn handle_sched_set_state(_data: *mut core::ffi::c_void, task: *mut task_struct, state: i32) {
    if state & TASK_INTERRUPTIBLE != 0 {
        ltl_atom_pulse(task, LTL_SLEEP, true);
    } else if state == TASK_RUNNING {
        ltl_atom_pulse(task, LTL_ABORT_SLEEP, true);
    }
}

unsafe fn handle_sched_exit(_data: *mut core::ffi::c_void, _is_switch: bool) {
    ltl_atom_pulse(rv_get_current(), LTL_SCHEDULE_IN, true);
}

unsafe fn handle_sched_waking(_data: *mut core::ffi::c_void, task: *mut task_struct) {
    if in_hardirq() {
        ltl_atom_pulse(task, LTL_WOKEN_BY_HARDIRQ, true);
    } else if in_task() {
        if (*rv_get_current()).prio <= (*task).prio {
            ltl_atom_pulse(task, LTL_WOKEN_BY_EQUAL_OR_HIGHER_PRIO, true);
        }
    } else if in_nmi() {
        ltl_atom_pulse(task, LTL_WOKEN_BY_NMI, true);
    }
}

unsafe fn handle_contention_begin(
    _data: *mut core::ffi::c_void,
    _lock: *mut core::ffi::c_void,
    flags: u32,
) {
    if flags & LCB_F_RT != 0 {
        ltl_atom_update(rv_get_current(), LTL_BLOCK_ON_RT_MUTEX, true);
    }
}

unsafe fn handle_contention_end(
    _data: *mut core::ffi::c_void,
    _lock: *mut core::ffi::c_void,
    _ret: i32,
) {
    ltl_atom_update(rv_get_current(), LTL_BLOCK_ON_RT_MUTEX, false);
}

unsafe fn handle_sys_enter(
    _data: *mut core::ffi::c_void,
    regs: *mut pt_regs,
    id: i64,
) {
    let mon = ltl_get_monitor(rv_get_current());
    let mut args = [0usize; 6];
    let mut op: usize;
    let mut cmd: usize;

    match id {
        __NR_clock_nanosleep | __NR_clock_nanosleep_time64 => {
            syscall_get_arguments(rv_get_current(), regs, args.as_mut_ptr());
            ltl_atom_set(mon, LTL_NANOSLEEP_CLOCK_REALTIME, args[0] == CLOCK_REALTIME);
            ltl_atom_set(mon, LTL_NANOSLEEP_TIMER_ABSTIME, args[1] == TIMER_ABSTIME);
            ltl_atom_update(rv_get_current(), LTL_CLOCK_NANOSLEEP, true);
        }
        __NR_futex | __NR_futex_time64 => {
            syscall_get_arguments(rv_get_current(), regs, args.as_mut_ptr());
            op = args[1];
            cmd = op & FUTEX_CMD_MASK;
            match cmd {
                FUTEX_LOCK_PI | FUTEX_LOCK_PI2 => {
                    ltl_atom_update(rv_get_current(), LTL_FUTEX_LOCK_PI, true);
                }
                FUTEX_WAIT | FUTEX_WAIT_BITSET | FUTEX_WAIT_REQUEUE_PI => {
                    ltl_atom_update(rv_get_current(), LTL_FUTEX_WAIT, true);
                }
                _ => {}
            }
        }
        __NR_epoll_wait => {
            ltl_atom_update(rv_get_current(), LTL_EPOLL_WAIT, true);
        }
        _ => {}
    }
}

unsafe fn handle_sys_exit(
    _data: *mut core::ffi::c_void,
    _regs: *mut pt_regs,
    _ret: i64,
) {
    let mon = ltl_get_monitor(rv_get_current());
    ltl_atom_set(mon, LTL_FUTEX_LOCK_PI, false);
    ltl_atom_set(mon, LTL_FUTEX_WAIT, false);
    ltl_atom_set(mon, LTL_NANOSLEEP_CLOCK_REALTIME, false);
    ltl_atom_set(mon, LTL_NANOSLEEP_TIMER_ABSTIME, false);
    ltl_atom_set(mon, LTL_EPOLL_WAIT, false);
    ltl_atom_update(rv_get_current(), LTL_CLOCK_NANOSLEEP, false);
}

unsafe fn enable_sleep() -> i32 {
    let retval = ltl_monitor_init();
    if retval != 0 {
        return retval;
    }
    rv_attach_trace_probe("rtapp_sleep", sched_waking, handle_sched_waking);
    rv_attach_trace_probe("rtapp_sleep", sched_exit_tp, handle_sched_exit);
    rv_attach_trace_probe("rtapp_sleep", sched_set_state_tp, handle_sched_set_state);
    rv_attach_trace_probe("rtapp_sleep", contention_begin, handle_contention_begin);
    rv_attach_trace_probe("rtapp_sleep", contention_end, handle_contention_end);
    rv_attach_trace_probe("rtapp_sleep", sys_enter, handle_sys_enter);
    rv_attach_trace_probe("rtapp_sleep", sys_exit, handle_sys_exit);
    0
}

unsafe fn disable_sleep() {
    rv_detach_trace_probe("rtapp_sleep", sched_waking, handle_sched_waking);
    rv_detach_trace_probe("rtapp_sleep", sched_exit_tp, handle_sched_exit);
    rv_detach_trace_probe("rtapp_sleep", sched_set_state_tp, handle_sched_set_state);
    rv_detach_trace_probe("rtapp_sleep", contention_begin, handle_contention_begin);
    rv_detach_trace_probe("rtapp_sleep", contention_end, handle_contention_end);
    rv_detach_trace_probe("rtapp_sleep", sys_enter, handle_sys_enter);
    rv_detach_trace_probe("rtapp_sleep", sys_exit, handle_sys_exit);
    ltl_monitor_destroy();
}

static mut rv_this: rv_monitor = rv_monitor {
    name: "sleep",
    description: "Monitor that RT tasks do not undesirably sleep",
    enable: enable_sleep,
    disable: disable_sleep,
};

unsafe fn register_sleep() -> i32 {
    rv_register_monitor(&mut rv_this, &mut rv_rtapp)
}

unsafe fn unregister_sleep() {
    rv_unregister_monitor(&mut rv_this);
}

// module_init(register_sleep);
// module_exit(unregister_sleep);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Nam Cao <namcao@linutronix.de>");
// MODULE_DESCRIPTION("sleep: Monitor that RT tasks do not undesirably sleep");

// Preserved condition: compiled only when CONFIG_RV_MONITORS_KUNIT_TEST is enabled.
#[cfg(CONFIG_RV_MONITORS_KUNIT_TEST)]
pub static rv_sleep_ops: rv_sleep_ops = rv_sleep_ops {
    mon: RV_MON_OPS_INIT!(),
    handle_sched_waking,
    handle_sched_exit,
    handle_sched_set_state,
    handle_contention_begin,
    handle_contention_end,
    handle_sys_enter,
    handle_sys_exit,
    handle_task_newtask,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
