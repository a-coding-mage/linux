// SPDX-License-Identifier: GPL-2.0
// Translated from lib/perf/tests/test-evsel.c.
// C includes translated as external dependencies:
// stdarg.h, stdio.h, string.h, linux/perf_event.h, linux/kernel.h,
// perf/cpumap.h, perf/threadmap.h, perf/evsel.h, internal/evsel.h,
// internal/tests.h, tests.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type __u64 = u64;
type uint64_t = u64;
type va_list = *mut c_void;

#[repr(C)]
pub struct perf_cpu_map {
	_private: [u8; 0],
}

#[repr(C)]
pub struct perf_thread_map {
	_private: [u8; 0],
}

#[repr(C)]
pub struct perf_event_mmap_page {
	pub cap_user_rdpmc: u64,
	pub index: u32,
	pub pmc_width: u16,
}

#[repr(C)]
pub struct perf_evsel {
	pub leader: *mut perf_evsel,
	pub nr_members: c_int,
}

#[repr(C)]
pub struct perf_counts_values {
	pub val: u64,
	pub ena: u64,
	pub run: u64,
	pub id: u64,
	pub lost: u64,
}

#[repr(C)]
pub struct perf_event_attr {
	pub type_: u32,
	pub config: u64,
	pub config1: u64,
	pub disabled: u64,
	pub read_format: u64,
}

#[repr(C)]
pub enum libperf_print_level {
	LIBPERF_WARN = 0,
	LIBPERF_INFO = 1,
	LIBPERF_DEBUG = 2,
}

unsafe extern "C" {
	static mut stderr: *mut c_void;
	static mut tests_failed: c_int;

	static PERF_TYPE_SOFTWARE: u32;
	static PERF_TYPE_HARDWARE: u32;
	static PERF_COUNT_SW_CPU_CLOCK: u64;
	static PERF_COUNT_SW_TASK_CLOCK: u64;
	static PERF_COUNT_HW_INSTRUCTIONS: c_int;
	static PERF_COUNT_HW_CPU_CYCLES: c_int;
	static PERF_FORMAT_TOTAL_TIME_ENABLED: u64;
	static PERF_FORMAT_TOTAL_TIME_RUNNING: u64;
	static PERF_FORMAT_ID: u64;
	static PERF_FORMAT_LOST: u64;
	static PERF_FORMAT_GROUP: u64;

	fn vfprintf(stream: *mut c_void, fmt: *const c_char, ap: va_list) -> c_int;
	fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;

	fn libperf_init(print_fn: Option<unsafe extern "C" fn(libperf_print_level, *const c_char, va_list) -> c_int>);
	fn perf_cpu_map__new_online_cpus() -> *mut perf_cpu_map;
	fn perf_cpu_map__nr(cpus: *mut perf_cpu_map) -> c_int;
	fn perf_cpu_map__put(cpus: *mut perf_cpu_map);
	fn perf_thread_map__new_dummy() -> *mut perf_thread_map;
	fn perf_thread_map__new_array(nr: c_int, array: *const c_int) -> *mut perf_thread_map;
	fn perf_thread_map__set_pid(threads: *mut perf_thread_map, idx: c_int, pid: c_int);
	fn perf_thread_map__put(threads: *mut perf_thread_map);
	fn perf_evsel__new(attr: *mut perf_event_attr) -> *mut perf_evsel;
	fn perf_evsel__open(evsel: *mut perf_evsel, cpus: *mut perf_cpu_map, threads: *mut perf_thread_map) -> c_int;
	fn perf_evsel__read(evsel: *mut perf_evsel, cpu: c_int, thread: c_int, counts: *mut perf_counts_values) -> c_int;
	fn perf_evsel__close(evsel: *mut perf_evsel);
	fn perf_evsel__delete(evsel: *mut perf_evsel);
	fn perf_evsel__enable(evsel: *mut perf_evsel) -> c_int;
	fn perf_evsel__disable(evsel: *mut perf_evsel) -> c_int;
	fn perf_evsel__mmap(evsel: *mut perf_evsel, pages: c_int) -> c_int;
	fn perf_evsel__mmap_base(evsel: *mut perf_evsel, cpu: c_int, thread: c_int) -> *mut perf_event_mmap_page;
	fn perf_evsel__munmap(evsel: *mut perf_evsel);
}

