// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Google */

// Translated from testing/selftests/bpf/prog_tests/cgroup_iter.c.
// External declarations correspond to test_progs.h, libbpf, BTF, skeletons,
// and cgroup helper symbols included by the original C source.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_ulonglong, c_void};
use core::mem;
use core::ptr;

const ROOT: usize = 0;
const PARENT: usize = 1;
const CHILD1: usize = 2;
const CHILD2: usize = 3;
const NUM_CGROUPS: usize = 4;

const PROLOGUE: &[u8] = b"prologue\n\0";
const EPILOGUE: &[u8] = b"epilogue\n\0";

const BPF_CGROUP_ITER_DESCENDANTS_PRE: c_int = 0;
const BPF_CGROUP_ITER_DESCENDANTS_POST: c_int = 1;
const BPF_CGROUP_ITER_ANCESTORS_UP: c_int = 2;
const BPF_CGROUP_ITER_SELF_ONLY: c_int = 3;
const BPF_CGROUP_ITER_CHILDREN: c_int = 4;

#[repr(C)]
pub struct bpf_program {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bpf_iter_attach_opts {
	pub sz: usize,
	pub link_info: *mut bpf_iter_link_info,
	pub link_info_len: c_uint,
}

#[repr(C)]
pub struct bpf_iter_cgroup {
	pub cgroup_fd: c_uint,
	pub cgroup_id: u64,
	pub order: c_uint,
}

#[repr(C)]
pub union bpf_iter_link_info {
	pub cgroup: bpf_iter_cgroup,
}

#[repr(C)]
pub struct cgroup_iter_bss {
	pub terminal_cgroup: c_ulonglong,
	pub terminate_early: c_int,
}

#[repr(C)]
pub struct cgroup_iter_progs {
	pub cgroup_id_printer: *mut bpf_program,
}

#[repr(C)]
pub struct cgroup_iter {
	pub progs: cgroup_iter_progs,
	pub bss: *mut cgroup_iter_bss,
}

#[repr(C)]
pub struct iters_css_task_bss {
	pub target_pid: c_int,
	pub css_task_cnt: c_int,
}

#[repr(C)]
pub struct iters_css_task_progs {
	pub cgroup_id_printer: *mut bpf_program,
}

#[repr(C)]
pub struct iters_css_task {
	pub progs: iters_css_task_progs,
	pub bss: *mut iters_css_task_bss,
}

static mut CG_PATH: [*const c_char; NUM_CGROUPS] = [
	b"/\0".as_ptr() as *const c_char,
	b"/parent\0".as_ptr() as *const c_char,
	b"/parent/child1\0".as_ptr() as *const c_char,
	b"/parent/child2\0".as_ptr() as *const c_char,
];

static mut CG_FD: [c_int; NUM_CGROUPS] = [-1, -1, -1, -1];
static mut CG_ID: [c_ulonglong; NUM_CGROUPS] = [0, 0, 0, 0];
static mut EXPECTED_OUTPUT: [c_char; 64] = [0; 64];

unsafe extern "C" {
	fn create_and_get_cgroup(path: *const c_char) -> c_int;
	fn get_cgroup_id(path: *const c_char) -> c_ulonglong;
	fn remove_cgroup(path: *const c_char);
	fn setup_cgroup_environment() -> c_int;
	fn cleanup_cgroup_environment();
	fn cleanup_cgroups();
	fn join_cgroup(path: *const c_char) -> c_int;
	fn kern_sync_rcu();

	fn close(fd: c_int) -> c_int;
	fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
	fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
	fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
	fn usleep(usec: c_uint) -> c_int;
	fn getpid() -> c_int;

	fn bpf_program__attach_iter(
		prog: *mut bpf_program,
		opts: *const bpf_iter_attach_opts,
	) -> *mut bpf_link;
	fn bpf_iter_create(link_fd: c_int) -> c_int;
	fn bpf_link__fd(link: *mut bpf_link) -> c_int;
	fn bpf_link__destroy(link: *mut bpf_link);
	fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);

	fn cgroup_iter__open_and_load() -> *mut cgroup_iter;
	fn cgroup_iter__destroy(obj: *mut cgroup_iter);
	fn iters_css_task__open() -> *mut iters_css_task;
	fn iters_css_task__load(obj: *mut iters_css_task) -> c_int;
	fn iters_css_task__destroy(obj: *mut iters_css_task);

	fn ASSERT_OK_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
	fn ASSERT_ERR_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
	fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
	fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
	fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
	fn ASSERT_STREQ(actual: *const c_char, expected: *const c_char, name: *const c_char) -> bool;
	fn test__start_subtest(name: *const c_char) -> bool;
}

