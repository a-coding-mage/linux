// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2026 Christian Brauner <brauner@kernel.org>
/*
 * Test extended attributes on sockfs sockets.
 *
 * Sockets created via socket() have their inodes in sockfs, which supports
 * user.* xattrs with per-inode limits: up to 128 xattrs and 128KB total
 * value size. These tests verify xattr operations via fsetxattr/fgetxattr/
 * flistxattr/fremovexattr on the socket fd, as well as limit enforcement.
 */

use core::ffi::{c_char, c_int, c_void};

const TEST_XATTR_NAME: &[u8] = b"user.testattr\0";
const TEST_XATTR_VALUE: &[u8] = b"testvalue\0";
const TEST_XATTR_VALUE2: &[u8] = b"newvalue\0";

/* Per-inode limits for user.* xattrs on sockfs (from include/linux/xattr.h) */
const SIMPLE_XATTR_MAX_NR: c_int = 128;
const SIMPLE_XATTR_MAX_SIZE: usize = 128 << 10; /* 128 KB */

/* #ifndef XATTR_SIZE_MAX */
const XATTR_SIZE_MAX: usize = 65536;

const AF_UNIX: c_int = 1;
const SOCK_STREAM: c_int = 1;
const XATTR_CREATE: c_int = 1;
const XATTR_REPLACE: c_int = 2;
const ENODATA: c_int = 61;
const EEXIST: c_int = 17;
const ERANGE: c_int = 34;
const ENOSPC: c_int = 28;

type SsizeT = isize;
type SizeT = usize;

unsafe extern "C" {
    static mut errno: c_int;

    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn fsetxattr(
        fd: c_int,
        name: *const c_char,
        value: *const c_void,
        size: SizeT,
        flags: c_int,
    ) -> c_int;
    fn fgetxattr(fd: c_int, name: *const c_char, value: *mut c_void, size: SizeT) -> SsizeT;
    fn flistxattr(fd: c_int, list: *mut c_char, size: SizeT) -> SsizeT;
    fn fremovexattr(fd: c_int, name: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> SizeT;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn snprintf(s: *mut c_char, n: SizeT, format: *const c_char, ...) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: SizeT) -> *mut c_void;
    fn malloc(size: SizeT) -> *mut c_void;
    fn free(ptr: *mut c_void);
}

macro_rules! th_log {
    ($($arg:tt)*) => {{
        let _ = format_args!($($arg)*);
    }};
}

/*
 * Fixture for sockfs socket xattr tests.
 * Creates an AF_UNIX socket (lives in sockfs, not bound to any path).
 */
struct XattrSockfs {
    sockfd: c_int,
}

impl XattrSockfs {
    unsafe fn setup() -> Self {
        let mut fixture = Self { sockfd: -1 };

        fixture.sockfd = socket(AF_UNIX, SOCK_STREAM, 0);
        assert!(fixture.sockfd >= 0, "Failed to create socket");

        fixture
    }

    unsafe fn teardown(&mut self) {
        if self.sockfd >= 0 {
            close(self.sockfd);
        }
    }
}

unsafe fn test_xattr_name() -> *const c_char {
    TEST_XATTR_NAME.as_ptr() as *const c_char
}

unsafe fn test_xattr_value() -> *const c_void {
    TEST_XATTR_VALUE.as_ptr() as *const c_void
}

unsafe fn test_xattr_value2() -> *const c_void {
    TEST_XATTR_VALUE2.as_ptr() as *const c_void
}

unsafe fn with_fixture(test: unsafe fn(&mut XattrSockfs)) {
    let mut fixture = XattrSockfs::setup();
    test(&mut fixture);
    fixture.teardown();
}

unsafe fn set_get_user_xattr(self_: &mut XattrSockfs) {
    let mut buf = [0 as c_char; 256];
    let mut ret: SsizeT;

    ret = fsetxattr(
        self_.sockfd,
        test_xattr_name(),
        test_xattr_value(),
        strlen(TEST_XATTR_VALUE.as_ptr() as *const c_char),
        0,
    ) as SsizeT;
    assert_eq!(ret, 0, "fsetxattr failed");

    memset(buf.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&buf));
    ret = fgetxattr(
        self_.sockfd,
        test_xattr_name(),
        buf.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&buf),
    );
    assert_eq!(
        ret,
        strlen(TEST_XATTR_VALUE.as_ptr() as *const c_char) as SsizeT,
        "fgetxattr returned"
    );
    assert_eq!(strcmp(buf.as_ptr(), TEST_XATTR_VALUE.as_ptr() as *const c_char), 0);
}

