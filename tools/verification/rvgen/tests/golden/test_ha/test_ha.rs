// SPDX-License-Identifier: GPL-2.0
// C dependencies: linux/ftrace.h, linux/tracepoint.h, linux/kernel.h,
// linux/module.h, linux/init.h, linux/rv.h, rv/instrumentation.h,
// rv_trace.h, test_ha.h, rv/ha_monitor.h.

pub const MODULE_NAME: &[u8] = b"test_ha\0";

/*
 * XXX: include required tracepoint headers, e.g.,
 * #include <trace/events/sched.h>
 */

/*
 * This is the self-generated part of the monitor. Generally, there is no need
 * to touch this section.
 */
pub const RV_MON_TYPE: u32 = RV_MON_PER_TASK;
/* XXX: If the monitor has several instances, consider HA_TIMER_WHEEL */
pub const HA_TIMER_TYPE: u32 = HA_TIMER_HRTIMER;

/*
 * This is the instrumentation part of the monitor.
 *
 * This is the section where manual work is required. Here the kernel events
 * are translated into model's event.
 *
 */
macro_rules! BAR_NS {
    ($ha_mon:expr) => {
        todo!("XXX: what is BAR_NS(ha_mon)?")
    };
}

macro_rules! FOO_NS {
    () => {
        todo!("XXX: what is FOO_NS?")
    };
}

