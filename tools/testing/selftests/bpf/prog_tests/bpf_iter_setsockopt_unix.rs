// SPDX-License-Identifier: GPL-2.0
/* Copyright Amazon.com Inc. or its affiliates. */
/* Translated from:
 * - <sys/socket.h>
 * - <sys/un.h>
 * - <test_progs.h>
 * - "bpf_iter_setsockopt_unix.skel.h"
 */

use core::ffi::{c_char, c_int, c_void};
use core::mem;
use core::ptr;

const NR_CASES: usize = 5;

#[repr(C)]
pub struct bpf_iter_setsockopt_unix {
    pub data: *mut bpf_iter_setsockopt_unix__data,
    pub bss: *mut bpf_iter_setsockopt_unix__bss,
    pub rodata: *mut c_void,
    pub links: bpf_iter_setsockopt_unix__links,
    pub progs: bpf_iter_setsockopt_unix__progs,
}

#[repr(C)]
pub struct bpf_iter_setsockopt_unix__data {
    pub sndbuf_getsockopt: [c_int; NR_CASES],
    pub sndbuf_setsockopt: [c_int; NR_CASES],
}

#[repr(C)]
pub struct bpf_iter_setsockopt_unix__bss {
    pub sun_path: [c_char; 108],
    pub sndbuf_getsockopt_expected: [c_int; NR_CASES],
}

#[repr(C)]
pub struct bpf_iter_setsockopt_unix__links {
    pub change_sndbuf: *mut bpf_link,
}

#[repr(C)]
pub struct bpf_iter_setsockopt_unix__progs {
    pub change_sndbuf: *mut bpf_program,
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_iter_setsockopt_unix__open_and_load() -> *mut bpf_iter_setsockopt_unix;
    fn bpf_iter_setsockopt_unix__destroy(skel: *mut bpf_iter_setsockopt_unix);
    fn bpf_program__attach_iter(
        prog: *mut bpf_program,
        opts: *const c_void,
    ) -> *mut bpf_link;
    fn bpf_link__fd(link: *mut bpf_link) -> c_int;
    fn bpf_iter_create(link_fd: c_int) -> c_int;

    fn ASSERT_NEQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK(actual: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
}

unsafe fn create_unix_socket(skel: *mut bpf_iter_setsockopt_unix) -> c_int {
    let mut addr: libc::sockaddr_un = mem::zeroed();
    addr.sun_family = libc::AF_UNIX as libc::sa_family_t;
    addr.sun_path[0] = 0;

    let mut len: libc::socklen_t;
    let fd: c_int;
    let mut err: c_int;

    fd = libc::socket(libc::AF_UNIX, libc::SOCK_STREAM, 0);
    if !ASSERT_NEQ(fd, -1, c"socket".as_ptr()) {
        return -1;
    }

    len = mem::offset_of!(libc::sockaddr_un, sun_path) as libc::socklen_t;
    err = libc::bind(
        fd,
        &addr as *const libc::sockaddr_un as *const libc::sockaddr,
        len,
    );
    if !ASSERT_OK(err, c"bind".as_ptr()) {
        return -1;
    }

    len = mem::size_of_val(&addr) as libc::socklen_t;
    err = libc::getsockname(
        fd,
        &mut addr as *mut libc::sockaddr_un as *mut libc::sockaddr,
        &mut len,
    );
    if !ASSERT_OK(err, c"getsockname".as_ptr()) {
        return -1;
    }

    ptr::copy_nonoverlapping(
        addr.sun_path.as_ptr(),
        (*(*skel).bss).sun_path.as_mut_ptr(),
        len as usize - mem::offset_of!(libc::sockaddr_un, sun_path),
    );

    fd
}

unsafe fn test_sndbuf(skel: *mut bpf_iter_setsockopt_unix, fd: c_int) {
    let mut optlen: libc::socklen_t;
    let mut i: c_int;
    let mut err: c_int;

    i = 0;
    while i < NR_CASES as c_int {
        if !ASSERT_NEQ(
            (*(*skel).data).sndbuf_getsockopt[i as usize],
            -1,
            c"bpf_(get|set)sockopt".as_ptr(),
        ) {
            return;
        }

        err = libc::setsockopt(
            fd,
            libc::SOL_SOCKET,
            libc::SO_SNDBUF,
            &(*(*skel).data).sndbuf_setsockopt[i as usize] as *const c_int as *const c_void,
            mem::size_of_val(&(*(*skel).data).sndbuf_setsockopt[i as usize])
                as libc::socklen_t,
        );
        if !ASSERT_OK(err, c"setsockopt".as_ptr()) {
            return;
        }

        optlen = mem::size_of_val(&(*(*skel).bss).sndbuf_getsockopt_expected[i as usize])
            as libc::socklen_t;
        err = libc::getsockopt(
            fd,
            libc::SOL_SOCKET,
            libc::SO_SNDBUF,
            &mut (*(*skel).bss).sndbuf_getsockopt_expected[i as usize] as *mut c_int
                as *mut c_void,
            &mut optlen,
        );
        if !ASSERT_OK(err, c"getsockopt".as_ptr()) {
            return;
        }

        if !ASSERT_EQ(
            (*(*skel).data).sndbuf_getsockopt[i as usize],
            (*(*skel).bss).sndbuf_getsockopt_expected[i as usize],
            c"bpf_(get|set)sockopt".as_ptr(),
        ) {
            return;
        }

        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_bpf_iter_setsockopt_unix() {
    let skel: *mut bpf_iter_setsockopt_unix;
    let mut err: c_int;
    let unix_fd: c_int;
    let iter_fd: c_int;
    let mut buf: c_char = 0;

    skel = bpf_iter_setsockopt_unix__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"open_and_load".as_ptr()) {
        return;
    }

    unix_fd = create_unix_socket(skel);
    if !ASSERT_NEQ(unix_fd, -1, c"create_unix_server".as_ptr()) {
        bpf_iter_setsockopt_unix__destroy(skel);
        return;
    }

    (*skel).links.change_sndbuf =
        bpf_program__attach_iter((*skel).progs.change_sndbuf, ptr::null());
    if !ASSERT_OK_PTR(
        (*skel).links.change_sndbuf as *const c_void,
        c"bpf_program__attach_iter".as_ptr(),
    ) {
        bpf_iter_setsockopt_unix__destroy(skel);
        return;
    }

    iter_fd = bpf_iter_create(bpf_link__fd((*skel).links.change_sndbuf));
    if !ASSERT_GE(iter_fd, 0, c"bpf_iter_create".as_ptr()) {
        bpf_iter_setsockopt_unix__destroy(skel);
        return;
    }

    loop {
        err = libc::read(
            iter_fd,
            &mut buf as *mut c_char as *mut c_void,
            mem::size_of_val(&buf),
        ) as c_int;
        if !(err == -1 && *libc::__errno_location() == libc::EAGAIN) {
            break;
        }
    }
    if !ASSERT_OK(err, c"read iter error".as_ptr()) {
        bpf_iter_setsockopt_unix__destroy(skel);
        return;
    }

    test_sndbuf(skel, unix_fd);

    bpf_iter_setsockopt_unix__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
