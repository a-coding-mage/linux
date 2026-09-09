/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Automatically generated C representation of nomiss automaton
 * For further information about this format, see kernel documentation:
 *   Documentation/trace/rv/deterministic_automata.rst
 */

// #define MONITOR_NAME nomiss
pub const MONITOR_NAME: &str = "nomiss";

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum states_nomiss {
    ready_nomiss,
    idle_nomiss,
    running_nomiss,
    sleeping_nomiss,
    throttled_nomiss,
    state_max_nomiss,
}

pub const INVALID_STATE: states_nomiss = states_nomiss::state_max_nomiss;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum events_nomiss {
    dl_replenish_nomiss,
    dl_server_idle_nomiss,
    dl_server_stop_nomiss,
    dl_throttle_nomiss,
    sched_switch_in_nomiss,
    sched_switch_suspend_nomiss,
    sched_wakeup_nomiss,
    event_max_nomiss,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum envs_nomiss {
    clk_nomiss,
    is_constr_dl_nomiss,
    is_defer_nomiss,
    env_max_nomiss,
}

pub const env_max_stored_nomiss: envs_nomiss = envs_nomiss::is_constr_dl_nomiss;
// _Static_assert(env_max_stored_nomiss <= MAX_HA_ENV_LEN, "Not enough slots");
// Requires the external MAX_HA_ENV_LEN build-time constant.
// #define HA_CLK_NS

#[repr(C)]
pub struct automaton_nomiss {
    pub state_names: [*const ::core::ffi::c_char; states_nomiss::state_max_nomiss as usize],
    pub event_names: [*const ::core::ffi::c_char; events_nomiss::event_max_nomiss as usize],
    pub env_names: [*const ::core::ffi::c_char; envs_nomiss::env_max_nomiss as usize],
    pub function: [[u8; events_nomiss::event_max_nomiss as usize]; states_nomiss::state_max_nomiss as usize],
    pub initial_state: u8,
    pub final_states: [bool; states_nomiss::state_max_nomiss as usize],
}

pub static automaton_nomiss: automaton_nomiss = automaton_nomiss {
    state_names: [
        b"ready\0".as_ptr() as *const ::core::ffi::c_char,
        b"idle\0".as_ptr() as *const ::core::ffi::c_char,
        b"running\0".as_ptr() as *const ::core::ffi::c_char,
        b"sleeping\0".as_ptr() as *const ::core::ffi::c_char,
        b"throttled\0".as_ptr() as *const ::core::ffi::c_char,
    ],
    event_names: [
        b"dl_replenish\0".as_ptr() as *const ::core::ffi::c_char,
        b"dl_server_idle\0".as_ptr() as *const ::core::ffi::c_char,
        b"dl_server_stop\0".as_ptr() as *const ::core::ffi::c_char,
        b"dl_throttle\0".as_ptr() as *const ::core::ffi::c_char,
        b"sched_switch_in\0".as_ptr() as *const ::core::ffi::c_char,
        b"sched_switch_suspend\0".as_ptr() as *const ::core::ffi::c_char,
        b"sched_wakeup\0".as_ptr() as *const ::core::ffi::c_char,
    ],
    env_names: [
        b"clk\0".as_ptr() as *const ::core::ffi::c_char,
        b"is_constr_dl\0".as_ptr() as *const ::core::ffi::c_char,
        b"is_defer\0".as_ptr() as *const ::core::ffi::c_char,
    ],
    function: [
        [0, 1, 3, 4, 2, 5, 0],
        [0, 1, 3, 4, 2, 5, 5],
        [2, 1, 3, 4, 2, 3, 2],
        [0, 3, 3, 4, 2, 5, 0],
        [0, 4, 5, 4, 5, 4, 4],
    ],
    initial_state: 0,
    final_states: [true, false, false, false, false],
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
