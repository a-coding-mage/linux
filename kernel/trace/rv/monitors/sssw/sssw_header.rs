/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Automatically generated C representation of sssw automaton
 * For further information about this format, see kernel documentation:
 *   Documentation/trace/rv/deterministic_automata.rst
 */

// #define MONITOR_NAME sssw

#[repr(C)]
#[derive(Copy, Clone)]
pub enum StatesSssw {
    runnable_sssw,
    signal_wakeup_sssw,
    sleepable_sssw,
    sleeping_sssw,
    state_max_sssw,
}

pub const INVALID_STATE: StatesSssw = StatesSssw::state_max_sssw;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum EventsSssw {
    sched_set_state_runnable_sssw,
    sched_set_state_sleepable_sssw,
    sched_switch_blocking_sssw,
    sched_switch_in_sssw,
    sched_switch_preempt_sssw,
    sched_switch_suspend_sssw,
    sched_switch_yield_sssw,
    sched_wakeup_sssw,
    signal_deliver_sssw,
    event_max_sssw,
}

#[repr(C)]
pub struct AutomatonSssw {
    pub state_names: [*mut std::ffi::c_char; StatesSssw::state_max_sssw as usize],
    pub event_names: [*mut std::ffi::c_char; EventsSssw::event_max_sssw as usize],
    pub function: [[u8; EventsSssw::event_max_sssw as usize]; StatesSssw::state_max_sssw as usize],
    pub initial_state: u8,
    pub final_states: [bool; StatesSssw::state_max_sssw as usize],
}

pub static AUTOMATON_SSSW: AutomatonSssw = AutomatonSssw {
    state_names: [
        "runnable\0".as_ptr() as *mut std::ffi::c_char,
        "signal_wakeup\0".as_ptr() as *mut std::ffi::c_char,
        "sleepable\0".as_ptr() as *mut std::ffi::c_char,
        "sleeping\0".as_ptr() as *mut std::ffi::c_char,
    ],
    event_names: [
        "sched_set_state_runnable\0".as_ptr() as *mut std::ffi::c_char,
        "sched_set_state_sleepable\0".as_ptr() as *mut std::ffi::c_char,
        "sched_switch_blocking\0".as_ptr() as *mut std::ffi::c_char,
        "sched_switch_in\0".as_ptr() as *mut std::ffi::c_char,
        "sched_switch_preempt\0".as_ptr() as *mut std::ffi::c_char,
        "sched_switch_suspend\0".as_ptr() as *mut std::ffi::c_char,
        "sched_switch_yield\0".as_ptr() as *mut std::ffi::c_char,
        "sched_wakeup\0".as_ptr() as *mut std::ffi::c_char,
        "signal_deliver\0".as_ptr() as *mut std::ffi::c_char,
    ],
    function: [
        [0, 2, 3, 0, 0, 4, 0, 0, 0],
        [4, 2, 4, 1, 1, 4, 1, 1, 0],
        [0, 2, 3, 2, 2, 3, 1, 0, 2],
        [4, 4, 4, 4, 4, 4, 4, 0, 4],
    ],
    initial_state: 0,
    final_states: [true, false, false, false],
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
