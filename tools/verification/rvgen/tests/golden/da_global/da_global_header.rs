/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Automatically generated C representation of da_global automaton
 * For further information about this format, see kernel documentation:
 *   Documentation/trace/rv/deterministic_automata.rst
 */

use core::ffi::c_char;

pub const MONITOR_NAME: &[u8] = b"da_global\0";

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum states_da_global {
    state_a_da_global,
    state_b_da_global,
    state_max_da_global,
}

pub const INVALID_STATE: states_da_global = states_da_global::state_max_da_global;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum events_da_global {
    event_1_da_global,
    event_2_da_global,
    event_max_da_global,
}

pub const STATE_MAX_DA_GLOBAL: usize = states_da_global::state_max_da_global as usize;
pub const EVENT_MAX_DA_GLOBAL: usize = events_da_global::event_max_da_global as usize;

#[repr(C)]
pub struct automaton_da_global {
    pub state_names: [*mut c_char; STATE_MAX_DA_GLOBAL],
    pub event_names: [*mut c_char; EVENT_MAX_DA_GLOBAL],
    pub function: [[u8; EVENT_MAX_DA_GLOBAL]; STATE_MAX_DA_GLOBAL],
    pub initial_state: u8,
    pub final_states: [bool; STATE_MAX_DA_GLOBAL],
}

pub const automaton_da_global: automaton_da_global = automaton_da_global {
    state_names: [
        b"state_a\0".as_ptr() as *mut c_char,
        b"state_b\0".as_ptr() as *mut c_char,
    ],
    event_names: [
        b"event_1\0".as_ptr() as *mut c_char,
        b"event_2\0".as_ptr() as *mut c_char,
    ],
    function: [
        [
            states_da_global::state_b_da_global as u8,
            states_da_global::state_a_da_global as u8,
        ],
        [
            INVALID_STATE as u8,
            states_da_global::state_a_da_global as u8,
        ],
    ],
    initial_state: states_da_global::state_a_da_global as u8,
    final_states: [true, false],
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
