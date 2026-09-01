// SPDX-License-Identifier: GPL-2.0-only

// C dependencies: <test_progs.h>, "test_lookup_and_delete.skel.h"

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type __u64 = u64;
type bpf_map_type = c_uint;

const START_VALUE: __u64 = 1234;
const NEW_VALUE: __u64 = 4321;
const MAX_ENTRIES: __u64 = 2;

static mut duration: c_int = 0;
static mut nr_cpus: c_int = 0;

#[repr(C)]
struct bpf_map {
	_private: [u8; 0],
}

#[repr(C)]
struct test_lookup_and_delete__maps {
	hash_map: *mut bpf_map,
}

#[repr(C)]
struct test_lookup_and_delete__bss {
	set_pid: c_int,
	set_key: __u64,
	set_value: __u64,
}

#[repr(C)]
struct test_lookup_and_delete {
	maps: test_lookup_and_delete__maps,
	bss: *mut test_lookup_and_delete__bss,
}

unsafe extern "C" {
	fn bpf_map_update_elem(
		fd: c_int,
		key: *const c_void,
		value: *const c_void,
		flags: __u64,
	) -> c_int;
	fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
	fn bpf_map__lookup_and_delete_elem(
		map: *mut bpf_map,
		key: *const c_void,
		key_sz: usize,
		value: *mut c_void,
		value_sz: usize,
		flags: __u64,
	) -> c_int;
	fn bpf_map__set_type(map: *mut bpf_map, type_: bpf_map_type) -> c_int;
	fn bpf_map__set_max_entries(map: *mut bpf_map, max_entries: __u32) -> c_int;
	fn bpf_map__fd(map: *mut bpf_map) -> c_int;
	fn bpf_num_possible_cpus() -> c_int;

	fn test_lookup_and_delete__open() -> *mut test_lookup_and_delete;
	fn test_lookup_and_delete__load(skel: *mut test_lookup_and_delete) -> c_int;
	fn test_lookup_and_delete__attach(skel: *mut test_lookup_and_delete) -> c_int;
	fn test_lookup_and_delete__detach(skel: *mut test_lookup_and_delete);
	fn test_lookup_and_delete__destroy(skel: *mut test_lookup_and_delete);

	fn getpid() -> c_int;
	fn syscall(number: c_long, ...) -> c_long;

	fn test__start_subtest(name: *const c_char) -> bool;
	fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
	fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
	fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
	fn ASSERT_GE(value: c_int, min: c_int, name: *const c_char) -> bool;
	fn CHECK(condition: bool, name: *const c_char, fmt: *const c_char, ...) -> bool;
}

type __u32 = u32;

unsafe fn fill_values(map_fd: c_int) -> c_int {
	let mut key: __u64;
	let value: __u64 = START_VALUE;
	let mut err: c_int;

	key = 1;
	while key < MAX_ENTRIES + 1 {
		err = bpf_map_update_elem(
			map_fd,
			&key as *const _ as *const c_void,
			&value as *const _ as *const c_void,
			BPF_NOEXIST,
		);
		if !ASSERT_OK(err, c"bpf_map_update_elem".as_ptr()) {
			return -1;
		}
		key += 1;
	}

	0
}

unsafe fn fill_values_percpu(map_fd: c_int) -> c_int {
	let mut key: __u64;
	let mut value = vec![0 as __u64; nr_cpus as usize];
	let mut i: c_int;
	let mut err: c_int;

	i = 0;
	while i < nr_cpus {
		value[i as usize] = START_VALUE;
		i += 1;
	}

	key = 1;
	while key < MAX_ENTRIES + 1 {
		err = bpf_map_update_elem(
			map_fd,
			&key as *const _ as *const c_void,
			value.as_ptr() as *const c_void,
			BPF_NOEXIST,
		);
		if !ASSERT_OK(err, c"bpf_map_update_elem".as_ptr()) {
			return -1;
		}
		key += 1;
	}

	0
}

