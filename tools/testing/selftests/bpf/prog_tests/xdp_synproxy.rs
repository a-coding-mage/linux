// SPDX-License-Identifier: LGPL-2.1 OR BSD-2-Clause
/* Copyright (c) 2022, NVIDIA CORPORATION & AFFILIATES. All rights reserved. */

/* C dependencies: test_progs.h, network_helpers.h, ctype.h */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

const CMD_OUT_BUF_SIZE: usize = 1023;
const AF_INET: c_int = 2;
const SOCK_STREAM: c_int = 1;

#[repr(C)]
pub struct FILE {
	_private: [u8; 0],
}

#[repr(C)]
pub struct nstoken {
	_private: [u8; 0],
}

unsafe extern "C" {
	fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
	fn popen(command: *const c_char, type_: *const c_char) -> *mut FILE;
	fn pclose(stream: *mut FILE) -> c_int;
	fn fread(ptr: *mut c_void, size: usize, nmemb: usize, stream: *mut FILE) -> usize;
	fn strlen(s: *const c_char) -> usize;
	fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
	fn memmem(haystack: *const c_void, haystacklen: usize, needle: *const c_void,
		  needlelen: usize) -> *mut c_void;
	fn isprint(c: c_int) -> c_int;
	fn accept(sockfd: c_int, addr: *mut c_void, addrlen: *mut c_void) -> c_int;
	fn close(fd: c_int) -> c_int;

	fn open_netns(name: *const c_char) -> *mut nstoken;
	fn close_netns(token: *mut nstoken);
	fn start_server(family: c_int, type_: c_int, addr: *const c_char, port: c_uint,
			timeout_ms: c_int) -> c_int;
	fn connect_to_fd(server_fd: c_int, timeout_ms: c_int) -> c_int;
	fn test__start_subtest(name: *const c_char) -> bool;

	fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
	fn ASSERT_GE(actual: c_long, expected: c_long, name: *const c_char) -> bool;
	fn ASSERT_LT(actual: *const c_char, expected: *const c_char, name: *const c_char) -> bool;
	fn CHECK(condition: bool, name: *const c_char, fmt: *const c_char, ...) -> bool;
	fn SYS_NOFAIL(command: *const c_char, ...);
	fn SYS(label: *const c_char, command: *const c_char, ...);
}

macro_rules! c_str {
	($s:literal) => {
		concat!($s, "\0").as_ptr() as *const c_char
	};
}

/* Rust equivalent of SYS_OUT(cmd, ...): format a command, popen it, and jump to
 * cleanup when ASSERT_OK_PTR fails.  The variadic formatting is expressed at
 * each call site because Rust macros cannot directly forward C varargs.
 */

/* out must be at least `size * 4 + 1` bytes long */
unsafe fn escape_str(mut out: *mut c_char, in_: *const c_char, size: usize)
{
	static HEX: &[u8; 17] = b"0123456789ABCDEF\0";
	let mut i: usize;

	i = 0;
	while i < size {
		let ch = *in_.add(i);

		if isprint(ch as c_int) != 0 && ch != b'\\' as c_char &&
		   ch != b'\'' as c_char {
			*out = ch;
			out = out.add(1);
		} else {
			*out = b'\\' as c_char;
			out = out.add(1);
			*out = b'x' as c_char;
			out = out.add(1);
			*out = HEX[((ch >> 4) & 0xf) as usize] as c_char;
			out = out.add(1);
			*out = HEX[(ch & 0xf) as usize] as c_char;
			out = out.add(1);
		}
		i += 1;
	}
	*out = b'\0' as c_char;
	out = out.add(1);
	let _ = out;
}

unsafe fn expect_str(buf: *mut c_char, size: usize, str_: *const c_char,
		     name: *const c_char) -> bool
{
	static mut ESCBUF_EXPECTED: [c_char; CMD_OUT_BUF_SIZE * 4] =
		[0; CMD_OUT_BUF_SIZE * 4];
	static mut ESCBUF_ACTUAL: [c_char; CMD_OUT_BUF_SIZE * 4] =
		[0; CMD_OUT_BUF_SIZE * 4];
	static mut DURATION: c_int = 0;
	let ok: bool;

	ok = size == strlen(str_) && memcmp(buf as *const c_void, str_ as *const c_void, size) == 0;

	if !ok {
		escape_str(core::ptr::addr_of_mut!(ESCBUF_EXPECTED) as *mut c_char,
			   str_, strlen(str_));
		escape_str(core::ptr::addr_of_mut!(ESCBUF_ACTUAL) as *mut c_char,
			   buf, size);
	}
	CHECK(!ok, name,
	      c_str!("unexpected %s: actual '%s' != expected '%s'\n"),
	      name,
	      core::ptr::addr_of_mut!(ESCBUF_ACTUAL) as *mut c_char,
	      core::ptr::addr_of_mut!(ESCBUF_EXPECTED) as *mut c_char);

	let _ = core::ptr::addr_of_mut!(DURATION);
	ok
}

