/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Runtime Verification.
 *
 * For futher information, see: kernel/trace/rv/rv.c.
 */

pub const MAX_DA_NAME_LEN: usize = 32;
pub const MAX_DA_RETRY_RACING_EVENTS: usize = 3;

pub const RV_MON_GLOBAL: i32 = 0;
pub const RV_MON_PER_CPU: i32 = 1;
pub const RV_MON_PER_TASK: i32 = 2;
pub const RV_MON_PER_OBJ: i32 = 3;

/* The following declarations are present when CONFIG_RV is enabled. */

/* Deterministic automaton per-object variables. */
#[repr(C)]
pub struct da_monitor {
    pub monitoring: bool,
    pub curr_state: ::core::ffi::c_uint,
}

/* CONFIG_RV_LTL_MONITOR declarations. */
pub const RV_MAX_LTL_ATOM: usize = 32;
pub const RV_MAX_BA_STATES: usize = 32;

#[repr(C)]
pub struct ltl_monitor {
    /* DECLARE_BITMAP(states, RV_MAX_BA_STATES); */
    pub states: [usize; 1],
    /* DECLARE_BITMAP(atoms, RV_MAX_LTL_ATOM); */
    pub atoms: [usize; 1],
    /* DECLARE_BITMAP(unknown_atoms, RV_MAX_LTL_ATOM); */
    pub unknown_atoms: [usize; 1],
}

#[inline]
pub unsafe fn rv_ltl_valid_state(mon: *mut ltl_monitor) -> bool {
    for i in 0..1 {
        if (*mon).states[i] != 0 {
            return true;
        }
    }
    false
}

#[inline]
pub unsafe fn rv_ltl_all_atoms_known(mon: *mut ltl_monitor) -> bool {
    for i in 0..1 {
        if (*mon).unknown_atoms[i] != 0 {
            return false;
        }
    }
    true
}

/* Empty fallback used when CONFIG_RV_LTL_MONITOR is disabled. */
#[repr(C)]
pub struct ltl_monitor_disabled {}

/* CONFIG_RV_HA_MONITOR declarations. */
pub const MAX_HA_ENV_LEN: usize = 1;
pub const HA_TIMER_NONE: i32 = 0;
pub const HA_TIMER_WHEEL: i32 = 1;
pub const HA_TIMER_HRTIMER: i32 = 2;

#[repr(C)]
pub union ha_monitor_timer {
    pub hrtimer: hrtimer,
    pub timer: timer_list,
}

#[repr(C)]
pub struct ha_monitor {
    pub da_mon: da_monitor,
    pub env_store: [u64; MAX_HA_ENV_LEN],
    pub timer: ha_monitor_timer,
}

#[repr(C)]
pub struct ha_monitor_disabled {}

#[repr(C)]
pub union rv_task_monitor {
    pub da_mon: da_monitor,
    pub ltl_mon: ltl_monitor,
    pub ha_mon: ha_monitor,
}

pub const RV_PER_TASK_MONITOR_INIT: usize = CONFIG_RV_PER_TASK_MONITORS;

#[repr(C)]
pub struct rv_reactor {
    pub name: *const ::core::ffi::c_char,
    pub description: *const ::core::ffi::c_char,
    pub react: Option<unsafe extern "C" fn(*const ::core::ffi::c_char, ...)>,
    pub list: list_head,
}

#[repr(C)]
pub struct rv_monitor {
    pub name: *const ::core::ffi::c_char,
    pub description: *const ::core::ffi::c_char,
    pub enabled: bool,
    pub enable: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub disable: Option<unsafe extern "C" fn()>,
    pub reset: Option<unsafe extern "C" fn()>,
    pub reactor: *mut rv_reactor,
    pub react: Option<unsafe extern "C" fn(*const ::core::ffi::c_char, ...)>,
    pub list: list_head,
    pub parent: *mut rv_monitor,
    pub root_d: *mut dentry,
}

extern "C" {
    pub fn rv_monitoring_on() -> bool;
    pub fn rv_unregister_monitor(monitor: *mut rv_monitor) -> ::core::ffi::c_int;
    pub fn rv_register_monitor(monitor: *mut rv_monitor, parent: *mut rv_monitor) -> ::core::ffi::c_int;
    pub fn rv_get_task_monitor_slot() -> ::core::ffi::c_int;
    pub fn rv_put_task_monitor_slot(slot: ::core::ffi::c_int);
    pub fn rv_unregister_reactor(reactor: *mut rv_reactor) -> ::core::ffi::c_int;
    pub fn rv_register_reactor(reactor: *mut rv_reactor) -> ::core::ffi::c_int;
    pub fn rv_react(monitor: *mut rv_monitor, msg: *const ::core::ffi::c_char, ...);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
