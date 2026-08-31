// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

/* Dependencies from the original C includes:
 * <errno.h>, <sys/syscall.h>, <unistd.h>,
 * "test_global_map_resize.skel.h", and "test_progs.h".
 */

use core::ffi::{c_char, c_int, c_long, c_void};
use core::mem::size_of;

type __u32 = u32;

const __NR_getpid: c_long = 39;
const __NR_getuid: c_long = 102;
const _SC_PAGE_SIZE: c_int = 30;

#[repr(C)]
pub struct bpf_map {
	_private: [u8; 0],
}

#[repr(C)]
pub struct test_global_map_resize__bss {
	pub sum: usize,
	pub array: [c_int; 1],
}

#[repr(C)]
pub struct test_global_map_resize__data_custom {
	pub my_array: [c_int; 1],
}

#[repr(C)]
pub struct test_global_map_resize__data_percpu_arr {
	pub percpu_arr: [c_int; 1],
}

#[repr(C)]
pub struct test_global_map_resize__data_array_not_last {
	pub my_array_first: [c_int; 1],
}

#[repr(C)]
pub struct test_global_map_resize__rodata {
	pub pid: c_int,
	pub bss_array_len: usize,
	pub data_array_len: usize,
}

#[repr(C)]
pub struct test_global_map_resize__maps {
	pub bss: *mut bpf_map,
	pub data_custom: *mut bpf_map,
	pub data_percpu_arr: *mut bpf_map,
	pub data_non_array: *mut bpf_map,
	pub data_array_not_last: *mut bpf_map,
}

#[repr(C)]
pub struct test_global_map_resize {
	pub maps: test_global_map_resize__maps,
	pub bss: *mut test_global_map_resize__bss,
	pub data_custom: *mut test_global_map_resize__data_custom,
	pub data_percpu_arr: *mut test_global_map_resize__data_percpu_arr,
	pub data_array_not_last: *mut test_global_map_resize__data_array_not_last,
	pub rodata: *mut test_global_map_resize__rodata,
}

extern "C" {
	fn syscall(num: c_long, ...) -> c_long;
	fn sysconf(name: c_int) -> c_long;
	fn getpid() -> c_int;

	fn test_global_map_resize__open() -> *mut test_global_map_resize;
	fn test_global_map_resize__load(skel: *mut test_global_map_resize) -> c_int;
	fn test_global_map_resize__attach(skel: *mut test_global_map_resize) -> c_int;
	fn test_global_map_resize__destroy(skel: *mut test_global_map_resize);

	fn bpf_map__set_value_size(map: *mut bpf_map, size: __u32) -> c_int;
	fn bpf_map__value_size(map: *const bpf_map) -> usize;
	fn bpf_map__initial_value(map: *mut bpf_map, psize: *mut usize) -> *mut c_void;
	fn bpf_map__btf_key_type_id(map: *const bpf_map) -> __u32;
	fn bpf_map__btf_value_type_id(map: *const bpf_map) -> __u32;
	fn libbpf_num_possible_cpus() -> c_int;

	fn test__start_subtest(name: *const c_char) -> bool;
	fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
	fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
	fn ASSERT_EQ<T: PartialEq>(actual: T, expected: T, name: *const c_char) -> bool;
	fn ASSERT_NEQ<T: PartialEq>(actual: T, expected: T, name: *const c_char) -> bool;
	fn ASSERT_GT<T: PartialOrd>(actual: T, expected: T, name: *const c_char) -> bool;
}

unsafe fn run_prog_bss_array_sum() {
	let _ = syscall(__NR_getpid);
}

unsafe fn run_prog_data_array_sum() {
	let _ = syscall(__NR_getuid);
}

