// SPDX-License-Identifier: GPL-2.0

/* Test that sockets listening on a specific address are preferred
 * over sockets listening on addr_any.
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const SOCK_DGRAM: c_int = 2;
const SOL_SOCKET: c_int = 1;
const SO_REUSEPORT: c_int = 15;
const INADDR_ANY: u32 = 0;
const EPOLLIN: u32 = 0x001;
const EPOLL_CTL_ADD: c_int = 1;

static IP4_ADDR: &[u8] = b"127.0.0.1\0";
static IP6_ADDR: &[u8] = b"::1\0";
static IP4_MAPPED6: &[u8] = b"::ffff:127.0.0.1\0";

static PORT: c_int = 8888;

#[repr(C)]
#[derive(Copy, Clone)]
struct in_addr {
    s_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct in6_addr {
    s6_addr: [u8; 16],
}

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct sockaddr_in {
    sin_family: u16,
    sin_port: u16,
    sin_addr: in_addr,
    sin_zero: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct sockaddr_in6 {
    sin6_family: u16,
    sin6_port: u16,
    sin6_flowinfo: u32,
    sin6_addr: in6_addr,
    sin6_scope_id: u32,
}

#[repr(C)]
union epoll_data {
    ptr: *mut c_void,
    fd: c_int,
    u32_: u32,
    u64_: u64,
}

#[repr(C, packed)]
struct epoll_event {
    events: u32,
    data: epoll_data,
}

unsafe extern "C" {
    static in6addr_any: in6_addr;
    static mut stderr: *mut c_void;

    fn __errno_location() -> *mut c_int;
    fn accept(fd: c_int, addr: *mut sockaddr, addr_len: *mut u32) -> c_int;
    fn bind(fd: c_int, addr: *const sockaddr, len: u32) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn connect(fd: c_int, addr: *const sockaddr, len: u32) -> c_int;
    fn epoll_create(size: c_int) -> c_int;
    fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut epoll_event) -> c_int;
    fn epoll_wait(epfd: c_int, events: *mut epoll_event, maxevents: c_int, timeout: c_int)
        -> c_int;
    fn error(status: c_int, errnum: c_int, format: *const c_char, ...) -> ();
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn htonl(hostlong: c_uint) -> c_uint;
    fn htons(hostshort: u16) -> u16;
    fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
    fn listen(sockfd: c_int, backlog: c_int) -> c_int;
    fn recv(sockfd: c_int, buf: *mut c_void, len: usize, flags: c_int) -> isize;
    fn send(sockfd: c_int, buf: *const c_void, len: usize, flags: c_int) -> isize;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: u32,
    ) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
}

unsafe fn errno() -> c_int {
    *__errno_location()
}

unsafe fn build_rcv_fd(
    family: c_int,
    proto: c_int,
    rcv_fds: *mut c_int,
    count: c_int,
    addr_str: *const c_char,
) {
    let mut addr4: sockaddr_in = mem::zeroed();
    let mut addr6: sockaddr_in6 = mem::zeroed();
    let mut addr: *mut sockaddr;
    let mut opt: c_int;
    let mut i: c_int;
    let sz: u32;

    ptr::write_bytes(
        &mut addr as *mut *mut sockaddr as *mut c_void,
        0,
        mem::size_of::<*mut sockaddr>(),
    );

    match family {
        AF_INET => {
            addr4.sin_family = family as u16;
            if addr_str.is_null() {
                addr4.sin_addr.s_addr = htonl(INADDR_ANY);
            } else if inet_pton(
                family,
                addr_str,
                &mut addr4.sin_addr.s_addr as *mut u32 as *mut c_void,
            ) == 0
            {
                error(
                    1,
                    errno(),
                    b"inet_pton failed: %s\0".as_ptr() as *const c_char,
                    addr_str,
                );
            }
            addr4.sin_port = htons(PORT as u16);
            sz = mem::size_of::<sockaddr_in>() as u32;
            addr = &mut addr4 as *mut sockaddr_in as *mut sockaddr;
        }
        AF_INET6 => {
            addr6.sin6_family = AF_INET6 as u16;
            if addr_str.is_null() {
                addr6.sin6_addr = in6addr_any;
            } else if inet_pton(
                family,
                addr_str,
                &mut addr6.sin6_addr as *mut in6_addr as *mut c_void,
            ) == 0
            {
                error(
                    1,
                    errno(),
                    b"inet_pton failed: %s\0".as_ptr() as *const c_char,
                    addr_str,
                );
            }
            addr6.sin6_port = htons(PORT as u16);
            sz = mem::size_of::<sockaddr_in6>() as u32;
            addr = &mut addr6 as *mut sockaddr_in6 as *mut sockaddr;
        }
        _ => {
            error(
                1,
                0,
                b"Unsupported family %d\0".as_ptr() as *const c_char,
                family,
            );
            /* clang does not recognize error() above as terminating
             * the program, so it complains that saddr, sz are
             * not initialized when this code path is taken. Silence it.
             */
            return;
        }
    }

    i = 0;
    while i < count {
        *rcv_fds.offset(i as isize) = socket(family, proto, 0);
        if *rcv_fds.offset(i as isize) < 0 {
            error(
                1,
                errno(),
                b"failed to create receive socket\0".as_ptr() as *const c_char,
            );
        }

        opt = 1;
        if setsockopt(
            *rcv_fds.offset(i as isize),
            SOL_SOCKET,
            SO_REUSEPORT,
            &opt as *const c_int as *const c_void,
            mem::size_of::<c_int>() as u32,
        ) != 0
        {
            error(
                1,
                errno(),
                b"failed to set SO_REUSEPORT\0".as_ptr() as *const c_char,
            );
        }

        if bind(*rcv_fds.offset(i as isize), addr, sz) != 0 {
            error(
                1,
                errno(),
                b"failed to bind receive socket\0".as_ptr() as *const c_char,
            );
        }

        if proto == SOCK_STREAM && listen(*rcv_fds.offset(i as isize), 10) != 0 {
            error(
                1,
                errno(),
                b"tcp: failed to listen on receive port\0".as_ptr() as *const c_char,
            );
        }

        i += 1;
    }
}

