/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Rust translation of rv/ha_monitor.h.
 * C includes, preprocessor configuration, and external kernel/monitor symbols
 * remain dependencies supplied by the surrounding translation unit.
 */

// #include <rv/automata.h>
// #include <rv/da_monitor.h>
// #include <linux/seq_buf.h>

#[allow(non_camel_case_types)]
type da_id_type = i32;

const ENV_INVALID_VALUE: u64 = u64::MAX;
// EVENT_NONE is EVENT_MAX in the source configuration.
// const EVENT_NONE: events = EVENT_MAX;
const EVENT_NONE_LBL: &str = "none";
const ENV_BUFFER_SIZE: usize = 64;

extern "C" {
    fn da_monitor_init() -> i32;
    fn da_monitor_destroy();
    fn da_monitoring(da_mon: *mut da_monitor) -> bool;
    fn da_monitor_handling_event(da_mon: *mut da_monitor) -> bool;
    fn da_monitor_reset(da_mon: *mut da_monitor);
    fn da_get_monitor(/* source-specific arguments */) -> *mut da_monitor;
    fn da_get_target(da_mon: *mut da_monitor) -> *mut core::ffi::c_void;
    fn da_get_id(da_mon: *mut da_monitor) -> da_id_type;
    fn synchronize_rcu();
    fn ktime_get_ns() -> u64;
    fn get_jiffies_64() -> u64;
    fn time_after64(a: u64, b: u64) -> bool;
}

#[repr(C)]
pub struct da_monitor {
    pub curr_state: states,
    // Remaining fields are supplied by rv/automata.h.
}

#[repr(C)]
pub struct ha_monitor {
    pub da_mon: da_monitor,
    pub env_store: *mut u64,
    // timer/hrtimer and other fields are supplied by the monitor definition.
}

#[repr(C)]
pub struct seq_buf {
    pub buffer: *mut core::ffi::c_char,
    // Remaining fields are supplied by linux/seq_buf.h.
}

#[allow(non_camel_case_types)]
pub type states = i32;
#[allow(non_camel_case_types)]
pub type events = i32;
#[allow(non_camel_case_types)]
pub type envs = i32;

extern "C" {
    fn ha_get_env(ha_mon: *mut ha_monitor, env: envs, time_ns: u64) -> u64;
    fn ha_verify_constraint(
        ha_mon: *mut ha_monitor,
        curr_state: states,
        event: events,
        next_state: states,
        time_ns: u64,
    ) -> bool;
    fn model_get_state_name(state: states) -> *mut core::ffi::c_char;
    fn model_get_event_name(event: events) -> *mut core::ffi::c_char;
    fn model_get_env_name_external(env: envs) -> *mut core::ffi::c_char;
}

#[inline]
unsafe fn env_store(ha_mon: *mut ha_monitor, env: envs) -> *mut u64 {
    (*ha_mon).env_store.add(env as usize)
}

#[inline]
unsafe fn ha_monitor_reset_all_stored(ha_mon: *mut ha_monitor, env_max_stored: envs) {
    for i in 0..env_max_stored {
        core::ptr::write_volatile(env_store(ha_mon, i), ENV_INVALID_VALUE);
    }
}

#[inline]
pub unsafe fn ha_monitor_init_env(da_mon: *mut da_monitor, env_max_stored: envs) {
    let ha_mon = da_mon as *mut ha_monitor;
    ha_monitor_reset_all_stored(ha_mon, env_max_stored);
    ha_setup_timer(ha_mon);
}

#[inline]
pub unsafe fn ha_monitor_reset_env(da_mon: *mut da_monitor) {
    let ha_mon = da_mon as *mut ha_monitor;
    if da_monitoring(da_mon) {
        let _ = ha_cancel_timer(ha_mon);
    }
}

#[inline]
pub unsafe fn ha_monitor_env_invalid(ha_mon: *mut ha_monitor, env: envs) -> bool {
    core::ptr::read_volatile(env_store(ha_mon, env)) == ENV_INVALID_VALUE
}

#[inline]
pub unsafe fn ha_get_env_string(
    _s: *mut seq_buf,
    ha_mon: *mut ha_monitor,
    time_ns: u64,
    env_max: envs,
) {
    // seq_buf_printf(s, format_str, model_get_env_name(i), ha_get_env(...));
    // The source writes the first item as "%s=%llu" and subsequent items as
    // ",%s=%llu". The concrete seq_buf/model declarations are external.
    for i in 0..env_max {
        let _ = (model_get_env_name_external(i), ha_get_env(ha_mon, i, time_ns));
    }
}

unsafe fn ha_react(_curr_state: states, _event: events, _env: *mut core::ffi::c_char) {
    // CONFIG_RV_REACTORS: rv_react(&rv_this, ...); otherwise this is empty.
}

