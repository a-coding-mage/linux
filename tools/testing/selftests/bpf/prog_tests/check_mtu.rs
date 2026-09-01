// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Jesper Dangaard Brouer */

/* Translated from C. Original includes:
 * <linux/if_link.h> before test_progs.h, to avoid bpf_util.h redefines
 * <test_progs.h>
 * "test_check_mtu.skel.h"
 * "network_helpers.h"
 * <stdlib.h>
 * <inttypes.h>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const IFINDEX_LO: u32 = 1;
const O_RDONLY: c_int = 0;
const ERANGE: c_int = 34;

static mut duration: u32 = 0; /* Hint: needed for CHECK macro */

#[repr(C)]
pub struct bpf_program {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link_info_xdp {
	pub ifindex: u32,
}

#[repr(C)]
pub struct bpf_link_info {
	pub type_: u32,
	pub xdp: bpf_link_info_xdp,
}

#[repr(C)]
pub struct bpf_test_run_opts {
	pub repeat: u32,
	pub data_in: *const c_void,
	pub data_size_in: u32,
	pub data_out: *mut c_void,
	pub data_size_out: u32,
	pub ctx_in: *const c_void,
	pub ctx_size_in: u32,
	pub retval: u32,
}

#[repr(C)]
pub struct __sk_buff {
	pub gso_size: u32,
}

#[repr(C)]
pub struct test_check_mtu__progs {
	pub xdp_use_helper_basic: *mut bpf_program,
	pub xdp_use_helper: *mut bpf_program,
	pub xdp_exceed_mtu: *mut bpf_program,
	pub xdp_minus_delta: *mut bpf_program,
	pub xdp_input_len: *mut bpf_program,
	pub xdp_input_len_exceed: *mut bpf_program,
	pub tc_use_helper: *mut bpf_program,
	pub tc_exceed_mtu: *mut bpf_program,
	pub tc_exceed_mtu_da: *mut bpf_program,
	pub tc_minus_delta: *mut bpf_program,
	pub tc_input_len: *mut bpf_program,
	pub tc_input_len_exceed: *mut bpf_program,
	pub tc_chk_segs_flag: *mut bpf_program,
}

#[repr(C)]
pub struct test_check_mtu__links {
	pub xdp_use_helper_basic: *mut bpf_link,
}

#[repr(C)]
pub struct test_check_mtu__rodata {
	pub GLOBAL_USER_MTU: u32,
	pub GLOBAL_USER_IFINDEX: u32,
}

#[repr(C)]
pub struct test_check_mtu__bss {
	pub global_bpf_mtu_xdp: u32,
	pub global_bpf_mtu_tc: u32,
}

#[repr(C)]
pub struct test_check_mtu {
	pub progs: test_check_mtu__progs,
	pub links: test_check_mtu__links,
	pub rodata: *mut test_check_mtu__rodata,
	pub bss: *mut test_check_mtu__bss,
}

unsafe extern "C" {
	static mut errno: c_int;
	static pkt_v4: c_void;
	static BPF_LINK_TYPE_XDP: u32;
	static XDP_PASS: c_int;
	static BPF_OK: c_int;

	fn open(pathname: *const c_char, flags: c_int, mode: c_int) -> c_int;
	fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
	fn close(fd: c_int) -> c_int;
	fn strtoimax(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> i64;
	fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;

	fn test_check_mtu__open_and_load() -> *mut test_check_mtu;
	fn test_check_mtu__open() -> *mut test_check_mtu;
	fn test_check_mtu__load(skel: *mut test_check_mtu) -> c_int;
	fn test_check_mtu__destroy(skel: *mut test_check_mtu);

	fn bpf_program__attach_xdp(prog: *mut bpf_program, ifindex: c_int) -> *mut bpf_link;
	fn bpf_link__fd(link: *mut bpf_link) -> c_int;
	fn bpf_link_get_info_by_fd(
		fd: c_int,
		info: *mut bpf_link_info,
		info_len: *mut u32,
	) -> c_int;
	fn bpf_link__detach(link: *mut bpf_link) -> c_int;
	fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
	fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;

	fn test__start_subtest(name: *const c_char) -> bool;
}

unsafe fn read_mtu_device_lo() -> c_int {
	let filename = b"/sys/class/net/lo/mtu\0";
	let mut buf: [c_char; 11] = [0; 11];
	let value: c_int;
	let n: isize;
	let fd: c_int;

	fd = open(filename.as_ptr() as *const c_char, 0, O_RDONLY);
	if fd == -1 {
		return -1;
	}

	n = read(fd, buf.as_mut_ptr() as *mut c_void, size_of::<[c_char; 11]>());
	close(fd);

	if n == -1 {
		return -2;
	}

	value = strtoimax(buf.as_ptr(), ptr::null_mut(), 10) as c_int;
	if errno == ERANGE {
		return -3;
	}

	value
}

unsafe fn test_check_mtu_xdp_attach() {
	let mut link_info: bpf_link_info = core::mem::zeroed();
	let mut link_info_len: u32 = size_of::<bpf_link_info>() as u32;
	let skel: *mut test_check_mtu;
	let prog: *mut bpf_program;
	let link: *mut bpf_link;
	let mut err: c_int = 0;
	let fd: c_int;

	skel = test_check_mtu__open_and_load();
	if CHECK!(skel.is_null(), "open and load skel", "failed") {
		return; /* Exit if e.g. helper unknown to kernel */
	}

	'out: {
		prog = (*skel).progs.xdp_use_helper_basic;

		link = bpf_program__attach_xdp(prog, IFINDEX_LO as c_int);
		if !ASSERT_OK_PTR!(link, "link_attach") {
			break 'out;
		}
		(*skel).links.xdp_use_helper_basic = link;

		memset(
			&mut link_info as *mut bpf_link_info as *mut c_void,
			0,
			size_of::<bpf_link_info>(),
		);
		fd = bpf_link__fd(link);
		err = bpf_link_get_info_by_fd(fd, &mut link_info, &mut link_info_len);
		if CHECK!(err != 0, "link_info", "failed: %d\n", err) {
			break 'out;
		}

		CHECK!(
			link_info.type_ != BPF_LINK_TYPE_XDP,
			"link_type",
			"got %u != exp %u\n",
			link_info.type_,
			BPF_LINK_TYPE_XDP
		);
		CHECK!(
			link_info.xdp.ifindex != IFINDEX_LO,
			"link_ifindex",
			"got %u != exp %u\n",
			link_info.xdp.ifindex,
			IFINDEX_LO
		);

		err = bpf_link__detach(link);
		CHECK!(err != 0, "link_detach", "failed %d\n", err);
	}
	test_check_mtu__destroy(skel);
}

unsafe fn test_check_mtu_run_xdp(
	skel: *mut test_check_mtu,
	prog: *mut bpf_program,
	mtu_expect: u32,
) {
	let retval_expect: c_int = XDP_PASS;
	let mut mtu_result: u32 = 0;
	let mut buf: [c_char; 256] = [0; 256];
	let err: c_int;
	let prog_fd: c_int = bpf_program__fd(prog);
	let mut topts = bpf_test_run_opts {
		repeat: 1,
		data_in: &pkt_v4 as *const c_void,
		data_size_in: size_of_val_raw!(&pkt_v4) as u32,
		data_out: buf.as_mut_ptr() as *mut c_void,
		data_size_out: size_of::<[c_char; 256]>() as u32,
		ctx_in: ptr::null(),
		ctx_size_in: 0,
		retval: 0,
	};

	err = bpf_prog_test_run_opts(prog_fd, &mut topts);
	ASSERT_OK!(err, "test_run");
	ASSERT_EQ!(topts.retval, retval_expect as u32, "retval");

	/* Extract MTU that BPF-prog got */
	mtu_result = (*(*skel).bss).global_bpf_mtu_xdp;
	ASSERT_EQ!(mtu_result, mtu_expect, "MTU-compare-user");
}

unsafe fn test_check_mtu_xdp(mtu: u32, ifindex: u32) {
	let skel: *mut test_check_mtu;
	let mut err: c_int;

	skel = test_check_mtu__open();
	if CHECK!(skel.is_null(), "skel_open", "failed") {
		return;
	}

	/* Update "constants" in BPF-prog *BEFORE* libbpf load */
	(*(*skel).rodata).GLOBAL_USER_MTU = mtu;
	(*(*skel).rodata).GLOBAL_USER_IFINDEX = ifindex;

	'cleanup: {
		err = test_check_mtu__load(skel);
		if CHECK!(err != 0, "skel_load", "failed: %d\n", err) {
			break 'cleanup;
		}

		test_check_mtu_run_xdp(skel, (*skel).progs.xdp_use_helper, mtu);
		test_check_mtu_run_xdp(skel, (*skel).progs.xdp_exceed_mtu, mtu);
		test_check_mtu_run_xdp(skel, (*skel).progs.xdp_minus_delta, mtu);
		test_check_mtu_run_xdp(skel, (*skel).progs.xdp_input_len, mtu);
		test_check_mtu_run_xdp(skel, (*skel).progs.xdp_input_len_exceed, mtu);
	}
	test_check_mtu__destroy(skel);
}

