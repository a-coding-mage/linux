// SPDX-License-Identifier: LGPL-2.1 OR BSD-2-Clause
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

/* Translated from:
 * #include <libarena/common.h>
 * #include <libarena/asan.h>
 * #include <libarena/buddy.h>
 */

type size_t = usize;
type uintptr_t = usize;

const ENOMEM: i32 = 12;
const EINVAL: i32 = 22;
const SEGARRLEN: usize = 512;

#[repr(C)]
pub struct buddy {
	_private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct segarr_entry {
	block: *mut u8,
	sz: size_t,
	poison: u8,
}

extern "C" {
	/* extern struct buddy __arena buddy; */
	static mut buddy: buddy;
	static mut zero: i32;
	static mut can_loop: bool;

	fn buddy_init(buddy: *mut buddy) -> i32;
	fn buddy_destroy(buddy: *mut buddy) -> i32;
	fn buddy_alloc(buddy: *mut buddy, sz: size_t) -> *mut core::ffi::c_void;
	fn buddy_free(buddy: *mut buddy, mem: *mut core::ffi::c_void);
	fn barrier_var(i: u32);
	fn arena_stdout(fmt: *const u8, ...);
}

static mut segarr: [segarr_entry; SEGARRLEN] = [segarr_entry {
	block: core::ptr::null_mut(),
	sz: 0,
	poison: 0,
}; SEGARRLEN];
static mut ptrs: [*mut core::ffi::c_void; 17] = [core::ptr::null_mut(); 17];
static mut alloc_sizes: [size_t; 8] = [3, 17, 1025, 129, 16350, 333, 9, 517];
static mut alloc_multiple_sizes: [size_t; 9] = [3, 17, 1025, 129, 16350, 333, 9, 517, 2099];
static mut alloc_free_sizes: [size_t; 8] = [3, 17, 64, 129, 256, 333, 512, 517];
static mut alignment_sizes: [size_t; 17] = [
	1, 3, 7, 8, 9, 15, 16, 17, 31, 32, 64, 100, 128, 255, 256, 512, 1000,
];

/* SEC("syscall") */
#[no_mangle]
pub unsafe extern "C" fn test_buddy_create() -> i32 {
	let iters: i32 = 10;
	let mut ret: i32;
	let mut i: i32;

	i = zero;
	while i < iters && can_loop {
		ret = buddy_init(&mut buddy);
		if ret != 0 {
			return ret;
		}

		ret = buddy_destroy(&mut buddy);
		if ret != 0 {
			return ret;
		}

		i += 1;
	}

	0
}

/* SEC("syscall") */
#[no_mangle]
pub unsafe extern "C" fn test_buddy_alloc() -> i32 {
	let mut mem: *mut core::ffi::c_void;
	let mut ret: i32;
	let mut i: i32;

	i = zero;
	while i < 8 && can_loop {
		ret = buddy_init(&mut buddy);
		if ret != 0 {
			return ret;
		}

		mem = buddy_alloc(&mut buddy, alloc_sizes[i as usize]);
		if mem.is_null() {
			buddy_destroy(&mut buddy);
			return -ENOMEM;
		}

		buddy_destroy(&mut buddy);

		i += 1;
	}

	0
}

/* SEC("syscall") */
#[no_mangle]
pub unsafe extern "C" fn test_buddy_alloc_free() -> i32 {
	let iters: i32 = 800;
	let mut mem: *mut core::ffi::c_void;
	let ret: i32;
	let mut i: i32;

	ret = buddy_init(&mut buddy);
	if ret != 0 {
		return ret;
	}

	i = zero;
	while i < iters && can_loop {
		mem = buddy_alloc(&mut buddy, alloc_free_sizes[((i * 5) % 8) as usize]);
		if mem.is_null() {
			buddy_destroy(&mut buddy);
			return -ENOMEM;
		}

		buddy_free(&mut buddy, mem);

		i += 1;
	}

	buddy_destroy(&mut buddy);

	0
}

/* SEC("syscall") */
#[no_mangle]
pub unsafe extern "C" fn test_buddy_alloc_multiple() -> i32 {
	let mut ret: i32;
	let mut j: i32;
	let mut i: u32;
	let mut idx: u32;
	let mut mem: *mut u8;
	let mut sz: size_t;
	let mut poison: u8;

	ret = buddy_init(&mut buddy);
	if ret != 0 {
		return ret;
	}

	/*
	 * Cycle through each size, allocating an entry in the
	 * segarr. Continue for SEGARRLEN iterations. For every
	 * allocation write down the size, use the current index
	 * as a poison value, and log it with the pointer in the
	 * segarr entry. Use the poison value to poison the entire
	 * allocated memory according to the size given.
	 */
	i = zero as u32;
	while (i as usize) < SEGARRLEN && can_loop {
		sz = alloc_multiple_sizes[(i % 9) as usize];
		poison = i as u8;

		mem = buddy_alloc(&mut buddy, sz) as *mut u8;
		if mem.is_null() {
			buddy_destroy(&mut buddy);
			arena_stdout(b"%s:%d\0".as_ptr(), b"test_buddy_alloc_multiple\0".as_ptr(), line!());
			return -ENOMEM;
		}

		segarr[i as usize].block = mem;
		segarr[i as usize].sz = sz;
		segarr[i as usize].poison = poison;

		j = zero;
		while (j as size_t) < sz && can_loop {
			*mem.add(j as usize) = poison;
			if *mem.add(j as usize) != poison {
				buddy_destroy(&mut buddy);
				return -EINVAL;
			}

			j += 1;
		}

		i += 1;
	}

	/*
	 * Go to (i * 17) % SEGARRLEN, and free the block pointed to.
	 * Before freeing, check all bytes have the poisoned value
	 * corresponding to the element. If any values are unexpected,
	 * return an error. Skip some elements to test destroying the
	 * buddy allocator while data is still allocated.
	 */
	i = 10;
	while (i as usize) < SEGARRLEN && can_loop {
		idx = (i * 17) % SEGARRLEN as u32;

		mem = segarr[idx as usize].block;
		sz = segarr[idx as usize].sz;
		poison = segarr[idx as usize].poison;

		j = zero;
		while (j as size_t) < sz && can_loop {
			if *mem.add(j as usize) != poison {
				buddy_destroy(&mut buddy);
				arena_stdout(
					b"%s:%d %lx %u vs %u\0".as_ptr(),
					b"test_buddy_alloc_multiple\0".as_ptr(),
					line!(),
					mem.add(j as usize) as uintptr_t,
					*mem.add(j as usize) as u32,
					poison as u32,
				);
				return -EINVAL;
			}

			j += 1;
		}

		buddy_free(&mut buddy, mem as *mut core::ffi::c_void);

		i += 1;
	}

	buddy_destroy(&mut buddy);

	0
}

/* SEC("syscall") */
#[no_mangle]
pub unsafe extern "C" fn test_buddy_alignment() -> i32 {
	let mut ret: i32;
	let mut i: u32;

	ret = buddy_init(&mut buddy);
	if ret != 0 {
		return ret;
	}

	/* Allocate various sizes and check alignment */
	i = zero as u32;
	while i < 17 && can_loop {
		barrier_var(i);
		ptrs[i as usize] = buddy_alloc(&mut buddy, alignment_sizes[i as usize]);
		if ptrs[i as usize].is_null() {
			arena_stdout(
				b"alignment test: alloc failed for size %lu\0".as_ptr(),
				alignment_sizes[i as usize],
			);
			buddy_destroy(&mut buddy);
			return -ENOMEM;
		}

		/* Check 8-byte alignment */
		if (ptrs[i as usize] as u64) & 0x7 != 0 {
			arena_stdout(
				b"alignment test: ptr %llx not 8-byte aligned (size %lu)\0".as_ptr(),
				ptrs[i as usize] as u64,
				alignment_sizes[i as usize],
			);
			buddy_destroy(&mut buddy);
			return -EINVAL;
		}

		i += 1;
	}

	/* Free all allocations */
	i = zero as u32;
	while i < 17 && can_loop {
		barrier_var(i);
		buddy_free(&mut buddy, ptrs[i as usize]);

		i += 1;
	}

	buddy_destroy(&mut buddy);

	0
}

/* __weak char _license[] SEC("license") = "GPL"; */
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";
