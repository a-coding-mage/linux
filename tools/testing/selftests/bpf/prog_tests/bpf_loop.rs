// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

/* Translated from C. External declarations correspond to:
 * <test_progs.h>, <network_helpers.h>, and "bpf_loop.skel.h".
 */

const E2BIG: i32 = 7;
const EINVAL: i32 = 22;
const BPF_NOEXIST: u64 = 1;

#[repr(C)]
pub struct bpf_link {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bpf_loop {
	pub bss: *mut bpf_loop_bss,
	pub data: *mut bpf_loop_data,
	pub progs: bpf_loop_progs,
	pub maps: bpf_loop_maps,
}

#[repr(C)]
pub struct bpf_loop_bss {
	pub nr_loops: i32,
	pub nr_loops_returned: i32,
	pub g_output: i32,
	pub err: i32,
	pub nested_callback_nr_loops: u32,
	pub callback_selector: i32,
	pub pid: i32,
}

#[repr(C)]
pub struct bpf_loop_data {
	pub stop_index: i32,
}

#[repr(C)]
pub struct bpf_loop_progs {
	pub test_prog: *mut bpf_program,
	pub prog_null_ctx: *mut bpf_program,
	pub prog_invalid_flags: *mut bpf_program,
	pub prog_nested_calls: *mut bpf_program,
	pub prog_non_constant_callback: *mut bpf_program,
	pub stack_check: *mut bpf_program,
}

#[repr(C)]
pub struct bpf_loop_maps {
	pub map1: *mut bpf_map,
}

unsafe extern "C" {
	fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
	fn bpf_link__destroy(link: *mut bpf_link);
	fn bpf_map__fd(map: *mut bpf_map) -> i32;
	fn bpf_map_update_elem(fd: i32, key: *const core::ffi::c_void, value: *const core::ffi::c_void, flags: u64) -> i32;
	fn bpf_map_lookup_elem(fd: i32, key: *const core::ffi::c_void, value: *mut core::ffi::c_void) -> i32;
	fn bpf_loop__open_and_load() -> *mut bpf_loop;
	fn bpf_loop__destroy(skel: *mut bpf_loop);
	fn getpid() -> i32;
	fn usleep(usec: u32) -> i32;

	fn ASSERT_OK_PTR(ptr: *const core::ffi::c_void, name: *const u8) -> bool;
	fn ASSERT_EQ(actual: i64, expected: i64, name: *const u8) -> bool;
	fn ASSERT_GE(actual: i64, expected: i64, name: *const u8) -> bool;
	fn ASSERT_OK(err: i32, name: *const u8) -> bool;
	fn test__start_subtest(name: *const u8) -> bool;
}

unsafe fn check_nr_loops(skel: *mut bpf_loop) {
	let link: *mut bpf_link;

	link = bpf_program__attach((*skel).progs.test_prog);
	if !ASSERT_OK_PTR(link.cast(), c"link".as_ptr().cast()) {
		return;
	}

	/* test 0 loops */
	(*(*skel).bss).nr_loops = 0;

	usleep(1);

	ASSERT_EQ((*(*skel).bss).nr_loops_returned as i64, (*(*skel).bss).nr_loops as i64,
		  c"0 loops".as_ptr().cast());

	/* test 500 loops */
	(*(*skel).bss).nr_loops = 500;

	usleep(1);

	ASSERT_EQ((*(*skel).bss).nr_loops_returned as i64, (*(*skel).bss).nr_loops as i64,
		  c"500 loops".as_ptr().cast());
	ASSERT_EQ((*(*skel).bss).g_output as i64, ((500 * 499) / 2) as i64, c"g_output".as_ptr().cast());

	/* test exceeding the max limit */
	(*(*skel).bss).nr_loops = -1;

	usleep(1);

	ASSERT_EQ((*(*skel).bss).err as i64, (-E2BIG) as i64, c"over max limit".as_ptr().cast());

	bpf_link__destroy(link);
}

unsafe fn check_callback_fn_stop(skel: *mut bpf_loop) {
	let link: *mut bpf_link;

	link = bpf_program__attach((*skel).progs.test_prog);
	if !ASSERT_OK_PTR(link.cast(), c"link".as_ptr().cast()) {
		return;
	}

	/* testing that loop is stopped when callback_fn returns 1 */
	(*(*skel).bss).nr_loops = 400;
	(*(*skel).data).stop_index = 50;

	usleep(1);

	ASSERT_EQ((*(*skel).bss).nr_loops_returned as i64, ((*(*skel).data).stop_index + 1) as i64,
		  c"nr_loops_returned".as_ptr().cast());
	ASSERT_EQ((*(*skel).bss).g_output as i64, ((50 * 49) / 2) as i64,
		  c"g_output".as_ptr().cast());

	bpf_link__destroy(link);
}

unsafe fn check_null_callback_ctx(skel: *mut bpf_loop) {
	let link: *mut bpf_link;

	/* check that user is able to pass in a null callback_ctx */
	link = bpf_program__attach((*skel).progs.prog_null_ctx);
	if !ASSERT_OK_PTR(link.cast(), c"link".as_ptr().cast()) {
		return;
	}

	(*(*skel).bss).nr_loops = 10;

	usleep(1);

	ASSERT_EQ((*(*skel).bss).nr_loops_returned as i64, (*(*skel).bss).nr_loops as i64,
		  c"nr_loops_returned".as_ptr().cast());

	bpf_link__destroy(link);
}

unsafe fn check_invalid_flags(skel: *mut bpf_loop) {
	let link: *mut bpf_link;

	/* check that passing in non-zero flags returns -EINVAL */
	link = bpf_program__attach((*skel).progs.prog_invalid_flags);
	if !ASSERT_OK_PTR(link.cast(), c"link".as_ptr().cast()) {
		return;
	}

	usleep(1);

	ASSERT_EQ((*(*skel).bss).err as i64, (-EINVAL) as i64, c"err".as_ptr().cast());

	bpf_link__destroy(link);
}

unsafe fn check_nested_calls(skel: *mut bpf_loop) {
	let nr_loops: u32 = 100;
	let nested_callback_nr_loops: u32 = 4;
	let link: *mut bpf_link;

	/* check that nested calls are supported */
	link = bpf_program__attach((*skel).progs.prog_nested_calls);
	if !ASSERT_OK_PTR(link.cast(), c"link".as_ptr().cast()) {
		return;
	}

	(*(*skel).bss).nr_loops = nr_loops as i32;
	(*(*skel).bss).nested_callback_nr_loops = nested_callback_nr_loops;

	usleep(1);

	ASSERT_EQ((*(*skel).bss).nr_loops_returned as i64, (nr_loops * nested_callback_nr_loops
		  * nested_callback_nr_loops) as i64, c"nr_loops_returned".as_ptr().cast());
	ASSERT_EQ((*(*skel).bss).g_output as i64, (((4 * 3) / 2) as u32 * nested_callback_nr_loops
		* nr_loops) as i64, c"g_output".as_ptr().cast());

	bpf_link__destroy(link);
}

unsafe fn check_non_constant_callback(skel: *mut bpf_loop) {
	let link: *mut bpf_link =
		bpf_program__attach((*skel).progs.prog_non_constant_callback);

	if !ASSERT_OK_PTR(link.cast(), c"link".as_ptr().cast()) {
		return;
	}

	(*(*skel).bss).callback_selector = 0x0F;
	usleep(1);
	ASSERT_EQ((*(*skel).bss).g_output as i64, 0x0F, c"g_output #1".as_ptr().cast());

	(*(*skel).bss).callback_selector = 0xF0;
	usleep(1);
	ASSERT_EQ((*(*skel).bss).g_output as i64, 0xF0, c"g_output #2".as_ptr().cast());

	bpf_link__destroy(link);
}

unsafe fn check_stack(skel: *mut bpf_loop) {
	let link: *mut bpf_link = bpf_program__attach((*skel).progs.stack_check);
	let max_key: i32 = 12;
	let mut key: i32;
	let map_fd: i32;

	if !ASSERT_OK_PTR(link.cast(), c"link".as_ptr().cast()) {
		return;
	}

	map_fd = bpf_map__fd((*skel).maps.map1);

	if !ASSERT_GE(map_fd as i64, 0, c"bpf_map__fd".as_ptr().cast()) {
		bpf_link__destroy(link);
		return;
	}

	key = 1;
	while key <= max_key {
		let val: i32 = key;
		let err: i32 = bpf_map_update_elem(map_fd, (&key as *const i32).cast(), (&val as *const i32).cast(), BPF_NOEXIST);

		if !ASSERT_OK(err, c"bpf_map_update_elem".as_ptr().cast()) {
			bpf_link__destroy(link);
			return;
		}
		key += 1;
	}

	usleep(1);

	key = 1;
	while key <= max_key {
		let mut val: i32 = 0;
		let err: i32 = bpf_map_lookup_elem(map_fd, (&key as *const i32).cast(), (&mut val as *mut i32).cast());

		if !ASSERT_OK(err, c"bpf_map_lookup_elem".as_ptr().cast()) {
			bpf_link__destroy(link);
			return;
		}
		if !ASSERT_EQ(val as i64, (key + 1) as i64, c"bad value in the map".as_ptr().cast()) {
			bpf_link__destroy(link);
			return;
		}
		key += 1;
	}

	bpf_link__destroy(link);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_bpf_loop() {
	let skel: *mut bpf_loop;

	skel = bpf_loop__open_and_load();
	if !ASSERT_OK_PTR(skel.cast(), c"bpf_loop__open_and_load".as_ptr().cast()) {
		return;
	}

	(*(*skel).bss).pid = getpid();

	if test__start_subtest(c"check_nr_loops".as_ptr().cast()) {
		check_nr_loops(skel);
	}
	if test__start_subtest(c"check_callback_fn_stop".as_ptr().cast()) {
		check_callback_fn_stop(skel);
	}
	if test__start_subtest(c"check_null_callback_ctx".as_ptr().cast()) {
		check_null_callback_ctx(skel);
	}
	if test__start_subtest(c"check_invalid_flags".as_ptr().cast()) {
		check_invalid_flags(skel);
	}
	if test__start_subtest(c"check_nested_calls".as_ptr().cast()) {
		check_nested_calls(skel);
	}
	if test__start_subtest(c"check_non_constant_callback".as_ptr().cast()) {
		check_non_constant_callback(skel);
	}
	if test__start_subtest(c"check_stack".as_ptr().cast()) {
		check_stack(skel);
	}

	bpf_loop__destroy(skel);
}
