// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

// C dependencies from:
// #include <stdlib.h>
// #include <string.h>
// #include <signal.h>
// #include <unistd.h>
// #include "actions.h"
// #include "trace.h"
// #include "utils.h"

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum action_type {
    ACTION_NONE = 0,
    ACTION_TRACE_OUTPUT = 1,
    ACTION_SIGNAL = 2,
    ACTION_SHELL = 3,
    ACTION_CONTINUE = 4,
}

pub const ACTION_MAX: usize = 5;

#[repr(C)]
#[derive(Copy, Clone)]
pub union action_data {
    pub command: *mut c_char,
    pub trace_output: *mut c_char,
    pub signal_pid: action_signal,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct action_signal {
    pub signal: c_int,
    pub pid: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct action {
    pub r#type: action_type,
    pub data: action_data,
}

#[repr(C)]
pub struct trace_instance {
    _private: [u8; 0],
}

#[repr(C)]
pub struct actions {
    pub size: usize,
    pub list: *mut action,
    pub len: usize,
    pub continue_flag: bool,
    pub trace_output_inst: *mut trace_instance,
    pub present: [bool; ACTION_MAX],
}

unsafe extern "C" {
    static action_default_size: usize;

    fn calloc_fatal(nmemb: usize, size: usize) -> *mut c_void;
    fn reallocarray_fatal(ptr: *mut c_void, nmemb: usize, size: usize) -> *mut c_void;
    fn strdup_fatal(s: *const c_char) -> *mut c_char;
    fn strtoi(s: *const c_char, value: *mut c_int) -> c_int;
    fn strncmp_static(s1: *const c_char, s2: *const c_char) -> c_int;
    fn save_trace_to_file(trace: *mut trace_instance, file: *const c_char) -> c_int;
    fn err_msg(fmt: *const c_char, ...);

    fn free(ptr: *mut c_void);
    fn strlen(s: *const c_char) -> usize;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strtok(str: *mut c_char, delim: *const c_char) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn system(command: *const c_char) -> c_int;
    fn kill(pid: c_int, sig: c_int) -> c_int;
    fn getppid() -> c_int;
}

const COMMA: &[u8] = b",\0";
const TRACE: &[u8] = b"trace\0";
const SIGNAL: &[u8] = b"signal\0";
const SHELL: &[u8] = b"shell\0";
const CONTINUE: &[u8] = b"continue\0";
const FILE_EQ: &[u8] = b"file=\0";
const NUM_EQ: &[u8] = b"num=\0";
const PID_EQ: &[u8] = b"pid=\0";
const COMMAND_EQ: &[u8] = b"command=\0";
const PARENT: &[u8] = b"parent\0";
const ERROR_SAVING_TRACE: &[u8] = b"Error saving trace\n\0";
const ERROR_SENDING_SIGNAL: &[u8] = b"Error sending signal\n\0";

unsafe fn action_at(self_: *mut actions, index: usize) -> *mut action {
    unsafe { (*self_).list.add(index) }
}

/*
 * actions_init - initialize struct actions
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn actions_init(self_: *mut actions) {
    unsafe {
        (*self_).size = action_default_size;
        (*self_).list = calloc_fatal((*self_).size, size_of::<action>()) as *mut action;
        (*self_).len = 0;
        (*self_).continue_flag = false;

        /* This has to be set by the user */
        (*self_).trace_output_inst = ptr::null_mut();
    }
}

/*
 * actions_destroy - destroy struct actions
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn actions_destroy(self_: *mut actions) {
    unsafe {
        /* Free any action-specific data */
        let mut i = 0usize;

        while i < (*self_).len {
            let action = action_at(self_, i);

            if (*action).r#type == action_type::ACTION_SHELL {
                free((*action).data.command as *mut c_void);
            }
            if (*action).r#type == action_type::ACTION_TRACE_OUTPUT {
                free((*action).data.trace_output as *mut c_void);
            }

            i += 1;
        }

        /* Free action list */
        free((*self_).list as *mut c_void);
    }
}

/*
 * actions_new - Get pointer to new action
 */
