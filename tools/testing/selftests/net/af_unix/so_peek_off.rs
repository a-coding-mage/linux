// SPDX-License-Identifier: GPL-2.0
/* Copyright 2025 Google LLC */

/*
 * Rust translation of testing/selftests/net/af_unix/so_peek_off.c.
 *
 * The original C file depends on kselftest_harness.h for FIXTURE,
 * FIXTURE_VARIANT, TEST_F, assertions, metadata, and TEST_HARNESS_MAIN.
 */

use core::ffi::{c_int, c_uint, c_void};

const SOCK_STREAM: c_int = 1;
const SOCK_DGRAM: c_int = 2;
const SOCK_SEQPACKET: c_int = 5;
const AF_UNIX: c_int = 1;
const SOL_SOCKET: c_int = 1;
const SO_RCVTIMEO_NEW: c_int = 66;
const SO_PEEK_OFF: c_int = 42;
const MSG_PEEK: c_int = 0x02;
const KSFT_FAIL: c_int = 1;

#[repr(C)]
struct timeval {
    tv_sec: i64,
    tv_usec: i64,
}

type socklen_t = c_uint;
type pid_t = c_int;
type size_t = usize;
type ssize_t = isize;

unsafe extern "C" {
    fn socketpair(domain: c_int, type_: c_int, protocol: c_int, sv: *mut c_int) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: socklen_t,
    ) -> c_int;
    fn getsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *mut c_void,
        option_len: *mut socklen_t,
    ) -> c_int;
    fn close_range(first: c_uint, last: c_uint, flags: c_int) -> c_int;
    fn send(socket: c_int, buffer: *const c_void, length: size_t, flags: c_int) -> ssize_t;
    fn recv(socket: c_int, buffer: *mut c_void, length: size_t, flags: c_int) -> ssize_t;
    fn fork() -> pid_t;
    fn usleep(usec: c_uint) -> c_int;
    fn exit(status: c_int) -> !;
}

extern "Rust" {
    fn __TH_LOG(msg: &str);
    fn __bail(code: c_int, metadata: *mut THMetadata) -> !;
}

#[repr(C)]
struct THMetadata {
    exit_code: c_int,
}

struct so_peek_off {
    fd: [c_int; 2], /* 0: sender, 1: receiver */
}

struct so_peek_off_variant {
    type_: c_int,
}

static STREAM: so_peek_off_variant = so_peek_off_variant {
    type_: SOCK_STREAM,
};

static DGRAM: so_peek_off_variant = so_peek_off_variant {
    type_: SOCK_DGRAM,
};

static SEQPACKET: so_peek_off_variant = so_peek_off_variant {
    type_: SOCK_SEQPACKET,
};

unsafe fn so_peek_off_setup(self_: *mut so_peek_off, variant: *const so_peek_off_variant) {
    let timeout = timeval {
        tv_sec: 5,
        tv_usec: 0,
    };
    let mut ret: c_int;

    ret = unsafe {
        socketpair(
            AF_UNIX,
            (*variant).type_,
            0,
            (*self_).fd.as_mut_ptr(),
        )
    };
    assert_eq!(0, ret);

    ret = unsafe {
        setsockopt(
            (*self_).fd[1],
            SOL_SOCKET,
            SO_RCVTIMEO_NEW,
            &timeout as *const timeval as *const c_void,
            core::mem::size_of::<timeval>() as socklen_t,
        )
    };
    assert_eq!(0, ret);

    ret = unsafe {
        setsockopt(
            (*self_).fd[1],
            SOL_SOCKET,
            SO_PEEK_OFF,
            &0_i32 as *const i32 as *const c_void,
            core::mem::size_of::<c_int>() as socklen_t,
        )
    };
    assert_eq!(0, ret);
}

unsafe fn so_peek_off_teardown(self_: *mut so_peek_off) {
    unsafe {
        close_range((*self_).fd[0] as c_uint, (*self_).fd[1] as c_uint, 0);
    }
}

macro_rules! sendeq {
    ($fd:expr, $str:expr, $flags:expr) => {{
        let bytes: ssize_t;
        let len: usize = $str.len();

        bytes = unsafe { send($fd, $str.as_ptr() as *const c_void, len, $flags) };
        assert_eq!(len as ssize_t, bytes);
    }};
}

macro_rules! recveq {
    ($fd:expr, $str:expr, $buflen:expr, $flags:expr) => {{
        let mut buf = vec![0_u8; ($buflen) + 1];
        let bytes: ssize_t;

        bytes = unsafe { recv($fd, buf.as_mut_ptr() as *mut c_void, $buflen, $flags) };
        assert_ne!(-1, bytes);
        assert_eq!($str.as_bytes(), &buf[..$str.len()]);
        assert_eq!(0, buf[$str.len()]);
    }};
}

macro_rules! peekoffeq {
    ($fd:expr, $expected:expr) => {{
        let mut optlen: socklen_t = core::mem::size_of::<c_int>() as socklen_t;
        let mut off: c_int = -1;
        let ret: c_int;

        ret = unsafe {
            getsockopt(
                $fd,
                SOL_SOCKET,
                SO_PEEK_OFF,
                &mut off as *mut c_int as *mut c_void,
                &mut optlen,
            )
        };
        assert_eq!(0, ret);
        assert_eq!(core::mem::size_of_val(&off) as socklen_t, optlen);
        assert_eq!($expected, off);
    }};
}

