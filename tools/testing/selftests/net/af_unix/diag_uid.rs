// SPDX-License-Identifier: GPL-2.0
/* Copyright Amazon.com Inc. or its affiliates. */

// Translated from C implementation source. C include dependencies:
// sched.h, unistd.h, linux/netlink.h, linux/rtnetlink.h, linux/sock_diag.h,
// linux/unix_diag.h, sys/socket.h, sys/stat.h, sys/types.h, sys/un.h,
// and "kselftest_harness.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::c_void;

type __u32 = u32;
type __u64 = u64;
type uid_t = u32;
type socklen_t = u32;
type size_t = usize;
type ssize_t = isize;
type c_int = i32;
type c_uint = u32;
type c_ulong = u64;
type sa_family_t = u16;

const _GNU_SOURCE: c_int = 1;

extern "C" {
    static AF_NETLINK: c_int;
    static AF_UNIX: c_int;
    static SOCK_RAW: c_int;
    static SOCK_STREAM: c_int;
    static NETLINK_SOCK_DIAG: c_int;
    static SOL_SOCKET: c_int;
    static SO_COOKIE: c_int;
    static CLONE_NEWUSER: c_int;
    static SOCK_DIAG_BY_FAMILY: c_int;
    static NLM_F_REQUEST: c_int;
    static UDIAG_SHOW_UID: c_int;
    static UNIX_DIAG_UID: c_int;

    fn unshare(flags: c_int) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn fstat(fd: c_int, statbuf: *mut stat) -> c_int;
    fn getsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *mut c_void,
        option_len: *mut socklen_t,
    ) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn sendmsg(socket: c_int, message: *const msghdr, flags: c_int) -> ssize_t;
    fn recvmsg(socket: c_int, message: *mut msghdr, flags: c_int) -> ssize_t;
    fn getuid() -> uid_t;
}

#[repr(C)]
pub struct __test_metadata {
    _unused: [u8; 0],
}

#[repr(C)]
struct stat {
    st_dev: c_ulong,
    st_ino: c_ulong,
}

#[repr(C)]
struct sockaddr {
    sa_family: sa_family_t,
    sa_data: [u8; 14],
}

