// SPDX-License-Identifier: GPL-2.0
/*
 * Test support for libpfm4 event encodings.
 *
 * Copyright 2020 Google LLC.
 */

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

#[repr(C)]
pub struct perf_evlist {
	_private: [u8; 0],
}

#[repr(C)]
pub struct perf_evsel {
	_private: [u8; 0],
}

#[repr(C)]
pub struct evlist {
	_private: [u8; 0],
}

#[repr(C)]
pub struct test_suite {
	pub desc: *const c_char,
	pub test_cases: *mut test_case,
}

#[repr(C)]
pub struct test_case {
	pub name: *const c_char,
	pub run_case: Option<unsafe extern "C" fn(*mut test_suite, c_int) -> c_int>,
	pub desc: *const c_char,
}

#[repr(C)]
pub struct option {
	pub value: *mut c_void,
}

const ENOMEM: c_int = 12;
const TEST_SKIP: c_int = 2;

unsafe extern "C" {
	fn perf_evlist__first(evlist: *mut perf_evlist) -> *mut perf_evsel;
	fn perf_evlist__next(evsel: *mut perf_evsel) -> *mut perf_evsel;
	fn evlist__new() -> *mut evlist;
	fn evlist__core(evlist: *mut evlist) -> *mut perf_evlist;
	fn evlist__nr_groups(evlist: *mut evlist) -> c_int;
	fn evlist__put(evlist: *mut evlist);
	fn parse_libpfm_events_option(opt: *const option, str_: *const c_char, unset: c_int) -> c_int;
	fn test_assert_equal(file: *const c_char, line: c_int, ctx: *const c_char, a: c_int, b: c_int) -> c_int;
}

#[cfg(HAVE_LIBPFM)]
unsafe extern "C" fn count_pfm_events(evlist: *mut perf_evlist) -> c_int {
	let mut evsel: *mut perf_evsel;
	let mut count: c_int = 0;

	evsel = perf_evlist__first(evlist);
	while !evsel.is_null() {
		count += 1;
		evsel = perf_evlist__next(evsel);
	}
	return count;
}

#[cfg(HAVE_LIBPFM)]
unsafe extern "C" fn test__pfm_events(_test: *mut test_suite, _subtest: c_int) -> c_int {
	#[repr(C)]
	struct table_entry {
		events: *const c_char,
		nr_events: c_int,
	}

	let mut evlist: *mut evlist;
	let mut opt: option = option {
		value: ptr::null_mut(),
	};
	let table: [table_entry; 7] = [
		table_entry {
			events: c"",
			nr_events: 0,
		},
		table_entry {
			events: c"instructions",
			nr_events: 1,
		},
		table_entry {
			events: c"instructions,cycles",
			nr_events: 2,
		},
		table_entry {
			events: c"stereolab",
			nr_events: 0,
		},
		table_entry {
			events: c"instructions,instructions",
			nr_events: 2,
		},
		table_entry {
			events: c"stereolab,instructions",
			nr_events: 0,
		},
		table_entry {
			events: c"instructions,stereolab",
			nr_events: 1,
		},
	];

	let mut i: usize = 0;
	while i < table.len() {
		evlist = evlist__new();
		if evlist.is_null() {
			return -ENOMEM;
		}

		opt.value = (&mut evlist as *mut *mut evlist).cast::<c_void>();
		parse_libpfm_events_option(&opt, table[i].events, 0);
		test_assert_equal(
			file!().as_ptr().cast::<c_char>(),
			line!() as c_int,
			table[i].events,
			count_pfm_events(evlist__core(evlist)),
			table[i].nr_events,
		);
		test_assert_equal(
			file!().as_ptr().cast::<c_char>(),
			line!() as c_int,
			table[i].events,
			evlist__nr_groups(evlist),
			0,
		);

		evlist__put(evlist);
		i += 1;
	}
	return 0;
}

#[cfg(HAVE_LIBPFM)]
unsafe extern "C" fn test__pfm_group(_test: *mut test_suite, _subtest: c_int) -> c_int {
	#[repr(C)]
	struct table_entry {
		events: *const c_char,
		nr_events: c_int,
		nr_groups: c_int,
	}

	let mut evlist: *mut evlist;
	let mut opt: option = option {
		value: ptr::null_mut(),
	};
	let table: [table_entry; 10] = [
		table_entry {
			events: c"{},",
			nr_events: 0,
			nr_groups: 0,
		},
		table_entry {
			events: c"{instructions}",
			nr_events: 1,
			nr_groups: 0,
		},
		table_entry {
			events: c"{instructions},{}",
			nr_events: 1,
			nr_groups: 0,
		},
		table_entry {
			events: c"{},{instructions}",
			nr_events: 1,
			nr_groups: 0,
		},
		table_entry {
			events: c"{instructions},{instructions}",
			nr_events: 2,
			nr_groups: 0,
		},
		table_entry {
			events: c"{instructions,cycles},{instructions,cycles}",
			nr_events: 4,
			nr_groups: 2,
		},
		table_entry {
			events: c"{stereolab}",
			nr_events: 0,
			nr_groups: 0,
		},
		table_entry {
			events: c"{instructions,cycles},{instructions,stereolab}",
			nr_events: 3,
			nr_groups: 1,
		},
		table_entry {
			events: c"instructions}",
			nr_events: 1,
			nr_groups: 0,
		},
		table_entry {
			events: c"{{instructions}}",
			nr_events: 0,
			nr_groups: 0,
		},
	];

	let mut i: usize = 0;
	while i < table.len() {
		evlist = evlist__new();
		if evlist.is_null() {
			return -ENOMEM;
		}

		opt.value = (&mut evlist as *mut *mut evlist).cast::<c_void>();
		parse_libpfm_events_option(&opt, table[i].events, 0);
		test_assert_equal(
			file!().as_ptr().cast::<c_char>(),
			line!() as c_int,
			table[i].events,
			count_pfm_events(evlist__core(evlist)),
			table[i].nr_events,
		);
		test_assert_equal(
			file!().as_ptr().cast::<c_char>(),
			line!() as c_int,
			table[i].events,
			evlist__nr_groups(evlist),
			table[i].nr_groups,
		);

		evlist__put(evlist);
		i += 1;
	}
	return 0;
}

#[cfg(not(HAVE_LIBPFM))]
unsafe extern "C" fn test__pfm_events(_test: *mut test_suite, _subtest: c_int) -> c_int {
	return TEST_SKIP;
}

#[cfg(not(HAVE_LIBPFM))]
unsafe extern "C" fn test__pfm_group(_test: *mut test_suite, _subtest: c_int) -> c_int {
	return TEST_SKIP;
}

static mut pfm_tests: [test_case; 3] = [
	test_case {
		name: c"test of individual --pfm-events",
		run_case: Some(test__pfm_events),
		desc: c"not compiled in",
	},
	test_case {
		name: c"test groups of --pfm-events",
		run_case: Some(test__pfm_group),
		desc: c"not compiled in",
	},
	test_case {
		name: ptr::null(),
		run_case: None,
		desc: ptr::null(),
	},
];

#[unsafe(no_mangle)]
pub static mut suite__pfm: test_suite = test_suite {
	desc: c"Test libpfm4 support",
	test_cases: unsafe { pfm_tests.as_mut_ptr() },
};
