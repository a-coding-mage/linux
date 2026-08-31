// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2022, Athira Rajeev, IBM Corp.
 */

use core::mem::MaybeUninit;
use std::os::raw::{c_char, c_int, c_ulong};

/*
 * Original C dependencies:
 * #include <stdio.h>
 * #include <sys/prctl.h>
 * #include <limits.h>
 * #include "../event.h"
 * #include "../sampling_tests/misc.h"
 */

#[repr(C)]
pub struct event {
	_private: [u8; 0],
}

extern "C" {
	static SPRN_PVR: c_int;
	static POWER9: c_int;
	static POWER10: c_int;
	static POWER11: c_int;
	static PERF_COUNT_HW_CPU_CYCLES: c_ulong;
	static PERF_COUNT_HW_INSTRUCTIONS: c_ulong;
	static PERF_COUNT_HW_CACHE_REFERENCES: c_ulong;
	static PERF_COUNT_HW_CACHE_MISSES: c_ulong;
	static PERF_COUNT_HW_BRANCH_INSTRUCTIONS: c_ulong;
	static PERF_COUNT_HW_BRANCH_MISSES: c_ulong;
	static PERF_COUNT_HW_BUS_CYCLES: c_ulong;
	static PERF_COUNT_HW_STALLED_CYCLES_FRONTEND: c_ulong;
	static PERF_COUNT_HW_STALLED_CYCLES_BACKEND: c_ulong;
	static PERF_COUNT_HW_REF_CPU_CYCLES: c_ulong;
	static PERF_TYPE_HARDWARE: c_int;

	fn mfspr(sprn: c_int) -> c_int;
	fn PVR_VER(pvr: c_int) -> c_int;
	fn platform_check_for_tests() -> c_int;
	fn check_for_generic_compat_pmu() -> c_int;
	fn event_init_opts(event: *mut event, config: c_ulong, type_: c_int, name: *const c_char);
	fn event_open(event: *mut event) -> c_int;
	fn event_close(event: *mut event);
	fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
	fn SKIP_IF(condition: c_int);
	fn FAIL_IF(condition: c_int);
}

/*
 * Testcase to ensure that using invalid event in generic
 * event for PERF_TYPE_HARDWARE should fail
 */
