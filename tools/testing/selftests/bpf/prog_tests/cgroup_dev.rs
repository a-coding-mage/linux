// SPDX-License-Identifier: GPL-2.0

// C dependencies translated from:
// <sys/stat.h>, <sys/sysmacros.h>, <errno.h>,
// "test_progs.h", "cgroup_helpers.h", "dev_cgroup.skel.h"

use libc::{c_char, c_int, c_void, dev_t, mode_t, size_t, ssize_t};

const TEST_CGROUP: &[u8] = b"/test-bpf-based-device-cgroup/\0";
const TEST_BUFFER_SIZE: usize = 64;

#[repr(C)]
pub struct bpf_program {
	_private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
	_private: [u8; 0],
}

#[repr(C)]
pub struct dev_cgroup {
	pub progs: dev_cgroup_progs,
	pub links: dev_cgroup_links,
}

#[repr(C)]
pub struct dev_cgroup_progs {
	pub bpf_prog1: *mut bpf_program,
}

#[repr(C)]
pub struct dev_cgroup_links {
	pub bpf_prog1: *mut bpf_link,
}

extern "C" {
	fn unlink(path: *const c_char) -> c_int;
	fn mknod(path: *const c_char, mode: mode_t, dev: dev_t) -> c_int;
	fn open(path: *const c_char, flags: c_int, ...) -> c_int;
	fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
	fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
	fn close(fd: c_int) -> c_int;
	fn __errno_location() -> *mut c_int;

	fn cgroup_setup_and_join(path: *const c_char) -> c_int;
	fn cleanup_cgroup_environment();
	fn dev_cgroup__open_and_load() -> *mut dev_cgroup;
	fn dev_cgroup__destroy(obj: *mut dev_cgroup);
	fn bpf_program__attach_cgroup(prog: *mut bpf_program, cgroup_fd: c_int) -> *mut bpf_link;
	fn test__start_subtest(name: *const c_char) -> bool;
	fn ASSERT_OK_FD(fd: c_int, name: *const c_char) -> bool;
	fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
}

unsafe fn errno_value() -> c_int {
	*__errno_location()
}

unsafe fn makedev(dev_major: c_int, dev_minor: c_int) -> dev_t {
	libc::makedev(dev_major as libc::c_uint, dev_minor as libc::c_uint) as dev_t
}

unsafe fn test_mknod(
	path: *const c_char,
	mode: mode_t,
	dev_major: c_int,
	dev_minor: c_int,
	expected_ret: c_int,
	expected_errno: c_int,
) {
	let ret: c_int;

	unlink(path);
	ret = mknod(path, mode, makedev(dev_major, dev_minor));
	ASSERT_EQ!(ret, expected_ret, "mknod");
	if expected_ret != 0 {
		ASSERT_EQ!(errno_value(), expected_errno, "mknod errno");
	} else {
		unlink(path);
	}
}

unsafe fn test_read(
	path: *const c_char,
	buf: *mut c_char,
	buf_size: c_int,
	expected_ret: c_int,
	expected_errno: c_int,
) {
	let ret: c_int;
	let fd: c_int;

	fd = open(path, libc::O_RDONLY);

	/* A bare open on unauthorized device should fail */
	if expected_ret < 0 {
		ASSERT_EQ!(fd, expected_ret, "open ret for read");
		ASSERT_EQ!(errno_value(), expected_errno, "open errno for read");
		if fd >= 0 {
			close(fd);
		}
		return;
	}

	if !ASSERT_OK_FD(fd, b"open ret for read\0".as_ptr() as *const c_char) {
		return;
	}

	ret = read(fd, buf as *mut c_void, buf_size as size_t) as c_int;
	ASSERT_EQ!(ret, expected_ret, "read");

	close(fd);
}

