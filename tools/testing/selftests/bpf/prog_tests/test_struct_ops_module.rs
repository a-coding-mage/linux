// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

/* Translated from C. External declarations correspond to test_progs.h,
 * sys/epoll.h, and generated *.skel.h dependencies.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

type u32 = c_uint;
type u64 = u64;

const EPOLL_CTL_ADD: c_int = 1;
const EPOLLHUP: u32 = 0x10;

#[repr(C)]
pub struct bpf_map {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map_info {
	pub btf_vmlinux_id: u32,
}

#[repr(C)]
pub struct bpf_btf_info {
	pub name: u64,
	pub name_len: u32,
}

#[repr(C)]
pub union epoll_data {
	pub ptr: *mut c_void,
	pub fd: c_int,
	pub u32_: u32,
	pub u64_: u64,
}

#[repr(C)]
pub struct epoll_event {
	pub events: u32,
	pub data: epoll_data,
}

#[repr(C)]
pub struct struct_ops_module_bss {
	pub test_1_result: c_int,
	pub test_2_result: c_int,
}

#[repr(C)]
pub struct struct_ops_module_struct_ops {
	pub testmod_1: *mut testmod_1_ops,
	pub testmod_2: *mut c_void,
	pub testmod_zeroed: *mut testmod_zeroed_ops,
}

#[repr(C)]
pub struct struct_ops_module_progs {
	pub test_2: *mut bpf_program,
	pub test_3: *mut bpf_program,
}

#[repr(C)]
pub struct struct_ops_module_maps {
	pub testmod_1: *mut bpf_map,
	pub testmod_2: *mut bpf_map,
	pub testmod_zeroed: *mut bpf_map,
	pub testmod_incompatible: *mut bpf_map,
}

#[repr(C)]
pub struct struct_ops_module {
	pub bss: *mut struct_ops_module_bss,
	pub struct_ops: struct_ops_module_struct_ops,
	pub progs: struct_ops_module_progs,
	pub maps: struct_ops_module_maps,
}

#[repr(C)]
pub struct testmod_1_ops {
	pub data: c_int,
	pub test_2: *mut bpf_program,
}

#[repr(C)]
pub struct testmod_zeroed_ops {
	pub zeroed: c_int,
	pub zeroed_op: *mut bpf_program,
}

#[repr(C)]
pub struct struct_ops_nulled_out_cb_struct_ops {
	pub ops: *mut nulled_out_ops,
}

#[repr(C)]
pub struct struct_ops_nulled_out_cb_progs {
	pub test_1_turn_off: *mut bpf_program,
}

#[repr(C)]
pub struct struct_ops_nulled_out_cb {
	pub struct_ops: struct_ops_nulled_out_cb_struct_ops,
	pub progs: struct_ops_nulled_out_cb_progs,
}

#[repr(C)]
pub struct nulled_out_ops {
	pub test_1: *mut bpf_program,
}

#[repr(C)]
pub struct struct_ops_forgotten_cb_struct_ops {
	pub ops: *mut forgotten_ops,
}

#[repr(C)]
pub struct struct_ops_forgotten_cb_progs {
	pub test_1_forgotten: *mut bpf_program,
}

#[repr(C)]
pub struct struct_ops_forgotten_cb {
	pub struct_ops: struct_ops_forgotten_cb_struct_ops,
	pub progs: struct_ops_forgotten_cb_progs,
}

#[repr(C)]
pub struct forgotten_ops {
	pub test_1: *mut bpf_program,
}

#[repr(C)]
pub struct struct_ops_detach_maps {
	pub testmod_do_detach: *mut bpf_map,
}

#[repr(C)]
pub struct struct_ops_detach {
	pub maps: struct_ops_detach_maps,
}

unsafe extern "C" {
	fn bpf_btf_get_fd_by_id(id: u32) -> c_int;
	fn bpf_btf_get_info_by_fd(fd: c_int, info: *mut bpf_btf_info, info_len: *mut u32) -> c_int;
	fn bpf_map__attach_struct_ops(map: *mut bpf_map) -> *mut bpf_link;
	fn bpf_link__destroy(link: *mut bpf_link);
	fn bpf_link__fd(link: *mut bpf_link) -> c_int;
	fn bpf_link__detach(link: *mut bpf_link) -> c_int;
	fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
	fn bpf_program__autoload(prog: *mut bpf_program) -> bool;
	fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
	fn bpf_map__set_autocreate(map: *mut bpf_map, autocreate: bool);
	fn bpf_map__fd(map: *mut bpf_map) -> c_int;
	fn bpf_map_get_info_by_fd(fd: c_int, info: *mut bpf_map_info, info_len: *mut u32) -> c_int;

	fn struct_ops_module__open() -> *mut struct_ops_module;
	fn struct_ops_module__load(skel: *mut struct_ops_module) -> c_int;
	fn struct_ops_module__destroy(skel: *mut struct_ops_module);

	fn struct_ops_nulled_out_cb__open() -> *mut struct_ops_nulled_out_cb;
	fn struct_ops_nulled_out_cb__load(skel: *mut struct_ops_nulled_out_cb) -> c_int;
	fn struct_ops_nulled_out_cb__destroy(skel: *mut struct_ops_nulled_out_cb);

	fn struct_ops_forgotten_cb__open() -> *mut struct_ops_forgotten_cb;
	fn struct_ops_forgotten_cb__load(skel: *mut struct_ops_forgotten_cb) -> c_int;
	fn struct_ops_forgotten_cb__destroy(skel: *mut struct_ops_forgotten_cb);

	fn struct_ops_detach__open_and_load() -> *mut struct_ops_detach;
	fn struct_ops_detach__destroy(skel: *mut struct_ops_detach);

	fn start_libbpf_log_capture();
	fn stop_libbpf_log_capture() -> *mut c_char;
	fn free(ptr: *mut c_void);
	fn close(fd: c_int) -> c_int;
	fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
	fn epoll_create1(flags: c_int) -> c_int;
	fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut epoll_event) -> c_int;
	fn epoll_wait(epfd: c_int, events: *mut epoll_event, maxevents: c_int, timeout: c_int) -> c_int;

	fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
	fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
	fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
	fn ASSERT_OK_PTR<T>(ptr: *mut T, name: *const c_char) -> bool;
	fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
	fn ASSERT_FALSE(value: bool, name: *const c_char) -> bool;
	fn ASSERT_LT(actual: c_int, expected: c_int, name: *const c_char) -> bool;
	fn ASSERT_TRUE(value: u32, name: *const c_char) -> bool;
	fn ASSERT_HAS_SUBSTR(log: *mut c_char, substr: *const c_char, name: *const c_char) -> bool;
	fn test__start_subtest(name: *const c_char) -> bool;
	fn RUN_TESTS_unsupported_ops();
}

fn ptr_to_u64<T>(ptr: *mut T) -> u64 {
	ptr as u64
}

unsafe fn check_map_info(info: *mut bpf_map_info) {
	let mut btf_info: bpf_btf_info = mem::zeroed();
	let mut btf_name = [0 as c_char; 256];
	let mut btf_info_len: u32 = mem::size_of::<bpf_btf_info>() as u32;
	let mut err: c_int;
	let fd: c_int;

	fd = bpf_btf_get_fd_by_id((*info).btf_vmlinux_id);
	if !ASSERT_GE(fd, 0, b"get_value_type_btf_obj_fd\0".as_ptr() as *const c_char) {
		return;
	}

	btf_info = mem::zeroed();
	btf_info.name = ptr_to_u64(btf_name.as_mut_ptr());
	btf_info.name_len = mem::size_of_val(&btf_name) as u32;
	err = bpf_btf_get_info_by_fd(fd, &mut btf_info, &mut btf_info_len);
	if !ASSERT_OK(err, b"get_value_type_btf_obj_info\0".as_ptr() as *const c_char) {
		goto_cleanup_check_map_info(fd);
		return;
	}

	if !ASSERT_EQ(
		strcmp(btf_name.as_ptr(), b"bpf_testmod\0".as_ptr() as *const c_char),
		0,
		b"get_value_type_btf_obj_name\0".as_ptr() as *const c_char,
	) {
		goto_cleanup_check_map_info(fd);
		return;
	}

	goto_cleanup_check_map_info(fd);
}

unsafe fn goto_cleanup_check_map_info(fd: c_int) {
	close(fd);
}

unsafe fn attach_ops_and_check(
	skel: *mut struct_ops_module,
	map: *mut bpf_map,
	expected_test_2_result: c_int,
) -> c_int {
	let link: *mut bpf_link;

	link = bpf_map__attach_struct_ops(map);
	ASSERT_OK_PTR(link, b"attach_test_mod_1\0".as_ptr() as *const c_char);
	if link.is_null() {
		return -1;
	}

	/* test_{1,2}() would be called from bpf_dummy_reg() in bpf_testmod.c */
	ASSERT_EQ((*(*skel).bss).test_1_result, 0xdeadbeef_u32 as c_int, b"test_1_result\0".as_ptr() as *const c_char);
	ASSERT_EQ((*(*skel).bss).test_2_result, expected_test_2_result, b"test_2_result\0".as_ptr() as *const c_char);

	bpf_link__destroy(link);
	0
}