unsafe fn test_check_mtu_run_tc(
	skel: *mut test_check_mtu,
	prog: *mut bpf_program,
	mtu_expect: u32,
) {
	let retval_expect: c_int = BPF_OK;
	let mut mtu_result: u32 = 0;
	let mut buf: [c_char; 256] = [0; 256];
	let err: c_int;
	let prog_fd: c_int = bpf_program__fd(prog);
	let mut topts = bpf_test_run_opts {
		data_in: &pkt_v4 as *const c_void,
		data_size_in: size_of_val_raw!(&pkt_v4) as u32,
		data_out: buf.as_mut_ptr() as *mut c_void,
		data_size_out: size_of::<[c_char; 256]>() as u32,
		repeat: 1,
		ctx_in: ptr::null(),
		ctx_size_in: 0,
		retval: 0,
	};

	err = bpf_prog_test_run_opts(prog_fd, &mut topts);
	ASSERT_OK!(err, "test_run");
	ASSERT_EQ!(topts.retval, retval_expect as u32, "retval");

	/* Extract MTU that BPF-prog got */
	mtu_result = (*(*skel).bss).global_bpf_mtu_tc;
	ASSERT_EQ!(mtu_result, mtu_expect, "MTU-compare-user");
}

unsafe fn test_chk_segs_flag(skel: *mut test_check_mtu, mtu: u32) {
	let err: c_int;
	let prog_fd: c_int = bpf_program__fd((*skel).progs.tc_chk_segs_flag);
	let mut skb = __sk_buff {
		gso_size: 10,
	};
	let mut topts = bpf_test_run_opts {
		data_in: &pkt_v4 as *const c_void,
		data_size_in: size_of_val_raw!(&pkt_v4) as u32,
		ctx_in: &mut skb as *mut __sk_buff as *const c_void,
		ctx_size_in: size_of::<__sk_buff>() as u32,
		repeat: 0,
		data_out: ptr::null_mut(),
		data_size_out: 0,
		retval: 0,
	};

	/* Lower the mtu to test the BPF_MTU_CHK_SEGS */
	SYS_NOFAIL!("ip link set dev lo mtu 10");
	err = bpf_prog_test_run_opts(prog_fd, &mut topts);
	SYS_NOFAIL!("ip link set dev lo mtu %u", mtu);
	ASSERT_OK!(err, "test_run");
	ASSERT_EQ!(topts.retval, BPF_OK as u32, "retval");
}

