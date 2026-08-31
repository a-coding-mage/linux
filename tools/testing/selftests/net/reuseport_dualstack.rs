// SPDX-License-Identifier: GPL-2.0
/*
 * It is possible to use SO_REUSEPORT to open multiple sockets bound to
 * equivalent local addresses using AF_INET and AF_INET6 at the same time.  If
 * the AF_INET6 socket has IPV6_V6ONLY set, it's clear which socket should
 * receive a given incoming packet.  However, when it is not set, incoming v4
 * packets should prefer the AF_INET socket(s).  This behavior was defined with
 * the original SO_REUSEPORT implementation, but broke with
 * e32ea7e74727 ("soreuseport: fast reuseport UDP socket selection")
 * This test creates these mixed AF_INET/AF_INET6 sockets and asserts the
 * AF_INET preference for v4 packets.
 */

use std::ffi::{c_char, c_int, c_uint, c_void};
use std::mem;
use std::ptr;

const PORT: c_int = 8888;

const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const SOCK_DGRAM: c_int = 2;
const SOL_SOCKET: c_int = 1;
const SO_REUSEPORT: c_int = 15;
const SO_DOMAIN: c_int = 39;
const INADDR_ANY: u32 = 0x00000000;
const INADDR_LOOPBACK: u32 = 0x7f000001;
const EPOLLIN: u32 = 0x001;
const EPOLL_CTL_ADD: c_int = 1;
const CLONE_NEWNET: c_int = 0x40000000;

type SocklenT = u32;
type SaFamilyT = u16;
type InPortT = u16;
type InAddrT = u32;

#[repr(C)]
struct in_addr {
    s_addr: InAddrT,
}

#[repr(C)]
struct in6_addr {
    s6_addr: [u8; 16],
}

#[repr(C)]
struct sockaddr {
    sa_family: SaFamilyT,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct sockaddr_in {
    sin_family: SaFamilyT,
    sin_port: InPortT,
    sin_addr: in_addr,
    sin_zero: [u8; 8],
}

#[repr(C)]
struct sockaddr_in6 {
    sin6_family: SaFamilyT,
    sin6_port: InPortT,
    sin6_flowinfo: u32,
    sin6_addr: in6_addr,
    sin6_scope_id: u32,
}

#[repr(C)]
struct sockaddr_storage {
    ss_family: SaFamilyT,
    __ss_padding: [u8; 118],
    __ss_align: u64,
}

#[repr(C)]
union epoll_data {
    ptr: *mut c_void,
    fd: c_int,
    u32_: u32,
    u64_: u64,
}

#[repr(C)]
struct epoll_event {
    events: u32,
    data: epoll_data,
}

unsafe extern "C" {
    static mut errno: c_int;
    static in6addr_any: in6_addr;
    static mut stderr: *mut c_void;

    fn accept(fd: c_int, addr: *mut sockaddr, addr_len: *mut SocklenT) -> c_int;
    fn bind(fd: c_int, addr: *const sockaddr, len: SocklenT) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn connect(fd: c_int, addr: *const sockaddr, len: SocklenT) -> c_int;
    fn epoll_create(size: c_int) -> c_int;
    fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut epoll_event) -> c_int;
    fn epoll_wait(epfd: c_int, events: *mut epoll_event, maxevents: c_int, timeout: c_int) -> c_int;
    fn error(status: c_int, errnum: c_int, format: *const c_char, ...);
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn getsockopt(
        fd: c_int,
        level: c_int,
        optname: c_int,
        optval: *mut c_void,
        optlen: *mut SocklenT,
    ) -> c_int;
    fn htonl(hostlong: c_uint) -> c_uint;
    fn htons(hostshort: u16) -> u16;
    fn listen(fd: c_int, backlog: c_int) -> c_int;
    fn recv(fd: c_int, buf: *mut c_void, len: usize, flags: c_int) -> isize;
    fn send(fd: c_int, buf: *const c_void, len: usize, flags: c_int) -> isize;
    fn setsockopt(
        fd: c_int,
        level: c_int,
        optname: c_int,
        optval: *const c_void,
        optlen: SocklenT,
    ) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn system(command: *const c_char) -> c_int;
    fn unshare(flags: c_int) -> c_int;
}