macro_rules! async_block {
    ($metadata:expr, $body:block) => {{
        let pid: pid_t = unsafe { fork() };

        if pid < 0 {
            unsafe {
                __TH_LOG("Failed to start async {}");
                (*$metadata).exit_code = KSFT_FAIL;
                __bail(1, $metadata);
            }
        }

        if pid == 0 {
            $body
            unsafe {
                exit(0);
            }
        }
    }};
}

unsafe fn single_chunk(self_: *mut so_peek_off, _variant: *const so_peek_off_variant) {
    sendeq!((*self_).fd[0], "aaaabbbb", 0);

    recveq!((*self_).fd[1], "aaaa", 4, MSG_PEEK);
    peekoffeq!((*self_).fd[1], 4);
    recveq!((*self_).fd[1], "bbbb", 100, MSG_PEEK);
    peekoffeq!((*self_).fd[1], 8);

    recveq!((*self_).fd[1], "aaaabbbb", 8, 0);
    peekoffeq!((*self_).fd[1], 0);
}

unsafe fn two_chunks(self_: *mut so_peek_off, _variant: *const so_peek_off_variant) {
    sendeq!((*self_).fd[0], "aaaa", 0);
    sendeq!((*self_).fd[0], "bbbb", 0);

    recveq!((*self_).fd[1], "aaaa", 4, MSG_PEEK);
    peekoffeq!((*self_).fd[1], 4);
    recveq!((*self_).fd[1], "bbbb", 100, MSG_PEEK);
    peekoffeq!((*self_).fd[1], 8);

    recveq!((*self_).fd[1], "aaaa", 4, 0);
    recveq!((*self_).fd[1], "bbbb", 4, 0);
    peekoffeq!((*self_).fd[1], 0);
}

unsafe fn two_chunks_blocking(
    self_: *mut so_peek_off,
    _variant: *const so_peek_off_variant,
    _metadata: *mut THMetadata,
) {
    async_block!(_metadata, {
        unsafe {
            usleep(1000);
        }
        sendeq!((*self_).fd[0], "aaaa", 0);
    });

    recveq!((*self_).fd[1], "aaaa", 4, MSG_PEEK);
    peekoffeq!((*self_).fd[1], 4);

    async_block!(_metadata, {
        unsafe {
            usleep(1000);
        }
        sendeq!((*self_).fd[0], "bbbb", 0);
    });

    /* goto again; -> goto redo; in unix_stream_read_generic(). */
    recveq!((*self_).fd[1], "bbbb", 100, MSG_PEEK);
    peekoffeq!((*self_).fd[1], 8);

    recveq!((*self_).fd[1], "aaaa", 4, 0);
    recveq!((*self_).fd[1], "bbbb", 4, 0);
    peekoffeq!((*self_).fd[1], 0);
}

unsafe fn two_chunks_overlap(self_: *mut so_peek_off, variant: *const so_peek_off_variant) {
    sendeq!((*self_).fd[0], "aaaa", 0);
    recveq!((*self_).fd[1], "aa", 2, MSG_PEEK);
    peekoffeq!((*self_).fd[1], 2);

    sendeq!((*self_).fd[0], "bbbb", 0);

    if (*variant).type_ == SOCK_STREAM {
        /* SOCK_STREAM tries to fill the buffer. */
        recveq!((*self_).fd[1], "aabb", 4, MSG_PEEK);
        peekoffeq!((*self_).fd[1], 6);
        recveq!((*self_).fd[1], "bb", 100, MSG_PEEK);
        peekoffeq!((*self_).fd[1], 8);
    } else {
        /* SOCK_DGRAM and SOCK_SEQPACKET returns at the skb boundary. */
        recveq!((*self_).fd[1], "aa", 100, MSG_PEEK);
        peekoffeq!((*self_).fd[1], 4);
        recveq!((*self_).fd[1], "bbbb", 100, MSG_PEEK);
        peekoffeq!((*self_).fd[1], 8);
    }

    recveq!((*self_).fd[1], "aaaa", 4, 0);
    recveq!((*self_).fd[1], "bbbb", 4, 0);
    peekoffeq!((*self_).fd[1], 0);
}

unsafe fn two_chunks_overlap_blocking(
    self_: *mut so_peek_off,
    _variant: *const so_peek_off_variant,
    _metadata: *mut THMetadata,
) {
    async_block!(_metadata, {
        unsafe {
            usleep(1000);
        }
        sendeq!((*self_).fd[0], "aaaa", 0);
    });

    recveq!((*self_).fd[1], "aa", 2, MSG_PEEK);
    peekoffeq!((*self_).fd[1], 2);

    async_block!(_metadata, {
        unsafe {
            usleep(1000);
        }
        sendeq!((*self_).fd[0], "bbbb", 0);
    });

    /* Even SOCK_STREAM does not wait if at least one byte is read. */
    recveq!((*self_).fd[1], "aa", 100, MSG_PEEK);
    peekoffeq!((*self_).fd[1], 4);

    recveq!((*self_).fd[1], "bbbb", 100, MSG_PEEK);
    peekoffeq!((*self_).fd[1], 8);

    recveq!((*self_).fd[1], "aaaa", 4, 0);
    recveq!((*self_).fd[1], "bbbb", 4, 0);
    peekoffeq!((*self_).fd[1], 0);
}

/* TEST_HARNESS_MAIN */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
