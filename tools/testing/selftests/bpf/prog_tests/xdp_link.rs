// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */
/* Rust translation of testing/selftests/bpf/prog_tests/xdp_link.c.
 * C includes translated as external dependencies:
 * <uapi/linux/if_link.h>, <test_progs.h>, "test_xdp_link.skel.h"
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

const IFINDEX_LO: c_int = 1;

const XDP_FLAGS_REPLACE: c_uint = 1 << 4;
const BPF_LINK_TYPE_XDP: c_uint = 6;

#[repr(C)]
pub struct bpf_program {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bpf_xdp_attach_opts {
	pub sz: usize,
	pub old_prog_fd: c_int,
}

#[repr(C)]
pub struct test_xdp_link__progs {
	pub xdp_handler: *mut bpf_program,
	pub tc_handler: *mut bpf_program,
}

#[repr(C)]
pub struct test_xdp_link__links {
	pub xdp_handler: *mut bpf_link,
	pub tc_handler: *mut bpf_link,
}

#[repr(C)]
pub struct test_xdp_link {
	pub progs: test_xdp_link__progs,
	pub links: test_xdp_link__links,
}

#[repr(C)]
pub struct bpf_prog_info {
	pub id: u32,
}

#[repr(C)]
pub struct bpf_link_info_xdp {
	pub ifindex: u32,
}

#[repr(C)]
pub union bpf_link_info_union {
	pub xdp: bpf_link_info_xdp,
}

#[repr(C)]
pub struct bpf_link_info {
	pub type_: u32,
	pub id: u32,
	pub prog_id: u32,
	pub u: bpf_link_info_union,
}

extern "C" {
	fn test_xdp_link__open_and_load() -> *mut test_xdp_link;
	fn test_xdp_link__destroy(obj: *mut test_xdp_link);

	fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
	fn bpf_program__attach_xdp(prog: *mut bpf_program, ifindex: c_int) -> *mut bpf_link;

	fn bpf_prog_get_info_by_fd(
		fd: c_int,
		info: *mut bpf_prog_info,
		info_len: *mut u32,
	) -> c_int;
	fn bpf_link_get_info_by_fd(
		fd: c_int,
		info: *mut bpf_link_info,
		info_len: *mut u32,
	) -> c_int;

	fn bpf_xdp_attach(
		ifindex: c_int,
		prog_fd: c_int,
		flags: c_uint,
		opts: *mut bpf_xdp_attach_opts,
	) -> c_int;
	fn bpf_xdp_detach(
		ifindex: c_int,
		flags: c_uint,
		opts: *mut bpf_xdp_attach_opts,
	) -> c_int;
	fn bpf_xdp_query_id(ifindex: c_int, flags: c_uint, prog_id: *mut u32) -> c_int;

	fn bpf_link__destroy(link: *mut bpf_link);
	fn bpf_link__update_program(link: *mut bpf_link, prog: *mut bpf_program) -> c_int;
	fn bpf_link__fd(link: *mut bpf_link) -> c_int;
	fn bpf_link__detach(link: *mut bpf_link) -> c_int;

	fn ASSERT_OK(res: c_int, name: *const c_char) -> bool;
	fn ASSERT_ERR(res: c_int, name: *const c_char) -> bool;
	fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
	fn ASSERT_ERR_PTR(ptr: *const c_void, name: *const c_char) -> bool;
	fn ASSERT_EQ(actual: u32, expected: u32, name: *const c_char) -> bool;
}

#[no_mangle]
pub unsafe extern "C" fn serial_test_xdp_link() {
	let mut skel1: *mut test_xdp_link = ptr::null_mut();
	let mut skel2: *mut test_xdp_link = ptr::null_mut();
	let mut id1: u32;
	let mut id2: u32;
	let mut id0: u32 = 0;
	let mut prog_fd1: c_int;
	let mut prog_fd2: c_int;
	let mut opts: bpf_xdp_attach_opts = mem::zeroed();
	opts.sz = mem::size_of::<bpf_xdp_attach_opts>();
	let mut link_info: bpf_link_info = mem::zeroed();
	let mut prog_info: bpf_prog_info = mem::zeroed();
	let mut link: *mut bpf_link;
	let mut err: c_int;
	let mut link_info_len: u32 = mem::size_of::<bpf_link_info>() as u32;
	let mut prog_info_len: u32 = mem::size_of::<bpf_prog_info>() as u32;

	skel1 = test_xdp_link__open_and_load();
	if !ASSERT_OK_PTR(skel1 as *const c_void, b"skel_load\0".as_ptr() as *const c_char) {
		goto_cleanup(skel1, skel2);
		return;
	}
	prog_fd1 = bpf_program__fd((*skel1).progs.xdp_handler);

	skel2 = test_xdp_link__open_and_load();
	if !ASSERT_OK_PTR(skel2 as *const c_void, b"skel_load\0".as_ptr() as *const c_char) {
		goto_cleanup(skel1, skel2);
		return;
	}
	prog_fd2 = bpf_program__fd((*skel2).progs.xdp_handler);

	prog_info = mem::zeroed();
	err = bpf_prog_get_info_by_fd(prog_fd1, &mut prog_info, &mut prog_info_len);
	if !ASSERT_OK(err, b"fd_info1\0".as_ptr() as *const c_char) {
		goto_cleanup(skel1, skel2);
		return;
	}
	id1 = prog_info.id;

	prog_info = mem::zeroed();
	err = bpf_prog_get_info_by_fd(prog_fd2, &mut prog_info, &mut prog_info_len);
	if !ASSERT_OK(err, b"fd_info2\0".as_ptr() as *const c_char) {
		goto_cleanup(skel1, skel2);
		return;
	}
	id2 = prog_info.id;

	/* set initial prog attachment */
	err = bpf_xdp_attach(IFINDEX_LO, prog_fd1, XDP_FLAGS_REPLACE, &mut opts);
	if !ASSERT_OK(err, b"fd_attach\0".as_ptr() as *const c_char) {
		goto_cleanup(skel1, skel2);
		return;
	}

	/* validate prog ID */
	err = bpf_xdp_query_id(IFINDEX_LO, 0, &mut id0);
	if !ASSERT_OK(err, b"id1_check_err\0".as_ptr() as *const c_char)
		|| !ASSERT_EQ(id0, id1, b"id1_check_val\0".as_ptr() as *const c_char)
	{
		goto_cleanup(skel1, skel2);
		return;
	}

	/* BPF link is not allowed to replace prog attachment */
	link = bpf_program__attach_xdp((*skel1).progs.xdp_handler, IFINDEX_LO);
	if !ASSERT_ERR_PTR(link as *const c_void, b"link_attach_should_fail\0".as_ptr() as *const c_char) {
		bpf_link__destroy(link);
		/* best-effort detach prog */
		opts.old_prog_fd = prog_fd1;
		bpf_xdp_detach(IFINDEX_LO, XDP_FLAGS_REPLACE, &mut opts);
		goto_cleanup(skel1, skel2);
		return;
	}

	/* detach BPF program */
	opts.old_prog_fd = prog_fd1;
	err = bpf_xdp_detach(IFINDEX_LO, XDP_FLAGS_REPLACE, &mut opts);
	if !ASSERT_OK(err, b"prog_detach\0".as_ptr() as *const c_char) {
		goto_cleanup(skel1, skel2);
		return;
	}

	/* now BPF link should attach successfully */
	link = bpf_program__attach_xdp((*skel1).progs.xdp_handler, IFINDEX_LO);
	if !ASSERT_OK_PTR(link as *const c_void, b"link_attach\0".as_ptr() as *const c_char) {
		goto_cleanup(skel1, skel2);
		return;
	}
	(*skel1).links.xdp_handler = link;

	/* validate prog ID */
	err = bpf_xdp_query_id(IFINDEX_LO, 0, &mut id0);
	if !ASSERT_OK(err, b"id1_check_err\0".as_ptr() as *const c_char)
		|| !ASSERT_EQ(id0, id1, b"id1_check_val\0".as_ptr() as *const c_char)
	{
		goto_cleanup(skel1, skel2);
		return;
	}

	/* BPF prog attach is not allowed to replace BPF link */
	opts.old_prog_fd = prog_fd1;
	err = bpf_xdp_attach(IFINDEX_LO, prog_fd2, XDP_FLAGS_REPLACE, &mut opts);
	if !ASSERT_ERR(err, b"prog_attach_fail\0".as_ptr() as *const c_char) {
		goto_cleanup(skel1, skel2);
		return;
	}

	/* Can't force-update when BPF link is active */
	err = bpf_xdp_attach(IFINDEX_LO, prog_fd2, 0, ptr::null_mut());
	if !ASSERT_ERR(err, b"prog_update_fail\0".as_ptr() as *const c_char) {
		goto_cleanup(skel1, skel2);
		return;
	}

	/* Can't force-detach when BPF link is active */
	err = bpf_xdp_detach(IFINDEX_LO, 0, ptr::null_mut());
	if !ASSERT_ERR(err, b"prog_detach_fail\0".as_ptr() as *const c_char) {
		goto_cleanup(skel1, skel2);
		return;
	}

	/* BPF link is not allowed to replace another BPF link */
	link = bpf_program__attach_xdp((*skel2).progs.xdp_handler, IFINDEX_LO);
	if !ASSERT_ERR_PTR(link as *const c_void, b"link_attach_should_fail\0".as_ptr() as *const c_char) {
		bpf_link__destroy(link);
		goto_cleanup(skel1, skel2);
		return;
	}

	bpf_link__destroy((*skel1).links.xdp_handler);
	(*skel1).links.xdp_handler = ptr::null_mut();

	/* new link attach should succeed */
	link = bpf_program__attach_xdp((*skel2).progs.xdp_handler, IFINDEX_LO);
	if !ASSERT_OK_PTR(link as *const c_void, b"link_attach\0".as_ptr() as *const c_char) {
		goto_cleanup(skel1, skel2);
		return;
	}
	(*skel2).links.xdp_handler = link;

	err = bpf_xdp_query_id(IFINDEX_LO, 0, &mut id0);
	if !ASSERT_OK(err, b"id2_check_err\0".as_ptr() as *const c_char)
		|| !ASSERT_EQ(id0, id2, b"id2_check_val\0".as_ptr() as *const c_char)
	{
		goto_cleanup(skel1, skel2);
		return;
	}

	/* updating program under active BPF link works as expected */
	err = bpf_link__update_program(link, (*skel1).progs.xdp_handler);
	if !ASSERT_OK(err, b"link_upd\0".as_ptr() as *const c_char) {
		goto_cleanup(skel1, skel2);
		return;
	}

	link_info = mem::zeroed();
	err = bpf_link_get_info_by_fd(bpf_link__fd(link), &mut link_info, &mut link_info_len);
	if !ASSERT_OK(err, b"link_info\0".as_ptr() as *const c_char) {
		goto_cleanup(skel1, skel2);
		return;
	}

	ASSERT_EQ(link_info.type_, BPF_LINK_TYPE_XDP, b"link_type\0".as_ptr() as *const c_char);
	ASSERT_EQ(link_info.prog_id, id1, b"link_prog_id\0".as_ptr() as *const c_char);
	ASSERT_EQ(link_info.u.xdp.ifindex, IFINDEX_LO as u32, b"link_ifindex\0".as_ptr() as *const c_char);

	/* updating program under active BPF link with different type fails */
	err = bpf_link__update_program(link, (*skel1).progs.tc_handler);
	if !ASSERT_ERR(err, b"link_upd_invalid\0".as_ptr() as *const c_char) {
		goto_cleanup(skel1, skel2);
		return;
	}

	err = bpf_link__detach(link);
	if !ASSERT_OK(err, b"link_detach\0".as_ptr() as *const c_char) {
		goto_cleanup(skel1, skel2);
		return;
	}

	link_info = mem::zeroed();
	err = bpf_link_get_info_by_fd(bpf_link__fd(link), &mut link_info, &mut link_info_len);

	ASSERT_OK(err, b"link_info\0".as_ptr() as *const c_char);
	ASSERT_EQ(link_info.prog_id, id1, b"link_prog_id\0".as_ptr() as *const c_char);
	/* ifindex should be zeroed out */
	ASSERT_EQ(link_info.u.xdp.ifindex, 0, b"link_ifindex\0".as_ptr() as *const c_char);

	goto_cleanup(skel1, skel2);
}

unsafe fn goto_cleanup(skel1: *mut test_xdp_link, skel2: *mut test_xdp_link) {
	test_xdp_link__destroy(skel1);
	test_xdp_link__destroy(skel2);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
