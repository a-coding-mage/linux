// SPDX-License-Identifier: GPL-2.0

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(static_mut_refs)]

use libc::{
    c_char, c_int, c_ulong, c_void, cmsghdr, iovec, msghdr, pollfd, sockaddr, sockaddr_in,
    sockaddr_in6, sockaddr_storage, timeval,
};
use std::mem;
use std::ptr;

// Original C source defined _GNU_SOURCE and included Linux/POSIX networking headers.

const UDP_GRO: c_int = 104;
const ETH_MAX_MTU: usize = 0xFFFF;

static mut cfg_port: c_int = 8000;
static mut cfg_tcp: bool = false;
static mut cfg_verify: bool = false;
static mut cfg_read_all: bool = false;
static mut cfg_gro_segment: bool = false;
static mut cfg_family: c_int = libc::PF_INET6;
static mut cfg_alen: c_int = mem::size_of::<sockaddr_in6>() as c_int;
static mut cfg_expected_pkt_nr: c_int = 0;
static mut cfg_expected_pkt_len: c_int = 0;
static mut cfg_expected_gso_size: c_int = 0;
static mut cfg_connect_timeout_ms: c_int = 0;
static mut cfg_rcv_timeout_ms: c_int = 0;
static mut cfg_bind_addr: sockaddr_storage = unsafe { mem::zeroed() };

static mut interrupted: bool = false;
static mut packets: c_ulong = 0;
static mut bytes: c_ulong = 0;

unsafe extern "C" {
    fn error(status: c_int, errnum: c_int, format: *const c_char, ...);
    fn signal(signum: c_int, handler: extern "C" fn(c_int)) -> usize;
    static mut optarg: *mut c_char;
    static mut optind: c_int;
    static mut stderr: *mut libc::FILE;
}

extern "C" fn sigint_handler(signum: c_int) {
    unsafe {
        if signum == libc::SIGINT {
            interrupted = true;
        }
    }
}

unsafe fn setup_sockaddr(domain: c_int, str_addr: *const c_char, sockaddr_ptr: *mut c_void) {
    let addr6 = sockaddr_ptr as *mut sockaddr_in6;
    let addr4 = sockaddr_ptr as *mut sockaddr_in;

    match domain {
        libc::PF_INET => {
            (*addr4).sin_family = libc::AF_INET as libc::sa_family_t;
            (*addr4).sin_port = libc::htons(cfg_port as u16);
            if libc::inet_pton(
                libc::AF_INET,
                str_addr,
                &mut (*addr4).sin_addr as *mut _ as *mut c_void,
            ) != 1
            {
                error(1, 0, c"ipv4 parse error: %s".as_ptr(), str_addr);
            }
        }
        libc::PF_INET6 => {
            (*addr6).sin6_family = libc::AF_INET6 as libc::sa_family_t;
            (*addr6).sin6_port = libc::htons(cfg_port as u16);
            if libc::inet_pton(
                libc::AF_INET6,
                str_addr,
                &mut (*addr6).sin6_addr as *mut _ as *mut c_void,
            ) != 1
            {
                error(1, 0, c"ipv6 parse error: %s".as_ptr(), str_addr);
            }
        }
        _ => {
            error(1, 0, c"illegal domain".as_ptr());
        }
    }
}

unsafe fn gettimeofday_ms() -> c_ulong {
    let mut tv: timeval = mem::zeroed();

    libc::gettimeofday(&mut tv, ptr::null_mut());
    ((tv.tv_sec * 1000) + (tv.tv_usec / 1000)) as c_ulong
}

unsafe fn do_poll(fd: c_int, mut timeout_ms: c_int) {
    let mut pfd: pollfd = mem::zeroed();
    let mut ret: c_int;

    pfd.events = libc::POLLIN;
    pfd.revents = 0;
    pfd.fd = fd;

    loop {
        ret = libc::poll(&mut pfd, 1, 10);
        if interrupted {
            break;
        }
        if ret == -1 {
            error(1, *libc::__errno_location(), c"poll".as_ptr());
        }
        if ret == 0 {
            if timeout_ms == 0 {
                continue;
            }

            timeout_ms -= 10;
            if timeout_ms <= 0 {
                interrupted = true;
                break;
            }

            /* no events and more time to wait, do poll again */
            continue;
        }
        if pfd.revents != libc::POLLIN {
            error(
                1,
                *libc::__errno_location(),
                c"poll: 0x%x expected 0x%x\n".as_ptr(),
                pfd.revents as c_int,
                libc::POLLIN as c_int,
            );
        }
        if ret != 0 {
            break;
        }
    }
}