unsafe fn test_struct_ops_load() {
	let skel: *mut struct_ops_module;
	let mut info: bpf_map_info = mem::zeroed();
	let mut err: c_int;
	let mut len: u32;

	skel = struct_ops_module__open();
	if !ASSERT_OK_PTR(skel, b"struct_ops_module_open\0".as_ptr() as *const c_char) {
		return;
	}

	(*(*skel).struct_ops.testmod_1).data = 13;
	(*(*skel).struct_ops.testmod_1).test_2 = (*skel).progs.test_3;
	/* Since test_2() is not being used, it should be disabled from
	 * auto-loading, or it will fail to load.
	 */
	bpf_program__set_autoload((*skel).progs.test_2, false);
	bpf_map__set_autocreate((*skel).maps.testmod_zeroed, false);

	err = struct_ops_module__load(skel);
	if !ASSERT_OK(err, b"struct_ops_module_load\0".as_ptr() as *const c_char) {
		struct_ops_module__destroy(skel);
		return;
	}

	len = mem::size_of::<bpf_map_info>() as u32;
	err = bpf_map_get_info_by_fd(bpf_map__fd((*skel).maps.testmod_1), &mut info, &mut len);
	if !ASSERT_OK(err, b"bpf_map_get_info_by_fd\0".as_ptr() as *const c_char) {
		struct_ops_module__destroy(skel);
		return;
	}

	check_map_info(&mut info);
	/* test_3() will be called from bpf_dummy_reg() in bpf_testmod.c
	 *
	 * In bpf_testmod.c it will pass 4 and 13 (the value of data) to
	 * .test_2.  So, the value of test_2_result should be 20 (4 + 13 +
	 * 3).
	 */
	if attach_ops_and_check(skel, (*skel).maps.testmod_1, 20) == 0 {
		struct_ops_module__destroy(skel);
		return;
	}
	if attach_ops_and_check(skel, (*skel).maps.testmod_2, 12) == 0 {
		struct_ops_module__destroy(skel);
		return;
	}

	struct_ops_module__destroy(skel);
}