unsafe fn actions_new(self_: *mut actions) -> *mut action {
    unsafe {
        if (*self_).len >= (*self_).size {
            let new_size: usize = (*self_).size * 2;

            (*self_).list =
                reallocarray_fatal((*self_).list as *mut c_void, new_size, size_of::<action>())
                    as *mut action;
            (*self_).size = new_size;
        }

        let action = (*self_).list.add((*self_).len);
        (*self_).len += 1;
        action
    }
}

/*
 * actions_add_trace_output - add an action to output trace
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn actions_add_trace_output(
    self_: *mut actions,
    trace_output: *const c_char,
) {
    unsafe {
        let action = actions_new(self_);

        (*self_).present[action_type::ACTION_TRACE_OUTPUT as usize] = true;
        (*action).r#type = action_type::ACTION_TRACE_OUTPUT;
        (*action).data.trace_output = strdup_fatal(trace_output);
    }
}

/*
 * actions_add_trace_output - add an action to send signal to a process
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn actions_add_signal(self_: *mut actions, signal: c_int, pid: c_int) {
    unsafe {
        let action = actions_new(self_);

        (*self_).present[action_type::ACTION_SIGNAL as usize] = true;
        (*action).r#type = action_type::ACTION_SIGNAL;
        (*action).data.signal_pid.signal = signal;
        (*action).data.signal_pid.pid = pid;
    }
}

/*
 * actions_add_shell - add an action to execute a shell command
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn actions_add_shell(self_: *mut actions, command: *const c_char) {
    unsafe {
        let action = actions_new(self_);

        (*self_).present[action_type::ACTION_SHELL as usize] = true;
        (*action).r#type = action_type::ACTION_SHELL;
        (*action).data.command = strdup_fatal(command);
    }
}

/*
 * actions_add_continue - add an action to resume measurement
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn actions_add_continue(self_: *mut actions) {
    unsafe {
        let action = actions_new(self_);

        (*self_).present[action_type::ACTION_CONTINUE as usize] = true;
        (*action).r#type = action_type::ACTION_CONTINUE;
    }
}

unsafe fn __extract_arg(token: *const c_char, opt: *const c_char, opt_len: usize) -> *const c_char {
    unsafe {
        let tok_len: usize = strlen(token);

        if tok_len <= opt_len {
            return ptr::null();
        }

        if strncmp(token, opt, opt_len) != 0 {
            return ptr::null();
        }

        token.add(opt_len)
    }
}

/*
 * extract_arg - extract argument value from option token
 * @token: option token (e.g., "file=trace.txt")
 * @opt: option name to match (e.g., "file")
 *
 * Returns pointer to argument value after "=" if token matches "opt=",
 * otherwise returns NULL.
 */
unsafe fn extract_arg(token: *const c_char, opt: &'static [u8]) -> *const c_char {
    unsafe { __extract_arg(token, opt.as_ptr() as *const c_char, opt.len() - 1) }
}