unsafe fn do_socket(do_tcp: bool) -> c_int {
    let mut val: c_int;

    let mut fd = libc::socket(
        cfg_family,
        if cfg_tcp { libc::SOCK_STREAM } else { libc::SOCK_DGRAM },
        0,
    );
    if fd == -1 {
        error(1, *libc::__errno_location(), c"socket".as_ptr());
    }

    val = 1 << 21;
    if libc::setsockopt(
        fd,
        libc::SOL_SOCKET,
        libc::SO_RCVBUF,
        &val as *const _ as *const c_void,
        mem::size_of_val(&val) as libc::socklen_t,
    ) != 0
    {
        error(1, *libc::__errno_location(), c"setsockopt rcvbuf".as_ptr());
    }
    val = 1;
    if libc::setsockopt(
        fd,
        libc::SOL_SOCKET,
        libc::SO_REUSEPORT,
        &val as *const _ as *const c_void,
        mem::size_of_val(&val) as libc::socklen_t,
    ) != 0
    {
        error(1, *libc::__errno_location(), c"setsockopt reuseport".as_ptr());
    }

    if libc::bind(
        fd,
        &cfg_bind_addr as *const _ as *const sockaddr,
        cfg_alen as libc::socklen_t,
    ) != 0
    {
        error(1, *libc::__errno_location(), c"bind".as_ptr());
    }

    if do_tcp {
        let accept_fd = fd;

        if libc::listen(accept_fd, 1) != 0 {
            error(1, *libc::__errno_location(), c"listen".as_ptr());
        }

        do_poll(accept_fd, cfg_connect_timeout_ms);
        if interrupted {
            libc::exit(0);
        }

        fd = libc::accept(accept_fd, ptr::null_mut(), ptr::null_mut());
        if fd == -1 {
            error(1, *libc::__errno_location(), c"accept".as_ptr());
        }
        if libc::close(accept_fd) != 0 {
            error(1, *libc::__errno_location(), c"close accept fd".as_ptr());
        }
    }

    fd
}

/* Flush all outstanding bytes for the tcp receive queue */
unsafe fn do_flush_tcp(fd: c_int) {
    let mut ret: c_int;

    loop {
        /* MSG_TRUNC flushes up to len bytes */
        ret = libc::recv(
            fd,
            ptr::null_mut(),
            1 << 21,
            libc::MSG_TRUNC | libc::MSG_DONTWAIT,
        ) as c_int;
        if ret == -1 && *libc::__errno_location() == libc::EAGAIN {
            return;
        }
        if ret == -1 {
            error(1, *libc::__errno_location(), c"flush".as_ptr());
        }
        if ret == 0 {
            /* client detached */
            libc::exit(0);
        }

        packets = packets.wrapping_add(1);
        bytes = bytes.wrapping_add(ret as c_ulong);
    }
}

fn sanitized_char(val: c_char) -> c_char {
    if val >= b'a' as c_char && val <= b'z' as c_char {
        val
    } else {
        b'.' as c_char
    }
}

unsafe fn do_verify_udp(data: *const c_char, len: c_int) {
    let mut cur = *data;
    let mut i: c_int;

    /* verify contents */
    if cur < b'a' as c_char || cur > b'z' as c_char {
        error(1, 0, c"data initial byte out of range".as_ptr());
    }

    i = 1;
    while i < len {
        if cur == b'z' as c_char {
            cur = b'a' as c_char;
        } else {
            cur = cur.wrapping_add(1);
        }

        if *data.offset(i as isize) != cur {
            error(
                1,
                0,
                c"data[%d]: len %d, %c(%hhu) != %c(%hhu)\n".as_ptr(),
                i,
                len,
                sanitized_char(*data.offset(i as isize)) as c_int,
                *data.offset(i as isize) as libc::c_uchar as c_int,
                sanitized_char(cur) as c_int,
                cur as libc::c_uchar as c_int,
            );
        }
        i += 1;
    }
}

