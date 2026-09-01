// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2023 Yafang Shao <laoar.shao@gmail.com> */

use core::ffi::{c_char, c_int, c_void};

type __u64 = u64;

#[repr(C)]
pub struct bpf_link {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
	_private: [u8; 0],
}

#[repr(C)]
pub struct test_cgroup1_hierarchy {
	pub progs: test_cgroup1_hierarchy__progs,
	pub bss: *mut test_cgroup1_hierarchy__bss,
}

#[repr(C)]
pub struct test_cgroup1_hierarchy__progs {
	pub lsm_run: *mut bpf_program,
	pub fentry_run: *mut bpf_program,
	pub lsm_s_run: *mut bpf_program,
}

#[repr(C)]
pub struct test_cgroup1_hierarchy__bss {
	pub target_pid: c_int,
	pub target_hid: c_int,
	pub target_ancestor_cgid: __u64,
	pub target_ancestor_level: c_int,
}

unsafe extern "C" {
	fn getpid() -> c_int;

	fn bpf_program__attach_lsm(prog: *mut bpf_program) -> *mut bpf_link;
	fn bpf_program__attach_trace(prog: *mut bpf_program) -> *mut bpf_link;
	fn bpf_link__destroy(link: *mut bpf_link) -> c_int;
	fn bpf_program__set_attach_target(
		prog: *mut bpf_program,
		attach_prog_fd: c_int,
		attach_func_name: *const c_char,
	) -> c_int;

	fn test_cgroup1_hierarchy__open() -> *mut test_cgroup1_hierarchy;
	fn test_cgroup1_hierarchy__load(skel: *mut test_cgroup1_hierarchy) -> c_int;
	fn test_cgroup1_hierarchy__destroy(skel: *mut test_cgroup1_hierarchy);

	fn setup_cgroup_environment() -> c_int;
	fn cleanup_cgroup_environment();
	fn setup_classid_environment() -> c_int;
	fn cleanup_classid_environment();
	fn join_classid() -> c_int;
	fn get_classid_cgroup_id() -> __u64;
	fn get_cgroup1_hierarchy_id(name: *const c_char) -> c_int;

	fn test__start_subtest(name: *const c_char) -> bool;

	fn ASSERT_OK_PTR_impl(ptr: *mut c_void, name: *const c_char) -> bool;
	fn ASSERT_NULL_impl(ptr: *mut c_void, name: *const c_char) -> bool;
	fn ASSERT_OK_impl(err: c_int, name: *const c_char) -> bool;
	fn ASSERT_GE_impl(actual: i128, expected: i128, name: *const c_char) -> bool;
}

macro_rules! c_str {
	($s:literal) => {
		concat!($s, "\0").as_ptr() as *const c_char
	};
}

macro_rules! ASSERT_OK_PTR {
	($ptr:expr, $name:literal) => {
		unsafe { ASSERT_OK_PTR_impl($ptr as *mut c_void, c_str!($name)) }
	};
}

macro_rules! ASSERT_NULL {
	($ptr:expr, $name:literal) => {
		unsafe { ASSERT_NULL_impl($ptr as *mut c_void, c_str!($name)) }
	};
}

macro_rules! ASSERT_OK {
	($err:expr, $name:literal) => {
		unsafe { ASSERT_OK_impl($err as c_int, c_str!($name)) }
	};
}

macro_rules! ASSERT_GE {
	($actual:expr, $expected:expr, $name:literal) => {
		unsafe { ASSERT_GE_impl($actual as i128, $expected as i128, c_str!($name)) }
	};
}

unsafe fn bpf_cgroup1(skel: *mut test_cgroup1_hierarchy) {
	let lsm_link: *mut bpf_link;
	let fentry_link: *mut bpf_link;
	let mut err: c_int;

	/* Attach LSM prog first */
	lsm_link = bpf_program__attach_lsm((*skel).progs.lsm_run);
	if !ASSERT_OK_PTR!(lsm_link, "lsm_attach") {
		return;
	}

	/* LSM prog will be triggered when attaching fentry */
	fentry_link = bpf_program__attach_trace((*skel).progs.fentry_run);
	ASSERT_NULL!(fentry_link, "fentry_attach_fail");

	err = bpf_link__destroy(lsm_link);
	ASSERT_OK!(err, "destroy_lsm");
}

unsafe fn bpf_cgroup1_sleepable(skel: *mut test_cgroup1_hierarchy) {
	let lsm_link: *mut bpf_link;
	let fentry_link: *mut bpf_link;
	let mut err: c_int;

	/* Attach LSM prog first */
	lsm_link = bpf_program__attach_lsm((*skel).progs.lsm_s_run);
	if !ASSERT_OK_PTR!(lsm_link, "lsm_attach") {
		return;
	}

	/* LSM prog will be triggered when attaching fentry */
	fentry_link = bpf_program__attach_trace((*skel).progs.fentry_run);
	ASSERT_NULL!(fentry_link, "fentry_attach_fail");

	err = bpf_link__destroy(lsm_link);
	ASSERT_OK!(err, "destroy_lsm");
}

