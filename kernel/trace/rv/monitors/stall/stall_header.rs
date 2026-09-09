/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Automatically generated C representation of stall automaton
 * For further information about this format, see kernel documentation:
 *   Documentation/trace/rv/deterministic_automata.rst
 */

pub const MONITOR_NAME: &str = "stall";

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum states_stall {
    dequeued_stall,
    enqueued_stall,
    running_stall,
    state_max_stall,
}

pub const INVALID_STATE: states_stall = states_stall::state_max_stall;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum events_stall {
    sched_switch_in_stall,
    sched_switch_preempt_stall,
    sched_switch_wait_stall,
    sched_wakeup_stall,
    event_max_stall,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum envs_stall {
    clk_stall,
    env_max_stall,
    env_max_stored_stall = envs_stall::env_max_stall as isize,
}

/* _Static_assert(env_max_stored_stall <= MAX_HA_ENV_LEN, "Not enough slots"); */

#[repr(C)]
pub struct automaton_stall {
    pub state_names: [*const std::os::raw::c_char; states_stall::state_max_stall as usize],
    pub event_names: [*const std::os::raw::c_char; events_stall::event_max_stall as usize],
    pub env_names: [*const std::os::raw::c_char; envs_stall::env_max_stall as usize],
    pub function: [[u8; events_stall::event_max_stall as usize]; states_stall::state_max_stall as usize],
    pub initial_state: u8,
    pub final_states: [bool; states_stall::state_max_stall as usize],
}

pub static mut automaton_stall: automaton_stall = automaton_stall {
    state_names: [
        b"dequeued\0".as_ptr() as *const std::os::raw::c_char,
        b"enqueued\0".as_ptr() as *const std::os::raw::c_char,
        b"running\0".as_ptr() as *const std::os::raw::c_char,
    ],
    event_names: [
        b"sched_switch_in\0".as_ptr() as *const std::os::raw::c_char,
        b"sched_switch_preempt\0".as_ptr() as *const std::os::raw::c_char,
        b"sched_switch_wait\0".as_ptr() as *const std::os::raw::c_char,
        b"sched_wakeup\0".as_ptr() as *const std::os::raw::c_char,
    ],
    env_names: [b"clk\0".as_ptr() as *const std::os::raw::c_char],
    function: [
        [
            states_stall::state_max_stall as u8,
            states_stall::state_max_stall as u8,
            states_stall::state_max_stall as u8,
            states_stall::enqueued_stall as u8,
        ],
        [
            states_stall::running_stall as u8,
            states_stall::state_max_stall as u8,
            states_stall::state_max_stall as u8,
            states_stall::enqueued_stall as u8,
        ],
        [
            states_stall::running_stall as u8,
            states_stall::enqueued_stall as u8,
            states_stall::dequeued_stall as u8,
            states_stall::running_stall as u8,
        ],
    ],
    initial_state: states_stall::dequeued_stall as u8,
    final_states: [true, false, false],
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
