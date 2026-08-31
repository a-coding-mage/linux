// SPDX-License-Identifier: GPL-2.0
/*
 * User Events FTrace Test Program
 *
 * Copyright (c) 2021 Beau Belgrave <beaub@linux.microsoft.com>
 */

#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]

use std::ffi::c_void;
use std::mem;
use std::ptr;
use std::slice;
use std::ffi::{c_char, c_int, c_long, c_uint, c_ulong};

pub type __u32 = u32;
pub type __u64 = u64;
pub type __s32 = i32;

pub const O_RDONLY: c_int = 0;
pub const O_RDWR: c_int = 2;
pub const O_CREAT: c_int = 0o100;
pub const O_EXCL: c_int = 0o200;

pub const PROT_READ: c_int = 1;
pub const MAP_PRIVATE: c_int = 2;
pub const MAP_ANONYMOUS: c_int = 0x20;
pub const MADV_DONTNEED: c_int = 4;

pub const EOK: c_int = 0;
pub const ENOENT: c_int = 2;
pub const EADDRINUSE: c_int = 98;
pub const EBUSY: c_int = 16;
pub const EBADF: c_int = 9;
pub const EINVAL: c_int = 22;
pub const EFAULT: c_int = 14;

pub const EOF: c_int = -1;
pub const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

// ioctl command numbers are macro-expanded from linux/user_events.h in the original build.
// Keep local symbolic placeholders so this file remains self-contained.
pub const DIAG_IOCSREG: c_ulong = 0;
pub const DIAG_IOCSUNREG: c_ulong = 0;
pub const DIAG_IOCSDEL: c_ulong = 0;

#[repr(C)]
pub struct user_reg {
    pub size: __u32,
    pub name_args: __u64,
    pub enable_bit: __u32,
    pub enable_addr: __u64,
    pub enable_size: __u32,
    pub write_index: __s32,
}

#[repr(C)]
pub struct user_unreg {
    pub size: __u32,
    pub disable_bit: __u32,
    pub disable_addr: __u64,
}

#[repr(C)]
pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: usize,
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct user {
    pub status_fd: c_int,
    pub data_fd: c_int,
    pub enable_fd: c_int,
    pub check: c_int,
    pub umount: bool,
}

