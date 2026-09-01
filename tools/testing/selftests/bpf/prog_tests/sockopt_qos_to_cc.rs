// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */
/* Dependencies from the original C file:
 * <test_progs.h>
 * <netinet/tcp.h>
 * "sockopt_qos_to_cc.skel.h"
 */

use std::ffi::c_void;
use std::mem;
use std::os::raw::{c_char, c_int, c_long};

type socklen_t = u32;

const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const SOL_TCP: c_int = 6;
const SOL_IPV6: c_int = 41;
const IPV6_TCLASS: c_int = 67;
const TCP_CONGESTION: c_int = 13;
const _SC_PAGESIZE: c_int = 30;

#[repr(C)]
pub struct bpf_program {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
	_private: [u8; 0],
}

#[repr(C)]
pub struct sockopt_qos_to_cc__bss {
	pub page_size: c_long,
}

#[repr(C)]
pub struct sockopt_qos_to_cc__progs {
	pub sockopt_qos_to_cc: *mut bpf_program,
}

#[repr(C)]
pub struct sockopt_qos_to_cc__links {
	pub sockopt_qos_to_cc: *mut bpf_link,
}

#[repr(C)]
pub struct sockopt_qos_to_cc {
	pub bss: *mut sockopt_qos_to_cc__bss,
	pub progs: sockopt_qos_to_cc__progs,
	pub links: sockopt_qos_to_cc__links,
}

unsafe extern "C" {
	fn setsockopt(
		socket: c_int,
		level: c_int,
		option_name: c_int,
		option_value: *const c_void,
		option_len: socklen_t,
	) -> c_int;
	fn getsockopt(
		socket: c_int,
		level: c_int,
		option_name: c_int,
		option_value: *mut c_void,
		option_len: *mut socklen_t,
	) -> c_int;
	fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
	fn close(fd: c_int) -> c_int;
	fn sysconf(name: c_int) -> c_long;

	fn test__join_cgroup(path: *const c_char) -> c_int;
	fn sockopt_qos_to_cc__open_and_load() -> *mut sockopt_qos_to_cc;
	fn sockopt_qos_to_cc__destroy(obj: *mut sockopt_qos_to_cc);
	fn bpf_program__attach_cgroup(prog: *mut bpf_program, cgroup_fd: c_int) -> *mut bpf_link;

	fn ASSERT_OK(res: c_int, name: *const c_char) -> bool;
	fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
	fn ASSERT_GE(res: c_int, expected: c_int, name: *const c_char) -> bool;
	fn ASSERT_STREQ(actual: *const c_char, expected: *const c_char, name: *const c_char) -> bool;
}

unsafe fn run_setsockopt_test(cg_fd: c_int, sock_fd: c_int) {
	let mut optlen: socklen_t;
	let mut cc: [c_char; 16] = [0; 16]; /* TCP_CA_NAME_MAX */
	let mut buf: c_int;
	let mut err: c_int = -1;

	let _ = cg_fd;

	buf = 0x2D;
	err = setsockopt(
		sock_fd,
		SOL_IPV6,
		IPV6_TCLASS,
		&buf as *const c_int as *const c_void,
		mem::size_of_val(&buf) as socklen_t,
	);
	if !ASSERT_OK(err, c"setsockopt(sock_fd, IPV6_TCLASS)".as_ptr()) {
		return;
	}

	/* Verify the setsockopt cc change */
	optlen = mem::size_of_val(&cc) as socklen_t;
	err = getsockopt(
		sock_fd,
		SOL_TCP,
		TCP_CONGESTION,
		cc.as_mut_ptr() as *mut c_void,
		&mut optlen,
	);
	if !ASSERT_OK(err, c"getsockopt(sock_fd, TCP_CONGESTION)".as_ptr()) {
		return;
	}

	if !ASSERT_STREQ(
		cc.as_ptr(),
		c"reno".as_ptr(),
		c"getsockopt(sock_fd, TCP_CONGESTION)".as_ptr(),
	) {
		return;
	}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_sockopt_qos_to_cc() {
	let mut skel: *mut sockopt_qos_to_cc;
	let cc_cubic: [c_char; 16] = [
		b'c' as c_char,
		b'u' as c_char,
		b'b' as c_char,
		b'i' as c_char,
		b'c' as c_char,
		0,
		0,
		0,
		0,
		0,
		0,
		0,
		0,
		0,
		0,
		0,
	]; /* TCP_CA_NAME_MAX */
	let mut cg_fd: c_int = -1;
	let mut sock_fd: c_int = -1;
	let mut err: c_int;

	cg_fd = test__join_cgroup(c"/sockopt_qos_to_cc".as_ptr());
	if !ASSERT_GE(cg_fd, 0, c"cg-join(sockopt_qos_to_cc)".as_ptr()) {
		return;
	}

	skel = sockopt_qos_to_cc__open_and_load();
	if !ASSERT_OK_PTR(skel as *const c_void, c"skel".as_ptr()) {
		goto_done(skel, sock_fd, cg_fd);
		return;
	}

	(*(*skel).bss).page_size = sysconf(_SC_PAGESIZE);

	sock_fd = socket(AF_INET6, SOCK_STREAM, 0);
	if !ASSERT_GE(sock_fd, 0, c"v6 socket open".as_ptr()) {
		goto_done(skel, sock_fd, cg_fd);
		return;
	}

	err = setsockopt(
		sock_fd,
		SOL_TCP,
		TCP_CONGESTION,
		&cc_cubic as *const [c_char; 16] as *const c_void,
		mem::size_of_val(&cc_cubic) as socklen_t,
	);
	if !ASSERT_OK(err, c"setsockopt(sock_fd, TCP_CONGESTION)".as_ptr()) {
		goto_done(skel, sock_fd, cg_fd);
		return;
	}

	(*skel).links.sockopt_qos_to_cc =
		bpf_program__attach_cgroup((*skel).progs.sockopt_qos_to_cc, cg_fd);
	if !ASSERT_OK_PTR(
		(*skel).links.sockopt_qos_to_cc as *const c_void,
		c"prog_attach(sockopt_qos_to_cc)".as_ptr(),
	) {
		goto_done(skel, sock_fd, cg_fd);
		return;
	}

	run_setsockopt_test(cg_fd, sock_fd);

	goto_done(skel, sock_fd, cg_fd);
}

unsafe fn goto_done(skel: *mut sockopt_qos_to_cc, sock_fd: c_int, cg_fd: c_int) {
	if sock_fd != -1 {
		close(sock_fd);
	}
	if cg_fd != -1 {
		close(cg_fd);
	}
	/* destroy can take null and error pointer */
	sockopt_qos_to_cc__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
