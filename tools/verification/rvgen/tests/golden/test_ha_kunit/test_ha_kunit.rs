// SPDX-License-Identifier: GPL-2.0
//
// C dependencies removed from executable Rust:
// linux/ftrace.h, linux/tracepoint.h, linux/kernel.h, linux/module.h,
// linux/init.h, linux/rv.h, rv/instrumentation.h, rv_trace.h,
// test_ha_kunit.h, rv/ha_monitor.h.

pub const MODULE_NAME: &str = "test_ha_kunit";

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
// #define BAR_NS(ha_mon) /* XXX: what is BAR_NS(ha_mon)? */

// #define FOO_NS /* XXX: what is FOO_NS? */

pub type U64 = u64;

#[repr(C)]
pub struct HaMonitor {
    _private: [u8; 0],
}

#[repr(C)]
pub struct TaskStruct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct RvMonitor {
    pub name: *const ::core::ffi::c_char,
    pub description: *const ::core::ffi::c_char,
    pub enable: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub disable: Option<unsafe extern "C" fn()>,
    pub reset: Option<unsafe extern "C" fn()>,
    pub enabled: ::core::ffi::c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum EnvsTestHaKunit {
    ClkTestHaKunit,
    Env1TestHaKunit,
    Env2TestHaKunit,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum States {
    S0TestHaKunit,
    S1TestHaKunit,
    S2TestHaKunit,
    S3TestHaKunit,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Events {
    Event0TestHaKunit,
    Event1TestHaKunit,
    Event2TestHaKunit,
}

unsafe extern "C" {
    static ENV_INVALID_VALUE: U64;
    static RV_MON_PER_TASK: u32;
    static HA_TIMER_HRTIMER: u32;

    fn ha_get_clk_ns(ha_mon: *mut HaMonitor, env: EnvsTestHaKunit, time_ns: U64) -> U64;
    fn ha_reset_clk_ns(ha_mon: *mut HaMonitor, env: EnvsTestHaKunit, time_ns: U64);
    fn ha_check_invariant_ns(
        ha_mon: *mut HaMonitor,
        env: EnvsTestHaKunit,
        time_ns: U64,
        value: U64,
    ) -> bool;
    fn ha_monitor_env_invalid(ha_mon: *mut HaMonitor, env: EnvsTestHaKunit) -> bool;
    fn ha_start_timer_ns(ha_mon: *mut HaMonitor, env: EnvsTestHaKunit, value: U64, time_ns: U64);
    fn ha_cancel_timer(ha_mon: *mut HaMonitor);
    fn ha_monitor_init() -> ::core::ffi::c_int;
    fn ha_monitor_destroy();

    fn da_handle_start_event(p: *mut TaskStruct, event: Events);
    fn da_handle_event(p: *mut TaskStruct, event: Events);
    fn da_monitor_reset_all();

    fn rv_attach_trace_probe(
        name: *const ::core::ffi::c_char,
        tracepoint: *const ::core::ffi::c_void,
        probe: *const ::core::ffi::c_void,
    );
    fn rv_detach_trace_probe(
        name: *const ::core::ffi::c_char,
        tracepoint: *const ::core::ffi::c_void,
        probe: *const ::core::ffi::c_void,
    );
    fn rv_register_monitor(
        monitor: *mut RvMonitor,
        data: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn rv_unregister_monitor(monitor: *mut RvMonitor);
}

#[inline]
unsafe fn BAR_NS(_ha_mon: *mut HaMonitor) -> U64 {
    /* XXX: what is BAR_NS(ha_mon)? */
    todo!()
}

#[inline]
unsafe fn FOO_NS() -> U64 {
    /* XXX: what is FOO_NS? */
    todo!()
}

#[inline]
unsafe fn bar_ns(_ha_mon: *mut HaMonitor) -> U64 {
    /* XXX: what is bar_ns(ha_mon)? */
    todo!()
}

static mut foo_ns: U64 = {
    /* XXX: default value */
    0
};
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
unsafe fn ha_get_env(ha_mon: *mut HaMonitor, env: EnvsTestHaKunit, time_ns: U64) -> U64 {
    if env == EnvsTestHaKunit::ClkTestHaKunit {
        unsafe { ha_get_clk_ns(ha_mon, env, time_ns) }
    } else if env == EnvsTestHaKunit::Env1TestHaKunit {
        /* XXX: how do I read env1? */
        todo!()
    } else if env == EnvsTestHaKunit::Env2TestHaKunit {
        /* XXX: how do I read env2? */
        todo!()
    } else {
        unsafe { ENV_INVALID_VALUE }
    }
}

unsafe fn ha_reset_env(ha_mon: *mut HaMonitor, env: EnvsTestHaKunit, time_ns: U64) {
    if env == EnvsTestHaKunit::ClkTestHaKunit {
        unsafe { ha_reset_clk_ns(ha_mon, env, time_ns) };
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
    ha_mon: *mut HaMonitor,
    curr_state: States,
    _event: Events,
    _next_state: States,
    time_ns: U64,
) -> bool {
    if curr_state == States::S0TestHaKunit {
        unsafe {
            ha_check_invariant_ns(
                ha_mon,
                EnvsTestHaKunit::ClkTestHaKunit,
                time_ns,
                bar_ns(ha_mon),
            )
        }
    } else if curr_state == States::S2TestHaKunit {
        unsafe {
            ha_check_invariant_ns(
                ha_mon,
                EnvsTestHaKunit::ClkTestHaKunit,
                time_ns,
                BAR_NS(ha_mon),
            )
        }
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
    time_ns: U64,
) -> bool {
    let mut res = true;

    if curr_state == States::S0TestHaKunit && event == Events::Event0TestHaKunit {
        unsafe { ha_reset_env(ha_mon, EnvsTestHaKunit::ClkTestHaKunit, time_ns) };
    } else if curr_state == States::S0TestHaKunit && event == Events::Event1TestHaKunit {
        unsafe { ha_reset_env(ha_mon, EnvsTestHaKunit::ClkTestHaKunit, time_ns) };
    } else if curr_state == States::S1TestHaKunit && event == Events::Event0TestHaKunit {
        unsafe { ha_reset_env(ha_mon, EnvsTestHaKunit::ClkTestHaKunit, time_ns) };
    } else if curr_state == States::S1TestHaKunit && event == Events::Event2TestHaKunit {
        res = unsafe { ha_get_env(ha_mon, EnvsTestHaKunit::Env1TestHaKunit, time_ns) } == 0u64;
        unsafe { ha_reset_env(ha_mon, EnvsTestHaKunit::ClkTestHaKunit, time_ns) };
    } else if curr_state == States::S2TestHaKunit && event == Events::Event1TestHaKunit {
        res = unsafe { ha_monitor_env_invalid(ha_mon, EnvsTestHaKunit::ClkTestHaKunit) }
            || unsafe { ha_get_env(ha_mon, EnvsTestHaKunit::ClkTestHaKunit, time_ns) }
                < unsafe { foo_ns };
    } else if curr_state == States::S3TestHaKunit && event == Events::Event0TestHaKunit {
        res = unsafe { ha_monitor_env_invalid(ha_mon, EnvsTestHaKunit::ClkTestHaKunit) }
            || (unsafe { ha_get_env(ha_mon, EnvsTestHaKunit::ClkTestHaKunit, time_ns) }
                < unsafe { FOO_NS() }
                && unsafe { ha_get_env(ha_mon, EnvsTestHaKunit::Env2TestHaKunit, time_ns) }
                    == 0u64);
    } else if curr_state == States::S3TestHaKunit && event == Events::Event1TestHaKunit {
        res = unsafe { ha_monitor_env_invalid(ha_mon, EnvsTestHaKunit::ClkTestHaKunit) }
            || (unsafe { ha_get_env(ha_mon, EnvsTestHaKunit::ClkTestHaKunit, time_ns) } < 5000u64
                && unsafe { ha_get_env(ha_mon, EnvsTestHaKunit::Env1TestHaKunit, time_ns) }
                    == 1u64);
        unsafe { ha_reset_env(ha_mon, EnvsTestHaKunit::ClkTestHaKunit, time_ns) };
    }
    res
}

#[inline]
unsafe fn ha_setup_invariants(
    ha_mon: *mut HaMonitor,
    curr_state: States,
    event: Events,
    next_state: States,
    time_ns: U64,
) {
    if next_state == curr_state && event != Events::Event0TestHaKunit {
        return;
    }
    if next_state == States::S0TestHaKunit {
        unsafe {
            ha_start_timer_ns(
                ha_mon,
                EnvsTestHaKunit::ClkTestHaKunit,
                bar_ns(ha_mon),
                time_ns,
            )
        };
    } else if next_state == States::S2TestHaKunit {
        unsafe {
            ha_start_timer_ns(
                ha_mon,
                EnvsTestHaKunit::ClkTestHaKunit,
                BAR_NS(ha_mon),
                time_ns,
            )
        };
    } else if curr_state == States::S0TestHaKunit {
        unsafe { ha_cancel_timer(ha_mon) };
    } else if curr_state == States::S2TestHaKunit {
        unsafe { ha_cancel_timer(ha_mon) };
    }
}

unsafe fn ha_verify_constraint(
    ha_mon: *mut HaMonitor,
    curr_state: States,
    event: Events,
    next_state: States,
    time_ns: U64,
) -> bool {
    if !unsafe { ha_verify_invariants(ha_mon, curr_state, event, next_state, time_ns) } {
        return false;
    }

    if !unsafe { ha_verify_guards(ha_mon, curr_state, event, next_state, time_ns) } {
        return false;
    }

    unsafe { ha_setup_invariants(ha_mon, curr_state, event, next_state, time_ns) };

    true
}

unsafe extern "C" fn handle_event0(_data: *mut ::core::ffi::c_void) {
    /* XXX: validate that this event always leads to the initial state */
    let p: *mut TaskStruct = {
        /* XXX: how do I get p? */
        todo!()
    };
    unsafe { da_handle_start_event(p, Events::Event0TestHaKunit) };
}

unsafe extern "C" fn handle_event1(_data: *mut ::core::ffi::c_void) {
    let p: *mut TaskStruct = {
        /* XXX: how do I get p? */
        todo!()
    };
    unsafe { da_handle_event(p, Events::Event1TestHaKunit) };
}

unsafe extern "C" fn handle_event2(_data: *mut ::core::ffi::c_void) {
    let p: *mut TaskStruct = {
        /* XXX: how do I get p? */
        todo!()
    };
    unsafe { da_handle_event(p, Events::Event2TestHaKunit) };
}

unsafe extern "C" fn enable_test_ha_kunit() -> ::core::ffi::c_int {
    let retval: ::core::ffi::c_int;

    retval = unsafe { ha_monitor_init() };
    if retval != 0 {
        return retval;
    }

    unsafe {
        rv_attach_trace_probe(
            c"test_ha_kunit".as_ptr(),
            /* XXX: tracepoint */
            ::core::ptr::null(),
            handle_event0 as *const ::core::ffi::c_void,
        );
        rv_attach_trace_probe(
            c"test_ha_kunit".as_ptr(),
            /* XXX: tracepoint */
            ::core::ptr::null(),
            handle_event1 as *const ::core::ffi::c_void,
        );
        rv_attach_trace_probe(
            c"test_ha_kunit".as_ptr(),
            /* XXX: tracepoint */
            ::core::ptr::null(),
            handle_event2 as *const ::core::ffi::c_void,
        );
    }

    0
}

unsafe extern "C" fn disable_test_ha_kunit() {
    unsafe {
        rv_this.enabled = 0;

        rv_detach_trace_probe(
            c"test_ha_kunit".as_ptr(),
            /* XXX: tracepoint */
            ::core::ptr::null(),
            handle_event0 as *const ::core::ffi::c_void,
        );
        rv_detach_trace_probe(
            c"test_ha_kunit".as_ptr(),
            /* XXX: tracepoint */
            ::core::ptr::null(),
            handle_event1 as *const ::core::ffi::c_void,
        );
        rv_detach_trace_probe(
            c"test_ha_kunit".as_ptr(),
            /* XXX: tracepoint */
            ::core::ptr::null(),
            handle_event2 as *const ::core::ffi::c_void,
        );

        ha_monitor_destroy();
    }
}

/*
 * This is the monitor register section.
 */
#[unsafe(no_mangle)]
pub static mut rv_this: RvMonitor = RvMonitor {
    name: c"test_ha_kunit".as_ptr(),
    description: c"auto-generated".as_ptr(),
    enable: Some(enable_test_ha_kunit),
    disable: Some(disable_test_ha_kunit),
    reset: Some(da_monitor_reset_all),
    enabled: 0,
};

unsafe extern "C" fn register_test_ha_kunit() -> ::core::ffi::c_int {
    unsafe { rv_register_monitor(&raw mut rv_this, ::core::ptr::null_mut()) }
}

unsafe extern "C" fn unregister_test_ha_kunit() {
    unsafe { rv_unregister_monitor(&raw mut rv_this) };
}

// module_init(register_test_ha_kunit);
// module_exit(unregister_test_ha_kunit);

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("rvgen: auto-generated");
// MODULE_DESCRIPTION("test_ha_kunit: auto-generated");

// Original condition: #if IS_ENABLED(CONFIG_RV_MONITORS_KUNIT_TEST)
#[cfg(CONFIG_RV_MONITORS_KUNIT_TEST)]
mod kunit_visibility {
    use super::*;

    #[repr(C)]
    pub struct RvTestHaKunitOps {
        pub mon: ::core::mem::MaybeUninit<()>,
        pub handle_event0: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void)>,
        pub handle_event1: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void)>,
        pub handle_event2: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void)>,
    }

    // Original headers: kunit/visibility.h and test_ha_kunit_kunit.h.
    // RV_MON_OPS_INIT() is a missing external dependency in this isolated file.
    #[unsafe(no_mangle)]
    pub static rv_test_ha_kunit_ops: RvTestHaKunitOps = RvTestHaKunitOps {
        mon: ::core::mem::MaybeUninit::uninit(),
        handle_event0: Some(handle_event0),
        handle_event1: Some(handle_event1),
        handle_event2: Some(handle_event2),
    };

    // EXPORT_SYMBOL_IF_KUNIT(rv_test_ha_kunit_ops);
}
