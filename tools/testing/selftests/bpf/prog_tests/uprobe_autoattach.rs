// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022, Oracle and/or its affiliates. */

// C dependencies:
// #include <test_progs.h>
// #include "test_uprobe_autoattach.skel.h"

use core::ffi::{c_char, c_int, c_void};

type __u64 = u64;

#[repr(C)]
pub struct FILE {
	_private: [u8; 0],
}

#[repr(C)]
pub struct test_uprobe_autoattach {
	pub bss: *mut test_uprobe_autoattach_bss,
}

#[repr(C)]
pub struct test_uprobe_autoattach_bss {
	pub test_pid: c_int,
	pub uprobe_byname_parm1: __u64,
	pub uprobe_byname_ran: __u64,
	pub uretprobe_byname_rc: __u64,
	pub uretprobe_byname_ret: __u64,
	pub uretprobe_byname_ran: __u64,
	pub uprobe_byname2_parm1: __u64,
	pub uprobe_byname2_ran: __u64,
	pub uretprobe_byname2_rc: __u64,
	pub uretprobe_byname2_ran: __u64,
	pub a: [__u64; 8],
}

extern "C" {
	fn test_uprobe_autoattach__open_and_load() -> *mut test_uprobe_autoattach;
	fn test_uprobe_autoattach__attach(skel: *mut test_uprobe_autoattach) -> c_int;
	fn test_uprobe_autoattach__destroy(skel: *mut test_uprobe_autoattach);

	fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
	fn ASSERT_OK(ret: c_int, name: *const c_char) -> bool;
	fn ASSERT_EQ(actual: __u64, expected: __u64, name: *const c_char) -> bool;

	fn getpid() -> c_int;
	fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
	fn fclose(stream: *mut FILE) -> c_int;
}

/* uprobe attach point */
#[inline(never)]
unsafe fn autoattach_trigger_func(
	arg1: c_int,
	arg2: c_int,
	arg3: c_int,
	arg4: c_int,
	arg5: c_int,
	arg6: c_int,
	arg7: c_int,
	arg8: c_int,
) -> c_int {
	core::arch::asm!("", options(nomem, nostack, preserves_flags));
	arg1 + arg2 + arg3 + arg4 + arg5 + arg6 + arg7 + arg8 + 1
}

#[no_mangle]
pub unsafe extern "C" fn test_uprobe_autoattach() {
	let devnull_str = b"/dev/null\0";
	let mut skel: *mut test_uprobe_autoattach;
	let trigger_ret: c_int;
	let devnull: *mut FILE;

	skel = test_uprobe_autoattach__open_and_load();
	if !ASSERT_OK_PTR(skel as *const c_void, b"skel_open\0".as_ptr() as *const c_char) {
		return;
	}

	if !ASSERT_OK(
		test_uprobe_autoattach__attach(skel),
		b"skel_attach\0".as_ptr() as *const c_char,
	) {
		test_uprobe_autoattach__destroy(skel);
		return;
	}

	(*(*skel).bss).test_pid = getpid();

	/* trigger & validate uprobe & uretprobe */
	trigger_ret = autoattach_trigger_func(1, 2, 3, 4, 5, 6, 7, 8);

	(*(*skel).bss).test_pid = getpid();

	/* trigger & validate shared library u[ret]probes attached by name */
	devnull = fopen(
		devnull_str.as_ptr() as *const c_char,
		b"r\0".as_ptr() as *const c_char,
	);

	ASSERT_EQ(
		(*(*skel).bss).uprobe_byname_parm1,
		1,
		b"check_uprobe_byname_parm1\0".as_ptr() as *const c_char,
	);
	ASSERT_EQ(
		(*(*skel).bss).uprobe_byname_ran,
		1,
		b"check_uprobe_byname_ran\0".as_ptr() as *const c_char,
	);
	ASSERT_EQ(
		(*(*skel).bss).uretprobe_byname_rc,
		trigger_ret as __u64,
		b"check_uretprobe_byname_rc\0".as_ptr() as *const c_char,
	);
	ASSERT_EQ(
		(*(*skel).bss).uretprobe_byname_ret,
		trigger_ret as __u64,
		b"check_uretprobe_byname_ret\0".as_ptr() as *const c_char,
	);
	ASSERT_EQ(
		(*(*skel).bss).uretprobe_byname_ran,
		2,
		b"check_uretprobe_byname_ran\0".as_ptr() as *const c_char,
	);
	ASSERT_EQ(
		(*(*skel).bss).uprobe_byname2_parm1,
		devnull_str.as_ptr() as isize as __u64,
		b"check_uprobe_byname2_parm1\0".as_ptr() as *const c_char,
	);
	ASSERT_EQ(
		(*(*skel).bss).uprobe_byname2_ran,
		3,
		b"check_uprobe_byname2_ran\0".as_ptr() as *const c_char,
	);
	ASSERT_EQ(
		(*(*skel).bss).uretprobe_byname2_rc,
		devnull as isize as __u64,
		b"check_uretprobe_byname2_rc\0".as_ptr() as *const c_char,
	);
	ASSERT_EQ(
		(*(*skel).bss).uretprobe_byname2_ran,
		4,
		b"check_uretprobe_byname2_ran\0".as_ptr() as *const c_char,
	);

	ASSERT_EQ((*(*skel).bss).a[0], 1, b"arg1\0".as_ptr() as *const c_char);
	ASSERT_EQ((*(*skel).bss).a[1], 2, b"arg2\0".as_ptr() as *const c_char);
	ASSERT_EQ((*(*skel).bss).a[2], 3, b"arg3\0".as_ptr() as *const c_char);
	// Original C conditional: #if FUNC_REG_ARG_CNT > 3
	ASSERT_EQ((*(*skel).bss).a[3], 4, b"arg4\0".as_ptr() as *const c_char);
	// Original C conditional: #endif
	// Original C conditional: #if FUNC_REG_ARG_CNT > 4
	ASSERT_EQ((*(*skel).bss).a[4], 5, b"arg5\0".as_ptr() as *const c_char);
	// Original C conditional: #endif
	// Original C conditional: #if FUNC_REG_ARG_CNT > 5
	ASSERT_EQ((*(*skel).bss).a[5], 6, b"arg6\0".as_ptr() as *const c_char);
	// Original C conditional: #endif
	// Original C conditional: #if FUNC_REG_ARG_CNT > 6
	ASSERT_EQ((*(*skel).bss).a[6], 7, b"arg7\0".as_ptr() as *const c_char);
	// Original C conditional: #endif
	// Original C conditional: #if FUNC_REG_ARG_CNT > 7
	ASSERT_EQ((*(*skel).bss).a[7], 8, b"arg8\0".as_ptr() as *const c_char);
	// Original C conditional: #endif

	fclose(devnull);
	test_uprobe_autoattach__destroy(skel);
}
