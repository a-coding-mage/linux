/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Automatically generated C representation of wwnr automaton
 * For further information about this format, see kernel documentation:
 *   Documentation/trace/rv/deterministic_automata.rst
 */

use core::ffi::c_char;

pub const MONITOR_NAME: &str = "wwnr";

#[repr(C)]
#[derive(Copy, Clone)]
pub enum StatesWwnr {
    NotRunningWwnr,
    RunningWwnr,
    StateMaxWwnr,
}

pub const INVALID_STATE: StatesWwnr = StatesWwnr::StateMaxWwnr;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum EventsWwnr {
    SwitchInWwnr,
    SwitchOutWwnr,
    WakeupWwnr,
    EventMaxWwnr,
}

#[repr(C)]
pub struct AutomatonWwnr {
    pub state_names: [*mut c_char; StatesWwnr::StateMaxWwnr as usize],
    pub event_names: [*mut c_char; EventsWwnr::EventMaxWwnr as usize],
    pub function: [[u8; EventsWwnr::EventMaxWwnr as usize]; StatesWwnr::StateMaxWwnr as usize],
    pub initial_state: u8,
    pub final_states: [bool; StatesWwnr::StateMaxWwnr as usize],
}

pub static automaton_wwnr: AutomatonWwnr = AutomatonWwnr {
    state_names: [
        b"not_running\0".as_ptr() as *mut c_char,
        b"running\0".as_ptr() as *mut c_char,
    ],
    event_names: [
        b"switch_in\0".as_ptr() as *mut c_char,
        b"switch_out\0".as_ptr() as *mut c_char,
        b"wakeup\0".as_ptr() as *mut c_char,
    ],
    function: [
        [
            StatesWwnr::RunningWwnr as u8,
            INVALID_STATE as u8,
            StatesWwnr::NotRunningWwnr as u8,
        ],
        [
            INVALID_STATE as u8,
            StatesWwnr::NotRunningWwnr as u8,
            INVALID_STATE as u8,
        ],
    ],
    initial_state: StatesWwnr::NotRunningWwnr as u8,
    final_states: [true, false],
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