unsafe fn setup_prog(
	map_type: bpf_map_type,
	map_fd: *mut c_int,
) -> *mut test_lookup_and_delete {
	let skel: *mut test_lookup_and_delete;
	let mut err: c_int;

	skel = test_lookup_and_delete__open();
	if !ASSERT_OK_PTR(skel as *const c_void, c"test_lookup_and_delete__open".as_ptr()) {
		return ptr::null_mut();
	}

	err = bpf_map__set_type((*skel).maps.hash_map, map_type);
	if !ASSERT_OK(err, c"bpf_map__set_type".as_ptr()) {
		test_lookup_and_delete__destroy(skel);
		return ptr::null_mut();
	}

	err = bpf_map__set_max_entries((*skel).maps.hash_map, MAX_ENTRIES as __u32);
	if !ASSERT_OK(err, c"bpf_map__set_max_entries".as_ptr()) {
		test_lookup_and_delete__destroy(skel);
		return ptr::null_mut();
	}

	err = test_lookup_and_delete__load(skel);
	if !ASSERT_OK(err, c"test_lookup_and_delete__load".as_ptr()) {
		test_lookup_and_delete__destroy(skel);
		return ptr::null_mut();
	}

	*map_fd = bpf_map__fd((*skel).maps.hash_map);
	if !ASSERT_GE(*map_fd, 0, c"bpf_map__fd".as_ptr()) {
		test_lookup_and_delete__destroy(skel);
		return ptr::null_mut();
	}

	skel
}

/* Triggers BPF program that updates map with given key and value */
unsafe fn trigger_tp(
	skel: *mut test_lookup_and_delete,
	key: __u64,
	value: __u64,
) -> c_int {
	let mut err: c_int;

	(*(*skel).bss).set_pid = getpid();
	(*(*skel).bss).set_key = key;
	(*(*skel).bss).set_value = value;

	err = test_lookup_and_delete__attach(skel);
	if !ASSERT_OK(err, c"test_lookup_and_delete__attach".as_ptr()) {
		return -1;
	}

	syscall(__NR_getpgid);

	test_lookup_and_delete__detach(skel);

	0
}

unsafe fn test_lookup_and_delete_hash() {
	let skel: *mut test_lookup_and_delete;
	let mut key: __u64;
	let mut value: __u64 = 0;
	let mut map_fd: c_int = 0;
	let mut err: c_int;

	/* Setup program and fill the map. */
	skel = setup_prog(BPF_MAP_TYPE_HASH, &mut map_fd);
	if !ASSERT_OK_PTR(skel as *const c_void, c"setup_prog".as_ptr()) {
		return;
	}

	loop {
		err = fill_values(map_fd);
		if !ASSERT_OK(err, c"fill_values".as_ptr()) {
			break;
		}

		/* Lookup and delete element. */
		key = 1;
		err = bpf_map__lookup_and_delete_elem(
			(*skel).maps.hash_map,
			&key as *const _ as *const c_void,
			size_of::<__u64>(),
			&mut value as *mut _ as *mut c_void,
			size_of::<__u64>(),
			0,
		);
		if !ASSERT_OK(err, c"bpf_map_lookup_and_delete_elem".as_ptr()) {
			break;
		}

		/* Fetched value should match the initially set value. */
		if CHECK(
			value != START_VALUE,
			c"bpf_map_lookup_and_delete_elem".as_ptr(),
			c"unexpected value=%lld\n".as_ptr(),
			value,
		) {
			break;
		}

		/* Check that the entry is non existent. */
		err = bpf_map_lookup_elem(
			map_fd,
			&key as *const _ as *const c_void,
			&mut value as *mut _ as *mut c_void,
		);
		if !ASSERT_ERR(err, c"bpf_map_lookup_elem".as_ptr()) {
			break;
		}

		break;
	}

	test_lookup_and_delete__destroy(skel);
}

