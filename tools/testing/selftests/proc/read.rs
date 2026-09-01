/*
 * Copyright © 2018 Alexey Dobriyan <adobriyan@gmail.com>
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
// Test
// 1) read and lseek on every file in /proc
// 2) readlink of every symlink in /proc
// 3) recursively (1) + (2) for every directory in /proc
// 4) write to /proc/*/clear_refs and /proc/*/task/*/clear_refs
// 5) write to /proc/sysrq-trigger

use std::ffi::{c_char, c_int, c_long, c_uint, c_void};
use std::ptr;

type size_t = usize;
type ssize_t = isize;
type off_t = i64;

const DT_DIR: u8 = 4;
const DT_REG: u8 = 8;
const DT_LNK: u8 = 10;

const O_RDONLY: c_int = 0;
const O_WRONLY: c_int = 1;
const O_NONBLOCK: c_int = 0o4000;
const O_DIRECTORY: c_int = 0o200000;
const SEEK_SET: c_int = 0;

#[repr(C)]
pub struct DIR {
	_private: [u8; 0],
}

#[repr(C)]
pub struct FILE {
	_private: [u8; 0],
}

#[repr(C)]
pub struct dirent {
	pub d_ino: u64,
	pub d_off: i64,
	pub d_reclen: u16,
	pub d_type: u8,
	pub d_name: [c_char; 256],
}

#[repr(C)]
pub struct fsid_t {
	pub __val: [c_int; 2],
}

#[repr(C)]
pub struct statfs {
	pub f_type: c_long,
	pub f_bsize: c_long,
	pub f_blocks: u64,
	pub f_bfree: u64,
	pub f_bavail: u64,
	pub f_files: u64,
	pub f_ffree: u64,
	pub f_fsid: fsid_t,
	pub f_namelen: c_long,
	pub f_frsize: c_long,
	pub f_flags: c_long,
	pub f_spare: [c_long; 4],
}

unsafe extern "C" {
	static mut stderr: *mut FILE;

	fn openat(dirfd: c_int, pathname: *const c_char, flags: c_int, ...) -> c_int;
	fn dirfd(dirp: *mut DIR) -> c_int;
	fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t;
	fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
	fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
	fn close(fd: c_int) -> c_int;
	fn readlinkat(
		dirfd: c_int,
		pathname: *const c_char,
		buf: *mut c_char,
		bufsiz: size_t,
	) -> ssize_t;
	fn fdopendir(fd: c_int) -> *mut DIR;
	fn closedir(dirp: *mut DIR) -> c_int;
	fn opendir(name: *const c_char) -> *mut DIR;
	fn fstatfs(fd: c_int, buf: *mut statfs) -> c_int;
	fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;

	// From "proc.h".
	fn xreaddir(dirp: *mut DIR) -> *mut dirent;
	fn streq(a: *const c_char, b: *const c_char) -> bool;
}

unsafe fn f_reg(d: *mut DIR, filename: *const c_char) {
	let mut buf = [0u8; 4096];
	let fd: c_int;
	let rv: ssize_t;

	/* read from /proc/kmsg can block */
	fd = openat(dirfd(d), filename, O_RDONLY | O_NONBLOCK);
	if fd == -1 {
		return;
	}
	/* struct proc_ops::proc_lseek is mandatory if file is seekable. */
	let _ = lseek(fd, 0, SEEK_SET);
	rv = read(fd, buf.as_mut_ptr() as *mut c_void, buf.len());
	assert!((0 <= rv && rv as usize <= buf.len()) || rv == -1);
	close(fd);
}

unsafe fn f_reg_write(d: *mut DIR, filename: *const c_char, buf: *const c_char, len: size_t) {
	let fd: c_int;
	let rv: ssize_t;

	fd = openat(dirfd(d), filename, O_WRONLY);
	if fd == -1 {
		return;
	}
	rv = write(fd, buf as *const c_void, len);
	assert!((0 <= rv && rv as usize <= len) || rv == -1);
	close(fd);
}

unsafe fn f_lnk(d: *mut DIR, filename: *const c_char) {
	let mut buf = [0 as c_char; 4096];
	let rv: ssize_t;

	rv = readlinkat(dirfd(d), filename, buf.as_mut_ptr(), buf.len());
	assert!((0 <= rv && rv as usize <= buf.len()) || rv == -1);
}

unsafe fn f(d: *mut DIR, level: c_uint) {
	let mut de: *mut dirent;

	de = xreaddir(d);
	assert!((*de).d_type == DT_DIR);
	assert!(streq((*de).d_name.as_ptr(), c".".as_ptr()));

	de = xreaddir(d);
	assert!((*de).d_type == DT_DIR);
	assert!(streq((*de).d_name.as_ptr(), c"..".as_ptr()));

	loop {
		de = xreaddir(d);
		if de.is_null() {
			break;
		}

		assert!(!streq((*de).d_name.as_ptr(), c".".as_ptr()));
		assert!(!streq((*de).d_name.as_ptr(), c"..".as_ptr()));

		match (*de).d_type {
			DT_REG => {
				if level == 0 && streq((*de).d_name.as_ptr(), c"sysrq-trigger".as_ptr()) {
					f_reg_write(d, (*de).d_name.as_ptr(), c"h".as_ptr(), 1);
				} else if level == 1 && streq((*de).d_name.as_ptr(), c"clear_refs".as_ptr()) {
					f_reg_write(d, (*de).d_name.as_ptr(), c"1".as_ptr(), 1);
				} else if level == 3 && streq((*de).d_name.as_ptr(), c"clear_refs".as_ptr()) {
					f_reg_write(d, (*de).d_name.as_ptr(), c"1".as_ptr(), 1);
				} else {
					f_reg(d, (*de).d_name.as_ptr());
				}
			}
			DT_DIR => {
				let fd: c_int;
				let dd: *mut DIR;

				fd = openat(dirfd(d), (*de).d_name.as_ptr(), O_DIRECTORY | O_RDONLY);
				if fd == -1 {
					continue;
				}
				dd = fdopendir(fd);
				if dd.is_null() {
					continue;
				}
				f(dd, level + 1);
				closedir(dd);
			}
			DT_LNK => {
				f_lnk(d, (*de).d_name.as_ptr());
			}
			_ => {
				assert!(false);
			}
		}
	}
}

fn main() {
	unsafe {
		let d: *mut DIR;
		let mut sfs: statfs = std::mem::zeroed();

		d = opendir(c"/proc".as_ptr());
		if d.is_null() {
			std::process::exit(4);
		}

		/* Ensure /proc is proc. */
		if fstatfs(dirfd(d), &mut sfs) == -1 {
			std::process::exit(1);
		}
		if sfs.f_type != 0x9fa0 {
			fprintf(
				stderr,
				c"error: unexpected f_type %lx\n".as_ptr(),
				sfs.f_type as c_long,
			);
			std::process::exit(2);
		}

		f(d, 0);
	}
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
