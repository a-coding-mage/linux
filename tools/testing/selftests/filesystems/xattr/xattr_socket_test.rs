// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2026 Christian Brauner <brauner@kernel.org>
/*
 * Test extended attributes on path-based Unix domain sockets.
 *
 * Path-based Unix domain sockets are bound to a filesystem path and their
 * inodes live on the underlying filesystem (e.g. tmpfs). These tests verify
 * that user.* and trusted.* xattr operations work correctly on them using
 * path-based syscalls (setxattr, getxattr, etc.).
 *
 * Covers SOCK_STREAM, SOCK_DGRAM, and SOCK_SEQPACKET socket types.
 */

// C source dependencies: errno.h, limits.h, stdio.h, stdlib.h, string.h,
// sys/socket.h, sys/stat.h, sys/types.h, sys/un.h, sys/xattr.h, unistd.h,
// and ../../kselftest_harness.h.

use libc::{
    bind, c_char, c_int, c_void, close, getpid, lgetxattr, listxattr, lsetxattr, memcmp, memset,
    removexattr, setxattr, sockaddr, sockaddr_un, socket, ssize_t, strerror, strncpy, unlink,
    AF_UNIX, EEXIST, ENODATA, EPERM, ERANGE, SOCK_DGRAM, SOCK_SEQPACKET, SOCK_STREAM,
    XATTR_CREATE, XATTR_REPLACE,
};
use std::ffi::{CStr, CString};
use std::mem;
use std::ptr;

const PATH_MAX: usize = 4096;
const TEST_XATTR_NAME: &str = "user.testattr";
const TEST_XATTR_VALUE: &str = "testvalue";
const TEST_XATTR_VALUE2: &str = "newvalue";

fn errno() -> c_int {
    unsafe { *libc::__errno_location() }
}

unsafe fn errno_string() -> String {
    CStr::from_ptr(strerror(errno()))
        .to_string_lossy()
        .into_owned()
}

fn cstring(s: &str) -> CString {
    CString::new(s).unwrap()
}

unsafe fn c_strlen(ptr: *const c_char) -> usize {
    libc::strlen(ptr)
}

/*
 * Fixture for path-based Unix domain socket tests.
 * Creates a SOCK_STREAM socket bound to a path in /tmp (typically tmpfs).
 */
#[repr(C)]
struct XattrSocket {
    socket_path: [c_char; PATH_MAX],
    sockfd: c_int,
}

#[repr(C)]
struct XattrSocketVariant {
    sock_type: c_int,
    name: &'static str,
}

static XATTR_SOCKET_VARIANTS: [XattrSocketVariant; 3] = [
    XattrSocketVariant {
        sock_type: SOCK_STREAM,
        name: "stream",
    },
    XattrSocketVariant {
        sock_type: SOCK_DGRAM,
        name: "dgram",
    },
    XattrSocketVariant {
        sock_type: SOCK_SEQPACKET,
        name: "seqpacket",
    },
];

unsafe fn xattr_socket_setup(self_: &mut XattrSocket, variant: &XattrSocketVariant) {
    let mut addr: sockaddr_un = mem::zeroed();
    let mut ret: c_int;

    self_.sockfd = -1;

    let path = format!("/tmp/xattr_socket_test_{}.{}", variant.name, getpid());
    let path_c = cstring(&path);
    memset(
        self_.socket_path.as_mut_ptr() as *mut c_void,
        0,
        self_.socket_path.len(),
    );
    strncpy(
        self_.socket_path.as_mut_ptr(),
        path_c.as_ptr(),
        self_.socket_path.len() - 1,
    );
    unlink(self_.socket_path.as_ptr());

    self_.sockfd = socket(AF_UNIX, variant.sock_type, 0);
    assert!(
        self_.sockfd >= 0,
        "Failed to create socket: {}",
        errno_string()
    );

    memset(
        &mut addr as *mut sockaddr_un as *mut c_void,
        0,
        mem::size_of::<sockaddr_un>(),
    );
    addr.sun_family = AF_UNIX as libc::sa_family_t;
    strncpy(
        addr.sun_path.as_mut_ptr(),
        self_.socket_path.as_ptr(),
        addr.sun_path.len() - 1,
    );

    ret = bind(
        self_.sockfd,
        &addr as *const sockaddr_un as *const sockaddr,
        mem::size_of::<sockaddr_un>() as libc::socklen_t,
    );
    assert_eq!(
        ret,
        0,
        "Failed to bind socket to {}: {}",
        CStr::from_ptr(self_.socket_path.as_ptr()).to_string_lossy(),
        errno_string()
    );
}

