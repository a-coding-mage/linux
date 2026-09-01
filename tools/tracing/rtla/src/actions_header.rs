/* SPDX-License-Identifier: GPL-2.0 */
/* C includes translated as external dependencies:
 * #include <tracefs.h>
 * #include <stdbool.h>
 */

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum action_type {
    ACTION_NONE = 0,
    ACTION_TRACE_OUTPUT,
    ACTION_SIGNAL,
    ACTION_SHELL,
    ACTION_CONTINUE,
    ACTION_FIELD_N,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct action_trace_output {
    /* For ACTION_TRACE_OUTPUT */
    pub trace_output: *mut ::std::os::raw::c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct action_signal {
    /* For ACTION_SIGNAL */
    pub signal: ::std::os::raw::c_int,
    pub pid: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct action_shell {
    /* For ACTION_SHELL */
    pub command: *mut ::std::os::raw::c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union action_data {
    pub trace_output: action_trace_output,
    pub signal: action_signal,
    pub shell: action_shell,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct action {
    pub type_: action_type,
    pub data: action_data,
}

pub const action_default_size: ::std::os::raw::c_int = 8;

#[repr(C)]
pub struct actions {
    pub list: *mut action,
    pub len: ::std::os::raw::c_int,
    pub size: ::std::os::raw::c_int,
    pub present: [bool; action_type::ACTION_FIELD_N as usize],
    pub continue_flag: bool,

    /* External dependencies */
    pub trace_output_inst: *mut tracefs_instance,
}

/* External dependency from <tracefs.h>. */
pub enum tracefs_instance {}

/*
 * C macro translated from:
 * #define for_each_action(actions, action) \
 *     for ((action) = (actions)->list; \
 *          (action) < (actions)->list + (actions)->len; \
 *          (action)++)
 */
#[macro_export]
macro_rules! for_each_action {
    ($actions:expr, $action:ident, $body:block) => {{
        $action = unsafe { (*$actions).list };
        while $action < unsafe { (*$actions).list.add((*$actions).len as usize) } {
            $body
            $action = unsafe { $action.add(1) };
        }
    }};
}

unsafe extern "C" {
    pub fn actions_init(self_: *mut actions);
    pub fn actions_destroy(self_: *mut actions);
    pub fn actions_add_trace_output(
        self_: *mut actions,
        trace_output: *const ::std::os::raw::c_char,
    );
    pub fn actions_add_signal(
        self_: *mut actions,
        signal: ::std::os::raw::c_int,
        pid: ::std::os::raw::c_int,
    );
    pub fn actions_add_shell(self_: *mut actions, command: *const ::std::os::raw::c_char);
    pub fn actions_add_continue(self_: *mut actions);
    pub fn actions_parse(
        self_: *mut actions,
        trigger: *const ::std::os::raw::c_char,
        tracefn: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn actions_perform(self_: *mut actions) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
