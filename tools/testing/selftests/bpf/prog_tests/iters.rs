// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

/* Translated from C. External declarations correspond to:
 * <sys/syscall.h>, <sys/mman.h>, <sys/wait.h>, <unistd.h>, <malloc.h>,
 * <stdlib.h>, <test_progs.h>, "cgroup_helpers.h", and the *.skel.h files.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

const SYS_getpgid: c_long = 121;
const EPERM: c_int = 1;
const THREAD_NUM: usize = 2;

#[repr(C)]
pub struct FILE {
	_private: [u8; 0],
}

#[repr(C)]
pub struct pthread_mutex_t {
	_private: [u8; 0],
}

pub type pthread_t = c_ulong;

#[repr(C)]
pub struct env {
	pub has_testmod: bool,
}

#[repr(C)]
pub struct vm_range {
	pub vm_start: c_ulong,
	pub vm_end: c_ulong,
}

#[repr(C)]
pub struct iters_num_bss {
	pub res_empty_zero: c_int,
	pub res_empty_int_min: c_int,
	pub res_empty_int_max: c_int,
	pub res_empty_minus_one: c_int,
	pub res_simple_sum: c_int,
	pub res_neg_sum: c_int,
	pub res_very_neg_sum: c_int,
	pub res_neg_pos_sum: c_int,
	pub res_invalid_range: c_int,
	pub res_max_range: c_int,
	pub res_e2big_range: c_int,
	pub res_succ_elem_cnt: c_int,
	pub res_overfetched_elem_cnt: c_int,
	pub res_fail_elem_cnt: c_int,
}

#[repr(C)]
pub struct iters_num_rodata {
	pub exp_empty_zero: c_int,
	pub exp_empty_int_min: c_int,
	pub exp_empty_int_max: c_int,
	pub exp_empty_minus_one: c_int,
	pub exp_simple_sum: c_int,
	pub exp_neg_sum: c_int,
	pub exp_very_neg_sum: c_int,
	pub exp_neg_pos_sum: c_int,
	pub exp_invalid_range: c_int,
	pub exp_max_range: c_int,
	pub exp_e2big_range: c_int,
	pub exp_succ_elem_cnt: c_int,
	pub exp_overfetched_elem_cnt: c_int,
	pub exp_fail_elem_cnt: c_int,
}

#[repr(C)]
pub struct iters_num {
	pub bss: *mut iters_num_bss,
	pub rodata: *mut iters_num_rodata,
}

#[repr(C)]
pub struct iters_testmod_seq_bss {
	pub res_empty: c_int,
	pub res_full: c_int,
	pub res_truncated: c_int,
}

#[repr(C)]
pub struct iters_testmod_seq_rodata {
	pub exp_empty: c_int,
	pub exp_full: c_int,
	pub exp_truncated: c_int,
}

#[repr(C)]
pub struct iters_testmod_seq {
	pub bss: *mut iters_testmod_seq_bss,
	pub rodata: *mut iters_testmod_seq_rodata,
}

#[repr(C)]
pub struct iters_task_vma_bss {
	pub target_pid: c_int,
	pub vmas_seen: c_uint,
	pub vm_ranges: *mut vm_range,
}

#[repr(C)]
pub struct iters_task_vma {
	pub bss: *mut iters_task_vma_bss,
}

#[repr(C)]
pub struct iters_task_bss {
	pub target_pid: c_int,
	pub procs_cnt: c_int,
	pub threads_cnt: c_int,
	pub proc_threads_cnt: c_int,
	pub invalid_cnt: c_int,
}

#[repr(C)]
pub struct iters_task {
	pub bss: *mut iters_task_bss,
}

#[repr(C)]
pub struct iters_css_task_bss {
	pub target_pid: c_int,
	pub cg_id: c_int,
	pub css_task_cnt: c_int,
}

#[repr(C)]
pub struct iters_css_task {
	pub bss: *mut iters_css_task_bss,
}

#[repr(C)]
pub struct iters_css_bss {
	pub target_pid: c_int,
	pub root_cg_id: c_int,
	pub leaf_cg_id: c_int,
	pub pre_order_cnt: c_int,
	pub first_cg_id: c_int,
	pub post_order_cnt: c_int,
	pub last_cg_id: c_int,
	pub children_cnt: c_int,
	pub tree_high: c_int,
}

#[repr(C)]
pub struct iters_css {
	pub bss: *mut iters_css_bss,
}

#[repr(C)]
struct cgroup {
	path: *const c_char,
	fd: c_int,
}

extern "C" {
	static mut env: env;
	static mut errno: c_int;

	fn usleep(usec: c_uint) -> c_int;
	fn getpid() -> c_int;
	fn getpgid(pid: c_int) -> c_int;
	fn syscall(num: c_long, ...) -> c_long;
	fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
	fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
	fn fclose(stream: *mut FILE) -> c_int;
	fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
	fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> c_int;
	fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> c_int;
	fn pthread_exit(value_ptr: *mut c_void) -> !;
	fn pthread_create(
		thread: *mut pthread_t,
		attr: *const c_void,
		start_routine: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
		arg: *mut c_void,
	) -> c_int;
	fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;

	fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
	fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
	fn ASSERT_EQ(actual: c_long, expected: c_long, name: *const c_char) -> bool;
	fn ASSERT_GT(actual: c_long, expected: c_long, name: *const c_char) -> bool;
	fn ASSERT_GE(actual: c_long, expected: c_long, name: *const c_char) -> bool;
	fn test__skip();
	fn test__start_subtest(name: *const c_char) -> bool;
	fn stack_mprotect() -> c_int;
	fn setup_cgroup_environment() -> c_int;
	fn create_and_get_cgroup(path: *const c_char) -> c_int;
	fn get_cgroup_id(path: *const c_char) -> c_int;
	fn join_cgroup(path: *const c_char) -> c_int;
	fn cleanup_cgroup_environment();

	fn iters_num__open_and_load() -> *mut iters_num;
	fn iters_num__attach(skel: *mut iters_num) -> c_int;
	fn iters_num__detach(skel: *mut iters_num);
	fn iters_num__destroy(skel: *mut iters_num);

	fn iters_testmod_seq__open_and_load() -> *mut iters_testmod_seq;
	fn iters_testmod_seq__attach(skel: *mut iters_testmod_seq) -> c_int;
	fn iters_testmod_seq__detach(skel: *mut iters_testmod_seq);
	fn iters_testmod_seq__destroy(skel: *mut iters_testmod_seq);

	fn iters_task_vma__open_and_load() -> *mut iters_task_vma;
	fn iters_task_vma__attach(skel: *mut iters_task_vma) -> c_int;
	fn iters_task_vma__detach(skel: *mut iters_task_vma);
	fn iters_task_vma__destroy(skel: *mut iters_task_vma);

	fn iters_task__open_and_load() -> *mut iters_task;
	fn iters_task__attach(skel: *mut iters_task) -> c_int;
	fn iters_task__detach(skel: *mut iters_task);
	fn iters_task__destroy(skel: *mut iters_task);

	fn iters_css_task__open_and_load() -> *mut iters_css_task;
	fn iters_css_task__attach(skel: *mut iters_css_task) -> c_int;
	fn iters_css_task__detach(skel: *mut iters_css_task);
	fn iters_css_task__destroy(skel: *mut iters_css_task);

	fn iters_css__open_and_load() -> *mut iters_css;
	fn iters_css__attach(skel: *mut iters_css) -> c_int;
	fn iters_css__detach(skel: *mut iters_css);
	fn iters_css__destroy(skel: *mut iters_css);

	fn RUN_TESTS(name: *const c_char);
}

macro_rules! cstr {
	($s:literal) => {
		concat!($s, "\0").as_ptr() as *const c_char
	};
}

macro_rules! validate_num_case {
	($skel:expr, $res:ident, $exp:ident, $name:literal) => {
		ASSERT_EQ(
			(*(*$skel).bss).$res as c_long,
			(*(*$skel).rodata).$exp as c_long,
			cstr!($name),
		)
	};
}

macro_rules! validate_testmod_seq_case {
	($skel:expr, $res:ident, $exp:ident, $name:literal) => {
		ASSERT_EQ(
			(*(*$skel).bss).$res as c_long,
			(*(*$skel).rodata).$exp as c_long,
			cstr!($name),
		)
	};
}

unsafe fn subtest_num_iters() {
	let skel: *mut iters_num;
	let err: c_int;

	skel = iters_num__open_and_load();
	if !ASSERT_OK_PTR(skel as *const c_void, cstr!("skel_open_and_load")) {
		return;
	}

	err = iters_num__attach(skel);
	if !ASSERT_OK(err, cstr!("skel_attach")) {
		goto_cleanup_num(skel);
		return;
	}

	usleep(1);
	iters_num__detach(skel);

	validate_num_case!(skel, res_empty_zero, exp_empty_zero, "empty_zero");
	validate_num_case!(skel, res_empty_int_min, exp_empty_int_min, "empty_int_min");
	validate_num_case!(skel, res_empty_int_max, exp_empty_int_max, "empty_int_max");
	validate_num_case!(skel, res_empty_minus_one, exp_empty_minus_one, "empty_minus_one");

	validate_num_case!(skel, res_simple_sum, exp_simple_sum, "simple_sum");
	validate_num_case!(skel, res_neg_sum, exp_neg_sum, "neg_sum");
	validate_num_case!(skel, res_very_neg_sum, exp_very_neg_sum, "very_neg_sum");
	validate_num_case!(skel, res_neg_pos_sum, exp_neg_pos_sum, "neg_pos_sum");

	validate_num_case!(skel, res_invalid_range, exp_invalid_range, "invalid_range");
	validate_num_case!(skel, res_max_range, exp_max_range, "max_range");
	validate_num_case!(skel, res_e2big_range, exp_e2big_range, "e2big_range");

	validate_num_case!(skel, res_succ_elem_cnt, exp_succ_elem_cnt, "succ_elem_cnt");
	validate_num_case!(skel, res_overfetched_elem_cnt, exp_overfetched_elem_cnt, "overfetched_elem_cnt");
	validate_num_case!(skel, res_fail_elem_cnt, exp_fail_elem_cnt, "fail_elem_cnt");

goto_cleanup_num(skel);
}

unsafe fn goto_cleanup_num(skel: *mut iters_num) {
	iters_num__destroy(skel);
}

unsafe fn subtest_testmod_seq_iters() {
	let skel: *mut iters_testmod_seq;
	let err: c_int;

	if !env.has_testmod {
		test__skip();
		return;
	}

	skel = iters_testmod_seq__open_and_load();
	if !ASSERT_OK_PTR(skel as *const c_void, cstr!("skel_open_and_load")) {
		return;
	}

	err = iters_testmod_seq__attach(skel);
	if !ASSERT_OK(err, cstr!("skel_attach")) {
		goto_cleanup_testmod_seq(skel);
		return;
	}

	usleep(1);
	iters_testmod_seq__detach(skel);

	validate_testmod_seq_case!(skel, res_empty, exp_empty, "empty");
	validate_testmod_seq_case!(skel, res_full, exp_full, "full");
	validate_testmod_seq_case!(skel, res_truncated, exp_truncated, "truncated");

	goto_cleanup_testmod_seq(skel);
}

unsafe fn goto_cleanup_testmod_seq(skel: *mut iters_testmod_seq) {
	iters_testmod_seq__destroy(skel);
}

unsafe fn subtest_task_vma_iters() {
	let mut start: c_ulong = 0;
	let mut end: c_ulong = 0;
	let mut bpf_iter_start: c_ulong;
	let mut bpf_iter_end: c_ulong;
	let skel: *mut iters_task_vma;
	let mut rest_of_line = [0 as c_char; 1000];
	let mut seen: c_uint;
	let mut f: *mut FILE = core::ptr::null_mut();
	let err: c_int;

	skel = iters_task_vma__open_and_load();
	if !ASSERT_OK_PTR(skel as *const c_void, cstr!("skel_open_and_load")) {
		return;
	}

	(*(*skel).bss).target_pid = getpid();

	err = iters_task_vma__attach(skel);
	if !ASSERT_OK(err, cstr!("skel_attach")) {
		goto_cleanup_task_vma(f, skel);
		return;
	}

	getpgid((*(*skel).bss).target_pid);
	iters_task_vma__detach(skel);

	if !ASSERT_GT((*(*skel).bss).vmas_seen as c_long, 0, cstr!("vmas_seen_gt_zero")) {
		goto_cleanup_task_vma(f, skel);
		return;
	}

	f = fopen(cstr!("/proc/self/maps"), cstr!("r"));
	if !ASSERT_OK_PTR(f as *const c_void, cstr!("proc_maps_fopen")) {
		goto_cleanup_task_vma(f, skel);
		return;
	}

	seen = 0;
	while fscanf(
		f,
		cstr!("%lx-%lx %[^\n]\n"),
		&mut start as *mut c_ulong,
		&mut end as *mut c_ulong,
		rest_of_line.as_mut_ptr(),
	) == 3
	{
		/* [vsyscall] vma isn't _really_ part of task->mm vmas.
		 * /proc/PID/maps returns it when out of vmas - see get_gate_vma
		 * calls in fs/proc/task_mmu.c
		 */
		if !strstr(rest_of_line.as_ptr(), cstr!("[vsyscall]")).is_null() {
			continue;
		}

		bpf_iter_start = (*(*(*skel).bss).vm_ranges.add(seen as usize)).vm_start;
		bpf_iter_end = (*(*(*skel).bss).vm_ranges.add(seen as usize)).vm_end;

		ASSERT_EQ(bpf_iter_start as c_long, start as c_long, cstr!("vma->vm_start match"));
		ASSERT_EQ(bpf_iter_end as c_long, end as c_long, cstr!("vma->vm_end match"));
		seen = seen.wrapping_add(1);
	}

	if !ASSERT_EQ(
		(*(*skel).bss).vmas_seen as c_long,
		seen as c_long,
		cstr!("vmas_seen_eq"),
	) {
		goto_cleanup_task_vma(f, skel);
		return;
	}

	goto_cleanup_task_vma(f, skel);
}

