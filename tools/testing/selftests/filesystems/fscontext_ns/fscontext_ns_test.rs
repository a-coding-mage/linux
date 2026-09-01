// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2026 Christian Brauner <brauner@kernel.org>
 *
 * Test that completing a filesystem context from another user namespace
 * doesn't warn.
 *
 * fsopen() records the caller's user namespace in fc->user_ns and hands
 * back an ordinary file descriptor. The task that issues
 * FSCONFIG_CMD_CREATE need not be the one that created the context: the fd
 * is inherited across fork() and exec() and it can be passed over a unix
 * socket. vfs_cmd_create() authorizes the create with mount_capable(),
 * which for FS_USERNS_MOUNT checks ns_capable(fc->user_ns, CAP_SYS_ADMIN),
 * and that succeeds for a task holding CAP_SYS_ADMIN in an ancestor of
 * fc->user_ns.
 *
 * binfmt_misc and overlayfs used to WARN_ON() that mismatch, which let an
 * unprivileged user taint the kernel, flood the log and panic a kernel
 * booted with panic_on_warn. The mount must still be refused, but it must
 * not warn.
 */

// C includes translated as external dependencies:
// errno.h, sched.h, stdio.h, stdlib.h, string.h, sys/socket.h, sys/wait.h,
// unistd.h, ../wrappers.h, ../utils.h, ../../kselftest_harness.h.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type bool_ = bool;
type pid_t = c_int;
type size_t = usize;
type ssize_t = isize;

#[repr(C)]
struct FILE {
	_private: [u8; 0],
}

#[repr(C)]
struct iovec {
	iov_base: *mut c_void,
	iov_len: size_t,
}

#[repr(C)]
struct msghdr {
	msg_name: *mut c_void,
	msg_namelen: c_uint,
	msg_iov: *mut iovec,
	msg_iovlen: size_t,
	msg_control: *mut c_void,
	msg_controllen: size_t,
	msg_flags: c_int,
}

#[repr(C)]
struct cmsghdr {
	cmsg_len: size_t,
	cmsg_level: c_int,
	cmsg_type: c_int,
}

unsafe extern "C" {
	static mut errno: c_int;

	fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
	fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
	fn fclose(stream: *mut FILE) -> c_int;
	fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
	fn socketpair(domain: c_int, type_: c_int, protocol: c_int, sv: *mut c_int) -> c_int;
	fn fork() -> pid_t;
	fn close(fd: c_int) -> c_int;
	fn unshare(flags: c_int) -> c_int;
	fn _exit(status: c_int) -> !;
	fn sendmsg(sockfd: c_int, msg: *const msghdr, flags: c_int) -> ssize_t;
	fn recvmsg(sockfd: c_int, msg: *mut msghdr, flags: c_int) -> ssize_t;
	fn waitpid(pid: pid_t, wstatus: *mut c_int, options: c_int) -> pid_t;

	fn sys_fsopen(fsname: *const c_char, flags: c_uint) -> c_int;
	fn sys_fsconfig(
		fd: c_int,
		cmd: c_uint,
		key: *const c_char,
		value: *const c_char,
		aux: c_int,
	) -> c_int;
	fn setup_userns() -> c_int;
	fn wait_for_pid(pid: pid_t) -> c_int;
}

const FSCONFIG_CMD_CREATE: c_uint = 6;

/* TAINT_WARN, i.e. bit 9 of /proc/sys/kernel/tainted. */
const TAINT_WARN_BIT: c_ulong = 9;

const AF_UNIX: c_int = 1;
const SOCK_STREAM: c_int = 1;
const SOL_SOCKET: c_int = 1;
const SCM_RIGHTS: c_int = 1;
const CLONE_NEWNS: c_int = 0x00020000;
const CLONE_NEWUSER: c_int = 0x10000000;
const WNOHANG: c_int = 1;
const ENODATA: c_int = 61;
const EINVAL: c_int = 22;
const EIO: c_int = 5;

const fn cmsg_align(len: usize) -> usize {
	(len + size_of::<usize>() - 1) & !(size_of::<usize>() - 1)
}

const fn cmsg_space(len: usize) -> usize {
	cmsg_align(size_of::<cmsghdr>()) + cmsg_align(len)
}

const fn cmsg_len(len: usize) -> usize {
	cmsg_align(size_of::<cmsghdr>()) + len
}