/*
 * Test listing xattrs on a sockfs socket.
 * Should include user.* xattrs and system.sockprotoname.
 */
unsafe fn list_user_xattr(self_: &mut XattrSockfs) {
    let mut list = [0 as c_char; 4096];
    let mut ret: SsizeT;
    let mut ptr: *mut c_char;
    let mut found_user = false;
    let mut found_proto = false;

    ret = fsetxattr(
        self_.sockfd,
        test_xattr_name(),
        test_xattr_value(),
        strlen(TEST_XATTR_VALUE.as_ptr() as *const c_char),
        0,
    ) as SsizeT;
    assert_eq!(ret, 0, "fsetxattr failed");

    memset(list.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&list));
    ret = flistxattr(
        self_.sockfd,
        list.as_mut_ptr(),
        core::mem::size_of_val(&list),
    );
    assert!(ret > 0, "flistxattr failed");

    ptr = list.as_mut_ptr();
    while ptr < list.as_mut_ptr().add(ret as usize) {
        if strcmp(ptr, test_xattr_name()) == 0 {
            found_user = true;
        }
        if strcmp(ptr, b"system.sockprotoname\0".as_ptr() as *const c_char) == 0 {
            found_proto = true;
        }
        ptr = ptr.add(strlen(ptr) + 1);
    }
    assert!(found_user, "user xattr not found in list");
    assert!(found_proto, "system.sockprotoname not found in list");
}

unsafe fn remove_user_xattr(self_: &mut XattrSockfs) {
    let mut buf = [0 as c_char; 256];
    let mut ret: SsizeT;

    ret = fsetxattr(
        self_.sockfd,
        test_xattr_name(),
        test_xattr_value(),
        strlen(TEST_XATTR_VALUE.as_ptr() as *const c_char),
        0,
    ) as SsizeT;
    assert_eq!(ret, 0);

    ret = fremovexattr(self_.sockfd, test_xattr_name()) as SsizeT;
    assert_eq!(ret, 0, "fremovexattr failed");

    ret = fgetxattr(
        self_.sockfd,
        test_xattr_name(),
        buf.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&buf),
    );
    assert_eq!(ret, -1);
    assert_eq!(errno, ENODATA);
}

unsafe fn update_user_xattr(self_: &mut XattrSockfs) {
    let mut buf = [0 as c_char; 256];
    let mut ret: SsizeT;

    ret = fsetxattr(
        self_.sockfd,
        test_xattr_name(),
        test_xattr_value(),
        strlen(TEST_XATTR_VALUE.as_ptr() as *const c_char),
        0,
    ) as SsizeT;
    assert_eq!(ret, 0);

    ret = fsetxattr(
        self_.sockfd,
        test_xattr_name(),
        test_xattr_value2(),
        strlen(TEST_XATTR_VALUE2.as_ptr() as *const c_char),
        0,
    ) as SsizeT;
    assert_eq!(ret, 0);

    memset(buf.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&buf));
    ret = fgetxattr(
        self_.sockfd,
        test_xattr_name(),
        buf.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&buf),
    );
    assert_eq!(ret, strlen(TEST_XATTR_VALUE2.as_ptr() as *const c_char) as SsizeT);
    assert_eq!(strcmp(buf.as_ptr(), TEST_XATTR_VALUE2.as_ptr() as *const c_char), 0);
}

unsafe fn xattr_create_flag(self_: &mut XattrSockfs) {
    let mut ret: c_int;

    ret = fsetxattr(
        self_.sockfd,
        test_xattr_name(),
        test_xattr_value(),
        strlen(TEST_XATTR_VALUE.as_ptr() as *const c_char),
        0,
    );
    assert_eq!(ret, 0);

    ret = fsetxattr(
        self_.sockfd,
        test_xattr_name(),
        test_xattr_value2(),
        strlen(TEST_XATTR_VALUE2.as_ptr() as *const c_char),
        XATTR_CREATE,
    );
    assert_eq!(ret, -1);
    assert_eq!(errno, EEXIST);
}

unsafe fn xattr_replace_flag(self_: &mut XattrSockfs) {
    let ret: c_int;

    ret = fsetxattr(
        self_.sockfd,
        test_xattr_name(),
        test_xattr_value(),
        strlen(TEST_XATTR_VALUE.as_ptr() as *const c_char),
        XATTR_REPLACE,
    );
    assert_eq!(ret, -1);
    assert_eq!(errno, ENODATA);
}