unsafe extern "C" fn generic_events_valid_test() -> c_int {
	let mut event = MaybeUninit::<event>::uninit();
	let pvr = mfspr(SPRN_PVR);
	let event_name = b"event\0".as_ptr() as *const c_char;

	/* Check for platform support for the test */
	SKIP_IF(platform_check_for_tests());

	/* generic events is different in compat_mode */
	SKIP_IF(check_for_generic_compat_pmu());

	/*
	 * Invalid generic events in power10:
	 * - PERF_COUNT_HW_BUS_CYCLES
	 * - PERF_COUNT_HW_STALLED_CYCLES_FRONTEND
	 * - PERF_COUNT_HW_STALLED_CYCLES_BACKEND
	 * - PERF_COUNT_HW_REF_CPU_CYCLES
	 */
	if (pvr == POWER10) || (pvr == POWER11) {
		event_init_opts(event.as_mut_ptr(), PERF_COUNT_HW_CPU_CYCLES, PERF_TYPE_HARDWARE, event_name);
		FAIL_IF(event_open(event.as_mut_ptr()));
		event_close(event.as_mut_ptr());

		event_init_opts(
			event.as_mut_ptr(),
			PERF_COUNT_HW_INSTRUCTIONS,
			PERF_TYPE_HARDWARE,
			event_name,
		);
		FAIL_IF(event_open(event.as_mut_ptr()));
		event_close(event.as_mut_ptr());

		event_init_opts(
			event.as_mut_ptr(),
			PERF_COUNT_HW_CACHE_REFERENCES,
			PERF_TYPE_HARDWARE,
			event_name,
		);
		FAIL_IF(event_open(event.as_mut_ptr()));
		event_close(event.as_mut_ptr());

		event_init_opts(event.as_mut_ptr(), PERF_COUNT_HW_CACHE_MISSES, PERF_TYPE_HARDWARE, event_name);
		FAIL_IF(event_open(event.as_mut_ptr()));
		event_close(event.as_mut_ptr());

		event_init_opts(
			event.as_mut_ptr(),
			PERF_COUNT_HW_BRANCH_INSTRUCTIONS,
			PERF_TYPE_HARDWARE,
			event_name,
		);
		FAIL_IF(event_open(event.as_mut_ptr()));
		event_close(event.as_mut_ptr());

		event_init_opts(event.as_mut_ptr(), PERF_COUNT_HW_BRANCH_MISSES, PERF_TYPE_HARDWARE, event_name);
		FAIL_IF(event_open(event.as_mut_ptr()));
		event_close(event.as_mut_ptr());

		event_init_opts(event.as_mut_ptr(), PERF_COUNT_HW_BUS_CYCLES, PERF_TYPE_HARDWARE, event_name);
		FAIL_IF((event_open(event.as_mut_ptr()) == 0) as c_int);

		event_init_opts(
			event.as_mut_ptr(),
			PERF_COUNT_HW_STALLED_CYCLES_FRONTEND,
			PERF_TYPE_HARDWARE,
			event_name,
		);
		FAIL_IF((event_open(event.as_mut_ptr()) == 0) as c_int);

		event_init_opts(
			event.as_mut_ptr(),
			PERF_COUNT_HW_STALLED_CYCLES_BACKEND,
			PERF_TYPE_HARDWARE,
			event_name,
		);
		FAIL_IF((event_open(event.as_mut_ptr()) == 0) as c_int);

		event_init_opts(event.as_mut_ptr(), PERF_COUNT_HW_REF_CPU_CYCLES, PERF_TYPE_HARDWARE, event_name);
		FAIL_IF((event_open(event.as_mut_ptr()) == 0) as c_int);
	} else if PVR_VER(mfspr(SPRN_PVR)) == POWER9 {
		/*
		 * Invalid generic events in power9:
		 * - PERF_COUNT_HW_BUS_CYCLES
		 * - PERF_COUNT_HW_REF_CPU_CYCLES
		 */
		event_init_opts(event.as_mut_ptr(), PERF_COUNT_HW_CPU_CYCLES, PERF_TYPE_HARDWARE, event_name);
		FAIL_IF(event_open(event.as_mut_ptr()));
		event_close(event.as_mut_ptr());

		event_init_opts(event.as_mut_ptr(), PERF_COUNT_HW_INSTRUCTIONS, PERF_TYPE_HARDWARE, event_name);
		FAIL_IF(event_open(event.as_mut_ptr()));
		event_close(event.as_mut_ptr());

		event_init_opts(
			event.as_mut_ptr(),
			PERF_COUNT_HW_CACHE_REFERENCES,
			PERF_TYPE_HARDWARE,
			event_name,
		);
		FAIL_IF(event_open(event.as_mut_ptr()));
		event_close(event.as_mut_ptr());

		event_init_opts(event.as_mut_ptr(), PERF_COUNT_HW_CACHE_MISSES, PERF_TYPE_HARDWARE, event_name);
		FAIL_IF(event_open(event.as_mut_ptr()));
		event_close(event.as_mut_ptr());

		event_init_opts(
			event.as_mut_ptr(),
			PERF_COUNT_HW_BRANCH_INSTRUCTIONS,
			PERF_TYPE_HARDWARE,
			event_name,
		);
		FAIL_IF(event_open(event.as_mut_ptr()));
		event_close(event.as_mut_ptr());

		event_init_opts(event.as_mut_ptr(), PERF_COUNT_HW_BRANCH_MISSES, PERF_TYPE_HARDWARE, event_name);
		FAIL_IF(event_open(event.as_mut_ptr()));
		event_close(event.as_mut_ptr());

		event_init_opts(event.as_mut_ptr(), PERF_COUNT_HW_BUS_CYCLES, PERF_TYPE_HARDWARE, event_name);
		FAIL_IF((event_open(event.as_mut_ptr()) == 0) as c_int);

		event_init_opts(
			event.as_mut_ptr(),
			PERF_COUNT_HW_STALLED_CYCLES_FRONTEND,
			PERF_TYPE_HARDWARE,
			event_name,
		);
		FAIL_IF(event_open(event.as_mut_ptr()));
		event_close(event.as_mut_ptr());

		event_init_opts(
			event.as_mut_ptr(),
			PERF_COUNT_HW_STALLED_CYCLES_BACKEND,
			PERF_TYPE_HARDWARE,
			event_name,
		);
		FAIL_IF(event_open(event.as_mut_ptr()));
		event_close(event.as_mut_ptr());

		event_init_opts(event.as_mut_ptr(), PERF_COUNT_HW_REF_CPU_CYCLES, PERF_TYPE_HARDWARE, event_name);
		FAIL_IF((event_open(event.as_mut_ptr()) == 0) as c_int);
	}

	0
}

fn main() -> c_int {
	unsafe { test_harness(generic_events_valid_test, b"generic_events_valid_test\0".as_ptr() as *const c_char) }
}
