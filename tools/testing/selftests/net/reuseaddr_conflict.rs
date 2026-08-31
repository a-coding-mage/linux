/*
 * Test for the regression introduced by
 *
 * b9470c27607b ("inet: kill smallest_size and smallest_port")
 *
 * If we open an ipv4 socket on a port with reuseaddr we shouldn't reset the tb
 * when we open the ipv6 conterpart, which is what was happening previously.
 */
// C dependencies: errno.h, error.h, arpa/inet.h, netinet/in.h, stdbool.h,
// stdio.h, sys/socket.h, sys/types.h, unistd.h.

use libc::{
    bind, close, fprintf, htonl, htons, in6_addr, in_addr, inet_addr, listen, perror, setsockopt,
    sockaddr, sockaddr_in, sockaddr_in6, socket, AF_INET, AF_INET6, INADDR_ANY, IPPROTO_IPV6,
    IPPROTO_TCP, IPV6_V6ONLY, SOCK_STREAM, SOL_SOCKET, SO_REUSEADDR,
};
use libc::{c_char, c_int, c_void, size_t, stderr};

const PORT: c_int = 9999;

extern "C" {
    fn error(status: c_int, errnum: c_int, format: *const c_char, ...);
    fn __errno_location() -> *mut c_int;
}

unsafe fn errno_value() -> c_int {
    *__errno_location()
}

unsafe fn open_port(ipv6: c_int, any: c_int) -> c_int {
    let mut fd: c_int = -1;
    let mut reuseaddr: c_int = 1;
    let mut v6only: c_int = 1;
    let addrlen: c_int;
    let addr: *mut sockaddr;
    let ret: c_int = -1;
    let family: c_int = if ipv6 != 0 { AF_INET6 } else { AF_INET };

    let mut addr6 = sockaddr_in6 {
        sin6_family: AF_INET6 as libc::sa_family_t,
        sin6_port: htons(PORT as u16),
        sin6_flowinfo: 0,
        sin6_addr: in6_addr { s6_addr: [0; 16] },
        sin6_scope_id: 0,
    };
    let mut addr4 = sockaddr_in {
        sin_family: AF_INET as libc::sa_family_t,
        sin_port: htons(PORT as u16),
        sin_addr: in_addr {
            s_addr: if any != 0 {
                htonl(INADDR_ANY)
            } else {
                inet_addr(b"127.0.0.1\0".as_ptr() as *const c_char)
            },
        },
        sin_zero: [0; 8],
    };

    if ipv6 != 0 {
        addr = &mut addr6 as *mut sockaddr_in6 as *mut sockaddr;
        addrlen = ::core::mem::size_of_val(&addr6) as c_int;
    } else {
        addr = &mut addr4 as *mut sockaddr_in as *mut sockaddr;
        addrlen = ::core::mem::size_of_val(&addr4) as c_int;
    }

    fd = socket(family, SOCK_STREAM, IPPROTO_TCP);
    if fd < 0 {
        perror(b"socket\0".as_ptr() as *const c_char);
        goto_out(fd, ret)
    } else {
        if ipv6 != 0
            && setsockopt(
                fd,
                IPPROTO_IPV6,
                IPV6_V6ONLY,
                &mut v6only as *mut c_int as *mut c_void,
                ::core::mem::size_of_val(&v6only) as libc::socklen_t,
            ) < 0
        {
            perror(b"setsockopt IPV6_V6ONLY\0".as_ptr() as *const c_char);
            goto_out(fd, ret)
        } else if setsockopt(
            fd,
            SOL_SOCKET,
            SO_REUSEADDR,
            &mut reuseaddr as *mut c_int as *mut c_void,
            ::core::mem::size_of_val(&reuseaddr) as libc::socklen_t,
        ) < 0
        {
            perror(b"setsockopt SO_REUSEADDR\0".as_ptr() as *const c_char);
            goto_out(fd, ret)
        } else if bind(fd, addr, addrlen as libc::socklen_t) < 0 {
            perror(b"bind\0".as_ptr() as *const c_char);
            goto_out(fd, ret)
        } else if any != 0 {
            fd
        } else if listen(fd, 1) < 0 {
            perror(b"listen\0".as_ptr() as *const c_char);
            goto_out(fd, ret)
        } else {
            fd
        }
    }
}

unsafe fn goto_out(fd: c_int, ret: c_int) -> c_int {
    close(fd);
    ret
}

unsafe fn main_impl() -> c_int {
    let listenfd: c_int;
    let mut fd1: c_int;
    let fd2: c_int;

    fprintf(
        stderr,
        b"Opening 127.0.0.1:%d\n\0".as_ptr() as *const c_char,
        PORT,
    );
    listenfd = open_port(0, 0);
    if listenfd < 0 {
        error(
            1,
            errno_value(),
            b"Couldn't open listen socket\0".as_ptr() as *const c_char,
        );
    }
    fprintf(
        stderr,
        b"Opening INADDR_ANY:%d\n\0".as_ptr() as *const c_char,
        PORT,
    );
    fd1 = open_port(0, 1);
    if fd1 >= 0 {
        error(
            1,
            0,
            b"Was allowed to create an ipv4 reuseport on a already bound non-reuseport socket\0"
                .as_ptr() as *const c_char,
        );
    }
    fprintf(
        stderr,
        b"Opening in6addr_any:%d\n\0".as_ptr() as *const c_char,
        PORT,
    );
    fd1 = open_port(1, 1);
    if fd1 < 0 {
        error(
            1,
            errno_value(),
            b"Couldn't open ipv6 reuseport\0".as_ptr() as *const c_char,
        );
    }
    fprintf(
        stderr,
        b"Opening INADDR_ANY:%d\n\0".as_ptr() as *const c_char,
        PORT,
    );
    fd2 = open_port(0, 1);
    if fd2 >= 0 {
        error(
            1,
            0,
            b"Was allowed to create an ipv4 reuseport on a already bound non-reuseport socket\0"
                .as_ptr() as *const c_char,
        );
    }
    close(fd1);
    fprintf(
        stderr,
        b"Opening INADDR_ANY:%d after closing ipv6 socket\n\0".as_ptr() as *const c_char,
        PORT,
    );
    fd1 = open_port(0, 1);
    if fd1 >= 0 {
        error(
            1,
            0,
            b"Was allowed to create an ipv4 reuseport on an already bound non-reuseport socket with no ipv6\0"
                .as_ptr() as *const c_char,
        );
    }
    fprintf(stderr, b"Success\n\0".as_ptr() as *const c_char);
    0
}

fn main() {
    unsafe {
        main_impl();
    }
}