unsafe fn build_rcv_fd(family: c_int, proto: c_int, rcv_fds: *mut c_int, count: c_int) {
    let mut addr: sockaddr_storage = mem::zeroed();
    let mut opt: c_int;
    let mut i: c_int;

    match family {
        AF_INET => {
            let addr4 = &mut addr as *mut sockaddr_storage as *mut sockaddr_in;
            (*addr4).sin_family = AF_INET as SaFamilyT;
            (*addr4).sin_addr.s_addr = htonl(INADDR_ANY);
            (*addr4).sin_port = htons(PORT as u16);
        }
        AF_INET6 => {
            let addr6 = &mut addr as *mut sockaddr_storage as *mut sockaddr_in6;
            (*addr6).sin6_family = AF_INET6 as SaFamilyT;
            (*addr6).sin6_addr = in6addr_any;
            (*addr6).sin6_port = htons(PORT as u16);
        }
        _ => {
            error(
                1,
                0,
                b"Unsupported family %d\0".as_ptr() as *const c_char,
                family,
            );
        }
    }

    i = 0;
    while i < count {
        *rcv_fds.add(i as usize) = socket(family, proto, 0);
        if *rcv_fds.add(i as usize) < 0 {
            error(
                1,
                errno,
                b"failed to create receive socket\0".as_ptr() as *const c_char,
            );
        }

        opt = 1;
        if setsockopt(
            *rcv_fds.add(i as usize),
            SOL_SOCKET,
            SO_REUSEPORT,
            &opt as *const c_int as *const c_void,
            mem::size_of_val(&opt) as SocklenT,
        ) != 0
        {
            error(
                1,
                errno,
                b"failed to set SO_REUSEPORT\0".as_ptr() as *const c_char,
            );
        }

        if bind(
            *rcv_fds.add(i as usize),
            &addr as *const sockaddr_storage as *const sockaddr,
            mem::size_of_val(&addr) as SocklenT,
        ) != 0
        {
            error(
                1,
                errno,
                b"failed to bind receive socket\0".as_ptr() as *const c_char,
            );
        }

        if proto == SOCK_STREAM && listen(*rcv_fds.add(i as usize), 10) != 0 {
            error(
                1,
                errno,
                b"failed to listen on receive port\0".as_ptr() as *const c_char,
            );
        }

        i += 1;
    }
}

unsafe fn send_from_v4(proto: c_int) {
    let mut saddr: sockaddr_in = mem::zeroed();
    let mut daddr: sockaddr_in = mem::zeroed();
    let fd: c_int;

    saddr.sin_family = AF_INET as SaFamilyT;
    saddr.sin_addr.s_addr = htonl(INADDR_ANY);
    saddr.sin_port = 0;

    daddr.sin_family = AF_INET as SaFamilyT;
    daddr.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
    daddr.sin_port = htons(PORT as u16);

    fd = socket(AF_INET, proto, 0);
    if fd < 0 {
        error(
            1,
            errno,
            b"failed to create send socket\0".as_ptr() as *const c_char,
        );
    }

    if bind(
        fd,
        &saddr as *const sockaddr_in as *const sockaddr,
        mem::size_of_val(&saddr) as SocklenT,
    ) != 0
    {
        error(
            1,
            errno,
            b"failed to bind send socket\0".as_ptr() as *const c_char,
        );
    }

    if connect(
        fd,
        &daddr as *const sockaddr_in as *const sockaddr,
        mem::size_of_val(&daddr) as SocklenT,
    ) != 0
    {
        error(
            1,
            errno,
            b"failed to connect send socket\0".as_ptr() as *const c_char,
        );
    }

    if send(fd, b"a\0".as_ptr() as *const c_void, 1, 0) < 0 {
        error(1, errno, b"failed to send message\0".as_ptr() as *const c_char);
    }

    close(fd);
}

unsafe fn receive_once(epfd: c_int, proto: c_int) -> c_int {
    let mut ev: epoll_event = mem::zeroed();
    let mut i: isize;
    let fd: c_int;
    let mut buf = [0 as c_char; 8];

    i = epoll_wait(epfd, &mut ev, 1, -1) as isize;
    if i < 0 {
        error(1, errno, b"epoll_wait failed\0".as_ptr() as *const c_char);
    }

    if proto == SOCK_STREAM {
        fd = accept(ev.data.fd, ptr::null_mut(), ptr::null_mut());
        if fd < 0 {
            error(1, errno, b"failed to accept\0".as_ptr() as *const c_char);
        }
        i = recv(fd, buf.as_mut_ptr() as *mut c_void, mem::size_of_val(&buf), 0);
        close(fd);
    } else {
        i = recv(
            ev.data.fd,
            buf.as_mut_ptr() as *mut c_void,
            mem::size_of_val(&buf),
            0,
        );
    }

    if i < 0 {
        error(1, errno, b"failed to recv\0".as_ptr() as *const c_char);
    }

    ev.data.fd
}

unsafe fn test(rcv_fds: *mut c_int, count: c_int, proto: c_int) {
    let mut ev: epoll_event = mem::zeroed();
    let epfd: c_int;
    let mut i: c_int;
    let test_fd: c_int;
    let mut test_family: c_int = 0;
    let mut len: SocklenT;

    epfd = epoll_create(1);
    if epfd < 0 {
        error(1, errno, b"failed to create epoll\0".as_ptr() as *const c_char);
    }

    ev.events = EPOLLIN;
    i = 0;
    while i < count {
        ev.data.fd = *rcv_fds.add(i as usize);
        if epoll_ctl(epfd, EPOLL_CTL_ADD, *rcv_fds.add(i as usize), &mut ev) != 0 {
            error(
                1,
                errno,
                b"failed to register sock epoll\0".as_ptr() as *const c_char,
            );
        }
        i += 1;
    }

    send_from_v4(proto);

    test_fd = receive_once(epfd, proto);
    len = mem::size_of_val(&test_family) as SocklenT;
    if getsockopt(
        test_fd,
        SOL_SOCKET,
        SO_DOMAIN,
        &mut test_family as *mut c_int as *mut c_void,
        &mut len,
    ) != 0
    {
        error(
            1,
            errno,
            b"failed to read socket domain\0".as_ptr() as *const c_char,
        );
    }
    if test_family != AF_INET {
        error(
            1,
            0,
            b"expected to receive on v4 socket but got v6 (%d)\0".as_ptr() as *const c_char,
            test_family,
        );
    }

    close(epfd);
}

