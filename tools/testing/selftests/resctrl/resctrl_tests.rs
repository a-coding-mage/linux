// SPDX-License-Identifier: GPL-2.0
/*
 * Resctrl tests
 *
 * Copyright (C) 2018 Intel Corporation
 *
 * Authors:
 *    Sai Praneeth Prakhya <sai.praneeth.prakhya@intel.com>,
 *    Fenghua Yu <fenghua.yu@intel.com>
 */
// C dependency: "resctrl.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type bool_t = bool;

#[repr(C)]
pub struct FILE {
	_private: [u8; 0],
}

#[repr(C)]
pub struct resctrl_test {
	pub name: *const c_char,
	pub group: *const c_char,
	pub disabled: bool_t,
	pub vendor_specific: c_uint,
	pub cleanup: Option<unsafe extern "C" fn()>,
	pub feature_check: unsafe extern "C" fn(*const resctrl_test) -> bool_t,
	pub run_test: unsafe extern "C" fn(*const resctrl_test, *const user_params) -> c_int,
}

#[repr(C)]
pub struct fill_buf_param {
	pub buf_size: c_ulong,
	pub memflush: bool_t,
}

#[repr(C)]
pub struct user_params {
	pub benchmark_cmd: [*mut c_char; BENCHMARK_ARGS],
	pub cpu: c_int,
	pub bits: c_int,
	pub fill_buf: *mut fill_buf_param,
}

unsafe extern "C" {
	static mut mbm_test: resctrl_test;
	static mut mba_test: resctrl_test;
	static mut cmt_test: resctrl_test;
	static mut l3_cat_test: resctrl_test;
	static mut l3_noncont_cat_test: resctrl_test;
	static mut l2_noncont_cat_test: resctrl_test;

	static mut optind: c_int;
	static mut optarg: *mut c_char;
	static mut errno: c_int;
	static mut snc_unreliable: bool_t;

	fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
	fn fclose(stream: *mut FILE) -> c_int;
	fn free(ptr: *mut c_void);
	fn malloc(size: usize) -> *mut c_void;
	fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
	fn printf(format: *const c_char, ...) -> c_int;
	fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
	fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
	fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
	fn strtok(str: *mut c_char, delim: *const c_char) -> *mut c_char;
	fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> core::ffi::c_long;
	fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
	fn atoi(nptr: *const c_char) -> c_int;
	fn getopt(argc: c_int, argv: *const *mut c_char, optstring: *const c_char) -> c_int;
	fn geteuid() -> c_uint;

	fn fgrep(inf: *mut FILE, str: *const c_char) -> *mut c_char;
	fn ksft_print_msg(format: *const c_char, ...);
	fn ksft_test_result_skip(format: *const c_char, ...);
	fn ksft_exit_fail_msg(format: *const c_char, ...) -> !;
	fn ksft_exit_skip(format: *const c_char, ...) -> !;
	fn ksft_print_header();
	fn ksft_set_plan(plan: c_int);
	fn ksft_finished() -> !;
	fn ksft_test_result(condition: bool_t, format: *const c_char, ...);
	fn signal_handler_register(test: *const resctrl_test) -> c_int;
	fn signal_handler_unregister();
	fn mount_resctrlfs() -> c_int;
	fn umount_resctrlfs() -> c_int;
	fn snc_nodes_per_l3_cache() -> c_int;
	fn check_resctrlfs_support() -> bool_t;
	fn filter_dmesg();
}

const ARCH_INTEL: c_uint = 1;
const ARCH_AMD: c_uint = 2;
const ARCH_HYGON: c_uint = 4;
const MINIMUM_SPAN: c_ulong = 250 * 1024 * 1024;
const BENCHMARK_ARGS: usize = 64;

/* Volatile memory sink to prevent compiler optimizations */
static mut sink_target: c_int = 0;
#[unsafe(no_mangle)]
pub static mut value_sink: *mut c_int = unsafe { &raw mut sink_target };

static mut resctrl_tests: [*mut resctrl_test; 6] = unsafe {
	[
		&raw mut mbm_test,
		&raw mut mba_test,
		&raw mut cmt_test,
		&raw mut l3_cat_test,
		&raw mut l3_noncont_cat_test,
		&raw mut l2_noncont_cat_test,
	]
};

