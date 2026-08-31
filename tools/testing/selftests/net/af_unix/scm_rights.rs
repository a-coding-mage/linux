// SPDX-License-Identifier: GPL-2.0
/* Copyright Amazon.com Inc. or its affiliates. */
/* _GNU_SOURCE */
/* C dependencies: sched.h, stdio.h, string.h, unistd.h, sys/types.h,
 * sys/socket.h, sys/un.h, kselftest_harness.h.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::{offset_of, size_of, zeroed};
use core::ptr::{null, null_mut};

type size_t = usize;
type ssize_t = isize;
type socklen_t = c_uint;
type sa_family_t = u16;
type FILE = c_void;

const CLONE_NEWNET: c_int = 0x40000000;
const AF_UNIX: c_int = 1;
const SOCK_STREAM: c_int = 1;
const SOCK_DGRAM: c_int = 2;
const SOL_SOCKET: c_int = 1;
const SCM_RIGHTS: c_int = 0x01;
const MSG_OOB: c_int = 0x01;
const SO_PASSRIGHTS: c_int = 72;
const EPERM: c_int = 1;

#[repr(C)]
struct __test_metadata {
    _unused: [u8; 0],
}

#[repr(C)]
struct scm_rights {
    fd: [c_int; 32],
}

#[repr(C)]
struct scm_rights_variant {
    name: [c_char; 32],
    type_: c_int,
    flags: c_int,
    test_listener: bool,
    disabled: bool,
}

#[repr(C)]
struct sockaddr {
    sa_family: sa_family_t,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct sockaddr_un {
    sun_family: sa_family_t,
    sun_path: [c_char; 108],
}

#[repr(C)]
struct iovec {
    iov_base: *mut c_void,
    iov_len: size_t,
}

#[repr(C)]
struct msghdr {
    msg_name: *mut c_void,
    msg_namelen: socklen_t,
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
    fn unshare(flags: c_int) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> ssize_t;
    fn fclose(stream: *mut FILE) -> c_int;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn free(ptr: *mut c_void);
    fn strlen(s: *const c_char) -> size_t;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn listen(sockfd: c_int, backlog: c_int) -> c_int;
    fn setsockopt(
        sockfd: c_int,
        level: c_int,
        optname: c_int,
        optval: *const c_void,
        optlen: socklen_t,
    ) -> c_int;
    fn getsockname(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn connect(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn socketpair(domain: c_int, type_: c_int, protocol: c_int, sv: *mut c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn sendmsg(sockfd: c_int, msg: *const msghdr, flags: c_int) -> ssize_t;
    fn __errno_location() -> *mut c_int;
}

macro_rules! ASSERT_NE {
    ($left:expr, $right:expr) => {
        assert!(($left) != ($right))
    };
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {
        assert!(($left) == ($right))
    };
}

macro_rules! ASSERT_LE {
    ($left:expr, $right:expr) => {
        assert!(($left) <= ($right))
    };
}

macro_rules! ASSERT_GE {
    ($left:expr, $right:expr) => {
        assert!(($left) >= ($right))
    };
}

const fn name32(s: &[u8]) -> [c_char; 32] {
    let mut out = [0 as c_char; 32];
    let mut i = 0;

    while i < s.len() && i < 32 {
        out[i] = s[i] as c_char;
        i += 1;
    }

    out
}

static dgram: scm_rights_variant = scm_rights_variant {
    name: name32(b"UNIX "),
    type_: SOCK_DGRAM,
    flags: 0,
    test_listener: false,
    disabled: false,
};

static dgram_disabled: scm_rights_variant = scm_rights_variant {
    name: name32(b"UNIX "),
    type_: SOCK_DGRAM,
    flags: 0,
    test_listener: false,
    disabled: true,
};

static stream: scm_rights_variant = scm_rights_variant {
    name: name32(b"UNIX-STREAM "),
    type_: SOCK_STREAM,
    flags: 0,
    test_listener: false,
    disabled: false,
};

static stream_disabled: scm_rights_variant = scm_rights_variant {
    name: name32(b"UNIX-STREAM "),
    type_: SOCK_STREAM,
    flags: 0,
    test_listener: false,
    disabled: true,
};

static stream_oob: scm_rights_variant = scm_rights_variant {
    name: name32(b"UNIX-STREAM "),
    type_: SOCK_STREAM,
    flags: MSG_OOB,
    test_listener: false,
    disabled: false,
};

static stream_oob_disabled: scm_rights_variant = scm_rights_variant {
    name: name32(b"UNIX-STREAM "),
    type_: SOCK_STREAM,
    flags: MSG_OOB,
    test_listener: false,
    disabled: true,
};

static stream_listener: scm_rights_variant = scm_rights_variant {
    name: name32(b"UNIX-STREAM "),
    type_: SOCK_STREAM,
    flags: 0,
    test_listener: true,
    disabled: false,
};

static stream_listener_disabled: scm_rights_variant = scm_rights_variant {
    name: name32(b"UNIX-STREAM "),
    type_: SOCK_STREAM,
    flags: 0,
    test_listener: true,
    disabled: true,
};

static stream_listener_oob: scm_rights_variant = scm_rights_variant {
    name: name32(b"UNIX-STREAM "),
    type_: SOCK_STREAM,
    flags: MSG_OOB,
    test_listener: true,
    disabled: false,
};

static stream_listener_oob_disabled: scm_rights_variant = scm_rights_variant {
    name: name32(b"UNIX-STREAM "),
    type_: SOCK_STREAM,
    flags: MSG_OOB,
    test_listener: true,
    disabled: true,
};

unsafe fn CMSG_ALIGN(len: size_t) -> size_t {
    (len + size_of::<size_t>() - 1) & !(size_of::<size_t>() - 1)
}

unsafe fn CMSG_SPACE(len: size_t) -> size_t {
    CMSG_ALIGN(size_of::<cmsghdr>()) + CMSG_ALIGN(len)
}

unsafe fn CMSG_LEN(len: size_t) -> size_t {
    CMSG_ALIGN(size_of::<cmsghdr>()) + len
}

unsafe fn CMSG_FIRSTHDR(mhdr: *const msghdr) -> *mut cmsghdr {
    if (*mhdr).msg_controllen >= size_of::<cmsghdr>() {
        (*mhdr).msg_control as *mut cmsghdr
    } else {
        null_mut()
    }
}

unsafe fn CMSG_DATA(cmsg: *mut cmsghdr) -> *mut c_uchar {
    (cmsg as *mut c_uchar).add(CMSG_ALIGN(size_of::<cmsghdr>()))
}

type c_uchar = u8;

unsafe fn count_sockets(
    _metadata: *mut __test_metadata,
    variant: *const scm_rights_variant,
) -> c_int {
    let mut sockets: c_int = -1;
    let mut ret: c_int;
    let mut line: *mut c_char = null_mut();
    let mut unused: size_t = 0;
    let f: *mut FILE;

    f = fopen(c"/proc/net/protocols".as_ptr(), c"r".as_ptr());
    ASSERT_NE!(null_mut(), f);

    let len = strlen((*variant).name.as_ptr());

    while getline(&mut line, &mut unused, f) != -1 {
        let mut unused2: c_int = 0;

        if strncmp(line, (*variant).name.as_ptr(), len) != 0 {
            continue;
        }

        ret = sscanf(
            line.add(len),
            c"%d %d".as_ptr(),
            &mut unused2 as *mut c_int,
            &mut sockets as *mut c_int,
        );
        ASSERT_EQ!(2, ret);

        break;
    }

    free(line as *mut c_void);

    ret = fclose(f);
    ASSERT_EQ!(0, ret);

    sockets
}

unsafe fn scm_rights_setup(_metadata: *mut __test_metadata, variant: *const scm_rights_variant) {
    let mut ret: c_int;

    ret = unshare(CLONE_NEWNET);
    ASSERT_EQ!(0, ret);

    if (*variant).disabled {
        return;
    }

    ret = count_sockets(_metadata, variant);
    ASSERT_EQ!(0, ret);
}

unsafe fn scm_rights_teardown(_metadata: *mut __test_metadata, variant: *const scm_rights_variant) {
    let mut ret: c_int;

    if (*variant).disabled {
        return;
    }

    sleep(1);

    ret = count_sockets(_metadata, variant);
    ASSERT_EQ!(0, ret);
}

unsafe fn create_listeners(
    _metadata: *mut __test_metadata,
    self_: *mut scm_rights,
    variant: *const scm_rights_variant,
    n: c_int,
) {
    let mut addr: sockaddr_un = zeroed();
    addr.sun_family = AF_UNIX as sa_family_t;
    let mut addrlen: socklen_t;
    let mut i: c_int;
    let mut ret: c_int;

    i = 0;
    while i < n * 2 {
        (*self_).fd[i as usize] = socket(AF_UNIX, SOCK_STREAM, 0);
        ASSERT_LE!(0, (*self_).fd[i as usize]);

        addrlen = size_of::<sa_family_t>() as socklen_t;
        ret = bind(
            (*self_).fd[i as usize],
            &addr as *const sockaddr_un as *const sockaddr,
            addrlen,
        );
        ASSERT_EQ!(0, ret);

        ret = listen((*self_).fd[i as usize], -1);
        ASSERT_EQ!(0, ret);

        if (*variant).disabled {
            let optval: c_int = 0;
            ret = setsockopt(
                (*self_).fd[i as usize],
                SOL_SOCKET,
                SO_PASSRIGHTS,
                &optval as *const c_int as *const c_void,
                size_of::<c_int>() as socklen_t,
            );
            ASSERT_EQ!(0, ret);
        }

        addrlen = size_of::<sockaddr_un>() as socklen_t;
        ret = getsockname(
            (*self_).fd[i as usize],
            &mut addr as *mut sockaddr_un as *mut sockaddr,
            &mut addrlen,
        );
        ASSERT_EQ!(0, ret);

        (*self_).fd[(i + 1) as usize] = socket(AF_UNIX, SOCK_STREAM, 0);
        ASSERT_LE!(0, (*self_).fd[(i + 1) as usize]);

        ret = connect(
            (*self_).fd[(i + 1) as usize],
            &addr as *const sockaddr_un as *const sockaddr,
            addrlen,
        );
        ASSERT_EQ!(0, ret);

        i += 2;
    }
}

unsafe fn create_socketpairs(
    _metadata: *mut __test_metadata,
    self_: *mut scm_rights,
    variant: *const scm_rights_variant,
    n: c_int,
) {
    let mut i: c_int;
    let mut ret: c_int;

    ASSERT_GE!(size_of::<[c_int; 32]>() / size_of::<c_int>(), n as usize);

    i = 0;
    while i < n * 2 {
        ret = socketpair(
            AF_UNIX,
            (*variant).type_,
            0,
            (*self_).fd.as_mut_ptr().add(i as usize),
        );
        ASSERT_EQ!(0, ret);

        if (*variant).disabled {
            let optval: c_int = 0;
            ret = setsockopt(
                (*self_).fd[i as usize],
                SOL_SOCKET,
                SO_PASSRIGHTS,
                &optval as *const c_int as *const c_void,
                size_of::<c_int>() as socklen_t,
            );
            ASSERT_EQ!(0, ret);
        }

        i += 2;
    }
}

unsafe fn __create_sockets(
    _metadata: *mut __test_metadata,
    self_: *mut scm_rights,
    variant: *const scm_rights_variant,
    n: c_int,
) {
    ASSERT_LE!(
        n * 2,
        (size_of::<[c_int; 32]>() / size_of::<c_int>()) as c_int
    );

    if (*variant).test_listener {
        create_listeners(_metadata, self_, variant, n);
    } else {
        create_socketpairs(_metadata, self_, variant, n);
    }
}

unsafe fn __close_sockets(
    _metadata: *mut __test_metadata,
    self_: *mut scm_rights,
    n: c_int,
) {
    let mut i: c_int;
    let mut ret: c_int;

    ASSERT_GE!(size_of::<[c_int; 32]>() / size_of::<c_int>(), n as usize);

    i = 0;
    while i < n * 2 {
        ret = close((*self_).fd[i as usize]);
        ASSERT_EQ!(0, ret);
        i += 1;
    }
}

unsafe fn __send_fd(
    _metadata: *mut __test_metadata,
    self_: *const scm_rights,
    variant: *const scm_rights_variant,
    inflight: c_int,
    receiver: c_int,
) {
    const MSG: &[u8; 2] = b"x\0";
    const MSGLEN: size_t = 1;
    let fds: [c_int; 2] = [
        (*self_).fd[(inflight * 2) as usize],
        (*self_).fd[(inflight * 2) as usize],
    ];
    let mut cmsg_buf = [0u8; CMSG_SPACE(size_of::<[c_int; 2]>())];
    let mut iov = iovec {
        iov_base: MSG.as_ptr() as *mut c_void,
        iov_len: MSGLEN,
    };
    let mut msg: msghdr = zeroed();
    msg.msg_name = null_mut();
    msg.msg_namelen = 0;
    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1;
    msg.msg_control = cmsg_buf.as_mut_ptr() as *mut c_void;
    msg.msg_controllen = size_of_val(&cmsg_buf);

    let cmsg = CMSG_FIRSTHDR(&msg);
    (*cmsg).cmsg_level = SOL_SOCKET;
    (*cmsg).cmsg_type = SCM_RIGHTS;
    (*cmsg).cmsg_len = CMSG_LEN(size_of::<[c_int; 2]>());
    core::ptr::copy_nonoverlapping(
        fds.as_ptr() as *const c_uchar,
        CMSG_DATA(cmsg),
        size_of::<[c_int; 2]>(),
    );

    let ret = sendmsg(
        (*self_).fd[(receiver * 2 + 1) as usize],
        &msg,
        (*variant).flags,
    );

    if (*variant).disabled {
        ASSERT_EQ!(-1, ret);
        ASSERT_EQ!(-EPERM, -*__errno_location());
    } else {
        ASSERT_EQ!(MSGLEN as ssize_t, ret);
    }
}

fn size_of_val<T>(val: &T) -> usize {
    core::mem::size_of_val(val)
}

unsafe fn create_sockets(
    _metadata: *mut __test_metadata,
    self_: *mut scm_rights,
    variant: *const scm_rights_variant,
    n: c_int,
) {
    __create_sockets(_metadata, self_, variant, n);
}

unsafe fn close_sockets(_metadata: *mut __test_metadata, self_: *mut scm_rights, n: c_int) {
    __close_sockets(_metadata, self_, n);
}

unsafe fn send_fd(
    _metadata: *mut __test_metadata,
    self_: *mut scm_rights,
    variant: *const scm_rights_variant,
    inflight: c_int,
    receiver: c_int,
) {
    __send_fd(_metadata, self_, variant, inflight, receiver);
}

unsafe fn self_ref(
    _metadata: *mut __test_metadata,
    self_: *mut scm_rights,
    variant: *const scm_rights_variant,
) {
    create_sockets(_metadata, self_, variant, 2);

    send_fd(_metadata, self_, variant, 0, 0);

    send_fd(_metadata, self_, variant, 1, 1);

    close_sockets(_metadata, self_, 2);
}

unsafe fn triangle(
    _metadata: *mut __test_metadata,
    self_: *mut scm_rights,
    variant: *const scm_rights_variant,
) {
    create_sockets(_metadata, self_, variant, 6);

    send_fd(_metadata, self_, variant, 0, 1);
    send_fd(_metadata, self_, variant, 1, 2);
    send_fd(_metadata, self_, variant, 2, 0);

    send_fd(_metadata, self_, variant, 3, 4);
    send_fd(_metadata, self_, variant, 4, 5);
    send_fd(_metadata, self_, variant, 5, 3);

    close_sockets(_metadata, self_, 6);
}

unsafe fn cross_edge(
    _metadata: *mut __test_metadata,
    self_: *mut scm_rights,
    variant: *const scm_rights_variant,
) {
    create_sockets(_metadata, self_, variant, 8);

    send_fd(_metadata, self_, variant, 0, 1);
    send_fd(_metadata, self_, variant, 1, 2);
    send_fd(_metadata, self_, variant, 2, 0);
    send_fd(_metadata, self_, variant, 1, 3);
    send_fd(_metadata, self_, variant, 3, 2);

    send_fd(_metadata, self_, variant, 4, 5);
    send_fd(_metadata, self_, variant, 5, 6);
    send_fd(_metadata, self_, variant, 6, 4);
    send_fd(_metadata, self_, variant, 5, 7);
    send_fd(_metadata, self_, variant, 7, 6);

    close_sockets(_metadata, self_, 8);
}

unsafe fn backtrack_from_scc(
    _metadata: *mut __test_metadata,
    self_: *mut scm_rights,
    variant: *const scm_rights_variant,
) {
    create_sockets(_metadata, self_, variant, 10);

    send_fd(_metadata, self_, variant, 0, 1);
    send_fd(_metadata, self_, variant, 0, 4);
    send_fd(_metadata, self_, variant, 1, 2);
    send_fd(_metadata, self_, variant, 2, 3);
    send_fd(_metadata, self_, variant, 3, 1);

    send_fd(_metadata, self_, variant, 5, 6);
    send_fd(_metadata, self_, variant, 5, 9);
    send_fd(_metadata, self_, variant, 6, 7);
    send_fd(_metadata, self_, variant, 7, 8);
    send_fd(_metadata, self_, variant, 8, 6);

    close_sockets(_metadata, self_, 10);
}

/* TEST_HARNESS_MAIN */