unsafe fn setup_cgroups_local() -> c_int {
	let mut fd: c_int;
	let mut i: usize = 0;

	while i < NUM_CGROUPS {
		fd = create_and_get_cgroup(CG_PATH[i]);
		if fd < 0 {
			return fd;
		}

		CG_FD[i] = fd;
		CG_ID[i] = get_cgroup_id(CG_PATH[i]);
		i += 1;
	}
	0
}

unsafe fn cleanup_cgroups_local() {
	let mut i: usize = 0;

	while i < NUM_CGROUPS {
		close(CG_FD[i]);
		i += 1;
	}
}

unsafe fn read_from_cgroup_iter(
	prog: *mut bpf_program,
	cgroup_fd: c_int,
	order: c_int,
	testname: *const c_char,
) {
	let mut opts = bpf_iter_attach_opts {
		sz: mem::size_of::<bpf_iter_attach_opts>(),
		link_info: ptr::null_mut(),
		link_info_len: 0,
	};
	let mut linfo: bpf_iter_link_info = mem::zeroed();
	let link: *mut bpf_link;
	let mut len: isize;
	let iter_fd: c_int;
	static mut BUF: [c_char; 128] = [0; 128];
	let mut left: usize;
	let mut p: *mut c_char;

	memset(
		&mut linfo as *mut _ as *mut c_void,
		0,
		mem::size_of::<bpf_iter_link_info>(),
	);
	linfo.cgroup.cgroup_fd = cgroup_fd as c_uint;
	linfo.cgroup.order = order as c_uint;
	opts.link_info = &mut linfo;
	opts.link_info_len = mem::size_of::<bpf_iter_link_info>() as c_uint;

	link = bpf_program__attach_iter(prog, &opts);
	if !ASSERT_OK_PTR(link as *mut c_void, b"attach_iter\0".as_ptr() as *const c_char) {
		return;
	}

	iter_fd = bpf_iter_create(bpf_link__fd(link));
	if iter_fd < 0 {
		goto_free_link(link);
		return;
	}

	memset(BUF.as_mut_ptr() as *mut c_void, 0, mem::size_of_val(&BUF));
	left = BUF.len();
	p = BUF.as_mut_ptr();
	loop {
		len = read(iter_fd, p as *mut c_void, left);
		if len <= 0 {
			break;
		}
		p = p.add(len as usize);
		left -= len as usize;
	}

	ASSERT_STREQ(BUF.as_ptr(), EXPECTED_OUTPUT.as_ptr(), testname);

	/* read() after iter finishes should be ok. */
	if len == 0 {
		ASSERT_OK(
			read(iter_fd, BUF.as_mut_ptr() as *mut c_void, mem::size_of_val(&BUF)) as c_int,
			b"second_read\0".as_ptr() as *const c_char,
		);
	}

	close(iter_fd);
	goto_free_link(link);
}

unsafe fn goto_free_link(link: *mut bpf_link) {
	bpf_link__destroy(link);
}

/* Invalid cgroup. */
unsafe fn test_invalid_cgroup(skel: *mut cgroup_iter) {
	let mut opts = bpf_iter_attach_opts {
		sz: mem::size_of::<bpf_iter_attach_opts>(),
		link_info: ptr::null_mut(),
		link_info_len: 0,
	};
	let mut linfo: bpf_iter_link_info = mem::zeroed();
	let link: *mut bpf_link;

	memset(
		&mut linfo as *mut _ as *mut c_void,
		0,
		mem::size_of::<bpf_iter_link_info>(),
	);
	linfo.cgroup.cgroup_fd = -1i32 as c_uint;
	opts.link_info = &mut linfo;
	opts.link_info_len = mem::size_of::<bpf_iter_link_info>() as c_uint;

	link = bpf_program__attach_iter((*skel).progs.cgroup_id_printer, &opts);
	ASSERT_ERR_PTR(link as *mut c_void, b"attach_iter\0".as_ptr() as *const c_char);
	bpf_link__destroy(link);
}

