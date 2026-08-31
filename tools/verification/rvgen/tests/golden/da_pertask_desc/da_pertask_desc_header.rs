/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Automatically generated C representation of da_pertask_desc automaton
 * For further information about this format, see kernel documentation:
 *   Documentation/trace/rv/deterministic_automata.rst
 */

use core::ffi::c_char;

pub const MONITOR_NAME: &str = "da_pertask_desc";

pub const state_a_da_pertask_desc: u32 = 0;
pub const state_b_da_pertask_desc: u32 = 1;
pub const state_c_da_pertask_desc: u32 = 2;
pub const state_max_da_pertask_desc: u32 = 3;

pub const INVALID_STATE: u32 = state_max_da_pertask_desc;

pub const event_1_da_pertask_desc: u32 = 0;
pub const event_2_da_pertask_desc: u32 = 1;
pub const event_3_da_pertask_desc: u32 = 2;
pub const event_max_da_pertask_desc: u32 = 3;

#[repr(C)]
pub struct automaton_da_pertask_desc {
    pub state_names: [*mut c_char; state_max_da_pertask_desc as usize],
    pub event_names: [*mut c_char; event_max_da_pertask_desc as usize],
    pub function:
        [[u8; event_max_da_pertask_desc as usize]; state_max_da_pertask_desc as usize],
    pub initial_state: u8,
    pub final_states: [bool; state_max_da_pertask_desc as usize],
}

pub const automaton_da_pertask_desc: automaton_da_pertask_desc = automaton_da_pertask_desc {
    state_names: [
        b"state_a\0".as_ptr() as *mut c_char,
        b"state_b\0".as_ptr() as *mut c_char,
        b"state_c\0".as_ptr() as *mut c_char,
    ],
    event_names: [
        b"event_1\0".as_ptr() as *mut c_char,
        b"event_2\0".as_ptr() as *mut c_char,
        b"event_3\0".as_ptr() as *mut c_char,
    ],
    function: [
        [
            state_b_da_pertask_desc as u8,
            state_c_da_pertask_desc as u8,
            INVALID_STATE as u8,
        ],
        [
            INVALID_STATE as u8,
            state_a_da_pertask_desc as u8,
            state_c_da_pertask_desc as u8,
        ],
        [
            INVALID_STATE as u8,
            INVALID_STATE as u8,
            INVALID_STATE as u8,
        ],
    ],
    initial_state: state_a_da_pertask_desc as u8,
    final_states: [true, false, false],
};
