// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2025 Google LLC. */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

const SHA256_DIGEST_SIZE: usize = 32;

const BPF_MAP_TYPE_ARRAY: c_uint = 2;
const BPF_MAP_TYPE_ARRAY_OF_MAPS: c_uint = 12;

const EACCES: c_int = 13;
const EPERM: c_int = 1;
const ENOTSUPP: c_int = 524;
const EINVAL: c_int = 22;

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
pub struct map_excl_maps {
	pub excl_map: *mut bpf_map,
}

#[repr(C)]
pub struct map_excl_progs {
	pub should_have_access: *mut bpf_program,
	pub should_not_have_access: *mut bpf_program,
}

#[repr(C)]
pub struct map_excl {
	pub maps: map_excl_maps,
	pub progs: map_excl_progs,
}

#[repr(C)]
pub struct bpf_iter_bpf_array_map_progs {
	pub dump_bpf_array_map: *mut bpf_program,
}

#[repr(C)]
pub struct bpf_iter_bpf_array_map {
	pub progs: bpf_iter_bpf_array_map_progs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_map_create_opts {
	pub sz: usize,
	pub btf_fd: c_uint,
	pub btf_key_type_id: c_uint,
	pub btf_value_type_id: c_uint,
	pub btf_vmlinux_value_type_id: c_uint,
	pub inner_map_fd: c_uint,
	pub map_flags: c_uint,
	pub map_extra: u64,
	pub numa_node: c_uint,
	pub map_ifindex: c_uint,
	pub value_type_btf_obj_fd: c_uint,
	pub token_fd: c_uint,
	pub excl_prog_hash: *mut u8,
	pub excl_prog_hash_size: u32,
}

impl Default for bpf_map_create_opts {
	fn default() -> Self {
		Self {
			sz: mem::size_of::<Self>(),
			btf_fd: 0,
			btf_key_type_id: 0,
			btf_value_type_id: 0,
			btf_vmlinux_value_type_id: 0,
			inner_map_fd: 0,
			map_flags: 0,
			map_extra: 0,
			numa_node: 0,
			map_ifindex: 0,
			value_type_btf_obj_fd: 0,
			token_fd: 0,
			excl_prog_hash: ptr::null_mut(),
			excl_prog_hash_size: 0,
		}
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_attach_opts {
	pub sz: usize,
	pub link_info: *mut bpf_iter_link_info,
	pub link_info_len: u32,
}

impl Default for bpf_iter_attach_opts {
	fn default() -> Self {
		Self {
			sz: mem::size_of::<Self>(),
			link_info: ptr::null_mut(),
			link_info_len: 0,
		}
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_link_info_map {
	pub map_fd: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union bpf_iter_link_info {
	pub map: bpf_iter_link_info_map,
}

unsafe extern "C" {
	fn close(fd: c_int) -> c_int;
	fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;

	fn map_excl__open() -> *mut map_excl;
	fn map_excl__load(skel: *mut map_excl) -> c_int;
	fn map_excl__destroy(skel: *mut map_excl);

	fn bpf_iter_bpf_array_map__open_and_load() -> *mut bpf_iter_bpf_array_map;
	fn bpf_iter_bpf_array_map__destroy(skel: *mut bpf_iter_bpf_array_map);

	fn bpf_map__set_exclusive_program(map: *mut bpf_map, prog: *mut bpf_program) -> c_int;
	fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
	fn bpf_program__attach_iter(
		prog: *mut bpf_program,
		opts: *mut bpf_iter_attach_opts,
	) -> *mut bpf_link;
	fn bpf_link__destroy(link: *mut bpf_link);
	fn libbpf_get_error(ptr: *const c_void) -> c_int;

	fn bpf_map_create(
		map_type: c_uint,
		map_name: *const c_char,
		key_size: c_uint,
		value_size: c_uint,
		max_entries: c_uint,
		opts: *const bpf_map_create_opts,
	) -> c_int;
	fn bpf_map_update_elem(
		fd: c_int,
		key: *const c_void,
		value: *const c_void,
		flags: u64,
	) -> c_int;

	fn ASSERT_OK(res: c_int, name: *const c_char) -> bool;
	fn ASSERT_OK_FD(fd: c_int, name: *const c_char) -> bool;
	fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
	fn ASSERT_ERR_PTR(ptr: *const c_void, name: *const c_char) -> bool;
	fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
	fn test__start_subtest(name: *const c_char) -> bool;
}

unsafe fn test_map_excl_allowed() {
	let skel = map_excl__open();
	let mut err: c_int;

	err = bpf_map__set_exclusive_program(
		(*skel).maps.excl_map,
		(*skel).progs.should_have_access,
	);
	if !ASSERT_OK(err, c"bpf_map__set_exclusive_program".as_ptr()) {
		goto_out_map_excl_allowed(skel);
		return;
	}

	bpf_program__set_autoload((*skel).progs.should_have_access, true);
	bpf_program__set_autoload((*skel).progs.should_not_have_access, false);

	err = map_excl__load(skel);
	ASSERT_OK(err, c"map_excl__load".as_ptr());
	goto_out_map_excl_allowed(skel);
}

unsafe fn goto_out_map_excl_allowed(skel: *mut map_excl) {
	map_excl__destroy(skel);
}

unsafe fn test_map_excl_denied() {
	let skel = map_excl__open();
	let mut err: c_int;

	err = bpf_map__set_exclusive_program(
		(*skel).maps.excl_map,
		(*skel).progs.should_have_access,
	);
	if !ASSERT_OK(err, c"bpf_map__make_exclusive".as_ptr()) {
		goto_out_map_excl_denied(skel);
		return;
	}

	bpf_program__set_autoload((*skel).progs.should_have_access, false);
	bpf_program__set_autoload((*skel).progs.should_not_have_access, true);

	err = map_excl__load(skel);
	ASSERT_EQ(err, -EACCES, c"exclusive map access not denied\n".as_ptr());
	goto_out_map_excl_denied(skel);
}

unsafe fn goto_out_map_excl_denied(skel: *mut map_excl) {
	map_excl__destroy(skel);
}

unsafe fn test_map_excl_no_map_in_map() {
	let mut hash = [0u8; SHA256_DIGEST_SIZE];
	let mut excl_opts = bpf_map_create_opts {
		excl_prog_hash: hash.as_mut_ptr(),
		excl_prog_hash_size: mem::size_of_val(&hash) as u32,
		..Default::default()
	};
	let mut outer_opts = bpf_map_create_opts::default();
	let mut tmpl_fd: c_int = -1;
	let mut outer_fd: c_int = -1;
	let mut err: c_int;
	let key: u32 = 0;

	let excl_fd = bpf_map_create(
		BPF_MAP_TYPE_ARRAY,
		c"excl_inner".as_ptr(),
		4,
		4,
		1,
		&excl_opts,
	);
	if !ASSERT_OK_FD(excl_fd, c"create exclusive map".as_ptr()) {
		return;
	}

	outer_opts.inner_map_fd = excl_fd as c_uint;
	err = bpf_map_create(
		BPF_MAP_TYPE_ARRAY_OF_MAPS,
		c"outer_from_excl".as_ptr(),
		4,
		4,
		1,
		&outer_opts,
	);
	if err >= 0 {
		close(err);
	}
	ASSERT_EQ(
		err,
		-ENOTSUPP,
		c"reject exclusive map as map-in-map template".as_ptr(),
	);

	tmpl_fd = bpf_map_create(
		BPF_MAP_TYPE_ARRAY,
		c"tmpl".as_ptr(),
		4,
		4,
		1,
		ptr::null(),
	);
	if !ASSERT_OK_FD(tmpl_fd, c"create inner template".as_ptr()) {
		goto_out_map_excl_no_map_in_map(outer_fd, tmpl_fd, excl_fd);
		return;
	}

	outer_opts.inner_map_fd = tmpl_fd as c_uint;
	outer_fd = bpf_map_create(
		BPF_MAP_TYPE_ARRAY_OF_MAPS,
		c"outer".as_ptr(),
		4,
		4,
		1,
		&outer_opts,
	);
	if !ASSERT_OK_FD(outer_fd, c"create map-of-maps".as_ptr()) {
		goto_out_map_excl_no_map_in_map(outer_fd, tmpl_fd, excl_fd);
		return;
	}

	err = bpf_map_update_elem(
		outer_fd,
		&key as *const _ as *const c_void,
		&excl_fd as *const _ as *const c_void,
		0,
	);
	ASSERT_EQ(
		err,
		-ENOTSUPP,
		c"reject exclusive map as map-in-map element".as_ptr(),
	);
	goto_out_map_excl_no_map_in_map(outer_fd, tmpl_fd, excl_fd);
}

unsafe fn goto_out_map_excl_no_map_in_map(outer_fd: c_int, tmpl_fd: c_int, excl_fd: c_int) {
	if outer_fd >= 0 {
		close(outer_fd);
	}
	if tmpl_fd >= 0 {
		close(tmpl_fd);
	}
	close(excl_fd);
}

unsafe fn test_map_excl_no_map_iter() {
	let mut hash = [0u8; SHA256_DIGEST_SIZE];
	let excl_opts = bpf_map_create_opts {
		excl_prog_hash: hash.as_mut_ptr(),
		excl_prog_hash_size: mem::size_of_val(&hash) as u32,
		..Default::default()
	};
	let mut opts = bpf_iter_attach_opts::default();
	let mut skel: *mut bpf_iter_bpf_array_map = ptr::null_mut();
	let mut linfo = mem::MaybeUninit::<bpf_iter_link_info>::uninit();
	let link: *mut bpf_link;

	let excl_fd = bpf_map_create(
		BPF_MAP_TYPE_ARRAY,
		c"excl_iter".as_ptr(),
		4,
		8,
		3,
		&excl_opts,
	);
	if !ASSERT_OK_FD(excl_fd, c"create exclusive map".as_ptr()) {
		return;
	}

	skel = bpf_iter_bpf_array_map__open_and_load();
	if !ASSERT_OK_PTR(
		skel as *const c_void,
		c"bpf_iter_bpf_array_map__open_and_load".as_ptr(),
	) {
		goto_out_map_excl_no_map_iter(skel, excl_fd);
		return;
	}

	memset(
		linfo.as_mut_ptr() as *mut c_void,
		0,
		mem::size_of::<bpf_iter_link_info>(),
	);
	let mut linfo = linfo.assume_init();
	linfo.map.map_fd = excl_fd as c_uint;
	opts.link_info = &mut linfo;
	opts.link_info_len = mem::size_of_val(&linfo) as u32;

	link = bpf_program__attach_iter((*skel).progs.dump_bpf_array_map, &mut opts);
	if !ASSERT_ERR_PTR(
		link as *const c_void,
		c"reject exclusive map as iter target".as_ptr(),
	) {
		bpf_link__destroy(link);
		goto_out_map_excl_no_map_iter(skel, excl_fd);
		return;
	}
	ASSERT_EQ(
		libbpf_get_error(link as *const c_void),
		-EPERM,
		c"iter attach errno".as_ptr(),
	);
	goto_out_map_excl_no_map_iter(skel, excl_fd);
}

unsafe fn goto_out_map_excl_no_map_iter(skel: *mut bpf_iter_bpf_array_map, excl_fd: c_int) {
	bpf_iter_bpf_array_map__destroy(skel);
	close(excl_fd);
}

unsafe fn test_map_excl_create_validation() {
	let mut o = bpf_map_create_opts::default();
	let mut hash = [0u8; SHA256_DIGEST_SIZE];
	let mut fd: c_int;

	o.excl_prog_hash = hash.as_mut_ptr();
	o.excl_prog_hash_size = (SHA256_DIGEST_SIZE / 2) as u32;
	fd = bpf_map_create(BPF_MAP_TYPE_ARRAY, c"excl".as_ptr(), 4, 4, 1, &o);
	if fd >= 0 {
		close(fd);
	}
	ASSERT_EQ(fd, -EINVAL, c"reject short excl_prog_hash_size".as_ptr());

	o.excl_prog_hash = hash.as_mut_ptr();
	o.excl_prog_hash_size = (SHA256_DIGEST_SIZE * 2) as u32;
	fd = bpf_map_create(BPF_MAP_TYPE_ARRAY, c"excl".as_ptr(), 4, 4, 1, &o);
	if fd >= 0 {
		close(fd);
	}
	ASSERT_EQ(fd, -EINVAL, c"reject long excl_prog_hash_size".as_ptr());

	o.excl_prog_hash = hash.as_mut_ptr();
	o.excl_prog_hash_size = 0;
	fd = bpf_map_create(BPF_MAP_TYPE_ARRAY, c"excl".as_ptr(), 4, 4, 1, &o);
	if fd >= 0 {
		close(fd);
	}
	ASSERT_EQ(fd, -EINVAL, c"reject hash pointer with zero size".as_ptr());

	o.excl_prog_hash = ptr::null_mut();
	o.excl_prog_hash_size = SHA256_DIGEST_SIZE as u32;
	fd = bpf_map_create(BPF_MAP_TYPE_ARRAY, c"excl".as_ptr(), 4, 4, 1, &o);
	if fd >= 0 {
		close(fd);
	}
	ASSERT_EQ(fd, -EINVAL, c"reject size with NULL hash pointer".as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn test_map_excl() {
	if test__start_subtest(c"map_excl_allowed".as_ptr()) {
		test_map_excl_allowed();
	}
	if test__start_subtest(c"map_excl_denied".as_ptr()) {
		test_map_excl_denied();
	}
	if test__start_subtest(c"map_excl_no_map_in_map".as_ptr()) {
		test_map_excl_no_map_in_map();
	}
	if test__start_subtest(c"map_excl_no_map_iter".as_ptr()) {
		test_map_excl_no_map_iter();
	}
	if test__start_subtest(c"map_excl_create_validation".as_ptr()) {
		test_map_excl_create_validation();
	}
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