unsafe fn xattr_socket_teardown(self_: &mut XattrSocket) {
    if self_.sockfd >= 0 {
        close(self_.sockfd);
    }
    unlink(self_.socket_path.as_ptr());
}

unsafe fn with_xattr_socket<F>(variant: &XattrSocketVariant, f: F)
where
    F: FnOnce(&mut XattrSocket),
{
    let mut fixture = XattrSocket {
        socket_path: [0; PATH_MAX],
        sockfd: -1,
    };
    xattr_socket_setup(&mut fixture, variant);
    f(&mut fixture);
    xattr_socket_teardown(&mut fixture);
}

macro_rules! test_all_xattr_socket_variants {
    ($body:expr) => {
        unsafe {
            for variant in &XATTR_SOCKET_VARIANTS {
                with_xattr_socket(variant, $body);
            }
        }
    };
}

#[test]
fn set_user_xattr() {
    test_all_xattr_socket_variants!(|self_: &mut XattrSocket| unsafe {
        let ret: c_int;
        let name = cstring(TEST_XATTR_NAME);
        let value = cstring(TEST_XATTR_VALUE);

        ret = setxattr(
            self_.socket_path.as_ptr(),
            name.as_ptr(),
            value.as_ptr() as *const c_void,
            TEST_XATTR_VALUE.len(),
            0,
        );
        assert_eq!(
            ret,
            0,
            "setxattr failed: {} (errno={})",
            errno_string(),
            errno()
        );
    });
}

#[test]
fn get_user_xattr() {
    test_all_xattr_socket_variants!(|self_: &mut XattrSocket| unsafe {
        let mut buf: [c_char; 256] = [0; 256];
        let mut ret: ssize_t;
        let name = cstring(TEST_XATTR_NAME);
        let value = cstring(TEST_XATTR_VALUE);

        ret = setxattr(
            self_.socket_path.as_ptr(),
            name.as_ptr(),
            value.as_ptr() as *const c_void,
            TEST_XATTR_VALUE.len(),
            0,
        );
        assert_eq!(ret, 0, "setxattr failed: {}", errno_string());

        memset(buf.as_mut_ptr() as *mut c_void, 0, buf.len());
        ret = libc::getxattr(
            self_.socket_path.as_ptr(),
            name.as_ptr(),
            buf.as_mut_ptr() as *mut c_void,
            buf.len(),
        );
        assert_eq!(
            ret,
            TEST_XATTR_VALUE.len() as ssize_t,
            "getxattr returned {}, expected {}: {}",
            ret,
            TEST_XATTR_VALUE.len(),
            errno_string()
        );
        assert_eq!(CStr::from_ptr(buf.as_ptr()).to_bytes(), TEST_XATTR_VALUE.as_bytes());
    });
}

