/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Automatically generated C representation of sco automaton
 * For further information about this format, see kernel documentation:
 *   Documentation/trace/rv/deterministic_automata.rst
 */

use core::ffi::c_char;

pub const MONITOR_NAME: &str = "sco";

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum states_sco {
    thread_context_sco,
    scheduling_context_sco,
    state_max_sco,
}

pub const INVALID_STATE: u8 = states_sco::state_max_sco as u8;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum events_sco {
    sched_set_state_sco,
    schedule_entry_sco,
    schedule_exit_sco,
    event_max_sco,
}

#[repr(C)]
pub struct automaton_sco {
    pub state_names: [*const c_char; states_sco::state_max_sco as usize],
    pub event_names: [*const c_char; events_sco::event_max_sco as usize],
    pub function: [[u8; events_sco::event_max_sco as usize]; states_sco::state_max_sco as usize],
    pub initial_state: u8,
    pub final_states: [bool; states_sco::state_max_sco as usize],
}

pub static automaton_sco: automaton_sco = automaton_sco {
    state_names: [
        b"thread_context\0".as_ptr() as *const c_char,
        b"scheduling_context\0".as_ptr() as *const c_char,
    ],
    event_names: [
        b"sched_set_state\0".as_ptr() as *const c_char,
        b"schedule_entry\0".as_ptr() as *const c_char,
        b"schedule_exit\0".as_ptr() as *const c_char,
    ],
    function: [
        [
            states_sco::thread_context_sco as u8,
            states_sco::scheduling_context_sco as u8,
            INVALID_STATE,
        ],
        [
            INVALID_STATE,
            INVALID_STATE,
            states_sco::thread_context_sco as u8,
        ],
    ],
    initial_state: states_sco::thread_context_sco as u8,
    final_states: [true, false],
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