/* Specifying both cgroup_fd and cgroup_id is invalid. */
unsafe fn test_invalid_cgroup_spec(skel: *mut cgroup_iter) {
	let mut opts = bpf_iter_attach_opts {
		sz: mem::size_of::<bpf_iter_attach_opts>(),
		link_info: ptr::null_mut(),
		link_info_len: 0,
	};
	let mut linfo: bpf_iter_link_info = mem::zeroed();
	let link: *mut bpf_link;

	memset(
		&mut linfo as *mut _ as *mut c_void,
		0,
		mem::size_of::<bpf_iter_link_info>(),
	);
	linfo.cgroup.cgroup_fd = CG_FD[PARENT] as c_uint;
	linfo.cgroup.cgroup_id = CG_ID[PARENT] as u64;
	opts.link_info = &mut linfo;
	opts.link_info_len = mem::size_of::<bpf_iter_link_info>() as c_uint;

	link = bpf_program__attach_iter((*skel).progs.cgroup_id_printer, &opts);
	ASSERT_ERR_PTR(link as *mut c_void, b"attach_iter\0".as_ptr() as *const c_char);
	bpf_link__destroy(link);
}

/* Preorder walk prints parent and child in order. */
unsafe fn test_walk_preorder(skel: *mut cgroup_iter) {
	snprintf(
		EXPECTED_OUTPUT.as_mut_ptr(),
		mem::size_of_val(&EXPECTED_OUTPUT),
		b"%s%8llu\n%8llu\n%8llu\n%s\0".as_ptr() as *const c_char,
		PROLOGUE.as_ptr() as *const c_char,
		CG_ID[PARENT],
		CG_ID[CHILD1],
		CG_ID[CHILD2],
		EPILOGUE.as_ptr() as *const c_char,
	);

	read_from_cgroup_iter(
		(*skel).progs.cgroup_id_printer,
		CG_FD[PARENT],
		BPF_CGROUP_ITER_DESCENDANTS_PRE,
		b"preorder\0".as_ptr() as *const c_char,
	);
}

/* Postorder walk prints child and parent in order. */
unsafe fn test_walk_postorder(skel: *mut cgroup_iter) {
	snprintf(
		EXPECTED_OUTPUT.as_mut_ptr(),
		mem::size_of_val(&EXPECTED_OUTPUT),
		b"%s%8llu\n%8llu\n%8llu\n%s\0".as_ptr() as *const c_char,
		PROLOGUE.as_ptr() as *const c_char,
		CG_ID[CHILD1],
		CG_ID[CHILD2],
		CG_ID[PARENT],
		EPILOGUE.as_ptr() as *const c_char,
	);

	read_from_cgroup_iter(
		(*skel).progs.cgroup_id_printer,
		CG_FD[PARENT],
		BPF_CGROUP_ITER_DESCENDANTS_POST,
		b"postorder\0".as_ptr() as *const c_char,
	);
}

/* Walking parents prints parent and then root. */
unsafe fn test_walk_ancestors_up(skel: *mut cgroup_iter) {
	/* terminate the walk when ROOT is met. */
	(*(*skel).bss).terminal_cgroup = CG_ID[ROOT];

	snprintf(
		EXPECTED_OUTPUT.as_mut_ptr(),
		mem::size_of_val(&EXPECTED_OUTPUT),
		b"%s%8llu\n%8llu\n%s\0".as_ptr() as *const c_char,
		PROLOGUE.as_ptr() as *const c_char,
		CG_ID[PARENT],
		CG_ID[ROOT],
		EPILOGUE.as_ptr() as *const c_char,
	);

	read_from_cgroup_iter(
		(*skel).progs.cgroup_id_printer,
		CG_FD[PARENT],
		BPF_CGROUP_ITER_ANCESTORS_UP,
		b"ancestors_up\0".as_ptr() as *const c_char,
	);

	(*(*skel).bss).terminal_cgroup = 0;
}

/* Early termination prints parent only. */
unsafe fn test_early_termination(skel: *mut cgroup_iter) {
	/* terminate the walk after the first element is processed. */
	(*(*skel).bss).terminate_early = 1;

	snprintf(
		EXPECTED_OUTPUT.as_mut_ptr(),
		mem::size_of_val(&EXPECTED_OUTPUT),
		b"%s%8llu\n%s\0".as_ptr() as *const c_char,
		PROLOGUE.as_ptr() as *const c_char,
		CG_ID[PARENT],
		EPILOGUE.as_ptr() as *const c_char,
	);

	read_from_cgroup_iter(
		(*skel).progs.cgroup_id_printer,
		CG_FD[PARENT],
		BPF_CGROUP_ITER_DESCENDANTS_PRE,
		b"early_termination\0".as_ptr() as *const c_char,
	);

	(*(*skel).bss).terminate_early = 0;
}

