/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Automatically generated C representation of nrp automaton
 * For further information about this format, see kernel documentation:
 *   Documentation/trace/rv/deterministic_automata.rst
 */

use std::ffi::c_char;

pub const MONITOR_NAME: &str = "nrp";

#[repr(C)]
#[derive(Copy, Clone)]
pub enum states_nrp {
    preempt_irq_nrp,
    any_thread_running_nrp,
    nested_preempt_nrp,
    rescheduling_nrp,
    state_max_nrp,
}

pub const INVALID_STATE: states_nrp = states_nrp::state_max_nrp;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum events_nrp {
    irq_entry_nrp,
    sched_need_resched_nrp,
    schedule_entry_nrp,
    schedule_entry_preempt_nrp,
    event_max_nrp,
}

#[repr(C)]
pub struct automaton_nrp {
    pub state_names: [*const c_char; 4],
    pub event_names: [*const c_char; 4],
    pub function: [[u8; 4]; 4],
    pub initial_state: u8,
    pub final_states: [bool; 4],
}

static automaton_nrp: automaton_nrp = automaton_nrp {
    state_names: [
        b"preempt_irq\0".as_ptr() as *const c_char,
        b"any_thread_running\0".as_ptr() as *const c_char,
        b"nested_preempt\0".as_ptr() as *const c_char,
        b"rescheduling\0".as_ptr() as *const c_char,
    ],
    event_names: [
        b"irq_entry\0".as_ptr() as *const c_char,
        b"sched_need_resched\0".as_ptr() as *const c_char,
        b"schedule_entry\0".as_ptr() as *const c_char,
        b"schedule_entry_preempt\0".as_ptr() as *const c_char,
    ],
    function: [
        [
            states_nrp::preempt_irq_nrp as u8,
            states_nrp::preempt_irq_nrp as u8,
            states_nrp::nested_preempt_nrp as u8,
            states_nrp::nested_preempt_nrp as u8,
        ],
        [
            states_nrp::any_thread_running_nrp as u8,
            states_nrp::rescheduling_nrp as u8,
            states_nrp::any_thread_running_nrp as u8,
            states_nrp::state_max_nrp as u8,
        ],
        [
            states_nrp::nested_preempt_nrp as u8,
            states_nrp::preempt_irq_nrp as u8,
            states_nrp::any_thread_running_nrp as u8,
            states_nrp::any_thread_running_nrp as u8,
        ],
        [
            states_nrp::preempt_irq_nrp as u8,
            states_nrp::rescheduling_nrp as u8,
            states_nrp::any_thread_running_nrp as u8,
            states_nrp::any_thread_running_nrp as u8,
        ],
    ],
    initial_state: states_nrp::preempt_irq_nrp as u8,
    final_states: [false, true, false, false],
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