unsafe fn global_map_resize_bss_subtest() {
	let mut err: c_int;
	let mut skel: *mut test_global_map_resize;
	let mut map: *mut bpf_map;
	let desired_sz: __u32 =
		(size_of::<usize>() as c_long + sysconf(_SC_PAGE_SIZE) * 2) as __u32;
	let mut array_len: usize;
	let mut actual_sz: usize = 0;
	let new_sz: usize;
	let array: *mut c_int;

	skel = test_global_map_resize__open();
	if !ASSERT_OK_PTR(skel as *const c_void, c"test_global_map_resize__open".as_ptr()) {
		goto_teardown_bss(skel);
		return;
	}

	/* set some initial value before resizing.
	 * it is expected this non-zero value will be preserved
	 * while resizing.
	 */
	(*(*skel).bss).array[0] = 1;

	/* resize map value and verify the new size */
	map = (*skel).maps.bss;
	err = bpf_map__set_value_size(map, desired_sz);
	if !ASSERT_OK(err, c"bpf_map__set_value_size".as_ptr()) {
		goto_teardown_bss(skel);
		return;
	}
	if !ASSERT_EQ(bpf_map__value_size(map), desired_sz as usize, c"resize".as_ptr()) {
		goto_teardown_bss(skel);
		return;
	}

	new_sz = size_of::<c_int>() * libbpf_num_possible_cpus() as usize;
	err = bpf_map__set_value_size((*skel).maps.data_percpu_arr, new_sz as __u32);
	ASSERT_OK(err, c"percpu_arr_resize".as_ptr());

	/* set the expected number of elements based on the resized array */
	array_len = (desired_sz as usize - size_of::<usize>()) / size_of::<c_int>();
	if !ASSERT_GT(array_len, 1usize, c"array_len".as_ptr()) {
		goto_teardown_bss(skel);
		return;
	}

	(*skel).bss = bpf_map__initial_value((*skel).maps.bss, &mut actual_sz)
		as *mut test_global_map_resize__bss;
	if !ASSERT_OK_PTR((*skel).bss as *const c_void, c"bpf_map__initial_value (ptr)".as_ptr()) {
		goto_teardown_bss(skel);
		return;
	}
	if !ASSERT_EQ(actual_sz, desired_sz as usize, c"bpf_map__initial_value (size)".as_ptr()) {
		goto_teardown_bss(skel);
		return;
	}

	/* fill the newly resized array with ones,
	 * skipping the first element which was previously set;
	 * access through a plain pointer to avoid -Warray-bounds
	 * since the array was resized beyond its declared length.
	 */
	array = (*(*skel).bss).array.as_mut_ptr();
	for i in 1..array_len {
		*array.add(i) = 1;
	}

	/* set global const values before loading */
	(*(*skel).rodata).pid = getpid();
	(*(*skel).rodata).bss_array_len = array_len;
	(*(*skel).rodata).data_array_len = 1;

	err = test_global_map_resize__load(skel);
	if !ASSERT_OK(err, c"test_global_map_resize__load".as_ptr()) {
		goto_teardown_bss(skel);
		return;
	}
	err = test_global_map_resize__attach(skel);
	if !ASSERT_OK(err, c"test_global_map_resize__attach".as_ptr()) {
		goto_teardown_bss(skel);
		return;
	}

	/* run the bpf program which will sum the contents of the array.
	 * since the array was filled with ones,verify the sum equals array_len
	 */
	run_prog_bss_array_sum();
	if !ASSERT_EQ((*(*skel).bss).sum, array_len, c"sum".as_ptr()) {
		goto_teardown_bss(skel);
		return;
	}

	goto_teardown_bss(skel);
}

unsafe fn goto_teardown_bss(skel: *mut test_global_map_resize) {
	test_global_map_resize__destroy(skel);
}

