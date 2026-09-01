// SPDX-License-Identifier: GPL-2.0
/*
 * Needs something like:
 *
 * iptables -t nat -A POSTROUTING -o nomatch -j MASQUERADE
 *
 * so NAT engine attaches a NAT null-binding to each connection.
 *
 * With unmodified kernels, child or parent will exit with
 * "Port number changed" error, even though no port translation
 * was requested.
 */

use std::ffi::{c_char, c_int, c_void};
use std::mem;
use std::ptr;

const LEN: usize = 512;
const PORT: u16 = 56789;
const TEST_TIME: time_t = 5;

const AF_INET: c_int = 2;
const SOCK_DGRAM: c_int = 2;
const IPPROTO_UDP: c_int = 17;
const SOL_SOCKET: c_int = 1;
const SO_RCVTIMEO: c_int = 20;
const INET_ADDRSTRLEN: usize = 16;

type socklen_t = u32;
type time_t = i64;
type suseconds_t = i64;

#[repr(C)]
#[derive(Copy, Clone)]
struct in_addr {
    s_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
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
struct timeval {
    tv_sec: time_t,
    tv_usec: suseconds_t,
}

unsafe extern "C" {
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    static mut stderr: *mut c_void;

    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: socklen_t,
    ) -> c_int;
    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn sendto(
        sockfd: c_int,
        buf: *const c_void,
        len: usize,
        flags: c_int,
        dest_addr: *const sockaddr,
        addrlen: socklen_t,
    ) -> isize;
    fn recvfrom(
        sockfd: c_int,
        buf: *mut c_void,
        len: usize,
        flags: c_int,
        src_addr: *mut sockaddr,
        addrlen: *mut socklen_t,
    ) -> isize;

    fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
    fn inet_ntop(
        af: c_int,
        src: *const c_void,
        dst: *mut c_char,
        size: socklen_t,
    ) -> *const c_char;

    fn fork() -> c_int;
    fn wait(wstatus: *mut c_int) -> c_int;
    fn time(tloc: *mut time_t) -> time_t;
}

fn htons(hostshort: u16) -> u16 {
    hostshort.to_be()
}

fn ntohs(netshort: u16) -> u16 {
    u16::from_be(netshort)
}

fn wifexited(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn wexitstatus(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

fn die(e: *const c_char) -> ! {
    unsafe {
        perror(e);
        exit(111);
    }
}

fn die_port(sin: *const sockaddr_in, want: u16) -> ! {
    let got = unsafe { ntohs((*sin).sin_port) };
    let mut str_: [c_char; INET_ADDRSTRLEN] = [0; INET_ADDRSTRLEN];

    unsafe {
        inet_ntop(
            AF_INET,
            ptr::addr_of!((*sin).sin_addr).cast::<c_void>(),
            str_.as_mut_ptr(),
            mem::size_of_val(&str_) as socklen_t,
        );

        fprintf(
            stderr,
            c"Port number changed, wanted %d got %d from %s\n".as_ptr(),
            want as c_int,
            got as c_int,
            str_.as_ptr(),
        );
        exit(1);
    }
}

fn udp_socket() -> c_int {
    static TV: timeval = timeval {
        tv_sec: 1,
        tv_usec: 0,
    };
    let fd = unsafe { socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP) };

    if fd < 0 {
        die(c"socket".as_ptr());
    }

    unsafe {
        setsockopt(
            fd,
            SOL_SOCKET,
            SO_RCVTIMEO,
            ptr::addr_of!(TV).cast::<c_void>(),
            mem::size_of_val(&TV) as socklen_t,
        );
    }
    fd
}

fn main() {
    let mut sa1 = sockaddr_in {
        sin_family: AF_INET as u16,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut sa2 = sockaddr_in {
        sin_family: AF_INET as u16,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let s1: c_int;
    let s2: c_int;
    let mut status: c_int = 0;
    let mut end: time_t;
    let mut now: time_t;
    let mut buf: [c_char; LEN] = [0; LEN];
    let child: bool;

    sa1.sin_port = htons(PORT);
    sa2.sin_port = htons(PORT + 1);

    s1 = udp_socket();
    s2 = udp_socket();

    unsafe {
        inet_pton(
            AF_INET,
            c"127.0.0.11".as_ptr(),
            ptr::addr_of_mut!(sa1.sin_addr).cast::<c_void>(),
        );
        inet_pton(
            AF_INET,
            c"127.0.0.12".as_ptr(),
            ptr::addr_of_mut!(sa2.sin_addr).cast::<c_void>(),
        );
    }

    if unsafe {
        bind(
            s1,
            ptr::addr_of!(sa1).cast::<sockaddr>(),
            mem::size_of_val(&sa1) as socklen_t,
        )
    } < 0
    {
        die(c"bind 1".as_ptr());
    }
    if unsafe {
        bind(
            s2,
            ptr::addr_of!(sa2).cast::<sockaddr>(),
            mem::size_of_val(&sa2) as socklen_t,
        )
    } < 0
    {
        die(c"bind 2".as_ptr());
    }

    child = unsafe { fork() == 0 };

    now = unsafe { time(ptr::null_mut()) };
    end = now + TEST_TIME;

    while now < end {
        let mut peer = sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        };
        let mut plen: socklen_t = mem::size_of_val(&peer) as socklen_t;

        now = unsafe { time(ptr::null_mut()) };

        if child {
            if unsafe {
                sendto(
                    s1,
                    buf.as_ptr().cast::<c_void>(),
                    LEN,
                    0,
                    ptr::addr_of!(sa2).cast::<sockaddr>(),
                    mem::size_of_val(&sa2) as socklen_t,
                )
            } != LEN as isize
            {
                continue;
            }

            if unsafe {
                recvfrom(
                    s2,
                    buf.as_mut_ptr().cast::<c_void>(),
                    LEN,
                    0,
                    ptr::addr_of_mut!(peer).cast::<sockaddr>(),
                    ptr::addr_of_mut!(plen),
                )
            } < 0
            {
                die(c"child recvfrom".as_ptr());
            }

            if peer.sin_port != htons(PORT) {
                die_port(ptr::addr_of!(peer), PORT);
            }
        } else {
            if unsafe {
                sendto(
                    s2,
                    buf.as_ptr().cast::<c_void>(),
                    LEN,
                    0,
                    ptr::addr_of!(sa1).cast::<sockaddr>(),
                    mem::size_of_val(&sa1) as socklen_t,
                )
            } != LEN as isize
            {
                continue;
            }

            if unsafe {
                recvfrom(
                    s1,
                    buf.as_mut_ptr().cast::<c_void>(),
                    LEN,
                    0,
                    ptr::addr_of_mut!(peer).cast::<sockaddr>(),
                    ptr::addr_of_mut!(plen),
                )
            } < 0
            {
                die(c"parent recvfrom".as_ptr());
            }

            if peer.sin_port != htons(PORT + 1) {
                die_port(ptr::addr_of!(peer), PORT + 1);
            }
        }
    }

    if child {
        std::process::exit(0);
    }

    unsafe {
        wait(ptr::addr_of_mut!(status));
    }

    if wifexited(status) {
        std::process::exit(wexitstatus(status));
    }

    std::process::exit(1);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