#[repr(C)]
struct sockaddr_nl {
    nl_family: sa_family_t,
    nl_pad: u16,
    nl_pid: __u32,
    nl_groups: __u32,
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
struct nlmsghdr {
    nlmsg_len: __u32,
    nlmsg_type: u16,
    nlmsg_flags: u16,
    nlmsg_seq: __u32,
    nlmsg_pid: __u32,
}

#[repr(C)]
struct unix_diag_req {
    sdiag_family: u8,
    sdiag_protocol: u8,
    pad: u16,
    udiag_states: __u32,
    udiag_ino: __u32,
    udiag_show: __u32,
    udiag_cookie: [__u32; 2],
}

#[repr(C)]
struct rtattr {
    rta_len: u16,
    rta_type: u16,
}

#[repr(C)]
struct diag_uid {
    netlink_fd: c_int,
    unix_fd: c_int,
    inode: __u32,
    cookie: __u64,
}

#[repr(C)]
struct diag_uid_variant {
    unshare: c_int,
    udiag_show: c_int,
}

static diag_uid_uid: diag_uid_variant = diag_uid_variant {
    unshare: 0,
    udiag_show: unsafe { UDIAG_SHOW_UID },
};

static diag_uid_uid_unshare: diag_uid_variant = diag_uid_variant {
    unshare: unsafe { CLONE_NEWUSER },
    udiag_show: unsafe { UDIAG_SHOW_UID },
};

const NLMSG_ALIGNTO: usize = 4;
const RTA_ALIGNTO: usize = 4;

const fn nlmsg_align(len: usize) -> usize {
    (len + NLMSG_ALIGNTO - 1) & !(NLMSG_ALIGNTO - 1)
}

const fn rta_align(len: usize) -> usize {
    (len + RTA_ALIGNTO - 1) & !(RTA_ALIGNTO - 1)
}

const fn nlmsg_length(len: usize) -> usize {
    len + nlmsg_align(core::mem::size_of::<nlmsghdr>())
}

unsafe fn NLMSG_LENGTH(len: usize) -> __u32 {
    nlmsg_length(len) as __u32
}

unsafe fn NLMSG_OK(nlh: *const nlmsghdr, len: c_int) -> c_int {
    (len >= core::mem::size_of::<nlmsghdr>() as c_int
        && (*nlh).nlmsg_len >= core::mem::size_of::<nlmsghdr>() as __u32
        && (*nlh).nlmsg_len <= len as __u32) as c_int
}

unsafe fn NLMSG_DATA(nlh: *mut nlmsghdr) -> *mut unix_diag_req {
    (nlh as *mut u8).add(nlmsg_length(0)) as *mut unix_diag_req
}

unsafe fn NLMSG_NEXT(nlh: *mut nlmsghdr, len: &mut c_int) -> *mut nlmsghdr {
    let aligned = nlmsg_align((*nlh).nlmsg_len as usize) as c_int;
    *len -= aligned;
    (nlh as *mut u8).add(aligned as usize) as *mut nlmsghdr
}

unsafe fn RTA_OK(rta: *const rtattr, len: c_uint) -> c_int {
    (len >= core::mem::size_of::<rtattr>() as c_uint
        && (*rta).rta_len >= core::mem::size_of::<rtattr>() as u16
        && ((*rta).rta_len as c_uint) <= len) as c_int
}

unsafe fn RTA_DATA(rta: *mut rtattr) -> *mut c_void {
    (rta as *mut u8).add(rta_align(core::mem::size_of::<rtattr>())) as *mut c_void
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

macro_rules! ASSERT_NE {
    ($left:expr, $right:expr) => {
        assert_ne!($left, $right)
    };
}

macro_rules! ASSERT_GT {
    ($left:expr, $right:expr) => {
        assert!($left > $right)
    };
}

unsafe fn diag_uid_setup(
    _metadata: *mut __test_metadata,
    self_: *mut diag_uid,
    variant: *const diag_uid_variant,
) {
    let mut file_stat: stat = core::mem::zeroed();
    let mut optlen: socklen_t;
    let mut ret: c_int;

    if (*variant).unshare != 0 {
        ASSERT_EQ!(unshare((*variant).unshare), 0);
    }

    (*self_).netlink_fd = socket(AF_NETLINK, SOCK_RAW, NETLINK_SOCK_DIAG);
    ASSERT_NE!((*self_).netlink_fd, -1);

    (*self_).unix_fd = socket(AF_UNIX, SOCK_STREAM, 0);
    ASSERT_NE!((*self_).unix_fd, -1);

    ret = fstat((*self_).unix_fd, &mut file_stat);
    ASSERT_EQ!(ret, 0);

    (*self_).inode = file_stat.st_ino as __u32;

    optlen = core::mem::size_of_val(&(*self_).cookie) as socklen_t;
    ret = getsockopt(
        (*self_).unix_fd,
        SOL_SOCKET,
        SO_COOKIE,
        &mut (*self_).cookie as *mut __u64 as *mut c_void,
        &mut optlen,
    );
    ASSERT_EQ!(ret, 0);
}

unsafe fn diag_uid_teardown(_metadata: *mut __test_metadata, self_: *mut diag_uid) {
    close((*self_).netlink_fd);
    close((*self_).unix_fd);
}

#[repr(C)]
struct send_request_req {
    nlh: nlmsghdr,
    udr: unix_diag_req,
}

unsafe fn send_request(
    _metadata: *mut __test_metadata,
    self_: *mut diag_uid,
    variant: *const diag_uid_variant,
) -> c_int {
    let mut req = send_request_req {
        nlh: nlmsghdr {
            nlmsg_len: core::mem::size_of::<send_request_req>() as __u32,
            nlmsg_type: SOCK_DIAG_BY_FAMILY as u16,
            nlmsg_flags: NLM_F_REQUEST as u16,
            nlmsg_seq: 0,
            nlmsg_pid: 0,
        },
        udr: unix_diag_req {
            sdiag_family: AF_UNIX as u8,
            sdiag_protocol: 0,
            pad: 0,
            udiag_states: 0,
            udiag_ino: (*self_).inode,
            udiag_show: (*variant).udiag_show as __u32,
            udiag_cookie: [
                (*self_).cookie as __u32,
                ((*self_).cookie >> 32) as __u32,
            ],
        },
    };
    let mut nladdr = sockaddr_nl {
        nl_family: AF_NETLINK as sa_family_t,
        nl_pad: 0,
        nl_pid: 0,
        nl_groups: 0,
    };
    let mut iov = iovec {
        iov_base: &mut req as *mut send_request_req as *mut c_void,
        iov_len: core::mem::size_of_val(&req),
    };
    let msg = msghdr {
        msg_name: &mut nladdr as *mut sockaddr_nl as *mut c_void,
        msg_namelen: core::mem::size_of_val(&nladdr) as socklen_t,
        msg_iov: &mut iov,
        msg_iovlen: 1,
        msg_control: core::ptr::null_mut(),
        msg_controllen: 0,
        msg_flags: 0,
    };

    sendmsg((*self_).netlink_fd, &msg, 0) as c_int
}

unsafe fn render_response(
    _metadata: *mut __test_metadata,
    udr: *mut unix_diag_req,
    len: __u32,
) {
    let mut rta_len: c_uint = len - NLMSG_LENGTH(core::mem::size_of_val(&*udr));
    let mut attr: *mut rtattr;
    let uid: uid_t;

    ASSERT_GT!(len, core::mem::size_of_val(&*udr) as __u32);
    ASSERT_EQ!((*udr).sdiag_family as c_int, AF_UNIX);

    attr = udr.add(1) as *mut rtattr;
    ASSERT_NE!(RTA_OK(attr, rta_len), 0);
    ASSERT_EQ!((*attr).rta_type as c_int, UNIX_DIAG_UID);

    uid = *(RTA_DATA(attr) as *mut uid_t);
    ASSERT_EQ!(uid, getuid());
}

unsafe fn receive_response(_metadata: *mut __test_metadata, self_: *mut diag_uid) {
    let mut buf: [isize; 8192 / core::mem::size_of::<isize>()] =
        [0; 8192 / core::mem::size_of::<isize>()];
    let mut nladdr = sockaddr_nl {
        nl_family: AF_NETLINK as sa_family_t,
        nl_pad: 0,
        nl_pid: 0,
        nl_groups: 0,
    };
    let mut iov = iovec {
        iov_base: buf.as_mut_ptr() as *mut c_void,
        iov_len: core::mem::size_of_val(&buf),
    };
    let mut msg = msghdr {
        msg_name: &mut nladdr as *mut sockaddr_nl as *mut c_void,
        msg_namelen: core::mem::size_of_val(&nladdr) as socklen_t,
        msg_iov: &mut iov,
        msg_iovlen: 1,
        msg_control: core::ptr::null_mut(),
        msg_controllen: 0,
        msg_flags: 0,
    };
    let mut nlh: *mut nlmsghdr;
    let mut ret: c_int;

    ret = recvmsg((*self_).netlink_fd, &mut msg, 0) as c_int;
    ASSERT_GT!(ret, 0);

    nlh = buf.as_mut_ptr() as *mut nlmsghdr;
    ASSERT_NE!(NLMSG_OK(nlh, ret), 0);
    ASSERT_EQ!((*nlh).nlmsg_type as c_int, SOCK_DIAG_BY_FAMILY);

    render_response(_metadata, NLMSG_DATA(nlh), (*nlh).nlmsg_len);

    nlh = NLMSG_NEXT(nlh, &mut ret);
    ASSERT_EQ!(NLMSG_OK(nlh, ret), 0);
}

unsafe fn diag_uid_1(
    _metadata: *mut __test_metadata,
    self_: *mut diag_uid,
    variant: *const diag_uid_variant,
) {
    let mut ret: c_int;

    ret = send_request(_metadata, self_, variant);
    ASSERT_GT!(ret, 0);

    receive_response(_metadata, self_);
}

// TEST_HARNESS_MAIN

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