unsafe fn goto_cleanup_task_vma(f: *mut FILE, skel: *mut iters_task_vma) {
	if !f.is_null() {
		fclose(f);
	}
	iters_task_vma__destroy(skel);
}

static mut do_nothing_mutex: pthread_mutex_t = pthread_mutex_t { _private: [] };

unsafe extern "C" fn do_nothing_wait(arg: *mut c_void) -> *mut c_void {
	pthread_mutex_lock(&mut do_nothing_mutex);
	pthread_mutex_unlock(&mut do_nothing_mutex);

	pthread_exit(arg);
}

unsafe fn subtest_task_iters() {
	let mut skel: *mut iters_task = core::ptr::null_mut();
	let mut thread_ids = [0 as pthread_t; THREAD_NUM];
	let mut ret: *mut c_void = core::ptr::null_mut();
	let err: c_int;

	skel = iters_task__open_and_load();
	if !ASSERT_OK_PTR(skel as *const c_void, cstr!("open_and_load")) {
		goto_cleanup_task(skel);
		return;
	}
	(*(*skel).bss).target_pid = getpid();
	err = iters_task__attach(skel);
	if !ASSERT_OK(err, cstr!("iters_task__attach")) {
		goto_cleanup_task(skel);
		return;
	}
	pthread_mutex_lock(&mut do_nothing_mutex);
	for i in 0..THREAD_NUM {
		ASSERT_OK(
			pthread_create(
				&mut thread_ids[i],
				core::ptr::null(),
				Some(do_nothing_wait),
				core::ptr::null_mut(),
			),
			cstr!("pthread_create"),
		);
	}

	syscall(SYS_getpgid);
	iters_task__detach(skel);
	ASSERT_EQ((*(*skel).bss).procs_cnt as c_long, 1, cstr!("procs_cnt"));
	ASSERT_EQ(
		(*(*skel).bss).threads_cnt as c_long,
		(THREAD_NUM + 2) as c_long,
		cstr!("threads_cnt"),
	);
	ASSERT_EQ(
		(*(*skel).bss).proc_threads_cnt as c_long,
		(THREAD_NUM + 2) as c_long,
		cstr!("proc_threads_cnt"),
	);
	ASSERT_EQ((*(*skel).bss).invalid_cnt as c_long, 0, cstr!("invalid_cnt"));
	pthread_mutex_unlock(&mut do_nothing_mutex);
	for i in 0..THREAD_NUM {
		ASSERT_OK(pthread_join(thread_ids[i], &mut ret), cstr!("pthread_join"));
	}
	goto_cleanup_task(skel);
}