#[test]
fn list_user_xattr() {
    test_all_xattr_socket_variants!(|self_: &mut XattrSocket| unsafe {
        let mut list: [c_char; 1024] = [0; 1024];
        let mut ret: ssize_t;
        let mut found = false;
        let mut ptr_: *mut c_char;
        let name = cstring(TEST_XATTR_NAME);
        let value = cstring(TEST_XATTR_VALUE);

        ret = setxattr(
            self_.socket_path.as_ptr(),
            name.as_ptr(),
            value.as_ptr() as *const c_void,
            TEST_XATTR_VALUE.len(),
            0,
        );
        assert_eq!(ret, 0, "setxattr failed: {}", errno_string());

        memset(list.as_mut_ptr() as *mut c_void, 0, list.len());
        ret = listxattr(self_.socket_path.as_ptr(), list.as_mut_ptr(), list.len());
        assert!(ret > 0, "listxattr failed: {}", errno_string());

        ptr_ = list.as_mut_ptr();
        while ptr_ < list.as_mut_ptr().add(ret as usize) {
            if libc::strcmp(ptr_, name.as_ptr()) == 0 {
                found = true;
                break;
            }
            ptr_ = ptr_.add(c_strlen(ptr_) + 1);
        }
        assert!(found, "xattr {} not found in list", TEST_XATTR_NAME);
    });
}

#[test]
fn remove_user_xattr() {
    test_all_xattr_socket_variants!(|self_: &mut XattrSocket| unsafe {
        let mut buf: [c_char; 256] = [0; 256];
        let mut ret: ssize_t;
        let name = cstring(TEST_XATTR_NAME);
        let value = cstring(TEST_XATTR_VALUE);

        ret = setxattr(
            self_.socket_path.as_ptr(),
            name.as_ptr(),
            value.as_ptr() as *const c_void,
            TEST_XATTR_VALUE.len(),
            0,
        );
        assert_eq!(ret, 0, "setxattr failed: {}", errno_string());

        ret = removexattr(self_.socket_path.as_ptr(), name.as_ptr()) as ssize_t;
        assert_eq!(ret, 0, "removexattr failed: {}", errno_string());

        ret = libc::getxattr(
            self_.socket_path.as_ptr(),
            name.as_ptr(),
            buf.as_mut_ptr() as *mut c_void,
            buf.len(),
        );
        assert_eq!(ret, -1);
        assert_eq!(errno(), ENODATA, "Expected ENODATA, got {}", errno_string());
    });
}

/*
 * Test that xattrs persist across socket close and reopen.
 * The xattr is on the filesystem inode, not the socket fd.
 */
#[test]
fn xattr_persistence() {
    test_all_xattr_socket_variants!(|self_: &mut XattrSocket| unsafe {
        let mut buf: [c_char; 256] = [0; 256];
        let mut ret: ssize_t;
        let name = cstring(TEST_XATTR_NAME);
        let value = cstring(TEST_XATTR_VALUE);

        ret = setxattr(
            self_.socket_path.as_ptr(),
            name.as_ptr(),
            value.as_ptr() as *const c_void,
            TEST_XATTR_VALUE.len(),
            0,
        );
        assert_eq!(ret, 0, "setxattr failed: {}", errno_string());

        close(self_.sockfd);
        self_.sockfd = -1;

        memset(buf.as_mut_ptr() as *mut c_void, 0, buf.len());
        ret = libc::getxattr(
            self_.socket_path.as_ptr(),
            name.as_ptr(),
            buf.as_mut_ptr() as *mut c_void,
            buf.len(),
        );
        assert_eq!(
            ret,
            TEST_XATTR_VALUE.len() as ssize_t,
            "getxattr after close failed: {}",
            errno_string()
        );
        assert_eq!(CStr::from_ptr(buf.as_ptr()).to_bytes(), TEST_XATTR_VALUE.as_bytes());
    });
}

