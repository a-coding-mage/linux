// SPDX-License-Identifier: GPL-2.0

// C source used _GNU_SOURCE and included sched.h, stddef.h, stdio.h, unistd.h,
// sys/socket.h, sys/un.h, and kselftest_harness.h.

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

type socklen_t = c_uint;
type sa_family_t = u16;

const AF_UNIX: c_int = 1;
const SOCK_STREAM: c_int = 1;
const SOCK_DGRAM: c_int = 2;
const CLONE_NEWNET: c_int = 0x40000000;
const ECONNREFUSED: c_int = 111;

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
struct unix_connect {
    server: c_int,
    client: c_int,
    family: c_int,
}

#[repr(C)]
struct unix_connect_variant {
    type_: c_int,
    sun_path: [c_char; 8],
    len: c_int,
    flags: c_int,
    err: c_int,
}

const fn sun_path(bytes: &[u8]) -> [c_char; 8] {
    let mut out = [0 as c_char; 8];
    let mut i = 0;

    while i < bytes.len() {
        out[i] = bytes[i] as c_char;
        i += 1;
    }

    out
}

static stream_pathname: unix_connect_variant = unix_connect_variant {
    type_: SOCK_STREAM,
    sun_path: sun_path(b"test"),
    len: 4 + 1,
    flags: 0,
    err: 0,
};

static stream_abstract: unix_connect_variant = unix_connect_variant {
    type_: SOCK_STREAM,
    sun_path: sun_path(b"\0test"),
    len: 5,
    flags: 0,
    err: 0,
};

static stream_pathname_netns: unix_connect_variant = unix_connect_variant {
    type_: SOCK_STREAM,
    sun_path: sun_path(b"test"),
    len: 4 + 1,
    flags: CLONE_NEWNET,
    err: 0,
};

static stream_abstract_netns: unix_connect_variant = unix_connect_variant {
    type_: SOCK_STREAM,
    sun_path: sun_path(b"\0test"),
    len: 5,
    flags: CLONE_NEWNET,
    err: ECONNREFUSED,
};

static dgram_pathname: unix_connect_variant = unix_connect_variant {
    type_: SOCK_DGRAM,
    sun_path: sun_path(b"test"),
    len: 4 + 1,
    flags: 0,
    err: 0,
};

static dgram_abstract: unix_connect_variant = unix_connect_variant {
    type_: SOCK_DGRAM,
    sun_path: sun_path(b"\0test"),
    len: 5,
    flags: 0,
    err: 0,
};

static dgram_pathname_netns: unix_connect_variant = unix_connect_variant {
    type_: SOCK_DGRAM,
    sun_path: sun_path(b"test"),
    len: 4 + 1,
    flags: CLONE_NEWNET,
    err: 0,
};

static dgram_abstract_netns: unix_connect_variant = unix_connect_variant {
    type_: SOCK_DGRAM,
    sun_path: sun_path(b"\0test"),
    len: 5,
    flags: CLONE_NEWNET,
    err: ECONNREFUSED,
};

unsafe extern "C" {
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn listen(sockfd: c_int, backlog: c_int) -> c_int;
    fn connect(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn remove(pathname: *const c_char) -> c_int;
    fn unshare(flags: c_int) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn __errno_location() -> *mut c_int;
}

unsafe fn errno() -> c_int {
    unsafe { *__errno_location() }
}

unsafe fn unix_connect_setup(self_: *mut unix_connect, _variant: *const unix_connect_variant) {
    unsafe {
        (*self_).family = AF_UNIX;
    }
}

unsafe fn unix_connect_teardown(self_: *mut unix_connect, variant: *const unix_connect_variant) {
    unsafe {
        close((*self_).server);
        close((*self_).client);

        if (*variant).sun_path[0] != 0 {
            remove(c"test".as_ptr());
        }
    }
}

unsafe fn unix_connect_test(self_: *mut unix_connect, variant: *const unix_connect_variant) {
    let mut addrlen: socklen_t;
    let mut addr = sockaddr_un {
        sun_family: unsafe { (*self_).family as sa_family_t },
        sun_path: [0; 108],
    };
    let mut err: c_int;

    unsafe {
        (*self_).server = socket((*self_).family, (*variant).type_, 0);
        assert_ne!(-1, (*self_).server);

        addrlen = (mem::offset_of!(sockaddr_un, sun_path) + (*variant).len as usize) as socklen_t;
        memcpy(
            addr.sun_path.as_mut_ptr() as *mut c_void,
            (*variant).sun_path.as_ptr() as *const c_void,
            (*variant).len as usize,
        );

        err = bind(
            (*self_).server,
            &addr as *const sockaddr_un as *const sockaddr,
            addrlen,
        );
        assert_eq!(0, err);

        if (*variant).type_ == SOCK_STREAM {
            err = listen((*self_).server, 32);
            assert_eq!(0, err);
        }

        err = unshare((*variant).flags);
        assert_eq!(0, err);

        (*self_).client = socket((*self_).family, (*variant).type_, 0);
        assert!(0 < (*self_).client);

        err = connect(
            (*self_).client,
            &addr as *const sockaddr_un as *const sockaddr,
            addrlen,
        );
        assert_eq!((*variant).err, if err == -1 { errno() } else { 0 });
    }
}

fn main() {
    // TEST_HARNESS_MAIN
    let variants: [&unix_connect_variant; 8] = [
        &stream_pathname,
        &stream_abstract,
        &stream_pathname_netns,
        &stream_abstract_netns,
        &dgram_pathname,
        &dgram_abstract,
        &dgram_pathname_netns,
        &dgram_abstract_netns,
    ];

    for variant in variants {
        let mut self_ = unix_connect {
            server: -1,
            client: -1,
            family: 0,
        };

        unsafe {
            unix_connect_setup(&mut self_, variant);
            unix_connect_test(&mut self_, variant);
            unix_connect_teardown(&mut self_, variant);
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