unsafe fn goto_cleanup_task(skel: *mut iters_task) {
	iters_task__destroy(skel);
}

unsafe fn subtest_css_task_iters() {
	let mut skel: *mut iters_css_task = core::ptr::null_mut();
	let mut err: c_int;
	let cg_fd: c_int;
	let cg_id: c_int;
	let cgrp_path = cstr!("/cg1");

	err = setup_cgroup_environment();
	if !ASSERT_OK(err, cstr!("setup_cgroup_environment")) {
		goto_cleanup_css_task(skel);
		return;
	}
	cg_fd = create_and_get_cgroup(cgrp_path);
	if !ASSERT_GE(cg_fd as c_long, 0, cstr!("create_and_get_cgroup")) {
		goto_cleanup_css_task(skel);
		return;
	}
	cg_id = get_cgroup_id(cgrp_path);
	err = join_cgroup(cgrp_path);
	if !ASSERT_OK(err, cstr!("join_cgroup")) {
		goto_cleanup_css_task(skel);
		return;
	}

	skel = iters_css_task__open_and_load();
	if !ASSERT_OK_PTR(skel as *const c_void, cstr!("open_and_load")) {
		goto_cleanup_css_task(skel);
		return;
	}

	(*(*skel).bss).target_pid = getpid();
	(*(*skel).bss).cg_id = cg_id;
	err = iters_css_task__attach(skel);
	if !ASSERT_OK(err, cstr!("iters_task__attach")) {
		goto_cleanup_css_task(skel);
		return;
	}
	err = stack_mprotect();
	if !ASSERT_EQ(err as c_long, -1, cstr!("stack_mprotect"))
		|| !ASSERT_EQ(errno as c_long, EPERM as c_long, cstr!("stack_mprotect"))
	{
		goto_cleanup_css_task(skel);
		return;
	}
	iters_css_task__detach(skel);
	ASSERT_EQ((*(*skel).bss).css_task_cnt as c_long, 1, cstr!("css_task_cnt"));

	goto_cleanup_css_task(skel);
}

