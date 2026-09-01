// SPDX-License-Identifier: GPL-2.0

// C source dependencies:
// #define _GNU_SOURCE
// #include <check.h>
// #include <signal.h>
// #include "../../src/actions.h"

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

const SIGINT: c_int = 2;

#[repr(C)]
pub struct action {
    pub r#type: c_int,
    pub trace_output: *mut c_char,
    pub signal: c_int,
    pub pid: c_int,
    pub command: *mut c_char,
}

#[repr(C)]
pub struct actions {
    pub list: *mut action,
    pub len: c_int,
    pub size: c_int,
    pub present: *mut bool,
    pub continue_flag: bool,
    pub trace_output_inst: *mut c_void,
}

#[repr(C)]
pub struct Suite {
    _private: [u8; 0],
}

#[repr(C)]
pub struct TCase {
    _private: [u8; 0],
}

unsafe extern "C" {
    static action_default_size: c_int;
    static ACTION_TRACE_OUTPUT: c_int;
    static ACTION_SIGNAL: c_int;
    static ACTION_SHELL: c_int;
    static ACTION_CONTINUE: c_int;

    fn actions_init(actions: *mut actions);
    fn actions_destroy(actions: *mut actions);
    fn actions_add_trace_output(actions: *mut actions, trace_output: *const c_char);
    fn actions_add_signal(actions: *mut actions, signal: c_int, pid: c_int);
    fn actions_add_shell(actions: *mut actions, command: *const c_char);
    fn actions_add_continue(actions: *mut actions);
    fn actions_parse(actions: *mut actions, action: *const c_char, trace_output: *const c_char) -> c_int;
    fn actions_perform(actions: *mut actions) -> c_int;

    fn suite_create(name: *const c_char) -> *mut Suite;
    fn tcase_create(name: *const c_char) -> *mut TCase;
    fn tcase_add_test(tc: *mut TCase, test: Option<unsafe extern "C" fn(c_int)>);
    fn tcase_add_checked_fixture(
        tc: *mut TCase,
        setup: Option<unsafe extern "C" fn()>,
        teardown: Option<unsafe extern "C" fn()>,
    );
    fn suite_add_tcase(s: *mut Suite, tc: *mut TCase);

    fn ck_assert_int_eq(a: c_int, b: c_int);
    fn ck_assert_ptr_eq(a: *const c_void, b: *const c_void);
    fn ck_assert_str_eq(a: *const c_char, b: *const c_char);
    fn ck_assert(expr: bool);
}

static mut actions_fixture: actions = actions {
    list: ptr::null_mut(),
    len: 0,
    size: 0,
    present: ptr::null_mut(),
    continue_flag: false,
    trace_output_inst: ptr::null_mut(),
};

unsafe fn present(actions: *mut actions, ty: c_int) -> bool {
    *(*actions).present.add(ty as usize)
}

unsafe extern "C" fn actions_fixture_setup() {
    actions_init(&raw mut actions_fixture);
}

unsafe extern "C" fn actions_fixture_teardown() {
    actions_destroy(&raw mut actions_fixture);
}

unsafe extern "C" fn test_actions_init(_i: c_int) {
    let mut actions: actions = core::mem::zeroed();

    actions_init(&mut actions);

    ck_assert_int_eq(actions.len, 0);
    ck_assert_int_eq(actions.size, action_default_size);
    ck_assert(!actions.continue_flag);
    ck_assert_ptr_eq(actions.trace_output_inst as *const c_void, ptr::null());
}

unsafe extern "C" fn test_actions_destroy(_i: c_int) {
    let mut actions: actions = core::mem::zeroed();

    actions_init(&mut actions);
    actions_destroy(&mut actions);
}

unsafe extern "C" fn test_actions_reallocate(_i: c_int) {
    let mut actions: actions = core::mem::zeroed();
    let mut i: c_int;

    actions_init(&mut actions);

    ck_assert_int_eq(actions.len, 0);
    ck_assert_int_eq(actions.size, action_default_size);

    /* Fill size of actions array */
    i = 0;
    while i < action_default_size {
        actions_add_continue(&mut actions);
        i += 1;
    }

    ck_assert_int_eq(actions.len, action_default_size);
    ck_assert_int_eq(actions.size, action_default_size);

    /* Add one more action to trigger reallocation */
    actions_add_continue(&mut actions);

    ck_assert_int_eq(actions.len, action_default_size + 1);
    ck_assert_int_eq(actions.size, action_default_size * 2);

    actions_destroy(&mut actions);
}

