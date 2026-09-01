// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */
/* Original C dependencies:
 * #define _GNU_SOURCE
 * #include <sched.h>
 * #include <test_progs.h>
 * #include "network_helpers.h"
 * #include "bpf_dctcp.skel.h"
 * #include "bpf_cubic.skel.h"
 * #include "bpf_iter_setsockopt.skel.h"
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

type socklen_t = u32;
type size_t = usize;

const CLONE_NEWNET: c_int = 0x40000000;
const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const SOL_TCP: c_int = 6;
const TCP_CONGESTION: c_int = 13;
const EAGAIN: c_int = 11;

#[repr(C)]
pub struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
pub struct in6_addr {
    s6_addr: [u8; 16],
}

#[repr(C)]
pub struct sockaddr_in6 {
    sin6_family: u16,
    sin6_port: u16,
    sin6_flowinfo: u32,
    sin6_addr: in6_addr,
    sin6_scope_id: u32,
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_iter_setsockopt_bss {
    listen_hport: u16,
    reuse_listen_hport: u16,
    random_retry: bool,
}

#[repr(C)]
pub struct bpf_iter_setsockopt_links {
    change_tcp_cc: *mut bpf_link,
}

#[repr(C)]
pub struct bpf_iter_setsockopt_progs {
    change_tcp_cc: *mut bpf_program,
}

#[repr(C)]
pub struct bpf_iter_setsockopt {
    bss: *mut bpf_iter_setsockopt_bss,
    links: bpf_iter_setsockopt_links,
    progs: bpf_iter_setsockopt_progs,
}

#[repr(C)]
pub struct bpf_cubic_maps {
    cubic: *mut bpf_map,
}

#[repr(C)]
pub struct bpf_cubic {
    maps: bpf_cubic_maps,
}

#[repr(C)]
pub struct bpf_dctcp_maps {
    dctcp: *mut bpf_map,
}

#[repr(C)]
pub struct bpf_dctcp {
    maps: bpf_dctcp_maps,
}

unsafe extern "C" {
    static mut errno: c_int;

    fn accept(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn bpf_cubic__destroy(obj: *mut bpf_cubic);
    fn bpf_cubic__open_and_load() -> *mut bpf_cubic;
    fn bpf_dctcp__destroy(obj: *mut bpf_dctcp);
    fn bpf_dctcp__open_and_load() -> *mut bpf_dctcp;
    fn bpf_iter_create(link_fd: c_int) -> c_int;
    fn bpf_iter_setsockopt__destroy(obj: *mut bpf_iter_setsockopt);
    fn bpf_iter_setsockopt__open_and_load() -> *mut bpf_iter_setsockopt;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_link__fd(link: *mut bpf_link) -> c_int;
    fn bpf_map__attach_struct_ops(map: *mut bpf_map) -> *mut bpf_link;
    fn bpf_program__attach_iter(
        prog: *mut bpf_program,
        opts: *const c_void,
    ) -> *mut bpf_link;
    fn close(fd: c_int) -> c_int;
    fn connect_to_fd(fd: c_int, timeout_ms: c_uint) -> c_int;
    fn free(ptr: *mut c_void);
    fn free_fds(fds: *mut c_int, nr_fds: c_uint);
    fn getsockname(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn getsockopt(
        sockfd: c_int,
        level: c_int,
        optname: c_int,
        optval: *mut c_void,
        optlen: *mut socklen_t,
    ) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn ntohs(netshort: u16) -> u16;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> isize;
    fn setsockopt(
        sockfd: c_int,
        level: c_int,
        optname: c_int,
        optval: *const c_void,
        optlen: socklen_t,
    ) -> c_int;
    fn start_reuseport_server(
        family: c_int,
        type_: c_int,
        addr: *const c_char,
        port: u16,
        timeout_ms: c_int,
        nr_listens: c_uint,
    ) -> *mut c_int;
    fn start_server(
        family: c_int,
        type_: c_int,
        addr: *const c_char,
        port: u16,
        timeout_ms: c_int,
    ) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn system(command: *const c_char) -> c_int;
    fn unshare(flags: c_int) -> c_int;

    fn ASSERT_EQ(actual: c_uint, expected: c_uint, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_NEQ(actual: u16, expected: u16, name: *const c_char) -> bool;
    fn ASSERT_OK(res: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
}

unsafe fn create_netns() -> c_int {
    if !ASSERT_OK(unshare(CLONE_NEWNET), c"create netns".as_ptr()) {
        return -1;
    }

    if !ASSERT_OK(
        system(c"ip link set dev lo up".as_ptr()),
        c"bring up lo".as_ptr(),
    ) {
        return -1;
    }

    0
}

unsafe fn set_bpf_cubic(fds: *mut c_int, nr_fds: c_uint) -> c_uint {
    let mut i: c_uint = 0;

    while i < nr_fds {
        if setsockopt(
            *fds.add(i as usize),
            SOL_TCP,
            TCP_CONGESTION,
            c"bpf_cubic".as_ptr() as *const c_void,
            size_of_val(c"bpf_cubic".to_bytes_with_nul()) as socklen_t,
        ) != 0
        {
            return i;
        }
        i += 1;
    }

    nr_fds
}

unsafe fn check_bpf_dctcp(fds: *mut c_int, nr_fds: c_uint) -> c_uint {
    let mut tcp_cc: [c_char; 16] = [0; 16];
    let mut optlen: socklen_t = size_of_val(&tcp_cc) as socklen_t;
    let mut i: c_uint = 0;

    while i < nr_fds {
        if getsockopt(
            *fds.add(i as usize),
            SOL_TCP,
            TCP_CONGESTION,
            tcp_cc.as_mut_ptr() as *mut c_void,
            &mut optlen,
        ) != 0
            || strcmp(tcp_cc.as_ptr(), c"bpf_dctcp".as_ptr()) != 0
        {
            return i;
        }
        i += 1;
    }

    nr_fds
}

unsafe fn make_established(
    listen_fd: c_int,
    nr_est: c_uint,
    paccepted_fds: *mut *mut c_int,
) -> *mut c_int {
    let est_fds: *mut c_int;
    let accepted_fds: *mut c_int;
    let mut i: c_uint;

    est_fds = malloc(size_of::<c_int>() * nr_est as usize) as *mut c_int;
    if est_fds.is_null() {
        return core::ptr::null_mut();
    }

    accepted_fds = malloc(size_of::<c_int>() * nr_est as usize) as *mut c_int;
    if accepted_fds.is_null() {
        free(est_fds as *mut c_void);
        return core::ptr::null_mut();
    }

    i = 0;
    while i < nr_est {
        *est_fds.add(i as usize) = connect_to_fd(listen_fd, 0);
        if *est_fds.add(i as usize) == -1 {
            break;
        }
        if set_bpf_cubic(est_fds.add(i as usize), 1) != 1 {
            close(*est_fds.add(i as usize));
            break;
        }

        *accepted_fds.add(i as usize) = accept(listen_fd, core::ptr::null_mut(), core::ptr::null_mut());
        if *accepted_fds.add(i as usize) == -1 {
            close(*est_fds.add(i as usize));
            break;
        }

        i += 1;
    }

    if !ASSERT_EQ(i, nr_est, c"create established fds".as_ptr()) {
        free_fds(accepted_fds, i);
        free_fds(est_fds, i);
        return core::ptr::null_mut();
    }

    *paccepted_fds = accepted_fds;
    est_fds
}

unsafe fn get_local_port(fd: c_int) -> u16 {
    let mut addr: sockaddr_in6 = core::mem::zeroed();
    let mut addrlen: socklen_t = size_of_val(&addr) as socklen_t;

    if getsockname(
        fd,
        &mut addr as *mut sockaddr_in6 as *mut sockaddr,
        &mut addrlen,
    ) == 0
    {
        return ntohs(addr.sin6_port);
    }

    0
}

unsafe fn do_bpf_iter_setsockopt(iter_skel: *mut bpf_iter_setsockopt, random_retry: bool) {
    let mut reuse_listen_fds: *mut c_int = core::ptr::null_mut();
    let mut accepted_fds: *mut c_int = core::ptr::null_mut();
    let mut est_fds: *mut c_int = core::ptr::null_mut();
    let nr_reuse_listens: c_uint = 256;
    let nr_est: c_uint = 256;
    let mut err: c_int;
    let mut iter_fd: c_int = -1;
    let mut listen_fd: c_int = -1;
    let mut buf: c_char = 0;

    /* Prepare non-reuseport listen_fd */
    listen_fd = start_server(AF_INET6, SOCK_STREAM, c"::1".as_ptr(), 0, 0);
    if !ASSERT_GE(listen_fd, 0, c"start_server".as_ptr()) {
        return;
    }
    if !ASSERT_EQ(
        set_bpf_cubic(&mut listen_fd, 1),
        1,
        c"set listen_fd to cubic".as_ptr(),
    ) {
        goto_done(iter_fd, listen_fd, reuse_listen_fds, nr_reuse_listens, accepted_fds, nr_est, est_fds);
        return;
    }
    (*(*iter_skel).bss).listen_hport = get_local_port(listen_fd);
    if !ASSERT_NEQ(
        (*(*iter_skel).bss).listen_hport,
        0,
        c"get_local_port(listen_fd)".as_ptr(),
    ) {
        goto_done(iter_fd, listen_fd, reuse_listen_fds, nr_reuse_listens, accepted_fds, nr_est, est_fds);
        return;
    }

    /* Connect to non-reuseport listen_fd */
    est_fds = make_established(listen_fd, nr_est, &mut accepted_fds);
    if !ASSERT_OK_PTR(est_fds as *const c_void, c"create established".as_ptr()) {
        goto_done(iter_fd, listen_fd, reuse_listen_fds, nr_reuse_listens, accepted_fds, nr_est, est_fds);
        return;
    }

    /* Prepare reuseport listen fds */
    reuse_listen_fds = start_reuseport_server(
        AF_INET6,
        SOCK_STREAM,
        c"::1".as_ptr(),
        0,
        0,
        nr_reuse_listens,
    );
    if !ASSERT_OK_PTR(
        reuse_listen_fds as *const c_void,
        c"start_reuseport_server".as_ptr(),
    ) {
        goto_done(iter_fd, listen_fd, reuse_listen_fds, nr_reuse_listens, accepted_fds, nr_est, est_fds);
        return;
    }
    if !ASSERT_EQ(
        set_bpf_cubic(reuse_listen_fds, nr_reuse_listens),
        nr_reuse_listens,
        c"set reuse_listen_fds to cubic".as_ptr(),
    ) {
        goto_done(iter_fd, listen_fd, reuse_listen_fds, nr_reuse_listens, accepted_fds, nr_est, est_fds);
        return;
    }
    (*(*iter_skel).bss).reuse_listen_hport = get_local_port(*reuse_listen_fds.add(0));
    if !ASSERT_NEQ(
        (*(*iter_skel).bss).reuse_listen_hport,
        0,
        c"get_local_port(reuse_listen_fds[0])".as_ptr(),
    ) {
        goto_done(iter_fd, listen_fd, reuse_listen_fds, nr_reuse_listens, accepted_fds, nr_est, est_fds);
        return;
    }

    /* Run bpf tcp iter to switch from bpf_cubic to bpf_dctcp */
    (*(*iter_skel).bss).random_retry = random_retry;
    iter_fd = bpf_iter_create(bpf_link__fd((*iter_skel).links.change_tcp_cc));
    if !ASSERT_GE(iter_fd, 0, c"create iter_fd".as_ptr()) {
        goto_done(iter_fd, listen_fd, reuse_listen_fds, nr_reuse_listens, accepted_fds, nr_est, est_fds);
        return;
    }

    loop {
        err = read(iter_fd, &mut buf as *mut c_char as *mut c_void, size_of_val(&buf)) as c_int;
        if !(err == -1 && errno == EAGAIN) {
            break;
        }
    }
    if !ASSERT_OK(err, c"read iter error".as_ptr()) {
        goto_done(iter_fd, listen_fd, reuse_listen_fds, nr_reuse_listens, accepted_fds, nr_est, est_fds);
        return;
    }

    /* Check reuseport listen fds for dctcp */
    ASSERT_EQ(
        check_bpf_dctcp(reuse_listen_fds, nr_reuse_listens),
        nr_reuse_listens,
        c"check reuse_listen_fds dctcp".as_ptr(),
    );

    /* Check non reuseport listen fd for dctcp */
    ASSERT_EQ(
        check_bpf_dctcp(&mut listen_fd, 1),
        1,
        c"check listen_fd dctcp".as_ptr(),
    );

    /* Check established fds for dctcp */
    ASSERT_EQ(
        check_bpf_dctcp(est_fds, nr_est),
        nr_est,
        c"check est_fds dctcp".as_ptr(),
    );

    /* Check accepted fds for dctcp */
    ASSERT_EQ(
        check_bpf_dctcp(accepted_fds, nr_est),
        nr_est,
        c"check accepted_fds dctcp".as_ptr(),
    );

    goto_done(iter_fd, listen_fd, reuse_listen_fds, nr_reuse_listens, accepted_fds, nr_est, est_fds);
}

