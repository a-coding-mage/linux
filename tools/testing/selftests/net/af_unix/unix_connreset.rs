// SPDX-License-Identifier: GPL-2.0
/*
 * Selftest for AF_UNIX socket close and ECONNRESET behaviour.
 *
 * This test verifies:
 *  1. SOCK_STREAM returns EOF when the peer closes normally.
 *  2. SOCK_STREAM returns ECONNRESET if peer closes with unread data.
 *  3. SOCK_SEQPACKET returns EOF when the peer closes normally.
 *  4. SOCK_SEQPACKET returns ECONNRESET if peer closes with unread data.
 *  5. SOCK_DGRAM does not return ECONNRESET when the peer closes.
 *
 * These tests document the intended Linux behaviour.
 *
 */

// C source used _GNU_SOURCE and included:
// string.h, fcntl.h, unistd.h, errno.h, sys/socket.h, sys/un.h,
// and ../../kselftest_harness.h.

use core::ffi::{c_char, c_int, c_void};

const SOCK_PATH: &[u8] = b"/tmp/af_unix_connreset.sock\0";

unsafe extern "C" {
    static mut errno: c_int;

    fn unlink(pathname: *const c_char) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn listen(sockfd: c_int, backlog: c_int) -> c_int;
    fn connect(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn accept(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn recv(sockfd: c_int, buf: *mut c_void, len: usize, flags: c_int) -> ssize_t;
    fn send(sockfd: c_int, buf: *const c_void, len: usize, flags: c_int) -> ssize_t;
    fn snprintf(str_: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
}

type ssize_t = isize;
type socklen_t = u32;

const AF_UNIX: c_int = 1;
const SOCK_STREAM: c_int = 1;
const SOCK_DGRAM: c_int = 2;
const SOCK_SEQPACKET: c_int = 5;
const SOCK_NONBLOCK: c_int = 0o0004000;
const EAGAIN: c_int = 11;
const ECONNRESET: c_int = 104;
const KSFT_XFAIL: c_int = 2;

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

static mut _metadata: *mut TestMetadata = core::ptr::null_mut();

#[repr(C)]
struct TestMetadata {
    results: *mut TestResults,
}

#[repr(C)]
struct TestResults {
    reason: [c_char; 1024],
}

unsafe fn remove_socket_file() {
    unsafe {
        unlink(SOCK_PATH.as_ptr() as *const c_char);
    }
}

struct unix_sock {
    server: c_int,
    client: c_int,
    child: c_int,
}

struct unix_sock_variant {
    socket_type: c_int,
    name: *const c_char,
}

static stream: unix_sock_variant = unix_sock_variant {
    socket_type: SOCK_STREAM,
    name: c"SOCK_STREAM".as_ptr(),
};

static dgram: unix_sock_variant = unix_sock_variant {
    socket_type: SOCK_DGRAM,
    name: c"SOCK_DGRAM".as_ptr(),
};

static seqpacket: unix_sock_variant = unix_sock_variant {
    socket_type: SOCK_SEQPACKET,
    name: c"SOCK_SEQPACKET".as_ptr(),
};

unsafe fn unix_sock_setup(self_: *mut unix_sock, variant: *const unix_sock_variant) {
    let mut addr: sockaddr_un = unsafe { core::mem::zeroed() };
    let mut err: c_int;

    addr.sun_family = AF_UNIX as u16;
    let path = SOCK_PATH;
    for i in 0..path.len() {
        addr.sun_path[i] = path[i] as c_char;
    }
    unsafe {
        remove_socket_file();

        (*self_).server = socket(AF_UNIX, (*variant).socket_type, 0);
        ASSERT_LT!(-1, (*self_).server);

        err = bind(
            (*self_).server,
            &addr as *const sockaddr_un as *const sockaddr,
            core::mem::size_of_val(&addr) as socklen_t,
        );
        ASSERT_EQ!(0, err);

        if (*variant).socket_type == SOCK_STREAM || (*variant).socket_type == SOCK_SEQPACKET {
            err = listen((*self_).server, 1);
            ASSERT_EQ!(0, err);
        }

        (*self_).client = socket(AF_UNIX, (*variant).socket_type | SOCK_NONBLOCK, 0);
        ASSERT_LT!(-1, (*self_).client);

        err = connect(
            (*self_).client,
            &addr as *const sockaddr_un as *const sockaddr,
            core::mem::size_of_val(&addr) as socklen_t,
        );
        ASSERT_EQ!(0, err);
    }
}

unsafe fn unix_sock_teardown(self_: *mut unix_sock, variant: *const unix_sock_variant) {
    unsafe {
        if (*variant).socket_type == SOCK_STREAM || (*variant).socket_type == SOCK_SEQPACKET {
            close((*self_).child);
        }

        close((*self_).client);
        close((*self_).server);
        remove_socket_file();
    }
}

/* Test 1: peer closes normally */
unsafe fn unix_sock_eof(self_: *mut unix_sock, variant: *const unix_sock_variant) {
    let mut buf: [c_char; 16] = [0; 16];
    let n: ssize_t;

    unsafe {
        if (*variant).socket_type == SOCK_STREAM || (*variant).socket_type == SOCK_SEQPACKET {
            (*self_).child = accept(
                (*self_).server,
                core::ptr::null_mut(),
                core::ptr::null_mut(),
            );
            ASSERT_LT!(-1, (*self_).child);

            close((*self_).child);
        } else {
            close((*self_).server);
        }

        n = recv((*self_).client, buf.as_mut_ptr() as *mut c_void, buf.len(), 0);

        if (*variant).socket_type == SOCK_STREAM || (*variant).socket_type == SOCK_SEQPACKET {
            ASSERT_EQ!(0, n);
        } else {
            ASSERT_EQ!(-1, n);
            ASSERT_EQ!(EAGAIN, errno);
        }
    }
}

/* Test 2: peer closes with unread data */
unsafe fn unix_sock_reset_unread_behavior(
    self_: *mut unix_sock,
    variant: *const unix_sock_variant,
) {
    let mut buf: [c_char; 16] = [0; 16];
    let n: ssize_t;

    unsafe {
        /* Send data that will remain unread */
        send(
            (*self_).client,
            c"hello".as_ptr() as *const c_void,
            5,
            0,
        );

        if (*variant).socket_type == SOCK_DGRAM {
            /* No real connection, just close the server */
            close((*self_).server);
        } else {
            (*self_).child = accept(
                (*self_).server,
                core::ptr::null_mut(),
                core::ptr::null_mut(),
            );
            ASSERT_LT!(-1, (*self_).child);

            /* Peer closes before client reads */
            close((*self_).child);
        }

        n = recv((*self_).client, buf.as_mut_ptr() as *mut c_void, buf.len(), 0);
        ASSERT_EQ!(-1, n);

        if (*variant).socket_type == SOCK_STREAM || (*variant).socket_type == SOCK_SEQPACKET {
            ASSERT_EQ!(ECONNRESET, errno);
        } else {
            ASSERT_EQ!(EAGAIN, errno);
        }
    }
}

/* Test 3: closing unaccepted (embryo) server socket should reset client. */
unsafe fn unix_sock_reset_closed_embryo(
    self_: *mut unix_sock,
    variant: *const unix_sock_variant,
) {
    let mut buf: [c_char; 16] = [0; 16];
    let n: ssize_t;

    unsafe {
        if (*variant).socket_type == SOCK_DGRAM {
            snprintf(
                (*(*_metadata).results).reason.as_mut_ptr(),
                core::mem::size_of_val(&(*(*_metadata).results).reason),
                c"Test only applies to SOCK_STREAM and SOCK_SEQPACKET".as_ptr(),
            );
            exit(KSFT_XFAIL);
        }

        /* Close server without accept()ing */
        close((*self_).server);

        n = recv((*self_).client, buf.as_mut_ptr() as *mut c_void, buf.len(), 0);

        ASSERT_EQ!(-1, n);
        ASSERT_EQ!(ECONNRESET, errno);
    }
}

// TEST_HARNESS_MAIN

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
