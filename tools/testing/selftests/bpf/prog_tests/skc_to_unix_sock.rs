/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2021 Hengqi Chen */

/* Translated from testing/selftests/bpf/prog_tests/skc_to_unix_sock.c. */
/* C dependencies: test_progs.h, sys/un.h, test_skc_to_unix_sock.skel.h. */

use core::ffi::{c_char, c_int, c_void};
use core::mem;

type SaFamilyT = u16;

#[repr(C)]
pub struct sockaddr {
    pub sa_family: SaFamilyT,
    pub sa_data: [c_char; 14],
}

#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: SaFamilyT,
    pub sun_path: [c_char; 108],
}

#[repr(C)]
pub struct test_skc_to_unix_sock_rodata {
    pub my_pid: c_int,
}

#[repr(C)]
pub struct test_skc_to_unix_sock_bss {
    pub path: *const c_char,
}

#[repr(C)]
pub struct test_skc_to_unix_sock {
    pub rodata: *mut test_skc_to_unix_sock_rodata,
    pub bss: *mut test_skc_to_unix_sock_bss,
}

unsafe extern "C" {
    static AF_UNIX: c_int;
    static SOCK_STREAM: c_int;

    fn test_skc_to_unix_sock__open() -> *mut test_skc_to_unix_sock;
    fn test_skc_to_unix_sock__load(skel: *mut test_skc_to_unix_sock) -> c_int;
    fn test_skc_to_unix_sock__attach(skel: *mut test_skc_to_unix_sock) -> c_int;
    fn test_skc_to_unix_sock__destroy(skel: *mut test_skc_to_unix_sock);

    fn ASSERT_OK_PTR(ptr: *const c_void, text: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, text: *const c_char) -> bool;
    fn ASSERT_GT(value: c_int, expected: c_int, text: *const c_char) -> bool;
    fn ASSERT_EQ(left: c_int, right: c_int, text: *const c_char) -> bool;

    fn getpid() -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: u32) -> c_int;
    fn listen(sockfd: c_int, backlog: c_int) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn close(fd: c_int) -> c_int;
}

static SOCK_PATH: &[u8; 18] = b"@skc_to_unix_sock\0";

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_skc_to_unix_sock() {
    let mut skel: *mut test_skc_to_unix_sock;
    let mut sockaddr: sockaddr_un = mem::zeroed();
    let mut err: c_int;
    let mut sockfd: c_int = 0;

    skel = test_skc_to_unix_sock__open();
    if !ASSERT_OK_PTR(
        skel as *const c_void,
        c"could not open BPF object".as_ptr(),
    ) {
        return;
    }

    (*(*skel).rodata).my_pid = getpid();

    err = test_skc_to_unix_sock__load(skel);
    if !ASSERT_OK(err, c"could not load BPF object".as_ptr()) {
        goto_cleanup(sockfd, skel);
        return;
    }

    err = test_skc_to_unix_sock__attach(skel);
    if !ASSERT_OK(err, c"could not attach BPF object".as_ptr()) {
        goto_cleanup(sockfd, skel);
        return;
    }

    /* trigger unix_listen */
    sockfd = socket(AF_UNIX, SOCK_STREAM, 0);
    if !ASSERT_GT(sockfd, 0, c"socket failed".as_ptr()) {
        goto_cleanup(sockfd, skel);
        return;
    }

    memset(
        &mut sockaddr as *mut sockaddr_un as *mut c_void,
        0,
        mem::size_of::<sockaddr_un>(),
    );
    sockaddr.sun_family = AF_UNIX as SaFamilyT;
    strscpy(sockaddr.sun_path.as_mut_ptr(), SOCK_PATH.as_ptr() as *const c_char);
    sockaddr.sun_path[0] = 0;

    err = bind(
        sockfd,
        &sockaddr as *const sockaddr_un as *const sockaddr,
        mem::size_of::<sockaddr_un>() as u32,
    );
    if !ASSERT_OK(err, c"bind failed".as_ptr()) {
        goto_cleanup(sockfd, skel);
        return;
    }

    err = listen(sockfd, 1);
    if !ASSERT_OK(err, c"listen failed".as_ptr()) {
        goto_cleanup(sockfd, skel);
        return;
    }

    ASSERT_EQ(
        strcmp((*(*skel).bss).path, SOCK_PATH.as_ptr() as *const c_char),
        0,
        c"bpf_skc_to_unix_sock failed".as_ptr(),
    );

    goto_cleanup(sockfd, skel);
}

unsafe fn goto_cleanup(sockfd: c_int, skel: *mut test_skc_to_unix_sock) {
    if sockfd != 0 {
        close(sockfd);
    }
    test_skc_to_unix_sock__destroy(skel);
}