unsafe fn bpf_cgroup1_invalid_id(skel: *mut test_cgroup1_hierarchy) {
	let lsm_link: *mut bpf_link;
	let fentry_link: *mut bpf_link;
	let mut err: c_int;

	/* Attach LSM prog first */
	lsm_link = bpf_program__attach_lsm((*skel).progs.lsm_run);
	if !ASSERT_OK_PTR!(lsm_link, "lsm_attach") {
		return;
	}

	/* LSM prog will be triggered when attaching fentry */
	fentry_link = bpf_program__attach_trace((*skel).progs.fentry_run);
	if !ASSERT_OK_PTR!(fentry_link, "fentry_attach_success") {
		goto_invalid_id_cleanup(skel, lsm_link);
		return;
	}

	err = bpf_link__destroy(fentry_link);
	ASSERT_OK!(err, "destroy_lsm");

	goto_invalid_id_cleanup(skel, lsm_link);
}

unsafe fn goto_invalid_id_cleanup(_skel: *mut test_cgroup1_hierarchy, lsm_link: *mut bpf_link) {
	let err: c_int;

	err = bpf_link__destroy(lsm_link);
	ASSERT_OK!(err, "destroy_fentry");
}

#[no_mangle]
pub unsafe extern "C" fn test_cgroup1_hierarchy() {
	let skel: *mut test_cgroup1_hierarchy;
	let current_cgid: __u64;
	let mut hid: c_int;
	let mut err: c_int;

	skel = test_cgroup1_hierarchy__open();
	if !ASSERT_OK_PTR!(skel, "open") {
		return;
	}

	(*(*skel).bss).target_pid = getpid();

	err = bpf_program__set_attach_target((*skel).progs.fentry_run, 0, c_str!("bpf_fentry_test1"));
	if !ASSERT_OK!(err, "fentry_set_target") {
		goto_destroy(skel);
		return;
	}

	err = test_cgroup1_hierarchy__load(skel);
	if !ASSERT_OK!(err, "load") {
		goto_destroy(skel);
		return;
	}

	/* Setup cgroup1 hierarchy */
	err = setup_cgroup_environment();
	if !ASSERT_OK!(err, "setup_cgroup_environment") {
		goto_destroy(skel);
		return;
	}
	err = setup_classid_environment();
	if !ASSERT_OK!(err, "setup_classid_environment") {
		goto_cleanup_cgroup(skel);
		return;
	}

	err = join_classid();
	if !ASSERT_OK!(err, "join_cgroup1") {
		goto_cleanup(skel);
		return;
	}

	current_cgid = get_classid_cgroup_id();
	if !ASSERT_GE!(current_cgid, 0, "cgroup1 id") {
		goto_cleanup(skel);
		return;
	}

	hid = get_cgroup1_hierarchy_id(c_str!("net_cls"));
	if !ASSERT_GE!(hid, 0, "cgroup1 id") {
		goto_cleanup(skel);
		return;
	}
	(*(*skel).bss).target_hid = hid;

	if test__start_subtest(c_str!("test_cgroup1_hierarchy")) {
		(*(*skel).bss).target_ancestor_cgid = current_cgid;
		bpf_cgroup1(skel);
	}

	if test__start_subtest(c_str!("test_root_cgid")) {
		(*(*skel).bss).target_ancestor_cgid = 1;
		(*(*skel).bss).target_ancestor_level = 0;
		bpf_cgroup1(skel);
	}

	if test__start_subtest(c_str!("test_invalid_level")) {
		(*(*skel).bss).target_ancestor_cgid = 1;
		(*(*skel).bss).target_ancestor_level = 1;
		bpf_cgroup1_invalid_id(skel);
	}

	if test__start_subtest(c_str!("test_invalid_cgid")) {
		(*(*skel).bss).target_ancestor_cgid = 0;
		bpf_cgroup1_invalid_id(skel);
	}

	if test__start_subtest(c_str!("test_invalid_hid")) {
		(*(*skel).bss).target_ancestor_cgid = 1;
		(*(*skel).bss).target_ancestor_level = 0;
		(*(*skel).bss).target_hid = -1;
		bpf_cgroup1_invalid_id(skel);
	}

	if test__start_subtest(c_str!("test_invalid_cgrp_name")) {
		(*(*skel).bss).target_hid = get_cgroup1_hierarchy_id(c_str!("net_cl"));
		(*(*skel).bss).target_ancestor_cgid = current_cgid;
		bpf_cgroup1_invalid_id(skel);
	}

	if test__start_subtest(c_str!("test_invalid_cgrp_name2")) {
		(*(*skel).bss).target_hid = get_cgroup1_hierarchy_id(c_str!("net_cls,"));
		(*(*skel).bss).target_ancestor_cgid = current_cgid;
		bpf_cgroup1_invalid_id(skel);
	}

	if test__start_subtest(c_str!("test_sleepable_prog")) {
		(*(*skel).bss).target_hid = hid;
		(*(*skel).bss).target_ancestor_cgid = current_cgid;
		bpf_cgroup1_sleepable(skel);
	}

	goto_cleanup(skel);
}

unsafe fn goto_cleanup(skel: *mut test_cgroup1_hierarchy) {
	cleanup_classid_environment();
	goto_cleanup_cgroup(skel);
}

unsafe fn goto_cleanup_cgroup(skel: *mut test_cgroup1_hierarchy) {
	cleanup_cgroup_environment();
	goto_destroy(skel);
}

unsafe fn goto_destroy(skel: *mut test_cgroup1_hierarchy) {
	test_cgroup1_hierarchy__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
