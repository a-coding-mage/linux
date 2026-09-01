// SPDX-License-Identifier: GPL-2.0
/*
 * User Events Dyn Events Test Program
 *
 * Copyright (c) 2021 Beau Belgrave <beaub@linux.microsoft.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_ulong, c_void};

// C dependencies: errno.h, linux/user_events.h, stdio.h, stdlib.h, fcntl.h,
// sys/ioctl.h, sys/stat.h, unistd.h, kselftest_harness.h,
// user_events_selftests.h.

type __u64 = u64;

const O_RDONLY: c_int = 0;
const O_RDWR: c_int = 2;
const O_APPEND: c_int = 0o2000;
const EADDRINUSE: c_int = 98;

extern "C" {
    static mut errno: c_int;

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn strlen(s: *const c_char) -> usize;
    fn usleep(usec: u32) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;

    fn test_harness_run(argc: c_int, argv: *mut *mut c_char) -> c_int;
}

// Provided by linux/user_events.h.
extern "C" {
    static DIAG_IOCSDEL: c_ulong;
    static DIAG_IOCSREG: c_ulong;
    static DIAG_IOCSUNREG: c_ulong;
}

#[repr(C)]
struct user_reg {
    size: u32,
    name_args: __u64,
    status_bit: u32,
    status_addr: __u64,
    enable_bit: u32,
    enable_addr: __u64,
    enable_size: u32,
}

#[repr(C)]
struct user_unreg {
    size: u32,
    disable_bit: u32,
    disable_addr: __u64,
}

static dyn_file: &[u8] = b"/sys/kernel/tracing/dynamic_events\0";
static abi_file: &[u8] = b"/sys/kernel/tracing/user_events_data\0";
static enable_file: &[u8] = b"/sys/kernel/tracing/events/user_events/__test_event/enable\0";

unsafe fn event_delete() -> c_int {
    let fd = open(abi_file.as_ptr() as *const c_char, O_RDWR);
    let ret: c_int;

    if fd < 0 {
        return -1;
    }

    ret = ioctl(fd, DIAG_IOCSDEL, c"__test_event".as_ptr());

    close(fd);

    ret
}

unsafe fn wait_for_delete() -> bool {
    let mut i: c_int;

    i = 0;
    while i < 1000 {
        let fd = open(enable_file.as_ptr() as *const c_char, O_RDONLY);

        if fd == -1 {
            return true;
        }

        close(fd);
        usleep(1000);

        i += 1;
    }

    false
}

unsafe fn reg_event(fd: c_int, check: *mut c_int, bit: c_int, value: *const c_char) -> c_int {
    let mut reg: user_reg = core::mem::zeroed();

    reg.size = core::mem::size_of_val(&reg) as u32;
    reg.name_args = value as __u64;
    reg.enable_bit = bit as u32;
    reg.enable_addr = check as __u64;
    reg.enable_size = core::mem::size_of::<c_int>() as u32;

    if ioctl(fd, DIAG_IOCSREG, &mut reg as *mut user_reg) == -1 {
        return -1;
    }

    0
}

unsafe fn unreg_event(fd: c_int, check: *mut c_int, bit: c_int) -> c_int {
    let mut unreg: user_unreg = core::mem::zeroed();

    unreg.size = core::mem::size_of_val(&unreg) as u32;
    unreg.disable_bit = bit as u32;
    unreg.disable_addr = check as __u64;

    ioctl(fd, DIAG_IOCSUNREG, &mut unreg as *mut user_unreg)
}

unsafe fn parse_dyn(value: *const c_char) -> c_int {
    let fd = open(dyn_file.as_ptr() as *const c_char, O_RDWR | O_APPEND);
    let len = strlen(value);
    let mut ret: c_int;

    if fd == -1 {
        return -1;
    }

    ret = write(fd, value as *const c_void, len) as c_int;

    if ret == len as c_int {
        ret = 0;
    } else {
        ret = -1;
    }

    close(fd);

    if ret == 0 {
        event_delete();
    }

    ret
}

unsafe fn parse_abi(check: *mut c_int, value: *const c_char) -> c_int {
    let fd = open(abi_file.as_ptr() as *const c_char, O_RDWR);
    let ret: c_int;

    if fd == -1 {
        return -1;
    }

    /* Until we have persist flags via dynamic events, use the base name */
    if *value.offset(0) != b'u' as c_char || *value.offset(1) != b':' as c_char {
        close(fd);
        return -1;
    }

    ret = reg_event(fd, check, 31, value.offset(2));

    if ret != -1 {
        if unreg_event(fd, check, 31) == -1 {
            printf(c"WARN: Couldn't unreg event\n".as_ptr());
        }
    }

    close(fd);

    wait_for_delete();

    ret
}

