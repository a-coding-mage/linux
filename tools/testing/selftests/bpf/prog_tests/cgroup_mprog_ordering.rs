// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */
// C dependencies: <test_progs.h>, "cgroup_helpers.h", "cgroup_preorder.skel.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type bool_ = bool;
type socklen_t = c_uint;
type __u8 = u8;

const BPF_F_ALLOW_MULTI: u32 = 1 << 1;
const BPF_F_BEFORE: u32 = 1 << 3;
const SOL_IP: c_int = 0;
const IP_TOS: c_int = 1;
const AF_INET: c_int = 2;
const SOCK_STREAM: c_int = 1;

#[repr(C)]
pub struct bpf_program {
	_private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum bpf_attach_type {
}

#[repr(C)]
pub struct bpf_prog_attach_opts {
	pub sz: usize,
	pub flags: u32,
	pub relative_fd: c_int,
}

#[repr(C)]
pub struct cgroup_preorder__progs {
	pub parent: *mut bpf_program,
	pub parent_2: *mut bpf_program,
}

#[repr(C)]
pub struct cgroup_preorder__bss {
	pub result: [__u8; 0],
}

#[repr(C)]
pub struct cgroup_preorder {
	pub progs: cgroup_preorder__progs,
	pub bss: *mut cgroup_preorder__bss,
}

unsafe extern "C" {
	fn cgroup_preorder__open_and_load() -> *mut cgroup_preorder;
	fn cgroup_preorder__destroy(skel: *mut cgroup_preorder);
	fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
	fn bpf_program__expected_attach_type(prog: *mut bpf_program) -> bpf_attach_type;
	fn bpf_prog_attach_opts(
		prog_fd: c_int,
		target_fd: c_int,
		atype: bpf_attach_type,
		opts: *const bpf_prog_attach_opts,
	) -> c_int;
	fn bpf_prog_detach2(prog_fd: c_int, target_fd: c_int, atype: bpf_attach_type) -> c_int;
	fn getsockopt(
		fd: c_int,
		level: c_int,
		optname: c_int,
		optval: *mut c_void,
		optlen: *mut socklen_t,
	) -> c_int;
	fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
	fn close(fd: c_int) -> c_int;
	fn test__join_cgroup(path: *const c_char) -> c_int;

	fn ASSERT_OK_PTR(ptr: *mut cgroup_preorder, name: *const c_char) -> bool_;
	fn ASSERT_OK(err: c_int, name: *const c_char) -> bool_;
	fn ASSERT_TRUE(expr: bool_, name: *const c_char) -> bool_;
	fn ASSERT_GE(a: c_int, b: c_int, name: *const c_char) -> bool_;
}

unsafe fn run_getsockopt_test(cg_parent: c_int, sock_fd: c_int, has_relative_fd: bool_) -> c_int {
	let mut opts = bpf_prog_attach_opts {
		sz: core::mem::size_of::<bpf_prog_attach_opts>(),
		flags: 0,
		relative_fd: 0,
	};
	let mut prog_p_atype: bpf_attach_type;
	let mut prog_p2_atype: bpf_attach_type;
	let prog_p_fd: c_int;
	let prog_p2_fd: c_int;
	let mut skel: *mut cgroup_preorder = ptr::null_mut();
	let mut prog: *mut bpf_program;
	let result: *mut __u8;
	let mut buf: __u8 = 0;
	let mut optlen: socklen_t = 1;
	let mut err: c_int = 0;

	skel = cgroup_preorder__open_and_load();
	if !ASSERT_OK_PTR(skel, c"cgroup_preorder__open_and_load".as_ptr()) {
		return 0;
	}

	opts = bpf_prog_attach_opts {
		sz: core::mem::size_of::<bpf_prog_attach_opts>(),
		flags: 0,
		relative_fd: 0,
	};
	opts.flags = BPF_F_ALLOW_MULTI;
	prog = (*skel).progs.parent;
	prog_p_fd = bpf_program__fd(prog);
	prog_p_atype = bpf_program__expected_attach_type(prog);
	err = bpf_prog_attach_opts(prog_p_fd, cg_parent, prog_p_atype, &opts);
	if !ASSERT_OK(err, c"bpf_prog_attach_opts-parent".as_ptr()) {
		goto_close_skel(skel);
		return err;
	}

	opts.flags = BPF_F_ALLOW_MULTI | BPF_F_BEFORE;
	if has_relative_fd {
		opts.relative_fd = prog_p_fd;
	}
	prog = (*skel).progs.parent_2;
	prog_p2_fd = bpf_program__fd(prog);
	prog_p2_atype = bpf_program__expected_attach_type(prog);
	err = bpf_prog_attach_opts(prog_p2_fd, cg_parent, prog_p2_atype, &opts);
	if !ASSERT_OK(err, c"bpf_prog_attach_opts-parent_2".as_ptr()) {
		goto_detach_parent(skel, prog_p_fd, cg_parent, prog_p_atype);
		return err;
	}

	err = getsockopt(
		sock_fd,
		SOL_IP,
		IP_TOS,
		&mut buf as *mut __u8 as *mut c_void,
		&mut optlen,
	);
	if !ASSERT_OK(err, c"getsockopt".as_ptr()) {
		goto_detach_parent_2(skel, prog_p2_fd, cg_parent, prog_p2_atype, prog_p_fd, prog_p_atype);
		return err;
	}

	result = (*(*skel).bss).result.as_mut_ptr();
	ASSERT_TRUE(
		*result.add(0) == 4 && *result.add(1) == 3,
		c"result values".as_ptr(),
	);

	goto_detach_parent_2(skel, prog_p2_fd, cg_parent, prog_p2_atype, prog_p_fd, prog_p_atype);
	err
}

unsafe fn goto_detach_parent_2(
	skel: *mut cgroup_preorder,
	prog_p2_fd: c_int,
	cg_parent: c_int,
	prog_p2_atype: bpf_attach_type,
	prog_p_fd: c_int,
	prog_p_atype: bpf_attach_type,
) {
	ASSERT_OK(
		bpf_prog_detach2(prog_p2_fd, cg_parent, prog_p2_atype),
		c"bpf_prog_detach2-parent_2".as_ptr(),
	);
	goto_detach_parent(skel, prog_p_fd, cg_parent, prog_p_atype);
}

unsafe fn goto_detach_parent(
	skel: *mut cgroup_preorder,
	prog_p_fd: c_int,
	cg_parent: c_int,
	prog_p_atype: bpf_attach_type,
) {
	ASSERT_OK(
		bpf_prog_detach2(prog_p_fd, cg_parent, prog_p_atype),
		c"bpf_prog_detach2-parent".as_ptr(),
	);
	goto_close_skel(skel);
}

unsafe fn goto_close_skel(skel: *mut cgroup_preorder) {
	cgroup_preorder__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_cgroup_mprog_ordering() {
	let mut cg_parent: c_int = -1;
	let mut sock_fd: c_int = -1;

	cg_parent = test__join_cgroup(c"/parent".as_ptr());
	if !ASSERT_GE(cg_parent, 0, c"join_cgroup /parent".as_ptr()) {
		close(sock_fd);
		close(cg_parent);
		return;
	}

	sock_fd = socket(AF_INET, SOCK_STREAM, 0);
	if !ASSERT_GE(sock_fd, 0, c"socket".as_ptr()) {
		close(sock_fd);
		close(cg_parent);
		return;
	}

	ASSERT_OK(
		run_getsockopt_test(cg_parent, sock_fd, false),
		c"getsockopt_test_1".as_ptr(),
	);
	ASSERT_OK(
		run_getsockopt_test(cg_parent, sock_fd, true),
		c"getsockopt_test_2".as_ptr(),
	);

	close(sock_fd);
	close(cg_parent);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
