// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Isovalent */

/* C dependencies:
 * #include <arpa/inet.h>
 * #include "test_progs.h"
 * #include "network_helpers.h"
 * #include "ksock_lsm.skel.h"
 * #include "ksock_lsm_verifier.skel.h"
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, size_of_val, zeroed};
use core::ptr;

const NS_TEST: &[u8] = b"ksock_lsm_ns\0";
const RECV_PORT: c_int = 7777;
const RECV_TIMEOUT_SEC: i64 = 5;

const AF_INET: c_int = 2;
const SOCK_DGRAM: c_int = 2;
const IPPROTO_UDP: c_int = 17;
const SOL_SOCKET: c_int = 1;
const SO_RCVTIMEO: c_int = 20;
const INADDR_LOOPBACK: u32 = 0x7f000001;

#[repr(C)]
struct nstoken {
	_private: [u8; 0],
}

#[repr(C)]
struct bpf_program {
	_private: [u8; 0],
}

#[repr(C)]
struct bpf_link {
	_private: [u8; 0],
}

#[repr(C)]
struct bpf_test_run_opts {
	sz: usize,
	retval: u32,
}

#[repr(C)]
struct in_addr {
	s_addr: u32,
}

#[repr(C)]
struct sockaddr {
	sa_family: u16,
	sa_data: [u8; 14],
}

#[repr(C)]
struct sockaddr_in {
	sin_family: u16,
	sin_port: u16,
	sin_addr: in_addr,
	sin_zero: [u8; 8],
}

#[repr(C)]
struct timeval {
	tv_sec: i64,
	tv_usec: i64,
}

#[repr(C)]
struct ksock_test_env {
	nstoken: *mut nstoken,
	rfd: c_int,
}

#[repr(C)]
struct ksock_lsm {
	bss: *mut ksock_lsm_bss,
	data: *mut ksock_lsm_data,
	progs: ksock_lsm_progs,
	links: ksock_lsm_links,
}

#[repr(C)]
struct ksock_lsm_bss {
	ipv4_remote: u32,
	remote_port: c_int,
	target_pid: c_int,
}

#[repr(C)]
struct ksock_lsm_data {
	send_data: [u8; 0],
	send_ret: isize,
}

#[repr(C)]
struct ksock_lsm_progs {
	ksock_setup: *mut bpf_program,
	ksock_socket_bind: *mut bpf_program,
}

#[repr(C)]
struct ksock_lsm_links {
	ksock_socket_bind: *mut bpf_link,
}

unsafe extern "C" {
	fn htonl(hostlong: u32) -> u32;
	fn htons(hostshort: u16) -> u16;
	fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
	fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
	fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: u32) -> c_int;
	fn setsockopt(
		sockfd: c_int,
		level: c_int,
		optname: c_int,
		optval: *const c_void,
		optlen: u32,
	) -> c_int;
	fn recvfrom(
		sockfd: c_int,
		buf: *mut c_void,
		len: usize,
		flags: c_int,
		src_addr: *mut sockaddr,
		addrlen: *mut u32,
	) -> isize;
	fn close(fd: c_int) -> c_int;
	fn getpid() -> c_int;

	fn make_netns(name: *const c_char) -> c_int;
	fn open_netns(name: *const c_char) -> *mut nstoken;
	fn close_netns(token: *mut nstoken);
	fn remove_netns(name: *const c_char);

	fn ksock_lsm__open_and_load() -> *mut ksock_lsm;
	fn ksock_lsm__destroy(obj: *mut ksock_lsm);
	fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
	fn bpf_prog_test_run_opts(fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
	fn bpf_program__attach_lsm(prog: *mut bpf_program) -> *mut bpf_link;

	fn ASSERT_OK(res: c_int, name: *const c_char) -> bool;
	fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
	fn ASSERT_OK_FD(fd: c_int, name: *const c_char) -> bool;
	fn ASSERT_EQ(actual: isize, expected: isize, name: *const c_char) -> bool;
	fn ASSERT_MEMEQ(actual: *const c_void, expected: *const c_void, len: usize, name: *const c_char);
	fn RUN_TESTS(name: *const c_char);
}

unsafe fn ksock_test_env_setup(env: *mut ksock_test_env) -> bool {
	let addr = sockaddr_in {
		sin_family: AF_INET as u16,
		sin_addr: in_addr {
			s_addr: htonl(INADDR_LOOPBACK),
		},
		sin_port: htons(RECV_PORT as u16),
		sin_zero: [0; 8],
	};
	let tv = timeval {
		tv_sec: RECV_TIMEOUT_SEC,
		tv_usec: 0,
	};
	let mut err: c_int;

	memset(
		env as *mut c_void,
		0,
		size_of::<ksock_test_env>(),
	);
	(*env).rfd = -1;

	if !ASSERT_OK(make_netns(NS_TEST.as_ptr() as *const c_char), c"make_netns".as_ptr()) {
		return false;
	}

	(*env).nstoken = open_netns(NS_TEST.as_ptr() as *const c_char);
	if !ASSERT_OK_PTR((*env).nstoken as *const c_void, c"open_netns".as_ptr()) {
		return false;
	}

	(*env).rfd = socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP);
	if !ASSERT_OK_FD((*env).rfd, c"receiver socket".as_ptr()) {
		return false;
	}

	err = bind(
		(*env).rfd,
		&addr as *const sockaddr_in as *const sockaddr,
		size_of::<sockaddr_in>() as u32,
	);
	if !ASSERT_OK(err, c"bind receiver".as_ptr()) {
		return false;
	}

	err = setsockopt(
		(*env).rfd,
		SOL_SOCKET,
		SO_RCVTIMEO,
		&tv as *const timeval as *const c_void,
		size_of::<timeval>() as u32,
	);
	if !ASSERT_OK(err, c"set rcvtimeo".as_ptr()) {
		return false;
	}

	true
}

#[no_mangle]
pub unsafe extern "C" fn test_ksock_lsm() {
	let mut opts: bpf_test_run_opts = zeroed();
	opts.sz = size_of::<bpf_test_run_opts>();
	let mut env: ksock_test_env = zeroed();
	let trigger_addr = sockaddr_in {
		sin_family: AF_INET as u16,
		sin_addr: in_addr {
			s_addr: htonl(INADDR_LOOPBACK),
		},
		sin_port: 0,
		sin_zero: [0; 8],
	};
	let mut skel: *mut ksock_lsm;
	let mut n: isize;
	let mut tfd: c_int = -1;
	let mut err: c_int;

	skel = ksock_lsm__open_and_load();
	if !ASSERT_OK_PTR(skel as *const c_void, c"skel open_and_load".as_ptr()) {
		return;
	}

	if !ksock_test_env_setup(&mut env) {
		goto_fail(skel, tfd, &mut env);
		return;
	}

	/* Step 1: Run the setup SYSCALL prog to create the ksock */
	(*(*skel).bss).ipv4_remote = htonl(INADDR_LOOPBACK);
	(*(*skel).bss).remote_port = RECV_PORT;
	err = bpf_prog_test_run_opts(
		bpf_program__fd((*skel).progs.ksock_setup),
		&mut opts,
	);
	if !ASSERT_OK(err, c"ksock_setup run".as_ptr()) {
		goto_fail(skel, tfd, &mut env);
		return;
	}
	if !ASSERT_OK(opts.retval as c_int, c"ksock_setup retval".as_ptr()) {
		goto_fail(skel, tfd, &mut env);
		return;
	}

	/* Step 2: Attach LSM prog and trigger socket_bind from userspace */
	(*skel).links.ksock_socket_bind = bpf_program__attach_lsm((*skel).progs.ksock_socket_bind);
	if !ASSERT_OK_PTR(
		(*skel).links.ksock_socket_bind as *const c_void,
		c"attach socket_bind lsm".as_ptr(),
	) {
		goto_fail(skel, tfd, &mut env);
		return;
	}

	tfd = socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP);
	if !ASSERT_OK_FD(tfd, c"trigger socket".as_ptr()) {
		goto_fail(skel, tfd, &mut env);
		return;
	}

	(*(*skel).bss).target_pid = getpid();
	err = bind(
		tfd,
		&trigger_addr as *const sockaddr_in as *const sockaddr,
		size_of::<sockaddr_in>() as u32,
	);
	(*(*skel).bss).target_pid = 0;
	if !ASSERT_OK(err, c"trigger bind".as_ptr()) {
		goto_fail(skel, tfd, &mut env);
		return;
	}

	/* Step 3: Verify the LSM hook sent the notification */
	let mut recv_data: [u8; 0] = [];
	if !ASSERT_EQ(
		(*(*skel).data).send_ret,
		size_of_val(&(*(*skel).data).send_data) as isize,
		c"LSM send bytes".as_ptr(),
	) {
		goto_fail(skel, tfd, &mut env);
		return;
	}

	n = recvfrom(
		env.rfd,
		recv_data.as_mut_ptr() as *mut c_void,
		recv_data.len(),
		0,
		ptr::null_mut(),
		ptr::null_mut(),
	);
	if ASSERT_EQ(n, recv_data.len() as isize, c"recvfrom len".as_ptr()) {
		ASSERT_MEMEQ(
			recv_data.as_ptr() as *const c_void,
			(*(*skel).data).send_data.as_ptr() as *const c_void,
			recv_data.len(),
			c"payload match".as_ptr(),
		);
	}

	goto_fail(skel, tfd, &mut env);
}

unsafe fn goto_fail(skel: *mut ksock_lsm, tfd: c_int, env: *mut ksock_test_env) {
	if tfd >= 0 {
		close(tfd);
	}
	if (*env).rfd >= 0 {
		close((*env).rfd);
	}
	if !(*env).nstoken.is_null() {
		close_netns((*env).nstoken);
	}
	remove_netns(NS_TEST.as_ptr() as *const c_char);
	ksock_lsm__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_ksock_lsm_verifier() {
	RUN_TESTS(c"ksock_lsm_verifier".as_ptr());
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
