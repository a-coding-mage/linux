/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Automatically generated C representation of test_ha_kunit automaton
 * For further information about this format, see kernel documentation:
 *   Documentation/trace/rv/deterministic_automata.rst
 */

use core::ffi::c_char;

pub const MONITOR_NAME: &str = "test_ha_kunit";

pub const S0_test_ha_kunit: u8 = 0;
pub const S1_test_ha_kunit: u8 = 1;
pub const S2_test_ha_kunit: u8 = 2;
pub const S3_test_ha_kunit: u8 = 3;
pub const state_max_test_ha_kunit: usize = 4;

pub const INVALID_STATE: u8 = state_max_test_ha_kunit as u8;

pub const event0_test_ha_kunit: usize = 0;
pub const event1_test_ha_kunit: usize = 1;
pub const event2_test_ha_kunit: usize = 2;
pub const event_max_test_ha_kunit: usize = 3;

pub const clk_test_ha_kunit: usize = 0;
pub const env1_test_ha_kunit: usize = 1;
pub const env2_test_ha_kunit: usize = 2;
pub const env_max_test_ha_kunit: usize = 3;
pub const env_max_stored_test_ha_kunit: usize = env1_test_ha_kunit;

// C source assertion:
// _Static_assert(env_max_stored_test_ha_kunit <= MAX_HA_ENV_LEN, "Not enough slots");
// MAX_HA_ENV_LEN is provided by an external dependency.

// C source defines an empty HA_CLK_NS macro.

#[repr(C)]
pub struct automaton_test_ha_kunit {
    pub state_names: [*const c_char; state_max_test_ha_kunit],
    pub event_names: [*const c_char; event_max_test_ha_kunit],
    pub env_names: [*const c_char; env_max_test_ha_kunit],
    pub function: [[u8; event_max_test_ha_kunit]; state_max_test_ha_kunit],
    pub initial_state: u8,
    pub final_states: [bool; state_max_test_ha_kunit],
}

pub const automaton_test_ha_kunit: automaton_test_ha_kunit = automaton_test_ha_kunit {
    state_names: [
        b"S0\0".as_ptr() as *const c_char,
        b"S1\0".as_ptr() as *const c_char,
        b"S2\0".as_ptr() as *const c_char,
        b"S3\0".as_ptr() as *const c_char,
    ],
    event_names: [
        b"event0\0".as_ptr() as *const c_char,
        b"event1\0".as_ptr() as *const c_char,
        b"event2\0".as_ptr() as *const c_char,
    ],
    env_names: [
        b"clk\0".as_ptr() as *const c_char,
        b"env1\0".as_ptr() as *const c_char,
        b"env2\0".as_ptr() as *const c_char,
    ],
    function: [
        [
            S0_test_ha_kunit,
            S1_test_ha_kunit,
            INVALID_STATE,
        ],
        [
            S0_test_ha_kunit,
            INVALID_STATE,
            S2_test_ha_kunit,
        ],
        [
            INVALID_STATE,
            S2_test_ha_kunit,
            S3_test_ha_kunit,
        ],
        [
            S0_test_ha_kunit,
            S1_test_ha_kunit,
            INVALID_STATE,
        ],
    ],
    initial_state: S0_test_ha_kunit,
    final_states: [true, false, false, false],
};