unsafe fn test_struct_ops_not_zeroed() {
	let mut skel: *mut struct_ops_module;
	let mut err: c_int;

	/* zeroed is 0, and zeroed_op is null */
	skel = struct_ops_module__open();
	if !ASSERT_OK_PTR(skel, b"struct_ops_module_open\0".as_ptr() as *const c_char) {
		return;
	}

	(*(*skel).struct_ops.testmod_zeroed).zeroed = 0;
	/* zeroed_op prog should be not loaded automatically now */
	(*(*skel).struct_ops.testmod_zeroed).zeroed_op = ptr::null_mut();

	err = struct_ops_module__load(skel);
	ASSERT_OK(err, b"struct_ops_module_load\0".as_ptr() as *const c_char);

	struct_ops_module__destroy(skel);

	/* zeroed is not 0 */
	skel = struct_ops_module__open();
	if !ASSERT_OK_PTR(skel, b"struct_ops_module_open_not_zeroed\0".as_ptr() as *const c_char) {
		return;
	}

	/* libbpf should reject the testmod_zeroed since struct
	 * bpf_testmod_ops in the kernel has no "zeroed" field and the
	 * value of "zeroed" is non-zero.
	 */
	(*(*skel).struct_ops.testmod_zeroed).zeroed = 0xdeadbeef_u32 as c_int;
	(*(*skel).struct_ops.testmod_zeroed).zeroed_op = ptr::null_mut();
	err = struct_ops_module__load(skel);
	ASSERT_ERR(err, b"struct_ops_module_load_not_zeroed\0".as_ptr() as *const c_char);

	struct_ops_module__destroy(skel);

	/* zeroed_op is not null */
	skel = struct_ops_module__open();
	if !ASSERT_OK_PTR(skel, b"struct_ops_module_open_not_zeroed_op\0".as_ptr() as *const c_char) {
		return;
	}

	/* libbpf should reject the testmod_zeroed since the value of its
	 * "zeroed_op" is not null.
	 */
	(*(*skel).struct_ops.testmod_zeroed).zeroed_op = (*skel).progs.test_3;
	err = struct_ops_module__load(skel);
	ASSERT_ERR(err, b"struct_ops_module_load_not_zeroed_op\0".as_ptr() as *const c_char);

	struct_ops_module__destroy(skel);
}