unsafe fn get_nonexistent(self_: &mut XattrSockfs) {
    let mut buf = [0 as c_char; 256];
    let ret: SsizeT;

    ret = fgetxattr(
        self_.sockfd,
        b"user.nonexistent\0".as_ptr() as *const c_char,
        buf.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&buf),
    );
    assert_eq!(ret, -1);
    assert_eq!(errno, ENODATA);
}

unsafe fn empty_value(self_: &mut XattrSockfs) {
    let mut ret: SsizeT;

    ret = fsetxattr(
        self_.sockfd,
        test_xattr_name(),
        b"\0".as_ptr() as *const c_void,
        0,
        0,
    ) as SsizeT;
    assert_eq!(ret, 0);

    ret = fgetxattr(self_.sockfd, test_xattr_name(), core::ptr::null_mut(), 0);
    assert_eq!(ret, 0);
}

unsafe fn get_size(self_: &mut XattrSockfs) {
    let mut ret: SsizeT;

    ret = fsetxattr(
        self_.sockfd,
        test_xattr_name(),
        test_xattr_value(),
        strlen(TEST_XATTR_VALUE.as_ptr() as *const c_char),
        0,
    ) as SsizeT;
    assert_eq!(ret, 0);

    ret = fgetxattr(self_.sockfd, test_xattr_name(), core::ptr::null_mut(), 0);
    assert_eq!(ret, strlen(TEST_XATTR_VALUE.as_ptr() as *const c_char) as SsizeT);
}

unsafe fn buffer_too_small(self_: &mut XattrSockfs) {
    let mut buf = [0 as c_char; 2];
    let mut ret: SsizeT;

    ret = fsetxattr(
        self_.sockfd,
        test_xattr_name(),
        test_xattr_value(),
        strlen(TEST_XATTR_VALUE.as_ptr() as *const c_char),
        0,
    ) as SsizeT;
    assert_eq!(ret, 0);

    ret = fgetxattr(
        self_.sockfd,
        test_xattr_name(),
        buf.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&buf),
    );
    assert_eq!(ret, -1);
    assert_eq!(errno, ERANGE);
}

/*
 * Test maximum number of user.* xattrs per socket.
 * The kernel enforces SIMPLE_XATTR_MAX_NR (128), so the 129th should
 * fail with ENOSPC.
 */
unsafe fn max_nr_xattrs(self_: &mut XattrSockfs) {
    let mut name = [0 as c_char; 32];
    let mut i: c_int;
    let mut ret: c_int;

    i = 0;
    while i < SIMPLE_XATTR_MAX_NR {
        snprintf(
            name.as_mut_ptr(),
            core::mem::size_of_val(&name),
            b"user.test%03d\0".as_ptr() as *const c_char,
            i,
        );
        ret = fsetxattr(
            self_.sockfd,
            name.as_ptr(),
            b"v\0".as_ptr() as *const c_void,
            1,
            0,
        );
        assert_eq!(ret, 0, "fsetxattr failed");
        i += 1;
    }

    ret = fsetxattr(
        self_.sockfd,
        b"user.overflow\0".as_ptr() as *const c_char,
        b"v\0".as_ptr() as *const c_void,
        1,
        0,
    );
    assert_eq!(ret, -1);
    assert_eq!(errno, ENOSPC, "Expected ENOSPC for xattr");
}

/*
 * Test maximum total value size for user.* xattrs.
 * The kernel enforces SIMPLE_XATTR_MAX_SIZE (128KB). Individual xattr
 * values are limited to XATTR_SIZE_MAX (64KB) by the VFS, so we need
 * at least two xattrs to hit the total limit.
 */
unsafe fn max_xattr_size(self_: &mut XattrSockfs) {
    let value: *mut c_char;
    let mut ret: c_int;

    value = malloc(XATTR_SIZE_MAX) as *mut c_char;
    assert!(!value.is_null());
    memset(value as *mut c_void, 'A' as c_int, XATTR_SIZE_MAX);

    /* First 64KB xattr - total = 64KB */
    ret = fsetxattr(
        self_.sockfd,
        b"user.big1\0".as_ptr() as *const c_char,
        value as *const c_void,
        XATTR_SIZE_MAX,
        0,
    );
    assert_eq!(ret, 0, "first large xattr failed");

    /* Second 64KB xattr - total = 128KB (exactly at limit) */
    ret = fsetxattr(
        self_.sockfd,
        b"user.big2\0".as_ptr() as *const c_char,
        value as *const c_void,
        XATTR_SIZE_MAX,
        0,
    );
    free(value as *mut c_void);
    assert_eq!(ret, 0, "second large xattr failed");

    /* Third xattr with 1 byte - total > 128KB, should fail */
    ret = fsetxattr(
        self_.sockfd,
        b"user.big3\0".as_ptr() as *const c_char,
        b"v\0".as_ptr() as *const c_void,
        1,
        0,
    );
    assert_eq!(ret, -1);
    assert_eq!(errno, ENOSPC, "Expected ENOSPC when exceeding size limit");
}