unsafe fn global_map_resize_data_subtest() {
	let mut skel: *mut test_global_map_resize;
	let mut map: *mut bpf_map;
	let desired_sz: __u32 = (sysconf(_SC_PAGE_SIZE) * 2) as __u32;
	let mut array_len: usize;
	let mut actual_sz: usize = 0;
	let new_sz: usize;
	let mut err: c_int;

	skel = test_global_map_resize__open();
	if !ASSERT_OK_PTR(skel as *const c_void, c"test_global_map_resize__open".as_ptr()) {
		goto_teardown_data(skel);
		return;
	}

	/* set some initial value before resizing.
	 * it is expected this non-zero value will be preserved
	 * while resizing.
	 */
	(*(*skel).data_custom).my_array[0] = 1;

	/* resize map value and verify the new size */
	map = (*skel).maps.data_custom;
	err = bpf_map__set_value_size(map, desired_sz);
	if !ASSERT_OK(err, c"bpf_map__set_value_size".as_ptr()) {
		goto_teardown_data(skel);
		return;
	}
	if !ASSERT_EQ(bpf_map__value_size(map), desired_sz as usize, c"resize".as_ptr()) {
		goto_teardown_data(skel);
		return;
	}

	new_sz = size_of::<c_int>() * libbpf_num_possible_cpus() as usize;
	err = bpf_map__set_value_size((*skel).maps.data_percpu_arr, new_sz as __u32);
	ASSERT_OK(err, c"percpu_arr_resize".as_ptr());

	/* set the expected number of elements based on the resized array */
	array_len = (desired_sz as usize - size_of::<usize>()) / size_of::<c_int>();
	if !ASSERT_GT(array_len, 1usize, c"array_len".as_ptr()) {
		goto_teardown_data(skel);
		return;
	}

	(*skel).data_custom = bpf_map__initial_value((*skel).maps.data_custom, &mut actual_sz)
		as *mut test_global_map_resize__data_custom;
	if !ASSERT_OK_PTR(
		(*skel).data_custom as *const c_void,
		c"bpf_map__initial_value (ptr)".as_ptr(),
	) {
		goto_teardown_data(skel);
		return;
	}
	if !ASSERT_EQ(actual_sz, desired_sz as usize, c"bpf_map__initial_value (size)".as_ptr()) {
		goto_teardown_data(skel);
		return;
	}

	/* fill the newly resized array with ones,
	 * skipping the first element which was previously set
	 */
	for i in 1..array_len {
		*(*(*skel).data_custom).my_array.as_mut_ptr().add(i) = 1;
	}

	/* set global const values before loading */
	(*(*skel).rodata).pid = getpid();
	(*(*skel).rodata).bss_array_len = 1;
	(*(*skel).rodata).data_array_len = array_len;

	err = test_global_map_resize__load(skel);
	if !ASSERT_OK(err, c"test_global_map_resize__load".as_ptr()) {
		goto_teardown_data(skel);
		return;
	}
	err = test_global_map_resize__attach(skel);
	if !ASSERT_OK(err, c"test_global_map_resize__attach".as_ptr()) {
		goto_teardown_data(skel);
		return;
	}

	/* run the bpf program which will sum the contents of the array.
	 * since the array was filled with ones,verify the sum equals array_len
	 */
	run_prog_data_array_sum();
	if !ASSERT_EQ((*(*skel).bss).sum, array_len, c"sum".as_ptr()) {
		goto_teardown_data(skel);
		return;
	}

	goto_teardown_data(skel);
}

unsafe fn goto_teardown_data(skel: *mut test_global_map_resize) {
	test_global_map_resize__destroy(skel);
}

