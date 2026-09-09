/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Automatically generated C representation of sts automaton
 * For further information about this format, see kernel documentation:
 *   Documentation/trace/rv/deterministic_automata.rst
 */

pub const MONITOR_NAME: &str = "sts";

pub const can_sched_sts: u8 = 0;
pub const cant_sched_sts: u8 = 1;
pub const disable_to_switch_sts: u8 = 2;
pub const enable_to_exit_sts: u8 = 3;
pub const in_irq_sts: u8 = 4;
pub const scheduling_sts: u8 = 5;
pub const switching_sts: u8 = 6;
pub const state_max_sts: usize = 7;

pub const INVALID_STATE: u8 = state_max_sts as u8;

pub const irq_disable_sts: u8 = 0;
pub const irq_enable_sts: u8 = 1;
pub const irq_entry_sts: u8 = 2;
pub const sched_switch_sts: u8 = 3;
pub const schedule_entry_sts: u8 = 4;
pub const schedule_exit_sts: u8 = 5;
pub const event_max_sts: usize = 6;

#[repr(C)]
pub struct automaton_sts {
    pub state_names: [*const core::ffi::c_char; state_max_sts],
    pub event_names: [*const core::ffi::c_char; event_max_sts],
    pub function: [[u8; event_max_sts]; state_max_sts],
    pub initial_state: u8,
    pub final_states: [bool; state_max_sts],
}

pub static automaton_sts: automaton_sts = automaton_sts {
    state_names: [
        b"can_sched\0".as_ptr() as *const core::ffi::c_char,
        b"cant_sched\0".as_ptr() as *const core::ffi::c_char,
        b"disable_to_switch\0".as_ptr() as *const core::ffi::c_char,
        b"enable_to_exit\0".as_ptr() as *const core::ffi::c_char,
        b"in_irq\0".as_ptr() as *const core::ffi::c_char,
        b"scheduling\0".as_ptr() as *const core::ffi::c_char,
        b"switching\0".as_ptr() as *const core::ffi::c_char,
    ],
    event_names: [
        b"irq_disable\0".as_ptr() as *const core::ffi::c_char,
        b"irq_enable\0".as_ptr() as *const core::ffi::c_char,
        b"irq_entry\0".as_ptr() as *const core::ffi::c_char,
        b"sched_switch\0".as_ptr() as *const core::ffi::c_char,
        b"schedule_entry\0".as_ptr() as *const core::ffi::c_char,
        b"schedule_exit\0".as_ptr() as *const core::ffi::c_char,
    ],
    function: [
        [cant_sched_sts, INVALID_STATE, INVALID_STATE, INVALID_STATE, scheduling_sts, INVALID_STATE],
        [INVALID_STATE, can_sched_sts, cant_sched_sts, INVALID_STATE, INVALID_STATE, INVALID_STATE],
        [INVALID_STATE, enable_to_exit_sts, in_irq_sts, switching_sts, INVALID_STATE, INVALID_STATE],
        [enable_to_exit_sts, enable_to_exit_sts, enable_to_exit_sts, INVALID_STATE, INVALID_STATE, can_sched_sts],
        [INVALID_STATE, scheduling_sts, in_irq_sts, INVALID_STATE, INVALID_STATE, INVALID_STATE],
        [disable_to_switch_sts, INVALID_STATE, INVALID_STATE, INVALID_STATE, INVALID_STATE, INVALID_STATE],
        [INVALID_STATE, enable_to_exit_sts, INVALID_STATE, INVALID_STATE, INVALID_STATE, INVALID_STATE],
    ],
    initial_state: can_sched_sts,
    final_states: [true, false, false, false, false, false, false],
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