unsafe fn sys_out_noarg(fmt: *const c_char) -> *mut FILE
{
	let mut buf = [0 as c_char; 1024];
	snprintf(buf.as_mut_ptr(), buf.len(), fmt);
	let f = popen(buf.as_ptr(), c_str!("r"));
	if !ASSERT_OK_PTR(f as *const c_void, buf.as_ptr()) {
		return core::ptr::null_mut();
	}
	f
}

unsafe fn sys_out_bool(fmt: *const c_char, suffix: *const c_char) -> *mut FILE
{
	let mut buf = [0 as c_char; 1024];
	snprintf(buf.as_mut_ptr(), buf.len(), fmt, suffix);
	let f = popen(buf.as_ptr(), c_str!("r"));
	if !ASSERT_OK_PTR(f as *const c_void, buf.as_ptr()) {
		return core::ptr::null_mut();
	}
	f
}

unsafe fn sys_out_prog(fmt: *const c_char, prog_id: *const c_char) -> *mut FILE
{
	let mut buf = [0 as c_char; 1024];
	snprintf(buf.as_mut_ptr(), buf.len(), fmt, prog_id);
	let f = popen(buf.as_ptr(), c_str!("r"));
	if !ASSERT_OK_PTR(f as *const c_void, buf.as_ptr()) {
		return core::ptr::null_mut();
	}
	f
}

