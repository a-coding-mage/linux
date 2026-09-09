/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Automatically generated C representation of wip automaton
 * For further information about this format, see kernel documentation:
 *   Documentation/trace/rv/deterministic_automata.rst
 */

pub const MONITOR_NAME: &str = "wip";

#[repr(C)]
#[derive(Copy, Clone)]
pub enum StatesWip {
    PreemptiveWip,
    NonPreemptiveWip,
    StateMaxWip,
}

pub const INVALID_STATE: StatesWip = StatesWip::StateMaxWip;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum EventsWip {
    PreemptDisableWip,
    PreemptEnableWip,
    SchedWakingWip,
    EventMaxWip,
}

#[repr(C)]
pub struct AutomatonWip {
    pub state_names: [*const core::ffi::c_char; StatesWip::StateMaxWip as usize],
    pub event_names: [*const core::ffi::c_char; EventsWip::EventMaxWip as usize],
    pub function: [[u8; EventsWip::EventMaxWip as usize]; StatesWip::StateMaxWip as usize],
    pub initial_state: u8,
    pub final_states: [bool; StatesWip::StateMaxWip as usize],
}

pub static AUTOMATON_WIP: AutomatonWip = AutomatonWip {
    state_names: [
        b"preemptive\0".as_ptr() as *const core::ffi::c_char,
        b"non_preemptive\0".as_ptr() as *const core::ffi::c_char,
    ],
    event_names: [
        b"preempt_disable\0".as_ptr() as *const core::ffi::c_char,
        b"preempt_enable\0".as_ptr() as *const core::ffi::c_char,
        b"sched_waking\0".as_ptr() as *const core::ffi::c_char,
    ],
    function: [
        [
            StatesWip::NonPreemptiveWip as u8,
            INVALID_STATE as u8,
            INVALID_STATE as u8,
        ],
        [
            INVALID_STATE as u8,
            StatesWip::PreemptiveWip as u8,
            StatesWip::NonPreemptiveWip as u8,
        ],
    ],
    initial_state: StatesWip::PreemptiveWip as u8,
    final_states: [true, false],
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