/* The signature of an implementation might not match the signature of the
 * function pointer prototype defined in the BPF program. This mismatch
 * should be allowed as long as the behavior of the operator program
 * adheres to the signature in the kernel. Libbpf should not enforce the
 * signature; rather, let the kernel verifier handle the enforcement.
 */
unsafe fn test_struct_ops_incompatible() {
	let skel: *mut struct_ops_module;
	let link: *mut bpf_link;
	let mut err: c_int;

	skel = struct_ops_module__open();
	if !ASSERT_OK_PTR(skel, b"struct_ops_module_open\0".as_ptr() as *const c_char) {
		return;
	}

	bpf_map__set_autocreate((*skel).maps.testmod_zeroed, false);

	err = struct_ops_module__load(skel);
	if !ASSERT_OK(err, b"skel_load\0".as_ptr() as *const c_char) {
		struct_ops_module__destroy(skel);
		return;
	}

	link = bpf_map__attach_struct_ops((*skel).maps.testmod_incompatible);
	if ASSERT_OK_PTR(link, b"attach_struct_ops\0".as_ptr() as *const c_char) {
		bpf_link__destroy(link);
	}

	struct_ops_module__destroy(skel);
}

/* validate that it's ok to "turn off" callback that kernel supports */
unsafe fn test_struct_ops_nulled_out_cb() {
	let skel: *mut struct_ops_nulled_out_cb;
	let mut err: c_int;

	skel = struct_ops_nulled_out_cb__open();
	if !ASSERT_OK_PTR(skel, b"skel_open\0".as_ptr() as *const c_char) {
		return;
	}

	/* kernel knows about test_1, but we still null it out */
	(*(*skel).struct_ops.ops).test_1 = ptr::null_mut();

	err = struct_ops_nulled_out_cb__load(skel);
	if !ASSERT_OK(err, b"skel_load\0".as_ptr() as *const c_char) {
		struct_ops_nulled_out_cb__destroy(skel);
		return;
	}

	ASSERT_FALSE(bpf_program__autoload((*skel).progs.test_1_turn_off), b"prog_autoload\0".as_ptr() as *const c_char);
	ASSERT_LT(bpf_program__fd((*skel).progs.test_1_turn_off), 0, b"prog_fd\0".as_ptr() as *const c_char);

	struct_ops_nulled_out_cb__destroy(skel);
}

/* validate that libbpf generates reasonable error message if struct_ops is
 * not referenced in any struct_ops map
 */
unsafe fn test_struct_ops_forgotten_cb() {
	let mut skel: *mut struct_ops_forgotten_cb;
	let mut log: *mut c_char;
	let mut err: c_int;

	skel = struct_ops_forgotten_cb__open();
	if !ASSERT_OK_PTR(skel, b"skel_open\0".as_ptr() as *const c_char) {
		return;
	}

	start_libbpf_log_capture();

	err = struct_ops_forgotten_cb__load(skel);
	if !ASSERT_ERR(err, b"skel_load\0".as_ptr() as *const c_char) {
		struct_ops_forgotten_cb__destroy(skel);
		return;
	}

	log = stop_libbpf_log_capture();
	ASSERT_HAS_SUBSTR(
		log,
		b"prog 'test_1_forgotten': SEC(\"struct_ops\") program isn't referenced anywhere, did you forget to use it?\0".as_ptr() as *const c_char,
		b"libbpf_log\0".as_ptr() as *const c_char,
	);
	free(log as *mut c_void);

	struct_ops_forgotten_cb__destroy(skel);

	/* now let's programmatically use it, we should be fine now */
	skel = struct_ops_forgotten_cb__open();
	if !ASSERT_OK_PTR(skel, b"skel_open\0".as_ptr() as *const c_char) {
		return;
	}

	(*(*skel).struct_ops.ops).test_1 = (*skel).progs.test_1_forgotten; /* not anymore */

	err = struct_ops_forgotten_cb__load(skel);
	if !ASSERT_OK(err, b"skel_load\0".as_ptr() as *const c_char) {
		struct_ops_forgotten_cb__destroy(skel);
		return;
	}

	struct_ops_forgotten_cb__destroy(skel);
}

