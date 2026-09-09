// SPDX-License-Identifier: GPL-2.0
// External Linux kernel and RV monitor headers are supplied by the build environment.

const MODULE_NAME: &str = "nomiss";
const RV_MON_TYPE: i32 = RV_MON_PER_OBJ;
const HA_TIMER_TYPE: i32 = HA_TIMER_WHEEL;
// The start condition is on sched_switch, it's dangerous to allocate there.
// DA_SKIP_AUTO_ALLOC

type MonitorTarget = *mut SchedDlEntity;

static mut DEADLINE_THRESH: u64 = TICK_NSEC;

#[inline]
unsafe fn deadline_ns(ha_mon: *mut HaMonitor) -> u64 {
    (*ha_get_target(ha_mon)).dl_deadline.wrapping_add(DEADLINE_THRESH)
}

unsafe fn ha_get_env(ha_mon: *mut HaMonitor, env: EnvNomiss, time_ns: u64) -> u64 {
    if env == CLK_NOMISS {
        ha_get_clk_ns(ha_mon, env, time_ns)
    } else if env == IS_CONSTR_DL_NOMISS {
        (!dl_is_implicit(ha_get_target(ha_mon))) as u64
    } else if env == IS_DEFER_NOMISS {
        (*ha_get_target(ha_mon)).dl_defer as u64
    } else {
        ENV_INVALID_VALUE
    }
}

unsafe fn ha_reset_env(ha_mon: *mut HaMonitor, env: EnvNomiss, time_ns: u64) {
    if env == CLK_NOMISS {
        ha_reset_clk_ns(ha_mon, env, time_ns);
    }
}

#[inline]
unsafe fn ha_verify_invariants(
    ha_mon: *mut HaMonitor,
    curr_state: States,
    _event: Events,
    _next_state: States,
    time_ns: u64,
) -> bool {
    if curr_state == READY_NOMISS || curr_state == RUNNING_NOMISS {
        ha_check_invariant_ns(ha_mon, CLK_NOMISS, time_ns, deadline_ns(ha_mon))
    } else {
        true
    }
}

#[inline]
unsafe fn ha_verify_guards(
    ha_mon: *mut HaMonitor,
    curr_state: States,
    event: Events,
    _next_state: States,
    time_ns: u64,
) -> bool {
    let mut res = true;

    if curr_state == READY_NOMISS && event == DL_REPLENISH_NOMISS {
        ha_reset_env(ha_mon, CLK_NOMISS, time_ns);
    } else if curr_state == READY_NOMISS && event == DL_THROTTLE_NOMISS {
        res = ha_get_env(ha_mon, IS_DEFER_NOMISS, time_ns) == 1u64;
    } else if curr_state == IDLE_NOMISS && event == DL_REPLENISH_NOMISS {
        ha_reset_env(ha_mon, CLK_NOMISS, time_ns);
    } else if curr_state == RUNNING_NOMISS && event == DL_REPLENISH_NOMISS {
        ha_reset_env(ha_mon, CLK_NOMISS, time_ns);
    } else if curr_state == SLEEPING_NOMISS && event == DL_REPLENISH_NOMISS {
        ha_reset_env(ha_mon, CLK_NOMISS, time_ns);
    } else if curr_state == SLEEPING_NOMISS && event == DL_THROTTLE_NOMISS {
        res = ha_get_env(ha_mon, IS_CONSTR_DL_NOMISS, time_ns) == 1u64
            || ha_get_env(ha_mon, IS_DEFER_NOMISS, time_ns) == 1u64;
    } else if curr_state == THROTTLED_NOMISS && event == DL_REPLENISH_NOMISS {
        ha_reset_env(ha_mon, CLK_NOMISS, time_ns);
    }
    res
}

