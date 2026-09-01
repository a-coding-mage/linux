// SPDX-License-Identifier: GPL-2.0

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

type size_t = usize;
type ssize_t = isize;
type socklen_t = u32;
type pid_t = c_int;
type __u64 = u64;

/* Includes in the C source provide libc, linux/coredump.h, linux/fs.h,
 * epoll, pidfd, and local filesystem wrapper declarations.
 */

/* Forward declarations to avoid including harness header */
#[repr(C)]
pub struct __test_metadata {
	_private: [u8; 0],
}

/* Match the fixture definition from coredump_test.h */
#[repr(C)]
pub struct _fixture_coredump_data {
	pub original_core_pattern: [c_char; 256],
	pub pid_coredump_server: pid_t,
	pub fd_tmpfs_detached: c_int,
}

const PAGE_SIZE: size_t = 4096;
const NUM_THREAD_SPAWN: c_int = 128;

const AF_UNIX: c_int = 1;
const SOCK_STREAM: c_int = 1;
const SOCK_CLOEXEC: c_int = 0o2000000;
const SOL_SOCKET: c_int = 1;
const SO_PEERPIDFD: c_int = 77;
const MSG_WAITALL: c_int = 0x100;
const MSG_PEEK: c_int = 0x2;
const MSG_NOSIGNAL: c_int = 0x4000;
const O_WRONLY: c_int = 0o1;
const O_RDWR: c_int = 0o2;
const O_EXCL: c_int = 0o200;
const O_CLOEXEC: c_int = 0o2000000;
const O_NONBLOCK: c_int = 0o4000;
const O_TMPFILE: c_int = 0o20000000 | 0o200000;
const F_GETFL: c_int = 3;
const F_SETFL: c_int = 4;
const EXIT_FAILURE: c_int = 1;
const EXIT_SUCCESS: c_int = 0;
const EAGAIN: c_int = 11;
const EWOULDBLOCK: c_int = EAGAIN;
const ENOSPC: c_int = 28;
const FSCONFIG_CMD_CREATE: c_uint = 6;
const EPOLLIN: c_uint = 0x001;
const EPOLLRDHUP: c_uint = 0x2000;
const EPOLLET: c_uint = 1u32 << 31;
const EPOLL_CTL_ADD: c_int = 1;
const PIDFD_GET_INFO: c_ulong = 0xc040_3e0b;
const PIDFD_INFO_EXIT: __u64 = 1 << 0;
const PIDFD_INFO_COREDUMP: __u64 = 1 << 1;
const PIDFD_INFO_COREDUMP_SIGNAL: __u64 = 1 << 2;
const COREDUMP_ACK_SIZE_VER0: size_t = size_of::<coredump_ack>();

#[repr(C)]
pub struct pthread_t {
	_private: c_ulong,
}

#[repr(C)]
pub struct sockaddr {
	pub sa_family: u16,
	pub sa_data: [c_char; 14],
}

#[repr(C)]
pub struct sockaddr_un {
	pub sun_family: u16,
	pub sun_path: [c_char; 108],
}

#[repr(C)]
pub union epoll_data_t {
	pub ptr: *mut c_void,
	pub fd: c_int,
	pub u32_: u32,
	pub u64_: u64,
}

#[repr(C)]
pub struct epoll_event {
	pub events: u32,
	pub data: epoll_data_t,
}

