/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Automatically generated C representation of test_ha automaton
 * For further information about this format, see kernel documentation:
 *   Documentation/trace/rv/deterministic_automata.rst
 */

use core::ffi::c_char;

pub const MONITOR_NAME: &str = "test_ha";

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum states_test_ha {
    S0_test_ha,
    S1_test_ha,
    S2_test_ha,
    S3_test_ha,
    state_max_test_ha,
}

pub const INVALID_STATE: states_test_ha = states_test_ha::state_max_test_ha;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum events_test_ha {
    event0_test_ha,
    event1_test_ha,
    event2_test_ha,
    event_max_test_ha,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum envs_test_ha {
    clk_test_ha,
    env1_test_ha,
    env2_test_ha,
    env_max_test_ha,
}

pub const env_max_stored_test_ha: envs_test_ha = envs_test_ha::env1_test_ha;

const _: () = assert!((env_max_stored_test_ha as usize) <= MAX_HA_ENV_LEN);

/* #define HA_CLK_NS */

#[repr(C)]
pub struct automaton_test_ha {
    pub state_names: [*mut c_char; states_test_ha::state_max_test_ha as usize],
    pub event_names: [*mut c_char; events_test_ha::event_max_test_ha as usize],
    pub env_names: [*mut c_char; envs_test_ha::env_max_test_ha as usize],
    pub function: [[u8; events_test_ha::event_max_test_ha as usize]; states_test_ha::state_max_test_ha as usize],
    pub initial_state: u8,
    pub final_states: [bool; states_test_ha::state_max_test_ha as usize],
}

pub const automaton_test_ha: automaton_test_ha = automaton_test_ha {
    state_names: [
        b"S0\0".as_ptr() as *mut c_char,
        b"S1\0".as_ptr() as *mut c_char,
        b"S2\0".as_ptr() as *mut c_char,
        b"S3\0".as_ptr() as *mut c_char,
    ],
    event_names: [
        b"event0\0".as_ptr() as *mut c_char,
        b"event1\0".as_ptr() as *mut c_char,
        b"event2\0".as_ptr() as *mut c_char,
    ],
    env_names: [
        b"clk\0".as_ptr() as *mut c_char,
        b"env1\0".as_ptr() as *mut c_char,
        b"env2\0".as_ptr() as *mut c_char,
    ],
    function: [
        [
            states_test_ha::S0_test_ha as u8,
            states_test_ha::S1_test_ha as u8,
            INVALID_STATE as u8,
        ],
        [
            states_test_ha::S0_test_ha as u8,
            INVALID_STATE as u8,
            states_test_ha::S2_test_ha as u8,
        ],
        [
            INVALID_STATE as u8,
            states_test_ha::S2_test_ha as u8,
            states_test_ha::S3_test_ha as u8,
        ],
        [
            states_test_ha::S0_test_ha as u8,
            states_test_ha::S1_test_ha as u8,
            INVALID_STATE as u8,
        ],
    ],
    initial_state: states_test_ha::S0_test_ha as u8,
    final_states: [true, false, false, false],
};
