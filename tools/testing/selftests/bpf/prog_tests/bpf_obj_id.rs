// SPDX-License-Identifier: GPL-2.0
// Translated from C source using declarations supplied by test_progs.h and
// related libbpf/kernel headers as external dependencies.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

const nr_iters: usize = 2;

type __u32 = u32;
type __u64 = u64;
type uid_t = c_uint;
type time_t = c_long;

const ENOENT: c_int = 2;
const EFAULT: c_int = 14;
const CLOCK_REALTIME: c_int = 0;
const CLOCK_BOOTTIME: c_int = 7;
const BPF_PROG_TYPE_RAW_TRACEPOINT: __u32 = 17;
const BPF_MAP_TYPE_ARRAY: __u32 = 2;
const BPF_LINK_TYPE_RAW_TRACEPOINT: __u32 = 1;

#[repr(C)]
pub struct bpf_object {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
	_private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timespec {
	pub tv_sec: time_t,
	pub tv_nsec: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_map_info {
	pub type_: __u32,
	pub id: __u32,
	pub key_size: __u32,
	pub value_size: __u32,
	pub max_entries: __u32,
	pub map_flags: __u32,
	pub name: [c_char; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_prog_info {
	pub type_: __u32,
	pub id: __u32,
	pub tag: [u8; 8],
	pub jited_prog_len: __u32,
	pub xlated_prog_len: __u32,
	pub jited_prog_insns: __u64,
	pub xlated_prog_insns: __u64,
	pub load_time: __u64,
	pub created_by_uid: __u32,
	pub nr_map_ids: __u32,
	pub map_ids: __u64,
	pub name: [c_char; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_link_info_raw_tracepoint {
	pub tp_name: __u64,
	pub tp_name_len: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_link_info {
	pub type_: __u32,
	pub id: __u32,
	pub prog_id: __u32,
	pub raw_tracepoint: bpf_link_info_raw_tracepoint,
}

#[repr(C)]
pub struct test_env {
	pub jit_enabled: bool,
}

unsafe extern "C" {
	static mut errno: c_int;
	static mut env: test_env;

	fn getuid() -> uid_t;
	fn time(tloc: *mut time_t) -> time_t;
	fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
	fn close(fd: c_int) -> c_int;
	fn bzero(s: *mut c_void, n: usize);
	fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
	fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;

	fn bpf_prog_get_fd_by_id(id: __u32) -> c_int;
	fn bpf_map_get_fd_by_id(id: __u32) -> c_int;
	fn bpf_link_get_fd_by_id(id: __u32) -> c_int;
	fn bpf_prog_get_next_id(start_id: __u32, next_id: *mut __u32) -> c_int;
	fn bpf_map_get_next_id(start_id: __u32, next_id: *mut __u32) -> c_int;
	fn bpf_link_get_next_id(start_id: __u32, next_id: *mut __u32) -> c_int;
	fn bpf_prog_get_info_by_fd(fd: c_int, info: *mut bpf_prog_info, info_len: *mut __u32) -> c_int;
	fn bpf_map_get_info_by_fd(fd: c_int, info: *mut bpf_map_info, info_len: *mut __u32) -> c_int;
	fn bpf_link_get_info_by_fd(fd: c_int, info: *mut bpf_link_info, info_len: *mut __u32) -> c_int;
	fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: __u64) -> c_int;
	fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
	fn bpf_prog_test_load(
		file: *const c_char,
		prog_type: __u32,
		pobj: *mut *mut bpf_object,
		prog_fd: *mut c_int,
	) -> c_int;
	fn bpf_find_map(test: *const c_char, obj: *mut bpf_object, name: *const c_char) -> c_int;
	fn bpf_object__find_program_by_name(obj: *mut bpf_object, name: *const c_char) -> *mut bpf_program;
	fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
	fn libbpf_get_error(ptr: *const c_void) -> c_int;
	fn bpf_link__fd(link: *mut bpf_link) -> c_int;
	fn bpf_link__destroy(link: *mut bpf_link);
	fn bpf_object__close(obj: *mut bpf_object);

	fn ASSERT_LT(actual: c_int, expected: c_int, name: *const c_char) -> bool;
	fn ASSERT_EQ_U32(actual: __u32, expected: __u32, name: *const c_char) -> bool;
	fn ASSERT_EQ_U64(actual: __u64, expected: __u64, name: *const c_char) -> bool;
	fn ASSERT_EQ_INT(actual: c_int, expected: c_int, name: *const c_char) -> bool;
	fn ASSERT_EQ_UID(actual: uid_t, expected: uid_t, name: *const c_char) -> bool;
	fn ASSERT_OK(actual: c_int, name: *const c_char) -> bool;
	fn ASSERT_ERR(actual: c_int, name: *const c_char) -> bool;
	fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
	fn ASSERT_LE_TIME(actual: time_t, expected: time_t, name: *const c_char) -> bool;
	fn ASSERT_GE_TIME(actual: time_t, expected: time_t, name: *const c_char) -> bool;
	fn ASSERT_FALSE(actual: bool, name: *const c_char) -> bool;
	fn ASSERT_NEQ_U32(actual: __u32, expected: __u32, name: *const c_char) -> bool;
	fn ASSERT_NEQ_INT(actual: c_int, expected: c_int, name: *const c_char) -> bool;
	fn ASSERT_STREQ(actual: *const c_char, expected: *const c_char, name: *const c_char) -> bool;
	fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
}

#[inline]
fn ptr_to_u64<T>(ptr: *const T) -> __u64 {
	ptr as usize as __u64
}

#[inline]
fn u64_to_ptr<T>(ptr: __u64) -> *mut T {
	ptr as usize as *mut T
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn serial_test_bpf_obj_id() {
	let array_magic_value: __u64 = 0xfaceb00c;
	let array_key: __u32 = 0;
	let file = c"./test_obj_id.bpf.o".as_ptr();
	let expected_prog_name = c"test_obj_id".as_ptr();
	let expected_map_name = c"test_map_id".as_ptr();
	let nsec_per_sec: __u64 = 1000000000;

	let mut objs: [*mut bpf_object; nr_iters] = [ptr::null_mut(); nr_iters];
	let mut links: [*mut bpf_link; nr_iters] = [ptr::null_mut(); nr_iters];
	let mut prog: *mut bpf_program;
	let mut prog_fds: [c_int; nr_iters] = [0; nr_iters];
	let mut map_fds: [c_int; nr_iters] = [0; nr_iters];
	/* +1 to test for the info_len returned by kernel */
	let mut prog_infos: [bpf_prog_info; nr_iters + 1] =
		[core::mem::zeroed::<bpf_prog_info>(); nr_iters + 1];
	let mut map_infos: [bpf_map_info; nr_iters + 1] =
		[core::mem::zeroed::<bpf_map_info>(); nr_iters + 1];
	let mut link_infos: [bpf_link_info; nr_iters + 1] =
		[core::mem::zeroed::<bpf_link_info>(); nr_iters + 1];
	/* Each prog only uses one map. +1 to test nr_map_ids
	 * returned by kernel.
	 */
	let mut map_ids: [__u32; nr_iters + 1] = [0; nr_iters + 1];
	let mut jited_insns = [0 as c_char; 128];
	let mut xlated_insns = [0 as c_char; 128];
	let mut zeros = [0 as c_char; 128];
	let mut tp_name = [0 as c_char; 128];
	let mut i: __u32;
	let mut next_id: __u32;
	let mut info_len: __u32;
	let mut nr_id_found: __u32;
	let mut real_time_ts: timespec = core::mem::zeroed();
	let mut boot_time_ts: timespec = core::mem::zeroed();
	let mut err: c_int = 0;
	let mut array_value: __u64 = 0;
	let my_uid: uid_t = getuid();
	let mut now: time_t;
	let mut load_time: time_t;

	err = bpf_prog_get_fd_by_id(0);
	ASSERT_LT(err, 0, c"bpf_prog_get_fd_by_id".as_ptr());
	ASSERT_EQ_INT(errno, ENOENT, c"bpf_prog_get_fd_by_id".as_ptr());

	err = bpf_map_get_fd_by_id(0);
	ASSERT_LT(err, 0, c"bpf_map_get_fd_by_id".as_ptr());
	ASSERT_EQ_INT(errno, ENOENT, c"bpf_map_get_fd_by_id".as_ptr());

	err = bpf_link_get_fd_by_id(0);
	ASSERT_LT(err, 0, c"bpf_map_get_fd_by_id".as_ptr());
	ASSERT_EQ_INT(errno, ENOENT, c"bpf_map_get_fd_by_id".as_ptr());

	/* Check bpf_map_get_info_by_fd() */
	bzero(zeros.as_mut_ptr() as *mut c_void, size_of_val(&zeros));
	i = 0;
	while (i as usize) < nr_iters {
		now = time(ptr::null_mut());
		err = bpf_prog_test_load(
			file,
			BPF_PROG_TYPE_RAW_TRACEPOINT,
			&mut objs[i as usize],
			&mut prog_fds[i as usize],
		);
		/* test_obj_id.o is a dumb prog. It should never fail
		 * to load.
		 */
		if !ASSERT_OK(err, c"bpf_prog_test_load".as_ptr()) {
			i += 1;
			continue;
		}

		/* Insert a magic value to the map */
		map_fds[i as usize] = bpf_find_map(c"serial_test_bpf_obj_id".as_ptr(), objs[i as usize], c"test_map_id".as_ptr());
		if !ASSERT_GE(map_fds[i as usize], 0, c"bpf_find_map".as_ptr()) {
			break;
		}

		err = bpf_map_update_elem(
			map_fds[i as usize],
			&array_key as *const _ as *const c_void,
			&array_magic_value as *const _ as *const c_void,
			0,
		);
		if !ASSERT_OK(err, c"bpf_map_update_elem".as_ptr()) {
			break;
		}

		prog = bpf_object__find_program_by_name(objs[i as usize], c"test_obj_id".as_ptr());
		if !ASSERT_OK_PTR(prog as *const c_void, c"bpf_object__find_program_by_name".as_ptr()) {
			break;
		}

		links[i as usize] = bpf_program__attach(prog);
		err = libbpf_get_error(links[i as usize] as *const c_void);
		if !ASSERT_OK(err, c"bpf_program__attach".as_ptr()) {
			links[i as usize] = ptr::null_mut();
			break;
		}

		/* Check getting map info */
		info_len = (size_of::<bpf_map_info>() * 2) as __u32;
		bzero(
			&mut map_infos[i as usize] as *mut _ as *mut c_void,
			info_len as usize,
		);
		err = bpf_map_get_info_by_fd(
			map_fds[i as usize],
			&mut map_infos[i as usize],
			&mut info_len,
		);
		if !ASSERT_OK(err, c"bpf_map_get_info_by_fd".as_ptr())
			|| !ASSERT_EQ_U32(map_infos[i as usize].type_, BPF_MAP_TYPE_ARRAY, c"map_type".as_ptr())
			|| !ASSERT_EQ_U32(map_infos[i as usize].key_size, size_of::<__u32>() as __u32, c"key_size".as_ptr())
			|| !ASSERT_EQ_U32(map_infos[i as usize].value_size, size_of::<__u64>() as __u32, c"value_size".as_ptr())
			|| !ASSERT_EQ_U32(map_infos[i as usize].max_entries, 1, c"max_entries".as_ptr())
			|| !ASSERT_EQ_U32(map_infos[i as usize].map_flags, 0, c"map_flags".as_ptr())
			|| !ASSERT_EQ_U32(info_len, size_of::<bpf_map_info>() as __u32, c"map_info_len".as_ptr())
			|| !ASSERT_STREQ(map_infos[i as usize].name.as_ptr(), expected_map_name, c"map_name".as_ptr())
		{
			break;
		}

		/* Check getting prog info */
		info_len = (size_of::<bpf_prog_info>() * 2) as __u32;
		bzero(
			&mut prog_infos[i as usize] as *mut _ as *mut c_void,
			info_len as usize,
		);
		bzero(jited_insns.as_mut_ptr() as *mut c_void, size_of_val(&jited_insns));
		bzero(xlated_insns.as_mut_ptr() as *mut c_void, size_of_val(&xlated_insns));
		prog_infos[i as usize].jited_prog_insns = ptr_to_u64(jited_insns.as_mut_ptr());
		prog_infos[i as usize].jited_prog_len = size_of_val(&jited_insns) as __u32;
		prog_infos[i as usize].xlated_prog_insns = ptr_to_u64(xlated_insns.as_mut_ptr());
		prog_infos[i as usize].xlated_prog_len = size_of_val(&xlated_insns) as __u32;
		prog_infos[i as usize].map_ids = ptr_to_u64(map_ids.as_mut_ptr().add(i as usize));
		prog_infos[i as usize].nr_map_ids = 2;

		err = clock_gettime(CLOCK_REALTIME, &mut real_time_ts);
		if !ASSERT_OK(err, c"clock_gettime".as_ptr()) {
			break;
		}

		err = clock_gettime(CLOCK_BOOTTIME, &mut boot_time_ts);
		if !ASSERT_OK(err, c"clock_gettime".as_ptr()) {
			break;
		}

		err = bpf_prog_get_info_by_fd(prog_fds[i as usize], &mut prog_infos[i as usize], &mut info_len);
		load_time = (real_time_ts.tv_sec - boot_time_ts.tv_sec)
			+ (prog_infos[i as usize].load_time / nsec_per_sec) as time_t;

		if !ASSERT_OK(err, c"bpf_prog_get_info_by_fd".as_ptr())
			|| !ASSERT_EQ_U32(prog_infos[i as usize].type_, BPF_PROG_TYPE_RAW_TRACEPOINT, c"prog_type".as_ptr())
			|| !ASSERT_EQ_U32(info_len, size_of::<bpf_prog_info>() as __u32, c"prog_info_len".as_ptr())
			|| !ASSERT_FALSE(env.jit_enabled && prog_infos[i as usize].jited_prog_len == 0, c"jited_prog_len".as_ptr())
			|| !ASSERT_FALSE(
				env.jit_enabled
					&& memcmp(
						jited_insns.as_ptr() as *const c_void,
						zeros.as_ptr() as *const c_void,
						size_of_val(&zeros),
					) == 0,
				c"jited_insns".as_ptr(),
			)
			|| !ASSERT_NEQ_U32(prog_infos[i as usize].xlated_prog_len, 0, c"xlated_prog_len".as_ptr())
			|| !ASSERT_NEQ_INT(
				memcmp(
					xlated_insns.as_ptr() as *const c_void,
					zeros.as_ptr() as *const c_void,
					size_of_val(&zeros),
				),
				0,
				c"xlated_insns".as_ptr(),
			)
			|| !ASSERT_GE_TIME(load_time, now - 60, c"load_time".as_ptr())
			|| !ASSERT_LE_TIME(load_time, now + 60, c"load_time".as_ptr())
			|| !ASSERT_EQ_UID(prog_infos[i as usize].created_by_uid, my_uid, c"created_by_uid".as_ptr())
			|| !ASSERT_EQ_U32(prog_infos[i as usize].nr_map_ids, 1, c"nr_map_ids".as_ptr())
			|| !ASSERT_EQ_INT(
				*(prog_infos[i as usize].map_ids as usize as *const c_int),
				map_infos[i as usize].id as c_int,
				c"map_ids".as_ptr(),
			)
			|| !ASSERT_STREQ(prog_infos[i as usize].name.as_ptr(), expected_prog_name, c"prog_name".as_ptr())
		{
			break;
		}

		/* Check getting link info */
		info_len = (size_of::<bpf_link_info>() * 2) as __u32;
		bzero(
			&mut link_infos[i as usize] as *mut _ as *mut c_void,
			info_len as usize,
		);
		link_infos[i as usize].raw_tracepoint.tp_name = ptr_to_u64(&mut tp_name);
		link_infos[i as usize].raw_tracepoint.tp_name_len = size_of_val(&tp_name) as __u32;
		err = bpf_link_get_info_by_fd(
			bpf_link__fd(links[i as usize]),
			&mut link_infos[i as usize],
			&mut info_len,
		);
		if !ASSERT_OK(err, c"bpf_link_get_info_by_fd".as_ptr())
			|| !ASSERT_EQ_U32(link_infos[i as usize].type_, BPF_LINK_TYPE_RAW_TRACEPOINT, c"link_type".as_ptr())
			|| !ASSERT_EQ_U32(link_infos[i as usize].prog_id, prog_infos[i as usize].id, c"prog_id".as_ptr())
			|| !ASSERT_EQ_U64(link_infos[i as usize].raw_tracepoint.tp_name, ptr_to_u64(&mut tp_name), c"&tp_name".as_ptr())
			|| !ASSERT_STREQ(
				u64_to_ptr::<c_char>(link_infos[i as usize].raw_tracepoint.tp_name),
				c"sys_enter".as_ptr(),
				c"tp_name".as_ptr(),
			)
		{
			break;
		}

		i += 1;
	}

	/* Check bpf_prog_get_next_id() */
	nr_id_found = 0;
	next_id = 0;
	while bpf_prog_get_next_id(next_id, &mut next_id) == 0 {
		let mut prog_info: bpf_prog_info = core::mem::zeroed();
		let saved_map_id: __u32;
		let prog_fd: c_int;
		let cmp_res: c_int;

		info_len = size_of_val(&prog_info) as __u32;

		prog_fd = bpf_prog_get_fd_by_id(next_id);
		if prog_fd < 0 && errno == ENOENT {
			/* The bpf_prog is in the dead row */
			continue;
		}
		if !ASSERT_GE(prog_fd, 0, c"bpf_prog_get_fd_by_id".as_ptr()) {
			break;
		}

		i = 0;
		while (i as usize) < nr_iters {
			if prog_infos[i as usize].id == next_id {
				break;
			}
			i += 1;
		}

		if (i as usize) == nr_iters {
			continue;
		}

		nr_id_found += 1;

		/* Negative test:
		 * prog_info.nr_map_ids = 1
		 * prog_info.map_ids = NULL
		 */
		prog_info.nr_map_ids = 1;
		err = bpf_prog_get_info_by_fd(prog_fd, &mut prog_info, &mut info_len);
		if !ASSERT_ERR(err, c"bpf_prog_get_info_by_fd".as_ptr())
			|| !ASSERT_EQ_INT(errno, EFAULT, c"bpf_prog_get_info_by_fd".as_ptr())
		{
			break;
		}
		bzero(&mut prog_info as *mut _ as *mut c_void, size_of_val(&prog_info));
		info_len = size_of_val(&prog_info) as __u32;

		saved_map_id = *(prog_infos[i as usize].map_ids as usize as *const c_int) as __u32;
		prog_info.map_ids = prog_infos[i as usize].map_ids;
		prog_info.nr_map_ids = 2;
		err = bpf_prog_get_info_by_fd(prog_fd, &mut prog_info, &mut info_len);
		prog_infos[i as usize].jited_prog_insns = 0;
		prog_infos[i as usize].xlated_prog_insns = 0;
		cmp_res = memcmp(
			&prog_info as *const _ as *const c_void,
			&prog_infos[i as usize] as *const _ as *const c_void,
			info_len as usize,
		);

		ASSERT_OK(err, c"bpf_prog_get_info_by_fd".as_ptr());
		ASSERT_EQ_U32(info_len, size_of::<bpf_prog_info>() as __u32, c"prog_info_len".as_ptr());
		ASSERT_OK(cmp_res, c"memcmp".as_ptr());
		ASSERT_EQ_INT(
			*(prog_info.map_ids as usize as *const c_int),
			saved_map_id as c_int,
			c"map_id".as_ptr(),
		);
		close(prog_fd);
	}
	ASSERT_EQ_U32(nr_id_found, nr_iters as __u32, c"prog_nr_id_found".as_ptr());

	/* Check bpf_map_get_next_id() */
	nr_id_found = 0;
	next_id = 0;
	while bpf_map_get_next_id(next_id, &mut next_id) == 0 {
		let mut map_info: bpf_map_info = core::mem::zeroed();
		let map_fd: c_int;
		let cmp_res: c_int;

		info_len = size_of_val(&map_info) as __u32;

		map_fd = bpf_map_get_fd_by_id(next_id);
		if map_fd < 0 && errno == ENOENT {
			/* The bpf_map is in the dead row */
			continue;
		}
		if !ASSERT_GE(map_fd, 0, c"bpf_map_get_fd_by_id".as_ptr()) {
			break;
		}

		i = 0;
		while (i as usize) < nr_iters {
			if map_infos[i as usize].id == next_id {
				break;
			}
			i += 1;
		}

		if (i as usize) == nr_iters {
			continue;
		}

		nr_id_found += 1;

		err = bpf_map_lookup_elem(
			map_fd,
			&array_key as *const _ as *const c_void,
			&mut array_value as *mut _ as *mut c_void,
		);
		if !ASSERT_OK(err, c"bpf_map_lookup_elem".as_ptr()) {
			break;
		}

		err = bpf_map_get_info_by_fd(map_fd, &mut map_info, &mut info_len);
		cmp_res = memcmp(
			&map_info as *const _ as *const c_void,
			&map_infos[i as usize] as *const _ as *const c_void,
			info_len as usize,
		);
		ASSERT_OK(err, c"bpf_map_get_info_by_fd".as_ptr());
		ASSERT_EQ_U32(info_len, size_of::<bpf_map_info>() as __u32, c"info_len".as_ptr());
		ASSERT_OK(cmp_res, c"memcmp".as_ptr());
		ASSERT_EQ_U64(array_value, array_magic_value, c"array_value".as_ptr());

		close(map_fd);
	}
	ASSERT_EQ_U32(nr_id_found, nr_iters as __u32, c"map_nr_id_found".as_ptr());

	/* Check bpf_link_get_next_id() */
	nr_id_found = 0;
	next_id = 0;
	while bpf_link_get_next_id(next_id, &mut next_id) == 0 {
		let mut link_info: bpf_link_info = core::mem::zeroed();
		let link_fd: c_int;
		let cmp_res: c_int;

		info_len = size_of_val(&link_info) as __u32;
		memset(&mut link_info as *mut _ as *mut c_void, 0, info_len as usize);

		link_fd = bpf_link_get_fd_by_id(next_id);
		if link_fd < 0 && errno == ENOENT {
			/* The bpf_link is in the dead row */
			continue;
		}
		if !ASSERT_GE(link_fd, 0, c"bpf_link_get_fd_by_id".as_ptr()) {
			break;
		}

		i = 0;
		while (i as usize) < nr_iters {
			if link_infos[i as usize].id == next_id {
				break;
			}
			i += 1;
		}

		if (i as usize) == nr_iters {
			continue;
		}

		nr_id_found += 1;

		err = bpf_link_get_info_by_fd(link_fd, &mut link_info, &mut info_len);
		cmp_res = memcmp(
			&link_info as *const _ as *const c_void,
			&link_infos[i as usize] as *const _ as *const c_void,
			offset_of!(bpf_link_info, raw_tracepoint),
		);
		ASSERT_OK(err, c"bpf_link_get_info_by_fd".as_ptr());
		ASSERT_EQ_U32(info_len, size_of_val(&link_info) as __u32, c"info_len".as_ptr());
		ASSERT_OK(cmp_res, c"memcmp".as_ptr());

		close(link_fd);
	}
	ASSERT_EQ_U32(nr_id_found, nr_iters as __u32, c"link_nr_id_found".as_ptr());

	i = 0;
	while (i as usize) < nr_iters {
		bpf_link__destroy(links[i as usize]);
		bpf_object__close(objs[i as usize]);
		i += 1;
	}
}

#[inline]
fn size_of_val<T>(val: &T) -> usize {
	size_of::<T>()
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
