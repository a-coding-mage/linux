/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Automatically generated C representation of snroc automaton
 * For further information about this format, see kernel documentation:
 *   Documentation/trace/rv/deterministic_automata.rst
 */

// #define MONITOR_NAME snroc
pub const MONITOR_NAME: &str = "snroc";

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum states_snroc {
    other_context_snroc,
    own_context_snroc,
    state_max_snroc,
}

pub const INVALID_STATE: states_snroc = states_snroc::state_max_snroc;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum events_snroc {
    sched_set_state_snroc,
    sched_switch_in_snroc,
    sched_switch_out_snroc,
    event_max_snroc,
}

#[repr(C)]
pub struct automaton_snroc {
    pub state_names: [*const core::ffi::c_char; states_snroc::state_max_snroc as usize],
    pub event_names: [*const core::ffi::c_char; events_snroc::event_max_snroc as usize],
    pub function:
        [[u8; events_snroc::event_max_snroc as usize]; states_snroc::state_max_snroc as usize],
    pub initial_state: u8,
    pub final_states: [bool; states_snroc::state_max_snroc as usize],
}

pub static automaton_snroc: automaton_snroc = automaton_snroc {
    state_names: [
        b"other_context\0".as_ptr() as *const core::ffi::c_char,
        b"own_context\0".as_ptr() as *const core::ffi::c_char,
    ],
    event_names: [
        b"sched_set_state\0".as_ptr() as *const core::ffi::c_char,
        b"sched_switch_in\0".as_ptr() as *const core::ffi::c_char,
        b"sched_switch_out\0".as_ptr() as *const core::ffi::c_char,
    ],
    function: [
        [
            states_snroc::state_max_snroc as u8,
            states_snroc::own_context_snroc as u8,
            states_snroc::state_max_snroc as u8,
        ],
        [
            states_snroc::own_context_snroc as u8,
            states_snroc::state_max_snroc as u8,
            states_snroc::other_context_snroc as u8,
        ],
    ],
    initial_state: states_snroc::other_context_snroc as u8,
    final_states: [true, false],
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