unsafe fn test_lookup_and_delete_percpu_hash() {
	let skel: *mut test_lookup_and_delete;
	let mut key: __u64;
	let mut val: __u64;
	let mut value = vec![0 as __u64; nr_cpus as usize];
	let mut map_fd: c_int = 0;
	let mut err: c_int;
	let mut i: c_int;

	/* Setup program and fill the map. */
	skel = setup_prog(BPF_MAP_TYPE_PERCPU_HASH, &mut map_fd);
	if !ASSERT_OK_PTR(skel as *const c_void, c"setup_prog".as_ptr()) {
		return;
	}

	loop {
		err = fill_values_percpu(map_fd);
		if !ASSERT_OK(err, c"fill_values_percpu".as_ptr()) {
			break;
		}

		/* Lookup and delete element. */
		key = 1;
		err = bpf_map__lookup_and_delete_elem(
			(*skel).maps.hash_map,
			&key as *const _ as *const c_void,
			size_of::<__u64>(),
			value.as_mut_ptr() as *mut c_void,
			value.len() * size_of::<__u64>(),
			0,
		);
		if !ASSERT_OK(err, c"bpf_map_lookup_and_delete_elem".as_ptr()) {
			break;
		}

		i = 0;
		while i < nr_cpus {
			val = value[i as usize];

			/* Fetched value should match the initially set value. */
			if CHECK(
				val != START_VALUE,
				c"map value".as_ptr(),
				c"unexpected for cpu %d: %lld\n".as_ptr(),
				i,
				val,
			) {
				break;
			}
			i += 1;
		}
		if i < nr_cpus {
			break;
		}

		/* Check that the entry is non existent. */
		err = bpf_map_lookup_elem(
			map_fd,
			&key as *const _ as *const c_void,
			value.as_mut_ptr() as *mut c_void,
		);
		if !ASSERT_ERR(err, c"bpf_map_lookup_elem".as_ptr()) {
			break;
		}

		break;
	}

	test_lookup_and_delete__destroy(skel);
}

unsafe fn test_lookup_and_delete_lru_hash() {
	let skel: *mut test_lookup_and_delete;
	let mut key: __u64;
	let mut value: __u64 = 0;
	let mut map_fd: c_int = 0;
	let mut err: c_int;

	/* Setup program and fill the LRU map. */
	skel = setup_prog(BPF_MAP_TYPE_LRU_HASH, &mut map_fd);
	if !ASSERT_OK_PTR(skel as *const c_void, c"setup_prog".as_ptr()) {
		return;
	}

	loop {
		err = fill_values(map_fd);
		if !ASSERT_OK(err, c"fill_values".as_ptr()) {
			break;
		}

		/* Insert new element at key=3, should reuse LRU element. */
		key = 3;
		err = trigger_tp(skel, key, NEW_VALUE);
		if !ASSERT_OK(err, c"trigger_tp".as_ptr()) {
			break;
		}

		/* Lookup and delete element 3. */
		err = bpf_map__lookup_and_delete_elem(
			(*skel).maps.hash_map,
			&key as *const _ as *const c_void,
			size_of::<__u64>(),
			&mut value as *mut _ as *mut c_void,
			size_of::<__u64>(),
			0,
		);
		if !ASSERT_OK(err, c"bpf_map_lookup_and_delete_elem".as_ptr()) {
			break;
		}

		/* Value should match the new value. */
		if CHECK(
			value != NEW_VALUE,
			c"bpf_map_lookup_and_delete_elem".as_ptr(),
			c"unexpected value=%lld\n".as_ptr(),
			value,
		) {
			break;
		}

		/* Check that entries 3 and 1 are non existent. */
		err = bpf_map_lookup_elem(
			map_fd,
			&key as *const _ as *const c_void,
			&mut value as *mut _ as *mut c_void,
		);
		if !ASSERT_ERR(err, c"bpf_map_lookup_elem".as_ptr()) {
			break;
		}

		key = 1;
		err = bpf_map_lookup_elem(
			map_fd,
			&key as *const _ as *const c_void,
			&mut value as *mut _ as *mut c_void,
		);
		if !ASSERT_ERR(err, c"bpf_map_lookup_elem".as_ptr()) {
			break;
		}

		break;
	}

	test_lookup_and_delete__destroy(skel);
}

