/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Automatically generated C representation of da_perobj_parent automaton
 * For further information about this format, see kernel documentation:
 *   Documentation/trace/rv/deterministic_automata.rst
 */

pub const MONITOR_NAME: &str = "da_perobj_parent";

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum states_da_perobj_parent {
    state_a_da_perobj_parent = 0,
    state_b_da_perobj_parent = 1,
    state_c_da_perobj_parent = 2,
    state_max_da_perobj_parent = 3,
}

pub const INVALID_STATE: states_da_perobj_parent =
    states_da_perobj_parent::state_max_da_perobj_parent;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum events_da_perobj_parent {
    event_1_da_perobj_parent = 0,
    event_2_da_perobj_parent = 1,
    event_3_da_perobj_parent = 2,
    event_max_da_perobj_parent = 3,
}

pub const state_max_da_perobj_parent: usize =
    states_da_perobj_parent::state_max_da_perobj_parent as usize;
pub const event_max_da_perobj_parent: usize =
    events_da_perobj_parent::event_max_da_perobj_parent as usize;

#[repr(C)]
pub struct automaton_da_perobj_parent {
    pub state_names: [*mut core::ffi::c_char; state_max_da_perobj_parent],
    pub event_names: [*mut core::ffi::c_char; event_max_da_perobj_parent],
    pub function: [[u8; event_max_da_perobj_parent]; state_max_da_perobj_parent],
    pub initial_state: u8,
    pub final_states: [bool; state_max_da_perobj_parent],
}

pub const automaton_da_perobj_parent: automaton_da_perobj_parent = automaton_da_perobj_parent {
    state_names: [
        b"state_a\0".as_ptr() as *mut core::ffi::c_char,
        b"state_b\0".as_ptr() as *mut core::ffi::c_char,
        b"state_c\0".as_ptr() as *mut core::ffi::c_char,
    ],
    event_names: [
        b"event_1\0".as_ptr() as *mut core::ffi::c_char,
        b"event_2\0".as_ptr() as *mut core::ffi::c_char,
        b"event_3\0".as_ptr() as *mut core::ffi::c_char,
    ],
    function: [
        [
            states_da_perobj_parent::state_b_da_perobj_parent as u8,
            states_da_perobj_parent::state_c_da_perobj_parent as u8,
            INVALID_STATE as u8,
        ],
        [
            INVALID_STATE as u8,
            states_da_perobj_parent::state_a_da_perobj_parent as u8,
            states_da_perobj_parent::state_c_da_perobj_parent as u8,
        ],
        [
            INVALID_STATE as u8,
            INVALID_STATE as u8,
            INVALID_STATE as u8,
        ],
    ],
    initial_state: states_da_perobj_parent::state_a_da_perobj_parent as u8,
    final_states: [true, false, false],
};
