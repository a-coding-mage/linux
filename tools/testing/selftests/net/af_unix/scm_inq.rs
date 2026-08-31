// SPDX-License-Identifier: GPL-2.0
/* Copyright 2025 Google LLC */

/*
 * C dependencies translated from:
 * <linux/sockios.h>, <sys/ioctl.h>, <sys/socket.h>, <sys/types.h>,
 * and "kselftest_harness.h".
 */

use core::ffi::{c_int, c_void};
use core::mem::{size_of, size_of_val, zeroed};
use core::ptr::{null_mut, write_bytes};

const NR_CHUNKS: c_int = 100;
const MSG_LEN: usize = 256;
const NR_PARTIAL_READS: usize = 3;

#[repr(C)]
pub struct __test_metadata {
    _private: [u8; 0],
}

#[repr(C)]
pub struct scm_inq {
    fd: [c_int; 2],
}

#[repr(C)]
pub struct scm_inq_variant {
    type_: c_int,
}

static stream: scm_inq_variant = scm_inq_variant {
    type_: libc::SOCK_STREAM,
};

static dgram: scm_inq_variant = scm_inq_variant {
    type_: libc::SOCK_DGRAM,
};

static seqpacket: scm_inq_variant = scm_inq_variant {
    type_: libc::SOCK_SEQPACKET,
};

const SO_INQ: c_int = libc::SO_RXQ_OVFL + 1;
const SCM_INQ: c_int = SO_INQ;
const ENOPROTOOPT: c_int = 92;

extern "C" {
    fn socketpair(domain: c_int, type_: c_int, protocol: c_int, sv: *mut c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn send(sockfd: c_int, buf: *const c_void, len: usize, flags: c_int) -> isize;
    fn recvmsg(sockfd: c_int, msg: *mut libc::msghdr, flags: c_int) -> isize;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: libc::socklen_t,
    ) -> c_int;
    fn ioctl(fd: c_int, request: libc::c_ulong, ...) -> c_int;
}

extern "C" {
    static mut errno: c_int;
}

macro_rules! ASSERT_EQ {
    ($expected:expr, $actual:expr) => {
        assert_eq!($expected, $actual)
    };
}

macro_rules! ASSERT_NE {
    ($expected:expr, $actual:expr) => {
        assert_ne!($expected, $actual)
    };
}

const fn CMSG_ALIGN(len: usize) -> usize {
    (len + size_of::<usize>() - 1) & !(size_of::<usize>() - 1)
}

const fn CMSG_SPACE(len: usize) -> usize {
    CMSG_ALIGN(size_of::<libc::cmsghdr>()) + CMSG_ALIGN(len)
}

const fn CMSG_LEN(len: usize) -> usize {
    CMSG_ALIGN(size_of::<libc::cmsghdr>()) + len
}

unsafe fn CMSG_FIRSTHDR(msg: *const libc::msghdr) -> *mut libc::cmsghdr {
    if (*msg).msg_controllen >= size_of::<libc::cmsghdr>() {
        (*msg).msg_control as *mut libc::cmsghdr
    } else {
        null_mut()
    }
}

unsafe fn CMSG_DATA(cmsg: *mut libc::cmsghdr) -> *mut libc::c_uchar {
    (cmsg as *mut u8).add(CMSG_ALIGN(size_of::<libc::cmsghdr>())) as *mut libc::c_uchar
}

unsafe fn scm_inq_setup(
    _metadata: *mut __test_metadata,
    self_: *mut scm_inq,
    variant: *const scm_inq_variant,
) {
    let err: c_int;

    err = socketpair(
        libc::AF_UNIX,
        (*variant).type_ | libc::SOCK_NONBLOCK,
        0,
        (*self_).fd.as_mut_ptr(),
    );
    ASSERT_EQ!(0, err);
}

unsafe fn scm_inq_teardown(_metadata: *mut __test_metadata, self_: *mut scm_inq) {
    close((*self_).fd[0]);
    close((*self_).fd[1]);
}

unsafe fn send_chunks(_metadata: *mut __test_metadata, self_: *mut scm_inq) {
    let buf: [u8; MSG_LEN] = [0; MSG_LEN];
    let mut i: c_int;
    let mut ret: isize;

    i = 0;
    while i < NR_CHUNKS {
        ret = send((*self_).fd[0], buf.as_ptr() as *const c_void, size_of_val(&buf), 0);
        ASSERT_EQ!(size_of_val(&buf) as isize, ret);
        i += 1;
    }
}

