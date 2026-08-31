// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2026 Christian Brauner */

/* Translated from testing/selftests/bpf/prog_tests/sock_xattr.c.
 * C includes referenced external libc, socket, xattr, test_progs, and
 * sock_read_xattr.skel.h declarations.
 */

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of_val;
use core::ptr;

static xattr_value: &[u8; 15] = b"bpf_sock_value\0";
static xattr_name: &[u8; 14] = b"user.bpf_test\0";

#[repr(C)]
pub struct bpf_link {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
	_private: [u8; 0],
}

#[repr(C)]
pub struct sock_read_xattr_bss {
	pub monitored_pid: c_int,
	pub value: *const c_char,
}

#[repr(C)]
pub struct sock_read_xattr_data {
	pub read_ret: c_int,
}

#[repr(C)]
pub struct sock_read_xattr_progs {
	pub read_sock_xattr: *mut bpf_program,
}

#[repr(C)]
pub struct sock_read_xattr {
	pub bss: *mut sock_read_xattr_bss,
	pub data: *mut sock_read_xattr_data,
	pub progs: sock_read_xattr_progs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in_addr {
	pub s_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr {
	pub sa_family: u16,
	pub sa_data: [c_char; 14],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_in {
	pub sin_family: u16,
	pub sin_port: u16,
	pub sin_addr: in_addr,
	pub sin_zero: [u8; 8],
}

const AF_INET: c_int = 2;
const SOCK_STREAM: c_int = 1;
const INADDR_LOOPBACK: u32 = 0x7f000001;

unsafe extern "C" {
	fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
	fn fsetxattr(
		fd: c_int,
		name: *const c_char,
		value: *const c_void,
		size: usize,
		flags: c_int,
	) -> c_int;
	fn sys_gettid() -> c_int;
	fn htons(hostshort: u16) -> u16;
	fn htonl(hostlong: u32) -> u32;
	fn connect(sockfd: c_int, addr: *const sockaddr, addrlen: u32) -> c_int;
	fn close(fd: c_int) -> c_int;

	fn sock_read_xattr__open_and_load() -> *mut sock_read_xattr;
	fn sock_read_xattr__destroy(obj: *mut sock_read_xattr);
	fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
	fn bpf_link__destroy(link: *mut bpf_link);

	fn ASSERT_OK_FD(fd: c_int, name: *const c_char) -> bool;
	fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
	fn ASSERT_OK_PTR<T>(ptr: *mut T, name: *const c_char) -> bool;
	fn ASSERT_EQ(left: c_int, right: usize, name: *const c_char) -> bool;
	fn ASSERT_STREQ(left: *const c_char, right: *const c_char, name: *const c_char) -> bool;
	fn RUN_TESTS(name: *const c_char);
	fn test__start_subtest(name: *const c_char) -> bool;
}

unsafe fn test_read_sock_xattr() {
	let mut addr: sockaddr_in = sockaddr_in {
		sin_family: 0,
		sin_port: 0,
		sin_addr: in_addr { s_addr: 0 },
		sin_zero: [0; 8],
	};
	let mut skel: *mut sock_read_xattr = ptr::null_mut();
	let mut link: *mut bpf_link = ptr::null_mut();
	let mut sock_fd: c_int = -1;
	let err: c_int;

	sock_fd = socket(AF_INET, SOCK_STREAM, 0);
	if !ASSERT_OK_FD(sock_fd, c"socket".as_ptr()) {
		return;
	}

	err = fsetxattr(
		sock_fd,
		xattr_name.as_ptr() as *const c_char,
		xattr_value.as_ptr() as *const c_void,
		size_of_val(xattr_value),
		0,
	);
	if !ASSERT_OK(err, c"fsetxattr".as_ptr()) {
		goto_out(skel, sock_fd, link);
		return;
	}

	skel = sock_read_xattr__open_and_load();
	if !ASSERT_OK_PTR(skel, c"sock_read_xattr__open_and_load".as_ptr()) {
		goto_out(skel, sock_fd, link);
		return;
	}

	(*(*skel).bss).monitored_pid = sys_gettid();

	/* Only attach the functional program; the verifier-only programs
	 * above are not pid-gated and would clobber the shared globals.
	 */
	link = bpf_program__attach((*skel).progs.read_sock_xattr);
	if !ASSERT_OK_PTR(link, c"attach read_sock_xattr".as_ptr()) {
		goto_out(skel, sock_fd, link);
		return;
	}

	addr.sin_family = AF_INET as u16;
	addr.sin_port = htons(1234);
	addr.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
	/* Only the lsm/socket_connect hook matters; the connect may fail. */
	connect(
		sock_fd,
		&addr as *const sockaddr_in as *const sockaddr,
		size_of_val(&addr) as u32,
	);

	ASSERT_EQ((*(*skel).data).read_ret, size_of_val(xattr_value), c"read_ret".as_ptr());
	ASSERT_STREQ(
		(*(*skel).bss).value,
		xattr_value.as_ptr() as *const c_char,
		c"value".as_ptr(),
	);

	goto_out(skel, sock_fd, link);
}

unsafe fn goto_out(skel: *mut sock_read_xattr, sock_fd: c_int, link: *mut bpf_link) {
	bpf_link__destroy(link);
	if sock_fd >= 0 {
		close(sock_fd);
	}
	sock_read_xattr__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_sock_xattr() {
	RUN_TESTS(c"sock_read_xattr".as_ptr());

	if test__start_subtest(c"read_sock_xattr".as_ptr()) {
		test_read_sock_xattr();
	}
}