unsafe fn cmsg_firsthdr(mhdr: *const msghdr) -> *mut cmsghdr {
	if (*mhdr).msg_controllen >= size_of::<cmsghdr>() {
		(*mhdr).msg_control as *mut cmsghdr
	} else {
		ptr::null_mut()
	}
}

unsafe fn cmsg_data(cmsg: *mut cmsghdr) -> *mut c_uchar {
	(cmsg as *mut c_uchar).add(cmsg_align(size_of::<cmsghdr>()))
}

type c_uchar = u8;

unsafe fn taint_warn_set() -> bool_ {
	let mut taint: c_ulong = 0;
	let f: *mut FILE;

	f = fopen(
		b"/proc/sys/kernel/tainted\0".as_ptr() as *const c_char,
		b"r\0".as_ptr() as *const c_char,
	);
	if f.is_null() {
		return false;
	}
	if fscanf(f, b"%lu\0".as_ptr() as *const c_char, &mut taint) != 1 {
		taint = 0;
	}
	fclose(f);

	return (taint & (1_u64 << TAINT_WARN_BIT) as c_ulong) != 0;
}

unsafe fn send_fd(sock: c_int, fd: c_int) -> c_int {
	let mut cmsgbuf: [c_uchar; cmsg_space(size_of::<c_int>())] =
		[0; cmsg_space(size_of::<c_int>())];
	let mut b: [c_char; 1] = [b'x' as c_char];
	let mut iov = iovec {
		iov_base: b.as_mut_ptr() as *mut c_void,
		iov_len: size_of_val(&b),
	};
	let mut msg: msghdr = zeroed();
	msg.msg_iov = &mut iov;
	msg.msg_iovlen = 1;
	msg.msg_control = cmsgbuf.as_mut_ptr() as *mut c_void;
	msg.msg_controllen = size_of_val(&cmsgbuf);
	let cmsg: *mut cmsghdr;

	cmsg = cmsg_firsthdr(&msg);
	(*cmsg).cmsg_level = SOL_SOCKET;
	(*cmsg).cmsg_type = SCM_RIGHTS;
	(*cmsg).cmsg_len = cmsg_len(size_of::<c_int>());
	memcpy(
		cmsg_data(cmsg) as *mut c_void,
		&fd as *const c_int as *const c_void,
		size_of::<c_int>(),
	);

	return if sendmsg(sock, &msg, 0) < 0 { -1 } else { 0 };
}

unsafe fn recv_fd(sock: c_int) -> c_int {
	let mut cmsgbuf: [c_uchar; cmsg_space(size_of::<c_int>())] =
		[0; cmsg_space(size_of::<c_int>())];
	let mut b: [c_char; 1] = [0; 1];
	let mut iov = iovec {
		iov_base: b.as_mut_ptr() as *mut c_void,
		iov_len: size_of_val(&b),
	};
	let mut msg: msghdr = zeroed();
	msg.msg_iov = &mut iov;
	msg.msg_iovlen = 1;
	msg.msg_control = cmsgbuf.as_mut_ptr() as *mut c_void;
	msg.msg_controllen = size_of_val(&cmsgbuf);
	let cmsg: *mut cmsghdr;
	let mut fd: c_int = -1;

	if recvmsg(sock, &mut msg, 0) <= 0 {
		return -1;
	}

	cmsg = cmsg_firsthdr(&msg);
	if cmsg.is_null() || (*cmsg).cmsg_type != SCM_RIGHTS {
		return -1;
	}
	memcpy(
		&mut fd as *mut c_int as *mut c_void,
		cmsg_data(cmsg) as *const c_void,
		size_of::<c_int>(),
	);

	return fd;
}

/*
 * Create a context for @fsname in a child and complete it here. With @nest
 * the child first creates its own user namespace, so that the context is
 * created in a descendant of the namespace completing it. The child needs a
 * mount namespace of its own as well: fsopen() gates on may_mount(), which
 * asks for CAP_SYS_ADMIN in the user namespace owning the caller's mount
 * namespace.
 *
 * Returns the result of FSCONFIG_CMD_CREATE with errno set, or -ENODATA if
 * the child could not create the context at all.
 */
