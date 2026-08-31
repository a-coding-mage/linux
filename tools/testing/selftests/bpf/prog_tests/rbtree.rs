// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

/* Translated from C source:
 * #include <test_progs.h>
 * #include <network_helpers.h>
 * #include "rbtree.skel.h"
 * #include "rbtree_fail.skel.h"
 * #include "rbtree_btf_fail__wrong_node_type.skel.h"
 * #include "rbtree_btf_fail__add_wrong_type.skel.h"
 * #include "rbtree_search.skel.h"
 * #include "rbtree_search_kptr.skel.h"
 */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct bpf_test_run_opts {
	pub sz: usize,
	pub data_in: *const c_void,
	pub data_size_in: u32,
	pub repeat: u32,
	pub retval: u32,
}

#[repr(C)]
pub struct bpf_program {
	_private: [u8; 0],
}

#[repr(C)]
pub struct rbtree {
	pub progs: rbtree_progs,
	pub data: *mut rbtree_data,
}

#[repr(C)]
pub struct rbtree_progs {
	pub rbtree_add_nodes: *mut bpf_program,
	pub rbtree_add_nodes_nested: *mut bpf_program,
	pub rbtree_add_and_remove: *mut bpf_program,
	pub rbtree_add_and_remove_array: *mut bpf_program,
	pub rbtree_first_and_remove: *mut bpf_program,
	pub rbtree_api_release_aliasing: *mut bpf_program,
}

#[repr(C)]
pub struct rbtree_data {
	pub less_callback_ran: c_int,
	pub removed_key: c_int,
	pub first_data: [c_int; 2],
}

#[repr(C)]
pub struct rbtree_btf_fail__wrong_node_type {
	_private: [u8; 0],
}

#[repr(C)]
pub struct rbtree_btf_fail__add_wrong_type {
	_private: [u8; 0],
}