unsafe fn global_map_resize_invalid_subtest() {
	let mut err: c_int;
	let mut skel: *mut test_global_map_resize;
	let mut map: *mut bpf_map;
	let mut element_sz: __u32;
	let mut desired_sz: __u32;

	skel = test_global_map_resize__open();
	if !ASSERT_OK_PTR(skel as *const c_void, c"test_global_map_resize__open".as_ptr()) {
		return;
	}

	 /* attempt to resize a global datasec map to size
	  * which does NOT align with array
	  */
	map = (*skel).maps.data_custom;
	if !ASSERT_NEQ(bpf_map__btf_value_type_id(map), 0, c".data.custom initial btf".as_ptr()) {
		goto_teardown_invalid(skel);
		return;
	}
	/* set desired size a fraction of element size beyond an aligned size */
	element_sz = size_of::<c_int>() as __u32;
	desired_sz = element_sz + element_sz / 2;
	/* confirm desired size does NOT align with array */
	if !ASSERT_NEQ(desired_sz % element_sz, 0, c"my_array alignment".as_ptr()) {
		goto_teardown_invalid(skel);
		return;
	}
	err = bpf_map__set_value_size(map, desired_sz);
	/* confirm resize is OK but BTF info is cleared */
	if !ASSERT_OK(err, c".data.custom bpf_map__set_value_size".as_ptr())
		|| !ASSERT_EQ(
			bpf_map__btf_key_type_id(map),
			0,
			c".data.custom clear btf key".as_ptr(),
		)
		|| !ASSERT_EQ(
			bpf_map__btf_value_type_id(map),
			0,
			c".data.custom clear btf val".as_ptr(),
		)
	{
		goto_teardown_invalid(skel);
		return;
	}

	/* attempt to resize a global datasec map whose only var is NOT an array */
	map = (*skel).maps.data_non_array;
	if !ASSERT_NEQ(
		bpf_map__btf_value_type_id(map),
		0,
		c".data.non_array initial btf".as_ptr(),
	) {
		goto_teardown_invalid(skel);
		return;
	}
	/* set desired size to arbitrary value */
	desired_sz = 1024;
	err = bpf_map__set_value_size(map, desired_sz);
	/* confirm resize is OK but BTF info is cleared */
	if !ASSERT_OK(err, c".data.non_array bpf_map__set_value_size".as_ptr())
		|| !ASSERT_EQ(
			bpf_map__btf_key_type_id(map),
			0,
			c".data.non_array clear btf key".as_ptr(),
		)
		|| !ASSERT_EQ(
			bpf_map__btf_value_type_id(map),
			0,
			c".data.non_array clear btf val".as_ptr(),
		)
	{
		goto_teardown_invalid(skel);
		return;
	}

	/* attempt to resize a global datasec map
	 * whose last var is NOT an array
	 */
	map = (*skel).maps.data_array_not_last;
	if !ASSERT_NEQ(
		bpf_map__btf_value_type_id(map),
		0,
		c".data.array_not_last initial btf".as_ptr(),
	) {
		goto_teardown_invalid(skel);
		return;
	}
	/* set desired size to a multiple of element size */
	element_sz = size_of::<c_int>() as __u32;
	desired_sz = element_sz * 8;
	/* confirm desired size aligns with array */
	if !ASSERT_EQ(desired_sz % element_sz, 0, c"my_array_first alignment".as_ptr()) {
		goto_teardown_invalid(skel);
		return;
	}
	err = bpf_map__set_value_size(map, desired_sz);
	/* confirm resize is OK but BTF info is cleared */
	if !ASSERT_OK(err, c".data.array_not_last bpf_map__set_value_size".as_ptr())
		|| !ASSERT_EQ(
			bpf_map__btf_key_type_id(map),
			0,
			c".data.array_not_last clear btf key".as_ptr(),
		)
		|| !ASSERT_EQ(
			bpf_map__btf_value_type_id(map),
			0,
			c".data.array_not_last clear btf val".as_ptr(),
		)
	{
		goto_teardown_invalid(skel);
		return;
	}

	goto_teardown_invalid(skel);
}

unsafe fn goto_teardown_invalid(skel: *mut test_global_map_resize) {
	test_global_map_resize__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_global_map_resize() {
	if test__start_subtest(c"global_map_resize_bss".as_ptr()) {
		global_map_resize_bss_subtest();
	}

	if test__start_subtest(c"global_map_resize_data".as_ptr()) {
		global_map_resize_data_subtest();
	}

	if test__start_subtest(c"global_map_resize_invalid".as_ptr()) {
		global_map_resize_invalid_subtest();
	}
}