// External test macros from internal/tests.h and tests.h, represented as
// local Rust macros preserving source-level control flow at call sites.
macro_rules! __T {
	($msg:expr, $cond:expr) => {
		if !($cond) {
			unsafe {
				tests_failed += 1;
			}
		}
	};
}

macro_rules! __T_VERBOSE {
	($($arg:tt)*) => {};
}

macro_rules! __T_START {
	() => {};
}

macro_rules! __T_END {
	() => {};
}

unsafe extern "C" fn libperf_print(
	_level: libperf_print_level,
	fmt: *const c_char,
	ap: va_list,
) -> c_int {
	unsafe { vfprintf(stderr, fmt, ap) }
}

unsafe fn test_stat_cpu() -> c_int {
	let mut cpus: *mut perf_cpu_map;
	let mut evsel: *mut perf_evsel;
	let mut attr = perf_event_attr {
		type_: unsafe { PERF_TYPE_SOFTWARE },
		config: unsafe { PERF_COUNT_SW_CPU_CLOCK },
		config1: 0,
		disabled: 0,
		read_format: 0,
	};
	let mut err: c_int;
	let mut idx: c_int;

	cpus = unsafe { perf_cpu_map__new_online_cpus() };
	__T!("failed to create cpus", !cpus.is_null());

	evsel = unsafe { perf_evsel__new(&mut attr) };
	__T!("failed to create evsel", !evsel.is_null());

	err = unsafe { perf_evsel__open(evsel, cpus, core::ptr::null_mut()) };
	__T!("failed to open evsel", err == 0);

	idx = 0;
	while idx < unsafe { perf_cpu_map__nr(cpus) } {
		let mut counts = perf_counts_values {
			val: 0,
			ena: 0,
			run: 0,
			id: 0,
			lost: 0,
		};

		unsafe { perf_evsel__read(evsel, idx, 0, &mut counts) };
		__T!("failed to read value for evsel", counts.val != 0);
		idx += 1;
	}

	unsafe { perf_evsel__close(evsel) };
	unsafe { perf_evsel__delete(evsel) };

	unsafe { perf_cpu_map__put(cpus) };
	0
}

unsafe fn test_stat_thread() -> c_int {
	let mut counts = perf_counts_values {
		val: 0,
		ena: 0,
		run: 0,
		id: 0,
		lost: 0,
	};
	let mut threads: *mut perf_thread_map;
	let mut evsel: *mut perf_evsel;
	let mut attr = perf_event_attr {
		type_: unsafe { PERF_TYPE_SOFTWARE },
		config: unsafe { PERF_COUNT_SW_TASK_CLOCK },
		config1: 0,
		disabled: 0,
		read_format: 0,
	};
	let mut err: c_int;

	threads = unsafe { perf_thread_map__new_dummy() };
	__T!("failed to create threads", !threads.is_null());

	unsafe { perf_thread_map__set_pid(threads, 0, 0) };

	evsel = unsafe { perf_evsel__new(&mut attr) };
	__T!("failed to create evsel", !evsel.is_null());

	err = unsafe { perf_evsel__open(evsel, core::ptr::null_mut(), threads) };
	__T!("failed to open evsel", err == 0);

	unsafe { perf_evsel__read(evsel, 0, 0, &mut counts) };
	__T!("failed to read value for evsel", counts.val != 0);

	unsafe { perf_evsel__close(evsel) };
	unsafe { perf_evsel__delete(evsel) };

	unsafe { perf_thread_map__put(threads) };
	0
}

unsafe fn test_stat_thread_enable() -> c_int {
	let mut counts = perf_counts_values {
		val: 0,
		ena: 0,
		run: 0,
		id: 0,
		lost: 0,
	};
	let mut threads: *mut perf_thread_map;
	let mut evsel: *mut perf_evsel;
	let mut attr = perf_event_attr {
		type_: unsafe { PERF_TYPE_SOFTWARE },
		config: unsafe { PERF_COUNT_SW_TASK_CLOCK },
		config1: 0,
		disabled: 1,
		read_format: 0,
	};
	let mut err: c_int;

	threads = unsafe { perf_thread_map__new_dummy() };
	__T!("failed to create threads", !threads.is_null());

	unsafe { perf_thread_map__set_pid(threads, 0, 0) };

	evsel = unsafe { perf_evsel__new(&mut attr) };
	__T!("failed to create evsel", !evsel.is_null());

	err = unsafe { perf_evsel__open(evsel, core::ptr::null_mut(), threads) };
	__T!("failed to open evsel", err == 0);

	unsafe { perf_evsel__read(evsel, 0, 0, &mut counts) };
	__T!("failed to read value for evsel", counts.val == 0);

	err = unsafe { perf_evsel__enable(evsel) };
	__T!("failed to enable evsel", err == 0);

	unsafe { perf_evsel__read(evsel, 0, 0, &mut counts) };
	__T!("failed to read value for evsel", counts.val != 0);

	err = unsafe { perf_evsel__disable(evsel) };
	__T!("failed to enable evsel", err == 0);

	unsafe { perf_evsel__close(evsel) };
	unsafe { perf_evsel__delete(evsel) };

	unsafe { perf_thread_map__put(threads) };
	0
}

