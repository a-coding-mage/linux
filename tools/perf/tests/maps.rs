// SPDX-License-Identifier: GPL-2.0
// Translated from perf/tests/maps.c.
// C dependencies: inttypes.h, linux/compiler.h, linux/kernel.h, tests.h,
// map.h, maps.h, dso.h, debug.h.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type u64 = u64;

const TEST_OK: c_int = 0;
const TEST_FAIL: c_int = -1;

#[repr(C)]
pub struct map {
	_private: [u8; 0],
}

#[repr(C)]
pub struct maps {
	_private: [u8; 0],
}

#[repr(C)]
pub struct dso {
	_private: [u8; 0],
}

#[repr(C)]
pub struct refcount_t {
	_private: [u8; 0],
}

#[repr(C)]
pub struct test_suite {
	desc: *const c_char,
	test_cases: *mut test_case,
}

#[repr(C)]
pub struct test_case {
	name: *const c_char,
	run_case: Option<unsafe extern "C" fn(*mut test_suite, c_int) -> c_int>,
}

#[repr(C)]
struct map_def {
	name: *const c_char,
	start: u64,
	end: u64,
}

#[repr(C)]
struct check_maps_cb_args {
	merged: *mut map_def,
	i: c_uint,
}

extern "C" {
	fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;

	fn pr_debug(fmt: *const c_char, ...);

	fn map__start(map: *mut map) -> u64;
	fn map__end(map: *mut map) -> u64;
	fn map__set_start(map: *mut map, start: u64);
	fn map__set_end(map: *mut map, end: u64);
	fn map__dso(map: *mut map) -> *mut dso;
	fn map__refcnt(map: *mut map) -> *mut refcount_t;
	fn map__put(map: *mut map);
	fn map__zput(map: *mut map);

	fn dso__name(dso: *mut dso) -> *const c_char;
	fn dso__new_map(name: *const c_char) -> *mut map;

	fn refcount_read(r: *mut refcount_t) -> c_int;

	fn maps__new(machine: *mut c_void) -> *mut maps;
	fn maps__nr_maps(maps: *mut maps) -> c_uint;
	fn maps__insert(maps: *mut maps, map: *mut map) -> c_int;
	fn maps__for_each_map(
		maps: *mut maps,
		cb: Option<unsafe extern "C" fn(*mut map, *mut c_void) -> c_int>,
		data: *mut c_void,
	) -> c_int;
	fn maps__merge_in(maps: *mut maps, map: *mut map) -> c_int;
	fn maps__fixup_overlap_and_insert(maps: *mut maps, map: *mut map) -> c_int;
	fn maps__zput(maps: *mut maps);
}

macro_rules! c_str {
	($s:literal) => {
		concat!($s, "\0").as_ptr() as *const c_char
	};
}

unsafe fn test_assert_val(msg: *const c_char, cond: bool) -> c_int {
	if !cond {
		pr_debug(c_str!("%s\n"), msg);
		return TEST_FAIL;
	}
	TEST_OK
}

unsafe extern "C" fn check_maps_cb(map: *mut map, data: *mut c_void) -> c_int {
	let args = data as *mut check_maps_cb_args;
	let merged = (*args).merged.add((*args).i as usize);

	if map__start(map) != (*merged).start
		|| map__end(map) != (*merged).end
		|| strcmp(dso__name(map__dso(map)), (*merged).name) != 0
		|| refcount_read(map__refcnt(map)) != 1
	{
		return 1;
	}
	(*args).i = (*args).i.wrapping_add(1);
	0
}

unsafe extern "C" fn failed_cb(map: *mut map, _data: *mut c_void) -> c_int {
	pr_debug(
		c_str!("\tstart: %llu end: %llu name: '%s' refcnt: %d\n"),
		map__start(map),
		map__end(map),
		dso__name(map__dso(map)),
		refcount_read(map__refcnt(map)),
	);

	0
}

unsafe fn check_maps(merged: *mut map_def, size: c_uint, maps: *mut maps) -> c_int {
	let mut failed = false;

	if maps__nr_maps(maps) != size {
		pr_debug(
			c_str!("Expected %d maps, got %d"),
			size,
			maps__nr_maps(maps),
		);
		failed = true;
	} else {
		let mut args = check_maps_cb_args {
			merged,
			i: 0,
		};
		failed = maps__for_each_map(
			maps,
			Some(check_maps_cb),
			&mut args as *mut check_maps_cb_args as *mut c_void,
		) != 0;
	}
	if failed {
		pr_debug(c_str!("Expected:\n"));
		let mut i: c_uint = 0;
		while i < size {
			let item = merged.add(i as usize);
			pr_debug(
				c_str!("\tstart: %llu end: %llu name: '%s' refcnt: 1\n"),
				(*item).start,
				(*item).end,
				(*item).name,
			);
			i = i.wrapping_add(1);
		}
		pr_debug(c_str!("Got:\n"));
		maps__for_each_map(maps, Some(failed_cb), ptr::null_mut());
	}
	if failed {
		TEST_FAIL
	} else {
		TEST_OK
	}
}