unsafe extern "C" fn test_actions_add_trace_output(_i: c_int) {
    actions_add_trace_output(&raw mut actions_fixture, c"trace_output.txt".as_ptr());

    ck_assert_int_eq(actions_fixture.len, 1);
    ck_assert_int_eq((*actions_fixture.list.add(0)).r#type, ACTION_TRACE_OUTPUT);
    ck_assert_str_eq((*actions_fixture.list.add(0)).trace_output, c"trace_output.txt".as_ptr());
    ck_assert(present(&raw mut actions_fixture, ACTION_TRACE_OUTPUT));
}

unsafe extern "C" fn test_actions_add_signal(_i: c_int) {
    actions_add_signal(&raw mut actions_fixture, SIGINT, 1234);

    ck_assert_int_eq(actions_fixture.len, 1);
    ck_assert_int_eq((*actions_fixture.list.add(0)).r#type, ACTION_SIGNAL);
    ck_assert_int_eq((*actions_fixture.list.add(0)).signal, SIGINT);
    ck_assert_int_eq((*actions_fixture.list.add(0)).pid, 1234);
    ck_assert(present(&raw mut actions_fixture, ACTION_SIGNAL));
}

unsafe extern "C" fn test_actions_add_shell(_i: c_int) {
    actions_add_shell(&raw mut actions_fixture, c"echo Hello".as_ptr());

    ck_assert_int_eq(actions_fixture.len, 1);
    ck_assert_int_eq((*actions_fixture.list.add(0)).r#type, ACTION_SHELL);
    ck_assert_str_eq((*actions_fixture.list.add(0)).command, c"echo Hello".as_ptr());
    ck_assert(present(&raw mut actions_fixture, ACTION_SHELL));
}

unsafe extern "C" fn test_actions_add_continue(_i: c_int) {
    actions_add_continue(&raw mut actions_fixture);

    ck_assert_int_eq(actions_fixture.len, 1);
    ck_assert_int_eq((*actions_fixture.list.add(0)).r#type, ACTION_CONTINUE);
    ck_assert(present(&raw mut actions_fixture, ACTION_CONTINUE));
}

unsafe extern "C" fn test_actions_add_multiple_same_action(_i: c_int) {
    actions_add_trace_output(&raw mut actions_fixture, c"trace1.txt".as_ptr());
    actions_add_trace_output(&raw mut actions_fixture, c"trace2.txt".as_ptr());

    ck_assert_int_eq(actions_fixture.len, 2);
    ck_assert_int_eq((*actions_fixture.list.add(0)).r#type, ACTION_TRACE_OUTPUT);
    ck_assert_str_eq((*actions_fixture.list.add(0)).trace_output, c"trace1.txt".as_ptr());
    ck_assert_int_eq((*actions_fixture.list.add(1)).r#type, ACTION_TRACE_OUTPUT);
    ck_assert_str_eq((*actions_fixture.list.add(1)).trace_output, c"trace2.txt".as_ptr());
    ck_assert(present(&raw mut actions_fixture, ACTION_TRACE_OUTPUT));
}

unsafe extern "C" fn test_actions_add_multiple_different_action(_i: c_int) {
    actions_add_trace_output(&raw mut actions_fixture, c"trace_output.txt".as_ptr());
    actions_add_signal(&raw mut actions_fixture, SIGINT, 1234);

    ck_assert_int_eq(actions_fixture.len, 2);
    ck_assert_int_eq((*actions_fixture.list.add(0)).r#type, ACTION_TRACE_OUTPUT);
    ck_assert_str_eq((*actions_fixture.list.add(0)).trace_output, c"trace_output.txt".as_ptr());
    ck_assert(present(&raw mut actions_fixture, ACTION_TRACE_OUTPUT));
    ck_assert_int_eq((*actions_fixture.list.add(1)).r#type, ACTION_SIGNAL);
    ck_assert_int_eq((*actions_fixture.list.add(1)).signal, SIGINT);
    ck_assert_int_eq((*actions_fixture.list.add(1)).pid, 1234);
    ck_assert(present(&raw mut actions_fixture, ACTION_SIGNAL));
}

unsafe extern "C" fn test_actions_parse_trace_output(_i: c_int) {
    ck_assert_int_eq(actions_parse(&raw mut actions_fixture, c"trace".as_ptr(), c"trace.txt".as_ptr()), 0);

    ck_assert_int_eq(actions_fixture.len, 1);
    ck_assert_int_eq((*actions_fixture.list.add(0)).r#type, ACTION_TRACE_OUTPUT);
    ck_assert_str_eq((*actions_fixture.list.add(0)).trace_output, c"trace.txt".as_ptr());
    ck_assert(present(&raw mut actions_fixture, ACTION_TRACE_OUTPUT));
}

unsafe extern "C" fn test_actions_parse_trace_output_arg(_i: c_int) {
    ck_assert_int_eq(actions_parse(&raw mut actions_fixture, c"trace,file=trace2.txt".as_ptr(), c"trace1.txt".as_ptr()), 0);

    ck_assert_int_eq(actions_fixture.len, 1);
    ck_assert_int_eq((*actions_fixture.list.add(0)).r#type, ACTION_TRACE_OUTPUT);
    ck_assert_str_eq((*actions_fixture.list.add(0)).trace_output, c"trace2.txt".as_ptr());
    ck_assert(present(&raw mut actions_fixture, ACTION_TRACE_OUTPUT));
}

unsafe extern "C" fn test_actions_parse_trace_output_arg_bad(_i: c_int) {
    ck_assert_int_eq(actions_parse(&raw mut actions_fixture, c"trace,foo=bar".as_ptr(), c"trace_output.txt".as_ptr()), -1);

    ck_assert_int_eq(actions_fixture.len, 0);
    ck_assert(!present(&raw mut actions_fixture, ACTION_TRACE_OUTPUT));
}

unsafe extern "C" fn test_actions_parse_signal(_i: c_int) {
    ck_assert_int_eq(actions_parse(&raw mut actions_fixture, c"signal,num=1,pid=1234".as_ptr(), ptr::null()), 0);

    ck_assert_int_eq(actions_fixture.len, 1);
    ck_assert_int_eq((*actions_fixture.list.add(0)).r#type, ACTION_SIGNAL);
    ck_assert_int_eq((*actions_fixture.list.add(0)).signal, 1);
    ck_assert_int_eq((*actions_fixture.list.add(0)).pid, 1234);
    ck_assert(present(&raw mut actions_fixture, ACTION_SIGNAL));
}

unsafe extern "C" fn test_actions_parse_signal_swapped(_i: c_int) {
    ck_assert_int_eq(actions_parse(&raw mut actions_fixture, c"signal,pid=1234,num=1".as_ptr(), ptr::null()), 0);

    ck_assert_int_eq(actions_fixture.len, 1);
    ck_assert_int_eq((*actions_fixture.list.add(0)).r#type, ACTION_SIGNAL);
    ck_assert_int_eq((*actions_fixture.list.add(0)).signal, 1);
    ck_assert_int_eq((*actions_fixture.list.add(0)).pid, 1234);
    ck_assert(present(&raw mut actions_fixture, ACTION_SIGNAL));
}

unsafe extern "C" fn test_actions_parse_signal_parent(_i: c_int) {
    ck_assert_int_eq(actions_parse(&raw mut actions_fixture, c"signal,pid=parent,num=1".as_ptr(), ptr::null()), 0);

    ck_assert_int_eq(actions_fixture.len, 1);
    ck_assert_int_eq((*actions_fixture.list.add(0)).r#type, ACTION_SIGNAL);
    ck_assert_int_eq((*actions_fixture.list.add(0)).signal, 1);
    ck_assert_int_eq((*actions_fixture.list.add(0)).pid, -1);
    ck_assert(present(&raw mut actions_fixture, ACTION_SIGNAL));
}

unsafe extern "C" fn test_actions_parse_signal_no_arg(_i: c_int) {
    ck_assert_int_eq(actions_parse(&raw mut actions_fixture, c"signal".as_ptr(), ptr::null()), -1);

    ck_assert_int_eq(actions_fixture.len, 0);
    ck_assert(!present(&raw mut actions_fixture, ACTION_SIGNAL));
}

unsafe extern "C" fn test_actions_parse_signal_no_pid(_i: c_int) {
    ck_assert_int_eq(actions_parse(&raw mut actions_fixture, c"signal,num=1".as_ptr(), ptr::null()), -1);

    ck_assert_int_eq(actions_fixture.len, 0);
    ck_assert(!present(&raw mut actions_fixture, ACTION_SIGNAL));
}

unsafe extern "C" fn test_actions_parse_signal_no_num(_i: c_int) {
    ck_assert_int_eq(actions_parse(&raw mut actions_fixture, c"signal,pid=1234".as_ptr(), ptr::null()), -1);

    ck_assert_int_eq(actions_fixture.len, 0);
    ck_assert(!present(&raw mut actions_fixture, ACTION_SIGNAL));
}

unsafe extern "C" fn test_actions_parse_signal_arg_bad(_i: c_int) {
    ck_assert_int_eq(actions_parse(&raw mut actions_fixture, c"signal,foo=bar".as_ptr(), ptr::null()), -1);

    ck_assert_int_eq(actions_fixture.len, 0);
    ck_assert(!present(&raw mut actions_fixture, ACTION_SIGNAL));
}

unsafe extern "C" fn test_actions_parse_shell(_i: c_int) {
    ck_assert_int_eq(actions_parse(&raw mut actions_fixture, c"shell,command=echo Hello".as_ptr(), ptr::null()), 0);

    ck_assert_int_eq(actions_fixture.len, 1);
    ck_assert_int_eq((*actions_fixture.list.add(0)).r#type, ACTION_SHELL);
    ck_assert_str_eq((*actions_fixture.list.add(0)).command, c"echo Hello".as_ptr());
    ck_assert(present(&raw mut actions_fixture, ACTION_SHELL));
}

unsafe extern "C" fn test_actions_parse_shell_no_arg(_i: c_int) {
    ck_assert_int_eq(actions_parse(&raw mut actions_fixture, c"shell".as_ptr(), ptr::null()), -1);

    ck_assert_int_eq(actions_fixture.len, 0);
    ck_assert(!present(&raw mut actions_fixture, ACTION_SHELL));
}

unsafe extern "C" fn test_actions_parse_shell_arg_bad(_i: c_int) {
    ck_assert_int_eq(actions_parse(&raw mut actions_fixture, c"shell,foo=bar".as_ptr(), ptr::null()), -1);
    ck_assert_int_eq(actions_fixture.len, 0);
    ck_assert(!present(&raw mut actions_fixture, ACTION_SHELL));
}

unsafe extern "C" fn test_actions_parse_continue(_i: c_int) {
    ck_assert_int_eq(actions_parse(&raw mut actions_fixture, c"continue".as_ptr(), ptr::null()), 0);

    ck_assert_int_eq(actions_fixture.len, 1);
    ck_assert_int_eq((*actions_fixture.list.add(0)).r#type, ACTION_CONTINUE);
    ck_assert(present(&raw mut actions_fixture, ACTION_CONTINUE));
}

unsafe extern "C" fn test_actions_parse_continue_arg_bad(_i: c_int) {
    ck_assert_int_eq(actions_parse(&raw mut actions_fixture, c"continue,foo=bar".as_ptr(), ptr::null()), -1);

    ck_assert_int_eq(actions_fixture.len, 0);
    ck_assert(!present(&raw mut actions_fixture, ACTION_CONTINUE));
}

unsafe extern "C" fn test_actions_parse_invalid(_i: c_int) {
    ck_assert_int_eq(actions_parse(&raw mut actions_fixture, c"foobar".as_ptr(), ptr::null()), -1);

    ck_assert_int_eq(actions_fixture.len, 0);
}

unsafe extern "C" fn test_actions_perform_continue(_i: c_int) {
    actions_add_continue(&raw mut actions_fixture);
    ck_assert_int_eq(actions_perform(&raw mut actions_fixture), 0);

    ck_assert(actions_fixture.continue_flag);
}

unsafe extern "C" fn test_actions_perform_continue_after_successful_shell_command(_i: c_int) {
    actions_add_shell(&raw mut actions_fixture, c"exit 0".as_ptr());
    actions_add_continue(&raw mut actions_fixture);
    ck_assert_int_eq(actions_perform(&raw mut actions_fixture), 0 << 8);

    ck_assert(actions_fixture.continue_flag);
}

unsafe extern "C" fn test_actions_perform_continue_after_failed_shell_command(_i: c_int) {
    actions_add_shell(&raw mut actions_fixture, c"exit 1".as_ptr());
    actions_add_continue(&raw mut actions_fixture);
    ck_assert_int_eq(actions_perform(&raw mut actions_fixture), 1 << 8);

    ck_assert(!actions_fixture.continue_flag);
}

unsafe extern "C" fn test_actions_perform_continue_unset_flag(_i: c_int) {
    actions_fixture.continue_flag = true;

    actions_add_shell(&raw mut actions_fixture, c"exit 1".as_ptr());
    actions_add_continue(&raw mut actions_fixture);
    ck_assert_int_eq(actions_perform(&raw mut actions_fixture), 1 << 8);

    ck_assert(!actions_fixture.continue_flag);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn actions_suite() -> *mut Suite {
    let s: *mut Suite = suite_create(c"actions".as_ptr());
    let mut tc: *mut TCase;

    tc = tcase_create(c"alloc".as_ptr());
    tcase_add_test(tc, Some(test_actions_init));
    tcase_add_test(tc, Some(test_actions_destroy));
    tcase_add_test(tc, Some(test_actions_reallocate));
    suite_add_tcase(s, tc);

    tc = tcase_create(c"add".as_ptr());
    tcase_add_checked_fixture(tc, Some(actions_fixture_setup), Some(actions_fixture_teardown));
    tcase_add_test(tc, Some(test_actions_add_trace_output));
    tcase_add_test(tc, Some(test_actions_add_signal));
    tcase_add_test(tc, Some(test_actions_add_shell));
    tcase_add_test(tc, Some(test_actions_add_continue));
    tcase_add_test(tc, Some(test_actions_add_multiple_same_action));
    tcase_add_test(tc, Some(test_actions_add_multiple_different_action));
    suite_add_tcase(s, tc);

    tc = tcase_create(c"parse".as_ptr());
    tcase_add_checked_fixture(tc, Some(actions_fixture_setup), Some(actions_fixture_teardown));
    tcase_add_test(tc, Some(test_actions_parse_trace_output));
    tcase_add_test(tc, Some(test_actions_parse_trace_output_arg));
    tcase_add_test(tc, Some(test_actions_parse_trace_output_arg_bad));
    tcase_add_test(tc, Some(test_actions_parse_signal));
    tcase_add_test(tc, Some(test_actions_parse_signal_swapped));
    tcase_add_test(tc, Some(test_actions_parse_signal_parent));
    tcase_add_test(tc, Some(test_actions_parse_signal_no_arg));
    tcase_add_test(tc, Some(test_actions_parse_signal_no_pid));
    tcase_add_test(tc, Some(test_actions_parse_signal_no_num));
    tcase_add_test(tc, Some(test_actions_parse_signal_arg_bad));
    tcase_add_test(tc, Some(test_actions_parse_shell));
    tcase_add_test(tc, Some(test_actions_parse_shell_no_arg));
    tcase_add_test(tc, Some(test_actions_parse_shell_arg_bad));
    tcase_add_test(tc, Some(test_actions_parse_continue));
    tcase_add_test(tc, Some(test_actions_parse_continue_arg_bad));
    tcase_add_test(tc, Some(test_actions_parse_invalid));
    suite_add_tcase(s, tc);

    tc = tcase_create(c"perform".as_ptr());
    tcase_add_checked_fixture(tc, Some(actions_fixture_setup), Some(actions_fixture_teardown));
    tcase_add_test(tc, Some(test_actions_perform_continue));
    tcase_add_test(tc, Some(test_actions_perform_continue_after_successful_shell_command));
    tcase_add_test(tc, Some(test_actions_perform_continue_after_failed_shell_command));
    tcase_add_test(tc, Some(test_actions_perform_continue_unset_flag));
    suite_add_tcase(s, tc);

    return s;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