unsafe fn detect_vendor() -> c_uint {
	static mut vendor_id: c_uint = 0;
	static mut initialized: bool_t = false;
	let mut s: *mut c_char = ptr::null_mut();
	let inf: *mut FILE;
	let res: *mut c_char;

	if initialized {
		return vendor_id;
	}

	inf = fopen(c"/proc/cpuinfo".as_ptr(), c"r".as_ptr());
	if inf.is_null() {
		vendor_id = 0;
		initialized = true;
		return vendor_id;
	}

	res = fgrep(inf, c"vendor_id".as_ptr());

	if !res.is_null() {
		s = strchr(res, ':' as c_int);
	}

	if !s.is_null() && strcmp(s, c": GenuineIntel\n".as_ptr()) == 0 {
		vendor_id = ARCH_INTEL;
	} else if !s.is_null() && strcmp(s, c": AuthenticAMD\n".as_ptr()) == 0 {
		vendor_id = ARCH_AMD;
	} else if !s.is_null() && strcmp(s, c": HygonGenuine\n".as_ptr()) == 0 {
		vendor_id = ARCH_HYGON;
	}

	fclose(inf);
	free(res.cast::<c_void>());

	initialized = true;
	vendor_id
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_vendor() -> c_uint {
	let vendor: c_uint;

	vendor = detect_vendor();

	if vendor == 0 {
		ksft_print_msg(c"Can not get vendor info...\n".as_ptr());
	}

	vendor
}

unsafe fn cmd_help() {
	let mut i: c_int;

	printf(c"usage: resctrl_tests [-h] [-t test list] [-n no_of_bits] [-b benchmark_cmd [option]...]\n".as_ptr());
	printf(c"\t-b benchmark_cmd [option]...: run specified benchmark for MBM, MBA and CMT\n".as_ptr());
	printf(c"\t   default benchmark is builtin fill_buf\n".as_ptr());
	printf(c"\t-t test list: run tests/groups specified by the list, ".as_ptr());
	printf(c"e.g. -t mbm,mba,cmt,cat\n".as_ptr());
	printf(c"\t\tSupported tests (group):\n".as_ptr());
	i = 0;
	while (i as usize) < resctrl_tests.len() {
		if !(*resctrl_tests[i as usize]).group.is_null() {
			printf(
				c"\t\t\t%s (%s)\n".as_ptr(),
				(*resctrl_tests[i as usize]).name,
				(*resctrl_tests[i as usize]).group,
			);
		} else {
			printf(c"\t\t\t%s\n".as_ptr(), (*resctrl_tests[i as usize]).name);
		}
		i += 1;
	}
	printf(c"\t-n no_of_bits: run cache tests using specified no of bits in cache bit mask\n".as_ptr());
	printf(c"\t-p cpu_no: specify CPU number to run the test. 1 is default\n".as_ptr());
	printf(c"\t-h: help\n".as_ptr());
}

unsafe fn test_prepare(test: *const resctrl_test) -> c_int {
	let mut res: c_int;

	res = signal_handler_register(test);
	if res != 0 {
		ksft_print_msg(c"Failed to register signal handler\n".as_ptr());
		return res;
	}

	res = mount_resctrlfs();
	if res != 0 {
		signal_handler_unregister();
		ksft_print_msg(c"Failed to mount resctrl FS\n".as_ptr());
		return res;
	}
	0
}

unsafe fn test_cleanup(test: *const resctrl_test) {
	if let Some(cleanup) = (*test).cleanup {
		cleanup();
	}
	umount_resctrlfs();
	signal_handler_unregister();
}

unsafe fn test_vendor_specific_check(test: *const resctrl_test) -> bool_t {
	if (*test).vendor_specific == 0 {
		return true;
	}

	(get_vendor() & (*test).vendor_specific) != 0
}

unsafe fn run_single_test(test: *const resctrl_test, uparams: *const user_params) {
	let ret: c_int;
	let snc_mode: c_int;

	if (*test).disabled {
		return;
	}

	if !test_vendor_specific_check(test) {
		ksft_test_result_skip(c"Hardware does not support %s\n".as_ptr(), (*test).name);
		return;
	}

	snc_mode = snc_nodes_per_l3_cache();

	ksft_print_msg(c"Starting %s test ...\n".as_ptr(), (*test).name);

	if snc_mode == 1 && snc_unreliable && get_vendor() == ARCH_INTEL {
		ksft_test_result_skip(c"SNC detection unreliable due to offline CPUs. Test results may not be accurate if SNC enabled.\n".as_ptr());
		return;
	}

	if test_prepare(test) != 0 {
		ksft_exit_fail_msg(c"Abnormal failure when preparing for the test\n".as_ptr());
	}

	if !((*test).feature_check)(test) {
		ksft_test_result_skip(
			c"Hardware does not support %s or %s is disabled\n".as_ptr(),
			(*test).name,
			(*test).name,
		);
		test_cleanup(test);
		return;
	}

	ret = ((*test).run_test)(test, uparams);
	ksft_test_result(ret == 0, c"%s: test\n".as_ptr(), (*test).name);

	test_cleanup(test);
}

/*
 * Allocate and initialize a struct fill_buf_param with user provided
 * (via "-b fill_buf <fill_buf parameters>") parameters.
 *
 * Use defaults (that may not be appropriate for all tests) for any
 * fill_buf parameters omitted by the user.
 *
 * Historically it may have been possible for user space to provide
 * additional parameters, "operation" ("read" vs "write") in
 * benchmark_cmd[3] and "once" (run "once" or until terminated) in
 * benchmark_cmd[4]. Changing these parameters have never been
 * supported with the default of "read" operation and running until
 * terminated built into the tests. Any unsupported values for
 * (original) "fill_buf" parameters are treated as failure.
 *
 * Return: On failure, forcibly exits the test on any parsing failure,
 *         returns NULL if no parsing needed (user did not actually provide
 *         "-b fill_buf").
 *         On success, returns pointer to newly allocated and fully
 *         initialized struct fill_buf_param that caller must free.
 */
unsafe fn alloc_fill_buf_param(uparams: *mut user_params) -> *mut fill_buf_param {
	let mut fill_param: *mut fill_buf_param = ptr::null_mut();
	let mut endptr: *mut c_char = ptr::null_mut();

	if (*uparams).benchmark_cmd[0].is_null()
		|| strcmp((*uparams).benchmark_cmd[0], c"fill_buf".as_ptr()) != 0
	{
		return ptr::null_mut();
	}

	fill_param = malloc(size_of::<fill_buf_param>()).cast::<fill_buf_param>();
	if fill_param.is_null() {
		ksft_exit_skip(c"Unable to allocate memory for fill_buf parameters.\n".as_ptr());
	}

	if !(*uparams).benchmark_cmd[1].is_null() && *(*uparams).benchmark_cmd[1] != b'\0' as c_char {
		errno = 0;
		(*fill_param).buf_size = strtoul((*uparams).benchmark_cmd[1], &mut endptr, 10);
		if errno != 0 || *endptr != b'\0' as c_char {
			free(fill_param.cast::<c_void>());
			ksft_exit_skip(c"Unable to parse benchmark buffer size.\n".as_ptr());
		}
	} else {
		(*fill_param).buf_size = MINIMUM_SPAN;
	}

	if !(*uparams).benchmark_cmd[2].is_null() && *(*uparams).benchmark_cmd[2] != b'\0' as c_char {
		errno = 0;
		(*fill_param).memflush = strtol((*uparams).benchmark_cmd[2], &mut endptr, 10) != 0;
		if errno != 0 || *endptr != b'\0' as c_char {
			free(fill_param.cast::<c_void>());
			ksft_exit_skip(c"Unable to parse benchmark memflush parameter.\n".as_ptr());
		}
	} else {
		(*fill_param).memflush = true;
	}

	if !(*uparams).benchmark_cmd[3].is_null() && *(*uparams).benchmark_cmd[3] != b'\0' as c_char {
		if strcmp((*uparams).benchmark_cmd[3], c"0".as_ptr()) != 0 {
			free(fill_param.cast::<c_void>());
			ksft_exit_skip(c"Only read operations supported.\n".as_ptr());
		}
	}

	if !(*uparams).benchmark_cmd[4].is_null() && *(*uparams).benchmark_cmd[4] != b'\0' as c_char {
		if strcmp((*uparams).benchmark_cmd[4], c"false".as_ptr()) != 0 {
			free(fill_param.cast::<c_void>());
			ksft_exit_skip(c"fill_buf is required to run until termination.\n".as_ptr());
		}
	}

	fill_param
}

unsafe fn init_user_params(uparams: *mut user_params) {
	memset(
		uparams.cast::<c_void>(),
		0,
		size_of::<user_params>(),
	);

	(*uparams).cpu = 1;
	(*uparams).bits = 0;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
	let mut fill_param: *mut fill_buf_param = ptr::null_mut();
	let mut tests: c_int = resctrl_tests.len() as c_int;
	let mut test_param_seen: bool_t = false;
	let mut uparams: user_params = core::mem::zeroed();
	let mut c: c_int;
	let mut i: c_int;

	init_user_params(&mut uparams);

	loop {
		c = getopt(argc, argv, c"ht:b:n:p:".as_ptr());
		if c == -1 {
			break;
		}

		match c as u8 as char {
			'b' => {
				/*
				 * First move optind back to the (first) optarg and
				 * then build the benchmark command using the
				 * remaining arguments.
				 */
				optind -= 1;
				if argc - optind >= BENCHMARK_ARGS as c_int {
					ksft_exit_fail_msg(c"Too long benchmark command".as_ptr());
				}

				/* Extract benchmark command from command line. */
				i = 0;
				while i < argc - optind {
					uparams.benchmark_cmd[i as usize] = *argv.offset((i + optind) as isize);
					i += 1;
				}
				uparams.benchmark_cmd[i as usize] = ptr::null_mut();

				break;
			}
			't' => {
				let mut token: *mut c_char;

				token = strtok(optarg, c",".as_ptr());

				if !test_param_seen {
					i = 0;
					while (i as usize) < resctrl_tests.len() {
						(*resctrl_tests[i as usize]).disabled = true;
						i += 1;
					}
					tests = 0;
					test_param_seen = true;
				}
				while !token.is_null() {
					let mut found: bool_t = false;

					i = 0;
					while (i as usize) < resctrl_tests.len() {
						if strcasecmp(token, (*resctrl_tests[i as usize]).name) == 0
							|| (!(*resctrl_tests[i as usize]).group.is_null()
								&& strcasecmp(token, (*resctrl_tests[i as usize]).group) == 0)
						{
							if (*resctrl_tests[i as usize]).disabled {
								tests += 1;
							}
							(*resctrl_tests[i as usize]).disabled = false;
							found = true;
						}
						i += 1;
					}

					if !found {
						printf(c"invalid test: %s\n".as_ptr(), token);

						return -1;
					}
					token = strtok(ptr::null_mut(), c",".as_ptr());
				}
			}
			'p' => {
				uparams.cpu = atoi(optarg);
			}
			'n' => {
				uparams.bits = atoi(optarg);
				if uparams.bits <= 0 {
					printf(c"Bail out! invalid argument for no_of_bits\n".as_ptr());
					return -1;
				}
			}
			'h' => {
				cmd_help();

				return 0;
			}
			_ => {
				printf(c"invalid argument\n".as_ptr());

				return -1;
			}
		}
	}

	fill_param = alloc_fill_buf_param(&mut uparams);
	if !fill_param.is_null() {
		uparams.fill_buf = fill_param;
	}

	ksft_print_header();

	/*
	 * Typically we need root privileges, because:
	 * 1. We write to resctrl FS
	 * 2. We execute perf commands
	 */
	if geteuid() != 0 {
		ksft_exit_skip(c"Not running as root. Skipping...\n".as_ptr());
	}

	if !check_resctrlfs_support() {
		ksft_exit_skip(c"resctrl FS does not exist. Enable X86_CPU_RESCTRL config option.\n".as_ptr());
	}

	if umount_resctrlfs() != 0 {
		ksft_exit_skip(c"resctrl FS unmount failed.\n".as_ptr());
	}

	filter_dmesg();

	ksft_set_plan(tests);

	i = 0;
	while (i as usize) < resctrl_tests.len() {
		run_single_test(resctrl_tests[i as usize], &uparams);
		i += 1;
	}

	free(fill_param.cast::<c_void>());
	ksft_finished();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