unsafe extern "C" fn test__maps__merge_in(
	_t: *mut test_suite,
	_subtest: c_int,
) -> c_int {
	let mut i: c_uint;
	let mut bpf_progs = [
		map_def { name: c_str!("bpf_prog_1"), start: 200, end: 300 },
		map_def { name: c_str!("bpf_prog_2"), start: 500, end: 600 },
		map_def { name: c_str!("bpf_prog_3"), start: 800, end: 900 },
	];
	let mut merged12 = [
		map_def { name: c_str!("kcore1"), start: 100, end: 200 },
		map_def { name: c_str!("bpf_prog_1"), start: 200, end: 300 },
		map_def { name: c_str!("kcore1"), start: 300, end: 500 },
		map_def { name: c_str!("bpf_prog_2"), start: 500, end: 600 },
		map_def { name: c_str!("kcore1"), start: 600, end: 800 },
		map_def { name: c_str!("bpf_prog_3"), start: 800, end: 900 },
		map_def { name: c_str!("kcore1"), start: 900, end: 1000 },
	];
	let mut merged3 = [
		map_def { name: c_str!("kcore1"), start: 100, end: 200 },
		map_def { name: c_str!("bpf_prog_1"), start: 200, end: 300 },
		map_def { name: c_str!("kcore1"), start: 300, end: 500 },
		map_def { name: c_str!("bpf_prog_2"), start: 500, end: 600 },
		map_def { name: c_str!("kcore1"), start: 600, end: 800 },
		map_def { name: c_str!("bpf_prog_3"), start: 800, end: 900 },
		map_def { name: c_str!("kcore1"), start: 900, end: 1000 },
		map_def { name: c_str!("kcore3"), start: 1000, end: 1100 },
	];
	let map_kcore1: *mut map;
	let map_kcore2: *mut map;
	let map_kcore3: *mut map;
	let mut ret: c_int;
	let maps = maps__new(ptr::null_mut());

	if test_assert_val(c_str!("failed to create maps"), !maps.is_null()) != TEST_OK {
		return TEST_FAIL;
	}

	i = 0;
	while (i as usize) < bpf_progs.len() {
		let map: *mut map;

		map = dso__new_map(bpf_progs[i as usize].name);
		if test_assert_val(c_str!("failed to create map"), !map.is_null()) != TEST_OK {
			return TEST_FAIL;
		}

		map__set_start(map, bpf_progs[i as usize].start);
		map__set_end(map, bpf_progs[i as usize].end);
		if test_assert_val(c_str!("failed to insert map"), maps__insert(maps, map) == 0) != TEST_OK {
			return TEST_FAIL;
		}
		map__put(map);
		i = i.wrapping_add(1);
	}

	map_kcore1 = dso__new_map(c_str!("kcore1"));
	if test_assert_val(c_str!("failed to create map"), !map_kcore1.is_null()) != TEST_OK {
		return TEST_FAIL;
	}

	map_kcore2 = dso__new_map(c_str!("kcore2"));
	if test_assert_val(c_str!("failed to create map"), !map_kcore2.is_null()) != TEST_OK {
		return TEST_FAIL;
	}

	map_kcore3 = dso__new_map(c_str!("kcore3"));
	if test_assert_val(c_str!("failed to create map"), !map_kcore3.is_null()) != TEST_OK {
		return TEST_FAIL;
	}

	/* kcore1 map overlaps over all bpf maps */
	map__set_start(map_kcore1, 100);
	map__set_end(map_kcore1, 1000);

	/* kcore2 map hides behind bpf_prog_2 */
	map__set_start(map_kcore2, 550);
	map__set_end(map_kcore2, 570);

	/* kcore3 map hides behind bpf_prog_3, kcore1 and adds new map */
	map__set_start(map_kcore3, 880);
	map__set_end(map_kcore3, 1100);

	ret = maps__merge_in(maps, map_kcore1);
	if test_assert_val(c_str!("failed to merge map"), ret == 0) != TEST_OK {
		return TEST_FAIL;
	}

	ret = check_maps(merged12.as_mut_ptr(), merged12.len() as c_uint, maps);
	if test_assert_val(c_str!("merge check failed"), ret == 0) != TEST_OK {
		return TEST_FAIL;
	}

	ret = maps__merge_in(maps, map_kcore2);
	if test_assert_val(c_str!("failed to merge map"), ret == 0) != TEST_OK {
		return TEST_FAIL;
	}

	ret = check_maps(merged12.as_mut_ptr(), merged12.len() as c_uint, maps);
	if test_assert_val(c_str!("merge check failed"), ret == 0) != TEST_OK {
		return TEST_FAIL;
	}

	ret = maps__merge_in(maps, map_kcore3);
	if test_assert_val(c_str!("failed to merge map"), ret == 0) != TEST_OK {
		return TEST_FAIL;
	}

	ret = check_maps(merged3.as_mut_ptr(), merged3.len() as c_uint, maps);
	if test_assert_val(c_str!("merge check failed"), ret == 0) != TEST_OK {
		return TEST_FAIL;
	}

	maps__zput(maps);
	map__zput(map_kcore1);
	map__zput(map_kcore2);
	map__zput(map_kcore3);
	TEST_OK
}

