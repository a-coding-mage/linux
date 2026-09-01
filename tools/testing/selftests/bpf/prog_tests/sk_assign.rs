// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2018 Facebook
// Copyright (c) 2019 Cloudflare
// Copyright (c) 2020 Isovalent, Inc.
/*
 * Test that the socket assign program is able to redirect traffic towards a
 * socket, regardless of whether the port or address destination of the traffic
 * matches the port.
 */

// C dependencies: <fcntl.h>, <signal.h>, <stdlib.h>, <unistd.h>,
// "test_progs.h", and "network_helpers.h".

use core::ffi::{c_char, c_int, c_void};

const BIND_PORT: u16 = 1234;
const CONNECT_PORT: u16 = 4321;
const TEST_DADDR: u32 = 0xC0A80203;
const NS_SELF: &[u8] = b"/proc/self/ns/net\0";
const SERVER_MAP_PATH: &[u8] = b"/sys/fs/bpf/tc/globals/server_map\0";

const BUFSIZ: usize = 8192;
const O_RDONLY: c_int = 0;
const CLONE_NEWNET: c_int = 0x40000000;
const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const SOCK_DGRAM: c_int = 2;
const INADDR_LOOPBACK: u32 = 0x7f000001;
const BPF_ANY: u64 = 0;
const VERBOSE_VERY: c_int = 2;

type socklen_t = u32;
type ssize_t = isize;
type in_port_t = u16;
type __u16 = u16;
type __s64 = i64;

#[repr(C)]
struct FILE {
	_private: [u8; 0],
}

#[repr(C)]
struct in_addr {
	s_addr: u32,
}

#[repr(C)]
struct sockaddr {
	sa_family: u16,
	sa_data: [c_char; 14],
}

#[repr(C)]
struct sockaddr_in {
	sin_family: u16,
	sin_port: in_port_t,
	sin_addr: in_addr,
	sin_zero: [u8; 8],
}

#[repr(C)]
union in6_addr_inner {
	s6_addr: [u8; 16],
	s6_addr32: [u32; 4],
}

#[repr(C)]
struct in6_addr {
	in6_u: in6_addr_inner,
}

#[repr(C)]
struct sockaddr_in6 {
	sin6_family: u16,
	sin6_port: in_port_t,
	sin6_flowinfo: u32,
	sin6_addr: in6_addr,
	sin6_scope_id: u32,
}

#[repr(C)]
struct sockaddr_storage {
	ss_family: u16,
	_padding: [u8; 126],
}

#[repr(C)]
struct test_sk_cfg {
	name: *const c_char,
	family: c_int,
	addr: *mut sockaddr,
	len: socklen_t,
	type_: c_int,
	rewrite_addr: bool,
}

#[repr(C)]
struct env_t {
	verbosity: c_int,
}

