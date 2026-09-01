// SPDX-License-Identifier: GPL-2.0

// Translated from testing/selftests/bpf/prog_tests/cgroup_storage.c.
// External declarations correspond to symbols provided by the original C
// headers: test_progs.h, cgroup_helpers.h, network_helpers.h, and
// cgroup_storage.skel.h.

use core::ffi::{c_char, c_int, c_ulonglong, c_void};

const TEST_CGROUP: &[u8] = b"/test-bpf-cgroup-storage-buf/\0";
const TEST_NS: &[u8] = b"cgroup_storage_ns\0";
const PING_CMD: &[u8] = b"ping localhost -c 1 -W 1 -q\0";

const AF_INET: c_int = 2;
const SOCK_DGRAM: c_int = 2;
const ENOENT: c_int = 2;

#[repr(C)]
pub struct nstoken {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bpf_cgroup_storage_key {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
	_private: [u8; 0],
}

#[repr(C)]
pub struct cgroup_storage_links {
	pub bpf_prog: *mut bpf_link,
	pub trigger_oob: *mut bpf_link,
}

#[repr(C)]
pub struct cgroup_storage_progs {
	pub bpf_prog: *mut bpf_program,
	pub trigger_oob: *mut bpf_program,
}

#[repr(C)]
pub struct cgroup_storage_maps {
	pub cgroup_storage: *mut bpf_map,
}

#[repr(C)]
pub struct cgroup_storage {
	pub links: cgroup_storage_links,
	pub progs: cgroup_storage_progs,
	pub maps: cgroup_storage_maps,
}

unsafe extern "C" {
	static mut errno: c_int;

	fn open_netns(name: *const c_char) -> *mut nstoken;
	fn close_netns(ns: *mut nstoken);

	fn cgroup_setup_and_join(path: *const c_char) -> c_int;
	fn cleanup_cgroup_environment();

	fn cgroup_storage__open_and_load() -> *mut cgroup_storage;
	fn cgroup_storage__destroy(obj: *mut cgroup_storage);

	fn bpf_program__attach_cgroup(prog: *mut bpf_program, cgroup_fd: c_int) -> *mut bpf_link;
	fn bpf_map__get_next_key(
		map: *mut bpf_map,
		key: *const c_void,
		next_key: *mut c_void,
		key_sz: usize,
	) -> c_int;
	fn bpf_map__lookup_elem(
		map: *mut bpf_map,
		key: *const c_void,
		key_sz: usize,
		value: *mut c_void,
		value_sz: usize,
		flags: u64,
	) -> c_int;
	fn bpf_map__update_elem(
		map: *mut bpf_map,
		key: *const c_void,
		key_sz: usize,
		value: *const c_void,
		value_sz: usize,
		flags: u64,
	) -> c_int;

	fn close(fd: c_int) -> c_int;
	fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;

	fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
	fn ASSERT_OK_FD(fd: c_int, name: *const c_char) -> bool;
	fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
	fn ASSERT_NEQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
	fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
	fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;

	fn SYS_NOFAIL(cmd: *const c_char, ...) -> c_int;
	fn SYS(label: *const c_char, cmd: *const c_char, ...) -> c_int;
}

unsafe fn setup_network(token: *mut *mut nstoken) -> c_int {
	if SYS(
		c"fail".as_ptr(),
		c"ip netns add %s".as_ptr(),
		TEST_NS.as_ptr() as *const c_char,
	) != 0
	{
		return -1;
	}
	*token = open_netns(TEST_NS.as_ptr() as *const c_char);
	if !ASSERT_OK_PTR(*token as *const c_void, c"open netns".as_ptr()) {
		SYS_NOFAIL(
			c"ip netns del %s".as_ptr(),
			TEST_NS.as_ptr() as *const c_char,
		);
		return -1;
	}
	if SYS(c"cleanup_ns".as_ptr(), c"ip link set lo up".as_ptr()) != 0 {
		SYS_NOFAIL(
			c"ip netns del %s".as_ptr(),
			TEST_NS.as_ptr() as *const c_char,
		);
		return -1;
	}

	0
}

unsafe fn cleanup_network(ns: *mut nstoken) {
	close_netns(ns);
	SYS_NOFAIL(
		c"ip netns del %s".as_ptr(),
		TEST_NS.as_ptr() as *const c_char,
	);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_cgroup_storage() {
	let mut key = core::mem::MaybeUninit::<bpf_cgroup_storage_key>::uninit();
	let mut skel: *mut cgroup_storage;
	let mut ns: *mut nstoken = core::ptr::null_mut();
	let mut value: c_ulonglong;
	let cgroup_fd: c_int;
	let mut err: c_int;

	cgroup_fd = cgroup_setup_and_join(TEST_CGROUP.as_ptr() as *const c_char);
	if !ASSERT_OK_FD(cgroup_fd, c"create cgroup".as_ptr()) {
		return;
	}

	if !ASSERT_OK(setup_network(&mut ns), c"setup network".as_ptr()) {
		goto_cleanup_cgroup(cgroup_fd);
		return;
	}

	skel = cgroup_storage__open_and_load();
	if !ASSERT_OK_PTR(skel as *const c_void, c"load program".as_ptr()) {
		goto_cleanup_network(ns, cgroup_fd);
		return;
	}

	(*skel).links.bpf_prog =
		bpf_program__attach_cgroup((*skel).progs.bpf_prog, cgroup_fd);
	if !ASSERT_OK_PTR((*skel).links.bpf_prog as *const c_void, c"attach program".as_ptr()) {
		goto_cleanup_progs(skel, ns, cgroup_fd);
		return;
	}

	/* Check that one out of every two packets is dropped */
	err = SYS_NOFAIL(PING_CMD.as_ptr() as *const c_char);
	ASSERT_OK(err, c"first ping".as_ptr());
	err = SYS_NOFAIL(PING_CMD.as_ptr() as *const c_char);
	ASSERT_NEQ(err, 0, c"second ping".as_ptr());
	err = SYS_NOFAIL(PING_CMD.as_ptr() as *const c_char);
	ASSERT_OK(err, c"third ping".as_ptr());

	err = bpf_map__get_next_key(
		(*skel).maps.cgroup_storage,
		core::ptr::null(),
		key.as_mut_ptr() as *mut c_void,
		core::mem::size_of::<bpf_cgroup_storage_key>(),
	);
	if !ASSERT_OK(err, c"get first key".as_ptr()) {
		goto_cleanup_progs(skel, ns, cgroup_fd);
		return;
	}
	value = core::mem::MaybeUninit::<c_ulonglong>::uninit().assume_init();
	err = bpf_map__lookup_elem(
		(*skel).maps.cgroup_storage,
		key.as_ptr() as *const c_void,
		core::mem::size_of::<bpf_cgroup_storage_key>(),
		&mut value as *mut _ as *mut c_void,
		core::mem::size_of::<c_ulonglong>(),
		0,
	);
	if !ASSERT_OK(err, c"first packet count read".as_ptr()) {
		goto_cleanup_progs(skel, ns, cgroup_fd);
		return;
	}

	/* Add one to the packet counter, check again packet filtering */
	value = value.wrapping_add(1);
	err = bpf_map__update_elem(
		(*skel).maps.cgroup_storage,
		key.as_ptr() as *const c_void,
		core::mem::size_of::<bpf_cgroup_storage_key>(),
		&value as *const _ as *const c_void,
		core::mem::size_of::<c_ulonglong>(),
		0,
	);
	if !ASSERT_OK(err, c"increment packet counter".as_ptr()) {
		goto_cleanup_progs(skel, ns, cgroup_fd);
		return;
	}
	err = SYS_NOFAIL(PING_CMD.as_ptr() as *const c_char);
	ASSERT_OK(err, c"fourth ping".as_ptr());
	err = SYS_NOFAIL(PING_CMD.as_ptr() as *const c_char);
	ASSERT_NEQ(err, 0, c"fifth ping".as_ptr());
	err = SYS_NOFAIL(PING_CMD.as_ptr() as *const c_char);
	ASSERT_OK(err, c"sixth ping".as_ptr());

	err = bpf_map__get_next_key(
		(*skel).maps.cgroup_storage,
		key.as_ptr() as *const c_void,
		key.as_mut_ptr() as *mut c_void,
		core::mem::size_of::<bpf_cgroup_storage_key>(),
	);
	ASSERT_ERR(err, c"bpf_map__get_next_key should fail".as_ptr());
	ASSERT_EQ(errno, ENOENT, c"no second key".as_ptr());

	goto_cleanup_progs(skel, ns, cgroup_fd);
}

unsafe fn goto_cleanup_progs(skel: *mut cgroup_storage, ns: *mut nstoken, cgroup_fd: c_int) {
	cgroup_storage__destroy(skel);
	goto_cleanup_network(ns, cgroup_fd);
}

unsafe fn goto_cleanup_network(ns: *mut nstoken, cgroup_fd: c_int) {
	cleanup_network(ns);
	goto_cleanup_cgroup(cgroup_fd);
}

unsafe fn goto_cleanup_cgroup(cgroup_fd: c_int) {
	close(cgroup_fd);
	cleanup_cgroup_environment();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_cgroup_storage_oob() {
	let mut skel: *mut cgroup_storage;
	let cgroup_fd: c_int;
	let sock_fd: c_int;

	cgroup_fd = cgroup_setup_and_join(TEST_CGROUP.as_ptr() as *const c_char);
	if !ASSERT_OK_FD(cgroup_fd, c"create cgroup".as_ptr()) {
		return;
	}

	/* Load and attach BPF program */
	skel = cgroup_storage__open_and_load();
	if !ASSERT_OK_PTR(skel as *const c_void, c"cgroup_storage__open_and_load".as_ptr()) {
		goto_cleanup_cgroup(cgroup_fd);
		return;
	}

	(*skel).links.trigger_oob =
		bpf_program__attach_cgroup((*skel).progs.trigger_oob, cgroup_fd);
	if !ASSERT_OK_PTR((*skel).links.trigger_oob as *const c_void, c"attach_cgroup".as_ptr()) {
		goto_cleanup_skel(skel, cgroup_fd);
		return;
	}

	/* Create a socket to trigger cgroup/sock_create hook.
	 * This will execute our BPF program and trigger the OOB read
	 * if the bug is present (before the fix).
	 */
	sock_fd = socket(AF_INET, SOCK_DGRAM, 0);
	if !ASSERT_OK_FD(sock_fd, c"create socket".as_ptr()) {
		goto_cleanup_skel(skel, cgroup_fd);
		return;
	}

	close(sock_fd);

	/* If we reach here without a kernel panic or KASAN report,
	 * the test passes (the fix is working).
	 */

	goto_cleanup_skel(skel, cgroup_fd);
}

unsafe fn goto_cleanup_skel(skel: *mut cgroup_storage, cgroup_fd: c_int) {
	cgroup_storage__destroy(skel);
	goto_cleanup_cgroup(cgroup_fd);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
