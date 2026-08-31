// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2018 Google Inc.
 * Author: Soheil Hassas Yeganeh (soheil@google.com)
 *
 * Simple example on how to use TCP_INQ and TCP_CM_INQ.
 */

use libc::{
    c_char, c_int, c_long, c_ulong, c_void, in6addr_loopback, iovec, msghdr, pthread_t,
    sockaddr, sockaddr_in, sockaddr_in6, sockaddr_storage, socklen_t, AF_INET, AF_INET6, EINTR,
    INADDR_LOOPBACK, MSG_CTRUNC, PF_INET, PF_INET6, SOCK_STREAM, SOL_SOCKET, SOL_TCP,
    SO_REUSEADDR,
};
use std::ffi::CString;
use std::mem;
use std::ptr;

const TCP_INQ: c_int = 36;
const TCP_CM_INQ: c_int = TCP_INQ;

const BUF_SIZE: usize = 8192;
const CMSG_SIZE: usize = 32;

static mut family: c_int = AF_INET6;
static mut addr_len: socklen_t = mem::size_of::<sockaddr_in6>() as socklen_t;
static mut port: c_int = 4974;

unsafe extern "C" {
    fn error(status: c_int, errnum: c_int, format: *const c_char, ...);
}

unsafe fn setup_loopback_addr(family: c_int, sockaddr: *mut sockaddr_storage) {
    let addr6 = sockaddr as *mut sockaddr_in6;
    let addr4 = sockaddr as *mut sockaddr_in;

    match family {
        PF_INET => {
            ptr::write_bytes(addr4, 0, 1);
            (*addr4).sin_family = AF_INET as libc::sa_family_t;
            (*addr4).sin_addr.s_addr = libc::htonl(INADDR_LOOPBACK);
            (*addr4).sin_port = libc::htons(port as u16);
        }
        PF_INET6 => {
            ptr::write_bytes(addr6, 0, 1);
            (*addr6).sin6_family = AF_INET6 as libc::sa_family_t;
            (*addr6).sin6_addr = in6addr_loopback;
            (*addr6).sin6_port = libc::htons(port as u16);
        }
        _ => {
            error(1, 0, b"illegal family\0".as_ptr() as *const c_char);
        }
    }
}

unsafe extern "C" fn start_server(arg: *mut c_void) -> *mut c_void {
    let server_fd = arg as c_ulong as c_int;
    let mut addr: sockaddr_in = mem::zeroed();
    let mut addrlen: socklen_t = mem::size_of_val(&addr) as socklen_t;
    let buf: *mut c_char;
    let mut fd: c_int;
    let mut r: libc::ssize_t;

    buf = libc::malloc(BUF_SIZE) as *mut c_char;

    loop {
        fd = libc::accept(
            server_fd,
            &mut addr as *mut sockaddr_in as *mut sockaddr,
            &mut addrlen,
        );
        if fd == -1 {
            libc::perror(b"accept\0".as_ptr() as *const c_char);
            break;
        }
        loop {
            r = libc::send(fd, buf as *const c_void, BUF_SIZE, 0);
            if !(r < 0 && *libc::__errno_location() == EINTR) {
                break;
            }
        }
        if r < 0 {
            libc::perror(b"send\0".as_ptr() as *const c_char);
        }
        if r != BUF_SIZE as libc::ssize_t {
            libc::fprintf(
                libc::stderr,
                b"can only send %d bytes\n\0".as_ptr() as *const c_char,
                r as c_int,
            );
        }
        /* TCP_INQ can overestimate in-queue by one byte if we send
         * the FIN packet. Sleep for 1 second, so that the client
         * likely invoked recvmsg().
         */
        libc::sleep(1);
        libc::close(fd);
    }

    libc::free(buf as *mut c_void);
    libc::close(server_fd);
    libc::pthread_exit(ptr::null_mut());
}