#[test]
fn update_user_xattr() {
    test_all_xattr_socket_variants!(|self_: &mut XattrSocket| unsafe {
        let mut buf: [c_char; 256] = [0; 256];
        let mut ret: ssize_t;
        let name = cstring(TEST_XATTR_NAME);
        let value = cstring(TEST_XATTR_VALUE);
        let value2 = cstring(TEST_XATTR_VALUE2);

        ret = setxattr(
            self_.socket_path.as_ptr(),
            name.as_ptr(),
            value.as_ptr() as *const c_void,
            TEST_XATTR_VALUE.len(),
            0,
        );
        assert_eq!(ret, 0);

        ret = setxattr(
            self_.socket_path.as_ptr(),
            name.as_ptr(),
            value2.as_ptr() as *const c_void,
            TEST_XATTR_VALUE2.len(),
            0,
        );
        assert_eq!(ret, 0);

        memset(buf.as_mut_ptr() as *mut c_void, 0, buf.len());
        ret = libc::getxattr(
            self_.socket_path.as_ptr(),
            name.as_ptr(),
            buf.as_mut_ptr() as *mut c_void,
            buf.len(),
        );
        assert_eq!(ret, TEST_XATTR_VALUE2.len() as ssize_t);
        assert_eq!(CStr::from_ptr(buf.as_ptr()).to_bytes(), TEST_XATTR_VALUE2.as_bytes());
    });
}

#[test]
fn xattr_create_flag() {
    test_all_xattr_socket_variants!(|self_: &mut XattrSocket| unsafe {
        let mut ret: c_int;
        let name = cstring(TEST_XATTR_NAME);
        let value = cstring(TEST_XATTR_VALUE);
        let value2 = cstring(TEST_XATTR_VALUE2);

        ret = setxattr(
            self_.socket_path.as_ptr(),
            name.as_ptr(),
            value.as_ptr() as *const c_void,
            TEST_XATTR_VALUE.len(),
            0,
        );
        assert_eq!(ret, 0);

        ret = setxattr(
            self_.socket_path.as_ptr(),
            name.as_ptr(),
            value2.as_ptr() as *const c_void,
            TEST_XATTR_VALUE2.len(),
            XATTR_CREATE,
        );
        assert_eq!(ret, -1);
        assert_eq!(errno(), EEXIST);
    });
}

#[test]
fn xattr_replace_flag() {
    test_all_xattr_socket_variants!(|self_: &mut XattrSocket| unsafe {
        let ret: c_int;
        let name = cstring(TEST_XATTR_NAME);
        let value = cstring(TEST_XATTR_VALUE);

        ret = setxattr(
            self_.socket_path.as_ptr(),
            name.as_ptr(),
            value.as_ptr() as *const c_void,
            TEST_XATTR_VALUE.len(),
            XATTR_REPLACE,
        );
        assert_eq!(ret, -1);
        assert_eq!(errno(), ENODATA);
    });
}

#[test]
fn multiple_xattrs() {
    test_all_xattr_socket_variants!(|self_: &mut XattrSocket| unsafe {
        let mut buf: [c_char; 256] = [0; 256];
        let mut ret: ssize_t;
        let mut i: c_int;
        let num_xattrs: c_int = 5;

        i = 0;
        while i < num_xattrs {
            let name_s = format!("user.test{}", i);
            let value_s = format!("value{}", i);
            let name = cstring(&name_s);
            let value = cstring(&value_s);
            ret = setxattr(
                self_.socket_path.as_ptr(),
                name.as_ptr(),
                value.as_ptr() as *const c_void,
                value_s.len(),
                0,
            ) as ssize_t;
            assert_eq!(
                ret,
                0,
                "setxattr {} failed: {}",
                name_s,
                errno_string()
            );
            i += 1;
        }

        i = 0;
        while i < num_xattrs {
            let name_s = format!("user.test{}", i);
            let value_s = format!("value{}", i);
            let name = cstring(&name_s);
            memset(buf.as_mut_ptr() as *mut c_void, 0, buf.len());
            ret = libc::getxattr(
                self_.socket_path.as_ptr(),
                name.as_ptr(),
                buf.as_mut_ptr() as *mut c_void,
                buf.len(),
            );
            assert_eq!(ret, value_s.len() as ssize_t);
            assert_eq!(CStr::from_ptr(buf.as_ptr()).to_bytes(), value_s.as_bytes());
            i += 1;
        }
    });
}