unsafe fn recv_msg(fd: c_int, buf: *mut c_char, len: c_int, gso_size: *mut c_int) -> c_int {
    let mut control = [0u8; unsafe { libc::CMSG_SPACE(mem::size_of::<c_int>() as u32) as usize }];
    let mut msg: msghdr = mem::zeroed();
    let mut iov: iovec = mem::zeroed();
    let mut cmsg: *mut cmsghdr;
    let ret: c_int;

    iov.iov_base = buf as *mut c_void;
    iov.iov_len = len as usize;

    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1;

    msg.msg_control = control.as_mut_ptr() as *mut c_void;
    msg.msg_controllen = control.len();

    *gso_size = -1;
    ret = libc::recvmsg(fd, &mut msg, libc::MSG_TRUNC | libc::MSG_DONTWAIT) as c_int;
    if ret != -1 {
        cmsg = libc::CMSG_FIRSTHDR(&msg);
        while !cmsg.is_null() {
            if (*cmsg).cmsg_level == libc::SOL_UDP && (*cmsg).cmsg_type == UDP_GRO {
                *gso_size = *(libc::CMSG_DATA(cmsg) as *mut c_int);
                break;
            }
            cmsg = libc::CMSG_NXTHDR(&msg, cmsg);
        }
    }
    ret
}

/* Flush all outstanding datagrams. Verify first few bytes of each. */
unsafe fn do_flush_udp(fd: c_int) {
    static mut rbuf: [c_char; ETH_MAX_MTU] = [0; ETH_MAX_MTU];
    let mut ret: c_int;
    let len: c_int;
    let mut gso_size: c_int = 0;
    let mut budget: c_int = 256;

    len = if cfg_read_all {
        mem::size_of_val(&rbuf) as c_int
    } else {
        0
    };
    while {
        let old = budget;
        budget -= 1;
        old != 0
    } {
        /* MSG_TRUNC will make return value full datagram length */
        if cfg_expected_gso_size == 0 {
            ret = libc::recv(
                fd,
                rbuf.as_mut_ptr() as *mut c_void,
                len as usize,
                libc::MSG_TRUNC | libc::MSG_DONTWAIT,
            ) as c_int;
        } else {
            ret = recv_msg(fd, rbuf.as_mut_ptr(), len, &mut gso_size);
        }
        if ret == -1 && *libc::__errno_location() == libc::EAGAIN {
            break;
        }
        if ret == -1 {
            error(1, *libc::__errno_location(), c"recv".as_ptr());
        }
        if cfg_expected_pkt_len != 0 && ret != cfg_expected_pkt_len {
            error(
                1,
                0,
                c"recv: bad packet len, got %d, expected %d\n".as_ptr(),
                ret,
                cfg_expected_pkt_len,
            );
        }
        if len != 0 && cfg_verify {
            if ret == 0 {
                error(1, *libc::__errno_location(), c"recv: 0 byte datagram\n".as_ptr());
            }

            do_verify_udp(rbuf.as_ptr(), ret);
        }
        if cfg_expected_gso_size != 0 && cfg_expected_gso_size != gso_size {
            error(
                1,
                0,
                c"recv: bad gso size, got %d, expected %d (-1 == no gso cmsg))\n".as_ptr(),
                gso_size,
                cfg_expected_gso_size,
            );
        }

        packets = packets.wrapping_add(1);
        bytes = bytes.wrapping_add(ret as c_ulong);
        if cfg_expected_pkt_nr != 0 && packets >= cfg_expected_pkt_nr as c_ulong {
            break;
        }
    }
}

unsafe fn usage(filepath: *const c_char) {
    error(
        1,
        0,
        c"Usage: %s [-C connect_timeout] [-Grtv] [-b addr] [-p port] [-l pktlen] [-n packetnr] [-R rcv_timeout] [-S gsosize]".as_ptr(),
        filepath,
    );
}

