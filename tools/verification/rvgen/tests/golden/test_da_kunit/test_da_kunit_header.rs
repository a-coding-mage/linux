/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Automatically generated Rust representation of test_da_kunit automaton
 * For further information about this format, see kernel documentation:
 *   Documentation/trace/rv/deterministic_automata.rst
 */

use core::ffi::c_char;

pub const MONITOR_NAME: &str = "test_da_kunit";

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum states_test_da_kunit {
    state_a_test_da_kunit,
    state_b_test_da_kunit,
    state_max_test_da_kunit,
}

pub const INVALID_STATE: states_test_da_kunit = states_test_da_kunit::state_max_test_da_kunit;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum events_test_da_kunit {
    event_1_test_da_kunit,
    event_2_test_da_kunit,
    event_max_test_da_kunit,
}

#[repr(C)]
pub struct automaton_test_da_kunit {
    pub state_names: [*mut c_char; states_test_da_kunit::state_max_test_da_kunit as usize],
    pub event_names: [*mut c_char; events_test_da_kunit::event_max_test_da_kunit as usize],
    pub function: [[u8; events_test_da_kunit::event_max_test_da_kunit as usize];
        states_test_da_kunit::state_max_test_da_kunit as usize],
    pub initial_state: u8,
    pub final_states: [bool; states_test_da_kunit::state_max_test_da_kunit as usize],
}

pub const automaton_test_da_kunit: automaton_test_da_kunit = automaton_test_da_kunit {
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
            states_test_da_kunit::state_b_test_da_kunit as u8,
            states_test_da_kunit::state_a_test_da_kunit as u8,
        ],
        [
            INVALID_STATE as u8,
            states_test_da_kunit::state_a_test_da_kunit as u8,
        ],
    ],
    initial_state: states_test_da_kunit::state_a_test_da_kunit as u8,
    final_states: [true, false],
};
