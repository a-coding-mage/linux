/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Automatically generated C representation of opid automaton
 * For further information about this format, see kernel documentation:
 *   Documentation/trace/rv/deterministic_automata.rst
 */

pub const MONITOR_NAME: &str = "opid";

#[repr(C)]
#[derive(Copy, Clone)]
pub enum states_opid {
    any_opid,
    state_max_opid,
}

pub const INVALID_STATE: states_opid = states_opid::state_max_opid;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum events_opid {
    sched_need_resched_opid,
    sched_waking_opid,
    event_max_opid,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum envs_opid {
    irq_off_opid,
    preempt_off_opid,
    env_max_opid,
    env_max_stored_opid = 0,
}

// _Static_assert(env_max_stored_opid <= MAX_HA_ENV_LEN, "Not enough slots");

#[repr(C)]
pub struct automaton_opid {
    pub state_names: [*mut core::ffi::c_char; 1],
    pub event_names: [*mut core::ffi::c_char; 2],
    pub env_names: [*mut core::ffi::c_char; 2],
    pub function: [[u8; 2]; 1],
    pub initial_state: states_opid,
    pub final_states: [bool; 1],
}

pub static automaton_opid: automaton_opid = automaton_opid {
    state_names: [b"any\0".as_ptr() as *mut core::ffi::c_char],
    event_names: [
        b"sched_need_resched\0".as_ptr() as *mut core::ffi::c_char,
        b"sched_waking\0".as_ptr() as *mut core::ffi::c_char,
    ],
    env_names: [
        b"irq_off\0".as_ptr() as *mut core::ffi::c_char,
        b"preempt_off\0".as_ptr() as *mut core::ffi::c_char,
    ],
    function: [[states_opid::any_opid as u8, states_opid::any_opid as u8]],
    initial_state: states_opid::any_opid,
    final_states: [true],
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