unsafe fn parse_opts(argc: c_int, argv: *mut *mut c_char) {
    let mut bind_addr: *const c_char = ptr::null();
    let mut c: c_int;

    loop {
        c = libc::getopt(argc, argv, c"4b:C:Gl:n:p:rR:S:tv".as_ptr());
        if c == -1 {
            break;
        }
        match c as u8 as char {
            '4' => {
                cfg_family = libc::PF_INET;
                cfg_alen = mem::size_of::<sockaddr_in>() as c_int;
            }
            'b' => {
                bind_addr = optarg;
            }
            'C' => {
                cfg_connect_timeout_ms =
                    libc::strtoul(optarg, ptr::null_mut(), 0) as c_int;
            }
            'G' => {
                cfg_gro_segment = true;
            }
            'l' => {
                cfg_expected_pkt_len = libc::strtoul(optarg, ptr::null_mut(), 0) as c_int;
            }
            'n' => {
                cfg_expected_pkt_nr = libc::strtoul(optarg, ptr::null_mut(), 0) as c_int;
            }
            'p' => {
                cfg_port = libc::strtoul(optarg, ptr::null_mut(), 0) as c_int;
            }
            'r' => {
                cfg_read_all = true;
            }
            'R' => {
                cfg_rcv_timeout_ms = libc::strtoul(optarg, ptr::null_mut(), 0) as c_int;
            }
            'S' => {
                cfg_expected_gso_size = libc::strtol(optarg, ptr::null_mut(), 0) as c_int;
            }
            't' => {
                cfg_tcp = true;
            }
            'v' => {
                cfg_verify = true;
                cfg_read_all = true;
            }
            _ => {
                libc::exit(1);
            }
        }
    }

    if bind_addr.is_null() {
        bind_addr = if cfg_family == libc::PF_INET6 {
            c"::".as_ptr()
        } else {
            c"0.0.0.0".as_ptr()
        };
    }

    setup_sockaddr(cfg_family, bind_addr, &mut cfg_bind_addr as *mut _ as *mut c_void);

    if optind != argc {
        usage(*argv.offset(0));
    }

    if cfg_tcp && cfg_verify {
        error(1, 0, c"TODO: implement verify mode for tcp".as_ptr());
    }
}

unsafe fn do_recv() {
    let mut timeout_ms = if cfg_tcp {
        cfg_rcv_timeout_ms
    } else {
        cfg_connect_timeout_ms
    };
    let mut tnow: c_ulong;
    let mut treport: c_ulong;
    let fd: c_int;

    fd = do_socket(cfg_tcp);

    if cfg_gro_segment && !cfg_tcp {
        let val: c_int = 1;
        if libc::setsockopt(
            fd,
            libc::IPPROTO_UDP,
            UDP_GRO,
            &val as *const _ as *const c_void,
            mem::size_of_val(&val) as libc::socklen_t,
        ) != 0
        {
            error(1, *libc::__errno_location(), c"setsockopt UDP_GRO".as_ptr());
        }
    }

    treport = gettimeofday_ms().wrapping_add(1000);
    loop {
        do_poll(fd, timeout_ms);

        if cfg_tcp {
            do_flush_tcp(fd);
        } else {
            do_flush_udp(fd);
        }

        tnow = gettimeofday_ms();
        if cfg_expected_pkt_nr == 0 && tnow > treport {
            if packets != 0 {
                libc::fprintf(
                    stderr,
                    c"%s rx: %6lu MB/s %8lu calls/s\n".as_ptr(),
                    if cfg_tcp { c"tcp".as_ptr() } else { c"udp".as_ptr() },
                    bytes >> 20,
                    packets,
                );
            }
            packets = 0;
            bytes = packets;
            treport = tnow.wrapping_add(1000);
        }

        timeout_ms = cfg_rcv_timeout_ms;

        if interrupted {
            break;
        }
    }

    if cfg_expected_pkt_nr != 0 && packets != cfg_expected_pkt_nr as c_ulong {
        error(
            1,
            0,
            c"wrong packet number! got %ld, expected %d\n".as_ptr(),
            packets,
            cfg_expected_pkt_nr,
        );
    }

    if libc::close(fd) != 0 {
        error(1, *libc::__errno_location(), c"close".as_ptr());
    }
}

fn main() {
    unsafe {
        let mut args: Vec<*mut c_char> = std::env::args()
            .map(|arg| std::ffi::CString::new(arg).unwrap().into_raw())
            .collect();
        args.push(ptr::null_mut());

        parse_opts((args.len() - 1) as c_int, args.as_mut_ptr());

        signal(libc::SIGINT, sigint_handler);

        do_recv();

        for arg in args.into_iter().take_while(|arg| !arg.is_null()) {
            let _ = std::ffi::CString::from_raw(arg);
        }
    }
}