unsafe extern "C" {
	static pkt_v4: [u8; 0];

	fn rbtree__open_and_load() -> *mut rbtree;
	fn rbtree__destroy(skel: *mut rbtree);

	fn rbtree_btf_fail__wrong_node_type__open_and_load() -> *mut rbtree_btf_fail__wrong_node_type;
	fn rbtree_btf_fail__wrong_node_type__destroy(skel: *mut rbtree_btf_fail__wrong_node_type);
	fn rbtree_btf_fail__add_wrong_type__open_and_load() -> *mut rbtree_btf_fail__add_wrong_type;
	fn rbtree_btf_fail__add_wrong_type__destroy(skel: *mut rbtree_btf_fail__add_wrong_type);

	fn bpf_program__fd(prog: *const bpf_program) -> c_int;
	fn bpf_prog_test_run_opts(fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;

	fn test__start_subtest(name: *const c_char) -> bool;
	fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
	fn ASSERT_ERR_PTR(ptr: *const c_void, name: *const c_char) -> bool;
	fn ASSERT_OK(ret: c_int, name: *const c_char) -> bool;
	fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;

	fn RUN_TESTS_rbtree_fail();
	fn RUN_TESTS_rbtree_search();
	fn RUN_TESTS_rbtree_search_kptr();
}

const PKT_V4_SIZE: u32 = core::mem::size_of_val(unsafe { &pkt_v4 }) as u32;

unsafe fn test_rbtree_add_nodes() {
	let mut opts = bpf_test_run_opts {
		sz: core::mem::size_of::<bpf_test_run_opts>(),
		data_in: unsafe { pkt_v4.as_ptr() as *const c_void },
		data_size_in: PKT_V4_SIZE,
		repeat: 1,
		retval: 0,
	};
	let mut skel: *mut rbtree;
	let ret: c_int;

	skel = unsafe { rbtree__open_and_load() };
	if !unsafe { ASSERT_OK_PTR(skel as *const c_void, c"rbtree__open_and_load".as_ptr()) } {
		return;
	}

	ret = unsafe {
		bpf_prog_test_run_opts(
			bpf_program__fd((*skel).progs.rbtree_add_nodes),
			&mut opts,
		)
	};
	unsafe { ASSERT_OK(ret, c"rbtree_add_nodes run".as_ptr()) };
	unsafe { ASSERT_OK(opts.retval as c_int, c"rbtree_add_nodes retval".as_ptr()) };
	unsafe {
		ASSERT_EQ(
			(*(*skel).data).less_callback_ran,
			1,
			c"rbtree_add_nodes less_callback_ran".as_ptr(),
		)
	};

	unsafe { rbtree__destroy(skel) };
}

unsafe fn test_rbtree_add_nodes_nested() {
	let mut opts = bpf_test_run_opts {
		sz: core::mem::size_of::<bpf_test_run_opts>(),
		data_in: unsafe { pkt_v4.as_ptr() as *const c_void },
		data_size_in: PKT_V4_SIZE,
		repeat: 1,
		retval: 0,
	};
	let mut skel: *mut rbtree;
	let ret: c_int;

	skel = unsafe { rbtree__open_and_load() };
	if !unsafe { ASSERT_OK_PTR(skel as *const c_void, c"rbtree__open_and_load".as_ptr()) } {
		return;
	}

	ret = unsafe {
		bpf_prog_test_run_opts(
			bpf_program__fd((*skel).progs.rbtree_add_nodes_nested),
			&mut opts,
		)
	};
	unsafe { ASSERT_OK(ret, c"rbtree_add_nodes_nested run".as_ptr()) };
	unsafe { ASSERT_OK(opts.retval as c_int, c"rbtree_add_nodes_nested retval".as_ptr()) };
	unsafe {
		ASSERT_EQ(
			(*(*skel).data).less_callback_ran,
			1,
			c"rbtree_add_nodes_nested less_callback_ran".as_ptr(),
		)
	};

	unsafe { rbtree__destroy(skel) };
}

unsafe fn test_rbtree_add_and_remove() {
	let mut opts = bpf_test_run_opts {
		sz: core::mem::size_of::<bpf_test_run_opts>(),
		data_in: unsafe { pkt_v4.as_ptr() as *const c_void },
		data_size_in: PKT_V4_SIZE,
		repeat: 1,
		retval: 0,
	};
	let mut skel: *mut rbtree;
	let ret: c_int;

	skel = unsafe { rbtree__open_and_load() };
	if !unsafe { ASSERT_OK_PTR(skel as *const c_void, c"rbtree__open_and_load".as_ptr()) } {
		return;
	}

	ret = unsafe {
		bpf_prog_test_run_opts(
			bpf_program__fd((*skel).progs.rbtree_add_and_remove),
			&mut opts,
		)
	};
	unsafe { ASSERT_OK(ret, c"rbtree_add_and_remove".as_ptr()) };
	unsafe { ASSERT_OK(opts.retval as c_int, c"rbtree_add_and_remove retval".as_ptr()) };
	unsafe {
		ASSERT_EQ(
			(*(*skel).data).removed_key,
			5,
			c"rbtree_add_and_remove first removed key".as_ptr(),
		)
	};

	unsafe { rbtree__destroy(skel) };
}

unsafe fn test_rbtree_add_and_remove_array() {
	let mut opts = bpf_test_run_opts {
		sz: core::mem::size_of::<bpf_test_run_opts>(),
		data_in: unsafe { pkt_v4.as_ptr() as *const c_void },
		data_size_in: PKT_V4_SIZE,
		repeat: 1,
		retval: 0,
	};
	let mut skel: *mut rbtree;
	let ret: c_int;

	skel = unsafe { rbtree__open_and_load() };
	if !unsafe { ASSERT_OK_PTR(skel as *const c_void, c"rbtree__open_and_load".as_ptr()) } {
		return;
	}

	ret = unsafe {
		bpf_prog_test_run_opts(
			bpf_program__fd((*skel).progs.rbtree_add_and_remove_array),
			&mut opts,
		)
	};
	unsafe { ASSERT_OK(ret, c"rbtree_add_and_remove_array".as_ptr()) };
	unsafe { ASSERT_OK(opts.retval as c_int, c"rbtree_add_and_remove_array retval".as_ptr()) };

	unsafe { rbtree__destroy(skel) };
}

unsafe fn test_rbtree_first_and_remove() {
	let mut opts = bpf_test_run_opts {
		sz: core::mem::size_of::<bpf_test_run_opts>(),
		data_in: unsafe { pkt_v4.as_ptr() as *const c_void },
		data_size_in: PKT_V4_SIZE,
		repeat: 1,
		retval: 0,
	};
	let mut skel: *mut rbtree;
	let ret: c_int;

	skel = unsafe { rbtree__open_and_load() };
	if !unsafe { ASSERT_OK_PTR(skel as *const c_void, c"rbtree__open_and_load".as_ptr()) } {
		return;
	}

	ret = unsafe {
		bpf_prog_test_run_opts(
			bpf_program__fd((*skel).progs.rbtree_first_and_remove),
			&mut opts,
		)
	};
	unsafe { ASSERT_OK(ret, c"rbtree_first_and_remove".as_ptr()) };
	unsafe { ASSERT_OK(opts.retval as c_int, c"rbtree_first_and_remove retval".as_ptr()) };
	unsafe {
		ASSERT_EQ(
			(*(*skel).data).first_data[0],
			2,
			c"rbtree_first_and_remove first rbtree_first()".as_ptr(),
		)
	};
	unsafe {
		ASSERT_EQ(
			(*(*skel).data).removed_key,
			1,
			c"rbtree_first_and_remove first removed key".as_ptr(),
		)
	};
	unsafe {
		ASSERT_EQ(
			(*(*skel).data).first_data[1],
			4,
			c"rbtree_first_and_remove second rbtree_first()".as_ptr(),
		)
	};

	unsafe { rbtree__destroy(skel) };
}

unsafe fn test_rbtree_api_release_aliasing() {
	let mut opts = bpf_test_run_opts {
		sz: core::mem::size_of::<bpf_test_run_opts>(),
		data_in: unsafe { pkt_v4.as_ptr() as *const c_void },
		data_size_in: PKT_V4_SIZE,
		repeat: 1,
		retval: 0,
	};
	let mut skel: *mut rbtree;
	let ret: c_int;

	skel = unsafe { rbtree__open_and_load() };
	if !unsafe { ASSERT_OK_PTR(skel as *const c_void, c"rbtree__open_and_load".as_ptr()) } {
		return;
	}

	ret = unsafe {
		bpf_prog_test_run_opts(
			bpf_program__fd((*skel).progs.rbtree_api_release_aliasing),
			&mut opts,
		)
	};
	unsafe { ASSERT_OK(ret, c"rbtree_api_release_aliasing".as_ptr()) };
	unsafe { ASSERT_OK(opts.retval as c_int, c"rbtree_api_release_aliasing retval".as_ptr()) };
	unsafe {
		ASSERT_EQ(
			(*(*skel).data).first_data[0],
			42,
			c"rbtree_api_release_aliasing first rbtree_remove()".as_ptr(),
		)
	};
	unsafe {
		ASSERT_EQ(
			(*(*skel).data).first_data[1],
			-1,
			c"rbtree_api_release_aliasing second rbtree_remove()".as_ptr(),
		)
	};

	unsafe { rbtree__destroy(skel) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_rbtree_success() {
	if unsafe { test__start_subtest(c"rbtree_add_nodes".as_ptr()) } {
		unsafe { test_rbtree_add_nodes() };
	}
	if unsafe { test__start_subtest(c"rbtree_add_nodes_nested".as_ptr()) } {
		unsafe { test_rbtree_add_nodes_nested() };
	}
	if unsafe { test__start_subtest(c"rbtree_add_and_remove".as_ptr()) } {
		unsafe { test_rbtree_add_and_remove() };
	}
	if unsafe { test__start_subtest(c"rbtree_add_and_remove_array".as_ptr()) } {
		unsafe { test_rbtree_add_and_remove_array() };
	}
	if unsafe { test__start_subtest(c"rbtree_first_and_remove".as_ptr()) } {
		unsafe { test_rbtree_first_and_remove() };
	}
	if unsafe { test__start_subtest(c"rbtree_api_release_aliasing".as_ptr()) } {
		unsafe { test_rbtree_api_release_aliasing() };
	}
}

/* Original C used BTF_FAIL_TEST(suffix) to generate these functions. */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_rbtree_btf_fail__wrong_node_type() {
	let skel: *mut rbtree_btf_fail__wrong_node_type;

	skel = unsafe { rbtree_btf_fail__wrong_node_type__open_and_load() };
	if !unsafe {
		ASSERT_ERR_PTR(
			skel as *const c_void,
			c"rbtree_btf_fail__wrong_node_type__open_and_load unexpected success".as_ptr(),
		)
	} {
		unsafe { rbtree_btf_fail__wrong_node_type__destroy(skel) };
	}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_rbtree_btf_fail__add_wrong_type() {
	let skel: *mut rbtree_btf_fail__add_wrong_type;

	skel = unsafe { rbtree_btf_fail__add_wrong_type__open_and_load() };
	if !unsafe {
		ASSERT_ERR_PTR(
			skel as *const c_void,
			c"rbtree_btf_fail__add_wrong_type__open_and_load unexpected success".as_ptr(),
		)
	} {
		unsafe { rbtree_btf_fail__add_wrong_type__destroy(skel) };
	}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_rbtree_btf_fail() {
	if unsafe { test__start_subtest(c"rbtree_btf_fail__wrong_node_type".as_ptr()) } {
		unsafe { test_rbtree_btf_fail__wrong_node_type() };
	}
	if unsafe { test__start_subtest(c"rbtree_btf_fail__add_wrong_type".as_ptr()) } {
		unsafe { test_rbtree_btf_fail__add_wrong_type() };
	}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_rbtree_fail() {
	unsafe { RUN_TESTS_rbtree_fail() };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_rbtree_search() {
	unsafe { RUN_TESTS_rbtree_search() };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_rbtree_search_kptr() {
	unsafe { RUN_TESTS_rbtree_search_kptr() };
}