unsafe fn parse(check: *mut c_int, value: *const c_char) -> c_int {
    let abi_ret = parse_abi(check, value);
    let dyn_ret = parse_dyn(value);

    /* Ensure both ABI and DYN parse the same way */
    if dyn_ret != abi_ret {
        return -1;
    }

    dyn_ret
}

unsafe fn check_match(
    check: *mut c_int,
    first: *const c_char,
    second: *const c_char,
    match_: *mut bool,
) -> c_int {
    let fd = open(abi_file.as_ptr() as *const c_char, O_RDWR);
    let mut ret: c_int = -1;

    if fd == -1 {
        return -1;
    }

    if reg_event(fd, check, 31, first) == -1 {
        goto_cleanup(fd, check);
        return ret;
    }

    if reg_event(fd, check, 30, second) == -1 {
        if errno == EADDRINUSE {
            /* Name is in use, with different fields */
            *match_ = false;
            ret = 0;
        }

        goto_cleanup(fd, check);
        return ret;
    }

    *match_ = true;
    ret = 0;

    goto_cleanup(fd, check);

    ret
}

unsafe fn goto_cleanup(fd: c_int, check: *mut c_int) {
    unreg_event(fd, check, 31);
    unreg_event(fd, check, 30);

    close(fd);

    wait_for_delete();
}

macro_rules! TEST_MATCH {
    ($self_:expr, $x:expr, $y:expr) => {{
        let mut match_: bool = false;
        assert_ne!(-1, check_match(&mut (*$self_).check, $x.as_ptr(), $y.as_ptr(), &mut match_));
        assert_eq!(true, match_);
    }};
}

macro_rules! TEST_NMATCH {
    ($self_:expr, $x:expr, $y:expr) => {{
        let mut match_: bool = false;
        assert_ne!(-1, check_match(&mut (*$self_).check, $x.as_ptr(), $y.as_ptr(), &mut match_));
        assert_eq!(false, match_);
    }};
}

macro_rules! TEST_PARSE {
    ($self_:expr, $x:expr) => {
        assert_ne!(-1, parse(&mut (*$self_).check, $x.as_ptr()));
    };
}

macro_rules! TEST_NPARSE {
    ($self_:expr, $x:expr) => {
        assert_eq!(-1, parse(&mut (*$self_).check, $x.as_ptr()));
    };
}

#[repr(C)]
struct user {
    check: c_int,
    umount: bool,
}

unsafe fn user_setup(self_: *mut user) {
    // USER_EVENT_FIXTURE_SETUP(return, self->umount);
    USER_EVENT_FIXTURE_SETUP_return(&mut (*self_).umount);
}

unsafe fn user_teardown(self_: *mut user) {
    // USER_EVENT_FIXTURE_TEARDOWN(self->umount);
    USER_EVENT_FIXTURE_TEARDOWN((*self_).umount);

    wait_for_delete();
}

extern "C" {
    fn USER_EVENT_FIXTURE_SETUP_return(umount: *mut bool);
    fn USER_EVENT_FIXTURE_TEARDOWN(umount: bool);
}

