/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2022 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2022 Tejun Heo <tj@kernel.org>
 * Copyright (c) 2022 David Vernet <dvernet@meta.com>
 */

/* C dependencies translated as external declarations:
 * stdio.h, unistd.h, inttypes.h, signal.h, assert.h, libgen.h,
 * bpf/bpf.h, scx/common.h, scx_central.bpf.skel.h.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_ulonglong, c_void};

type bool_ = bool;
type __u64 = u64;
type __s32 = i32;
type u32 = u32;
type s32 = i32;
type size_t = usize;
type va_list = *mut c_void;

const SIGINT: c_int = 2;
const SIGTERM: c_int = 15;
const INT32_MAX: c_uint = 2147483647;
const LIBBPF_DEBUG: libbpf_print_level = 0;

#[repr(C)]
pub struct FILE {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
	_private: [u8; 0],
}

#[repr(C)]
pub struct scx_central_rodata {
	central_cpu: s32,
	nr_cpu_ids: u32,
	slice_ns: __u64,
}

#[repr(C)]
pub struct scx_central_data {
	timer_pinned: bool_,
	cpu_gimme_task: *mut c_void,
	cpu_started_at: *mut c_void,
}

#[repr(C)]
pub struct scx_central_bss {
	nr_total: __u64,
	nr_locals: __u64,
	nr_queued: __u64,
	nr_lost_pids: __u64,
	nr_timers: __u64,
	nr_dispatches: __u64,
	nr_mismatches: __u64,
	nr_retries: __u64,
	nr_overflows: __u64,
}

#[repr(C)]
pub struct scx_central {
	rodata: *mut scx_central_rodata,
	data: *mut scx_central_data,
	bss: *mut scx_central_bss,
}

type libbpf_print_level = c_int;

static HELP_FMT: &[u8] =
	b"A central FIFO sched_ext scheduler.\n\
\n\
See the top-level comment in .bpf.c for more details.\n\
\n\
Usage: %s [-s SLICE_US] [-c CPU] [-v]\n\
\n\
  -s SLICE_US   Override slice duration\n\
  -c CPU        Override the central CPU (default: 0)\n\
  -v            Print libbpf debug messages\n\
  -h            Display this help and exit\n\0";

static mut verbose: bool_ = false;
static mut exit_req: c_int = 0;

unsafe extern "C" {
	static mut stderr: *mut FILE;
	static mut stdout: *mut FILE;
	static mut optind: c_int;
	static mut optarg: *mut c_char;

	fn vfprintf(stream: *mut FILE, format: *const c_char, arg: va_list) -> c_int;
	fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
	fn printf(format: *const c_char, ...) -> c_int;
	fn fflush(stream: *mut FILE) -> c_int;
	fn sleep(seconds: c_uint) -> c_uint;
	fn signal(signum: c_int, handler: extern "C" fn(c_int)) -> extern "C" fn(c_int);
	fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
	fn basename(path: *mut c_char) -> *mut c_char;
	fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulonglong;
	fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;

	fn libbpf_set_print(
		fn_: Option<unsafe extern "C" fn(libbpf_print_level, *const c_char, va_list) -> c_int>,
	);
	fn libbpf_num_possible_cpus() -> c_int;
	fn bpf_link__destroy(link: *mut bpf_link);
	fn scx_central__destroy(skel: *mut scx_central);

	/* Macro-provided helpers from scx/common.h and scx_central.bpf.skel.h. */
	fn SCX_OPS_OPEN(ops: *const c_char, skel_name: *const c_char) -> *mut scx_central;
	fn RESIZE_ARRAY(skel: *mut scx_central, section: *const c_char, name: *const c_char, count: u32);
	fn SCX_OPS_LOAD(
		skel: *mut scx_central,
		ops: *const c_char,
		skel_name: *const c_char,
		uei: *const c_char,
	);
	fn SCX_OPS_ATTACH(
		skel: *mut scx_central,
		ops: *const c_char,
		skel_name: *const c_char,
	) -> *mut bpf_link;
	fn UEI_EXITED(skel: *mut scx_central, uei: *const c_char) -> bool_;
	fn UEI_REPORT(skel: *mut scx_central, uei: *const c_char) -> __u64;
	fn UEI_ECODE_RESTART(ecode: __u64) -> bool_;
	fn __COMPAT_ENUM_OR_ZERO(enum_name: *const c_char, value_name: *const c_char) -> __u64;
}

unsafe extern "C" fn libbpf_print_fn(
	level: libbpf_print_level,
	format: *const c_char,
	args: va_list,
) -> c_int {
	if level == LIBBPF_DEBUG && !unsafe { verbose } {
		return 0;
	}
	unsafe { vfprintf(stderr, format, args) }
}