/*
 * Test that removing an xattr frees limit space, allowing re-addition.
 */
unsafe fn limit_remove_readd(self_: &mut XattrSockfs) {
    let mut name = [0 as c_char; 32];
    let mut i: c_int;
    let mut ret: c_int;

    /* Fill up to the maximum count */
    i = 0;
    while i < SIMPLE_XATTR_MAX_NR {
        snprintf(
            name.as_mut_ptr(),
            core::mem::size_of_val(&name),
            b"user.test%03d\0".as_ptr() as *const c_char,
            i,
        );
        ret = fsetxattr(
            self_.sockfd,
            name.as_ptr(),
            b"v\0".as_ptr() as *const c_void,
            1,
            0,
        );
        assert_eq!(ret, 0);
        i += 1;
    }

    /* Verify we're at the limit */
    ret = fsetxattr(
        self_.sockfd,
        b"user.overflow\0".as_ptr() as *const c_char,
        b"v\0".as_ptr() as *const c_void,
        1,
        0,
    );
    assert_eq!(ret, -1);
    assert_eq!(errno, ENOSPC);

    /* Remove one xattr */
    ret = fremovexattr(self_.sockfd, b"user.test000\0".as_ptr() as *const c_char);
    assert_eq!(ret, 0);

    /* Now we should be able to add one more */
    ret = fsetxattr(
        self_.sockfd,
        b"user.newattr\0".as_ptr() as *const c_char,
        b"v\0".as_ptr() as *const c_void,
        1,
        0,
    );
    assert_eq!(ret, 0, "re-add after remove failed");
}

/*
 * Test that two different sockets have independent xattr limits.
 */
unsafe fn limits_per_inode(self_: &mut XattrSockfs) {
    let mut buf = [0 as c_char; 256];
    let sock2: c_int;
    let mut ret: SsizeT;

    sock2 = socket(AF_UNIX, SOCK_STREAM, 0);
    assert!(sock2 >= 0);

    /* Set xattr on first socket */
    ret = fsetxattr(
        self_.sockfd,
        test_xattr_name(),
        test_xattr_value(),
        strlen(TEST_XATTR_VALUE.as_ptr() as *const c_char),
        0,
    ) as SsizeT;
    assert_eq!(ret, 0);

    /* First socket's xattr should not be visible on second socket */
    ret = fgetxattr(sock2, test_xattr_name(), core::ptr::null_mut(), 0);
    assert_eq!(ret, -1);
    assert_eq!(errno, ENODATA);

    /* Second socket should independently accept xattrs */
    ret = fsetxattr(
        sock2,
        test_xattr_name(),
        test_xattr_value2(),
        strlen(TEST_XATTR_VALUE2.as_ptr() as *const c_char),
        0,
    ) as SsizeT;
    assert_eq!(ret, 0);

    /* Verify each socket has its own value */
    memset(buf.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&buf));
    ret = fgetxattr(
        self_.sockfd,
        test_xattr_name(),
        buf.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&buf),
    );
    assert_eq!(ret, strlen(TEST_XATTR_VALUE.as_ptr() as *const c_char) as SsizeT);
    assert_eq!(strcmp(buf.as_ptr(), TEST_XATTR_VALUE.as_ptr() as *const c_char), 0);

    memset(buf.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&buf));
    ret = fgetxattr(
        sock2,
        test_xattr_name(),
        buf.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&buf),
    );
    assert_eq!(ret, strlen(TEST_XATTR_VALUE2.as_ptr() as *const c_char) as SsizeT);
    assert_eq!(strcmp(buf.as_ptr(), TEST_XATTR_VALUE2.as_ptr() as *const c_char), 0);

    close(sock2);
}

fn main() {
    unsafe {
        with_fixture(set_get_user_xattr);
        with_fixture(list_user_xattr);
        with_fixture(remove_user_xattr);
        with_fixture(update_user_xattr);
        with_fixture(xattr_create_flag);
        with_fixture(xattr_replace_flag);
        with_fixture(get_nonexistent);
        with_fixture(empty_value);
        with_fixture(get_size);
        with_fixture(buffer_too_small);
        with_fixture(max_nr_xattrs);
        with_fixture(max_xattr_size);
        with_fixture(limit_remove_readd);
        with_fixture(limits_per_inode);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