unsafe fn connect_and_send(family: c_int, proto: c_int) -> c_int {
    let mut saddr4: sockaddr_in = mem::zeroed();
    let mut daddr4: sockaddr_in = mem::zeroed();
    let mut saddr6: sockaddr_in6 = mem::zeroed();
    let mut daddr6: sockaddr_in6 = mem::zeroed();
    let saddr: *mut sockaddr;
    let daddr: *mut sockaddr;
    let fd: c_int;
    let sz: u32;

    match family {
        AF_INET => {
            saddr4.sin_family = AF_INET as u16;
            saddr4.sin_addr.s_addr = htonl(INADDR_ANY);
            saddr4.sin_port = 0;

            daddr4.sin_family = AF_INET as u16;
            if inet_pton(
                family,
                IP4_ADDR.as_ptr() as *const c_char,
                &mut daddr4.sin_addr.s_addr as *mut u32 as *mut c_void,
            ) == 0
            {
                error(
                    1,
                    errno(),
                    b"inet_pton failed: %s\0".as_ptr() as *const c_char,
                    IP4_ADDR.as_ptr() as *const c_char,
                );
            }
            daddr4.sin_port = htons(PORT as u16);

            sz = mem::size_of::<sockaddr_in>() as u32;
            saddr = &mut saddr4 as *mut sockaddr_in as *mut sockaddr;
            daddr = &mut daddr4 as *mut sockaddr_in as *mut sockaddr;
        }
        AF_INET6 => {
            saddr6.sin6_family = AF_INET6 as u16;
            saddr6.sin6_addr = in6addr_any;

            daddr6.sin6_family = AF_INET6 as u16;
            if inet_pton(
                family,
                IP6_ADDR.as_ptr() as *const c_char,
                &mut daddr6.sin6_addr as *mut in6_addr as *mut c_void,
            ) == 0
            {
                error(
                    1,
                    errno(),
                    b"inet_pton failed: %s\0".as_ptr() as *const c_char,
                    IP6_ADDR.as_ptr() as *const c_char,
                );
            }
            daddr6.sin6_port = htons(PORT as u16);

            sz = mem::size_of::<sockaddr_in6>() as u32;
            saddr = &mut saddr6 as *mut sockaddr_in6 as *mut sockaddr;
            daddr = &mut daddr6 as *mut sockaddr_in6 as *mut sockaddr;
        }
        _ => {
            error(
                1,
                0,
                b"Unsupported family %d\0".as_ptr() as *const c_char,
                family,
            );
            /* clang does not recognize error() above as terminating
             * the program, so it complains that saddr, daddr, sz are
             * not initialized when this code path is taken. Silence it.
             */
            return -1;
        }
    }

    fd = socket(family, proto, 0);
    if fd < 0 {
        error(
            1,
            errno(),
            b"failed to create send socket\0".as_ptr() as *const c_char,
        );
    }

    if bind(fd, saddr, sz) != 0 {
        error(
            1,
            errno(),
            b"failed to bind send socket\0".as_ptr() as *const c_char,
        );
    }

    if connect(fd, daddr, sz) != 0 {
        error(
            1,
            errno(),
            b"failed to connect send socket\0".as_ptr() as *const c_char,
        );
    }

    if send(fd, b"a\0".as_ptr() as *const c_void, 1, 0) < 0 {
        error(
            1,
            errno(),
            b"failed to send message\0".as_ptr() as *const c_char,
        );
    }

    fd
}