unsafe extern "C" {
	static mut stop: c_int;
	static mut duration: c_int;
	static mut env: env_t;
	static in6addr_loopback: in6_addr;

	fn popen(command: *const c_char, type_: *const c_char) -> *mut FILE;
	fn pclose(stream: *mut FILE) -> c_int;
	fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
	fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
	fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
	fn system(command: *const c_char) -> c_int;
	fn unshare(flags: c_int) -> c_int;
	fn getsockname(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
	fn read(fd: c_int, buf: *mut c_void, count: usize) -> ssize_t;
	fn recvfrom(
		sockfd: c_int,
		buf: *mut c_void,
		len: usize,
		flags: c_int,
		src_addr: *mut sockaddr,
		addrlen: *mut socklen_t,
	) -> ssize_t;
	fn write(fd: c_int, buf: *const c_void, count: usize) -> ssize_t;
	fn accept(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
	fn close(fd: c_int) -> c_int;
	fn htons(hostshort: u16) -> u16;
	fn htonl(hostlong: u32) -> u32;
	fn ntohs(netshort: u16) -> u16;
	fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
	fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
	static mut stderr: *mut FILE;
	fn perror(s: *const c_char);
	fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
	fn bpf_obj_get(pathname: *const c_char) -> c_int;
	fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64) -> c_int;
	fn unlink(pathname: *const c_char) -> c_int;
	fn setns(fd: c_int, nstype: c_int) -> c_int;

	fn connect_to_addr(
		type_: c_int,
		addr: *mut sockaddr_storage,
		len: socklen_t,
		opts: *mut c_void,
	) -> c_int;
	fn start_server_addr(
		type_: c_int,
		addr: *const sockaddr_storage,
		len: socklen_t,
		opts: *mut c_void,
	) -> c_int;
	fn test__start_subtest(name: *const c_char) -> bool;
}

macro_rules! cstr {
	($s:literal) => {
		concat!($s, "\0").as_ptr() as *const c_char
	};
}

// External test macros from test_progs.h.
macro_rules! CHECK_FAIL {
	($cond:expr) => {
		$cond
	};
}

macro_rules! CHECK {
	($cond:expr, $name:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {
		$cond
	};
}

macro_rules! READ_ONCE {
	($expr:expr) => {
		core::ptr::read_volatile(core::ptr::addr_of!($expr))
	};
}

macro_rules! WRITE_ONCE {
	($expr:expr, $val:expr) => {
		core::ptr::write_volatile(core::ptr::addr_of_mut!($expr), $val)
	};
}

unsafe fn configure_stack() -> bool {
	let mut tc_version: [c_char; 128] = [0; 128];
	let mut tc_cmd: [c_char; BUFSIZ] = [0; BUFSIZ];
	let prog: *const c_char;
	let tc: *mut FILE;

	/* Check whether tc is built with libbpf. */
	tc = popen(cstr!("tc -V"), cstr!("r"));
	if CHECK_FAIL!(tc.is_null()) {
		return false;
	}
	if CHECK_FAIL!(fgets(tc_version.as_mut_ptr(), tc_version.len() as c_int, tc).is_null()) {
		pclose(tc);
		return false;
	}
	if !strstr(tc_version.as_ptr(), cstr!(", libbpf ")).is_null() {
		prog = cstr!("test_sk_assign_libbpf.bpf.o");
	} else {
		prog = cstr!("test_sk_assign.bpf.o");
	}
	if CHECK_FAIL!(pclose(tc) != 0) {
		return false;
	}

	/* Move to a new networking namespace */
	if CHECK_FAIL!(unshare(CLONE_NEWNET) != 0) {
		return false;
	}

	/* Configure necessary links, routes */
	if CHECK_FAIL!(system(cstr!("ip link set dev lo up")) != 0) {
		return false;
	}
	if CHECK_FAIL!(system(cstr!("ip route add local default dev lo")) != 0) {
		return false;
	}
	if CHECK_FAIL!(system(cstr!("ip -6 route add local default dev lo")) != 0) {
		return false;
	}

	/* Load qdisc, BPF program */
	if CHECK_FAIL!(system(cstr!("tc qdisc add dev lo clsact")) != 0) {
		return false;
	}
	sprintf(
		tc_cmd.as_mut_ptr(),
		cstr!("%s %s %s %s %s"),
		cstr!("tc filter add dev lo ingress bpf"),
		cstr!("direct-action object-file"),
		prog,
		cstr!("section tc"),
		if env.verbosity < VERBOSE_VERY {
			cstr!(" 2>/dev/null")
		} else {
			cstr!("verbose")
		},
	);
	if CHECK!(
		system(tc_cmd.as_ptr()) != 0,
		cstr!("BPF load failed;"),
		cstr!("run with -vv for more info\n")
	) {
		return false;
	}

	true
}

unsafe fn get_port(fd: c_int) -> in_port_t {
	let mut ss: sockaddr_storage = core::mem::zeroed();
	let mut slen: socklen_t = core::mem::size_of_val(&ss) as socklen_t;
	let mut port: in_port_t = 0;

	if CHECK_FAIL!(getsockname(fd, &mut ss as *mut _ as *mut sockaddr, &mut slen) != 0) {
		return port;
	}

	match ss.ss_family as c_int {
		AF_INET => {
			port = (*(core::ptr::addr_of_mut!(ss) as *mut sockaddr_in)).sin_port;
		}
		AF_INET6 => {
			port = (*(core::ptr::addr_of_mut!(ss) as *mut sockaddr_in6)).sin6_port;
		}
		_ => {
			CHECK!(true, cstr!("Invalid address family"), cstr!("%d\n"), ss.ss_family as c_int);
		}
	}
	port
}

unsafe fn rcv_msg(srv_client: c_int, type_: c_int) -> ssize_t {
	let mut buf: [c_char; BUFSIZ] = [0; BUFSIZ];

	if type_ == SOCK_STREAM {
		read(srv_client, buf.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&buf))
	} else {
		recvfrom(
			srv_client,
			buf.as_mut_ptr() as *mut c_void,
			core::mem::size_of_val(&buf),
			0,
			core::ptr::null_mut(),
			core::ptr::null_mut(),
		)
	}
}

unsafe fn run_test(server_fd: c_int, addr: *const sockaddr, len: socklen_t, type_: c_int) -> c_int {
	let mut client: c_int = -1;
	let mut srv_client: c_int = -1;
	let buf = *b"testing\0";
	let mut port: in_port_t;
	let mut ret: c_int = 1;

	client = connect_to_addr(type_, addr as *mut sockaddr_storage, len, core::ptr::null_mut());
	if client == -1 {
		perror(cstr!("Cannot connect to server"));
		goto_out(server_fd, client, srv_client, ret);
		return ret;
	}

	if type_ == SOCK_STREAM {
		srv_client = accept(server_fd, core::ptr::null_mut(), core::ptr::null_mut());
		if CHECK_FAIL!(srv_client == -1) {
			perror(cstr!("Can't accept connection"));
			goto_out(server_fd, client, srv_client, ret);
			return ret;
		}
	} else {
		srv_client = server_fd;
	}
	if CHECK_FAIL!(write(client, buf.as_ptr() as *const c_void, core::mem::size_of_val(&buf)) != core::mem::size_of_val(&buf) as ssize_t) {
		perror(cstr!("Can't write on client"));
		goto_out(server_fd, client, srv_client, ret);
		return ret;
	}
	if CHECK_FAIL!(rcv_msg(srv_client, type_) != core::mem::size_of_val(&buf) as ssize_t) {
		perror(cstr!("Can't read on server"));
		goto_out(server_fd, client, srv_client, ret);
		return ret;
	}

	port = get_port(srv_client);
	if CHECK_FAIL!(port == 0) {
		goto_out(server_fd, client, srv_client, ret);
		return ret;
	}
	/* SOCK_STREAM is connected via accept(), so the server's local address
	 * will be the CONNECT_PORT rather than the BIND port that corresponds
	 * to the listen socket. SOCK_DGRAM on the other hand is connectionless
	 * so we can't really do the same check there; the server doesn't ever
	 * create a socket with CONNECT_PORT.
	 */
	if type_ == SOCK_STREAM
		&& CHECK!(
			port != htons(CONNECT_PORT),
			cstr!("Expected"),
			cstr!("port %u but got %u"),
			CONNECT_PORT as c_int,
			ntohs(port) as c_int
		)
	{
		goto_out(server_fd, client, srv_client, ret);
		return ret;
	} else if type_ == SOCK_DGRAM
		&& CHECK!(
			port != htons(BIND_PORT),
			cstr!("Expected"),
			cstr!("port %u but got %u"),
			BIND_PORT as c_int,
			ntohs(port) as c_int
		)
	{
		goto_out(server_fd, client, srv_client, ret);
		return ret;
	}

	ret = 0;
	goto_out(server_fd, client, srv_client, ret);
	ret
}

unsafe fn goto_out(server_fd: c_int, client: c_int, srv_client: c_int, ret: c_int) {
	close(client);
	if srv_client != server_fd {
		close(srv_client);
	}
	if ret != 0 {
		WRITE_ONCE!(stop, 1);
	}
}

unsafe fn prepare_addr(addr: *mut sockaddr, family: c_int, port: __u16, rewrite_addr: bool) {
	let addr4: *mut sockaddr_in;
	let addr6: *mut sockaddr_in6;

	match family {
		AF_INET => {
			addr4 = addr as *mut sockaddr_in;
			memset(addr4 as *mut c_void, 0, core::mem::size_of::<sockaddr_in>());
			(*addr4).sin_family = family as u16;
			(*addr4).sin_port = htons(port);
			if rewrite_addr {
				(*addr4).sin_addr.s_addr = htonl(TEST_DADDR);
			} else {
				(*addr4).sin_addr.s_addr = htonl(INADDR_LOOPBACK);
			}
		}
		AF_INET6 => {
			addr6 = addr as *mut sockaddr_in6;
			memset(addr6 as *mut c_void, 0, core::mem::size_of::<sockaddr_in6>());
			(*addr6).sin6_family = family as u16;
			(*addr6).sin6_port = htons(port);
			(*addr6).sin6_addr = in6addr_loopback;
			if rewrite_addr {
				(*addr6).sin6_addr.in6_u.s6_addr32[3] = htonl(TEST_DADDR);
			}
		}
		_ => {
			fprintf(stderr, cstr!("Invalid family %d"), family);
		}
	}
}

macro_rules! TEST {
	($name:literal, $family:expr, $type_:expr, $rewrite:expr, $addr4:expr, $addr6:expr) => {
		test_sk_cfg {
			name: cstr!($name),
			family: $family,
			addr: if $family == AF_INET {
				$addr4 as *mut sockaddr_in as *mut sockaddr
			} else {
				$addr6 as *mut sockaddr_in6 as *mut sockaddr
			},
			len: if $family == AF_INET {
				core::mem::size_of::<sockaddr_in>() as socklen_t
			} else {
				core::mem::size_of::<sockaddr_in6>() as socklen_t
			},
			type_: $type_,
			rewrite_addr: $rewrite,
		}
	};
}

#[no_mangle]
pub unsafe extern "C" fn test_sk_assign() {
	let mut addr4: sockaddr_in = core::mem::zeroed();
	let mut addr6: sockaddr_in6 = core::mem::zeroed();
	let mut tests = [
		TEST!("ipv4 tcp port redir", AF_INET, SOCK_STREAM, false, &mut addr4, &mut addr6),
		TEST!("ipv4 tcp addr redir", AF_INET, SOCK_STREAM, true, &mut addr4, &mut addr6),
		TEST!("ipv6 tcp port redir", AF_INET6, SOCK_STREAM, false, &mut addr4, &mut addr6),
		TEST!("ipv6 tcp addr redir", AF_INET6, SOCK_STREAM, true, &mut addr4, &mut addr6),
		TEST!("ipv4 udp port redir", AF_INET, SOCK_DGRAM, false, &mut addr4, &mut addr6),
		TEST!("ipv4 udp addr redir", AF_INET, SOCK_DGRAM, true, &mut addr4, &mut addr6),
		TEST!("ipv6 udp port redir", AF_INET6, SOCK_DGRAM, false, &mut addr4, &mut addr6),
		TEST!("ipv6 udp addr redir", AF_INET6, SOCK_DGRAM, true, &mut addr4, &mut addr6),
	];
	let mut server: __s64 = -1;
	let server_map: c_int;
	let self_net: c_int;
	let mut i: usize;

	self_net = open(NS_SELF.as_ptr() as *const c_char, O_RDONLY);
	if CHECK_FAIL!(self_net < 0) {
		perror(cstr!("Unable to open /proc/self/ns/net"));
		return;
	}

	if !configure_stack() {
		perror(cstr!("configure_stack"));
		goto_cleanup(self_net);
		return;
	}

	server_map = bpf_obj_get(SERVER_MAP_PATH.as_ptr() as *const c_char);
	if CHECK_FAIL!(server_map < 0) {
		perror(cstr!("Unable to open /sys/fs/bpf/tc/globals/server_map"));
		goto_cleanup(self_net);
		return;
	}

	i = 0;
	while i < tests.len() && READ_ONCE!(stop) == 0 {
		let test: *mut test_sk_cfg = &mut tests[i];
		let addr: *const sockaddr;
		let zero: c_int = 0;
		let err: c_int;

		if !test__start_subtest((*test).name) {
			i += 1;
			continue;
		}
		prepare_addr((*test).addr, (*test).family, BIND_PORT, false);
		addr = (*test).addr as *const sockaddr;
		server = start_server_addr(
			(*test).type_,
			addr as *const sockaddr_storage,
			(*test).len,
			core::ptr::null_mut(),
		) as __s64;
		if server == -1 {
			break;
		}

		err = bpf_map_update_elem(
			server_map,
			&zero as *const _ as *const c_void,
			&server as *const _ as *const c_void,
			BPF_ANY,
		);
		if CHECK_FAIL!(err != 0) {
			perror(cstr!("Unable to update server_map"));
			break;
		}

		/* connect to unbound ports */
		prepare_addr((*test).addr, (*test).family, CONNECT_PORT, (*test).rewrite_addr);
		if run_test(server as c_int, addr, (*test).len, (*test).type_) != 0 {
			break;
		}

		close(server as c_int);
		server = -1;
		i += 1;
	}

	close(server as c_int);
	close(server_map);
	goto_cleanup(self_net);
}

unsafe fn goto_cleanup(self_net: c_int) {
	if CHECK_FAIL!(unlink(SERVER_MAP_PATH.as_ptr() as *const c_char) != 0) {
		perror(cstr!("Unable to unlink /sys/fs/bpf/tc/globals/server_map"));
	}
	if CHECK_FAIL!(setns(self_net, CLONE_NEWNET) != 0) {
		perror(cstr!("Failed to setns(/proc/self/ns/net)"));
	}
	close(self_net);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
