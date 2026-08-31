// SPDX-License-Identifier: GPL-2.0

use std::ffi::{c_char, c_int, c_void};
use std::ptr;

// C dependencies:
// #include <test_progs.h>
// #include "connect4_dropper.skel.h"
// #include "cgroup_helpers.h"
// #include "network_helpers.h"

const AF_INET: c_int = 2;
const SOCK_STREAM: c_int = 1;
const EPERM: c_int = 1;

#[repr(C)]
pub struct bpf_program {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
	_private: [u8; 0],
}

#[repr(C)]
pub struct connect4_dropper_bss {
	pub port: c_int,
}

#[repr(C)]
pub struct connect4_dropper_progs {
	pub connect_v4_dropper: *mut bpf_program,
}

#[repr(C)]
pub struct connect4_dropper_links {
	pub connect_v4_dropper: *mut bpf_link,
}

#[repr(C)]
pub struct connect4_dropper {
	pub bss: *mut connect4_dropper_bss,
	pub progs: connect4_dropper_progs,
	pub links: connect4_dropper_links,
}

#[repr(C)]
pub struct network_helper_opts {
	_private: [u8; 0],
}

unsafe extern "C" {
	fn connect4_dropper__open_and_load() -> *mut connect4_dropper;
	fn connect4_dropper__destroy(obj: *mut connect4_dropper);
	fn bpf_program__attach_cgroup(prog: *mut bpf_program, cgroup_fd: c_int) -> *mut bpf_link;

	fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
	fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
	fn ASSERT_OK(res: c_int, name: *const c_char) -> bool;

	fn get_socket_local_port(fd: c_int) -> c_int;
	fn connect_to_fd_opts(server_fd: c_int, opts: *const network_helper_opts) -> c_int;
	fn start_server(family: c_int, type_: c_int, addr: *const c_void, port: c_int, timeout_ms: c_int) -> c_int;
	fn test__join_cgroup(path: *const c_char) -> c_int;

	fn join_classid() -> c_int;
	fn setup_classid_environment();
	fn set_classid();
	fn cleanup_classid_environment();

	fn ntohs(netshort: u16) -> u16;
	fn close(fd: c_int) -> c_int;
	fn log_err(fmt: *const c_char, ...);
	fn __errno_location() -> *mut c_int;
}

unsafe fn run_test(cgroup_fd: c_int, server_fd: c_int, classid: bool) -> c_int {
	let mut fd: c_int;
	let mut err: c_int = 0;
	let port: c_int;

	let skel = connect4_dropper__open_and_load();
	if !ASSERT_OK_PTR(skel as *const c_void, c"skel_open".as_ptr()) {
		return -1;
	}

	port = get_socket_local_port(server_fd);
	if !ASSERT_GE(port, 0, c"get_socket_local_port".as_ptr()) {
		return -1;
	}

	(*(*skel).bss).port = ntohs(port as u16) as c_int;

	(*skel).links.connect_v4_dropper =
		bpf_program__attach_cgroup((*skel).progs.connect_v4_dropper, cgroup_fd);
	if !ASSERT_OK_PTR(
		(*skel).links.connect_v4_dropper as *const c_void,
		c"prog_attach".as_ptr(),
	) {
		err = -1;
		goto_out(skel);
		return err;
	}

	if classid && !ASSERT_OK(join_classid(), c"join_classid".as_ptr()) {
		err = -1;
		goto_out(skel);
		return err;
	}

	*__errno_location() = 0;
	fd = connect_to_fd_opts(server_fd, ptr::null());
	if fd >= 0 {
		log_err(c"Unexpected success to connect to server".as_ptr());
		err = -1;
		close(fd);
	} else if *__errno_location() != EPERM {
		log_err(c"Unexpected errno from connect to server".as_ptr());
		err = -1;
	}

	goto_out(skel);
	err
}

unsafe fn goto_out(skel: *mut connect4_dropper) {
	connect4_dropper__destroy(skel);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_cgroup_v1v2() {
	let opts: network_helper_opts = std::mem::zeroed();
	let mut server_fd: c_int;
	let client_fd: c_int;
	let cgroup_fd: c_int;

	/* Step 1: Check base connectivity works without any BPF. */
	server_fd = start_server(AF_INET, SOCK_STREAM, ptr::null(), 0, 0);
	if !ASSERT_GE(server_fd, 0, c"server_fd".as_ptr()) {
		return;
	}
	client_fd = connect_to_fd_opts(server_fd, &opts);
	if !ASSERT_GE(client_fd, 0, c"client_fd".as_ptr()) {
		close(server_fd);
		return;
	}
	close(client_fd);
	close(server_fd);

	/* Step 2: Check BPF policy prog attached to cgroups drops connectivity. */
	cgroup_fd = test__join_cgroup(c"/connect_dropper".as_ptr());
	if !ASSERT_GE(cgroup_fd, 0, c"cgroup_fd".as_ptr()) {
		return;
	}
	server_fd = start_server(AF_INET, SOCK_STREAM, ptr::null(), 0, 0);
	if !ASSERT_GE(server_fd, 0, c"server_fd".as_ptr()) {
		close(cgroup_fd);
		return;
	}
	ASSERT_OK(run_test(cgroup_fd, server_fd, false), c"cgroup-v2-only".as_ptr());
	setup_classid_environment();
	set_classid();
	ASSERT_OK(run_test(cgroup_fd, server_fd, true), c"cgroup-v1v2".as_ptr());
	cleanup_classid_environment();
	close(server_fd);
	close(cgroup_fd);
}