#[test]
fn xattr_empty_value() {
    test_all_xattr_socket_variants!(|self_: &mut XattrSocket| unsafe {
        let mut buf: [c_char; 256] = [0; 256];
        let mut ret: ssize_t;
        let name = cstring(TEST_XATTR_NAME);
        let empty = cstring("");

        ret = setxattr(
            self_.socket_path.as_ptr(),
            name.as_ptr(),
            empty.as_ptr() as *const c_void,
            0,
            0,
        ) as ssize_t;
        assert_eq!(ret, 0);

        ret = libc::getxattr(
            self_.socket_path.as_ptr(),
            name.as_ptr(),
            buf.as_mut_ptr() as *mut c_void,
            buf.len(),
        );
        assert_eq!(ret, 0);
    });
}

#[test]
fn xattr_get_size() {
    test_all_xattr_socket_variants!(|self_: &mut XattrSocket| unsafe {
        let mut ret: ssize_t;
        let name = cstring(TEST_XATTR_NAME);
        let value = cstring(TEST_XATTR_VALUE);

        ret = setxattr(
            self_.socket_path.as_ptr(),
            name.as_ptr(),
            value.as_ptr() as *const c_void,
            TEST_XATTR_VALUE.len(),
            0,
        ) as ssize_t;
        assert_eq!(ret, 0);

        ret = libc::getxattr(self_.socket_path.as_ptr(), name.as_ptr(), ptr::null_mut(), 0);
        assert_eq!(ret, TEST_XATTR_VALUE.len() as ssize_t);
    });
}

#[test]
fn xattr_buffer_too_small() {
    test_all_xattr_socket_variants!(|self_: &mut XattrSocket| unsafe {
        let mut buf: [c_char; 2] = [0; 2];
        let mut ret: ssize_t;
        let name = cstring(TEST_XATTR_NAME);
        let value = cstring(TEST_XATTR_VALUE);

        ret = setxattr(
            self_.socket_path.as_ptr(),
            name.as_ptr(),
            value.as_ptr() as *const c_void,
            TEST_XATTR_VALUE.len(),
            0,
        ) as ssize_t;
        assert_eq!(ret, 0);

        ret = libc::getxattr(
            self_.socket_path.as_ptr(),
            name.as_ptr(),
            buf.as_mut_ptr() as *mut c_void,
            buf.len(),
        );
        assert_eq!(ret, -1);
        assert_eq!(errno(), ERANGE);
    });
}

#[test]
fn xattr_nonexistent() {
    test_all_xattr_socket_variants!(|self_: &mut XattrSocket| unsafe {
        let mut buf: [c_char; 256] = [0; 256];
        let ret: ssize_t;
        let name = cstring("user.nonexistent");

        ret = libc::getxattr(
            self_.socket_path.as_ptr(),
            name.as_ptr(),
            buf.as_mut_ptr() as *mut c_void,
            buf.len(),
        );
        assert_eq!(ret, -1);
        assert_eq!(errno(), ENODATA);
    });
}

#[test]
fn remove_nonexistent_xattr() {
    test_all_xattr_socket_variants!(|self_: &mut XattrSocket| unsafe {
        let ret: c_int;
        let name = cstring("user.nonexistent");

        ret = removexattr(self_.socket_path.as_ptr(), name.as_ptr());
        assert_eq!(ret, -1);
        assert_eq!(errno(), ENODATA);
    });
}

