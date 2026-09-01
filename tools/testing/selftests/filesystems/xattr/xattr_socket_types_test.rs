// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2026 Christian Brauner <brauner@kernel.org>
/*
 * Test user.* xattrs on various socket families.
 *
 * All socket types use sockfs for their inodes, so user.* xattrs should
 * work on any socket regardless of address family. This tests AF_INET,
 * AF_INET6, AF_NETLINK, AF_PACKET, and abstract namespace AF_UNIX sockets.
 */

// C dependencies: errno.h, stddef.h, stdio.h, stdlib.h, string.h,
// sys/socket.h, sys/types.h, sys/un.h, sys/xattr.h, linux/netlink.h,
// unistd.h, and ../../kselftest_harness.h.

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

type c_char = i8;
type c_int = i32;
type c_uint = u32;
type c_void = core::ffi::c_void;
type size_t = usize;
type ssize_t = isize;
type socklen_t = u32;
type pid_t = i32;

const TEST_XATTR_NAME: &[u8] = b"user.testattr\0";
const TEST_XATTR_VALUE: &[u8] = b"testvalue\0";

const AF_UNIX: c_int = 1;
const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const AF_NETLINK: c_int = 16;
const AF_PACKET: c_int = 17;

const SOCK_STREAM: c_int = 1;
const SOCK_DGRAM: c_int = 2;
const SOCK_RAW: c_int = 3;

const NETLINK_USERSOCK: c_int = 2;

const EACCES: c_int = 13;
const EAFNOSUPPORT: c_int = 97;
const ENODATA: c_int = 61;
const EPERM: c_int = 1;

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct sockaddr_un {
    sun_family: u16,
    sun_path: [c_char; 108],
}

#[repr(C)]
struct xattr_socket_types {
    sockfd: c_int,
}

#[repr(C)]
struct xattr_socket_types_variant {
    family: c_int,
    type_: c_int,
    protocol: c_int,
}

static xattr_socket_types_variant_inet: xattr_socket_types_variant =
    xattr_socket_types_variant {
        family: AF_INET,
        type_: SOCK_STREAM,
        protocol: 0,
    };

static xattr_socket_types_variant_inet6: xattr_socket_types_variant =
    xattr_socket_types_variant {
        family: AF_INET6,
        type_: SOCK_STREAM,
        protocol: 0,
    };

static xattr_socket_types_variant_netlink: xattr_socket_types_variant =
    xattr_socket_types_variant {
        family: AF_NETLINK,
        type_: SOCK_RAW,
        protocol: NETLINK_USERSOCK,
    };

static xattr_socket_types_variant_packet: xattr_socket_types_variant =
    xattr_socket_types_variant {
        family: AF_PACKET,
        type_: SOCK_DGRAM,
        protocol: 0,
    };

unsafe extern "C" {
    static mut errno: c_int;

    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn fgetxattr(fd: c_int, name: *const c_char, value: *mut c_void, size: size_t) -> ssize_t;
    fn flistxattr(fd: c_int, list: *mut c_char, size: size_t) -> ssize_t;
    fn fremovexattr(fd: c_int, name: *const c_char) -> c_int;
    fn fsetxattr(
        fd: c_int,
        name: *const c_char,
        value: *const c_void,
        size: size_t,
        flags: c_int,
    ) -> c_int;
    fn getpid() -> pid_t;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn snprintf(s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
}

unsafe fn assert_eq_ssize(left: ssize_t, right: ssize_t) {
    assert_eq!(left, right);
}

unsafe fn assert_eq_int(left: c_int, right: c_int) {
    assert_eq!(left, right);
}

unsafe fn assert_ge_int(left: c_int, right: c_int) {
    assert!(left >= right);
}

unsafe fn assert_gt_ssize(left: ssize_t, right: ssize_t) {
    assert!(left > right);
}

unsafe fn assert_true(value: bool) {
    assert!(value);
}

unsafe fn assert_streq(left: *const c_char, right: *const c_char) {
    assert_eq!(
        memcmp(
            left as *const c_void,
            right as *const c_void,
            strlen(right) + 1,
        ),
        0,
    );
}

unsafe fn xattr_socket_types_setup(
    self_: *mut xattr_socket_types,
    variant: *const xattr_socket_types_variant,
) {
    (*self_).sockfd = socket(
        (*variant).family,
        (*variant).type_,
        (*variant).protocol,
    );
    if (*self_).sockfd < 0
        && (errno == EAFNOSUPPORT || errno == EPERM || errno == EACCES)
    {
        /*
         * SKIP(return, "socket(%d, %d, %d) not available: %s",
         *      variant->family, variant->type, variant->protocol,
         *      strerror(errno));
         */
        return;
    }
    if (*self_).sockfd < 0 {
        /*
         * TH_LOG("Failed to create socket(%d, %d, %d): %s",
         *        variant->family, variant->type, variant->protocol,
         *        strerror(errno));
         */
        let _ = strerror(errno);
    }
    assert_ge_int((*self_).sockfd, 0);
}

unsafe fn xattr_socket_types_teardown(self_: *mut xattr_socket_types) {
    if (*self_).sockfd >= 0 {
        close((*self_).sockfd);
    }
}

unsafe fn xattr_socket_types_set_get_list_remove(self_: *mut xattr_socket_types) {
    let mut buf: [c_char; 256] = [0; 256];
    let mut list: [c_char; 4096] = [0; 4096];
    let mut ptr: *mut c_char;
    let mut ret: ssize_t;
    let mut found: bool;

    ret = fsetxattr(
        (*self_).sockfd,
        TEST_XATTR_NAME.as_ptr() as *const c_char,
        TEST_XATTR_VALUE.as_ptr() as *const c_void,
        strlen(TEST_XATTR_VALUE.as_ptr() as *const c_char),
        0,
    ) as ssize_t;
    if ret != 0 {
        /*
         * TH_LOG("fsetxattr failed: %s", strerror(errno));
         */
        let _ = strerror(errno);
    }
    assert_eq_ssize(ret, 0);

    memset(buf.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&buf));
    ret = fgetxattr(
        (*self_).sockfd,
        TEST_XATTR_NAME.as_ptr() as *const c_char,
        buf.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&buf),
    );
    assert_eq_ssize(ret, strlen(TEST_XATTR_VALUE.as_ptr() as *const c_char) as ssize_t);
    assert_streq(
        buf.as_ptr(),
        TEST_XATTR_VALUE.as_ptr() as *const c_char,
    );

    memset(list.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&list));
    ret = flistxattr(
        (*self_).sockfd,
        list.as_mut_ptr(),
        core::mem::size_of_val(&list),
    );
    assert_gt_ssize(ret, 0);
    found = false;
    ptr = list.as_mut_ptr();
    while ptr < list.as_mut_ptr().add(ret as usize) {
        if memcmp(
            ptr as *const c_void,
            TEST_XATTR_NAME.as_ptr() as *const c_void,
            strlen(TEST_XATTR_NAME.as_ptr() as *const c_char) + 1,
        ) == 0
        {
            found = true;
        }
        ptr = ptr.add(strlen(ptr) + 1);
    }
    assert_true(found);

    ret = fremovexattr(
        (*self_).sockfd,
        TEST_XATTR_NAME.as_ptr() as *const c_char,
    ) as ssize_t;
    assert_eq_ssize(ret, 0);

    ret = fgetxattr(
        (*self_).sockfd,
        TEST_XATTR_NAME.as_ptr() as *const c_char,
        buf.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&buf),
    );
    assert_eq_ssize(ret, -1);
    assert_eq_int(errno, ENODATA);
}

