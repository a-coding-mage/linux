// SPDX-License-Identifier: GPL-2.0
/* Copyright 2026 Google LLC */

/* Translated from C. External test, BPF, and skeleton APIs are provided by
 * the surrounding selftest infrastructure.
 */

use core::ffi::{c_char, c_int, c_longlong, c_void};

const O_WRONLY: c_int = 1;
const BTF_KIND_FUNC: c_int = 12;

#[repr(C)]
pub struct btf {
	_private: [u8; 0],
}

#[repr(C)]
pub struct ring_buffer {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
	_private: [u8; 0],
}

#[repr(C)]
pub struct test_wakeup_source_maps {
	pub rb: *mut bpf_map,
}

#[repr(C)]
pub struct test_wakeup_source_progs {
	pub iterate_wakeupsources: *mut bpf_program,
}

#[repr(C)]
pub struct test_wakeup_source {
	pub maps: test_wakeup_source_maps,
	pub progs: test_wakeup_source_progs,
}

#[repr(C)]
pub struct wakeup_event_t {
	pub name: [c_char; 0],
	pub active_time_ns: c_longlong,
	pub total_time_ns: c_longlong,
}

unsafe extern "C" {
	fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
	fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
	fn close(fd: c_int) -> c_int;
	fn strlen(s: *const c_char) -> usize;
	fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
	fn printf(format: *const c_char, ...) -> c_int;

	fn ASSERT_OK_FD(fd: c_int, name: *const c_char) -> bool;
	fn ASSERT_EQ(actual: isize, expected: usize, name: *const c_char) -> bool;
	fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
	fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
	fn ASSERT_TRUE(cond: bool, name: *const c_char) -> bool;
	fn ASSERT_GT(actual: c_longlong, expected: c_longlong, name: *const c_char) -> bool;
	fn test__skip();
	fn test__start_subtest(name: *const c_char) -> bool;

	fn btf__load_vmlinux_btf() -> *mut btf;
	fn btf__find_by_name_kind(btf: *mut btf, name: *const c_char, kind: c_int) -> c_int;
	fn btf__free(btf: *mut btf);

	fn bpf_map__fd(map: *mut bpf_map) -> c_int;
	fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
	fn bpf_prog_test_run_opts(fd: c_int, opts: *mut c_void) -> c_int;

	fn ring_buffer__new(
		map_fd: c_int,
		sample_cb: Option<unsafe extern "C" fn(*mut c_void, *mut c_void, usize) -> c_int>,
		ctx: *mut c_void,
		opts: *mut c_void,
	) -> *mut ring_buffer;
	fn ring_buffer__consume(rb: *mut ring_buffer) -> c_int;
	fn ring_buffer__free(rb: *mut ring_buffer);

	fn test_wakeup_source__open_and_load() -> *mut test_wakeup_source;
	fn test_wakeup_source__destroy(obj: *mut test_wakeup_source);
	fn RUN_TESTS_wakeup_source_fail();
}

unsafe fn lock_ws(name: *const c_char) -> c_int {
	let fd: c_int;
	let bytes: isize;

	fd = open(c"/sys/power/wake_lock".as_ptr(), O_WRONLY);
	if !ASSERT_OK_FD(fd, c"open /sys/power/wake_lock".as_ptr()) {
		return -1;
	}

	bytes = write(fd, name as *const c_void, strlen(name));
	close(fd);
	if !ASSERT_EQ(bytes, strlen(name), c"write to wake_lock".as_ptr()) {
		return -1;
	}

	0
}

unsafe fn unlock_ws(name: *const c_char) {
	let fd: c_int;

	fd = open(c"/sys/power/wake_unlock".as_ptr(), O_WRONLY);
	if fd < 0 {
		return;
	}

	write(fd, name as *const c_void, strlen(name));
	close(fd);
}

#[repr(C)]
struct rb_ctx {
	name: *const c_char,
	found: bool,
	active_time_ns: c_longlong,
	total_time_ns: c_longlong,
}

unsafe extern "C" fn process_sample(ctx: *mut c_void, data: *mut c_void, _len: usize) -> c_int {
	let rb_ctx: *mut rb_ctx = ctx as *mut rb_ctx;
	let e: *mut wakeup_event_t = data as *mut wakeup_event_t;

	if strcmp((*e).name.as_ptr(), (*rb_ctx).name) == 0 {
		(*rb_ctx).found = true;
		(*rb_ctx).active_time_ns = (*e).active_time_ns;
		(*rb_ctx).total_time_ns = (*e).total_time_ns;
	}
	0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_wakeup_source() {
	let btf: *mut btf;
	let id: c_int;

	btf = btf__load_vmlinux_btf();
	if !ASSERT_OK_PTR(btf as *const c_void, c"btf_vmlinux".as_ptr()) {
		return;
	}

	id = btf__find_by_name_kind(
		btf,
		c"bpf_wakeup_sources_get_head".as_ptr(),
		BTF_KIND_FUNC,
	);
	btf__free(btf);

	if id < 0 {
		printf(
			c"%s:SKIP:bpf_wakeup_sources_get_head kfunc not found in BTF\n".as_ptr(),
			c"test_wakeup_source".as_ptr(),
		);
		test__skip();
		return;
	}

	if test__start_subtest(c"iterate_and_verify_times".as_ptr()) {
		let skel: *mut test_wakeup_source;
		let mut rb: *mut ring_buffer = core::ptr::null_mut();
		let mut rb_ctx = rb_ctx {
			name: c"bpf_selftest_ws_times".as_ptr(),
			found: false,
			active_time_ns: 0,
			total_time_ns: 0,
		};
		let err: c_int;

		skel = test_wakeup_source__open_and_load();
		if !ASSERT_OK_PTR(skel as *const c_void, c"skel_open_and_load".as_ptr()) {
			return;
		}

		rb = ring_buffer__new(
			bpf_map__fd((*skel).maps.rb),
			Some(process_sample),
			&mut rb_ctx as *mut rb_ctx as *mut c_void,
			core::ptr::null_mut(),
		);
		if !ASSERT_OK_PTR(rb as *const c_void, c"ring_buffer__new".as_ptr()) {
			test_wakeup_source__destroy(skel);
			return;
		}

		/* Create a temporary wakeup source */
		if !ASSERT_OK(lock_ws(rb_ctx.name), c"lock_ws".as_ptr()) {
			unlock_ws(rb_ctx.name);
			if !rb.is_null() {
				ring_buffer__free(rb);
			}
			test_wakeup_source__destroy(skel);
			return;
		}

		err = bpf_prog_test_run_opts(
			bpf_program__fd((*skel).progs.iterate_wakeupsources),
			core::ptr::null_mut(),
		);
		ASSERT_OK(err, c"bpf_prog_test_run".as_ptr());

		ring_buffer__consume(rb);

		ASSERT_TRUE(rb_ctx.found, c"found_test_ws_in_rb".as_ptr());
		ASSERT_GT(rb_ctx.active_time_ns, 0, c"active_time_gt_0".as_ptr());
		ASSERT_GT(rb_ctx.total_time_ns, 0, c"total_time_gt_0".as_ptr());

		unlock_ws(rb_ctx.name);
		if !rb.is_null() {
			ring_buffer__free(rb);
		}
		test_wakeup_source__destroy(skel);
	}

	RUN_TESTS_wakeup_source_fail();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