/* Waling self prints self only. */
unsafe fn test_walk_self_only(skel: *mut cgroup_iter) {
	snprintf(
		EXPECTED_OUTPUT.as_mut_ptr(),
		mem::size_of_val(&EXPECTED_OUTPUT),
		b"%s%8llu\n%s\0".as_ptr() as *const c_char,
		PROLOGUE.as_ptr() as *const c_char,
		CG_ID[PARENT],
		EPILOGUE.as_ptr() as *const c_char,
	);

	read_from_cgroup_iter(
		(*skel).progs.cgroup_id_printer,
		CG_FD[PARENT],
		BPF_CGROUP_ITER_SELF_ONLY,
		b"self_only\0".as_ptr() as *const c_char,
	);
}

unsafe fn test_walk_children(skel: *mut cgroup_iter) {
	snprintf(
		EXPECTED_OUTPUT.as_mut_ptr(),
		mem::size_of_val(&EXPECTED_OUTPUT),
		b"%s%8llu\n%8llu\n%s\0".as_ptr() as *const c_char,
		PROLOGUE.as_ptr() as *const c_char,
		CG_ID[CHILD1],
		CG_ID[CHILD2],
		EPILOGUE.as_ptr() as *const c_char,
	);

	read_from_cgroup_iter(
		(*skel).progs.cgroup_id_printer,
		CG_FD[PARENT],
		BPF_CGROUP_ITER_CHILDREN,
		b"children\0".as_ptr() as *const c_char,
	);
}

unsafe fn test_walk_dead_self_only(skel: *mut cgroup_iter) {
	let mut opts = bpf_iter_attach_opts {
		sz: mem::size_of::<bpf_iter_attach_opts>(),
		link_info: ptr::null_mut(),
		link_info_len: 0,
	};
	let mut expected_output: [c_char; 128] = [0; 128];
	let mut buf: [c_char; 128] = [0; 128];
	let cgrp_name: *const c_char = b"/dead\0".as_ptr() as *const c_char;
	let mut linfo: bpf_iter_link_info = mem::zeroed();
	let mut len: isize;
	let cgrp_fd: c_int;
	let iter_fd: c_int;
	let link: *mut bpf_link;
	let mut left: usize;
	let mut p: *mut c_char;

	cgrp_fd = create_and_get_cgroup(cgrp_name);
	if !ASSERT_GE(cgrp_fd, 0, b"create cgrp\0".as_ptr() as *const c_char) {
		return;
	}

	/* The cgroup will be dead during read() iteration, so it only has
	 * epilogue in the output
	 */
	snprintf(
		expected_output.as_mut_ptr(),
		mem::size_of_val(&expected_output),
		b"%s\0".as_ptr() as *const c_char,
		EPILOGUE.as_ptr() as *const c_char,
	);

	memset(
		&mut linfo as *mut _ as *mut c_void,
		0,
		mem::size_of::<bpf_iter_link_info>(),
	);
	linfo.cgroup.cgroup_fd = cgrp_fd as c_uint;
	linfo.cgroup.order = BPF_CGROUP_ITER_SELF_ONLY as c_uint;
	opts.link_info = &mut linfo;
	opts.link_info_len = mem::size_of::<bpf_iter_link_info>() as c_uint;

	link = bpf_program__attach_iter((*skel).progs.cgroup_id_printer, &opts);
	if !ASSERT_OK_PTR(link as *mut c_void, b"attach_iter\0".as_ptr() as *const c_char) {
		close(cgrp_fd);
		return;
	}

	iter_fd = bpf_iter_create(bpf_link__fd(link));
	if !ASSERT_GE(iter_fd, 0, b"iter_create\0".as_ptr() as *const c_char) {
		bpf_link__destroy(link);
		close(cgrp_fd);
		return;
	}

	/* Close link fd and cgroup fd */
	bpf_link__destroy(link);
	close(cgrp_fd);

	/* Remove cgroup to mark it as dead */
	remove_cgroup(cgrp_name);

	/* Two kern_sync_rcu() and usleep() pairs are used to wait for the
	 * releases of cgroup css, and the last kern_sync_rcu() and usleep()
	 * pair is used to wait for the free of cgroup itself.
	 */
	kern_sync_rcu();
	usleep(8000);
	kern_sync_rcu();
	usleep(8000);
	kern_sync_rcu();
	usleep(1000);

	memset(buf.as_mut_ptr() as *mut c_void, 0, mem::size_of_val(&buf));
	left = buf.len();
	p = buf.as_mut_ptr();
	loop {
		len = read(iter_fd, p as *mut c_void, left);
		if len <= 0 {
			break;
		}
		p = p.add(len as usize);
		left -= len as usize;
	}

	ASSERT_STREQ(
		buf.as_ptr(),
		expected_output.as_ptr(),
		b"dead cgroup output\0".as_ptr() as *const c_char,
	);

	/* read() after iter finishes should be ok. */
	if len == 0 {
		ASSERT_OK(
			read(iter_fd, buf.as_mut_ptr() as *mut c_void, mem::size_of_val(&buf)) as c_int,
			b"second_read\0".as_ptr() as *const c_char,
		);
	}

	close(iter_fd);
}