unsafe fn test_lookup_and_delete_lru_percpu_hash() {
	let skel: *mut test_lookup_and_delete;
	let mut key: __u64;
	let mut val: __u64;
	let mut value = vec![0 as __u64; nr_cpus as usize];
	let mut map_fd: c_int = 0;
	let mut err: c_int;
	let mut i: c_int;
	let mut cpucnt: c_int = 0;

	/* Setup program and fill the LRU map. */
	skel = setup_prog(BPF_MAP_TYPE_LRU_PERCPU_HASH, &mut map_fd);
	if !ASSERT_OK_PTR(skel as *const c_void, c"setup_prog".as_ptr()) {
		return;
	}

	loop {
		err = fill_values_percpu(map_fd);
		if !ASSERT_OK(err, c"fill_values_percpu".as_ptr()) {
			break;
		}

		/* Insert new element at key=3, should reuse LRU element 1. */
		key = 3;
		err = trigger_tp(skel, key, NEW_VALUE);
		if !ASSERT_OK(err, c"trigger_tp".as_ptr()) {
			break;
		}

		/* Clean value. */
		i = 0;
		while i < nr_cpus {
			value[i as usize] = 0;
			i += 1;
		}

		/* Lookup and delete element 3. */
		err = bpf_map__lookup_and_delete_elem(
			(*skel).maps.hash_map,
			&key as *const _ as *const c_void,
			size_of::<__u64>(),
			value.as_mut_ptr() as *mut c_void,
			value.len() * size_of::<__u64>(),
			0,
		);
		if !ASSERT_OK(err, c"bpf_map_lookup_and_delete_elem".as_ptr()) {
			break;
		}

		/* Check if only one CPU has set the value. */
		i = 0;
		while i < nr_cpus {
			val = value[i as usize];
			if val != 0 {
				if CHECK(
					val != NEW_VALUE,
					c"map value".as_ptr(),
					c"unexpected for cpu %d: %lld\n".as_ptr(),
					i,
					val,
				) {
					break;
				}
				cpucnt += 1;
			}
			i += 1;
		}
		if i < nr_cpus {
			break;
		}
		if CHECK(
			cpucnt != 1,
			c"map value".as_ptr(),
			c"set for %d CPUs instead of 1!\n".as_ptr(),
			cpucnt,
		) {
			break;
		}

		/* Check that entries 3 and 1 are non existent. */
		err = bpf_map_lookup_elem(
			map_fd,
			&key as *const _ as *const c_void,
			value.as_mut_ptr() as *mut c_void,
		);
		if !ASSERT_ERR(err, c"bpf_map_lookup_elem".as_ptr()) {
			break;
		}

		key = 1;
		err = bpf_map_lookup_elem(
			map_fd,
			&key as *const _ as *const c_void,
			value.as_mut_ptr() as *mut c_void,
		);
		if !ASSERT_ERR(err, c"bpf_map_lookup_elem".as_ptr()) {
			break;
		}

		break;
	}

	test_lookup_and_delete__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_lookup_and_delete() {
	nr_cpus = bpf_num_possible_cpus();

	if test__start_subtest(c"lookup_and_delete".as_ptr()) {
		test_lookup_and_delete_hash();
	}
	if test__start_subtest(c"lookup_and_delete_percpu".as_ptr()) {
		test_lookup_and_delete_percpu_hash();
	}
	if test__start_subtest(c"lookup_and_delete_lru".as_ptr()) {
		test_lookup_and_delete_lru_hash();
	}
	if test__start_subtest(c"lookup_and_delete_lru_percpu".as_ptr()) {
		test_lookup_and_delete_lru_percpu_hash();
	}
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
