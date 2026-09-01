// SPDX-License-Identifier: GPL-2.0

use std::ffi::{c_char, c_int, c_void};

const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const IPPROTO_IP: c_int = 0;
const SOL_SOCKET: c_int = 1;
const SO_REUSEADDR: c_int = 2;

type SocklenT = u32;
type SaFamilyT = u16;
type InPortT = u16;
type InAddrT = u32;

#[repr(C)]
struct In6Addr {
    s6_addr: [u8; 16],
}

#[repr(C)]
struct Sockaddr {
    sa_family: SaFamilyT,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct SockaddrIn6 {
    sin6_family: SaFamilyT,
    sin6_port: InPortT,
    sin6_flowinfo: u32,
    sin6_addr: In6Addr,
    sin6_scope_id: u32,
}

unsafe extern "C" {
    fn __errno_location() -> *mut c_int;
    fn htons(hostshort: u16) -> u16;
    fn htonl(hostlong: InAddrT) -> InAddrT;
    fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn setsockopt(
        fd: c_int,
        level: c_int,
        optname: c_int,
        optval: *const c_void,
        optlen: SocklenT,
    ) -> c_int;
    fn bind(fd: c_int, addr: *const Sockaddr, len: SocklenT) -> c_int;
    fn listen(fd: c_int, n: c_int) -> c_int;
    fn connect(fd: c_int, addr: *const Sockaddr, len: SocklenT) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn error(status: c_int, errnum: c_int, format: *const c_char, ...);
}

unsafe fn errno() -> c_int {
    *__errno_location()
}

fn main() -> c_int {
    unsafe {
        let mut fd1: c_int;
        let mut fd2: c_int;
        let one: c_int = 1;
        let mut bind_addr = SockaddrIn6 {
            sin6_family: AF_INET6 as SaFamilyT,
            sin6_port: htons(20000),
            sin6_flowinfo: htonl(0),
            sin6_addr: In6Addr { s6_addr: [0; 16] },
            sin6_scope_id: 0,
        };

        inet_pton(
            AF_INET6,
            c"::".as_ptr(),
            &mut bind_addr.sin6_addr as *mut In6Addr as *mut c_void,
        );

        fd1 = socket(AF_INET6, SOCK_STREAM, IPPROTO_IP);
        if fd1 < 0 {
            error(1, errno(), c"socket fd1".as_ptr());
            return -1;
        }

        let ret = 'out_err1: {
            if setsockopt(
                fd1,
                SOL_SOCKET,
                SO_REUSEADDR,
                &one as *const c_int as *const c_void,
                std::mem::size_of_val(&one) as SocklenT,
            ) != 0
            {
                error(1, errno(), c"setsockopt(SO_REUSEADDR) fd1".as_ptr());
                break 'out_err1 -1;
            }

            if bind(
                fd1,
                &bind_addr as *const SockaddrIn6 as *const Sockaddr,
                std::mem::size_of_val(&bind_addr) as SocklenT,
            ) != 0
            {
                error(1, errno(), c"bind fd1".as_ptr());
                break 'out_err1 -1;
            }

            if listen(fd1, 0) != 0 {
                error(1, errno(), c"listen".as_ptr());
                break 'out_err1 -1;
            }

            fd2 = socket(AF_INET6, SOCK_STREAM, IPPROTO_IP);
            if fd2 < 0 {
                error(1, errno(), c"socket fd2".as_ptr());
                break 'out_err1 -1;
            }

            let ret = 'out_err2: {
                if connect(
                    fd2,
                    &bind_addr as *const SockaddrIn6 as *const Sockaddr,
                    std::mem::size_of_val(&bind_addr) as SocklenT,
                ) != 0
                {
                    error(1, errno(), c"bind fd2".as_ptr());
                    break 'out_err2 -1;
                }

                close(fd2);
                close(fd1);
                return 0;
            };

            close(fd2);
            ret
        };

        close(fd1);
        ret
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
