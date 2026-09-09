// SPDX-License-Identifier: GPL-2.0
// C dependencies: linux/ftrace.h, linux/tracepoint.h, linux/kernel.h,
// linux/module.h, linux/init.h, linux/rv.h, rv/instrumentation.h,
// trace/events/sched.h, rv_trace.h, monitors/sched/sched.h, opid.h,
// rv/ha_monitor.h.

const MODULE_NAME: &str = "opid";

// Build-time C condition: RV_MON_TYPE is RV_MON_PER_CPU.

unsafe fn ha_get_env(
    ha_mon: *mut ha_monitor,
    env: envs_opid,
    time_ns: u64,
) -> u64 {
    let _ = ha_mon;
    let _ = time_ns;

    if env == irq_off_opid {
        irqs_disabled()
    } else if env == preempt_off_opid {
        // C: if (IS_ENABLED(CONFIG_PREEMPTION)) ...; otherwise return true.
        if cfg!(feature = "CONFIG_PREEMPTION") {
            ((preempt_count() & PREEMPT_MASK) > 0) as u64
        } else {
            1
        }
    } else {
        ENV_INVALID_VALUE
    }
}

#[inline]
unsafe fn ha_verify_guards(
    ha_mon: *mut ha_monitor,
    curr_state: states,
    event: events,
    next_state: states,
    time_ns: u64,
) -> bool {
    let _ = next_state;
    let mut res = true;

    if curr_state == any_opid && event == sched_need_resched_opid {
        res = ha_get_env(ha_mon, irq_off_opid, time_ns) == 1u64;
    } else if curr_state == any_opid && event == sched_waking_opid {
        res = ha_get_env(ha_mon, irq_off_opid, time_ns) == 1u64
            && ha_get_env(ha_mon, preempt_off_opid, time_ns) == 1u64;
    }
    res
}

unsafe fn ha_verify_constraint(
    ha_mon: *mut ha_monitor,
    curr_state: states,
    event: events,
    next_state: states,
    time_ns: u64,
) -> bool {
    if !ha_verify_guards(ha_mon, curr_state, event, next_state, time_ns) {
        return false;
    }
    true
}

unsafe extern "C" fn handle_sched_need_resched(
    data: *mut core::ffi::c_void,
    tsk: *mut task_struct,
    cpu: i32,
    tif: i32,
) {
    let _ = (data, tsk, cpu, tif);
    da_handle_start_run_event(sched_need_resched_opid);
}

unsafe extern "C" fn handle_sched_waking(
    data: *mut core::ffi::c_void,
    p: *mut task_struct,
) {
    let _ = (data, p);
    da_handle_start_run_event(sched_waking_opid);
}

unsafe extern "C" fn enable_opid() -> i32 {
    let retval = ha_monitor_init();
    if retval != 0 {
        return retval;
    }

    rv_attach_trace_probe("opid", sched_set_need_resched_tp, handle_sched_need_resched);
    rv_attach_trace_probe("opid", sched_waking, handle_sched_waking);

    0
}

unsafe extern "C" fn disable_opid() {
    rv_this.enabled = 0;

    rv_detach_trace_probe("opid", sched_set_need_resched_tp, handle_sched_need_resched);
    rv_detach_trace_probe("opid", sched_waking, handle_sched_waking);

    ha_monitor_destroy();
}

/*
 * This is the monitor register section.
 */
static mut rv_this: rv_monitor = rv_monitor {
    name: "opid",
    description: "operations with preemption and irq disabled.",
    enable: enable_opid,
    disable: disable_opid,
    reset: da_monitor_reset_all,
    enabled: 0,
};

unsafe extern "C" fn register_opid() -> i32 {
    rv_register_monitor(&mut rv_this, &mut rv_sched)
}

unsafe extern "C" fn unregister_opid() {
    rv_unregister_monitor(&mut rv_this);
}

// C module initialization/exit declarations.
module_init!(register_opid);
module_exit!(unregister_opid);

module_license!("GPL");
module_author!("Gabriele Monaco <gmonaco@redhat.com>");
module_description!("opid: operations with preemption and irq disabled.");

// C condition: IS_ENABLED(CONFIG_RV_MONITORS_KUNIT_TEST).
#[cfg(feature = "CONFIG_RV_MONITORS_KUNIT_TEST")]
pub static rv_opid_ops: rv_opid_ops = rv_opid_ops {
    mon: RV_MON_OPS_INIT!(),
    handle_sched_need_resched: handle_sched_need_resched,
    handle_sched_waking: handle_sched_waking,
};

#[cfg(feature = "CONFIG_RV_MONITORS_KUNIT_TEST")]
export_symbol_if_kunit!(rv_opid_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