unsafe fn recv_chunks(_metadata: *mut __test_metadata, self_: *mut scm_inq) {
    let mut cmsg_buf: [u8; CMSG_SPACE(size_of::<c_int>())] = [0; CMSG_SPACE(size_of::<c_int>())];
    let mut msg: libc::msghdr = zeroed();
    let mut iov: libc::iovec = zeroed();
    let mut cmsg: *mut libc::cmsghdr;
    let mut buf: [u8; MSG_LEN] = [0; MSG_LEN];
    let mut i: c_int;
    let mut ret: isize;
    let mut inq: c_int = 0;

    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1;
    msg.msg_control = cmsg_buf.as_mut_ptr() as *mut c_void;
    msg.msg_controllen = size_of_val(&cmsg_buf);

    iov.iov_base = buf.as_mut_ptr() as *mut c_void;
    iov.iov_len = size_of_val(&buf);

    i = 0;
    while i < NR_CHUNKS {
        write_bytes(buf.as_mut_ptr(), 0, size_of_val(&buf));
        write_bytes(cmsg_buf.as_mut_ptr(), 0, size_of_val(&cmsg_buf));

        ret = recvmsg((*self_).fd[1], &mut msg, 0);
        ASSERT_EQ!(MSG_LEN as isize, ret);

        cmsg = CMSG_FIRSTHDR(&msg);
        ASSERT_NE!(null_mut::<libc::cmsghdr>(), cmsg);
        ASSERT_EQ!(CMSG_LEN(size_of::<c_int>()), (*cmsg).cmsg_len as usize);
        ASSERT_EQ!(libc::SOL_SOCKET, (*cmsg).cmsg_level);
        ASSERT_EQ!(SCM_INQ, (*cmsg).cmsg_type);

        ret = ioctl((*self_).fd[1], libc::SIOCINQ, &mut inq) as isize;
        ASSERT_EQ!(0, ret);
        ASSERT_EQ!(*(CMSG_DATA(cmsg) as *mut c_int), inq);

        i += 1;
    }
}

unsafe fn scm_inq_basic(
    _metadata: *mut __test_metadata,
    self_: *mut scm_inq,
    variant: *const scm_inq_variant,
) {
    let mut err: c_int;
    let mut inq: c_int = 0;
    let one: c_int = 1;

    err = setsockopt(
        (*self_).fd[1],
        libc::SOL_SOCKET,
        SO_INQ,
        &one as *const c_int as *const c_void,
        size_of::<c_int>() as libc::socklen_t,
    );
    if (*variant).type_ != libc::SOCK_STREAM {
        ASSERT_EQ!(-ENOPROTOOPT, -errno);
        return;
    }

    ASSERT_EQ!(0, err);

    err = ioctl((*self_).fd[1], libc::SIOCINQ, &mut inq);
    ASSERT_EQ!(0, err);
    ASSERT_EQ!(0, inq);

    send_chunks(_metadata, self_);
    recv_chunks(_metadata, self_);
}

unsafe fn scm_inq_partial_read(
    _metadata: *mut __test_metadata,
    self_: *mut scm_inq,
    variant: *const scm_inq_variant,
) {
    let mut buf: [u8; MSG_LEN * NR_PARTIAL_READS] = [0; MSG_LEN * NR_PARTIAL_READS];
    let mut cmsg_buf: [u8; CMSG_SPACE(size_of::<c_int>())] = [0; CMSG_SPACE(size_of::<c_int>())];
    let mut msg: libc::msghdr = zeroed();
    let mut iov: libc::iovec = zeroed();
    let mut cmsg: *mut libc::cmsghdr;
    let mut err: c_int;
    let mut inq: c_int = 0;
    let mut ret: isize;
    let mut i: c_int;
    let mut remain: c_int;
    let one: c_int = 1;

    err = setsockopt(
        (*self_).fd[1],
        libc::SOL_SOCKET,
        SO_INQ,
        &one as *const c_int as *const c_void,
        size_of::<c_int>() as libc::socklen_t,
    );
    if (*variant).type_ != libc::SOCK_STREAM {
        ASSERT_EQ!(-ENOPROTOOPT, -errno);
        return;
    }
    ASSERT_EQ!(0, err);

    ret = send((*self_).fd[0], buf.as_ptr() as *const c_void, size_of_val(&buf), 0);
    ASSERT_EQ!(size_of_val(&buf) as isize, ret);

    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1;
    msg.msg_control = cmsg_buf.as_mut_ptr() as *mut c_void;
    msg.msg_controllen = size_of_val(&cmsg_buf);

    iov.iov_base = buf.as_mut_ptr() as *mut c_void;
    iov.iov_len = MSG_LEN;

    i = 0;
    while i < NR_PARTIAL_READS as c_int {
        remain = (MSG_LEN * (NR_PARTIAL_READS - 1 - i as usize)) as c_int;

        write_bytes(buf.as_mut_ptr(), 0, MSG_LEN);
        write_bytes(cmsg_buf.as_mut_ptr(), 0, size_of_val(&cmsg_buf));
        ret = recvmsg((*self_).fd[1], &mut msg, 0);
        ASSERT_EQ!(MSG_LEN as isize, ret);

        cmsg = CMSG_FIRSTHDR(&msg);
        ASSERT_NE!(null_mut::<libc::cmsghdr>(), cmsg);
        ASSERT_EQ!(CMSG_LEN(size_of::<c_int>()), (*cmsg).cmsg_len as usize);
        ASSERT_EQ!(libc::SOL_SOCKET, (*cmsg).cmsg_level);
        ASSERT_EQ!(SCM_INQ, (*cmsg).cmsg_type);
        ASSERT_EQ!(remain, *(CMSG_DATA(cmsg) as *mut c_int));

        ret = ioctl((*self_).fd[1], libc::SIOCINQ, &mut inq) as isize;
        ASSERT_EQ!(0, ret);
        ASSERT_EQ!(remain, inq);

        i += 1;
    }
}

fn main() {
    /* TEST_HARNESS_MAIN */
}