fn main() {
    unsafe {
        let mut listen_addr: sockaddr_storage = mem::zeroed();
        let mut addr: sockaddr_storage = mem::zeroed();
        let mut c: c_int;
        let mut one: c_int = 1;
        let mut inq: c_int = -1;
        let mut server_thread: pthread_t = mem::zeroed();
        let mut cmsgbuf: [c_char; CMSG_SIZE] = [0; CMSG_SIZE];
        let mut iov: [iovec; 1] = [mem::zeroed()];
        let mut cm: *mut libc::cmsghdr;
        let mut msg: msghdr = mem::zeroed();
        let server_fd: c_int;
        let fd: c_int;
        let buf: *mut c_char;
        let mut argv_storage: Vec<CString> = std::env::args()
            .map(|arg| CString::new(arg).unwrap())
            .collect();
        let mut argv: Vec<*mut c_char> = argv_storage
            .iter_mut()
            .map(|arg| arg.as_ptr() as *mut c_char)
            .collect();
        argv.push(ptr::null_mut());
        let argc = (argv.len() - 1) as c_int;

        loop {
            c = libc::getopt(
                argc,
                argv.as_mut_ptr(),
                b"46p:\0".as_ptr() as *const c_char,
            );
            if c == -1 {
                break;
            }
            match c {
                x if x == b'4' as c_int => {
                    family = PF_INET;
                    addr_len = mem::size_of::<sockaddr_in>() as socklen_t;
                }
                x if x == b'6' as c_int => {
                    family = PF_INET6;
                    addr_len = mem::size_of::<sockaddr_in6>() as socklen_t;
                }
                x if x == b'p' as c_int => {
                    port = libc::atoi(libc::optarg);
                }
                _ => {}
            }
        }

        server_fd = libc::socket(family, SOCK_STREAM, 0);
        if server_fd < 0 {
            error(1, *libc::__errno_location(), b"server socket\0".as_ptr() as *const c_char);
        }
        setup_loopback_addr(family, &mut listen_addr);
        if libc::setsockopt(
            server_fd,
            SOL_SOCKET,
            SO_REUSEADDR,
            &mut one as *mut c_int as *const c_void,
            mem::size_of_val(&one) as socklen_t,
        ) != 0
        {
            error(
                1,
                *libc::__errno_location(),
                b"setsockopt(SO_REUSEADDR)\0".as_ptr() as *const c_char,
            );
        }
        if libc::bind(
            server_fd,
            &listen_addr as *const sockaddr_storage as *const sockaddr,
            addr_len,
        ) == -1
        {
            error(1, *libc::__errno_location(), b"bind\0".as_ptr() as *const c_char);
        }
        if libc::listen(server_fd, 128) == -1 {
            error(1, *libc::__errno_location(), b"listen\0".as_ptr() as *const c_char);
        }
        if libc::pthread_create(
            &mut server_thread,
            ptr::null(),
            start_server,
            server_fd as c_ulong as *mut c_void,
        ) != 0
        {
            error(
                1,
                *libc::__errno_location(),
                b"pthread_create\0".as_ptr() as *const c_char,
            );
        }

        fd = libc::socket(family, SOCK_STREAM, 0);
        if fd < 0 {
            error(1, *libc::__errno_location(), b"client socket\0".as_ptr() as *const c_char);
        }
        setup_loopback_addr(family, &mut addr);
        if libc::connect(fd, &addr as *const sockaddr_storage as *const sockaddr, addr_len) == -1 {
            error(1, *libc::__errno_location(), b"connect\0".as_ptr() as *const c_char);
        }
        if libc::setsockopt(
            fd,
            SOL_TCP,
            TCP_INQ,
            &mut one as *mut c_int as *const c_void,
            mem::size_of_val(&one) as socklen_t,
        ) != 0
        {
            error(
                1,
                *libc::__errno_location(),
                b"setsockopt(TCP_INQ)\0".as_ptr() as *const c_char,
            );
        }

        msg.msg_name = ptr::null_mut();
        msg.msg_namelen = 0;
        msg.msg_iov = iov.as_mut_ptr();
        msg.msg_iovlen = 1;
        msg.msg_control = cmsgbuf.as_mut_ptr() as *mut c_void;
        msg.msg_controllen = mem::size_of_val(&cmsgbuf);
        msg.msg_flags = 0;

        buf = libc::malloc(BUF_SIZE) as *mut c_char;
        iov[0].iov_base = buf as *mut c_void;
        iov[0].iov_len = BUF_SIZE / 2;

        if libc::recvmsg(fd, &mut msg, 0) != iov[0].iov_len as libc::ssize_t {
            error(1, *libc::__errno_location(), b"recvmsg\0".as_ptr() as *const c_char);
        }
        if msg.msg_flags & MSG_CTRUNC != 0 {
            error(
                1,
                0,
                b"control message is truncated\0".as_ptr() as *const c_char,
            );
        }

        cm = libc::CMSG_FIRSTHDR(&msg);
        while !cm.is_null() {
            if (*cm).cmsg_level == SOL_TCP && (*cm).cmsg_type == TCP_CM_INQ {
                inq = *(libc::CMSG_DATA(cm) as *mut c_int);
            }
            cm = libc::CMSG_NXTHDR(&msg, cm);
        }

        if inq != (BUF_SIZE - iov[0].iov_len) as c_int {
            libc::fprintf(
                libc::stderr,
                b"unexpected inq: %d\n\0".as_ptr() as *const c_char,
                inq,
            );
            libc::exit(1);
        }

        libc::printf(b"PASSED\n\0".as_ptr() as *const c_char);
        libc::free(buf as *mut c_void);
        libc::close(fd);
    }
}