unsafe fn receive_once(epfd: c_int, proto: c_int) -> c_int {
    let mut ev: epoll_event = mem::zeroed();
    let mut i: c_int;
    let fd: c_int;
    let mut buf: [c_char; 8] = [0; 8];

    i = epoll_wait(epfd, &mut ev, 1, 3);
    if i < 0 {
        error(
            1,
            errno(),
            b"epoll_wait failed\0".as_ptr() as *const c_char,
        );
    }

    if proto == SOCK_STREAM {
        fd = accept(ev.data.fd, ptr::null_mut(), ptr::null_mut());
        if fd < 0 {
            error(1, errno(), b"failed to accept\0".as_ptr() as *const c_char);
        }
        i = recv(fd, buf.as_mut_ptr() as *mut c_void, mem::size_of_val(&buf), 0) as c_int;
        close(fd);
    } else {
        i = recv(
            ev.data.fd,
            buf.as_mut_ptr() as *mut c_void,
            mem::size_of_val(&buf),
            0,
        ) as c_int;
    }

    if i < 0 {
        error(1, errno(), b"failed to recv\0".as_ptr() as *const c_char);
    }

    ev.data.fd
}

unsafe fn test(rcv_fds: *mut c_int, count: c_int, family: c_int, proto: c_int, fd: c_int) {
    let mut ev: epoll_event = mem::zeroed();
    let epfd: c_int;
    let mut i: c_int;
    let send_fd: c_int;
    let recv_fd: c_int;

    epfd = epoll_create(1);
    if epfd < 0 {
        error(
            1,
            errno(),
            b"failed to create epoll\0".as_ptr() as *const c_char,
        );
    }

    ev.events = EPOLLIN;
    i = 0;
    while i < count {
        ev.data.fd = *rcv_fds.offset(i as isize);
        if epoll_ctl(epfd, EPOLL_CTL_ADD, *rcv_fds.offset(i as isize), &mut ev) != 0 {
            error(
                1,
                errno(),
                b"failed to register sock epoll\0".as_ptr() as *const c_char,
            );
        }
        i += 1;
    }

    send_fd = connect_and_send(family, proto);

    recv_fd = receive_once(epfd, proto);
    if recv_fd != fd {
        error(
            1,
            0,
            b"received on an unexpected socket\0".as_ptr() as *const c_char,
        );
    }

    close(send_fd);
    close(epfd);
}

unsafe fn run_one_test(fam_send: c_int, fam_rcv: c_int, proto: c_int, addr_str: *const c_char) {
    /* Below we test that a socket listening on a specific address
     * is always selected in preference over a socket listening
     * on addr_any. Bugs where this is not the case often result
     * in sockets created first or last to get picked. So below
     * we make sure that there are always addr_any sockets created
     * before and after a specific socket is created.
     */
    let mut rcv_fds: [c_int; 10] = [0; 10];
    let mut i: c_int;

    build_rcv_fd(AF_INET, proto, rcv_fds.as_mut_ptr(), 2, ptr::null());
    build_rcv_fd(AF_INET6, proto, rcv_fds.as_mut_ptr().offset(2), 2, ptr::null());
    build_rcv_fd(fam_rcv, proto, rcv_fds.as_mut_ptr().offset(4), 1, addr_str);
    build_rcv_fd(AF_INET, proto, rcv_fds.as_mut_ptr().offset(5), 2, ptr::null());
    build_rcv_fd(AF_INET6, proto, rcv_fds.as_mut_ptr().offset(7), 2, ptr::null());
    test(rcv_fds.as_mut_ptr(), 9, fam_send, proto, rcv_fds[4]);
    i = 0;
    while i < 9 {
        close(rcv_fds[i as usize]);
        i += 1;
    }
    fprintf(stderr, b"pass\n\0".as_ptr() as *const c_char);
}

unsafe fn test_proto(proto: c_int, proto_str: *const c_char) {
    fprintf(
        stderr,
        b"%s IPv4 ... \0".as_ptr() as *const c_char,
        proto_str,
    );
    run_one_test(AF_INET, AF_INET, proto, IP4_ADDR.as_ptr() as *const c_char);

    fprintf(
        stderr,
        b"%s IPv6 ... \0".as_ptr() as *const c_char,
        proto_str,
    );
    run_one_test(AF_INET6, AF_INET6, proto, IP6_ADDR.as_ptr() as *const c_char);

    fprintf(
        stderr,
        b"%s IPv4 mapped to IPv6 ... \0".as_ptr() as *const c_char,
        proto_str,
    );
    run_one_test(
        AF_INET,
        AF_INET6,
        proto,
        IP4_MAPPED6.as_ptr() as *const c_char,
    );
}

fn main() {
    unsafe {
        test_proto(SOCK_DGRAM, b"UDP\0".as_ptr() as *const c_char);
        test_proto(SOCK_STREAM, b"TCP\0".as_ptr() as *const c_char);

        fprintf(stderr, b"SUCCESS\n\0".as_ptr() as *const c_char);
    }
}