/*
 * actions_parse - add an action based on text specification
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn actions_parse(
    self_: *mut actions,
    trigger: *const c_char,
    tracefn: *const c_char,
) -> c_int {
    unsafe {
        let mut type_: action_type = action_type::ACTION_NONE;
        let mut token: *mut c_char;
        let trigger_len = strlen(trigger);
        let mut trigger_c = vec![0 as c_char; trigger_len + 1];
        let mut arg_value: *const c_char;

        /* For ACTION_SIGNAL */
        let mut signal: c_int = 0;
        let mut pid: c_int = 0;

        /* For ACTION_TRACE_OUTPUT */
        let trace_output: *const c_char;

        strcpy(trigger_c.as_mut_ptr(), trigger);
        token = strtok(trigger_c.as_mut_ptr(), COMMA.as_ptr() as *const c_char);
        if token.is_null() {
            return -1;
        }

        if strcmp(token, TRACE.as_ptr() as *const c_char) == 0 {
            type_ = action_type::ACTION_TRACE_OUTPUT;
        } else if strcmp(token, SIGNAL.as_ptr() as *const c_char) == 0 {
            type_ = action_type::ACTION_SIGNAL;
        } else if strcmp(token, SHELL.as_ptr() as *const c_char) == 0 {
            type_ = action_type::ACTION_SHELL;
        } else if strcmp(token, CONTINUE.as_ptr() as *const c_char) == 0 {
            type_ = action_type::ACTION_CONTINUE;
        } else {
            /* Invalid trigger type */
            return -1;
        }

        token = strtok(ptr::null_mut(), COMMA.as_ptr() as *const c_char);

        match type_ {
            action_type::ACTION_TRACE_OUTPUT => {
                /* Takes no argument */
                if token.is_null() {
                    trace_output = tracefn;
                } else {
                    trace_output = extract_arg(token, FILE_EQ);
                    if trace_output.is_null() {
                        /* Invalid argument */
                        return -1;
                    }

                    token = strtok(ptr::null_mut(), COMMA.as_ptr() as *const c_char);
                    if !token.is_null() {
                        /* Only one argument allowed */
                        return -1;
                    }
                }
                actions_add_trace_output(self_, trace_output);
            }
            action_type::ACTION_SIGNAL => {
                /* Takes two arguments, num (signal) and pid */
                while !token.is_null() {
                    arg_value = extract_arg(token, NUM_EQ);
                    if !arg_value.is_null() {
                        if strtoi(arg_value, &mut signal) != 0 {
                            return -1;
                        }
                    } else {
                        arg_value = extract_arg(token, PID_EQ);
                        if !arg_value.is_null() {
                            if strncmp_static(arg_value, PARENT.as_ptr() as *const c_char) == 0 {
                                pid = -1;
                            } else if strtoi(arg_value, &mut pid) != 0 {
                                return -1;
                            }
                        } else {
                            /* Invalid argument */
                            return -1;
                        }
                    }

                    token = strtok(ptr::null_mut(), COMMA.as_ptr() as *const c_char);
                }

                if signal == 0 || pid == 0 {
                    /* Missing argument */
                    return -1;
                }

                actions_add_signal(self_, signal, pid);
            }
            action_type::ACTION_SHELL => {
                if token.is_null() {
                    return -1;
                }
                arg_value = extract_arg(token, COMMAND_EQ);
                if arg_value.is_null() {
                    return -1;
                }
                actions_add_shell(self_, arg_value);
            }
            action_type::ACTION_CONTINUE => {
                /* Takes no argument */
                if !token.is_null() {
                    return -1;
                }
                actions_add_continue(self_);
            }
            _ => {
                return -1;
            }
        }

        0
    }
}

/*
 * actions_perform - perform all actions
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn actions_perform(self_: *mut actions) -> c_int {
    unsafe {
        let mut pid: c_int;
        let mut retval: c_int;

        (*self_).continue_flag = false;

        let mut i = 0usize;
        while i < (*self_).len {
            let action = action_at(self_, i) as *const action;

            match (*action).r#type {
                action_type::ACTION_TRACE_OUTPUT => {
                    retval = save_trace_to_file(
                        (*self_).trace_output_inst,
                        (*action).data.trace_output,
                    );
                    if retval != 0 {
                        err_msg(ERROR_SAVING_TRACE.as_ptr() as *const c_char);
                        return retval;
                    }
                }
                action_type::ACTION_SIGNAL => {
                    if (*action).data.signal_pid.pid == -1 {
                        pid = getppid();
                    } else {
                        pid = (*action).data.signal_pid.pid;
                    }
                    retval = kill(pid, (*action).data.signal_pid.signal);
                    if retval != 0 {
                        err_msg(ERROR_SENDING_SIGNAL.as_ptr() as *const c_char);
                        return retval;
                    }
                }
                action_type::ACTION_SHELL => {
                    retval = system((*action).data.command);
                    if retval != 0 {
                        return retval;
                    }
                }
                action_type::ACTION_CONTINUE => {
                    (*self_).continue_flag = true;
                    return 0;
                }
                _ => {}
            }

            i += 1;
        }

        0
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