#[inline]
unsafe fn ha_setup_invariants(
    ha_mon: *mut HaMonitor,
    curr_state: States,
    event: Events,
    next_state: States,
    time_ns: u64,
) {
    if next_state == curr_state && event != DL_REPLENISH_NOMISS {
        return;
    }
    if next_state == READY_NOMISS || next_state == RUNNING_NOMISS {
        ha_start_timer_ns(ha_mon, CLK_NOMISS, deadline_ns(ha_mon), time_ns);
    } else if curr_state == READY_NOMISS || curr_state == RUNNING_NOMISS {
        ha_cancel_timer(ha_mon);
    }
}

unsafe fn ha_verify_constraint(
    ha_mon: *mut HaMonitor,
    curr_state: States,
    event: Events,
    next_state: States,
    time_ns: u64,
) -> bool {
    if !ha_verify_invariants(ha_mon, curr_state, event, next_state, time_ns) {
        return false;
    }
    if !ha_verify_guards(ha_mon, curr_state, event, next_state, time_ns) {
        return false;
    }
    ha_setup_invariants(ha_mon, curr_state, event, next_state, time_ns);
    true
}

unsafe fn handle_dl_replenish(data: *mut core::ffi::c_void, dl_se: *mut SchedDlEntity, cpu: i32, ty: u8) {
    if is_supported_type(ty) {
        da_handle_event(expand_id!(dl_se, cpu, ty), DL_REPLENISH_NOMISS);
    }
}

unsafe fn handle_dl_throttle(data: *mut core::ffi::c_void, dl_se: *mut SchedDlEntity, cpu: i32, ty: u8) {
    if is_supported_type(ty) {
        da_handle_event(expand_id!(dl_se, cpu, ty), DL_THROTTLE_NOMISS);
    }
}

unsafe fn handle_dl_server_stop(data: *mut core::ffi::c_void, dl_se: *mut SchedDlEntity, cpu: i32, ty: u8) {
    // This isn't the standard use of da_handle_start_run_event since this event cannot only occur from the initial state.
    // It is fine to use here because it always brings to a known state and the pretended initial transition has no side effect.
    if is_supported_type(ty) {
        da_handle_start_run_event(expand_id!(dl_se, cpu, ty), DL_SERVER_STOP_NOMISS);
    }
}

#[inline]
unsafe fn handle_server_switch(next: *mut TaskStruct, cpu: i32, ty: u8) {
    let dl_se = get_server(next, ty);
    if !dl_se.is_null() && is_idle_task(next) {
        da_handle_event(expand_id!(dl_se, cpu, ty), DL_SERVER_IDLE_NOMISS);
    }
}

unsafe fn handle_sched_switch(data: *mut core::ffi::c_void, preempt: bool, prev: *mut TaskStruct, next: *mut TaskStruct, prev_state: u32) {
    let cpu = task_cpu(next);
    if prev_state != TASK_RUNNING && !preempt && (*prev).policy == SCHED_DEADLINE {
        da_handle_event(expand_id_task!(prev), SCHED_SWITCH_SUSPEND_NOMISS);
    }
    if (*next).policy == SCHED_DEADLINE {
        da_handle_start_run_event(expand_id_task!(next), SCHED_SWITCH_IN_NOMISS);
    }
    // The server is available in next only if the next task is boosted, otherwise retrieve it.
    // The server continues running/armed until actually stopped, so a throttle remains expected.
    if !(*next).dl_server.is_null() {
        da_handle_start_event(expand_id!((*next).dl_server, cpu, get_server_type(next)), SCHED_SWITCH_IN_NOMISS);
    } else {
        handle_server_switch(next, cpu, DL_SERVER_FAIR);
        if IS_ENABLED!(CONFIG_SCHED_CLASS_EXT) {
            handle_server_switch(next, cpu, DL_SERVER_EXT);
        }
    }
}

unsafe fn handle_sys_enter(data: *mut core::ffi::c_void, regs: *mut PtRegs, id: i64) {
    let mut p: *mut TaskStruct;
    let mut new_policy: i32 = -1;
    let mut pid: Pid = 0;
    new_policy = extract_params(regs, id, &mut pid);
    if new_policy < 0 { return; }
    guard!(rcu)();
    p = if pid != 0 { find_task_by_vpid(pid) } else { current };
    if p.is_null() || new_policy == (*p).policy { return; }
    if (*p).policy == SCHED_DEADLINE {
        da_reset(expand_id_task!(p));
    } else if new_policy == SCHED_DEADLINE {
        da_create_or_get(expand_id_task!(p));
    }
}