unsafe fn test_write(
	path: *const c_char,
	buf: *mut c_char,
	buf_size: c_int,
	expected_ret: c_int,
	expected_errno: c_int,
) {
	let ret: c_int;
	let fd: c_int;

	fd = open(path, libc::O_WRONLY);

	/* A bare open on unauthorized device should fail */
	if expected_ret < 0 {
		ASSERT_EQ!(fd, expected_ret, "open ret for write");
		ASSERT_EQ!(errno_value(), expected_errno, "open errno for write");
		if fd >= 0 {
			close(fd);
		}
		return;
	}

	if !ASSERT_OK_FD(fd, b"open ret for write\0".as_ptr() as *const c_char) {
		return;
	}

	ret = write(fd, buf as *const c_void, buf_size as size_t) as c_int;
	ASSERT_EQ!(ret, expected_ret, "write");

	close(fd);
}

#[no_mangle]
pub unsafe extern "C" fn test_cgroup_dev() {
	let mut buf: [c_char; TEST_BUFFER_SIZE] = [0; TEST_BUFFER_SIZE];
	let init = b"some random test data\0";
	let mut i = 0usize;
	while i < init.len() {
		buf[i] = init[i] as c_char;
		i += 1;
	}

	let skel: *mut dev_cgroup;
	let cgroup_fd: c_int;

	cgroup_fd = cgroup_setup_and_join(TEST_CGROUP.as_ptr() as *const c_char);
	if !ASSERT_OK_FD(cgroup_fd, b"cgroup switch\0".as_ptr() as *const c_char) {
		return;
	}

	skel = dev_cgroup__open_and_load();
	if !ASSERT_OK_PTR(skel as *const c_void, b"load program\0".as_ptr() as *const c_char) {
		cleanup_cgroup_environment();
		return;
	}

	(*skel).links.bpf_prog1 = bpf_program__attach_cgroup((*skel).progs.bpf_prog1, cgroup_fd);
	if !ASSERT_OK_PTR(
		(*skel).links.bpf_prog1 as *const c_void,
		b"attach_program\0".as_ptr() as *const c_char,
	) {
		dev_cgroup__destroy(skel);
		cleanup_cgroup_environment();
		return;
	}

	if test__start_subtest(b"allow-mknod\0".as_ptr() as *const c_char) {
		test_mknod(
			b"/dev/test_dev_cgroup_null\0".as_ptr() as *const c_char,
			libc::S_IFCHR as mode_t,
			1,
			3,
			0,
			0,
		);
	}

	if test__start_subtest(b"allow-read\0".as_ptr() as *const c_char) {
		test_read(
			b"/dev/urandom\0".as_ptr() as *const c_char,
			buf.as_mut_ptr(),
			TEST_BUFFER_SIZE as c_int,
			TEST_BUFFER_SIZE as c_int,
			0,
		);
	}

	if test__start_subtest(b"allow-write\0".as_ptr() as *const c_char) {
		test_write(
			b"/dev/null\0".as_ptr() as *const c_char,
			buf.as_mut_ptr(),
			TEST_BUFFER_SIZE as c_int,
			TEST_BUFFER_SIZE as c_int,
			0,
		);
	}

	if test__start_subtest(b"deny-mknod\0".as_ptr() as *const c_char) {
		test_mknod(
			b"/dev/test_dev_cgroup_zero\0".as_ptr() as *const c_char,
			libc::S_IFCHR as mode_t,
			1,
			5,
			-1,
			libc::EPERM,
		);
	}

	if test__start_subtest(b"deny-read\0".as_ptr() as *const c_char) {
		test_read(
			b"/dev/random\0".as_ptr() as *const c_char,
			buf.as_mut_ptr(),
			TEST_BUFFER_SIZE as c_int,
			-1,
			libc::EPERM,
		);
	}

	if test__start_subtest(b"deny-write\0".as_ptr() as *const c_char) {
		test_write(
			b"/dev/zero\0".as_ptr() as *const c_char,
			buf.as_mut_ptr(),
			TEST_BUFFER_SIZE as c_int,
			-1,
			libc::EPERM,
		);
	}

	if test__start_subtest(b"deny-mknod-wrong-type\0".as_ptr() as *const c_char) {
		test_mknod(
			b"/dev/test_dev_cgroup_block\0".as_ptr() as *const c_char,
			libc::S_IFBLK as mode_t,
			1,
			3,
			-1,
			libc::EPERM,
		);
	}

	dev_cgroup__destroy(skel);
	cleanup_cgroup_environment();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
