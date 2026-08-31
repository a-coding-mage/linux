// SPDX-License-Identifier: GPL-2.0

// C dependencies translated from:
// <arpa/inet.h>, <errno.h>, <error.h>, <netinet/in.h>, <netinet/tcp.h>,
// <signal.h>, <stdio.h>, <stdlib.h>, <sys/socket.h>, <sys/time.h>, <unistd.h>

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, size_of_val, zeroed, MaybeUninit};
use core::ptr::null_mut;

static mut child_pid: c_int = 0;

unsafe extern "C" {
    fn __errno_location() -> *mut c_int;
    fn error(status: c_int, errnum: c_int, format: *const c_char, ...) -> !;
    fn gettimeofday(tv: *mut libc::timeval, tz: *mut c_void) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: libc::socklen_t,
    ) -> c_int;
    fn htons(hostshort: u16) -> u16;
    fn ntohs(netshort: u16) -> u16;
    fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
    fn connect(sockfd: c_int, addr: *const libc::sockaddr, addrlen: libc::socklen_t) -> c_int;
    fn send(sockfd: c_int, buf: *const c_void, len: usize, flags: c_int) -> isize;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn getsockname(
        sockfd: c_int,
        addr: *mut libc::sockaddr,
        addrlen: *mut libc::socklen_t,
    ) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fflush(stream: *mut libc::FILE) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn accept(
        sockfd: c_int,
        addr: *mut libc::sockaddr,
        addrlen: *mut libc::socklen_t,
    ) -> c_int;
    fn kill(pid: c_int, sig: c_int) -> c_int;
    fn exit(status: c_int) -> !;
    fn signal(signum: c_int, handler: libc::sighandler_t) -> libc::sighandler_t;
    fn bind(sockfd: c_int, addr: *const libc::sockaddr, addrlen: libc::socklen_t) -> c_int;
    fn listen(sockfd: c_int, backlog: c_int) -> c_int;
    fn fprintf(stream: *mut libc::FILE, format: *const c_char, ...) -> c_int;
    fn fork() -> c_int;
}

unsafe fn errno() -> c_int {
    *__errno_location()
}

unsafe fn timediff(s: libc::timeval, e: libc::timeval) -> libc::c_ulong {
    let s_us: libc::c_ulong;
    let e_us: libc::c_ulong;

    s_us = (s.tv_sec as libc::c_ulong)
        .wrapping_mul(1000000)
        .wrapping_add(s.tv_usec as libc::c_ulong);
    e_us = (e.tv_sec as libc::c_ulong)
        .wrapping_mul(1000000)
        .wrapping_add(e.tv_usec as libc::c_ulong);
    if s_us > e_us {
        return 0;
    }
    e_us.wrapping_sub(s_us)
}

unsafe fn client(port: c_int) {
    let mut sock: c_int;
    let mut addr: libc::sockaddr_in = zeroed();
    let mut laddr: libc::sockaddr_in = zeroed();
    let mut len: libc::socklen_t = size_of_val(&laddr) as libc::socklen_t;
    let mut sl: libc::linger = zeroed();
    let flag: c_int = 1;
    let mut buffer: MaybeUninit<c_int> = MaybeUninit::uninit();
    let mut start: libc::timeval = zeroed();
    let mut end: libc::timeval = zeroed();
    let mut lat: libc::c_ulong;
    let mut sum_lat: libc::c_ulong = 0;
    let mut nr_lat: libc::c_ulong = 0;

    loop {
        gettimeofday(&mut start, null_mut());

        sock = socket(libc::AF_INET, libc::SOCK_STREAM, 0);
        if sock < 0 {
            error(-1, errno(), c"socket creation".as_ptr());
        }

        sl.l_onoff = 1;
        sl.l_linger = 0;
        if setsockopt(
            sock,
            libc::SOL_SOCKET,
            libc::SO_LINGER,
            &sl as *const _ as *const c_void,
            size_of_val(&sl) as libc::socklen_t,
        ) != 0
        {
            error(-1, errno(), c"setsockopt(linger)".as_ptr());
        }

        if setsockopt(
            sock,
            libc::IPPROTO_TCP,
            libc::TCP_NODELAY,
            &flag as *const _ as *const c_void,
            size_of_val(&flag) as libc::socklen_t,
        ) != 0
        {
            error(-1, errno(), c"setsockopt(nodelay)".as_ptr());
        }

        addr.sin_family = libc::AF_INET as libc::sa_family_t;
        addr.sin_port = htons(port as u16);

        if inet_pton(
            libc::AF_INET,
            c"127.0.0.1".as_ptr(),
            &mut addr.sin_addr as *mut _ as *mut c_void,
        ) <= 0
        {
            error(-1, errno(), c"inet_pton".as_ptr());
        }

        if connect(
            sock,
            &addr as *const _ as *const libc::sockaddr,
            size_of_val(&addr) as libc::socklen_t,
        ) < 0
        {
            error(-1, errno(), c"connect".as_ptr());
        }

        send(
            sock,
            buffer.as_ptr() as *const c_void,
            size_of::<c_int>(),
            0,
        );
        if read(
            sock,
            buffer.as_mut_ptr() as *mut c_void,
            size_of::<c_int>(),
        ) == -1
        {
            error(-1, errno(), c"waiting read".as_ptr());
        }

        gettimeofday(&mut end, null_mut());
        lat = timediff(start, end);
        sum_lat = sum_lat.wrapping_add(lat);
        nr_lat = nr_lat.wrapping_add(1);
        if lat >= 1000000 {
            if getsockname(
                sock,
                &mut laddr as *mut _ as *mut libc::sockaddr,
                &mut len,
            ) == -1
            {
                error(-1, errno(), c"getsockname".as_ptr());
            }
            printf(
                c"port: %d, lat: %lu, avg: %lu, nr: %lu\n".as_ptr(),
                ntohs(laddr.sin_port) as c_int,
                lat,
                sum_lat / nr_lat,
                nr_lat,
            );
        }

        fflush(libc::stdout);
        close(sock);
    }
}

