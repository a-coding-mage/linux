// SPDX-License-Identifier: GPL-2.0-only

/* Connects 6 network namespaces through veths.
 * Each NS may have different IPv6 global scope addresses :
 *
 *          NS1            NS2             NS3              NS4               NS5             NS6
 *      lo  veth1 <-> veth2 veth3 <-> veth4 veth5 <-> veth6 lo veth7 <-> veth8 veth9 <-> veth10 lo
 * fb00 ::1  ::12      ::21  ::34      ::43  ::56      ::65     ::78      ::87  ::910     ::109  ::6
 * fd00                                                                                          ::4
 * fc42                                                     ::1
 *
 * All IPv6 packets going to fb00::/16 through NS2 will be encapsulated in a
 * IPv6 header with a Segment Routing Header, with segments :
 *	fd00::1 -> fd00::2 -> fd00::3 -> fd00::4
 *
 * 3 fd00::/16 IPv6 addresses are binded to seg6local End.BPF actions :
 * - fd00::1 : add a TLV, change the flags and apply a End.X action to fc42::1
 * - fd00::2 : remove the TLV, change the flags, add a tag
 * - fd00::3 : apply an End.T action to fd00::4, through routing table 117
 *
 * fd00::4 is a simple Segment Routing node decapsulating the inner IPv6 packet.
 * Each End.BPF action will validate the operations applied on the SRH by the
 * previous BPF program in the chain, otherwise the packet is dropped.
 *
 * An UDP datagram is sent from fb00::1 to fb00::6. The test succeeds if this
 * datagram can be read on NS6 when binding to fb00::6.
 */

// C dependencies: "network_helpers.h", "test_progs.h".

use core::ffi::{c_char, c_int, c_void};

const NETNS_BASE: &[u8] = b"lwt-seg6local-\0";
const NETNS_BASE_1: &[u8] = b"lwt-seg6local-1\0";
const NETNS_BASE_6: &[u8] = b"lwt-seg6local-6\0";
const BPF_FILE: &[u8] = b"test_lwt_seg6local.bpf.o\0";

const SERVER_PORT: c_int = 7330;
const CLIENT_PORT: c_int = 2121;

const AF_INET6: c_int = 10;
const SOCK_DGRAM: c_int = 2;

type SsizeT = isize;
type SocklenT = u32;

#[repr(C)]
pub struct in6_addr {
	pub s6_addr: [u8; 16],
}

#[repr(C)]
pub struct sockaddr {
	pub sa_family: u16,
	pub sa_data: [c_char; 14],
}

#[repr(C)]
pub struct sockaddr_in6 {
	pub sin6_family: u16,
	pub sin6_port: u16,
	pub sin6_flowinfo: u32,
	pub sin6_addr: in6_addr,
	pub sin6_scope_id: u32,
}

#[repr(C)]
pub struct nstoken {
	_private: [u8; 0],
}

unsafe extern "C" {
	fn open_netns(name: *const c_char) -> *mut nstoken;
	fn close_netns(token: *mut nstoken);
	fn start_server_str(
		family: c_int,
		type_: c_int,
		addr: *const c_char,
		port: c_int,
		opts: *mut c_void,
	) -> c_int;
	fn htons(hostshort: u16) -> u16;
	fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
	fn sendto(
		sockfd: c_int,
		buf: *const c_void,
		len: usize,
		flags: c_int,
		dest_addr: *const sockaddr,
		addrlen: SocklenT,
	) -> SsizeT;
	fn read(fd: c_int, buf: *mut c_void, count: usize) -> SsizeT;
	fn close(fd: c_int) -> c_int;
}

// External test macros from test_progs.h and command macros used by this test.
macro_rules! SYS_NOFAIL {
	($($arg:tt)*) => {
		todo!("external SYS_NOFAIL macro from test_progs.h")
	};
}

macro_rules! SYS {
	($label:ident, $($arg:tt)*) => {
		todo!("external SYS macro from test_progs.h")
	};
}

macro_rules! ASSERT_OK {
	($actual:expr, $name:expr) => {
		todo!("external ASSERT_OK macro from test_progs.h")
	};
}

macro_rules! ASSERT_OK_PTR {
	($actual:expr, $name:expr) => {
		todo!("external ASSERT_OK_PTR macro from test_progs.h")
	};
}

macro_rules! ASSERT_OK_FD {
	($actual:expr, $name:expr) => {
		todo!("external ASSERT_OK_FD macro from test_progs.h")
	};
}

macro_rules! ASSERT_EQ {
	($actual:expr, $expected:expr, $name:expr) => {
		todo!("external ASSERT_EQ macro from test_progs.h")
	};
}

macro_rules! ASSERT_STREQ {
	($actual:expr, $expected:expr, $name:expr) => {
		todo!("external ASSERT_STREQ macro from test_progs.h")
	};
}