extern "C" fn sigint_handler(_dummy: c_int) {
	unsafe {
		exit_req = 1;
	}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
	let mut skel: *mut scx_central;
	let mut link: *mut bpf_link;
	let mut seq: __u64 = 0;
	let mut ecode: __u64;
	let mut opt: __s32;

	unsafe {
		libbpf_set_print(Some(libbpf_print_fn));
		signal(SIGINT, sigint_handler);
		signal(SIGTERM, sigint_handler);
	}

	'restart: loop {
		unsafe {
			optind = 1;
			skel = SCX_OPS_OPEN(c"central_ops".as_ptr(), c"scx_central".as_ptr());

			(*(*skel).rodata).central_cpu = 0;
			(*(*skel).rodata).nr_cpu_ids = libbpf_num_possible_cpus() as u32;
			(*(*skel).rodata).slice_ns =
				__COMPAT_ENUM_OR_ZERO(c"scx_public_consts".as_ptr(), c"SCX_SLICE_DFL".as_ptr());

			assert!((*(*skel).rodata).nr_cpu_ids > 0);
			assert!((*(*skel).rodata).nr_cpu_ids <= INT32_MAX);

			loop {
				opt = getopt(argc, argv, c"s:c:vh".as_ptr());
				if opt == -1 {
					break;
				}

				match opt as u8 as char {
					's' => {
						(*(*skel).rodata).slice_ns = strtoull(optarg, core::ptr::null_mut(), 0) * 1000;
					}
					'c' => {
						let central_cpu: u32 = strtoul(optarg, core::ptr::null_mut(), 0) as u32;
						if central_cpu >= (*(*skel).rodata).nr_cpu_ids {
							fprintf(
								stderr,
								c"invalid central CPU id value, %u given (%u max)\n".as_ptr(),
								central_cpu,
								(*(*skel).rodata).nr_cpu_ids,
							);
							scx_central__destroy(skel);
							return -1;
						}
						(*(*skel).rodata).central_cpu = central_cpu as s32;
					}
					'v' => {
						verbose = true;
					}
					_ => {
						fprintf(stderr, HELP_FMT.as_ptr() as *const c_char, basename(*argv));
						return (opt != 'h' as __s32) as c_int;
					}
				}
			}

			/* Resize arrays so their element count is equal to cpu count. */
			RESIZE_ARRAY(
				skel,
				c"data".as_ptr(),
				c"cpu_gimme_task".as_ptr(),
				(*(*skel).rodata).nr_cpu_ids,
			);
			RESIZE_ARRAY(
				skel,
				c"data".as_ptr(),
				c"cpu_started_at".as_ptr(),
				(*(*skel).rodata).nr_cpu_ids,
			);

			SCX_OPS_LOAD(
				skel,
				c"central_ops".as_ptr(),
				c"scx_central".as_ptr(),
				c"uei".as_ptr(),
			);

			link = SCX_OPS_ATTACH(skel, c"central_ops".as_ptr(), c"scx_central".as_ptr());

			if !(*(*skel).data).timer_pinned {
				printf(c"WARNING : BPF_F_TIMER_CPU_PIN not available, timer not pinned to central\n".as_ptr());
			}

			while exit_req == 0 && !UEI_EXITED(skel, c"uei".as_ptr()) {
				printf(c"[SEQ %llu]\n".as_ptr(), seq);
				seq = seq.wrapping_add(1);
				printf(
					c"total   :%10llu    local:%10llu   queued:%10llu  lost:%10llu\n".as_ptr(),
					(*(*skel).bss).nr_total,
					(*(*skel).bss).nr_locals,
					(*(*skel).bss).nr_queued,
					(*(*skel).bss).nr_lost_pids,
				);
				printf(
					c"timer   :%10llu dispatch:%10llu mismatch:%10llu retry:%10llu\n".as_ptr(),
					(*(*skel).bss).nr_timers,
					(*(*skel).bss).nr_dispatches,
					(*(*skel).bss).nr_mismatches,
					(*(*skel).bss).nr_retries,
				);
				printf(c"overflow:%10llu\n".as_ptr(), (*(*skel).bss).nr_overflows);
				fflush(stdout);
				sleep(1);
			}

			bpf_link__destroy(link);
			ecode = UEI_REPORT(skel, c"uei".as_ptr());
			scx_central__destroy(skel);

			if exit_req == 0 && UEI_ECODE_RESTART(ecode) {
				continue 'restart;
			}
			return 0;
		}
	}
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