unsafe fn handle_sched_wakeup(data: *mut core::ffi::c_void, tsk: *mut TaskStruct) {
    if (*tsk).policy == SCHED_DEADLINE {
        da_handle_event(expand_id_task!(tsk), SCHED_WAKEUP_NOMISS);
    }
}

unsafe fn enable_nomiss() -> i32 {
    let mut retval = ha_monitor_init();
    if retval != 0 { return retval; }
    retval = init_storage(false);
    if retval != 0 { return retval; }
    rv_attach_trace_probe!("nomiss", sched_dl_replenish_tp, handle_dl_replenish);
    rv_attach_trace_probe!("nomiss", sched_dl_throttle_tp, handle_dl_throttle);
    rv_attach_trace_probe!("nomiss", sched_dl_server_stop_tp, handle_dl_server_stop);
    rv_attach_trace_probe!("nomiss", sched_switch, handle_sched_switch);
    rv_attach_trace_probe!("nomiss", sched_wakeup, handle_sched_wakeup);
    if !should_skip_syscall_handle() { rv_attach_trace_probe!("nomiss", sys_enter, handle_sys_enter); }
    rv_attach_trace_probe!("nomiss", task_newtask, handle_newtask);
    rv_attach_trace_probe!("nomiss", sched_process_exit, handle_exit);
    0
}

unsafe fn disable_nomiss() {
    rv_this.enabled = 0;
    // Those are RCU writers, detach earlier hoping to close a bit faster.
    rv_detach_trace_probe!("nomiss", task_newtask, handle_newtask);
    rv_detach_trace_probe!("nomiss", sched_process_exit, handle_exit);
    if !should_skip_syscall_handle() { rv_detach_trace_probe!("nomiss", sys_enter, handle_sys_enter); }
    rv_detach_trace_probe!("nomiss", sched_dl_replenish_tp, handle_dl_replenish);
    rv_detach_trace_probe!("nomiss", sched_dl_throttle_tp, handle_dl_throttle);
    rv_detach_trace_probe!("nomiss", sched_dl_server_stop_tp, handle_dl_server_stop);
    rv_detach_trace_probe!("nomiss", sched_switch, handle_sched_switch);
    rv_detach_trace_probe!("nomiss", sched_wakeup, handle_sched_wakeup);
    ha_monitor_destroy();
}

static mut rv_this: RvMonitor = RvMonitor {
    name: "nomiss",
    description: "dl entities run to completion before their deadline.",
    enable: enable_nomiss,
    disable: disable_nomiss,
    reset: da_monitor_reset_all,
    enabled: 0,
};

unsafe fn register_nomiss() -> i32 { rv_register_monitor(&mut rv_this, &rv_deadline) }
unsafe fn unregister_nomiss() { rv_unregister_monitor(&mut rv_this); }

// module_init!(register_nomiss);
// module_exit!(unregister_nomiss);
// MODULE_LICENSE!("GPL");
// MODULE_AUTHOR!("Gabriele Monaco <gmonaco@redhat.com>");
// MODULE_DESCRIPTION!("nomiss: dl entities run to completion before their deadline.");

#[cfg(CONFIG_RV_MONITORS_KUNIT_TEST)]
pub static rv_nomiss_ops: RvNomissOps = RvNomissOps {
    mon: RV_MON_OPS_INIT!(),
    deadline_thresh: unsafe { &raw mut DEADLINE_THRESH },
    handle_dl_replenish,
    handle_dl_throttle,
    handle_dl_server_stop,
    handle_sched_switch,
    handle_sched_wakeup,
    handle_sys_enter,
    handle_newtask,
};

// EXPORT_SYMBOL_IF_KUNIT!(rv_nomiss_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
