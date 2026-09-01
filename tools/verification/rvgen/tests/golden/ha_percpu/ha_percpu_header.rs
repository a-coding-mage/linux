/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Automatically generated C representation of ha_percpu automaton
 * For further information about this format, see kernel documentation:
 *   Documentation/trace/rv/deterministic_automata.rst
 */

pub const MONITOR_NAME: &str = "ha_percpu";

pub type states_ha_percpu = ::core::ffi::c_uint;
pub const S0_ha_percpu: states_ha_percpu = 0;
pub const S1_ha_percpu: states_ha_percpu = 1;
pub const S2_ha_percpu: states_ha_percpu = 2;
pub const S3_ha_percpu: states_ha_percpu = 3;
pub const state_max_ha_percpu: states_ha_percpu = 4;

pub const INVALID_STATE: states_ha_percpu = state_max_ha_percpu;

pub type events_ha_percpu = ::core::ffi::c_uint;
pub const event0_ha_percpu: events_ha_percpu = 0;
pub const event1_ha_percpu: events_ha_percpu = 1;
pub const event2_ha_percpu: events_ha_percpu = 2;
pub const event_max_ha_percpu: events_ha_percpu = 3;

pub type envs_ha_percpu = ::core::ffi::c_uint;
pub const clk_ha_percpu: envs_ha_percpu = 0;
pub const env1_ha_percpu: envs_ha_percpu = 1;
pub const env2_ha_percpu: envs_ha_percpu = 2;
pub const env_max_ha_percpu: envs_ha_percpu = 3;
pub const env_max_stored_ha_percpu: envs_ha_percpu = env1_ha_percpu;

// C static assertion:
// _Static_assert(env_max_stored_ha_percpu <= MAX_HA_ENV_LEN, "Not enough slots");
// Depends on external MAX_HA_ENV_LEN.
// C empty marker macro: #define HA_CLK_NS

#[repr(C)]
pub struct automaton_ha_percpu {
    pub state_names: [*const ::core::ffi::c_char; state_max_ha_percpu as usize],
    pub event_names: [*const ::core::ffi::c_char; event_max_ha_percpu as usize],
    pub env_names: [*const ::core::ffi::c_char; env_max_ha_percpu as usize],
    pub function:
        [[::core::ffi::c_uchar; event_max_ha_percpu as usize]; state_max_ha_percpu as usize],
    pub initial_state: ::core::ffi::c_uchar,
    pub final_states: [bool; state_max_ha_percpu as usize],
}

pub const automaton_ha_percpu: automaton_ha_percpu = automaton_ha_percpu {
    state_names: [
        b"S0\0".as_ptr() as *const ::core::ffi::c_char,
        b"S1\0".as_ptr() as *const ::core::ffi::c_char,
        b"S2\0".as_ptr() as *const ::core::ffi::c_char,
        b"S3\0".as_ptr() as *const ::core::ffi::c_char,
    ],
    event_names: [
        b"event0\0".as_ptr() as *const ::core::ffi::c_char,
        b"event1\0".as_ptr() as *const ::core::ffi::c_char,
        b"event2\0".as_ptr() as *const ::core::ffi::c_char,
    ],
    env_names: [
        b"clk\0".as_ptr() as *const ::core::ffi::c_char,
        b"env1\0".as_ptr() as *const ::core::ffi::c_char,
        b"env2\0".as_ptr() as *const ::core::ffi::c_char,
    ],
    function: [
        [S0_ha_percpu as ::core::ffi::c_uchar, S1_ha_percpu as ::core::ffi::c_uchar, INVALID_STATE as ::core::ffi::c_uchar],
        [S0_ha_percpu as ::core::ffi::c_uchar, INVALID_STATE as ::core::ffi::c_uchar, S2_ha_percpu as ::core::ffi::c_uchar],
        [INVALID_STATE as ::core::ffi::c_uchar, S2_ha_percpu as ::core::ffi::c_uchar, S3_ha_percpu as ::core::ffi::c_uchar],
        [S0_ha_percpu as ::core::ffi::c_uchar, S1_ha_percpu as ::core::ffi::c_uchar, INVALID_STATE as ::core::ffi::c_uchar],
    ],
    initial_state: S0_ha_percpu as ::core::ffi::c_uchar,
    final_states: [true, false, false, false],
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
