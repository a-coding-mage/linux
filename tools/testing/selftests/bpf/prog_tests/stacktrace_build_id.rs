// SPDX-License-Identifier: GPL-2.0
// Translated from C source:
//   testing/selftests/bpf/prog_tests/stacktrace_build_id.c
//
// C dependencies removed from executable Rust:
//   #include <test_progs.h>
//   #include "test_stacktrace_build_id.skel.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr::{null, null_mut};

type __u32 = u32;

const BPF_BUILD_ID_SIZE: usize = 20;
const PERF_MAX_STACK_DEPTH: usize = 127;
const BPF_STACK_BUILD_ID_VALID: c_int = 1;

#[repr(C)]
pub struct bpf_map {
	_private: [u8; 0],
}

#[repr(C)]
pub struct test_stacktrace_build_id__maps {
	pub control_map: *mut bpf_map,
	pub stackid_hmap: *mut bpf_map,
	pub stackmap: *mut bpf_map,
	pub stack_amap: *mut bpf_map,
}

#[repr(C)]
pub struct test_stacktrace_build_id {
	pub maps: test_stacktrace_build_id__maps,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_stack_build_id {
	pub status: c_int,
	pub build_id: [u8; BPF_BUILD_ID_SIZE],
	pub offset: u64,
}

unsafe extern "C" {
	fn test_stacktrace_build_id__open_and_load() -> *mut test_stacktrace_build_id;
	fn test_stacktrace_build_id__attach(skel: *mut test_stacktrace_build_id) -> c_int;
	fn test_stacktrace_build_id__destroy(skel: *mut test_stacktrace_build_id);

	fn bpf_map__fd(map: *mut bpf_map) -> c_int;
	fn bpf_map__get_next_key(
		map: *mut bpf_map,
		key: *const c_void,
		next_key: *mut c_void,
		key_sz: usize,
	) -> c_int;
	fn bpf_map_update_elem(
		fd: c_int,
		key: *const c_void,
		value: *const c_void,
		flags: u64,
	) -> c_int;
	fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;

	fn compare_map_keys(map1_fd: c_int, map2_fd: c_int) -> c_int;
	fn compare_stack_ips(map1_fd: c_int, map2_fd: c_int, stack_trace_len: c_int) -> c_int;
	fn read_build_id(file: *const c_char, buf: *mut c_char, size: usize) -> c_int;

	fn system(command: *const c_char) -> c_int;
	fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
	fn printf(format: *const c_char, ...) -> c_int;

	static mut errno: c_int;

	// test_progs.h supplies CHECK/CHECK_FAIL as test macros in C.
	fn CHECK(condition: bool, tag: *const c_char, format: *const c_char, ...) -> bool;
	fn CHECK_FAIL(condition: c_int) -> bool;
}

pub unsafe fn test_stacktrace_build_id() {
	let control_map_fd: c_int;
	let stackid_hmap_fd: c_int;
	let stackmap_fd: c_int;
	let stack_amap_fd: c_int;
	let mut skel: *mut test_stacktrace_build_id;
	let mut err: c_int;
	let stack_trace_len: c_int;
	let build_id_size: c_int;
	let mut key: __u32;
	let mut prev_key: __u32;
	let mut val: __u32;
	let _duration: __u32 = 0;
	let mut buf = [0 as c_char; BPF_BUILD_ID_SIZE];
	let mut id_offs = [bpf_stack_build_id {
		status: 0,
		build_id: [0; BPF_BUILD_ID_SIZE],
		offset: 0,
	}; PERF_MAX_STACK_DEPTH];
	let mut build_id_matches: c_int = 0;
	let mut i: c_int;
	let mut retry: c_int = 1;

	loop {
		skel = test_stacktrace_build_id__open_and_load();
		if CHECK(
			skel.is_null(),
			c"skel_open_and_load".as_ptr(),
			c"skeleton open/load failed\n".as_ptr(),
		) {
			return;
		}

		err = test_stacktrace_build_id__attach(skel);
		if CHECK(err != 0, c"attach_tp".as_ptr(), c"err %d\n".as_ptr(), err) {
			goto_cleanup(skel);
			return;
		}

		/* find map fds */
		control_map_fd = bpf_map__fd((*skel).maps.control_map);
		stackid_hmap_fd = bpf_map__fd((*skel).maps.stackid_hmap);
		stackmap_fd = bpf_map__fd((*skel).maps.stackmap);
		stack_amap_fd = bpf_map__fd((*skel).maps.stack_amap);

		if CHECK_FAIL(system(c"dd if=/dev/urandom of=/dev/zero count=4 2> /dev/null".as_ptr())) {
			goto_cleanup(skel);
			return;
		}
		if CHECK_FAIL(system(c"./urandom_read".as_ptr())) {
			goto_cleanup(skel);
			return;
		}
		/* disable stack trace collection */
		key = 0;
		val = 1;
		bpf_map_update_elem(
			control_map_fd,
			&key as *const __u32 as *const c_void,
			&val as *const __u32 as *const c_void,
			0,
		);

		/* for every element in stackid_hmap, we can find a corresponding one
		 * in stackmap, and vice versa.
		 */
		err = compare_map_keys(stackid_hmap_fd, stackmap_fd);
		if CHECK(
			err != 0,
			c"compare_map_keys stackid_hmap vs. stackmap".as_ptr(),
			c"err %d errno %d\n".as_ptr(),
			err,
			errno,
		) {
			goto_cleanup(skel);
			return;
		}

		err = compare_map_keys(stackmap_fd, stackid_hmap_fd);
		if CHECK(
			err != 0,
			c"compare_map_keys stackmap vs. stackid_hmap".as_ptr(),
			c"err %d errno %d\n".as_ptr(),
			err,
			errno,
		) {
			goto_cleanup(skel);
			return;
		}

		build_id_size = read_build_id(
			c"urandom_read".as_ptr(),
			buf.as_mut_ptr(),
			size_of_val(&buf),
		);
		err = if build_id_size < 0 { build_id_size } else { 0 };

		if CHECK(
			err != 0,
			c"read_build_id".as_ptr(),
			c"err %d errno %d\n".as_ptr(),
			err,
			errno,
		) {
			goto_cleanup(skel);
			return;
		}

		err = bpf_map__get_next_key(
			(*skel).maps.stackmap,
			null(),
			&mut key as *mut __u32 as *mut c_void,
			size_of::<__u32>(),
		);
		if CHECK(
			err != 0,
			c"get_next_key from stackmap".as_ptr(),
			c"err %d, errno %d\n".as_ptr(),
			err,
			errno,
		) {
			goto_cleanup(skel);
			return;
		}

		loop {
			err = bpf_map_lookup_elem(
				stackmap_fd,
				&key as *const __u32 as *const c_void,
				id_offs.as_mut_ptr() as *mut c_void,
			);
			if CHECK(
				err != 0,
				c"lookup_elem from stackmap".as_ptr(),
				c"err %d, errno %d\n".as_ptr(),
				err,
				errno,
			) {
				goto_cleanup(skel);
				return;
			}
			i = 0;
			while i < PERF_MAX_STACK_DEPTH as c_int {
				if id_offs[i as usize].status == BPF_STACK_BUILD_ID_VALID
					&& id_offs[i as usize].offset != 0
				{
					if memcmp(
						buf.as_ptr() as *const c_void,
						id_offs[i as usize].build_id.as_ptr() as *const c_void,
						build_id_size as usize,
					) == 0
					{
						build_id_matches = 1;
					}
				}
				i += 1;
			}
			prev_key = key;
			if bpf_map__get_next_key(
				(*skel).maps.stackmap,
				&prev_key as *const __u32 as *const c_void,
				&mut key as *mut __u32 as *mut c_void,
				size_of::<__u32>(),
			) != 0
			{
				break;
			}
		}

		/* stack_map_get_build_id_offset() is racy and sometimes can return
		 * BPF_STACK_BUILD_ID_IP instead of BPF_STACK_BUILD_ID_VALID;
		 * try it one more time.
		 */
		if build_id_matches < 1 && retry != 0 {
			retry -= 1;
			test_stacktrace_build_id__destroy(skel);
			printf(
				c"%s:WARN:Didn't find expected build ID from the map, retrying\n".as_ptr(),
				c"test_stacktrace_build_id".as_ptr(),
			);
			continue;
		}

		if CHECK(
			build_id_matches < 1,
			c"build id match".as_ptr(),
			c"Didn't find expected build ID from the map\n".as_ptr(),
		) {
			goto_cleanup(skel);
			return;
		}

		stack_trace_len =
			(PERF_MAX_STACK_DEPTH * size_of::<bpf_stack_build_id>()) as c_int;
		err = compare_stack_ips(stackmap_fd, stack_amap_fd, stack_trace_len);
		CHECK(
			err != 0,
			c"compare_stack_ips stackmap vs. stack_amap".as_ptr(),
			c"err %d errno %d\n".as_ptr(),
			err,
			errno,
		);

		goto_cleanup(skel);
		return;
	}
}

unsafe fn goto_cleanup(skel: *mut test_stacktrace_build_id) {
	test_stacktrace_build_id__destroy(skel);
}

fn size_of_val<T>(val: &T) -> usize {
	core::mem::size_of_val(val)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