unsafe fn user_basic_types(self_: *mut user) {
    /* All should work */
    TEST_PARSE!(self_, c"u:__test_event u64 a");
    TEST_PARSE!(self_, c"u:__test_event u32 a");
    TEST_PARSE!(self_, c"u:__test_event u16 a");
    TEST_PARSE!(self_, c"u:__test_event u8 a");
    TEST_PARSE!(self_, c"u:__test_event char a");
    TEST_PARSE!(self_, c"u:__test_event unsigned char a");
    TEST_PARSE!(self_, c"u:__test_event int a");
    TEST_PARSE!(self_, c"u:__test_event unsigned int a");
    TEST_PARSE!(self_, c"u:__test_event short a");
    TEST_PARSE!(self_, c"u:__test_event unsigned short a");
    TEST_PARSE!(self_, c"u:__test_event char[20] a");
    TEST_PARSE!(self_, c"u:__test_event unsigned char[20] a");
    TEST_PARSE!(self_, c"u:__test_event char[0x14] a");
    TEST_PARSE!(self_, c"u:__test_event unsigned char[0x14] a");
    /* Bad size format should fail */
    TEST_NPARSE!(self_, c"u:__test_event char[aa] a");
    /* Large size should fail */
    TEST_NPARSE!(self_, c"u:__test_event char[9999] a");
    /* Long size string should fail */
    TEST_NPARSE!(self_, c"u:__test_event char[0x0000000000001] a");
}

unsafe fn user_loc_types(self_: *mut user) {
    /* All should work */
    TEST_PARSE!(self_, c"u:__test_event __data_loc char[] a");
    TEST_PARSE!(self_, c"u:__test_event __data_loc unsigned char[] a");
    TEST_PARSE!(self_, c"u:__test_event __rel_loc char[] a");
    TEST_PARSE!(self_, c"u:__test_event __rel_loc unsigned char[] a");
}

unsafe fn user_size_types(self_: *mut user) {
    /* Should work */
    TEST_PARSE!(self_, c"u:__test_event struct custom a 20");
    /* Size not specified on struct should fail */
    TEST_NPARSE!(self_, c"u:__test_event struct custom a");
    /* Size specified on non-struct should fail */
    TEST_NPARSE!(self_, c"u:__test_event char a 20");
}

unsafe fn user_matching(self_: *mut user) {
    /* Single name matches */
    TEST_MATCH!(
        self_,
        c"__test_event u32 a",
        c"__test_event u32 a"
    );

    /* Multiple names match */
    TEST_MATCH!(
        self_,
        c"__test_event u32 a; u32 b",
        c"__test_event u32 a; u32 b"
    );

    /* Multiple names match with dangling ; */
    TEST_MATCH!(
        self_,
        c"__test_event u32 a; u32 b",
        c"__test_event u32 a; u32 b;"
    );

    /* Single name doesn't match */
    TEST_NMATCH!(
        self_,
        c"__test_event u32 a",
        c"__test_event u32 b"
    );

    /* Multiple names don't match */
    TEST_NMATCH!(
        self_,
        c"__test_event u32 a; u32 b",
        c"__test_event u32 b; u32 a"
    );

    /* Types don't match */
    TEST_NMATCH!(
        self_,
        c"__test_event u64 a; u64 b",
        c"__test_event u32 a; u32 b"
    );

    /* Struct name and size matches */
    TEST_MATCH!(
        self_,
        c"__test_event struct my_struct a 20",
        c"__test_event struct my_struct a 20"
    );

    /* Struct name don't match */
    TEST_NMATCH!(
        self_,
        c"__test_event struct my_struct a 20",
        c"__test_event struct my_struct b 20"
    );

    /* Struct size don't match */
    TEST_NMATCH!(
        self_,
        c"__test_event struct my_struct a 20",
        c"__test_event struct my_struct a 21"
    );
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    test_harness_run(argc, argv)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
