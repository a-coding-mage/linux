/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Automatically generated C representation of snep automaton
 * For further information about this format, see kernel documentation:
 *   Documentation/trace/rv/deterministic_automata.rst
 */

// #define MONITOR_NAME snep
pub const MONITOR_NAME: &str = "snep";

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum StatesSnep {
    NonSchedulingContextSnep,
    SchedulingContexSnep,
    StateMaxSnep,
}

pub const INVALID_STATE: StatesSnep = StatesSnep::StateMaxSnep;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum EventsSnep {
    PreemptDisableSnep,
    PreemptEnableSnep,
    ScheduleEntrySnep,
    ScheduleExitSnep,
    EventMaxSnep,
}

#[repr(C)]
pub struct AutomatonSnep {
    pub state_names: [&'static str; StatesSnep::StateMaxSnep as usize],
    pub event_names: [&'static str; EventsSnep::EventMaxSnep as usize],
    pub function: [[u8; EventsSnep::EventMaxSnep as usize]; StatesSnep::StateMaxSnep as usize],
    pub initial_state: u8,
    pub final_states: [bool; StatesSnep::StateMaxSnep as usize],
}

pub static AUTOMATON_SNEP: AutomatonSnep = AutomatonSnep {
    state_names: [
        "non_scheduling_context",
        "scheduling_contex",
    ],
    event_names: [
        "preempt_disable",
        "preempt_enable",
        "schedule_entry",
        "schedule_exit",
    ],
    function: [
        [
            StatesSnep::NonSchedulingContextSnep as u8,
            StatesSnep::NonSchedulingContextSnep as u8,
            StatesSnep::SchedulingContexSnep as u8,
            INVALID_STATE as u8,
        ],
        [
            INVALID_STATE as u8,
            INVALID_STATE as u8,
            INVALID_STATE as u8,
            StatesSnep::NonSchedulingContextSnep as u8,
        ],
    ],
    initial_state: StatesSnep::NonSchedulingContextSnep as u8,
    final_states: [true, false],
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