unsafe fn goto_done(
    iter_fd: c_int,
    listen_fd: c_int,
    reuse_listen_fds: *mut c_int,
    nr_reuse_listens: c_uint,
    accepted_fds: *mut c_int,
    nr_est: c_uint,
    est_fds: *mut c_int,
) {
    if iter_fd != -1 {
        close(iter_fd);
    }
    if listen_fd != -1 {
        close(listen_fd);
    }
    free_fds(reuse_listen_fds, nr_reuse_listens);
    free_fds(accepted_fds, nr_est);
    free_fds(est_fds, nr_est);
}

#[no_mangle]
pub unsafe extern "C" fn serial_test_bpf_iter_setsockopt() {
    let mut iter_skel: *mut bpf_iter_setsockopt = core::ptr::null_mut();
    let mut cubic_skel: *mut bpf_cubic = core::ptr::null_mut();
    let mut dctcp_skel: *mut bpf_dctcp = core::ptr::null_mut();
    let mut cubic_link: *mut bpf_link = core::ptr::null_mut();
    let mut dctcp_link: *mut bpf_link = core::ptr::null_mut();

    if create_netns() != 0 {
        return;
    }

    /* Load iter_skel */
    iter_skel = bpf_iter_setsockopt__open_and_load();
    if !ASSERT_OK_PTR(iter_skel as *const c_void, c"iter_skel".as_ptr()) {
        return;
    }
    (*iter_skel).links.change_tcp_cc =
        bpf_program__attach_iter((*iter_skel).progs.change_tcp_cc, core::ptr::null());
    if !ASSERT_OK_PTR(
        (*iter_skel).links.change_tcp_cc as *const c_void,
        c"attach iter".as_ptr(),
    ) {
        goto_serial_done(cubic_link, dctcp_link, cubic_skel, dctcp_skel, iter_skel);
        return;
    }

    /* Load bpf_cubic */
    cubic_skel = bpf_cubic__open_and_load();
    if !ASSERT_OK_PTR(cubic_skel as *const c_void, c"cubic_skel".as_ptr()) {
        goto_serial_done(cubic_link, dctcp_link, cubic_skel, dctcp_skel, iter_skel);
        return;
    }
    cubic_link = bpf_map__attach_struct_ops((*cubic_skel).maps.cubic);
    if !ASSERT_OK_PTR(cubic_link as *const c_void, c"cubic_link".as_ptr()) {
        goto_serial_done(cubic_link, dctcp_link, cubic_skel, dctcp_skel, iter_skel);
        return;
    }

    /* Load bpf_dctcp */
    dctcp_skel = bpf_dctcp__open_and_load();
    if !ASSERT_OK_PTR(dctcp_skel as *const c_void, c"dctcp_skel".as_ptr()) {
        goto_serial_done(cubic_link, dctcp_link, cubic_skel, dctcp_skel, iter_skel);
        return;
    }
    dctcp_link = bpf_map__attach_struct_ops((*dctcp_skel).maps.dctcp);
    if !ASSERT_OK_PTR(dctcp_link as *const c_void, c"dctcp_link".as_ptr()) {
        goto_serial_done(cubic_link, dctcp_link, cubic_skel, dctcp_skel, iter_skel);
        return;
    }

    do_bpf_iter_setsockopt(iter_skel, true);
    do_bpf_iter_setsockopt(iter_skel, false);

    goto_serial_done(cubic_link, dctcp_link, cubic_skel, dctcp_skel, iter_skel);
}

unsafe fn goto_serial_done(
    cubic_link: *mut bpf_link,
    dctcp_link: *mut bpf_link,
    cubic_skel: *mut bpf_cubic,
    dctcp_skel: *mut bpf_dctcp,
    iter_skel: *mut bpf_iter_setsockopt,
) {
    bpf_link__destroy(cubic_link);
    bpf_link__destroy(dctcp_link);
    bpf_cubic__destroy(cubic_skel);
    bpf_dctcp__destroy(dctcp_skel);
    bpf_iter_setsockopt__destroy(iter_skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
