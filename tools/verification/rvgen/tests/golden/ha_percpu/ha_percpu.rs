// SPDX-License-Identifier: GPL-2.0
//
// C dependencies removed from executable Rust:
// linux/ftrace.h, linux/tracepoint.h, linux/kernel.h, linux/module.h,
// linux/init.h, linux/rv.h, rv/instrumentation.h, rv_trace.h,
// ha_percpu.h, rv/ha_monitor.h.

const MODULE_NAME: &[u8] = b"ha_percpu\0";

/*
 * XXX: include required tracepoint headers, e.g.,
 * #include <trace/events/sched.h>
 */

/*
 * This is the self-generated part of the monitor. Generally, there is no need
 * to touch this section.
 */
const RV_MON_TYPE: u32 = RV_MON_PER_CPU;
/* XXX: If the monitor has several instances, consider HA_TIMER_WHEEL */
const HA_TIMER_TYPE: u32 = HA_TIMER_HRTIMER;

/*
 * This is the instrumentation part of the monitor.
 *
 * This is the section where manual work is required. Here the kernel events
 * are translated into model's event.
 *
 */

type u64_t = u64;
type c_int = i32;
type c_char = i8;
type c_void = core::ffi::c_void;

#[repr(C)]
pub struct ha_monitor {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rv_monitor {
    pub name: *const c_char,
    pub description: *const c_char,
    pub enable: Option<unsafe extern "C" fn() -> c_int>,
    pub disable: Option<unsafe extern "C" fn()>,
    pub reset: Option<unsafe extern "C" fn()>,
    pub enabled: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum envs_ha_percpu {
    clk_ha_percpu,
    env1_ha_percpu,
    env2_ha_percpu,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum states {
    S0_ha_percpu,
    S1_ha_percpu,
    S2_ha_percpu,
    S3_ha_percpu,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum events {
    event0_ha_percpu,
    event1_ha_percpu,
    event2_ha_percpu,
}

extern "C" {
    static mut rv_this: rv_monitor;

    static RV_MON_PER_CPU: u32;
    static HA_TIMER_HRTIMER: u32;
    static ENV_INVALID_VALUE: u64_t;

    fn ha_get_clk_ns(ha_mon: *mut ha_monitor, env: envs_ha_percpu, time_ns: u64_t) -> u64_t;
    fn ha_reset_clk_ns(ha_mon: *mut ha_monitor, env: envs_ha_percpu, time_ns: u64_t);
    fn ha_check_invariant_ns(
        ha_mon: *mut ha_monitor,
        env: envs_ha_percpu,
        time_ns: u64_t,
        value: u64_t,
    ) -> bool;
    fn ha_monitor_env_invalid(ha_mon: *mut ha_monitor, env: envs_ha_percpu) -> bool;
    fn ha_start_timer_ns(
        ha_mon: *mut ha_monitor,
        env: envs_ha_percpu,
        timeout_ns: u64_t,
        time_ns: u64_t,
    );
    fn ha_cancel_timer(ha_mon: *mut ha_monitor);
    fn ha_monitor_init() -> c_int;
    fn ha_monitor_destroy();

    fn da_handle_start_event(event: events);
    fn da_handle_event(event: events);
    fn da_monitor_reset_all();

    fn rv_attach_trace_probe(
        monitor: *const c_char,
        tracepoint: *const c_void,
        handler: unsafe extern "C" fn(*mut c_void),
    );
    fn rv_detach_trace_probe(
        monitor: *const c_char,
        tracepoint: *const c_void,
        handler: unsafe extern "C" fn(*mut c_void),
    );
    fn rv_register_monitor(monitor: *mut rv_monitor, data: *mut c_void) -> c_int;
    fn rv_unregister_monitor(monitor: *mut rv_monitor);
}

#[inline]
unsafe fn BAR_NS(_ha_mon: *mut ha_monitor) -> u64_t {
    /* XXX: what is BAR_NS(ha_mon)? */
    todo!("XXX: what is BAR_NS(ha_mon)?")
}

#[inline]
unsafe fn FOO_NS() -> u64_t {
    /* XXX: what is FOO_NS? */
    todo!("XXX: what is FOO_NS?")
}

#[inline]
unsafe fn bar_ns(_ha_mon: *mut ha_monitor) -> u64_t {
    /* XXX: what is bar_ns(ha_mon)? */
    todo!("XXX: what is bar_ns(ha_mon)?")
}

static mut foo_ns: u64_t = {
    /* XXX: default value */
    0
};
/* module_param(foo_ns, ullong, 0644); */

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
unsafe fn ha_get_env(ha_mon: *mut ha_monitor, env: envs_ha_percpu, time_ns: u64_t) -> u64_t {
    if env == envs_ha_percpu::clk_ha_percpu {
        ha_get_clk_ns(ha_mon, env, time_ns)
    } else if env == envs_ha_percpu::env1_ha_percpu {
        /* XXX: how do I read env1? */
        todo!("XXX: how do I read env1?")
    } else if env == envs_ha_percpu::env2_ha_percpu {
        /* XXX: how do I read env2? */
        todo!("XXX: how do I read env2?")
    } else {
        ENV_INVALID_VALUE
    }
}

unsafe fn ha_reset_env(ha_mon: *mut ha_monitor, env: envs_ha_percpu, time_ns: u64_t) {
    if env == envs_ha_percpu::clk_ha_percpu {
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
    time_ns: u64_t,
) -> bool {
    if curr_state == states::S0_ha_percpu {
        ha_check_invariant_ns(ha_mon, envs_ha_percpu::clk_ha_percpu, time_ns, bar_ns(ha_mon))
    } else if curr_state == states::S2_ha_percpu {
        ha_check_invariant_ns(ha_mon, envs_ha_percpu::clk_ha_percpu, time_ns, BAR_NS(ha_mon))
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
    time_ns: u64_t,
) -> bool {
    let mut res = true;

    if curr_state == states::S0_ha_percpu && event == events::event0_ha_percpu {
        ha_reset_env(ha_mon, envs_ha_percpu::clk_ha_percpu, time_ns);
    } else if curr_state == states::S0_ha_percpu && event == events::event1_ha_percpu {
        ha_reset_env(ha_mon, envs_ha_percpu::clk_ha_percpu, time_ns);
    } else if curr_state == states::S1_ha_percpu && event == events::event0_ha_percpu {
        ha_reset_env(ha_mon, envs_ha_percpu::clk_ha_percpu, time_ns);
    } else if curr_state == states::S1_ha_percpu && event == events::event2_ha_percpu {
        res = ha_get_env(ha_mon, envs_ha_percpu::env1_ha_percpu, time_ns) == 0u64;
        ha_reset_env(ha_mon, envs_ha_percpu::clk_ha_percpu, time_ns);
    } else if curr_state == states::S2_ha_percpu && event == events::event1_ha_percpu {
        res = ha_monitor_env_invalid(ha_mon, envs_ha_percpu::clk_ha_percpu)
            || ha_get_env(ha_mon, envs_ha_percpu::clk_ha_percpu, time_ns) < foo_ns;
    } else if curr_state == states::S3_ha_percpu && event == events::event0_ha_percpu {
        res = ha_monitor_env_invalid(ha_mon, envs_ha_percpu::clk_ha_percpu)
            || (ha_get_env(ha_mon, envs_ha_percpu::clk_ha_percpu, time_ns) < FOO_NS()
                && ha_get_env(ha_mon, envs_ha_percpu::env2_ha_percpu, time_ns) == 0u64);
    } else if curr_state == states::S3_ha_percpu && event == events::event1_ha_percpu {
        res = ha_monitor_env_invalid(ha_mon, envs_ha_percpu::clk_ha_percpu)
            || (ha_get_env(ha_mon, envs_ha_percpu::clk_ha_percpu, time_ns) < 5000u64
                && ha_get_env(ha_mon, envs_ha_percpu::env1_ha_percpu, time_ns) == 1u64);
        ha_reset_env(ha_mon, envs_ha_percpu::clk_ha_percpu, time_ns);
    }
    res
}

#[inline]
unsafe fn ha_setup_invariants(
    ha_mon: *mut ha_monitor,
    curr_state: states,
    event: events,
    next_state: states,
    time_ns: u64_t,
) {
    if next_state == curr_state && event != events::event0_ha_percpu {
        return;
    }
    if next_state == states::S0_ha_percpu {
        ha_start_timer_ns(
            ha_mon,
            envs_ha_percpu::clk_ha_percpu,
            bar_ns(ha_mon),
            time_ns,
        );
    } else if next_state == states::S2_ha_percpu {
        ha_start_timer_ns(
            ha_mon,
            envs_ha_percpu::clk_ha_percpu,
            BAR_NS(ha_mon),
            time_ns,
        );
    } else if curr_state == states::S0_ha_percpu {
        ha_cancel_timer(ha_mon);
    } else if curr_state == states::S2_ha_percpu {
        ha_cancel_timer(ha_mon);
    }
}

unsafe fn ha_verify_constraint(
    ha_mon: *mut ha_monitor,
    curr_state: states,
    event: events,
    next_state: states,
    time_ns: u64_t,
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

unsafe extern "C" fn handle_event0(_data: *mut c_void) {
    /* XXX: fill header */
    /* XXX: validate that this event always leads to the initial state */
    da_handle_start_event(events::event0_ha_percpu);
}

unsafe extern "C" fn handle_event1(_data: *mut c_void) {
    /* XXX: fill header */
    da_handle_event(events::event1_ha_percpu);
}

unsafe extern "C" fn handle_event2(_data: *mut c_void) {
    /* XXX: fill header */
    da_handle_event(events::event2_ha_percpu);
}

unsafe extern "C" fn enable_ha_percpu() -> c_int {
    let retval: c_int;

    retval = ha_monitor_init();
    if retval != 0 {
        return retval;
    }

    rv_attach_trace_probe(
        b"ha_percpu\0".as_ptr() as *const c_char,
        core::ptr::null(),
        handle_event0,
    ); /* XXX: tracepoint */
    rv_attach_trace_probe(
        b"ha_percpu\0".as_ptr() as *const c_char,
        core::ptr::null(),
        handle_event1,
    ); /* XXX: tracepoint */
    rv_attach_trace_probe(
        b"ha_percpu\0".as_ptr() as *const c_char,
        core::ptr::null(),
        handle_event2,
    ); /* XXX: tracepoint */

    0
}

unsafe extern "C" fn disable_ha_percpu() {
    rv_this.enabled = 0;

    rv_detach_trace_probe(
        b"ha_percpu\0".as_ptr() as *const c_char,
        core::ptr::null(),
        handle_event0,
    ); /* XXX: tracepoint */
    rv_detach_trace_probe(
        b"ha_percpu\0".as_ptr() as *const c_char,
        core::ptr::null(),
        handle_event1,
    ); /* XXX: tracepoint */
    rv_detach_trace_probe(
        b"ha_percpu\0".as_ptr() as *const c_char,
        core::ptr::null(),
        handle_event2,
    ); /* XXX: tracepoint */

    ha_monitor_destroy();
}

/*
 * This is the monitor register section.
 */
#[no_mangle]
pub static mut rv_this: rv_monitor = rv_monitor {
    name: b"ha_percpu\0".as_ptr() as *const c_char,
    description: b"auto-generated\0".as_ptr() as *const c_char,
    enable: Some(enable_ha_percpu),
    disable: Some(disable_ha_percpu),
    reset: Some(da_monitor_reset_all),
    enabled: 0,
};

unsafe extern "C" fn register_ha_percpu() -> c_int {
    rv_register_monitor(core::ptr::addr_of_mut!(rv_this), core::ptr::null_mut())
}

unsafe extern "C" fn unregister_ha_percpu() {
    rv_unregister_monitor(core::ptr::addr_of_mut!(rv_this));
}

/* module_init(register_ha_percpu); */
/* module_exit(unregister_ha_percpu); */

/* MODULE_LICENSE("GPL"); */
/* MODULE_AUTHOR("rvgen: auto-generated"); */
/* MODULE_DESCRIPTION("ha_percpu: auto-generated"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