unsafe fn setup_netns() {
    if unshare(CLONE_NEWNET) != 0 {
        error(1, errno, b"failed to unshare netns\0".as_ptr() as *const c_char);
    }
    if system(b"ip link set lo up\0".as_ptr() as *const c_char) != 0 {
        error(
            1,
            0,
            b"failed to bring up lo interface in netns\0".as_ptr() as *const c_char,
        );
    }
}

fn main() {
    unsafe {
        let mut rcv_fds = [0 as c_int; 32];
        let mut i: c_int;

        setup_netns();

        fprintf(
            stderr,
            b"---- UDP IPv4 created before IPv6 ----\n\0".as_ptr() as *const c_char,
        );
        build_rcv_fd(AF_INET, SOCK_DGRAM, rcv_fds.as_mut_ptr(), 5);
        build_rcv_fd(AF_INET6, SOCK_DGRAM, rcv_fds.as_mut_ptr().add(5), 5);
        test(rcv_fds.as_mut_ptr(), 10, SOCK_DGRAM);
        i = 0;
        while i < 10 {
            close(rcv_fds[i as usize]);
            i += 1;
        }

        fprintf(
            stderr,
            b"---- UDP IPv6 created before IPv4 ----\n\0".as_ptr() as *const c_char,
        );
        build_rcv_fd(AF_INET6, SOCK_DGRAM, rcv_fds.as_mut_ptr(), 5);
        build_rcv_fd(AF_INET, SOCK_DGRAM, rcv_fds.as_mut_ptr().add(5), 5);
        test(rcv_fds.as_mut_ptr(), 10, SOCK_DGRAM);
        i = 0;
        while i < 10 {
            close(rcv_fds[i as usize]);
            i += 1;
        }

        /* NOTE: UDP socket lookups traverse a different code path when there
         * are > 10 sockets in a group.
         */
        fprintf(
            stderr,
            b"---- UDP IPv4 created before IPv6 (large) ----\n\0".as_ptr() as *const c_char,
        );
        build_rcv_fd(AF_INET, SOCK_DGRAM, rcv_fds.as_mut_ptr(), 16);
        build_rcv_fd(AF_INET6, SOCK_DGRAM, rcv_fds.as_mut_ptr().add(16), 16);
        test(rcv_fds.as_mut_ptr(), 32, SOCK_DGRAM);
        i = 0;
        while i < 32 {
            close(rcv_fds[i as usize]);
            i += 1;
        }

        fprintf(
            stderr,
            b"---- UDP IPv6 created before IPv4 (large) ----\n\0".as_ptr() as *const c_char,
        );
        build_rcv_fd(AF_INET6, SOCK_DGRAM, rcv_fds.as_mut_ptr(), 16);
        build_rcv_fd(AF_INET, SOCK_DGRAM, rcv_fds.as_mut_ptr().add(16), 16);
        test(rcv_fds.as_mut_ptr(), 32, SOCK_DGRAM);
        i = 0;
        while i < 32 {
            close(rcv_fds[i as usize]);
            i += 1;
        }

        fprintf(
            stderr,
            b"---- TCP IPv4 created before IPv6 ----\n\0".as_ptr() as *const c_char,
        );
        build_rcv_fd(AF_INET, SOCK_STREAM, rcv_fds.as_mut_ptr(), 5);
        build_rcv_fd(AF_INET6, SOCK_STREAM, rcv_fds.as_mut_ptr().add(5), 5);
        test(rcv_fds.as_mut_ptr(), 10, SOCK_STREAM);
        i = 0;
        while i < 10 {
            close(rcv_fds[i as usize]);
            i += 1;
        }

        fprintf(
            stderr,
            b"---- TCP IPv6 created before IPv4 ----\n\0".as_ptr() as *const c_char,
        );
        build_rcv_fd(AF_INET6, SOCK_STREAM, rcv_fds.as_mut_ptr(), 5);
        build_rcv_fd(AF_INET, SOCK_STREAM, rcv_fds.as_mut_ptr().add(5), 5);
        test(rcv_fds.as_mut_ptr(), 10, SOCK_STREAM);
        i = 0;
        while i < 10 {
            close(rcv_fds[i as usize]);
            i += 1;
        }

        fprintf(stderr, b"SUCCESS\n\0".as_ptr() as *const c_char);
    }
}