unsafe fn test_stat_user_read(event: c_int) -> c_int {
	let mut counts = perf_counts_values {
		val: 0,
		ena: 0,
		run: 0,
		id: 0,
		lost: 0,
	};
	let mut threads: *mut perf_thread_map;
	let mut evsel: *mut perf_evsel;
	let mut pc: *mut perf_event_mmap_page;
	let mut attr = perf_event_attr {
		type_: unsafe { PERF_TYPE_HARDWARE },
		config: event as u64,
		// #ifdef __aarch64__: Request user access.
		#[cfg(target_arch = "aarch64")]
		config1: 0x2,
		#[cfg(not(target_arch = "aarch64"))]
		config1: 0,
		disabled: 0,
		read_format: 0,
	};
	let mut err: c_int;
	let mut i: c_int;

	threads = unsafe { perf_thread_map__new_dummy() };
	__T!("failed to create threads", !threads.is_null());

	unsafe { perf_thread_map__set_pid(threads, 0, 0) };

	evsel = unsafe { perf_evsel__new(&mut attr) };
	__T!("failed to create evsel", !evsel.is_null());

	err = unsafe { perf_evsel__open(evsel, core::ptr::null_mut(), threads) };
	__T!("failed to open evsel", err == 0);

	err = unsafe { perf_evsel__mmap(evsel, 0) };
	__T!("failed to mmap evsel", err == 0);

	pc = unsafe { perf_evsel__mmap_base(evsel, 0, 0) };
	__T!("failed to get mmapped address", !pc.is_null());

	#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
	{
		__T!("userspace counter access not supported", unsafe { (*pc).cap_user_rdpmc != 0 });
		__T!("userspace counter access not enabled", unsafe { (*pc).index != 0 });
		__T!("userspace counter width not set", unsafe { (*pc).pmc_width >= 32 });
	}

	unsafe { perf_evsel__read(evsel, 0, 0, &mut counts) };
	__T!("failed to read value for evsel", counts.val != 0);

	i = 0;
	while i < 5 {
		let mut count: c_int = 0x10000i32 << i;
		let mut start: __u64;
		let mut end: __u64;
		let mut last: __u64 = 0;

		__T_VERBOSE!("\tloop = %u, ", count as c_uint);

		unsafe { perf_evsel__read(evsel, 0, 0, &mut counts) };
		start = counts.val;

		while {
			let old = count;
			count -= 1;
			old != 0
		} {}

		unsafe { perf_evsel__read(evsel, 0, 0, &mut counts) };
		end = counts.val;

		__T!("invalid counter data", end.wrapping_sub(start) > last);
		last = end.wrapping_sub(start);
		__T_VERBOSE!("count = %llu\n", end.wrapping_sub(start));
		i += 1;
	}

	unsafe { perf_evsel__munmap(evsel) };
	unsafe { perf_evsel__close(evsel) };
	unsafe { perf_evsel__delete(evsel) };

	unsafe { perf_thread_map__put(threads) };
	0
}

