// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

use core::ffi::{c_char, c_int, c_uint, c_void};

type __u32 = u32;

#[repr(C)]
pub struct bpf_map {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map_info {
	pub id: __u32,
	/* Fields are supplied by the BPF headers in the original C build. */
}

#[repr(C)]
pub struct test_btf_map_in_map__maps {
	pub inner_map1: *mut bpf_map,
	pub inner_map2: *mut bpf_map,
	pub inner_map3: *mut bpf_map,
	pub inner_map4: *mut bpf_map,
	pub inner_map5: *mut bpf_map,
	pub outer_arr_dyn: *mut bpf_map,
	pub outer_arr: *mut bpf_map,
	pub outer_hash: *mut bpf_map,
	pub sockarr_sz2: *mut bpf_map,
	pub outer_sockarr: *mut bpf_map,
	pub inner_map_sz2: *mut bpf_map,
}

#[repr(C)]
pub struct test_btf_map_in_map__bss {
	pub input: c_int,
}

#[repr(C)]
pub struct test_btf_map_in_map {
	pub maps: test_btf_map_in_map__maps,
	pub bss: *mut test_btf_map_in_map__bss,
}

unsafe extern "C" {
	fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
	fn printf(format: *const c_char, ...) -> c_int;
	fn usleep(usec: c_uint) -> c_int;

	fn bpf_map__fd(map: *mut bpf_map) -> c_int;
	fn bpf_map_get_info_by_fd(fd: c_int, info: *mut bpf_map_info, info_len: *__u32) -> c_int;
	fn bpf_map_update_elem(
		fd: c_int,
		key: *const c_void,
		value: *const c_void,
		flags: u64,
	) -> c_int;
	fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;

	fn test_btf_map_in_map__open_and_load() -> *mut test_btf_map_in_map;
	fn test_btf_map_in_map__attach(skel: *mut test_btf_map_in_map) -> c_int;
	fn test_btf_map_in_map__destroy(skel: *mut test_btf_map_in_map);

	fn test__start_subtest(name: *const c_char) -> bool;

	fn CHECK(condition: bool, name: *const c_char, format: *const c_char, ...) -> bool;
	fn CHECK_FAIL(condition: c_int) -> bool;
}

static mut duration: c_int = 0;

unsafe fn bpf_map_id(map: *mut bpf_map) -> __u32 {
	let mut info: bpf_map_info = core::mem::zeroed();
	let mut info_len: __u32 = core::mem::size_of::<bpf_map_info>() as __u32;
	let err: c_int;

	memset(
		&mut info as *mut bpf_map_info as *mut c_void,
		0,
		info_len as usize,
	);
	err = bpf_map_get_info_by_fd(bpf_map__fd(map), &mut info, &mut info_len);
	if err != 0 {
		return 0;
	}
	return info.id;
}

unsafe fn test_lookup_update() {
	let map1_fd: c_int;
	let map2_fd: c_int;
	let map3_fd: c_int;
	let map4_fd: c_int;
	let map5_fd: c_int;
	let map1_id: c_int;
	let map2_id: c_int;
	let outer_arr_fd: c_int;
	let outer_hash_fd: c_int;
	let outer_arr_dyn_fd: c_int;
	let mut skel: *mut test_btf_map_in_map;
	let mut err: c_int;
	let mut key: c_int = 0;
	let mut val: c_int = 0;
	let mut i: c_int;

	skel = test_btf_map_in_map__open_and_load();
	if CHECK(
		skel.is_null(),
		c"skel_open".as_ptr(),
		c"failed to open&load skeleton\n".as_ptr(),
	) {
		return;
	}

	err = test_btf_map_in_map__attach(skel);
	if CHECK(
		err != 0,
		c"skel_attach".as_ptr(),
		c"skeleton attach failed: %d\n".as_ptr(),
		err,
	) {
		goto_cleanup(skel);
		return;
	}

	map1_fd = bpf_map__fd((*skel).maps.inner_map1);
	map2_fd = bpf_map__fd((*skel).maps.inner_map2);
	map3_fd = bpf_map__fd((*skel).maps.inner_map3);
	map4_fd = bpf_map__fd((*skel).maps.inner_map4);
	map5_fd = bpf_map__fd((*skel).maps.inner_map5);
	outer_arr_dyn_fd = bpf_map__fd((*skel).maps.outer_arr_dyn);
	outer_arr_fd = bpf_map__fd((*skel).maps.outer_arr);
	outer_hash_fd = bpf_map__fd((*skel).maps.outer_hash);

	/* inner1 = input, inner2 = input + 1, inner3 = input + 2 */
	bpf_map_update_elem(
		outer_arr_fd,
		&key as *const c_int as *const c_void,
		&map1_fd as *const c_int as *const c_void,
		0,
	);
	bpf_map_update_elem(
		outer_hash_fd,
		&key as *const c_int as *const c_void,
		&map2_fd as *const c_int as *const c_void,
		0,
	);
	bpf_map_update_elem(
		outer_arr_dyn_fd,
		&key as *const c_int as *const c_void,
		&map3_fd as *const c_int as *const c_void,
		0,
	);
	(*(*skel).bss).input = 1;
	usleep(1);
	bpf_map_lookup_elem(
		map1_fd,
		&key as *const c_int as *const c_void,
		&mut val as *mut c_int as *mut c_void,
	);
	CHECK(val != 1, c"inner1".as_ptr(), c"got %d != exp %d\n".as_ptr(), val, 1);
	bpf_map_lookup_elem(
		map2_fd,
		&key as *const c_int as *const c_void,
		&mut val as *mut c_int as *mut c_void,
	);
	CHECK(val != 2, c"inner2".as_ptr(), c"got %d != exp %d\n".as_ptr(), val, 2);
	bpf_map_lookup_elem(
		map3_fd,
		&key as *const c_int as *const c_void,
		&mut val as *mut c_int as *mut c_void,
	);
	CHECK(val != 3, c"inner3".as_ptr(), c"got %d != exp %d\n".as_ptr(), val, 3);

	/* inner2 = input, inner1 = input + 1, inner4 = input + 2 */
	bpf_map_update_elem(
		outer_arr_fd,
		&key as *const c_int as *const c_void,
		&map2_fd as *const c_int as *const c_void,
		0,
	);
	bpf_map_update_elem(
		outer_hash_fd,
		&key as *const c_int as *const c_void,
		&map1_fd as *const c_int as *const c_void,
		0,
	);
	bpf_map_update_elem(
		outer_arr_dyn_fd,
		&key as *const c_int as *const c_void,
		&map4_fd as *const c_int as *const c_void,
		0,
	);
	(*(*skel).bss).input = 3;
	usleep(1);
	bpf_map_lookup_elem(
		map1_fd,
		&key as *const c_int as *const c_void,
		&mut val as *mut c_int as *mut c_void,
	);
	CHECK(val != 4, c"inner1".as_ptr(), c"got %d != exp %d\n".as_ptr(), val, 4);
	bpf_map_lookup_elem(
		map2_fd,
		&key as *const c_int as *const c_void,
		&mut val as *mut c_int as *mut c_void,
	);
	CHECK(val != 3, c"inner2".as_ptr(), c"got %d != exp %d\n".as_ptr(), val, 3);
	bpf_map_lookup_elem(
		map4_fd,
		&key as *const c_int as *const c_void,
		&mut val as *mut c_int as *mut c_void,
	);
	CHECK(val != 5, c"inner4".as_ptr(), c"got %d != exp %d\n".as_ptr(), val, 5);

	/* inner5 = input + 2 */
	bpf_map_update_elem(
		outer_arr_dyn_fd,
		&key as *const c_int as *const c_void,
		&map5_fd as *const c_int as *const c_void,
		0,
	);
	(*(*skel).bss).input = 5;
	usleep(1);
	bpf_map_lookup_elem(
		map5_fd,
		&key as *const c_int as *const c_void,
		&mut val as *mut c_int as *mut c_void,
	);
	CHECK(val != 7, c"inner5".as_ptr(), c"got %d != exp %d\n".as_ptr(), val, 7);

	i = 0;
	while i < 5 {
		val = if i % 2 != 0 { map1_fd } else { map2_fd };
		err = bpf_map_update_elem(
			outer_hash_fd,
			&key as *const c_int as *const c_void,
			&val as *const c_int as *const c_void,
			0,
		);
		if CHECK_FAIL(err) {
			printf(c"failed to update hash_of_maps on iter #%d\n".as_ptr(), i);
			goto_cleanup(skel);
			return;
		}
		err = bpf_map_update_elem(
			outer_arr_fd,
			&key as *const c_int as *const c_void,
			&val as *const c_int as *const c_void,
			0,
		);
		if CHECK_FAIL(err) {
			printf(c"failed to update array_of_maps on iter #%d\n".as_ptr(), i);
			goto_cleanup(skel);
			return;
		}
		val = if i % 2 != 0 { map4_fd } else { map5_fd };
		err = bpf_map_update_elem(
			outer_arr_dyn_fd,
			&key as *const c_int as *const c_void,
			&val as *const c_int as *const c_void,
			0,
		);
		if CHECK_FAIL(err) {
			printf(
				c"failed to update array_of_maps (dyn) on iter #%d\n".as_ptr(),
				i,
			);
			goto_cleanup(skel);
			return;
		}
		i += 1;
	}

	map1_id = bpf_map_id((*skel).maps.inner_map1) as c_int;
	map2_id = bpf_map_id((*skel).maps.inner_map2) as c_int;
	CHECK(map1_id == 0, c"map1_id".as_ptr(), c"failed to get ID 1\n".as_ptr());
	CHECK(map2_id == 0, c"map2_id".as_ptr(), c"failed to get ID 2\n".as_ptr());

	goto_cleanup(skel);
}

unsafe fn goto_cleanup(skel: *mut test_btf_map_in_map) {
	test_btf_map_in_map__destroy(skel);
}

unsafe fn test_diff_size() {
	let mut skel: *mut test_btf_map_in_map;
	let mut err: c_int;
	let mut inner_map_fd: c_int;
	let mut zero: c_int = 0;

	skel = test_btf_map_in_map__open_and_load();
	if CHECK(
		skel.is_null(),
		c"skel_open".as_ptr(),
		c"failed to open&load skeleton\n".as_ptr(),
	) {
		return;
	}

	inner_map_fd = bpf_map__fd((*skel).maps.sockarr_sz2);
	err = bpf_map_update_elem(
		bpf_map__fd((*skel).maps.outer_sockarr),
		&zero as *const c_int as *const c_void,
		&inner_map_fd as *const c_int as *const c_void,
		0,
	);
	CHECK(
		err != 0,
		c"outer_sockarr inner map size check".as_ptr(),
		c"cannot use a different size inner_map\n".as_ptr(),
	);

	inner_map_fd = bpf_map__fd((*skel).maps.inner_map_sz2);
	err = bpf_map_update_elem(
		bpf_map__fd((*skel).maps.outer_arr),
		&zero as *const c_int as *const c_void,
		&inner_map_fd as *const c_int as *const c_void,
		0,
	);
	CHECK(
		err == 0,
		c"outer_arr inner map size check".as_ptr(),
		c"incorrectly updated with a different size inner_map\n".as_ptr(),
	);

	test_btf_map_in_map__destroy(skel);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_btf_map_in_map() {
	if test__start_subtest(c"lookup_update".as_ptr()) {
		test_lookup_update();
	}

	if test__start_subtest(c"diff_size".as_ptr()) {
		test_diff_size();
	}
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
