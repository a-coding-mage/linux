// SPDX-License-Identifier: GPL-2.0

use std::ffi::{c_char, c_int, c_void};
use std::mem;
use std::ptr;

const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const IPPROTO_IP: c_int = 0;
const SOL_SOCKET: c_int = 1;
const SO_REUSEADDR: c_int = 2;
const MSG_FASTOPEN: c_int = 0x20000000;

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
struct sockaddr_in6 {
    sin6_family: u16,
    sin6_port: u16,
    sin6_flowinfo: u32,
    sin6_addr: in6_addr,
    sin6_scope_id: u32,
}

unsafe extern "C" {
    fn htons(hostshort: u16) -> u16;
    fn htonl(hostlong: u32) -> u32;
    fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: u32,
    ) -> c_int;
    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: u32) -> c_int;
    fn sendto(
        sockfd: c_int,
        buf: *const c_void,
        len: usize,
        flags: c_int,
        dest_addr: *const sockaddr,
        addrlen: u32,
    ) -> isize;
    fn listen(sockfd: c_int, backlog: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn error(status: c_int, errnum: c_int, format: *const c_char, ...);
    fn __errno_location() -> *mut c_int;
}

unsafe fn errno() -> c_int {
    unsafe { *__errno_location() }
}

fn main() {
    unsafe {
        let mut fd1: c_int;
        let mut fd2: c_int;
        let one: c_int = 1;
        let mut bind_addr = sockaddr_in6 {
            sin6_family: AF_INET6 as u16,
            sin6_port: htons(20000),
            sin6_flowinfo: htonl(0),
            sin6_addr: in6_addr { s6_addr: [0; 16] },
            sin6_scope_id: 0,
        };

        inet_pton(
            AF_INET6,
            c"::".as_ptr(),
            &mut bind_addr.sin6_addr as *mut in6_addr as *mut c_void,
        );

        fd1 = socket(AF_INET6, SOCK_STREAM, IPPROTO_IP);
        if fd1 < 0 {
            error(1, errno(), c"socket fd1".as_ptr());
            std::process::exit(-1);
        }

        'out_err1: {
            if setsockopt(
                fd1,
                SOL_SOCKET,
                SO_REUSEADDR,
                &one as *const c_int as *const c_void,
                mem::size_of_val(&one) as u32,
            ) != 0
            {
                error(1, errno(), c"setsockopt(SO_REUSEADDR) fd1".as_ptr());
                break 'out_err1;
            }

            if bind(
                fd1,
                &bind_addr as *const sockaddr_in6 as *const sockaddr,
                mem::size_of_val(&bind_addr) as u32,
            ) != 0
            {
                error(1, errno(), c"bind fd1".as_ptr());
                break 'out_err1;
            }

            if sendto(
                fd1,
                ptr::null(),
                0,
                MSG_FASTOPEN,
                &bind_addr as *const sockaddr_in6 as *const sockaddr,
                mem::size_of_val(&bind_addr) as u32,
            ) != 0
            {
                error(1, errno(), c"sendto fd1".as_ptr());
                break 'out_err1;
            }

            fd2 = socket(AF_INET6, SOCK_STREAM, IPPROTO_IP);
            if fd2 < 0 {
                error(1, errno(), c"socket fd2".as_ptr());
                break 'out_err1;
            }

            'out_err2: {
                if setsockopt(
                    fd2,
                    SOL_SOCKET,
                    SO_REUSEADDR,
                    &one as *const c_int as *const c_void,
                    mem::size_of_val(&one) as u32,
                ) != 0
                {
                    error(1, errno(), c"setsockopt(SO_REUSEADDR) fd2".as_ptr());
                    break 'out_err2;
                }

                if bind(
                    fd2,
                    &bind_addr as *const sockaddr_in6 as *const sockaddr,
                    mem::size_of_val(&bind_addr) as u32,
                ) != 0
                {
                    error(1, errno(), c"bind fd2".as_ptr());
                    break 'out_err2;
                }

                if sendto(
                    fd2,
                    ptr::null(),
                    0,
                    MSG_FASTOPEN,
                    &bind_addr as *const sockaddr_in6 as *const sockaddr,
                    mem::size_of_val(&bind_addr) as u32,
                ) != -1
                {
                    error(1, errno(), c"sendto fd2".as_ptr());
                    break 'out_err2;
                }

                if listen(fd2, 0) != 0 {
                    error(1, errno(), c"listen".as_ptr());
                    break 'out_err2;
                }

                close(fd2);
                close(fd1);
                std::process::exit(0);
            }

            close(fd2);
        }

        close(fd1);
        std::process::exit(-1);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