#[test]
fn large_xattr_value() {
    test_all_xattr_socket_variants!(|self_: &mut XattrSocket| unsafe {
        let mut large_value: [c_char; 4096] = [0; 4096];
        let mut read_buf: [c_char; 4096] = [0; 4096];
        let mut ret: ssize_t;
        let name = cstring(TEST_XATTR_NAME);

        memset(
            large_value.as_mut_ptr() as *mut c_void,
            'A' as c_int,
            large_value.len(),
        );

        ret = setxattr(
            self_.socket_path.as_ptr(),
            name.as_ptr(),
            large_value.as_ptr() as *const c_void,
            large_value.len(),
            0,
        ) as ssize_t;
        assert_eq!(
            ret,
            0,
            "setxattr with large value failed: {}",
            errno_string()
        );

        memset(read_buf.as_mut_ptr() as *mut c_void, 0, read_buf.len());
        ret = libc::getxattr(
            self_.socket_path.as_ptr(),
            name.as_ptr(),
            read_buf.as_mut_ptr() as *mut c_void,
            read_buf.len(),
        );
        assert_eq!(ret, large_value.len() as ssize_t);
        assert_eq!(
            memcmp(
                large_value.as_ptr() as *const c_void,
                read_buf.as_ptr() as *const c_void,
                large_value.len()
            ),
            0
        );
    });
}

/*
 * Test lsetxattr/lgetxattr (don't follow symlinks).
 * Socket files aren't symlinks, so this should work the same.
 */
#[test]
fn lsetxattr_lgetxattr() {
    test_all_xattr_socket_variants!(|self_: &mut XattrSocket| unsafe {
        let mut buf: [c_char; 256] = [0; 256];
        let mut ret: ssize_t;
        let name = cstring(TEST_XATTR_NAME);
        let value = cstring(TEST_XATTR_VALUE);

        ret = lsetxattr(
            self_.socket_path.as_ptr(),
            name.as_ptr(),
            value.as_ptr() as *const c_void,
            TEST_XATTR_VALUE.len(),
            0,
        ) as ssize_t;
        assert_eq!(ret, 0, "lsetxattr failed: {}", errno_string());

        memset(buf.as_mut_ptr() as *mut c_void, 0, buf.len());
        ret = lgetxattr(
            self_.socket_path.as_ptr(),
            name.as_ptr(),
            buf.as_mut_ptr() as *mut c_void,
            buf.len(),
        );
        assert_eq!(ret, TEST_XATTR_VALUE.len() as ssize_t);
        assert_eq!(CStr::from_ptr(buf.as_ptr()).to_bytes(), TEST_XATTR_VALUE.as_bytes());
    });
}

/*
 * Fixture for trusted.* xattr tests.
 * These require CAP_SYS_ADMIN.
 */
#[repr(C)]
struct XattrSocketTrusted {
    socket_path: [c_char; PATH_MAX],
    sockfd: c_int,
}

#[repr(C)]
struct XattrSocketTrustedVariant {
    sock_type: c_int,
    name: &'static str,
}

static XATTR_SOCKET_TRUSTED_VARIANTS: [XattrSocketTrustedVariant; 3] = [
    XattrSocketTrustedVariant {
        sock_type: SOCK_STREAM,
        name: "stream",
    },
    XattrSocketTrustedVariant {
        sock_type: SOCK_DGRAM,
        name: "dgram",
    },
    XattrSocketTrustedVariant {
        sock_type: SOCK_SEQPACKET,
        name: "seqpacket",
    },
];

