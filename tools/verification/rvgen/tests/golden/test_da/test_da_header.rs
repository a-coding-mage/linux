/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Automatically generated C representation of test_da automaton
 * For further information about this format, see kernel documentation:
 *   Documentation/trace/rv/deterministic_automata.rst
 */

use core::ffi::c_char;

pub const MONITOR_NAME: &str = "test_da";

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum states_test_da {
    state_a_test_da,
    state_b_test_da,
    state_max_test_da,
}

pub const INVALID_STATE: states_test_da = states_test_da::state_max_test_da;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum events_test_da {
    event_1_test_da,
    event_2_test_da,
    event_max_test_da,
}

#[repr(C)]
pub struct automaton_test_da {
    pub state_names: [*const c_char; states_test_da::state_max_test_da as usize],
    pub event_names: [*const c_char; events_test_da::event_max_test_da as usize],
    pub function: [[u8; events_test_da::event_max_test_da as usize]; states_test_da::state_max_test_da as usize],
    pub initial_state: u8,
    pub final_states: [bool; states_test_da::state_max_test_da as usize],
}

unsafe impl Sync for automaton_test_da {}

pub static automaton_test_da: automaton_test_da = automaton_test_da {
    state_names: [b"state_a\0".as_ptr() as *const c_char, b"state_b\0".as_ptr() as *const c_char],
    event_names: [b"event_1\0".as_ptr() as *const c_char, b"event_2\0".as_ptr() as *const c_char],
    function: [
        [
            states_test_da::state_b_test_da as u8,
            states_test_da::state_a_test_da as u8,
        ],
        [
            INVALID_STATE as u8,
            states_test_da::state_a_test_da as u8,
        ],
    ],
    initial_state: states_test_da::state_a_test_da as u8,
    final_states: [true, false],
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