#[link(name = "c")]
unsafe extern "C" {
    fn open(pathname: *const c_char, flags: c_int, mode: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn getc(stream: *mut FILE) -> c_int;
    fn fopen(filename: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fgets(s: *mut c_char, n: c_int, stream: *mut FILE) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn ioctl(fd: c_int, request: c_ulong, argp: *mut c_void) -> c_int;
    fn mmap(addr: *mut c_void, len: usize, prot: c_int, flags: c_int, fd: c_int, offset: isize) -> *mut c_void;
    fn munmap(addr: *mut c_void, len: usize) -> c_int;
    fn madvise(addr: *mut c_void, length: usize, advice: c_int) -> c_int;
    fn writev(fd: c_int, iov: *const iovec, iovcnt: c_int) -> isize;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strchrnul(s: *const c_char, c: c_int) -> *mut c_char;
    fn printf(format: *const c_char, ...) -> c_int;
    fn snprintf(buf: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn test_harness_run(argc: c_int, argv: *mut *mut c_char) -> c_int;
    fn usleep(usec: c_ulong) -> c_int;
    #[cfg(target_os = "linux")]
    fn __errno_location() -> *mut c_int;
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {{
        let lhs = $left;
        let rhs = $right;
        if lhs != rhs {
            panic!("ASSERT_EQ failed: {:?} != {:?}", lhs, rhs);
        }
    }};
}

macro_rules! ASSERT_NE {
    ($left:expr, $right:expr) => {{
        let lhs = $left;
        let rhs = $right;
        if lhs == rhs {
            panic!("ASSERT_NE failed: {:?} == {:?}", lhs, rhs);
        }
    }};
}

macro_rules! ASSERT_GT {
    ($left:expr, $right:expr) => {{
        let lhs = $left;
        let rhs = $right;
        if !(lhs > rhs) {
            panic!("ASSERT_GT failed: {:?} <= {:?}", lhs, rhs);
        }
    }};
}

macro_rules! ASSERT_TRUE {
    ($expr:expr) => {{
        if !$expr {
            panic!("ASSERT_TRUE failed");
        }
    }};
}

fn errno() -> c_int {
    #[cfg(target_os = "linux")]
    unsafe {
        *__errno_location()
    }
    #[cfg(not(target_os = "linux"))]
    0
}

fn set_errno(value: c_int) {
    #[cfg(target_os = "linux")]
    unsafe {
        *__errno_location() = value;
    }
}

fn c_str(s: &'static [u8]) -> *const c_char {
    s.as_ptr() as *const c_char
}

fn c_mut_ptr(s: &'static [u8]) -> *mut c_char {
    s.as_ptr() as *mut c_char
}

fn c_slice_ptr<T>(s: &mut T) -> *mut c_void {
    s as *mut T as *mut c_void
}

const DATA_FILE: &[u8] = b"/sys/kernel/tracing/user_events_data\0";
const STATUS_FILE: &[u8] = b"/sys/kernel/tracing/user_events_status\0";
const ENABLE_FILE: &[u8] = b"/sys/kernel/tracing/events/user_events/__test_event/enable\0";
const TRACE_FILE: &[u8] = b"/sys/kernel/tracing/trace\0";
const FMT_FILE: &[u8] = b"/sys/kernel/tracing/events/user_events/__test_event/format\0";

static DATA_FILE_PTR: *const c_char = DATA_FILE.as_ptr() as *const c_char;
static STATUS_FILE_PTR: *const c_char = STATUS_FILE.as_ptr() as *const c_char;
static ENABLE_FILE_PTR: *const c_char = ENABLE_FILE.as_ptr() as *const c_char;
static TRACE_FILE_PTR: *const c_char = TRACE_FILE.as_ptr() as *const c_char;
static FMT_FILE_PTR: *const c_char = FMT_FILE.as_ptr() as *const c_char;

static C_STR0: &[u8] = b"0\0";
static C_STR1: &[u8] = b"1\0";

fn c_true() -> c_int { 1 }
fn c_false() -> c_int { 0 }

static _DATA_FILE: *const c_char = DATA_FILE_PTR;
static _STATUS_FILE: *const c_char = STATUS_FILE_PTR;
static _ENABLE_FILE: *const c_char = ENABLE_FILE_PTR;
static _TRACE_FILE: *const c_char = TRACE_FILE_PTR;
static _FMT_FILE: *const c_char = FMT_FILE_PTR;

#[allow(non_upper_case_globals)]
pub const data_file: *const c_char = _DATA_FILE;
pub const status_file: *const c_char = _STATUS_FILE;
pub const enable_file: *const c_char = _ENABLE_FILE;
pub const trace_file: *const c_char = _TRACE_FILE;
pub const fmt_file: *const c_char = _FMT_FILE;

fn trace_bytes() -> c_int {
    let fd = unsafe { open(trace_file, O_RDONLY, 0) };
    let mut buf = [0u8; 256];
    let mut bytes: c_int = 0;

    if fd == -1 {
        return -1;
    }

    loop {
        let got = unsafe { read(fd, buf.as_mut_ptr() as *mut c_void, buf.len()) };

        if got == -1 {
            unsafe {
                close(fd);
            }
            return -1;
        }

        if got == 0 {
            break;
        }

        bytes += got as c_int;
    }

    unsafe { close(fd) };

    bytes
}

fn skip_until_empty_line(fp: *mut FILE) -> c_int {
    let mut c: c_int;
    let mut last: c_int = 0;

    loop {
        unsafe {
            c = getc(fp);
        }

        if c == EOF {
            break;
        }

        if last == b'\n' as c_int && c == b'\n' as c_int {
            return 0;
        }

        last = c;
    }

    -1
}

fn get_print_fmt(buffer: &mut [c_char], len: c_int) -> c_int {
    let fp = unsafe { fopen(fmt_file, c_str(b"r\0")) };

    if fp.is_null() {
        return -1;
    }

    /* Read until empty line (Skip Common) */
    if skip_until_empty_line(fp) < 0 {
        unsafe {
            fclose(fp);
        }
        return -1;
    }

    /* Read until empty line (Skip Properties) */
    if skip_until_empty_line(fp) < 0 {
        unsafe {
            fclose(fp);
        }
        return -1;
    }

    /* Read in print_fmt: */
    let got = unsafe { fgets(buffer.as_mut_ptr(), len, fp) };

    if got.is_null() {
        unsafe {
            fclose(fp);
        }
        return -1;
    }

    let newline = unsafe { strchr(buffer.as_ptr(), '\n' as c_int) };

    if !newline.is_null() {
        unsafe {
            *newline = c_false() as c_char;
        }
    }

    unsafe { fclose(fp) };

    0
}

fn wait_for_delete() -> bool {
    let mut i = 0;

    while i < 1000 {
        let fd = unsafe { open(enable_file, O_RDONLY, 0) };

        if fd == -1 {
            return true;
        }

        unsafe {
            close(fd);
            usleep(1000);
        }
        i += 1;
    }

    false
}

fn clear(check: *mut c_int) -> c_int {
    let mut unreg: user_unreg = unsafe { mem::zeroed() };
    let fd: c_int;

    unreg.size = mem::size_of::<user_unreg>() as __u32;
    unreg.disable_bit = 31;
    unreg.disable_addr = (check as usize) as __u64;

    fd = unsafe { open(data_file, O_RDWR, 0) };

    if fd == -1 {
        return -1;
    }

    let rc = unsafe { ioctl(fd, DIAG_IOCSUNREG, (&mut unreg as *mut user_unreg).cast()) };
    if rc == -1 {
        let e = errno();
        if e != ENOENT {
            unsafe { close(fd) };
            return -1;
        }
    }

    let rc = unsafe { ioctl(fd, DIAG_IOCSDEL, c_mut_ptr(b"__test_event\0" ) as *mut c_void) };
    if rc == -1 {
        let e = errno();
        if e == EBUSY {
            if !wait_for_delete() {
                unsafe { close(fd) };
                return -1;
            }
        } else if e != ENOENT {
            unsafe { close(fd) };
            return -1;
        }
    }

    unsafe { close(fd) };

    0
}

fn check_print_fmt(event: *const c_char, expected: *const c_char, check: *mut c_int) -> c_int {
    let mut reg: user_reg = unsafe { mem::zeroed() };
    let mut print_fmt = [0 as c_char; 256];
    let ret: c_int;
    let fd: c_int;

    /* Ensure cleared */
    ret = clear(check);

    if ret != 0 {
        return ret;
    }

    fd = unsafe { open(data_file, O_RDWR, 0) };

    if fd == -1 {
        return fd;
    }

    reg.size = mem::size_of::<user_reg>() as __u32;
    reg.name_args = (event as usize) as __u64;
    reg.enable_bit = 31;
    reg.enable_addr = (check as usize) as __u64;
    reg.enable_size = mem::size_of::<c_int>() as __u32;

    /* Register should work */
    ret = unsafe { ioctl(fd, DIAG_IOCSREG, (&mut reg as *mut user_reg).cast()) };

    if ret != 0 {
        unsafe {
            close(fd);
            printf(c_str(b"Reg failed in fmt\n\0"));
        }
        return ret;
    }

    /* Ensure correct print_fmt */
    ret = get_print_fmt(&mut print_fmt, print_fmt.len() as c_int);

    unsafe { close(fd) };

    if ret != 0 {
        return ret;
    }

    unsafe { strcmp(print_fmt.as_ptr(), expected) }
}

fn fix_up_user(user: &mut user) {
    // USER_EVENT_FIXTURE_SETUP(return, self->umount);
    user.status_fd = unsafe { open(status_file, O_RDONLY, 0) };
    ASSERT_NE!(-1, user.status_fd);

    user.data_fd = unsafe { open(data_file, O_RDWR, 0) };
    ASSERT_NE!(-1, user.data_fd);

    user.enable_fd = -1;
}

fn tear_down_user(user: &mut user) {
    // USER_EVENT_FIXTURE_TEARDOWN(self->umount);
    unsafe {
        close(user.status_fd);
        close(user.data_fd);

        if user.enable_fd != -1 {
            let _ = write(user.enable_fd, C_STR0.as_ptr() as *const c_void, C_STR0.len());
            close(user.enable_fd);
        }
    }

    if clear(&mut user.check as *mut c_int) != 0 {
        unsafe {
            printf(c_str(b"WARNING: Clear didn't work!\n\0"));
        }
    }
}

fn test_user_register_events(self_: &mut user) {
    let mut reg: user_reg = unsafe { mem::zeroed() };
    let mut unreg: user_unreg = unsafe { mem::zeroed() };

    reg.size = mem::size_of::<user_reg>() as __u32;
    reg.name_args = c_mut_ptr(b"__test_event u32 field1; u32 field2\0") as usize as __u64;
    reg.enable_bit = 31;
    reg.enable_addr = (ptr::addr_of_mut!(self_->check) as *mut c_int) as usize as __u64;
    reg.enable_size = mem::size_of::<c_int>() as __u32;

    unreg.size = mem::size_of::<user_unreg>() as __u32;
    unreg.disable_bit = 31;
    unreg.disable_addr = (ptr::addr_of_mut!(self_->check) as *mut c_int) as usize as __u64;

    /* Register should work */
    ASSERT_EQ!(0, unsafe { ioctl(self_.data_fd, DIAG_IOCSREG, (&mut reg as *mut user_reg).cast()) });
    ASSERT_EQ!(0, reg.write_index);

    /* Multiple registers to the same addr + bit should fail */
    ASSERT_EQ!(-1, unsafe { ioctl(self_.data_fd, DIAG_IOCSREG, (&mut reg as *mut user_reg).cast()) });
    ASSERT_EQ!(EADDRINUSE, errno());

    /* Multiple registers to same name should result in same index */
    reg.enable_bit = 30;
    ASSERT_EQ!(0, unsafe { ioctl(self_.data_fd, DIAG_IOCSREG, (&mut reg as *mut user_reg).cast()) });
    ASSERT_EQ!(0, reg.write_index);

    /* Register without separator spacing should still match */
    reg.enable_bit = 29;
    reg.name_args = c_mut_ptr(b"__test_event u32 field1;u32 field2\0") as usize as __u64;
    ASSERT_EQ!(0, unsafe { ioctl(self_.data_fd, DIAG_IOCSREG, (&mut reg as *mut user_reg).cast()) });
    ASSERT_EQ!(0, reg.write_index);

    /* Multiple registers to same name but different args should fail */
    reg.enable_bit = 29;
    reg.name_args = c_mut_ptr(b"__test_event u32 field1;\0") as usize as __u64;
    ASSERT_EQ!(-1, unsafe { ioctl(self_.data_fd, DIAG_IOCSREG, (&mut reg as *mut user_reg).cast()) });
    ASSERT_EQ!(EADDRINUSE, errno());

    /* Ensure disabled */
    self_.enable_fd = unsafe { open(enable_file, O_RDWR, 0) };
    ASSERT_NE!(-1, self_.enable_fd);
    ASSERT_NE!(-1, unsafe { write(self_.enable_fd, C_STR0.as_ptr() as *const c_void, C_STR0.len()) });

    /* Enable event and ensure bits updated in status */
    ASSERT_NE!(-1, unsafe { write(self_.enable_fd, C_STR1.as_ptr() as *const c_void, C_STR1.len()) });
    ASSERT_EQ!((1i32 << reg.enable_bit), self_.check);

    /* Disable event and ensure bits updated in status */
    ASSERT_NE!(-1, unsafe { write(self_.enable_fd, C_STR0.as_ptr() as *const c_void, C_STR0.len()) });
    ASSERT_EQ!(0, self_.check);

    /* File still open should return -EBUSY for delete */
    ASSERT_EQ!(-1, unsafe { ioctl(self_.data_fd, DIAG_IOCSDEL, c_mut_ptr(b"__test_event\0") as *mut c_void) });
    ASSERT_EQ!(EBUSY, errno());

    /* Unregister */
    ASSERT_EQ!(0, unsafe { ioctl(self_.data_fd, DIAG_IOCSUNREG, (&mut unreg as *mut user_unreg).cast()) });
    unreg.disable_bit = 30;
    ASSERT_EQ!(0, unsafe { ioctl(self_.data_fd, DIAG_IOCSUNREG, (&mut unreg as *mut user_unreg).cast()) });
    unreg.disable_bit = 29;
    ASSERT_EQ!(0, unsafe { ioctl(self_.data_fd, DIAG_IOCSUNREG, (&mut unreg as *mut user_unreg).cast()) });

    /* Delete should have been auto-done after close and unregister */
    unsafe {
        close(self_.data_fd);
    }

    ASSERT_TRUE!(wait_for_delete());
}

fn test_user_write_events(self_: &mut user) {
    let mut reg: user_reg = unsafe { mem::zeroed() };
    let mut io: [iovec; 3] = unsafe { mem::zeroed() };
    let field1: c_uint = 1;
    let field2: c_uint = 2;
    let mut before: c_int = 0;
    let mut after: c_int = 0;

    reg.size = mem::size_of::<user_reg>() as __u32;
    reg.name_args = c_mut_ptr(b"__test_event u32 field1; u32 field2\0") as usize as __u64;
    reg.enable_bit = 31;
    reg.enable_addr = (ptr::addr_of_mut!(self_.check) as *mut c_int) as usize as __u64;
    reg.enable_size = mem::size_of::<c_int>() as __u32;

    io[0].iov_base = c_mut_ptr(b"" ) as *mut c_void;
    io[0].iov_len = mem::size_of_val(&reg.write_index);
    io[1].iov_base = c_mut_ptr(b"\0");
    io[1].iov_len = mem::size_of_val(&field1);
    io[2].iov_base = c_mut_ptr(b"\0");
    io[2].iov_len = mem::size_of_val(&field2);

    io[0].iov_base = ptr::addr_of!(reg.write_index) as *mut c_void;
    io[0].iov_len = mem::size_of::<c_int>();
    io[1].iov_base = (&field1 as *const c_uint) as *mut c_void;
    io[1].iov_len = mem::size_of::<c_uint>();
    io[2].iov_base = (&field2 as *const c_uint) as *mut c_void;
    io[2].iov_len = mem::size_of::<c_uint>();

    /* Register should work */
    ASSERT_EQ!(0, unsafe { ioctl(self_.data_fd, DIAG_IOCSREG, (&mut reg as *mut user_reg).cast()) });
    ASSERT_EQ!(0, reg.write_index);
    ASSERT_EQ!(0, self_.check);

    /* Write should fail on invalid slot with ENOENT */
    io[0].iov_base = (&field2 as *const c_uint) as *mut c_void;
    io[0].iov_len = mem::size_of::<c_uint>();
    ASSERT_EQ!(-1, unsafe { writev(self_.data_fd, io.as_ptr(), 3) });
    ASSERT_EQ!(ENOENT, errno());
    io[0].iov_base = ptr::addr_of!(reg.write_index) as *mut c_void;
    io[0].iov_len = mem::size_of::<c_int>();

    /* Write should return -EBADF when event is not enabled */
    ASSERT_EQ!(-1, unsafe { writev(self_.data_fd, io.as_ptr(), 3) });
    ASSERT_EQ!(EBADF, errno());

    /* Enable event */
    self_.enable_fd = unsafe { open(enable_file, O_RDWR, 0) };
    ASSERT_NE!(-1, unsafe { write(self_.enable_fd, C_STR1.as_ptr() as *const c_void, C_STR1.len()) });

    /* Event should now be enabled */
    ASSERT_NE!(1i32 << reg.enable_bit, self_.check);

    /* Write should make it out to ftrace buffers */
    before = trace_bytes();
    ASSERT_NE!(-1, unsafe { writev(self_.data_fd, io.as_ptr(), 3) });
    after = trace_bytes();
    ASSERT_GT!(after, before);

    /* Negative index should fail with EINVAL */
    reg.write_index = -1;
    ASSERT_EQ!(-1, unsafe { writev(self_.data_fd, io.as_ptr(), 3) });
    ASSERT_EQ!(EINVAL, errno());
}

fn test_user_write_empty_events(self_: &mut user) {
    let mut reg: user_reg = unsafe { mem::zeroed() };
    let io: [iovec; 1] = unsafe { mem::zeroed() };
    let mut before = 0;
    let mut after = 0;
    let mut io = io;

    reg.size = mem::size_of::<user_reg>() as __u32;
    reg.name_args = c_mut_ptr(b"__test_event\0") as usize as __u64;
    reg.enable_bit = 31;
    reg.enable_addr = (ptr::addr_of_mut!(self_.check) as *mut c_int) as usize as __u64;
    reg.enable_size = mem::size_of::<c_int>() as __u32;

    io[0].iov_base = ptr::addr_of!(reg.write_index) as *mut c_void;
    io[0].iov_len = mem::size_of::<c_int>();

    /* Register should work */
    ASSERT_EQ!(0, unsafe { ioctl(self_.data_fd, DIAG_IOCSREG, (&mut reg as *mut user_reg).cast()) });
    ASSERT_EQ!(0, reg.write_index);
    ASSERT_EQ!(0, self_.check);

    /* Enable event */
    self_.enable_fd = unsafe { open(enable_file, O_RDWR, 0) };
    ASSERT_NE!(-1, unsafe { write(self_.enable_fd, C_STR1.as_ptr() as *const c_void, C_STR1.len()) });

    /* Event should now be enabled */
    ASSERT_EQ!(1i32 << reg.enable_bit, self_.check);

    /* Write should make it out to ftrace buffers */
    before = trace_bytes();
    ASSERT_NE!(-1, unsafe { writev(self_.data_fd, io.as_ptr(), 1) });
    after = trace_bytes();
    ASSERT_GT!(after, before);
}

fn test_user_write_fault(self_: &mut user) {
    let mut reg: user_reg = unsafe { mem::zeroed() };
    let mut io: [iovec; 2] = unsafe { mem::zeroed() };
    let l = mem::size_of::<__u64>();
    let anon = unsafe { mmap(ptr::null_mut(), l, PROT_READ, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0) };

    ASSERT_NE!(MAP_FAILED, anon);

    reg.size = mem::size_of::<user_reg>() as __u32;
    reg.name_args = c_mut_ptr(b"__test_event u64 anon\0") as usize as __u64;
    reg.enable_bit = 31;
    reg.enable_addr = (ptr::addr_of_mut!(self_.check) as *mut c_int) as usize as __u64;
    reg.enable_size = mem::size_of::<c_int>() as __u32;

    io[0].iov_base = ptr::addr_of!(reg.write_index) as *mut c_void;
    io[0].iov_len = mem::size_of::<c_int>();
    io[1].iov_base = anon;
    io[1].iov_len = l;

    /* Register should work */
    ASSERT_EQ!(0, unsafe { ioctl(self_.data_fd, DIAG_IOCSREG, (&mut reg as *mut user_reg).cast()) });
    ASSERT_EQ!(0, reg.write_index);

    /* Enable event */
    self_.enable_fd = unsafe { open(enable_file, O_RDWR, 0) };
    ASSERT_NE!(-1, unsafe { write(self_.enable_fd, C_STR1.as_ptr() as *const c_void, C_STR1.len()) });

    /* Write should work normally */
    ASSERT_NE!(-1, unsafe { writev(self_.data_fd, io.as_ptr(), 2) });

    /* Faulted data should zero fill and work */
    ASSERT_EQ!(0, unsafe { madvise(anon, l, MADV_DONTNEED) });
    ASSERT_NE!(-1, unsafe { writev(self_.data_fd, io.as_ptr(), 2) });
    ASSERT_EQ!(0, unsafe { munmap(anon, l) });
}

fn test_user_write_validator(self_: &mut user) {
    let mut reg: user_reg = unsafe { mem::zeroed() };
    let mut io: [iovec; 3] = unsafe { mem::zeroed() };
    let mut loc: c_int = 0;
    let mut data: [c_char; 8] = [0; 8];
    let mut bytes: c_int;
    let mut before: c_int = 0;
    let mut after: c_int = 0;

    reg.size = mem::size_of::<user_reg>() as __u32;
    reg.name_args = c_mut_ptr(b"__test_event __rel_loc char[] data\0") as usize as __u64;
    reg.enable_bit = 31;
    reg.enable_addr = (ptr::addr_of_mut!(self_.check) as *mut c_int) as usize as __u64;
    reg.enable_size = mem::size_of::<c_int>() as __u32;

    /* Register should work */
    ASSERT_EQ!(0, unsafe { ioctl(self_.data_fd, DIAG_IOCSREG, (&mut reg as *mut user_reg).cast()) });
    ASSERT_EQ!(0, reg.write_index);
    ASSERT_EQ!(0, self_.check);

    io[0].iov_base = ptr::addr_of!(reg.write_index) as *mut c_void;
    io[0].iov_len = mem::size_of::<c_int>();
    io[1].iov_base = ptr::addr_of_mut!(loc) as *mut c_void;
    io[1].iov_len = mem::size_of::<c_int>();
    io[2].iov_base = data.as_mut_ptr() as *mut c_void;
    bytes = unsafe { snprintf(data.as_mut_ptr(), data.len(), c_mut_ptr(b"Test\0")) } + 1;
    io[2].iov_len = bytes as usize;

    /* Undersized write should fail */
    ASSERT_EQ!(-1, unsafe { writev(self_.data_fd, io.as_ptr(), 1) });
    ASSERT_EQ!(EINVAL, errno());

    /* Enable event */
    self_.enable_fd = unsafe { open(enable_file, O_RDWR, 0) };
    ASSERT_NE!(-1, unsafe { write(self_.enable_fd, C_STR1.as_ptr() as *const c_void, C_STR1.len()) });

    /* Event should now be enabled */
    ASSERT_EQ!(1i32 << reg.enable_bit, self_.check);

    /* Full in-bounds write should work */
    before = trace_bytes();
    loc = DYN_LOC(0, bytes);
    ASSERT_NE!(-1, unsafe { writev(self_.data_fd, io.as_ptr(), 3) });
    after = trace_bytes();
    ASSERT_GT!(after, before);

    /* Out of bounds write should fault (offset way out) */
    loc = DYN_LOC(1024, bytes);
    ASSERT_EQ!(-1, unsafe { writev(self_.data_fd, io.as_ptr(), 3) });
    ASSERT_EQ!(EFAULT, errno());

    /* Out of bounds write should fault (offset 1 byte out) */
    loc = DYN_LOC(1, bytes);
    ASSERT_EQ!(-1, unsafe { writev(self_.data_fd, io.as_ptr(), 3) });
    ASSERT_EQ!(EFAULT, errno());

    /* Out of bounds write should fault (size way out) */
    loc = DYN_LOC(0, bytes + 1024);
    ASSERT_EQ!(-1, unsafe { writev(self_.data_fd, io.as_ptr(), 3) });
    ASSERT_EQ!(EFAULT, errno());

    /* Out of bounds write should fault (size 1 byte out) */
    loc = DYN_LOC(0, bytes + 1);
    ASSERT_EQ!(-1, unsafe { writev(self_.data_fd, io.as_ptr(), 3) });
    ASSERT_EQ!(EFAULT, errno());

    /* Non-Null should fault */
    unsafe {
        memset(data.as_mut_ptr() as *mut c_void, b'A' as c_int, data.len());
    }
    loc = DYN_LOC(0, bytes);
    ASSERT_EQ!(-1, unsafe { writev(self_.data_fd, io.as_ptr(), 3) });
    ASSERT_EQ!(EFAULT, errno());
}

macro_rules! DYN_LOC {
    ($offset:expr, $size:expr) => {
        (($offset as c_int) << 16) | ($size as c_int & 0xffff)
    };
}

fn test_user_print_fmt(self_: &mut user) {
    let mut ret: c_int;

    ret = check_print_fmt(
        c_mut_ptr(b"__test_event __rel_loc char[] data\0"),
        c_mut_ptr(b"print fmt: \"data=%s\", __get_rel_str(data)\0"),
        &mut self_.check,
    );
    ASSERT_EQ!(0, ret);

    ret = check_print_fmt(
        c_mut_ptr(b"__test_event __data_loc char[] data\0"),
        c_mut_ptr(b"print fmt: \"data=%s\", __get_str(data)\0"),
        &mut self_.check,
    );
    ASSERT_EQ!(0, ret);

    ret = check_print_fmt(
        c_mut_ptr(b"__test_event s64 data\0"),
        c_mut_ptr(b"print fmt: \"data=%lld\", REC->data\0"),
        &mut self_.check,
    );
    ASSERT_EQ!(0, ret);

    ret = check_print_fmt(
        c_mut_ptr(b"__test_event u64 data\0"),
        c_mut_ptr(b"print fmt: \"data=%llu\", REC->data\0"),
        &mut self_.check,
    );
    ASSERT_EQ!(0, ret);

    ret = check_print_fmt(
        c_mut_ptr(b"__test_event s32 data\0"),
        c_mut_ptr(b"print fmt: \"data=%d\", REC->data\0"),
        &mut self_.check,
    );
    ASSERT_EQ!(0, ret);

    ret = check_print_fmt(
        c_mut_ptr(b"__test_event u32 data\0"),
        c_mut_ptr(b"print fmt: \"data=%u\", REC->data\0"),
        &mut self_.check,
    );
    ASSERT_EQ!(0, ret);

    ret = check_print_fmt(
        c_mut_ptr(b"__test_event int data\0"),
        c_mut_ptr(b"print fmt: \"data=%d\", REC->data\0"),
        &mut self_.check,
    );
    ASSERT_EQ!(0, ret);

    ret = check_print_fmt(
        c_mut_ptr(b"__test_event unsigned int data\0"),
        c_mut_ptr(b"print fmt: \"data=%u\", REC->data\0"),
        &mut self_.check,
    );
    ASSERT_EQ!(0, ret);

    ret = check_print_fmt(
        c_mut_ptr(b"__test_event s16 data\0"),
        c_mut_ptr(b"print fmt: \"data=%d\", REC->data\0"),
        &mut self_.check,
    );
    ASSERT_EQ!(0, ret);

    ret = check_print_fmt(
        c_mut_ptr(b"__test_event u16 data\0"),
        c_mut_ptr(b"print fmt: \"data=%u\", REC->data\0"),
        &mut self_.check,
    );
    ASSERT_EQ!(0, ret);

    ret = check_print_fmt(
        c_mut_ptr(b"__test_event short data\0"),
        c_mut_ptr(b"print fmt: \"data=%d\", REC->data\0"),
        &mut self_.check,
    );
    ASSERT_EQ!(0, ret);

    ret = check_print_fmt(
        c_mut_ptr(b"__test_event unsigned short data\0"),
        c_mut_ptr(b"print fmt: \"data=%u\", REC->data\0"),
        &mut self_.check,
    );
    ASSERT_EQ!(0, ret);

    ret = check_print_fmt(
        c_mut_ptr(b"__test_event s8 data\0"),
        c_mut_ptr(b"print fmt: \"data=%d\", REC->data\0"),
        &mut self_.check,
    );
    ASSERT_EQ!(0, ret);

    ret = check_print_fmt(
        c_mut_ptr(b"__test_event u8 data\0"),
        c_mut_ptr(b"print fmt: \"data=%u\", REC->data\0"),
        &mut self_.check,
    );
    ASSERT_EQ!(0, ret);

    ret = check_print_fmt(
        c_mut_ptr(b"__test_event char data\0"),
        c_mut_ptr(b"print fmt: \"data=%d\", REC->data\0"),
        &mut self_.check,
    );
    ASSERT_EQ!(0, ret);

    ret = check_print_fmt(
        c_mut_ptr(b"__test_event unsigned char data\0"),
        c_mut_ptr(b"print fmt: \"data=%u\", REC->data\0"),
        &mut self_.check,
    );
    ASSERT_EQ!(0, ret);

    ret = check_print_fmt(
        c_mut_ptr(b"__test_event char[4] data\0"),
        c_mut_ptr(b"print fmt: \"data=%s\", REC->data\0"),
        &mut self_.check,
    );
    ASSERT_EQ!(0, ret);
}

#[allow(unused_unsafe)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    test_harness_run(argc, argv)
}