/*
 * Test abstract namespace AF_UNIX socket.
 * Abstract sockets don't have a filesystem path; their inodes live in
 * sockfs so user.* xattrs should work via fsetxattr/fgetxattr.
 */
#[repr(C)]
struct xattr_abstract {
    sockfd: c_int,
}

unsafe fn xattr_abstract_setup(self_: *mut xattr_abstract) {
    let mut addr: sockaddr_un = core::mem::zeroed();
    let mut name: [c_char; 64] = [0; 64];
    let mut ret: c_int;
    let len: c_int;

    (*self_).sockfd = socket(AF_UNIX, SOCK_STREAM, 0);
    assert_ge_int((*self_).sockfd, 0);

    len = snprintf(
        name.as_mut_ptr(),
        core::mem::size_of_val(&name),
        b"xattr_test_abstract_%d\0".as_ptr() as *const c_char,
        getpid(),
    );

    memset(
        &mut addr as *mut sockaddr_un as *mut c_void,
        0,
        core::mem::size_of::<sockaddr_un>(),
    );
    addr.sun_family = AF_UNIX as u16;
    addr.sun_path[0] = 0;
    memcpy(
        addr.sun_path.as_mut_ptr().add(1) as *mut c_void,
        name.as_ptr() as *const c_void,
        len as size_t,
    );

    ret = bind(
        (*self_).sockfd,
        &addr as *const sockaddr_un as *const sockaddr,
        (core::mem::offset_of!(sockaddr_un, sun_path) + 1 + len as usize) as socklen_t,
    );
    assert_eq_int(ret, 0);
}

unsafe fn xattr_abstract_teardown(self_: *mut xattr_abstract) {
    if (*self_).sockfd >= 0 {
        close((*self_).sockfd);
    }
}

unsafe fn xattr_abstract_set_get(self_: *mut xattr_abstract) {
    let mut buf: [c_char; 256] = [0; 256];
    let mut ret: ssize_t;

    ret = fsetxattr(
        (*self_).sockfd,
        TEST_XATTR_NAME.as_ptr() as *const c_char,
        TEST_XATTR_VALUE.as_ptr() as *const c_void,
        strlen(TEST_XATTR_VALUE.as_ptr() as *const c_char),
        0,
    ) as ssize_t;
    if ret != 0 {
        /*
         * TH_LOG("fsetxattr on abstract socket failed: %s",
         *        strerror(errno));
         */
        let _ = strerror(errno);
    }
    assert_eq_ssize(ret, 0);

    memset(buf.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&buf));
    ret = fgetxattr(
        (*self_).sockfd,
        TEST_XATTR_NAME.as_ptr() as *const c_char,
        buf.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&buf),
    );
    assert_eq_ssize(ret, strlen(TEST_XATTR_VALUE.as_ptr() as *const c_char) as ssize_t);
    assert_streq(
        buf.as_ptr(),
        TEST_XATTR_VALUE.as_ptr() as *const c_char,
    );
}

fn main() {
    // TEST_HARNESS_MAIN
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