#[repr(C)]
pub struct ha_monitor {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rv_monitor {
    pub name: *const core::ffi::c_char,
    pub description: *const core::ffi::c_char,
    pub enable: Option<unsafe extern "C" fn() -> core::ffi::c_int>,
    pub disable: Option<unsafe extern "C" fn()>,
    pub reset: Option<unsafe extern "C" fn()>,
    pub enabled: core::ffi::c_int,
}

pub type u64 = core::ffi::c_ulonglong;

pub const RV_MON_PER_TASK: u32 = 0;
pub const HA_TIMER_HRTIMER: u32 = 0;
pub const ENV_INVALID_VALUE: u64 = !0u64;

pub type envs_test_ha = core::ffi::c_uint;
pub type states = core::ffi::c_uint;
pub type events = core::ffi::c_uint;

unsafe extern "C" {
    pub static clk_test_ha: envs_test_ha;
    pub static env1_test_ha: envs_test_ha;
    pub static env2_test_ha: envs_test_ha;

    pub static S0_test_ha: states;
    pub static S1_test_ha: states;
    pub static S2_test_ha: states;
    pub static S3_test_ha: states;

    pub static event0_test_ha: events;
    pub static event1_test_ha: events;
    pub static event2_test_ha: events;

    pub fn ha_get_clk_ns(ha_mon: *mut ha_monitor, env: envs_test_ha, time_ns: u64) -> u64;
    pub fn ha_reset_clk_ns(ha_mon: *mut ha_monitor, env: envs_test_ha, time_ns: u64);
    pub fn ha_check_invariant_ns(
        ha_mon: *mut ha_monitor,
        env: envs_test_ha,
        time_ns: u64,
        value: u64,
    ) -> bool;
    pub fn ha_monitor_env_invalid(ha_mon: *mut ha_monitor, env: envs_test_ha) -> bool;
    pub fn ha_start_timer_ns(
        ha_mon: *mut ha_monitor,
        env: envs_test_ha,
        duration_ns: u64,
        time_ns: u64,
    );
    pub fn ha_cancel_timer(ha_mon: *mut ha_monitor);
    pub fn ha_monitor_init() -> core::ffi::c_int;
    pub fn ha_monitor_destroy();

    pub fn da_handle_start_event(p: *mut task_struct, event: events);
    pub fn da_handle_event(p: *mut task_struct, event: events);
    pub fn da_monitor_reset_all();

    pub fn rv_attach_trace_probe(
        name: *const core::ffi::c_char,
        tracepoint: *const core::ffi::c_void,
        callback: *const core::ffi::c_void,
    );
    pub fn rv_detach_trace_probe(
        name: *const core::ffi::c_char,
        tracepoint: *const core::ffi::c_void,
        callback: *const core::ffi::c_void,
    );
    pub fn rv_register_monitor(
        monitor: *mut rv_monitor,
        data: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    pub fn rv_unregister_monitor(monitor: *mut rv_monitor);
}

#[inline]
unsafe fn bar_ns(_ha_mon: *mut ha_monitor) -> u64 {
    todo!("XXX: what is bar_ns(ha_mon)?")
}

static mut foo_ns: u64 = todo!("XXX: default value");
// module_param(foo_ns, ullong, 0644);

/*
 * These functions define how to read and reset the environment variable.
 *
 * Common environment variables like ns-based and jiffy-based clocks have
 * pre-define getters and resetters you can use. The parser can infer the type
 * of the environment variable if you supply a measure unit in the constraint.
 * If you define your own functions, make sure to add appropriate memory
 * barriers if required.
 * Some environment variables don't require a storage as they read a system
 * state (e.g. preemption count). Those variables are never reset, so we don't
 * define a reset function on monitors only relying on this type of variables.
 */
unsafe fn ha_get_env(ha_mon: *mut ha_monitor, env: envs_test_ha, time_ns: u64) -> u64 {
    if env == clk_test_ha {
        ha_get_clk_ns(ha_mon, env, time_ns)
    } else if env == env1_test_ha {
        todo!("XXX: how do I read env1?")
    } else if env == env2_test_ha {
        todo!("XXX: how do I read env2?")
    } else {
        ENV_INVALID_VALUE
    }
}

unsafe fn ha_reset_env(ha_mon: *mut ha_monitor, env: envs_test_ha, time_ns: u64) {
    if env == clk_test_ha {
        ha_reset_clk_ns(ha_mon, env, time_ns);
    }
}

/*
 * These functions are used to validate state transitions.
 *
 * They are generated by parsing the model, there is usually no need to change them.
 * If the monitor requires a timer, there are functions responsible to arm it when
 * the next state has a constraint, cancel it in any other case and to check
 * that it didn't expire before the callback run. Transitions to the same state
 * without a reset never affect timers.
 */
#[inline]
unsafe fn ha_verify_invariants(
    ha_mon: *mut ha_monitor,
    curr_state: states,
    _event: events,
    _next_state: states,
    time_ns: u64,
) -> bool {
    if curr_state == S0_test_ha {
        ha_check_invariant_ns(ha_mon, clk_test_ha, time_ns, bar_ns(ha_mon))
    } else if curr_state == S2_test_ha {
        ha_check_invariant_ns(ha_mon, clk_test_ha, time_ns, BAR_NS!(ha_mon))
    } else {
        true
    }
}

#[inline]
unsafe fn ha_verify_guards(
    ha_mon: *mut ha_monitor,
    curr_state: states,
    event: events,
    _next_state: states,
    time_ns: u64,
) -> bool {
    let mut res = true;

    if curr_state == S0_test_ha && event == event0_test_ha {
        ha_reset_env(ha_mon, clk_test_ha, time_ns);
    } else if curr_state == S0_test_ha && event == event1_test_ha {
        ha_reset_env(ha_mon, clk_test_ha, time_ns);
    } else if curr_state == S1_test_ha && event == event0_test_ha {
        ha_reset_env(ha_mon, clk_test_ha, time_ns);
    } else if curr_state == S1_test_ha && event == event2_test_ha {
        res = ha_get_env(ha_mon, env1_test_ha, time_ns) == 0u64;
        ha_reset_env(ha_mon, clk_test_ha, time_ns);
    } else if curr_state == S2_test_ha && event == event1_test_ha {
        res = ha_monitor_env_invalid(ha_mon, clk_test_ha)
            || ha_get_env(ha_mon, clk_test_ha, time_ns) < foo_ns;
    } else if curr_state == S3_test_ha && event == event0_test_ha {
        res = ha_monitor_env_invalid(ha_mon, clk_test_ha)
            || (ha_get_env(ha_mon, clk_test_ha, time_ns) < FOO_NS!()
                && ha_get_env(ha_mon, env2_test_ha, time_ns) == 0u64);
    } else if curr_state == S3_test_ha && event == event1_test_ha {
        res = ha_monitor_env_invalid(ha_mon, clk_test_ha)
            || (ha_get_env(ha_mon, clk_test_ha, time_ns) < 5000u64
                && ha_get_env(ha_mon, env1_test_ha, time_ns) == 1u64);
        ha_reset_env(ha_mon, clk_test_ha, time_ns);
    }
    res
}

#[inline]
unsafe fn ha_setup_invariants(
    ha_mon: *mut ha_monitor,
    curr_state: states,
    event: events,
    next_state: states,
    time_ns: u64,
) {
    if next_state == curr_state && event != event0_test_ha {
        return;
    }
    if next_state == S0_test_ha {
        ha_start_timer_ns(ha_mon, clk_test_ha, bar_ns(ha_mon), time_ns);
    } else if next_state == S2_test_ha {
        ha_start_timer_ns(ha_mon, clk_test_ha, BAR_NS!(ha_mon), time_ns);
    } else if curr_state == S0_test_ha {
        ha_cancel_timer(ha_mon);
    } else if curr_state == S2_test_ha {
        ha_cancel_timer(ha_mon);
    }
}

unsafe fn ha_verify_constraint(
    ha_mon: *mut ha_monitor,
    curr_state: states,
    event: events,
    next_state: states,
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

unsafe extern "C" fn handle_event0(_data: *mut core::ffi::c_void) {
    /* XXX: validate that this event always leads to the initial state */
    let p: *mut task_struct = todo!("XXX: how do I get p?");
    da_handle_start_event(p, event0_test_ha);
}

unsafe extern "C" fn handle_event1(_data: *mut core::ffi::c_void) {
    let p: *mut task_struct = todo!("XXX: how do I get p?");
    da_handle_event(p, event1_test_ha);
}

unsafe extern "C" fn handle_event2(_data: *mut core::ffi::c_void) {
    let p: *mut task_struct = todo!("XXX: how do I get p?");
    da_handle_event(p, event2_test_ha);
}

unsafe extern "C" fn enable_test_ha() -> core::ffi::c_int {
    let retval: core::ffi::c_int;

    retval = ha_monitor_init();
    if retval != 0 {
        return retval;
    }

    rv_attach_trace_probe(
        c"test_ha".as_ptr(),
        todo!("XXX: tracepoint"),
        handle_event0 as *const core::ffi::c_void,
    );
    rv_attach_trace_probe(
        c"test_ha".as_ptr(),
        todo!("XXX: tracepoint"),
        handle_event1 as *const core::ffi::c_void,
    );
    rv_attach_trace_probe(
        c"test_ha".as_ptr(),
        todo!("XXX: tracepoint"),
        handle_event2 as *const core::ffi::c_void,
    );

    0
}

unsafe extern "C" fn disable_test_ha() {
    rv_this.enabled = 0;

    rv_detach_trace_probe(
        c"test_ha".as_ptr(),
        todo!("XXX: tracepoint"),
        handle_event0 as *const core::ffi::c_void,
    );
    rv_detach_trace_probe(
        c"test_ha".as_ptr(),
        todo!("XXX: tracepoint"),
        handle_event1 as *const core::ffi::c_void,
    );
    rv_detach_trace_probe(
        c"test_ha".as_ptr(),
        todo!("XXX: tracepoint"),
        handle_event2 as *const core::ffi::c_void,
    );

    ha_monitor_destroy();
}

/*
 * This is the monitor register section.
 */
static mut rv_this: rv_monitor = rv_monitor {
    name: c"test_ha".as_ptr(),
    description: c"auto-generated".as_ptr(),
    enable: Some(enable_test_ha),
    disable: Some(disable_test_ha),
    reset: Some(da_monitor_reset_all),
    enabled: 0,
};

unsafe extern "C" fn register_test_ha() -> core::ffi::c_int {
    rv_register_monitor(&raw mut rv_this, core::ptr::null_mut())
}

unsafe extern "C" fn unregister_test_ha() {
    rv_unregister_monitor(&raw mut rv_this);
}

// module_init(register_test_ha);
// module_exit(unregister_test_ha);

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("rvgen: auto-generated");
// MODULE_DESCRIPTION("test_ha: auto-generated");