unsafe fn cleanup() {
	let mut ns: c_int;

	ns = 1;
	while ns < 7 {
		SYS_NOFAIL!("ip netns del %s%d", NETNS_BASE.as_ptr(), ns);
		ns += 1;
	}
}

unsafe fn setup() -> c_int {
	let mut ns: c_int;

	ns = 1;
	while ns < 7 {
		SYS!(fail, "ip netns add %s%d", NETNS_BASE.as_ptr(), ns);
		ns += 1;
	}

	SYS!(fail, "ip -n %s6 link set dev lo up", NETNS_BASE.as_ptr());

	ns = 1;
	while ns < 6 {
		let local_id: c_int = ns * 2 - 1;
		let peer_id: c_int = ns * 2;
		let next_ns: c_int = ns + 1;

		SYS!(
			fail,
			"ip -n %s%d link add veth%d type veth peer name veth%d netns %s%d",
			NETNS_BASE.as_ptr(),
			ns,
			local_id,
			peer_id,
			NETNS_BASE.as_ptr(),
			next_ns
		);

		SYS!(
			fail,
			"ip -n %s%d link set dev veth%d up",
			NETNS_BASE.as_ptr(),
			ns,
			local_id
		);
		SYS!(
			fail,
			"ip -n %s%d link set dev veth%d up",
			NETNS_BASE.as_ptr(),
			next_ns,
			peer_id
		);

		/* All link scope addresses to veths */
		SYS!(
			fail,
			"ip -n %s%d -6 addr add fb00::%d%d/16 dev veth%d scope link",
			NETNS_BASE.as_ptr(),
			ns,
			local_id,
			peer_id,
			local_id
		);
		SYS!(
			fail,
			"ip -n %s%d -6 addr add fb00::%d%d/16 dev veth%d scope link",
			NETNS_BASE.as_ptr(),
			next_ns,
			peer_id,
			local_id,
			peer_id
		);

		ns += 1;
	}

	SYS!(
		fail,
		"ip -n %s5 -6 route add fb00::109 table 117 dev veth9 scope link",
		NETNS_BASE.as_ptr()
	);

	SYS!(fail, "ip -n %s1 -6 addr add fb00::1/16 dev lo", NETNS_BASE.as_ptr());
	SYS!(
		fail,
		"ip -n %s1 -6 route add fb00::6 dev veth1 via fb00::21",
		NETNS_BASE.as_ptr()
	);

	SYS!(
		fail,
		"ip -n %s2 -6 route add fb00::6 encap bpf in obj %s sec encap_srh dev veth2",
		NETNS_BASE.as_ptr(),
		BPF_FILE.as_ptr()
	);
	SYS!(
		fail,
		"ip -n %s2 -6 route add fd00::1 dev veth3 via fb00::43 scope link",
		NETNS_BASE.as_ptr()
	);

	SYS!(
		fail,
		"ip -n %s3 -6 route add fc42::1 dev veth5 via fb00::65",
		NETNS_BASE.as_ptr()
	);
	SYS!(
		fail,
		"ip -n %s3 -6 route add fd00::1 encap seg6local action End.BPF endpoint obj %s sec add_egr_x dev veth4",
		NETNS_BASE.as_ptr(),
		BPF_FILE.as_ptr()
	);

	SYS!(
		fail,
		"ip -n %s4 -6 route add fd00::2 encap seg6local action End.BPF endpoint obj %s sec pop_egr dev veth6",
		NETNS_BASE.as_ptr(),
		BPF_FILE.as_ptr()
	);
	SYS!(fail, "ip -n %s4 -6 addr add fc42::1 dev lo", NETNS_BASE.as_ptr());
	SYS!(
		fail,
		"ip -n %s4 -6 route add fd00::3 dev veth7 via fb00::87",
		NETNS_BASE.as_ptr()
	);

	SYS!(
		fail,
		"ip -n %s5 -6 route add fd00::4 table 117 dev veth9 via fb00::109",
		NETNS_BASE.as_ptr()
	);
	SYS!(
		fail,
		"ip -n %s5 -6 route add fd00::3 encap seg6local action End.BPF endpoint obj %s sec inspect_t dev veth8",
		NETNS_BASE.as_ptr(),
		BPF_FILE.as_ptr()
	);

	SYS!(fail, "ip -n %s6 -6 addr add fb00::6/16 dev lo", NETNS_BASE.as_ptr());
	SYS!(fail, "ip -n %s6 -6 addr add fd00::4/16 dev lo", NETNS_BASE.as_ptr());

	ns = 1;
	while ns < 6 {
		SYS!(
			fail,
			"ip netns exec %s%d sysctl -wq net.ipv6.conf.all.forwarding=1",
			NETNS_BASE.as_ptr(),
			ns
		);
		ns += 1;
	}

	SYS!(
		fail,
		"ip netns exec %s6 sysctl -wq net.ipv6.conf.all.seg6_enabled=1",
		NETNS_BASE.as_ptr()
	);
	SYS!(
		fail,
		"ip netns exec %s6 sysctl -wq net.ipv6.conf.lo.seg6_enabled=1",
		NETNS_BASE.as_ptr()
	);
	SYS!(
		fail,
		"ip netns exec %s6 sysctl -wq net.ipv6.conf.veth10.seg6_enabled=1",
		NETNS_BASE.as_ptr()
	);

	return 0;

	#[allow(unreachable_code)]
	{
		return -1;
	}
}