/* Detach a link from a user space program */
unsafe fn test_detach_link() {
	let mut ev: epoll_event = mem::zeroed();
	let mut events: [epoll_event; 2] = mem::zeroed();
	let skel: *mut struct_ops_detach;
	let mut link: *mut bpf_link = ptr::null_mut();
	let mut fd: c_int;
	let mut epollfd: c_int = -1;
	let mut nfds: c_int;
	let mut err: c_int;

	skel = struct_ops_detach__open_and_load();
	if !ASSERT_OK_PTR(skel, b"struct_ops_detach__open_and_load\0".as_ptr() as *const c_char) {
		return;
	}

	link = bpf_map__attach_struct_ops((*skel).maps.testmod_do_detach);
	if !ASSERT_OK_PTR(link, b"attach_struct_ops\0".as_ptr() as *const c_char) {
		struct_ops_detach__destroy(skel);
		return;
	}

	fd = bpf_link__fd(link);
	if !ASSERT_GE(fd, 0, b"link_fd\0".as_ptr() as *const c_char) {
		bpf_link__destroy(link);
		struct_ops_detach__destroy(skel);
		return;
	}

	epollfd = epoll_create1(0);
	if !ASSERT_GE(epollfd, 0, b"epoll_create1\0".as_ptr() as *const c_char) {
		bpf_link__destroy(link);
		struct_ops_detach__destroy(skel);
		return;
	}

	ev.events = EPOLLHUP;
	ev.data.fd = fd;
	err = epoll_ctl(epollfd, EPOLL_CTL_ADD, fd, &mut ev);
	if !ASSERT_OK(err, b"epoll_ctl\0".as_ptr() as *const c_char) {
		if epollfd >= 0 {
			close(epollfd);
		}
		bpf_link__destroy(link);
		struct_ops_detach__destroy(skel);
		return;
	}

	err = bpf_link__detach(link);
	if !ASSERT_OK(err, b"detach_link\0".as_ptr() as *const c_char) {
		if epollfd >= 0 {
			close(epollfd);
		}
		bpf_link__destroy(link);
		struct_ops_detach__destroy(skel);
		return;
	}

	/* Wait for EPOLLHUP */
	nfds = epoll_wait(epollfd, events.as_mut_ptr(), 2, 500);
	if !ASSERT_EQ(nfds, 1, b"epoll_wait\0".as_ptr() as *const c_char) {
		if epollfd >= 0 {
			close(epollfd);
		}
		bpf_link__destroy(link);
		struct_ops_detach__destroy(skel);
		return;
	}

	if !ASSERT_EQ(events[0].data.fd, fd, b"epoll_wait_fd\0".as_ptr() as *const c_char) {
		if epollfd >= 0 {
			close(epollfd);
		}
		bpf_link__destroy(link);
		struct_ops_detach__destroy(skel);
		return;
	}
	if !ASSERT_TRUE(events[0].events & EPOLLHUP, b"events[0].events\0".as_ptr() as *const c_char) {
		if epollfd >= 0 {
			close(epollfd);
		}
		bpf_link__destroy(link);
		struct_ops_detach__destroy(skel);
		return;
	}

	if epollfd >= 0 {
		close(epollfd);
	}
	bpf_link__destroy(link);
	struct_ops_detach__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn serial_test_struct_ops_module() {
	if test__start_subtest(b"struct_ops_load\0".as_ptr() as *const c_char) {
		test_struct_ops_load();
	}
	if test__start_subtest(b"struct_ops_not_zeroed\0".as_ptr() as *const c_char) {
		test_struct_ops_not_zeroed();
	}
	if test__start_subtest(b"struct_ops_incompatible\0".as_ptr() as *const c_char) {
		test_struct_ops_incompatible();
	}
	if test__start_subtest(b"struct_ops_null_out_cb\0".as_ptr() as *const c_char) {
		test_struct_ops_nulled_out_cb();
	}
	if test__start_subtest(b"struct_ops_forgotten_cb\0".as_ptr() as *const c_char) {
		test_struct_ops_forgotten_cb();
	}
	if test__start_subtest(b"test_detach_link\0".as_ptr() as *const c_char) {
		test_detach_link();
	}
	RUN_TESTS_unsupported_ops();
}
