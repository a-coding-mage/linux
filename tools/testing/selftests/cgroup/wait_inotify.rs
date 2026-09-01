// SPDX-License-Identifier: GPL-2.0
/*
 * Wait until an inotify event on the given cgroup file.
 */

use std::ffi::c_void;
use std::mem::size_of;
use std::os::raw::{c_char, c_int, c_long, c_short};
use std::ptr;

const POLLIN: c_short = 0x0001;
const O_RDONLY: c_int = 0;
const IN_MODIFY: u32 = 0x0000_0002;
const EINTR: c_int = 4;

#[repr(C)]
struct FILE {
	_private: [u8; 0],
}

#[repr(C)]
struct pollfd {
	fd: c_int,
	events: c_short,
	revents: c_short,
}

#[repr(C)]
struct inotify_event {
	wd: c_int,
	mask: u32,
	cookie: u32,
	len: u32,
}

unsafe extern "C" {
	static mut stderr: *mut FILE;

	fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
	fn printf(format: *const c_char, ...) -> c_int;
	fn perror(s: *const c_char);
	fn exit(status: c_int) -> !;
	fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
	fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
	fn close(fd: c_int) -> c_int;
	fn inotify_init() -> c_int;
	fn inotify_add_watch(fd: c_int, pathname: *const c_char, mask: u32) -> c_int;
	fn poll(fds: *mut pollfd, nfds: usize, timeout: c_int) -> c_int;
	fn usleep(usec: u32) -> c_int;
	fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
	fn __errno_location() -> *mut c_int;
}

static USAGE: &[u8] = b"Usage: %s [-v] <cgroup_file>\n\0";
static mut FILE_NAME: *mut c_char = ptr::null_mut();
static mut VERBOSE: c_int = 0;

unsafe fn fail_message(msg: *const c_char) -> ! {
	unsafe {
		fprintf(stderr, msg, FILE_NAME);
		exit(1);
	}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
	let cmd: *mut c_char = unsafe { *argv };
	let mut c: c_int;
	let mut fd: c_int;
	let mut fds = pollfd {
		fd: 0,
		events: POLLIN,
		revents: 0,
	};

	loop {
		c = unsafe { getopt(argc, argv, b"v\0".as_ptr() as *const c_char) };
		if c == -1 {
			break;
		}
		match c {
			118 => unsafe {
				VERBOSE += 1;
			},
			_ => {}
		}
		argv = unsafe { argv.add(1) };
		argc -= 1;
	}

	if argc != 2 {
		unsafe {
			fprintf(stderr, USAGE.as_ptr() as *const c_char, cmd);
		}
		return -1;
	}
	unsafe {
		FILE_NAME = *argv.add(1);
	}
	fd = unsafe { open(FILE_NAME, O_RDONLY) };
	if fd < 0 {
		unsafe {
			fail_message(b"Cgroup file %s not found!\n\0".as_ptr() as *const c_char);
		}
	}
	unsafe {
		close(fd);
	}

	fd = unsafe { inotify_init() };
	if fd < 0 {
		unsafe {
			fail_message(b"inotify_init() fails on %s!\n\0".as_ptr() as *const c_char);
		}
	}
	if unsafe { inotify_add_watch(fd, FILE_NAME, IN_MODIFY) } < 0 {
		unsafe {
			fail_message(b"inotify_add_watch() fails on %s!\n\0".as_ptr() as *const c_char);
		}
	}
	fds.fd = fd;

	/*
	 * poll waiting loop
	 */
	loop {
		let ret: c_int = unsafe { poll(&mut fds, 1, 10000) };

		if ret < 0 {
			if unsafe { *__errno_location() } == EINTR {
				continue;
			}
			unsafe {
				perror(b"poll\0".as_ptr() as *const c_char);
				exit(1);
			}
		}
		if (ret > 0) && ((fds.revents & POLLIN) != 0) {
			break;
		}
	}
	if unsafe { VERBOSE } != 0 {
		let mut events: [inotify_event; 10] = unsafe { std::mem::zeroed() };
		let len: c_long;

		unsafe {
			usleep(1000);
			len = read(
				fd,
				events.as_mut_ptr() as *mut c_void,
				size_of::<[inotify_event; 10]>(),
			) as c_long;
			printf(
				b"Number of events read = %ld\n\0".as_ptr() as *const c_char,
				len / size_of::<inotify_event>() as c_long,
			);
		}
	}
	unsafe {
		close(fd);
	}
	return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