unsafe fn goto_cleanup_css_task(skel: *mut iters_css_task) {
	cleanup_cgroup_environment();
	iters_css_task__destroy(skel);
}

unsafe fn subtest_css_iters() {
	let mut skel: *mut iters_css = core::ptr::null_mut();
	let mut cgs = [
		cgroup { path: cstr!("/cg1"), fd: 0 },
		cgroup { path: cstr!("/cg1/cg2"), fd: 0 },
		cgroup { path: cstr!("/cg1/cg2/cg3"), fd: 0 },
		cgroup { path: cstr!("/cg1/cg2/cg3/cg4"), fd: 0 },
		cgroup { path: cstr!("/cg1/cg5"), fd: 0 },
		cgroup { path: cstr!("/cg1/cg5/cg6"), fd: 0 },
		cgroup { path: cstr!("/cg1/cg7"), fd: 0 },
		cgroup { path: cstr!("/cg1/cg7/cg8"), fd: 0 },
		cgroup { path: cstr!("/cg1/cg7/cg8/cg9"), fd: 0 },
	];
	let mut err: c_int;
	let cg_nr: c_int = cgs.len() as c_int;
	let mut i: c_int;

	err = setup_cgroup_environment();
	if !ASSERT_OK(err, cstr!("setup_cgroup_environment")) {
		goto_cleanup_css(skel);
		return;
	}
	i = 0;
	while i < cg_nr {
		cgs[i as usize].fd = create_and_get_cgroup(cgs[i as usize].path);
		if !ASSERT_GE(cgs[i as usize].fd as c_long, 0, cstr!("create_and_get_cgroup")) {
			goto_cleanup_css(skel);
			return;
		}
		i += 1;
	}

	skel = iters_css__open_and_load();
	if !ASSERT_OK_PTR(skel as *const c_void, cstr!("open_and_load")) {
		goto_cleanup_css(skel);
		return;
	}

	(*(*skel).bss).target_pid = getpid();
	(*(*skel).bss).root_cg_id = get_cgroup_id(cgs[0].path);
	(*(*skel).bss).leaf_cg_id = get_cgroup_id(cgs[(cg_nr - 1) as usize].path);
	err = iters_css__attach(skel);

	if !ASSERT_OK(err, cstr!("iters_task__attach")) {
		goto_cleanup_css(skel);
		return;
	}

	syscall(SYS_getpgid);
	ASSERT_EQ((*(*skel).bss).pre_order_cnt as c_long, cg_nr as c_long, cstr!("pre_order_cnt"));
	ASSERT_EQ(
		(*(*skel).bss).first_cg_id as c_long,
		get_cgroup_id(cgs[0].path) as c_long,
		cstr!("first_cg_id"),
	);

	ASSERT_EQ((*(*skel).bss).post_order_cnt as c_long, cg_nr as c_long, cstr!("post_order_cnt"));
	ASSERT_EQ(
		(*(*skel).bss).last_cg_id as c_long,
		get_cgroup_id(cgs[0].path) as c_long,
		cstr!("last_cg_id"),
	);
	ASSERT_EQ((*(*skel).bss).children_cnt as c_long, 3, cstr!("children_cnt"));
	ASSERT_EQ((*(*skel).bss).tree_high as c_long, 3, cstr!("tree_high"));
	iters_css__detach(skel);
	goto_cleanup_css(skel);
}

unsafe fn goto_cleanup_css(skel: *mut iters_css) {
	cleanup_cgroup_environment();
	iters_css__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_iters() {
	RUN_TESTS(cstr!("iters_state_safety"));
	RUN_TESTS(cstr!("iters_looping"));
	RUN_TESTS(cstr!("iters"));
	RUN_TESTS(cstr!("iters_css_task"));

	if env.has_testmod {
		RUN_TESTS(cstr!("iters_testmod"));
		RUN_TESTS(cstr!("iters_testmod_seq"));
	}

	if test__start_subtest(cstr!("num")) {
		subtest_num_iters();
	}
	if test__start_subtest(cstr!("testmod_seq")) {
		subtest_testmod_seq_iters();
	}
	if test__start_subtest(cstr!("task_vma")) {
		subtest_task_vma_iters();
	}
	if test__start_subtest(cstr!("task")) {
		subtest_task_iters();
	}
	if test__start_subtest(cstr!("css_task")) {
		subtest_css_task_iters();
	}
	if test__start_subtest(cstr!("css")) {
		subtest_css_iters();
	}
	RUN_TESTS(cstr!("iters_task_failure"));
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