unsafe fn create_from_child(fsname: *const c_char, nest: bool_) -> c_int {
	let mut sock: [c_int; 2] = [0; 2];
	let mut fd: c_int;
	let ret: c_int;
	let mut status: c_int = 0;
	let pid: pid_t;

	if socketpair(AF_UNIX, SOCK_STREAM, 0, sock.as_mut_ptr()) != 0 {
		return -ENODATA;
	}

	pid = fork();
	if pid < 0 {
		close(sock[0]);
		close(sock[1]);
		return -ENODATA;
	}

	if pid == 0 {
		close(sock[0]);

		if nest && unshare(CLONE_NEWUSER | CLONE_NEWNS) != 0 {
			_exit(1);
		}

		fd = sys_fsopen(fsname, 0);
		if fd < 0 {
			_exit(1);
		}
		if send_fd(sock[1], fd) != 0 {
			_exit(1);
		}
		_exit(0);
	}

	close(sock[1]);
	fd = recv_fd(sock[0]);
	close(sock[0]);
	wait_for_pid(pid);
	waitpid(pid, &mut status, WNOHANG);

	if fd < 0 {
		return -ENODATA;
	}

	errno = 0;
	ret = sys_fsconfig(fd, FSCONFIG_CMD_CREATE, ptr::null(), ptr::null(), 0);
	status = errno;
	close(fd);
	errno = status;

	return ret;
}

#[repr(C)]
struct fscontext_ns {
	warn_before: bool_,
}

// FIXTURE_SETUP(fscontext_ns)
unsafe fn fscontext_ns_setup(self_: *mut fscontext_ns) {
	(*self_).warn_before = taint_warn_set();

	if setup_userns() != 0 {
		// SKIP(return, "setup_userns failed");
		return;
	}
}

// FIXTURE_TEARDOWN(fscontext_ns)
unsafe fn fscontext_ns_teardown(_self: *mut fscontext_ns) {}

/*
 * The condition the kernel used to WARN about. It has to be refused, and it
 * has to be refused quietly: an unprivileged task reaches this.
 */
#[repr(C)]
struct fscontext_ns_variant {
	fsname: *const c_char,
	expected_errno: c_int,
}

// FIXTURE_VARIANT_ADD(fscontext_ns, binfmt_misc)
static binfmt_misc: fscontext_ns_variant = fscontext_ns_variant {
	fsname: b"binfmt_misc\0".as_ptr() as *const c_char,
	expected_errno: EINVAL,
};

// FIXTURE_VARIANT_ADD(fscontext_ns, overlay)
static overlay: fscontext_ns_variant = fscontext_ns_variant {
	fsname: b"overlay\0".as_ptr() as *const c_char,
	expected_errno: EIO,
};

// TEST_F(fscontext_ns, create_from_descendant_userns)
unsafe fn create_from_descendant_userns(self_: *mut fscontext_ns, variant: *const fscontext_ns_variant) {
	let ret: c_int;

	ret = create_from_child((*variant).fsname, true);
	if ret == -ENODATA {
		// SKIP(return, "%s unavailable", variant->fsname);
		return;
	}

	// ASSERT_EQ(-1, ret);
	assert_eq!(-1, ret);
	// ASSERT_EQ(variant->expected_errno, errno);
	assert_eq!((*variant).expected_errno, errno);

	/*
	 * Only meaningful if nothing had warned before us. Note that an
	 * unrelated warning racing this test would look like a failure.
	 */
	if (*self_).warn_before {
		// TH_LOG("TAINT_WARN already set, not checking for a new warning");
	} else {
		// ASSERT_FALSE(taint_warn_set());
		assert!(!taint_warn_set());
	}
}

/*
 * The same handover within one user namespace is a supported thing to do and
 * has to keep working. binfmt_misc takes no options, so the create succeeds
 * outright and this also shows the test really drives the create path.
 */
// TEST(create_from_same_userns)
unsafe fn create_from_same_userns() {
	let ret: c_int;

	if setup_userns() != 0 {
		// SKIP(return, "setup_userns failed");
		return;
	}

	ret = create_from_child(b"binfmt_misc\0".as_ptr() as *const c_char, false);
	if ret == -ENODATA {
		// SKIP(return, "binfmt_misc unavailable");
		return;
	}

	// ASSERT_EQ(0, ret);
	assert_eq!(0, ret);
}

// TEST_HARNESS_MAIN


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