unsafe fn test_walk_self_only_css_task() {
	let skel: *mut iters_css_task;
	let mut err: c_int;

	skel = iters_css_task__open();
	if !ASSERT_OK_PTR(skel as *mut c_void, b"skel_open\0".as_ptr() as *const c_char) {
		return;
	}

	bpf_program__set_autoload((*skel).progs.cgroup_id_printer, true);

	err = iters_css_task__load(skel);
	if !ASSERT_OK(err, b"skel_load\0".as_ptr() as *const c_char) {
		iters_css_task__destroy(skel);
		return;
	}

	err = join_cgroup(CG_PATH[CHILD2]);
	if !ASSERT_OK(err, b"join_cgroup\0".as_ptr() as *const c_char) {
		iters_css_task__destroy(skel);
		return;
	}

	(*(*skel).bss).target_pid = getpid();
	snprintf(
		EXPECTED_OUTPUT.as_mut_ptr(),
		mem::size_of_val(&EXPECTED_OUTPUT),
		b"%s%8llu\n%s\0".as_ptr() as *const c_char,
		PROLOGUE.as_ptr() as *const c_char,
		CG_ID[CHILD2],
		EPILOGUE.as_ptr() as *const c_char,
	);
	read_from_cgroup_iter(
		(*skel).progs.cgroup_id_printer,
		CG_FD[CHILD2],
		BPF_CGROUP_ITER_SELF_ONLY,
		b"test_walk_self_only_css_task\0".as_ptr() as *const c_char,
	);
	ASSERT_EQ((*(*skel).bss).css_task_cnt, 1, b"css_task_cnt\0".as_ptr() as *const c_char);
	iters_css_task__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_cgroup_iter() {
	let mut skel: *mut cgroup_iter = ptr::null_mut();

	if setup_cgroup_environment() != 0 {
		return;
	}

	if setup_cgroups_local() != 0 {
		cgroup_iter__destroy(skel);
		cleanup_cgroups_local();
		cleanup_cgroup_environment();
		return;
	}

	skel = cgroup_iter__open_and_load();
	if !ASSERT_OK_PTR(
		skel as *mut c_void,
		b"cgroup_iter__open_and_load\0".as_ptr() as *const c_char,
	) {
		cgroup_iter__destroy(skel);
		cleanup_cgroups_local();
		cleanup_cgroup_environment();
		return;
	}

	if test__start_subtest(b"cgroup_iter__invalid_cgroup\0".as_ptr() as *const c_char) {
		test_invalid_cgroup(skel);
	}
	if test__start_subtest(b"cgroup_iter__invalid_cgroup_spec\0".as_ptr() as *const c_char) {
		test_invalid_cgroup_spec(skel);
	}
	if test__start_subtest(b"cgroup_iter__preorder\0".as_ptr() as *const c_char) {
		test_walk_preorder(skel);
	}
	if test__start_subtest(b"cgroup_iter__postorder\0".as_ptr() as *const c_char) {
		test_walk_postorder(skel);
	}
	if test__start_subtest(b"cgroup_iter__ancestors_up_walk\0".as_ptr() as *const c_char) {
		test_walk_ancestors_up(skel);
	}
	if test__start_subtest(b"cgroup_iter__early_termination\0".as_ptr() as *const c_char) {
		test_early_termination(skel);
	}
	if test__start_subtest(b"cgroup_iter__self_only\0".as_ptr() as *const c_char) {
		test_walk_self_only(skel);
	}
	if test__start_subtest(b"cgroup_iter__dead_self_only\0".as_ptr() as *const c_char) {
		test_walk_dead_self_only(skel);
	}
	if test__start_subtest(b"cgroup_iter__self_only_css_task\0".as_ptr() as *const c_char) {
		test_walk_self_only_css_task();
	}
	if test__start_subtest(b"cgroup_iter__children\0".as_ptr() as *const c_char) {
		test_walk_children(skel);
	}

	cgroup_iter__destroy(skel);
	cleanup_cgroups_local();
	cleanup_cgroup_environment();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
