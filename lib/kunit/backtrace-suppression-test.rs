// SPDX-License-Identifier: GPL-2.0
/*
 * KUnit test for suppressing warning tracebacks.
 *
 * Copyright (C) 2024, Guenter Roeck
 * Author: Guenter Roeck <linux@roeck-us.net>
 */

// Dependencies supplied by the surrounding kernel/KUnit translation.

unsafe fn backtrace_suppression_test_warn_direct(test: *mut kunit) {
	if !is_enabled(config_bug()) {
		kunit_skip!(test, "requires CONFIG_BUG");
	}

	kunit_warning_suppress!(test, {
		warn!(1, "This backtrace should be suppressed");
		/*
		 * Count must be checked inside the scope; the handle
		 * is not accessible after the block exits.
		 */
		kunit_expect_suppressed_warning_count!(test, 1);
	});
	kunit_expect_false!(test, kunit_has_active_suppress_warning());
}

#[inline(never)]
unsafe fn trigger_backtrace_warn() {
	warn!(1, "This backtrace should be suppressed");
}

unsafe fn backtrace_suppression_test_warn_indirect(test: *mut kunit) {
	if !is_enabled(config_bug()) {
		kunit_skip!(test, "requires CONFIG_BUG");
	}

	kunit_warning_suppress!(test, {
		trigger_backtrace_warn();
		kunit_expect_suppressed_warning_count!(test, 1);
	});
}

unsafe fn backtrace_suppression_test_warn_multi(test: *mut kunit) {
	if !is_enabled(config_bug()) {
		kunit_skip!(test, "requires CONFIG_BUG");
	}

	kunit_warning_suppress!(test, {
		warn!(1, "This backtrace should be suppressed");
		trigger_backtrace_warn();
		kunit_expect_suppressed_warning_count!(test, 2);
	});
}

unsafe fn backtrace_suppression_test_warn_on_direct(test: *mut kunit) {
	if !is_enabled(config_bug()) {
		kunit_skip!(test, "requires CONFIG_BUG");
	}
	if !is_enabled(config_debug_bugverbose()) && !is_enabled(config_kallsyms()) {
		kunit_skip!(test, "requires CONFIG_DEBUG_BUGVERBOSE or CONFIG_KALLSYMS");
	}

	kunit_warning_suppress!(test, {
		warn_on!(1);
		kunit_expect_suppressed_warning_count!(test, 1);
	});
}

#[inline(never)]
unsafe fn trigger_backtrace_warn_on() {
	warn_on!(1);
}

unsafe fn backtrace_suppression_test_warn_on_indirect(test: *mut kunit) {
	if !is_enabled(config_bug()) {
		kunit_skip!(test, "requires CONFIG_BUG");
	}
	if !is_enabled(config_debug_bugverbose()) {
		kunit_skip!(test, "requires CONFIG_DEBUG_BUGVERBOSE");
	}

	kunit_warning_suppress!(test, {
		trigger_backtrace_warn_on();
		kunit_expect_suppressed_warning_count!(test, 1);
	});
}

unsafe fn backtrace_suppression_test_count(test: *mut kunit) {
	if !is_enabled(config_bug()) {
		kunit_skip!(test, "requires CONFIG_BUG");
	}

	kunit_warning_suppress!(test, {
		kunit_expect_suppressed_warning_count!(test, 0);

		warn!(1, "suppressed");
		kunit_expect_suppressed_warning_count!(test, 1);

		warn!(1, "suppressed again");
		kunit_expect_suppressed_warning_count!(test, 2);
	});
}

unsafe fn backtrace_suppression_test_active_state(test: *mut kunit) {
	kunit_expect_false!(test, kunit_has_active_suppress_warning());

	kunit_warning_suppress!(test, {
		kunit_expect_true!(test, kunit_has_active_suppress_warning());
	});

	kunit_expect_false!(test, kunit_has_active_suppress_warning());

	kunit_warning_suppress!(test, {
		kunit_expect_true!(test, kunit_has_active_suppress_warning());
	});

	kunit_expect_false!(test, kunit_has_active_suppress_warning());
}

unsafe fn backtrace_suppression_test_multi_scope(test: *mut kunit) {
	let (mut sw1, mut sw2): (*mut kunit_suppressed_warning, *mut kunit_suppressed_warning);

	if !is_enabled(config_bug()) {
		kunit_skip!(test, "requires CONFIG_BUG");
	}
	if !is_enabled(config_debug_bugverbose()) {
		kunit_skip!(test, "requires CONFIG_DEBUG_BUGVERBOSE");
	}

	sw1 = kunit_start_suppress_warning(test);
	trigger_backtrace_warn_on();
	warn!(1, "suppressed by sw1");
	kunit_end_suppress_warning(test, sw1);

	sw2 = kunit_start_suppress_warning(test);
	warn!(1, "suppressed by sw2");
	kunit_end_suppress_warning(test, sw2);

	kunit_expect_eq!(test, kunit_suppressed_warning_count(sw1), 2);
	kunit_expect_eq!(test, kunit_suppressed_warning_count(sw2), 1);
}

#[repr(C)]
struct cross_kthread_data {
	was_active: bool,
	done: completion,
}

unsafe fn cross_kthread_fn(data: *mut core::ffi::c_void) -> i32 {
	let d = data as *mut cross_kthread_data;

	(*d).was_active = kunit_has_active_suppress_warning();
	complete(&mut (*d).done);
	while !kthread_should_stop() {
		schedule();
	}
	0
}

unsafe fn backtrace_suppression_test_cross_kthread(test: *mut kunit) {
	let mut data: cross_kthread_data;
	let mut task: *mut task_struct;

	data.was_active = false;
	init_completion(&mut data.done);

	kunit_warning_suppress!(test, {
		task = kthread_run(cross_kthread_fn, &mut data as *mut _ as *mut core::ffi::c_void, "kunit-cross-test");
		kunit_assert_false!(test, is_err(task));
		wait_for_completion(&mut data.done);
		kthread_stop(task);
	});

	kunit_expect_false!(test, data.was_active);
}

static mut backtrace_suppression_test_cases: [kunit_case; 10] = [
	kunit_case!(backtrace_suppression_test_warn_direct),
	kunit_case!(backtrace_suppression_test_warn_indirect),
	kunit_case!(backtrace_suppression_test_warn_multi),
	kunit_case!(backtrace_suppression_test_warn_on_direct),
	kunit_case!(backtrace_suppression_test_warn_on_indirect),
	kunit_case!(backtrace_suppression_test_count),
	kunit_case!(backtrace_suppression_test_active_state),
	kunit_case!(backtrace_suppression_test_multi_scope),
	kunit_case!(backtrace_suppression_test_cross_kthread),
	kunit_case_empty!(),
];

static mut backtrace_suppression_test_suite: kunit_suite = kunit_suite {
	name: "backtrace-suppression-test",
	test_cases: backtrace_suppression_test_cases.as_mut_ptr(),
};
kunit_test_suites!(backtrace_suppression_test_suite);

module_license!("GPL");
module_description!("KUnit test to verify warning backtrace suppression");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
