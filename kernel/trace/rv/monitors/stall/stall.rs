// SPDX-License-Identifier: GPL-2.0
//
// C dependencies:
// linux/ftrace.h, linux/tracepoint.h, linux/kernel.h, linux/module.h,
// linux/init.h, linux/rv.h, rv/instrumentation.h, trace/events/sched.h,
// rv_trace.h, stall.h, and rv/ha_monitor.h.

// #define MODULE_NAME "stall"
// #define RV_MON_TYPE RV_MON_PER_TASK
// #define HA_TIMER_TYPE HA_TIMER_WHEEL

use crate::{
    da_handle_event, da_handle_start_event, da_monitor_reset_all, enum_envs_stall,
    enum_events, enum_states, ha_cancel_timer, ha_check_invariant_jiffy,
    ha_get_clk_jiffy, ha_monitor_destroy, ha_monitor_init, ha_reset_clk_jiffy,
    ha_start_timer_jiffy, rv_attach_trace_probe, rv_detach_trace_probe,
    rv_register_monitor, rv_unregister_monitor, task_struct, ENV_INVALID_VALUE,
    HA_TIMER_WHEEL, RV_MON_PER_TASK, TASK_RUNNING, clk_stall, dequeued_stall,
    enqueued_stall, running_stall, sched_switch_in_stall, sched_switch_preempt_stall,
    sched_switch_wait_stall, sched_wakeup_stall,
};

use core::ffi::{c_int, c_uint, c_void};

// module_param(threshold_jiffies, ullong, 0644);
static mut threshold_jiffies: u64 = 1000;

unsafe fn ha_get_env(
    ha_mon: *mut crate::ha_monitor,
    env: enum_envs_stall,
    _time_ns: u64,
) -> u64 {
    if env == clk_stall {
        return ha_get_clk_jiffy(ha_mon, env);
    }
    ENV_INVALID_VALUE
}

unsafe fn ha_reset_env(
    ha_mon: *mut crate::ha_monitor,
    env: enum_envs_stall,
    _time_ns: u64,
) {
    if env == clk_stall {
        ha_reset_clk_jiffy(ha_mon, env);
    }
}

#[inline]
unsafe fn ha_verify_invariants(
    ha_mon: *mut crate::ha_monitor,
    curr_state: enum_states,
    _event: enum_events,
    _next_state: enum_states,
    time_ns: u64,
) -> bool {
    if curr_state == enqueued_stall {
        return ha_check_invariant_jiffy(ha_mon, clk_stall, time_ns, threshold_jiffies);
    }
    true
}

#[inline]
unsafe fn ha_verify_guards(
    ha_mon: *mut crate::ha_monitor,
    curr_state: enum_states,
    event: enum_events,
    _next_state: enum_states,
    time_ns: u64,
) -> bool {
    let res = true;

    if curr_state == dequeued_stall && event == sched_wakeup_stall {
        ha_reset_env(ha_mon, clk_stall, time_ns);
    } else if curr_state == running_stall && event == sched_switch_preempt_stall {
        ha_reset_env(ha_mon, clk_stall, time_ns);
    }
    res
}

#[inline]
unsafe fn ha_setup_invariants(
    ha_mon: *mut crate::ha_monitor,
    curr_state: enum_states,
    _event: enum_events,
    next_state: enum_states,
    time_ns: u64,
) {
    if next_state == curr_state {
        return;
    }
    if next_state == enqueued_stall {
        ha_start_timer_jiffy(ha_mon, clk_stall, threshold_jiffies, time_ns);
    } else if curr_state == enqueued_stall {
        ha_cancel_timer(ha_mon);
    }
}

unsafe fn ha_verify_constraint(
    ha_mon: *mut crate::ha_monitor,
    curr_state: enum_states,
    event: enum_events,
    next_state: enum_states,
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

unsafe extern "C" fn handle_sched_switch(
    _data: *mut c_void,
    preempt: bool,
    prev: *mut task_struct,
    next: *mut task_struct,
    prev_state: c_uint,
) {
    if !preempt && prev_state != TASK_RUNNING {
        da_handle_start_event(prev, sched_switch_wait_stall);
    } else {
        da_handle_event(prev, sched_switch_preempt_stall);
    }
    da_handle_event(next, sched_switch_in_stall);
}

unsafe extern "C" fn handle_sched_wakeup(_data: *mut c_void, p: *mut task_struct) {
    da_handle_event(p, sched_wakeup_stall);
}

unsafe extern "C" fn enable_stall() -> c_int {
    let retval = ha_monitor_init();
    if retval != 0 {
        return retval;
    }
    rv_attach_trace_probe("stall", sched_switch, handle_sched_switch);
    rv_attach_trace_probe("stall", sched_wakeup, handle_sched_wakeup);
    0
}

unsafe extern "C" fn disable_stall() {
    rv_this.enabled = 0;
    rv_detach_trace_probe("stall", sched_switch, handle_sched_switch);
    rv_detach_trace_probe("stall", sched_wakeup, handle_sched_wakeup);
    ha_monitor_destroy();
}

static mut rv_this: crate::rv_monitor = crate::rv_monitor {
    name: "stall",
    description: "identify tasks stalled for longer than a threshold.",
    enable: enable_stall,
    disable: disable_stall,
    reset: da_monitor_reset_all,
    enabled: 0,
};

unsafe extern "C" fn register_stall() -> c_int {
    rv_register_monitor(&raw mut rv_this, core::ptr::null_mut())
}

unsafe extern "C" fn unregister_stall() {
    rv_unregister_monitor(&raw mut rv_this);
}

// module_init(register_stall);
// module_exit(unregister_stall);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Gabriele Monaco <gmonaco@redhat.com>");
// MODULE_DESCRIPTION("stall: identify tasks stalled for longer than a threshold.");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