unsafe fn test_stat_read_format_single(
	attr: *mut perf_event_attr,
	threads: *mut perf_thread_map,
) -> c_int {
	let mut evsel: *mut perf_evsel;
	let mut counts = perf_counts_values {
		val: 0,
		ena: 0,
		run: 0,
		id: 0,
		lost: 0,
	};
	let mut count: c_int = 0x100000;
	let mut err: c_int;

	evsel = unsafe { perf_evsel__new(attr) };
	__T!("failed to create evsel", !evsel.is_null());

	/* skip old kernels that don't support the format */
	err = unsafe { perf_evsel__open(evsel, core::ptr::null_mut(), threads) };
	if err < 0 {
		return 0;
	}

	while {
		let old = count;
		count -= 1;
		old != 0
	} {}

	unsafe {
		memset(
			&mut counts as *mut perf_counts_values as *mut c_void,
			-1,
			core::mem::size_of_val(&counts),
		);
		perf_evsel__read(evsel, 0, 0, &mut counts);
	}

	__T!("failed to read value", counts.val != 0);
	if unsafe { (*attr).read_format } & unsafe { PERF_FORMAT_TOTAL_TIME_ENABLED } != 0 {
		__T!("failed to read TOTAL_TIME_ENABLED", counts.ena != 0);
	}
	if unsafe { (*attr).read_format } & unsafe { PERF_FORMAT_TOTAL_TIME_RUNNING } != 0 {
		__T!("failed to read TOTAL_TIME_RUNNING", counts.run != 0);
	}
	if unsafe { (*attr).read_format } & unsafe { PERF_FORMAT_ID } != 0 {
		__T!("failed to read ID", counts.id != 0);
	}
	if unsafe { (*attr).read_format } & unsafe { PERF_FORMAT_LOST } != 0 {
		__T!("failed to read LOST", counts.lost == 0);
	}

	unsafe { perf_evsel__close(evsel) };
	unsafe { perf_evsel__delete(evsel) };
	0
}

unsafe fn test_stat_read_format_group(
	attr: *mut perf_event_attr,
	threads: *mut perf_thread_map,
) -> c_int {
	let mut leader: *mut perf_evsel;
	let mut member: *mut perf_evsel;
	let mut counts = perf_counts_values {
		val: 0,
		ena: 0,
		run: 0,
		id: 0,
		lost: 0,
	};
	let mut count: c_int = 0x100000;
	let mut err: c_int;

	unsafe {
		(*attr).read_format |= PERF_FORMAT_GROUP;
	}
	leader = unsafe { perf_evsel__new(attr) };
	__T!("failed to create leader", !leader.is_null());

	unsafe {
		(*attr).read_format &= !PERF_FORMAT_GROUP;
	}
	member = unsafe { perf_evsel__new(attr) };
	__T!("failed to create member", !member.is_null());

	unsafe {
		(*member).leader = leader;
		(*leader).nr_members = 2;
	}

	/* skip old kernels that don't support the format */
	err = unsafe { perf_evsel__open(leader, core::ptr::null_mut(), threads) };
	if err < 0 {
		return 0;
	}
	err = unsafe { perf_evsel__open(member, core::ptr::null_mut(), threads) };
	if err < 0 {
		return 0;
	}

	while {
		let old = count;
		count -= 1;
		old != 0
	} {}

	unsafe {
		memset(
			&mut counts as *mut perf_counts_values as *mut c_void,
			-1,
			core::mem::size_of_val(&counts),
		);
		perf_evsel__read(leader, 0, 0, &mut counts);
	}

	__T!("failed to read leader value", counts.val != 0);
	if unsafe { (*attr).read_format } & unsafe { PERF_FORMAT_TOTAL_TIME_ENABLED } != 0 {
		__T!("failed to read leader TOTAL_TIME_ENABLED", counts.ena != 0);
	}
	if unsafe { (*attr).read_format } & unsafe { PERF_FORMAT_TOTAL_TIME_RUNNING } != 0 {
		__T!("failed to read leader TOTAL_TIME_RUNNING", counts.run != 0);
	}
	if unsafe { (*attr).read_format } & unsafe { PERF_FORMAT_ID } != 0 {
		__T!("failed to read leader ID", counts.id != 0);
	}
	if unsafe { (*attr).read_format } & unsafe { PERF_FORMAT_LOST } != 0 {
		__T!("failed to read leader LOST", counts.lost == 0);
	}

	unsafe {
		memset(
			&mut counts as *mut perf_counts_values as *mut c_void,
			-1,
			core::mem::size_of_val(&counts),
		);
		perf_evsel__read(member, 0, 0, &mut counts);
	}

	__T!("failed to read member value", counts.val != 0);
	if unsafe { (*attr).read_format } & unsafe { PERF_FORMAT_TOTAL_TIME_ENABLED } != 0 {
		__T!("failed to read member TOTAL_TIME_ENABLED", counts.ena != 0);
	}
	if unsafe { (*attr).read_format } & unsafe { PERF_FORMAT_TOTAL_TIME_RUNNING } != 0 {
		__T!("failed to read member TOTAL_TIME_RUNNING", counts.run != 0);
	}
	if unsafe { (*attr).read_format } & unsafe { PERF_FORMAT_ID } != 0 {
		__T!("failed to read member ID", counts.id != 0);
	}
	if unsafe { (*attr).read_format } & unsafe { PERF_FORMAT_LOST } != 0 {
		__T!("failed to read member LOST", counts.lost == 0);
	}

	unsafe { perf_evsel__close(member) };
	unsafe { perf_evsel__close(leader) };
	unsafe { perf_evsel__delete(member) };
	unsafe { perf_evsel__delete(leader) };
	0
}