unsafe extern "C" fn test__maps__fixup_overlap_and_insert(
	_t: *mut test_suite,
	_subtest: c_int,
) -> c_int {
	let mut initial_maps = [
		map_def { name: c_str!("target_map"), start: 1000, end: 2000 },
		map_def { name: c_str!("next_map"), start: 3000, end: 4000 },
	];
	let insert_split = map_def { name: c_str!("split_map"), start: 1400, end: 1600 };
	let mut expected_after_split = [
		map_def { name: c_str!("target_map"), start: 1000, end: 1400 },
		map_def { name: c_str!("split_map"), start: 1400, end: 1600 },
		map_def { name: c_str!("target_map"), start: 1600, end: 2000 },
		map_def { name: c_str!("next_map"), start: 3000, end: 4000 },
	];

	let insert_eclipse = map_def { name: c_str!("eclipse_map"), start: 2500, end: 4500 };
	let mut expected_final = [
		map_def { name: c_str!("target_map"), start: 1000, end: 1400 },
		map_def { name: c_str!("split_map"), start: 1400, end: 1600 },
		map_def { name: c_str!("target_map"), start: 1600, end: 2000 },
		map_def { name: c_str!("eclipse_map"), start: 2500, end: 4500 },
		/* "next_map" (3000-4000) is removed */
	];

	let map_split: *mut map;
	let map_eclipse: *mut map;
	let mut ret: c_int;
	let mut i: c_uint;
	let maps = maps__new(ptr::null_mut());

	if test_assert_val(c_str!("failed to create maps"), !maps.is_null()) != TEST_OK {
		return TEST_FAIL;
	}

	i = 0;
	while (i as usize) < initial_maps.len() {
		let map = dso__new_map(initial_maps[i as usize].name);

		if test_assert_val(c_str!("failed to create map"), !map.is_null()) != TEST_OK {
			return TEST_FAIL;
		}
		map__set_start(map, initial_maps[i as usize].start);
		map__set_end(map, initial_maps[i as usize].end);
		if test_assert_val(c_str!("failed to insert map"), maps__insert(maps, map) == 0) != TEST_OK {
			return TEST_FAIL;
		}
		map__put(map);
		i = i.wrapping_add(1);
	}

	// Check splitting.
	map_split = dso__new_map(insert_split.name);
	if test_assert_val(c_str!("failed to create split map"), !map_split.is_null()) != TEST_OK {
		return TEST_FAIL;
	}
	map__set_start(map_split, insert_split.start);
	map__set_end(map_split, insert_split.end);

	ret = maps__fixup_overlap_and_insert(maps, map_split);
	if test_assert_val(c_str!("failed to fixup and insert split map"), ret == 0) != TEST_OK {
		return TEST_FAIL;
	}

	map__zput(map_split);
	ret = check_maps(
		expected_after_split.as_mut_ptr(),
		expected_after_split.len() as c_uint,
		maps,
	);
	if test_assert_val(c_str!("split check failed"), ret == 0) != TEST_OK {
		return TEST_FAIL;
	}

	// Check cover 1 map with another.
	map_eclipse = dso__new_map(insert_eclipse.name);
	if test_assert_val(c_str!("failed to create eclipse map"), !map_eclipse.is_null()) != TEST_OK {
		return TEST_FAIL;
	}
	map__set_start(map_eclipse, insert_eclipse.start);
	map__set_end(map_eclipse, insert_eclipse.end);

	ret = maps__fixup_overlap_and_insert(maps, map_eclipse);
	if test_assert_val(c_str!("failed to fixup and insert eclipse map"), ret == 0) != TEST_OK {
		return TEST_FAIL;
	}

	map__zput(map_eclipse);
	ret = check_maps(expected_final.as_mut_ptr(), expected_final.len() as c_uint, maps);
	if test_assert_val(c_str!("eclipse check failed"), ret == 0) != TEST_OK {
		return TEST_FAIL;
	}

	maps__zput(maps);
	TEST_OK
}

#[no_mangle]
pub static mut tests__maps: [test_case; 3] = [
	test_case {
		name: c_str!("Test merge_in interface"),
		run_case: Some(test__maps__merge_in),
	},
	test_case {
		name: c_str!("Test fix up overlap interface"),
		run_case: Some(test__maps__fixup_overlap_and_insert),
	},
	test_case {
		name: ptr::null(),
		run_case: None,
	},
];

#[no_mangle]
pub static mut suite__maps: test_suite = test_suite {
	desc: c_str!("Maps - per process mmaps abstraction"),
	test_cases: unsafe { tests__maps.as_mut_ptr() },
};
