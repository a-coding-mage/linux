/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Automatically generated C representation of scpd automaton
 * For further information about this format, see kernel documentation:
 *   Documentation/trace/rv/deterministic_automata.rst
 */

pub const MONITOR_NAME: &str = "scpd";

pub const cant_sched_scpd: usize = 0;
pub const can_sched_scpd: usize = 1;
pub const state_max_scpd: usize = 2;

pub const INVALID_STATE: usize = state_max_scpd;

pub const preempt_disable_scpd: usize = 0;
pub const preempt_enable_scpd: usize = 1;
pub const schedule_entry_scpd: usize = 2;
pub const schedule_exit_scpd: usize = 3;
pub const event_max_scpd: usize = 4;

#[repr(C)]
pub struct automaton_scpd {
    pub state_names: [*const core::ffi::c_char; state_max_scpd],
    pub event_names: [*const core::ffi::c_char; event_max_scpd],
    pub function: [[u8; event_max_scpd]; state_max_scpd],
    pub initial_state: u8,
    pub final_states: [bool; state_max_scpd],
}

pub static automaton_scpd: automaton_scpd = automaton_scpd {
    state_names: [
        b"cant_sched\0".as_ptr() as *const core::ffi::c_char,
        b"can_sched\0".as_ptr() as *const core::ffi::c_char,
    ],
    event_names: [
        b"preempt_disable\0".as_ptr() as *const core::ffi::c_char,
        b"preempt_enable\0".as_ptr() as *const core::ffi::c_char,
        b"schedule_entry\0".as_ptr() as *const core::ffi::c_char,
        b"schedule_exit\0".as_ptr() as *const core::ffi::c_char,
    ],
    function: [
        [can_sched_scpd as u8, INVALID_STATE as u8, INVALID_STATE as u8, INVALID_STATE as u8],
        [INVALID_STATE as u8, cant_sched_scpd as u8, can_sched_scpd as u8, can_sched_scpd as u8],
    ],
    initial_state: cant_sched_scpd as u8,
    final_states: [true, false],
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
