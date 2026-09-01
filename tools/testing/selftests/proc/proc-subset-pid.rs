/*
 * Copyright (c) 2021 Alexey Dobriyan <adobriyan@gmail.com>
 *
 * Permission to use, copy, modify, and distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */
/*
 * Test that "mount -t proc -o subset=pid" hides everything but pids,
 * /proc/self and /proc/thread-self.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::c_void;
use std::os::raw::{c_char, c_int, c_long, c_uint, c_ulong};
use std::ptr;

const CLONE_NEWNS: c_int = 0x00020000;
const DT_DIR: u8 = 4;
const DT_LNK: u8 = 10;
const ENOENT: c_int = 2;
const ENOSYS: c_int = 38;
const EPERM: c_int = 1;
const MS_PRIVATE: c_ulong = 1 << 18;
const MS_REC: c_ulong = 16384;
const O_RDONLY: c_int = 0;

#[repr(C)]
struct DIR {
	_private: [u8; 0],
}

#[repr(C)]
struct dirent {
	d_ino: u64,
	d_off: i64,
	d_reclen: u16,
	d_type: u8,
	d_name: [c_char; 256],
}

unsafe extern "C" {
	static mut stderr: *mut c_void;

	fn __errno_location() -> *mut c_int;
	fn exit(status: c_int) -> !;
	fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
	fn mount(
		source: *const c_char,
		target: *const c_char,
		filesystemtype: *const c_char,
		mountflags: c_ulong,
		data: *const c_void,
	) -> c_int;
	fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
	fn opendir(name: *const c_char) -> *mut DIR;
	fn readdir(dirp: *mut DIR) -> *mut dirent;
	fn readlink(pathname: *const c_char, buf: *mut c_char, bufsiz: usize) -> isize;
	fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
	fn unshare(flags: c_int) -> c_int;
}

unsafe fn errno() -> c_int {
	unsafe { *__errno_location() }
}

#[inline]
unsafe fn streq(a: *const c_char, b: *const c_char) -> bool {
	unsafe { strcmp(a, b) == 0 }
}

unsafe fn make_private_proc() {
	unsafe {
		if unshare(CLONE_NEWNS) == -1 {
			if errno() == ENOSYS || errno() == EPERM {
				exit(4);
			}
			exit(1);
		}
		if mount(
			ptr::null(),
			c"/".as_ptr(),
			ptr::null(),
			MS_PRIVATE | MS_REC,
			ptr::null(),
		) == -1
		{
			exit(1);
		}
		if mount(
			ptr::null(),
			c"/proc".as_ptr(),
			c"proc".as_ptr(),
			0,
			c"subset=pid".as_ptr() as *const c_void,
		) == -1
		{
			exit(1);
		}
	}
}

unsafe fn string_is_pid(mut s: *const c_char) -> bool {
	unsafe {
		loop {
			let ch = *s;
			s = s.add(1);
			match ch {
				b'0' as c_char
				| b'1' as c_char
				| b'2' as c_char
				| b'3' as c_char
				| b'4' as c_char
				| b'5' as c_char
				| b'6' as c_char
				| b'7' as c_char
				| b'8' as c_char
				| b'9' as c_char => continue,

				0 => return true,

				_ => return false,
			}
		}
	}
}

fn main() {
	unsafe {
		make_private_proc();

		let d = opendir(c"/proc".as_ptr());
		assert!(!d.is_null());

		let mut de: *mut dirent;

		let mut dot = false;
		let mut dot_dot = false;
		let mut self_ = false;
		let mut thread_self = false;

		loop {
			de = readdir(d);
			if de.is_null() {
				break;
			}

			if streq((*de).d_name.as_ptr(), c".".as_ptr()) {
				assert!(!dot);
				dot = true;
				assert!((*de).d_type == DT_DIR);
			} else if streq((*de).d_name.as_ptr(), c"..".as_ptr()) {
				assert!(!dot_dot);
				dot_dot = true;
				assert!((*de).d_type == DT_DIR);
			} else if streq((*de).d_name.as_ptr(), c"self".as_ptr()) {
				assert!(!self_);
				self_ = true;
				assert!((*de).d_type == DT_LNK);
			} else if streq((*de).d_name.as_ptr(), c"thread-self".as_ptr()) {
				assert!(!thread_self);
				thread_self = true;
				assert!((*de).d_type == DT_LNK);
			} else {
				if !string_is_pid((*de).d_name.as_ptr()) {
					fprintf(stderr, c"d_name '%s'\n".as_ptr(), (*de).d_name.as_ptr());
					assert!(false);
				}
				assert!((*de).d_type == DT_DIR);
			}
		}

		let mut c: c_char = 0;
		let rv = readlink(c"/proc/cpuinfo".as_ptr(), &mut c, 1);
		assert!(rv == -1 && errno() == ENOENT);

		let fd = open(c"/proc/cpuinfo".as_ptr(), O_RDONLY);
		assert!(fd == -1 && errno() == ENOENT);
	}
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