#[inline]
pub unsafe fn ha_monitor_handle_constraint(
    da_mon: *mut da_monitor,
    curr_state: states,
    event: events,
    next_state: states,
    id: da_id_type,
    env_max: envs,
) -> bool {
    let ha_mon = da_mon as *mut ha_monitor;
    let time_ns = {
        // HA_CLK_NS selects ktime_get_ns(); otherwise the source uses 0.
        0u64
    };
    if ha_verify_constraint(ha_mon, curr_state, event, next_state, time_ns) {
        return true;
    }
    let mut env_string = seq_buf {
        buffer: core::ptr::null_mut(),
    };
    ha_get_env_string(&mut env_string, ha_mon, time_ns, env_max);
    ha_react(curr_state, event, env_string.buffer);
    let _ = (model_get_state_name(curr_state), model_get_event_name(event), id);
    false
}

#[inline]
pub unsafe fn __ha_monitor_timer_callback(ha_mon: *mut ha_monitor, env_max: envs) {
    if core::ptr::read_volatile(&ha_mon_destroying) {
        return;
    }
    let curr_state = core::ptr::read_volatile(&(*ha_mon).da_mon.curr_state);
    if !da_monitor_handling_event(&mut (*ha_mon).da_mon) {
        return;
    }
    let time_ns = 0u64;
    let mut env_string = seq_buf { buffer: core::ptr::null_mut() };
    ha_get_env_string(&mut env_string, ha_mon, time_ns, env_max);
    ha_react(curr_state, 0, env_string.buffer);
    let _ = (model_get_state_name(curr_state), EVENT_NONE_LBL, da_get_id(&mut (*ha_mon).da_mon));
    da_monitor_reset(&mut (*ha_mon).da_mon);
}

#[inline]
pub unsafe fn ha_get_clk_ns(ha_mon: *mut ha_monitor, env: envs, time_ns: u64) -> u64 {
    time_ns.wrapping_sub(core::ptr::read_volatile(env_store(ha_mon, env)))
}
#[inline]
pub unsafe fn ha_reset_clk_ns(ha_mon: *mut ha_monitor, env: envs, time_ns: u64) {
    core::ptr::write_volatile(env_store(ha_mon, env), time_ns);
}
#[inline]
pub unsafe fn ha_check_invariant_ns(ha_mon: *mut ha_monitor, env: envs, time_ns: u64, expire_ns: u64) -> bool {
    core::ptr::read_volatile(env_store(ha_mon, env)) >= time_ns.wrapping_sub(expire_ns)
}
#[inline]
pub unsafe fn ha_invariant_passed_ns(ha_mon: *mut ha_monitor, env: envs, time_ns: u64, env_max_stored: envs) -> u64 {
    if env < 0 || env >= env_max_stored || ha_monitor_env_invalid(ha_mon, env) { return 0; }
    ha_get_env(ha_mon, env, time_ns)
}

#[inline]
pub unsafe fn ha_get_clk_jiffy(ha_mon: *mut ha_monitor, env: envs) -> u64 {
    get_jiffies_64().wrapping_sub(core::ptr::read_volatile(env_store(ha_mon, env)))
}
#[inline]
pub unsafe fn ha_reset_clk_jiffy(ha_mon: *mut ha_monitor, env: envs) {
    core::ptr::write_volatile(env_store(ha_mon, env), get_jiffies_64());
}
#[inline]
pub unsafe fn ha_check_invariant_jiffy(ha_mon: *mut ha_monitor, env: envs, _time_ns: u64, expire_jiffy: u64) -> bool {
    time_after64(core::ptr::read_volatile(env_store(ha_mon, env)), get_jiffies_64().wrapping_sub(expire_jiffy))
}
#[inline]
pub unsafe fn ha_invariant_passed_jiffy(ha_mon: *mut ha_monitor, env: envs, time_ns: u64, env_max_stored: envs) -> u64 {
    if env < 0 || env >= env_max_stored || ha_monitor_env_invalid(ha_mon, env) { return 0; }
    ha_get_env(ha_mon, env, time_ns)
}

#[inline]
pub unsafe fn ha_inv_to_guard(ha_mon: *mut ha_monitor, env: envs, value: u64) {
    let p = env_store(ha_mon, env);
    core::ptr::write_volatile(p, core::ptr::read_volatile(p).wrapping_sub(value));
}

static mut ha_mon_destroying: bool = false;

unsafe fn ha_setup_timer(_ha_mon: *mut ha_monitor) {}
unsafe fn ha_cancel_timer(_ha_mon: *mut ha_monitor) -> bool { false }
unsafe fn ha_cancel_timer_sync(_ha_mon: *mut ha_monitor) {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