unsafe fn xattr_socket_trusted_setup(
    self_: &mut XattrSocketTrusted,
    variant: &XattrSocketTrustedVariant,
) {
    let mut addr: sockaddr_un = mem::zeroed();
    let mut ret: c_int;

    self_.sockfd = -1;

    let path = format!("/tmp/xattr_socket_trusted_{}.{}", variant.name, getpid());
    let path_c = cstring(&path);
    memset(
        self_.socket_path.as_mut_ptr() as *mut c_void,
        0,
        self_.socket_path.len(),
    );
    strncpy(
        self_.socket_path.as_mut_ptr(),
        path_c.as_ptr(),
        self_.socket_path.len() - 1,
    );
    unlink(self_.socket_path.as_ptr());

    self_.sockfd = socket(AF_UNIX, variant.sock_type, 0);
    assert!(self_.sockfd >= 0);

    memset(
        &mut addr as *mut sockaddr_un as *mut c_void,
        0,
        mem::size_of::<sockaddr_un>(),
    );
    addr.sun_family = AF_UNIX as libc::sa_family_t;
    strncpy(
        addr.sun_path.as_mut_ptr(),
        self_.socket_path.as_ptr(),
        addr.sun_path.len() - 1,
    );

    ret = bind(
        self_.sockfd,
        &addr as *const sockaddr_un as *const sockaddr,
        mem::size_of::<sockaddr_un>() as libc::socklen_t,
    );
    assert_eq!(ret, 0);
}

unsafe fn xattr_socket_trusted_teardown(self_: &mut XattrSocketTrusted) {
    if self_.sockfd >= 0 {
        close(self_.sockfd);
    }
    unlink(self_.socket_path.as_ptr());
}

unsafe fn with_xattr_socket_trusted<F>(variant: &XattrSocketTrustedVariant, f: F)
where
    F: FnOnce(&mut XattrSocketTrusted),
{
    let mut fixture = XattrSocketTrusted {
        socket_path: [0; PATH_MAX],
        sockfd: -1,
    };
    xattr_socket_trusted_setup(&mut fixture, variant);
    f(&mut fixture);
    xattr_socket_trusted_teardown(&mut fixture);
}

macro_rules! test_all_xattr_socket_trusted_variants {
    ($body:expr) => {
        unsafe {
            for variant in &XATTR_SOCKET_TRUSTED_VARIANTS {
                with_xattr_socket_trusted(variant, $body);
            }
        }
    };
}

#[test]
fn set_trusted_xattr() {
    test_all_xattr_socket_trusted_variants!(|self_: &mut XattrSocketTrusted| unsafe {
        let mut buf: [c_char; 256] = [0; 256];
        let mut len: ssize_t;
        let mut ret: c_int;
        let name = cstring("trusted.testattr");
        let value = cstring(TEST_XATTR_VALUE);

        ret = setxattr(
            self_.socket_path.as_ptr(),
            name.as_ptr(),
            value.as_ptr() as *const c_void,
            TEST_XATTR_VALUE.len(),
            0,
        );
        if ret == -1 && errno() == EPERM {
            eprintln!("Need CAP_SYS_ADMIN for trusted.* xattrs");
            return;
        }
        assert_eq!(
            ret,
            0,
            "setxattr trusted.testattr failed: {}",
            errno_string()
        );

        memset(buf.as_mut_ptr() as *mut c_void, 0, buf.len());
        len = libc::getxattr(
            self_.socket_path.as_ptr(),
            name.as_ptr(),
            buf.as_mut_ptr() as *mut c_void,
            buf.len(),
        );
        assert_eq!(len, TEST_XATTR_VALUE.len() as ssize_t);
        assert_eq!(CStr::from_ptr(buf.as_ptr()).to_bytes(), TEST_XATTR_VALUE.as_bytes());
    });
}

#[test]
fn get_trusted_xattr_unprivileged() {
    test_all_xattr_socket_trusted_variants!(|self_: &mut XattrSocketTrusted| unsafe {
        let mut buf: [c_char; 256] = [0; 256];
        let ret: ssize_t;
        let name = cstring("trusted.testattr");

        ret = libc::getxattr(
            self_.socket_path.as_ptr(),
            name.as_ptr(),
            buf.as_mut_ptr() as *mut c_void,
            buf.len(),
        );
        assert_eq!(ret, -1);
        assert!(
            errno() == ENODATA || errno() == EPERM,
            "Expected ENODATA or EPERM, got {}",
            errno_string()
        );
    });
}

// TEST_HARNESS_MAIN