#[no_mangle]
pub unsafe extern "C" fn test_lwt_seg6local() {
	let mut server_addr: sockaddr_in6 = core::mem::zeroed();
	let ns1: *const c_char = NETNS_BASE_1.as_ptr() as *const c_char;
	let ns6: *const c_char = NETNS_BASE_6.as_ptr() as *const c_char;
	let mut nstoken: *mut nstoken = core::ptr::null_mut();
	let foobar: [c_char; 7] = [
		b'f' as c_char,
		b'o' as c_char,
		b'o' as c_char,
		b'b' as c_char,
		b'a' as c_char,
		b'r' as c_char,
		0,
	];
	let mut bytes: SsizeT;
	let mut sfd: c_int = 0;
	let mut cfd: c_int = 0;
	let mut buf: [c_char; 7] = [0; 7];

	if !ASSERT_OK!(setup(), "setup") {
		goto_out();
		return;
	}

	nstoken = open_netns(ns6);
	if !ASSERT_OK_PTR!(nstoken, "open ns6") {
		goto_out();
		return;
	}

	sfd = start_server_str(
		AF_INET6,
		SOCK_DGRAM,
		b"fb00::6\0".as_ptr() as *const c_char,
		SERVER_PORT,
		core::ptr::null_mut(),
	);
	if !ASSERT_OK_FD!(sfd, "start server") {
		goto_close_netns(nstoken);
		goto_out();
		return;
	}

	close_netns(nstoken);

	nstoken = open_netns(ns1);
	if !ASSERT_OK_PTR!(nstoken, "open ns1") {
		goto_close_server(sfd);
		goto_close_netns(nstoken);
		goto_out();
		return;
	}

	cfd = start_server_str(
		AF_INET6,
		SOCK_DGRAM,
		b"fb00::1\0".as_ptr() as *const c_char,
		CLIENT_PORT,
		core::ptr::null_mut(),
	);
	if !ASSERT_OK_FD!(cfd, "start client") {
		goto_close_server(sfd);
		goto_close_netns(nstoken);
		goto_out();
		return;
	}

	close_netns(nstoken);
	nstoken = core::ptr::null_mut();

	/* Send a packet larger than MTU */
	server_addr.sin6_family = AF_INET6 as u16;
	server_addr.sin6_port = htons(SERVER_PORT as u16);
	if !ASSERT_EQ!(
		inet_pton(
			AF_INET6,
			b"fb00::6\0".as_ptr() as *const c_char,
			&mut server_addr.sin6_addr as *mut in6_addr as *mut c_void
		),
		1,
		"build target addr"
	) {
		goto_close_client(cfd);
		goto_close_server(sfd);
		goto_close_netns(nstoken);
		goto_out();
		return;
	}

	bytes = sendto(
		cfd,
		foobar.as_ptr() as *const c_void,
		core::mem::size_of_val(&foobar),
		0,
		&server_addr as *const sockaddr_in6 as *const sockaddr,
		core::mem::size_of_val(&server_addr) as SocklenT,
	);
	if !ASSERT_EQ!(bytes, core::mem::size_of_val(&foobar) as SsizeT, "send packet") {
		goto_close_client(cfd);
		goto_close_server(sfd);
		goto_close_netns(nstoken);
		goto_out();
		return;
	}

	/* Verify we received all expected bytes */
	bytes = read(
		sfd,
		buf.as_mut_ptr() as *mut c_void,
		core::mem::size_of_val(&buf),
	);
	if !ASSERT_EQ!(bytes, core::mem::size_of_val(&buf) as SsizeT, "receive packet") {
		goto_close_client(cfd);
		goto_close_server(sfd);
		goto_close_netns(nstoken);
		goto_out();
		return;
	}
	ASSERT_STREQ!(buf.as_ptr(), foobar.as_ptr(), "check udp packet");

	goto_close_client(cfd);
	goto_close_server(sfd);
	goto_close_netns(nstoken);
	goto_out();
}

unsafe fn goto_close_client(cfd: c_int) {
	close(cfd);
}

unsafe fn goto_close_server(sfd: c_int) {
	close(sfd);
}

unsafe fn goto_close_netns(nstoken: *mut nstoken) {
	close_netns(nstoken);
}

unsafe fn goto_out() {
	cleanup();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