unsafe fn test_stat_read_format() -> c_int {
	let mut threads: *mut perf_thread_map;
	let mut attr = perf_event_attr {
		type_: unsafe { PERF_TYPE_SOFTWARE },
		config: unsafe { PERF_COUNT_SW_TASK_CLOCK },
		config1: 0,
		disabled: 0,
		read_format: 0,
	};
	let mut err: c_int;
	let mut i: c_int;

	// #define FMT(_fmt)  PERF_FORMAT_ ## _fmt
	// #define FMT_TIME  (FMT(TOTAL_TIME_ENABLED) | FMT(TOTAL_TIME_RUNNING))
	let test_formats: [uint64_t; 8] = [
		0,
		unsafe { PERF_FORMAT_TOTAL_TIME_ENABLED | PERF_FORMAT_TOTAL_TIME_RUNNING },
		unsafe { PERF_FORMAT_ID },
		unsafe { PERF_FORMAT_LOST },
		unsafe { PERF_FORMAT_TOTAL_TIME_ENABLED | PERF_FORMAT_TOTAL_TIME_RUNNING | PERF_FORMAT_ID },
		unsafe { PERF_FORMAT_TOTAL_TIME_ENABLED | PERF_FORMAT_TOTAL_TIME_RUNNING | PERF_FORMAT_LOST },
		unsafe {
			PERF_FORMAT_TOTAL_TIME_ENABLED
				| PERF_FORMAT_TOTAL_TIME_RUNNING
				| PERF_FORMAT_ID
				| PERF_FORMAT_LOST
		},
		unsafe { PERF_FORMAT_ID | PERF_FORMAT_LOST },
	];
	// #undef FMT
	// #undef FMT_TIME

	threads = unsafe { perf_thread_map__new_dummy() };
	__T!("failed to create threads", !threads.is_null());

	unsafe { perf_thread_map__set_pid(threads, 0, 0) };

	i = 0;
	while i < test_formats.len() as c_int {
		attr.read_format = test_formats[i as usize];
		__T_VERBOSE!(
			"testing single read with read_format: %lx\n",
			test_formats[i as usize] as c_ulong
		);

		err = unsafe { test_stat_read_format_single(&mut attr, threads) };
		__T!("failed to read single format", err == 0);
		i += 1;
	}

	unsafe { perf_thread_map__put(threads) };

	threads = unsafe { perf_thread_map__new_array(2, core::ptr::null()) };
	__T!("failed to create threads", !threads.is_null());

	unsafe { perf_thread_map__set_pid(threads, 0, 0) };
	unsafe { perf_thread_map__set_pid(threads, 1, 0) };

	i = 0;
	while i < test_formats.len() as c_int {
		attr.read_format = test_formats[i as usize];
		__T_VERBOSE!(
			"testing group read with read_format: %lx\n",
			test_formats[i as usize] as c_ulong
		);

		err = unsafe { test_stat_read_format_group(&mut attr, threads) };
		__T!("failed to read group format", err == 0);
		i += 1;
	}

	unsafe { perf_thread_map__put(threads) };
	0
}

#[no_mangle]
pub unsafe extern "C" fn test_evsel(argc: c_int, argv: *mut *mut c_char) -> c_int {
	let _ = argc;
	let _ = argv;

	__T_START!();

	unsafe { libperf_init(Some(libperf_print)) };

	unsafe { test_stat_cpu() };
	unsafe { test_stat_thread() };
	unsafe { test_stat_thread_enable() };
	unsafe { test_stat_user_read(PERF_COUNT_HW_INSTRUCTIONS) };
	unsafe { test_stat_user_read(PERF_COUNT_HW_CPU_CYCLES) };
	unsafe { test_stat_read_format() };

	__T_END!();
	if unsafe { tests_failed } == 0 {
		0
	} else {
		-1
	}
}