unsafe fn test_check_mtu_tc(mtu: u32, ifindex: u32) {
	let skel: *mut test_check_mtu;
	let mut err: c_int;

	skel = test_check_mtu__open();
	if CHECK!(skel.is_null(), "skel_open", "failed") {
		return;
	}

	/* Update "constants" in BPF-prog *BEFORE* libbpf load */
	(*(*skel).rodata).GLOBAL_USER_MTU = mtu;
	(*(*skel).rodata).GLOBAL_USER_IFINDEX = ifindex;

	'cleanup: {
		err = test_check_mtu__load(skel);
		if CHECK!(err != 0, "skel_load", "failed: %d\n", err) {
			break 'cleanup;
		}

		test_check_mtu_run_tc(skel, (*skel).progs.tc_use_helper, mtu);
		test_check_mtu_run_tc(skel, (*skel).progs.tc_exceed_mtu, mtu);
		test_check_mtu_run_tc(skel, (*skel).progs.tc_exceed_mtu_da, mtu);
		test_check_mtu_run_tc(skel, (*skel).progs.tc_minus_delta, mtu);
		test_check_mtu_run_tc(skel, (*skel).progs.tc_input_len, mtu);
		test_check_mtu_run_tc(skel, (*skel).progs.tc_input_len_exceed, mtu);
		test_chk_segs_flag(skel, mtu);
	}
	test_check_mtu__destroy(skel);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_ns_check_mtu() {
	let mtu_lo: c_int;

	if test__start_subtest(c"bpf_check_mtu XDP-attach".as_ptr()) {
		test_check_mtu_xdp_attach();
	}

	mtu_lo = read_mtu_device_lo();
	if CHECK!(mtu_lo < 0, "reading MTU value", "failed (err:%d)", mtu_lo) {
		return;
	}

	if test__start_subtest(c"bpf_check_mtu XDP-run".as_ptr()) {
		test_check_mtu_xdp(mtu_lo as u32, 0);
	}

	if test__start_subtest(c"bpf_check_mtu XDP-run ifindex-lookup".as_ptr()) {
		test_check_mtu_xdp(mtu_lo as u32, IFINDEX_LO);
	}

	if test__start_subtest(c"bpf_check_mtu TC-run".as_ptr()) {
		test_check_mtu_tc(mtu_lo as u32, 0);
	}

	if test__start_subtest(c"bpf_check_mtu TC-run ifindex-lookup".as_ptr()) {
		test_check_mtu_tc(mtu_lo as u32, IFINDEX_LO);
	}
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