unsafe fn server(sock: c_int, mut address: libc::sockaddr_in) {
    let mut accepted: c_int;
    let mut addrlen: c_int = size_of_val(&address) as c_int;
    let mut buffer: MaybeUninit<c_int> = MaybeUninit::uninit();

    loop {
        accepted = accept(
            sock,
            &mut address as *mut _ as *mut libc::sockaddr,
            &mut addrlen as *mut _ as *mut libc::socklen_t,
        );
        if accepted < 0 {
            error(-1, errno(), c"accept".as_ptr());
        }

        if read(
            accepted,
            buffer.as_mut_ptr() as *mut c_void,
            size_of::<c_int>(),
        ) == -1
        {
            error(-1, errno(), c"read".as_ptr());
        }
        close(accepted);
    }
}

unsafe extern "C" fn sig_handler(_signum: c_int) {
    if child_pid > 0 {
        kill(child_pid, libc::SIGTERM);
    }
    exit(0);
}

fn main() {
    unsafe {
        let mut sock: c_int;
        let opt: c_int = 1;
        let mut address: libc::sockaddr_in = zeroed();
        let mut laddr: libc::sockaddr_in = zeroed();
        let mut len: libc::socklen_t = size_of_val(&laddr) as libc::socklen_t;

        if signal(libc::SIGTERM, sig_handler as libc::sighandler_t) == libc::SIG_ERR {
            error(-1, errno(), c"signal".as_ptr());
        }

        sock = socket(libc::AF_INET, libc::SOCK_STREAM, 0);
        if sock < 0 {
            error(-1, errno(), c"socket".as_ptr());
        }

        if setsockopt(
            sock,
            libc::SOL_SOCKET,
            libc::SO_REUSEADDR | libc::SO_REUSEPORT,
            &opt as *const _ as *const c_void,
            size_of_val(&opt) as libc::socklen_t,
        ) == -1
        {
            error(-1, errno(), c"setsockopt".as_ptr());
        }

        address.sin_family = libc::AF_INET as libc::sa_family_t;
        address.sin_addr.s_addr = libc::INADDR_ANY;
        /* dynamically allocate unused port */
        address.sin_port = 0;

        if bind(
            sock,
            &address as *const _ as *const libc::sockaddr,
            size_of_val(&address) as libc::socklen_t,
        ) < 0
        {
            error(-1, errno(), c"bind".as_ptr());
        }

        if listen(sock, 3) < 0 {
            error(-1, errno(), c"listen".as_ptr());
        }

        if getsockname(
            sock,
            &mut laddr as *mut _ as *mut libc::sockaddr,
            &mut len,
        ) == -1
        {
            error(-1, errno(), c"getsockname".as_ptr());
        }

        fprintf(
            libc::stderr,
            c"server port: %d\n".as_ptr(),
            ntohs(laddr.sin_port) as c_int,
        );
        child_pid = fork();
        if child_pid < 0 {
            error(-1, errno(), c"fork".as_ptr());
        }
        if child_pid == 0 {
            client(ntohs(laddr.sin_port) as c_int);
        } else {
            server(sock, laddr);
        }
    }
}