#[repr(C)]
pub struct pidfd_info {
	pub mask: __u64,
	pub coredump_mask: u32,
	pub coredump_signal: i32,
	pub coredump_code: i32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum coredump_mark {
	COREDUMP_MARK_REQACK = 0,
	COREDUMP_MARK_MINSIZE = 1,
	COREDUMP_MARK_MAXSIZE = 2,
	COREDUMP_MARK_UNSUPPORTED = 3,
	COREDUMP_MARK_CONFLICTING = 4,
}

#[repr(C)]
pub struct coredump_req {
	pub size: u32,
	pub size_ack: u32,
	pub mask: __u64,
}

#[repr(C)]
pub struct coredump_ack {
	pub size: size_t,
	pub mask: __u64,
}

unsafe extern "C" {
	static mut errno: c_int;
	static mut stderr: *mut c_void;

	fn pause() -> c_int;
	fn pthread_create(
		thread: *mut pthread_t,
		attr: *const c_void,
		start_routine: extern "C" fn(*mut c_void) -> *mut c_void,
		arg: *mut c_void,
	) -> c_int;
	fn sys_fsopen(fs_name: *const c_char, flags: c_uint) -> c_int;
	fn sys_fsconfig(
		fd: c_int,
		cmd: c_uint,
		key: *const c_char,
		value: *const c_void,
		aux: c_int,
	) -> c_int;
	fn sys_fsmount(fd: c_int, flags: c_uint, attr_flags: c_uint) -> c_int;
	fn close(fd: c_int) -> c_int;
	fn strlen(s: *const c_char) -> size_t;
	fn strncpy(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
	fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
	fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
	fn listen(sockfd: c_int, backlog: c_int) -> c_int;
	fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
	fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
	fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
	fn getsockopt(
		sockfd: c_int,
		level: c_int,
		optname: c_int,
		optval: *mut c_void,
		optlen: *mut socklen_t,
	) -> c_int;
	fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
	fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
	fn recv(sockfd: c_int, buf: *mut c_void, len: size_t, flags: c_int) -> ssize_t;
	fn send(sockfd: c_int, buf: *const c_void, len: size_t, flags: c_int) -> ssize_t;
	fn openat(dirfd: c_int, pathname: *const c_char, flags: c_int, ...) -> c_int;
	fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
	fn epoll_create1(flags: c_int) -> c_int;
	fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut epoll_event) -> c_int;
	fn epoll_wait(epfd: c_int, events: *mut epoll_event, maxevents: c_int, timeout: c_int) -> c_int;
	fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
	fn _exit(status: c_int) -> !;
}

#[unsafe(no_mangle)]
pub extern "C" fn do_nothing(arg: *mut c_void) -> *mut c_void {
	let _ = arg;
	loop {
		unsafe {
			pause();
		}
	}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn crashing_child() {
	let mut thread = pthread_t { _private: 0 };
	let mut i: c_int;

	i = 0;
	while i < NUM_THREAD_SPAWN {
		unsafe {
			pthread_create(&mut thread, ptr::null(), do_nothing, ptr::null_mut());
		}
		i += 1;
	}

	/* crash on purpose */
	unsafe {
		i = ptr::read_volatile(ptr::null::<c_int>());
	}
	let _ = i;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn create_detached_tmpfs() -> c_int {
	let fd_context: c_int;
	let fd_tmpfs: c_int;

	fd_context = unsafe { sys_fsopen(c"tmpfs".as_ptr(), 0) };
	if fd_context < 0 {
		return -1;
	}

	if unsafe { sys_fsconfig(fd_context, FSCONFIG_CMD_CREATE, ptr::null(), ptr::null(), 0) } < 0 {
		return -1;
	}

	fd_tmpfs = unsafe { sys_fsmount(fd_context, 0, 0) };
	unsafe {
		close(fd_context);
	}
	fd_tmpfs
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn create_and_listen_unix_socket(path: *const c_char) -> c_int {
	let mut addr = sockaddr_un {
		sun_family: AF_UNIX as u16,
		sun_path: [0; 108],
	};
	assert!(unsafe { strlen(path) } < size_of_val(&addr.sun_path) - 1);
	unsafe {
		strncpy(addr.sun_path.as_mut_ptr(), path, size_of_val(&addr.sun_path) - 1);
	}
	let addr_len: size_t = offset_of!(sockaddr_un, sun_path) + unsafe { strlen(path) } + 1;
	let mut fd: c_int = -1;
	let mut ret: c_int;

	fd = unsafe { socket(AF_UNIX, SOCK_STREAM | SOCK_CLOEXEC, 0) };
	if fd < 0 {
		goto_out(fd);
		return -1;
	}

	ret = unsafe { bind(fd, &addr as *const sockaddr_un as *const sockaddr, addr_len as socklen_t) };
	if ret < 0 {
		goto_out(fd);
		return -1;
	}

	ret = unsafe { listen(fd, 128) };
	if ret < 0 {
		goto_out(fd);
		return -1;
	}

	return fd;

	unsafe fn goto_out(fd: c_int) {
		if fd >= 0 {
			unsafe {
				close(fd);
			}
		}
	}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_core_pattern(pattern: *const c_char) -> bool {
	let fd: c_int;
	let ret: ssize_t;

	fd = unsafe { open(c"/proc/sys/kernel/core_pattern".as_ptr(), O_WRONLY | O_CLOEXEC) };
	if fd < 0 {
		return false;
	}

	ret = unsafe { write(fd, pattern as *const c_void, strlen(pattern)) };
	unsafe {
		close(fd);
	}
	if ret < 0 {
		return false;
	}

	unsafe {
		fprintf(
			stderr,
			c"Set core_pattern to '%s' | %zu == %zu\n".as_ptr(),
			pattern,
			ret,
			strlen(pattern),
		);
	}
	ret as size_t == unsafe { strlen(pattern) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_peer_pidfd(fd: c_int) -> c_int {
	let mut fd_peer_pidfd: c_int = 0;
	let mut fd_peer_pidfd_len: socklen_t = size_of::<c_int>() as socklen_t;
	let ret = unsafe {
		getsockopt(
			fd,
			SOL_SOCKET,
			SO_PEERPIDFD,
			&mut fd_peer_pidfd as *mut c_int as *mut c_void,
			&mut fd_peer_pidfd_len,
		)
	};
	if ret < 0 {
		unsafe {
			fprintf(stderr, c"get_peer_pidfd: getsockopt(SO_PEERPIDFD) failed: %m\n".as_ptr());
		}
		return -1;
	}
	unsafe {
		fprintf(stderr, c"get_peer_pidfd: successfully retrieved pidfd %d\n".as_ptr(), fd_peer_pidfd);
	}
	fd_peer_pidfd
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_pidfd_info(fd_peer_pidfd: c_int, info: *mut pidfd_info) -> bool {
	let ret: c_int;
	unsafe {
		memset(info as *mut c_void, 0, size_of::<pidfd_info>());
		(*info).mask = PIDFD_INFO_EXIT | PIDFD_INFO_COREDUMP | PIDFD_INFO_COREDUMP_SIGNAL;
	}
	ret = unsafe { ioctl(fd_peer_pidfd, PIDFD_GET_INFO, info) };
	if ret < 0 {
		unsafe {
			fprintf(stderr, c"get_pidfd_info: ioctl(PIDFD_GET_INFO) failed: %m\n".as_ptr());
		}
		return false;
	}
	unsafe {
		fprintf(
			stderr,
			c"get_pidfd_info: mask=0x%llx, coredump_mask=0x%x, coredump_signal=%d, coredump_code=%d\n".as_ptr(),
			(*info).mask as c_ulong,
			(*info).coredump_mask,
			(*info).coredump_signal,
			(*info).coredump_code,
		);
	}
	true
}

/* Protocol helper functions */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn recv_marker(fd: c_int) -> ssize_t {
	let mut mark = coredump_mark::COREDUMP_MARK_REQACK;
	let ret: ssize_t;

	ret = unsafe { recv(fd, &mut mark as *mut coredump_mark as *mut c_void, size_of::<coredump_mark>(), MSG_WAITALL) };
	if ret as size_t != size_of::<coredump_mark>() {
		return -1;
	}

	match mark {
		coredump_mark::COREDUMP_MARK_REQACK => {
			unsafe { fprintf(stderr, c"Received marker: ReqAck\n".as_ptr()) };
			coredump_mark::COREDUMP_MARK_REQACK as ssize_t
		}
		coredump_mark::COREDUMP_MARK_MINSIZE => {
			unsafe { fprintf(stderr, c"Received marker: MinSize\n".as_ptr()) };
			coredump_mark::COREDUMP_MARK_MINSIZE as ssize_t
		}
		coredump_mark::COREDUMP_MARK_MAXSIZE => {
			unsafe { fprintf(stderr, c"Received marker: MaxSize\n".as_ptr()) };
			coredump_mark::COREDUMP_MARK_MAXSIZE as ssize_t
		}
		coredump_mark::COREDUMP_MARK_UNSUPPORTED => {
			unsafe { fprintf(stderr, c"Received marker: Unsupported\n".as_ptr()) };
			coredump_mark::COREDUMP_MARK_UNSUPPORTED as ssize_t
		}
		coredump_mark::COREDUMP_MARK_CONFLICTING => {
			unsafe { fprintf(stderr, c"Received marker: Conflicting\n".as_ptr()) };
			coredump_mark::COREDUMP_MARK_CONFLICTING as ssize_t
		}
		_ => {
			unsafe {
				fprintf(stderr, c"Received unknown marker: %u\n".as_ptr(), mark as c_uint);
			}
			-1
		}
	}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_marker(fd: c_int, mark: coredump_mark) -> bool {
	let ret: ssize_t;

	ret = unsafe { recv_marker(fd) };
	if ret < 0 {
		return false;
	}
	ret == mark as ssize_t
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_coredump_req(fd: c_int, req: *mut coredump_req) -> bool {
	let mut ret: ssize_t;
	let field_size: size_t;
	let user_size: size_t;
	let ack_size: size_t;
	let kernel_size: size_t;
	let remaining_size: size_t;

	unsafe {
		memset(req as *mut c_void, 0, size_of::<coredump_req>());
	}
	field_size = size_of::<u32>();

	/* Peek the size of the coredump request. */
	ret = unsafe { recv(fd, req as *mut c_void, field_size, MSG_PEEK | MSG_WAITALL) };
	if ret as size_t != field_size {
		unsafe {
			fprintf(
				stderr,
				c"read_coredump_req: peek failed (got %zd, expected %zu): %m\n".as_ptr(),
				ret,
				field_size,
			);
		}
		return false;
	}
	kernel_size = unsafe { (*req).size as size_t };

	if kernel_size < COREDUMP_ACK_SIZE_VER0 {
		unsafe {
			fprintf(
				stderr,
				c"read_coredump_req: kernel_size %zu < min %d\n".as_ptr(),
				kernel_size,
				COREDUMP_ACK_SIZE_VER0 as c_int,
			);
		}
		return false;
	}
	if kernel_size >= PAGE_SIZE {
		unsafe {
			fprintf(
				stderr,
				c"read_coredump_req: kernel_size %zu >= PAGE_SIZE %d\n".as_ptr(),
				kernel_size,
				PAGE_SIZE as c_int,
			);
		}
		return false;
	}

	/* Use the minimum of user and kernel size to read the full request. */
	user_size = size_of::<coredump_req>();
	ack_size = if user_size < kernel_size { user_size } else { kernel_size };
	ret = unsafe { recv(fd, req as *mut c_void, ack_size, MSG_WAITALL) };
	if ret as size_t != ack_size {
		return false;
	}

	unsafe {
		fprintf(
			stderr,
			c"Read coredump request with size %u and mask 0x%llx\n".as_ptr(),
			(*req).size,
			(*req).mask as c_ulong,
		);
	}

	if user_size > kernel_size {
		remaining_size = user_size - kernel_size;
	} else {
		remaining_size = kernel_size - user_size;
	}

	if PAGE_SIZE <= remaining_size {
		return false;
	}

	/*
	 * Discard any additional data if the kernel's request was larger than
	 * what we knew about or cared about.
	 */
	if remaining_size != 0 {
		let mut buffer = [0 as c_char; PAGE_SIZE];

		ret = unsafe { recv(fd, buffer.as_mut_ptr() as *mut c_void, size_of_val(&buffer), MSG_WAITALL) };
		if ret as size_t != remaining_size {
			return false;
		}
		unsafe {
			fprintf(stderr, c"Discarded %zu bytes of data after coredump request\n".as_ptr(), remaining_size);
		}
	}

	true
}

#[repr(C)]
struct large_ack_for_size_testing {
	ack: coredump_ack,
	buffer: [c_char; PAGE_SIZE],
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn send_coredump_ack(
	fd: c_int,
	req: *const coredump_req,
	mask: __u64,
	mut size_ack: size_t,
) -> bool {
	let ret: ssize_t;
	/*
	 * Wrap struct coredump_ack in a larger struct so we can
	 * simulate sending to much data to the kernel.
	 */
	let mut large_ack = large_ack_for_size_testing {
		ack: coredump_ack { size: 0, mask: 0 },
		buffer: [0; PAGE_SIZE],
	};

	if size_ack == 0 {
		size_ack = if size_of::<coredump_ack>() < unsafe { (*req).size_ack as size_t } {
			size_of::<coredump_ack>()
		} else {
			unsafe { (*req).size_ack as size_t }
		};
	}
	large_ack.ack.mask = mask;
	large_ack.ack.size = size_ack;
	ret = unsafe { send(fd, &large_ack as *const large_ack_for_size_testing as *const c_void, size_ack, MSG_NOSIGNAL) };
	if ret as size_t != size_ack {
		return false;
	}

	unsafe {
		fprintf(
			stderr,
			c"Sent coredump ack with size %zu and mask 0x%llx\n".as_ptr(),
			size_ack,
			mask as c_ulong,
		);
	}
	true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_coredump_req(
	req: *const coredump_req,
	min_size: size_t,
	required_mask: __u64,
) -> bool {
	if unsafe { (*req).size as size_t } < min_size {
		return false;
	}
	if unsafe { (*req).mask } & required_mask != required_mask {
		return false;
	}
	if unsafe { (*req).mask } & !required_mask != 0 {
		return false;
	}
	true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn open_coredump_tmpfile(fd_tmpfs_detached: c_int) -> c_int {
	unsafe { openat(fd_tmpfs_detached, c".".as_ptr(), O_TMPFILE | O_RDWR | O_EXCL, 0o600) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn process_coredump_worker(
	fd_coredump: c_int,
	fd_peer_pidfd: c_int,
	fd_core_file: c_int,
) {
	let mut epfd: c_int = -1;
	let mut exit_code: c_int = EXIT_FAILURE;
	let mut ev = epoll_event {
		events: 0,
		data: epoll_data_t { fd: 0 },
	};
	let flags: c_int;

	/* Set socket to non-blocking mode for edge-triggered epoll */
	flags = unsafe { fcntl(fd_coredump, F_GETFL, 0) };
	if flags < 0 {
		unsafe {
			fprintf(stderr, c"Worker: fcntl(F_GETFL) failed: %m\n".as_ptr());
		}
		goto_out(epfd, fd_core_file, fd_peer_pidfd, fd_coredump, exit_code);
	}
	if unsafe { fcntl(fd_coredump, F_SETFL, flags | O_NONBLOCK) } < 0 {
		unsafe {
			fprintf(stderr, c"Worker: fcntl(F_SETFL, O_NONBLOCK) failed: %m\n".as_ptr());
		}
		goto_out(epfd, fd_core_file, fd_peer_pidfd, fd_coredump, exit_code);
	}

	epfd = unsafe { epoll_create1(0) };
	if epfd < 0 {
		unsafe {
			fprintf(stderr, c"Worker: epoll_create1() failed: %m\n".as_ptr());
		}
		goto_out(epfd, fd_core_file, fd_peer_pidfd, fd_coredump, exit_code);
	}

	ev.events = EPOLLIN | EPOLLRDHUP | EPOLLET;
	ev.data.fd = fd_coredump;
	if unsafe { epoll_ctl(epfd, EPOLL_CTL_ADD, fd_coredump, &mut ev) } < 0 {
		unsafe {
			fprintf(stderr, c"Worker: epoll_ctl(EPOLL_CTL_ADD) failed: %m\n".as_ptr());
		}
		goto_out(epfd, fd_core_file, fd_peer_pidfd, fd_coredump, exit_code);
	}

	loop {
		let mut events = [epoll_event {
			events: 0,
			data: epoll_data_t { fd: 0 },
		}; 1];
		let n = unsafe { epoll_wait(epfd, events.as_mut_ptr(), 1, -1) };
		if n < 0 {
			unsafe {
				fprintf(stderr, c"Worker: epoll_wait() failed: %m\n".as_ptr());
			}
			break;
		}

		if events[0].events & (EPOLLIN | EPOLLRDHUP) != 0 {
			loop {
				let mut buffer = [0 as c_char; 4096];
				let bytes_read = unsafe { read(fd_coredump, buffer.as_mut_ptr() as *mut c_void, size_of_val(&buffer)) };
				if bytes_read < 0 {
					if unsafe { errno } == EAGAIN || unsafe { errno } == EWOULDBLOCK {
						break;
					}
					unsafe {
						fprintf(stderr, c"Worker: read() failed: %m\n".as_ptr());
					}
					goto_out(epfd, fd_core_file, fd_peer_pidfd, fd_coredump, exit_code);
				}
				if bytes_read == 0 {
					exit_code = EXIT_SUCCESS;
					unsafe {
						fprintf(stderr, c"Worker: completed successfully\n".as_ptr());
					}
					goto_out(epfd, fd_core_file, fd_peer_pidfd, fd_coredump, exit_code);
				}
				let bytes_write = unsafe { write(fd_core_file, buffer.as_ptr() as *const c_void, bytes_read as size_t) };
				if bytes_write != bytes_read {
					if bytes_write < 0 && unsafe { errno } == ENOSPC {
						continue;
					}
					unsafe {
						fprintf(
							stderr,
							c"Worker: write() failed (read=%zd, write=%zd): %m\n".as_ptr(),
							bytes_read,
							bytes_write,
						);
					}
					goto_out(epfd, fd_core_file, fd_peer_pidfd, fd_coredump, exit_code);
				}
			}
		}
	}

	unsafe fn goto_out(
		epfd: c_int,
		fd_core_file: c_int,
		fd_peer_pidfd: c_int,
		fd_coredump: c_int,
		exit_code: c_int,
	) -> ! {
		if epfd >= 0 {
			unsafe {
				close(epfd);
			}
		}
		if fd_core_file >= 0 {
			unsafe {
				close(fd_core_file);
			}
		}
		if fd_peer_pidfd >= 0 {
			unsafe {
				close(fd_peer_pidfd);
			}
		}
		if fd_coredump >= 0 {
			unsafe {
				close(fd_coredump);
			}
		}
		unsafe {
			_exit(exit_code);
		}
	}
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