unsafe fn test_synproxy(xdp: bool)
{
	let mut server_fd: c_int = -1;
	let mut client_fd: c_int = -1;
	let mut accept_fd: c_int = -1;
	let mut prog_id: *mut c_char = core::ptr::null_mut();
	let mut prog_id_end: *mut c_char;
	let mut ns: *mut nstoken = core::ptr::null_mut();
	let mut ctrl_file: *mut FILE = core::ptr::null_mut();
	let mut buf = [0 as c_char; CMD_OUT_BUF_SIZE];
	let mut size: usize;

	'out: loop {
		SYS(c_str!("out"), c_str!("ip netns add synproxy"));

		SYS(c_str!("out"), c_str!("ip link add tmp0 type veth peer name tmp1"));
		SYS(c_str!("out"), c_str!("ip link set tmp1 netns synproxy"));
		SYS(c_str!("out"), c_str!("ip link set tmp0 up"));
		SYS(c_str!("out"), c_str!("ip addr replace 198.18.0.1/24 dev tmp0"));

		/* When checksum offload is enabled, the XDP program sees wrong
		 * checksums and drops packets.
		 */
		SYS(c_str!("out"), c_str!("ethtool -K tmp0 tx off"));
		if xdp {
			/* Workaround required for veth. */
			SYS(c_str!("out"), c_str!("ip link set tmp0 xdp object xdp_dummy.bpf.o section xdp 2> /dev/null"));
		}

		ns = open_netns(c_str!("synproxy"));
		if !ASSERT_OK_PTR(ns as *const c_void, c_str!("setns")) {
			break 'out;
		}

		SYS(c_str!("out"), c_str!("ip link set lo up"));
		SYS(c_str!("out"), c_str!("ip link set tmp1 up"));
		SYS(c_str!("out"), c_str!("ip addr replace 198.18.0.2/24 dev tmp1"));
		SYS(c_str!("out"), c_str!("sysctl -w net.ipv4.tcp_syncookies=2"));
		SYS(c_str!("out"), c_str!("sysctl -w net.ipv4.tcp_timestamps=1"));
		SYS(c_str!("out"), c_str!("sysctl -w net.netfilter.nf_conntrack_tcp_loose=0"));
		SYS(c_str!("out"), c_str!("iptables-legacy -t raw -I PREROUTING \
	    -i tmp1 -p tcp -m tcp --syn --dport 8080 -j CT --notrack"));
		SYS(c_str!("out"), c_str!("iptables-legacy -t filter -A INPUT \
	    -i tmp1 -p tcp -m tcp --dport 8080 -m state --state INVALID,UNTRACKED \
	    -j SYNPROXY --sack-perm --timestamp --wscale 7 --mss 1460"));
		SYS(c_str!("out"), c_str!("iptables-legacy -t filter -A INPUT \
	    -i tmp1 -m state --state INVALID -j DROP"));

		ctrl_file = sys_out_bool(c_str!("./xdp_synproxy --iface tmp1 --ports 8080 \
			    --single --mss4 1460 --mss6 1440 \
			    --wscale 7 --ttl 64%s"), if xdp { c_str!("") } else { c_str!(" --tc") });
		if ctrl_file.is_null() {
			break 'out;
		}
		size = fread(buf.as_mut_ptr() as *mut c_void, 1, buf.len(), ctrl_file);
		pclose(ctrl_file);
		if !expect_str(buf.as_mut_ptr(), size, c_str!("Total SYNACKs generated: 0\n"),
			       c_str!("initial SYNACKs")) {
			break 'out;
		}

		if !xdp {
			ctrl_file = sys_out_noarg(c_str!("tc filter show dev tmp1 ingress"));
			if ctrl_file.is_null() {
				break 'out;
			}
			size = fread(buf.as_mut_ptr() as *mut c_void, 1, buf.len(), ctrl_file);
			pclose(ctrl_file);
			prog_id = memmem(buf.as_ptr() as *const c_void, size,
					 c_str!(" id ") as *const c_void, 4) as *mut c_char;
			if !ASSERT_OK_PTR(prog_id as *const c_void, c_str!("find prog id")) {
				break 'out;
			}
			prog_id = prog_id.add(4);
			if !ASSERT_LT(prog_id as *const c_char,
				      buf.as_ptr().add(size) as *const c_char,
				      c_str!("find prog id begin")) {
				break 'out;
			}
			prog_id_end = prog_id;
			while prog_id_end < buf.as_mut_ptr().add(size) &&
			      *prog_id_end >= b'0' as c_char &&
			      *prog_id_end <= b'9' as c_char {
				prog_id_end = prog_id_end.add(1);
			}
			if !ASSERT_LT(prog_id_end as *const c_char,
				      buf.as_ptr().add(size) as *const c_char,
				      c_str!("find prog id end")) {
				break 'out;
			}
			*prog_id_end = b'\0' as c_char;
		}

		server_fd = start_server(AF_INET, SOCK_STREAM, c_str!("198.18.0.2"), 8080, 0);
		if !ASSERT_GE(server_fd as c_long, 0, c_str!("start_server")) {
			break 'out;
		}

		close_netns(ns);
		ns = core::ptr::null_mut();

		client_fd = connect_to_fd(server_fd, 10000);
		if !ASSERT_GE(client_fd as c_long, 0, c_str!("connect_to_fd")) {
			break 'out;
		}

		accept_fd = accept(server_fd, core::ptr::null_mut(), core::ptr::null_mut());
		if !ASSERT_GE(accept_fd as c_long, 0, c_str!("accept")) {
			break 'out;
		}

		ns = open_netns(c_str!("synproxy"));
		if !ASSERT_OK_PTR(ns as *const c_void, c_str!("setns")) {
			break 'out;
		}

		if xdp {
			ctrl_file = sys_out_noarg(c_str!("./xdp_synproxy --iface tmp1 --single"));
		} else {
			ctrl_file = sys_out_prog(c_str!("./xdp_synproxy --prog %s --single"),
						 prog_id as *const c_char);
		}
		if ctrl_file.is_null() {
			break 'out;
		}
		size = fread(buf.as_mut_ptr() as *mut c_void, 1, buf.len(), ctrl_file);
		pclose(ctrl_file);
		if !expect_str(buf.as_mut_ptr(), size, c_str!("Total SYNACKs generated: 1\n"),
			       c_str!("SYNACKs after connection")) {
			break 'out;
		}

		break 'out;
	}

	if accept_fd >= 0 {
		close(accept_fd);
	}
	if client_fd >= 0 {
		close(client_fd);
	}
	if server_fd >= 0 {
		close(server_fd);
	}
	if !ns.is_null() {
		close_netns(ns);
	}

	SYS_NOFAIL(c_str!("ip link del tmp0"));
	SYS_NOFAIL(c_str!("ip netns del synproxy"));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_xdp_synproxy()
{
	if test__start_subtest(c_str!("xdp")) {
		test_synproxy(true);
	}
	if test__start_subtest(c_str!("tc")) {
		test_synproxy(false);
	}
}
