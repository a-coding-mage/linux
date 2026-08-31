// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

/* Dependencies from the original C source:
 * #include "vmlinux.h"
 * #include <bpf/bpf_helpers.h>
 * #include "bpf_misc.h"
 */

const HASHMAP_SZ: u32 = 4194304;

#[repr(C)]
pub struct task_struct {
	_opaque: [u8; 0],
}

#[repr(C)]
struct ArrayOfLocalStorageMaps {
	/* Original BPF map definition:
	 * __uint(type, BPF_MAP_TYPE_ARRAY_OF_MAPS);
	 * __uint(max_entries, 1000);
	 * __type(key, int);
	 * __type(value, int);
	 * __array(values, struct {
	 *     __uint(type, BPF_MAP_TYPE_TASK_STORAGE);
	 *     __uint(map_flags, BPF_F_NO_PREALLOC);
	 *     __type(key, int);
	 *     __type(value, int);
	 * });
	 */
	_private: [u8; 0],
}

#[repr(C)]
struct ArrayOfHashMaps {
	/* Original BPF map definition:
	 * __uint(type, BPF_MAP_TYPE_ARRAY_OF_MAPS);
	 * __uint(max_entries, 1000);
	 * __type(key, int);
	 * __type(value, int);
	 * __array(values, struct {
	 *     __uint(type, BPF_MAP_TYPE_HASH);
	 *     __uint(max_entries, HASHMAP_SZ);
	 *     __type(key, int);
	 *     __type(value, int);
	 * });
	 */
	_private: [u8; 0],
}

/* SEC(".maps") */
#[no_mangle]
static mut array_of_local_storage_maps: ArrayOfLocalStorageMaps =
	ArrayOfLocalStorageMaps { _private: [] };

/* SEC(".maps") */
#[no_mangle]
static mut array_of_hash_maps: ArrayOfHashMaps = ArrayOfHashMaps { _private: [] };

#[no_mangle]
static mut important_hits: i64 = 0;
#[no_mangle]
static mut hits: i64 = 0;

/* set from user-space */
#[no_mangle]
static use_hashmap: u32 = 0;
#[no_mangle]
static hashmap_num_keys: u32 = 0;
#[no_mangle]
static num_maps: u32 = 0;
#[no_mangle]
static interleave: u32 = 0;

#[repr(C)]
struct loop_ctx {
	task: *mut task_struct,
	loop_hits: i64,
	loop_important_hits: i64,
}

const BPF_LOCAL_STORAGE_GET_F_CREATE: u64 = 0;

unsafe extern "C" {
	fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void)
		-> *mut core::ffi::c_void;
	fn bpf_get_prandom_u32() -> u32;
	fn bpf_task_storage_get(
		map: *mut core::ffi::c_void,
		task: *mut task_struct,
		value: *const core::ffi::c_void,
		flags: u64,
	) -> *mut core::ffi::c_void;
	fn bpf_get_current_task_btf() -> *mut task_struct;
	fn bpf_loop(
		nr_loops: u32,
		callback_fn: Option<unsafe extern "C" fn(u32, *mut core::ffi::c_void) -> i64>,
		callback_ctx: *mut core::ffi::c_void,
		flags: u64,
	) -> i64;
}

unsafe fn do_lookup(elem: u32, lctx: *mut loop_ctx) -> i32 {
	let map: *mut core::ffi::c_void;
	let inner_map: *mut core::ffi::c_void;
	let mut idx: i32 = 0;

	if core::ptr::read_volatile(&use_hashmap) != 0 {
		map = &raw mut array_of_hash_maps as *mut core::ffi::c_void;
	} else {
		map = &raw mut array_of_local_storage_maps as *mut core::ffi::c_void;
	}

	inner_map = bpf_map_lookup_elem(map, &elem as *const u32 as *const core::ffi::c_void);
	if inner_map.is_null() {
		return -1;
	}

	if core::ptr::read_volatile(&use_hashmap) != 0 {
		idx = (bpf_get_prandom_u32() % core::ptr::read_volatile(&hashmap_num_keys)) as i32;
		bpf_map_lookup_elem(inner_map, &idx as *const i32 as *const core::ffi::c_void);
	} else {
		bpf_task_storage_get(
			inner_map,
			(*lctx).task,
			&idx as *const i32 as *const core::ffi::c_void,
			BPF_LOCAL_STORAGE_GET_F_CREATE,
		);
	}

	(*lctx).loop_hits += 1;
	if elem == 0 {
		(*lctx).loop_important_hits += 1;
	}
	0
}

unsafe extern "C" fn loop_(index: u32, ctx: *mut core::ffi::c_void) -> i64 {
	let lctx: *mut loop_ctx = ctx as *mut loop_ctx;
	let map_idx: u32 = index % core::ptr::read_volatile(&num_maps);

	do_lookup(map_idx, lctx);
	if core::ptr::read_volatile(&interleave) != 0 && map_idx % 3 == 0 {
		do_lookup(0, lctx);
	}
	0
}

/* SEC("fentry/" SYS_PREFIX "sys_getpgid") */
#[no_mangle]
pub unsafe extern "C" fn get_local(ctx: *mut core::ffi::c_void) -> i32 {
	let mut lctx: loop_ctx = loop_ctx {
		task: core::ptr::null_mut(),
		loop_hits: 0,
		loop_important_hits: 0,
	};

	lctx.task = bpf_get_current_task_btf();
	lctx.loop_hits = 0;
	lctx.loop_important_hits = 0;
	bpf_loop(
		10000,
		Some(loop_),
		&mut lctx as *mut loop_ctx as *mut core::ffi::c_void,
		0,
	);
	core::intrinsics::atomic_xadd_relaxed(&raw mut hits, lctx.loop_hits);
	core::intrinsics::atomic_xadd_relaxed(&raw mut important_hits, lctx.loop_important_hits);
	let _ = ctx;
	0
}

/* SEC("license") */
#[no_mangle]
static mut _license: [u8; 4] = *b"GPL\0";
