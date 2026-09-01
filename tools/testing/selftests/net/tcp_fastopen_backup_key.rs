// SPDX-License-Identifier: GPL-2.0

/*
 * Test key rotation for TFO.
 * New keys are 'rotated' in two steps:
 * 1) Add new key as the 'backup' key 'behind' the primary key
 * 2) Make new key the primary by swapping the backup and primary keys
 *
 * The rotation is done in stages using multiple sockets bound
 * to the same port via SO_REUSEPORT. This simulates key rotation
 * behind say a load balancer. We verify that across the rotation
 * there are no cases in which a cookie is not accepted by verifying
 * that TcpExtTCPFastOpenPassiveFail remains 0.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

// C dependencies: arpa/inet.h, errno.h, error.h, stdbool.h, stdio.h, stdlib.h,
// string.h, sys/epoll.h, unistd.h, netinet/tcp.h, fcntl.h, time.h,
// and "kselftest.h".

const TCP_FASTOPEN_KEY: c_int = 33;

const N_LISTEN: usize = 10;
const PROC_FASTOPEN_KEY: &[u8] = b"/proc/sys/net/ipv4/tcp_fastopen_key\0";
const KEY_LENGTH: usize = 16;

static mut DO_IPV6: bool = false;
static mut DO_SOCKOPT: bool = false;
static mut DO_ROTATE: bool = false;
static mut KEY_LEN: c_int = KEY_LENGTH as c_int;
static mut RCV_FDS: [c_int; N_LISTEN] = [0; N_LISTEN];
static mut PROC_FD: c_int = 0;
static IP4_ADDR: &[u8] = b"127.0.0.1\0";
static IP6_ADDR: &[u8] = b"::1\0";
const PORT: c_int = 8891;

const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const SOL_SOCKET: c_int = 1;
const SO_REUSEPORT: c_int = 15;
const SOL_TCP: c_int = 6;
const TCP_FASTOPEN: c_int = 23;
const INADDR_ANY: u32 = 0;
const SEEK_SET: c_int = 0;
const MSG_FASTOPEN: c_int = 0x20000000;
const EPOLLIN: u32 = 0x001;
const EPOLL_CTL_ADD: c_int = 1;
const EPOLL_CTL_DEL: c_int = 2;
const O_RDWR: c_int = 0o2;

type socklen_t = u32;
type ssize_t = isize;
type time_t = i64;

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
struct sockaddr_in6 {
    sin6_family: u16,
    sin6_port: u16,
    sin6_flowinfo: u32,
    sin6_addr: in6_addr,
    sin6_scope_id: u32,
}

#[repr(C)]
union epoll_data_t {
    ptr: *mut c_void,
    fd: c_int,
    u32_: u32,
    u64_: u64,
}

#[repr(C)]
struct epoll_event {
    events: u32,
    data: epoll_data_t,
}

static mut IN6ADDR_ANY: in6_addr = in6_addr { s6_addr: [0; 16] };

unsafe extern "C" {
    fn *__errno_location() -> *mut c_int;
    fn error(status: c_int, errnum: c_int, format: *const c_char, ...) -> !;
    fn getsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *mut c_void,
        option_len: *mut socklen_t,
    ) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: socklen_t,
    ) -> c_int;
    fn lseek(fd: c_int, offset: i64, whence: c_int) -> i64;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> ssize_t;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> ssize_t;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn htonl(hostlong: u32) -> u32;
    fn htons(hostshort: u16) -> u16;
    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn listen(sockfd: c_int, backlog: c_int) -> c_int;
    fn rand() -> c_int;
    fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
    fn sendto(
        sockfd: c_int,
        buf: *const c_void,
        len: usize,
        flags: c_int,
        dest_addr: *const sockaddr,
        addrlen: socklen_t,
    ) -> ssize_t;
    fn epoll_create(size: c_int) -> c_int;
    fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut epoll_event) -> c_int;
    fn epoll_wait(
        epfd: c_int,
        events: *mut epoll_event,
        maxevents: c_int,
        timeout: c_int,
    ) -> c_int;
    fn accept(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn recv(sockfd: c_int, buf: *mut c_void, len: usize, flags: c_int) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn srand(seed: c_uint);
    fn time(tloc: *mut time_t) -> time_t;
    static mut stderr: *mut c_void;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
}

unsafe fn errno() -> c_int {
    *__errno_location()
}

unsafe fn get_keys(fd: c_int, keys: *mut u32) {
    let mut buf: [c_char; 128] = [0; 128];
    let mut len: socklen_t = (KEY_LENGTH * 2) as socklen_t;

    if DO_SOCKOPT {
        if getsockopt(
            fd,
            SOL_TCP,
            TCP_FASTOPEN_KEY,
            keys as *mut c_void,
            &mut len,
        ) != 0
        {
            error(1, errno(), c"Unable to get key".as_ptr());
        }
        return;
    }
    lseek(PROC_FD, 0, SEEK_SET);
    if read(PROC_FD, buf.as_mut_ptr() as *mut c_void, buf.len()) <= 0 {
        error(
            1,
            errno(),
            c"Unable to read %s".as_ptr(),
            PROC_FASTOPEN_KEY.as_ptr() as *const c_char,
        );
    }
    if sscanf(
        buf.as_ptr(),
        c"%x-%x-%x-%x,%x-%x-%x-%x".as_ptr(),
        keys,
        keys.add(1),
        keys.add(2),
        keys.add(3),
        keys.add(4),
        keys.add(5),
        keys.add(6),
        keys.add(7),
    ) != 8
    {
        error(
            1,
            0,
            c"Unable to parse %s".as_ptr(),
            PROC_FASTOPEN_KEY.as_ptr() as *const c_char,
        );
    }
}

unsafe fn set_keys(fd: c_int, keys: *mut u32) {
    let mut buf: [c_char; 128] = [0; 128];

    if DO_SOCKOPT {
        if setsockopt(
            fd,
            SOL_TCP,
            TCP_FASTOPEN_KEY,
            keys as *const c_void,
            KEY_LEN as socklen_t,
        ) != 0
        {
            error(1, errno(), c"Unable to set key".as_ptr());
        }
        return;
    }
    if DO_ROTATE {
        snprintf(
            buf.as_mut_ptr(),
            128,
            c"%08x-%08x-%08x-%08x,%08x-%08x-%08x-%08x".as_ptr(),
            *keys.add(0),
            *keys.add(1),
            *keys.add(2),
            *keys.add(3),
            *keys.add(4),
            *keys.add(5),
            *keys.add(6),
            *keys.add(7),
        );
    } else {
        snprintf(
            buf.as_mut_ptr(),
            128,
            c"%08x-%08x-%08x-%08x".as_ptr(),
            *keys.add(0),
            *keys.add(1),
            *keys.add(2),
            *keys.add(3),
        );
    }
    lseek(PROC_FD, 0, SEEK_SET);
    if write(PROC_FD, buf.as_ptr() as *const c_void, buf.len()) <= 0 {
        error(
            1,
            errno(),
            c"Unable to write %s".as_ptr(),
            PROC_FASTOPEN_KEY.as_ptr() as *const c_char,
        );
    }
}

unsafe fn build_rcv_fd(family: c_int, proto: c_int, rcv_fds: *mut c_int) {
    let mut addr4: sockaddr_in = mem::zeroed();
    let mut addr6: sockaddr_in6 = mem::zeroed();
    let addr: *mut sockaddr;
    let mut opt: c_int = 1;
    let mut sz: c_int;
    let mut qlen: c_int = 100;
    let mut keys: [u32; 8] = [0; 8];

    match family {
        AF_INET => {
            addr4.sin_family = family as u16;
            addr4.sin_addr.s_addr = htonl(INADDR_ANY);
            addr4.sin_port = htons(PORT as u16);
            sz = mem::size_of_val(&addr4) as c_int;
            addr = &mut addr4 as *mut sockaddr_in as *mut sockaddr;
        }
        AF_INET6 => {
            addr6.sin6_family = AF_INET6 as u16;
            addr6.sin6_addr = IN6ADDR_ANY;
            addr6.sin6_port = htons(PORT as u16);
            sz = mem::size_of_val(&addr6) as c_int;
            addr = &mut addr6 as *mut sockaddr_in6 as *mut sockaddr;
        }
        _ => {
            error(1, 0, c"Unsupported family %d".as_ptr(), family);
            /*
             * clang does not recognize error() above as terminating
             * the program, so it complains that saddr, sz are
             * not initialized when this code path is taken. Silence it.
             */
        }
    }
    for i in 0..keys.len() {
        keys[i] = rand() as u32;
    }
    for i in 0..N_LISTEN {
        *rcv_fds.add(i) = socket(family, proto, 0);
        if *rcv_fds.add(i) < 0 {
            error(1, errno(), c"failed to create receive socket".as_ptr());
        }
        if setsockopt(
            *rcv_fds.add(i),
            SOL_SOCKET,
            SO_REUSEPORT,
            &mut opt as *mut c_int as *const c_void,
            mem::size_of_val(&opt) as socklen_t,
        ) != 0
        {
            error(1, errno(), c"failed to set SO_REUSEPORT".as_ptr());
        }
        if bind(*rcv_fds.add(i), addr, sz as socklen_t) != 0 {
            error(1, errno(), c"failed to bind receive socket".as_ptr());
        }
        if setsockopt(
            *rcv_fds.add(i),
            SOL_TCP,
            TCP_FASTOPEN,
            &mut qlen as *mut c_int as *const c_void,
            mem::size_of_val(&qlen) as socklen_t,
        ) != 0
        {
            error(1, errno(), c"failed to set TCP_FASTOPEN".as_ptr());
        }
        set_keys(*rcv_fds.add(i), keys.as_mut_ptr());
        if proto == SOCK_STREAM && listen(*rcv_fds.add(i), 10) != 0 {
            error(1, errno(), c"failed to listen on receive port".as_ptr());
        }
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
    let sz: c_int;
    let ret: ssize_t;
    let mut data: [c_char; 1] = [0; 1];

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
                    c"inet_pton failed: %s".as_ptr(),
                    IP4_ADDR.as_ptr() as *const c_char,
                );
            }
            daddr4.sin_port = htons(PORT as u16);

            sz = mem::size_of_val(&saddr4) as c_int;
            saddr = &mut saddr4 as *mut sockaddr_in as *mut sockaddr;
            daddr = &mut daddr4 as *mut sockaddr_in as *mut sockaddr;
        }
        AF_INET6 => {
            saddr6.sin6_family = AF_INET6 as u16;
            saddr6.sin6_addr = IN6ADDR_ANY;

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
                    c"inet_pton failed: %s".as_ptr(),
                    IP6_ADDR.as_ptr() as *const c_char,
                );
            }
            daddr6.sin6_port = htons(PORT as u16);

            sz = mem::size_of_val(&saddr6) as c_int;
            saddr = &mut saddr6 as *mut sockaddr_in6 as *mut sockaddr;
            daddr = &mut daddr6 as *mut sockaddr_in6 as *mut sockaddr;
        }
        _ => {
            error(1, 0, c"Unsupported family %d".as_ptr(), family);
            /*
             * clang does not recognize error() above as terminating
             * the program, so it complains that saddr, daddr, sz are
             * not initialized when this code path is taken. Silence it.
             */
        }
    }
    fd = socket(family, proto, 0);
    if fd < 0 {
        error(1, errno(), c"failed to create send socket".as_ptr());
    }
    if bind(fd, saddr, sz as socklen_t) != 0 {
        error(1, errno(), c"failed to bind send socket".as_ptr());
    }
    data[0] = b'a' as c_char;
    ret = sendto(
        fd,
        data.as_ptr() as *const c_void,
        1,
        MSG_FASTOPEN,
        daddr,
        sz as socklen_t,
    );
    if ret != 1 {
        error(1, errno(), c"failed to sendto".as_ptr());
    }

    fd
}

unsafe fn is_listen_fd(fd: c_int) -> bool {
    for i in 0..N_LISTEN {
        if RCV_FDS[i] == fd {
            return true;
        }
    }
    false
}

unsafe fn rotate_key(fd: c_int) {
    static mut ITER: c_int = 0;
    static mut NEW_KEY: [u32; 4] = [0; 4];
    let mut keys: [u32; 8] = [0; 8];
    let mut tmp_key: [u32; 4] = [0; 4];

    if ITER < N_LISTEN as c_int {
        /* first set new key as backups */
        if ITER == 0 {
            for i in 0..NEW_KEY.len() {
                NEW_KEY[i] = rand() as u32;
            }
        }
        get_keys(fd, keys.as_mut_ptr());
        ptr::copy_nonoverlapping(
            NEW_KEY.as_ptr() as *const c_void,
            keys.as_mut_ptr().add(4) as *mut c_void,
            KEY_LENGTH,
        );
        set_keys(fd, keys.as_mut_ptr());
    } else {
        /* swap the keys */
        get_keys(fd, keys.as_mut_ptr());
        ptr::copy_nonoverlapping(
            keys.as_ptr().add(4) as *const c_void,
            tmp_key.as_mut_ptr() as *mut c_void,
            KEY_LENGTH,
        );
        ptr::copy(
            keys.as_ptr() as *const c_void,
            keys.as_mut_ptr().add(4) as *mut c_void,
            KEY_LENGTH,
        );
        ptr::copy_nonoverlapping(
            tmp_key.as_ptr() as *const c_void,
            keys.as_mut_ptr() as *mut c_void,
            KEY_LENGTH,
        );
        set_keys(fd, keys.as_mut_ptr());
    }
    ITER += 1;
    if ITER >= (N_LISTEN * 2) as c_int {
        ITER = 0;
    }
}

unsafe fn run_one_test(family: c_int) {
    let mut ev: epoll_event = mem::zeroed();
    let mut i: c_int;
    let mut send_fd: c_int;
    let mut n_loops: c_int = 10000;
    let mut rotate_key_fd: c_int = 0;
    let key_rotate_interval: c_int = 50;
    let fd: c_int;
    let epfd: c_int;
    let mut buf: [c_char; 1] = [0; 1];

    build_rcv_fd(family, SOCK_STREAM, RCV_FDS.as_mut_ptr());
    epfd = epoll_create(1);
    if epfd < 0 {
        error(1, errno(), c"failed to create epoll".as_ptr());
    }
    ev.events = EPOLLIN;
    for i_idx in 0..N_LISTEN {
        ev.data.fd = RCV_FDS[i_idx];
        if epoll_ctl(epfd, EPOLL_CTL_ADD, RCV_FDS[i_idx], &mut ev) != 0 {
            error(1, errno(), c"failed to register sock epoll".as_ptr());
        }
    }
    while {
        let old = n_loops;
        n_loops -= 1;
        old != 0
    } {
        send_fd = connect_and_send(family, SOCK_STREAM);
        if DO_ROTATE && (n_loops % key_rotate_interval) == 0 {
            rotate_key(RCV_FDS[rotate_key_fd as usize]);
            rotate_key_fd += 1;
            if rotate_key_fd >= N_LISTEN as c_int {
                rotate_key_fd = 0;
            }
        }
        loop {
            i = epoll_wait(epfd, &mut ev, 1, -1);
            if i < 0 {
                error(1, errno(), c"epoll_wait failed".as_ptr());
            }
            if is_listen_fd(ev.data.fd) {
                fd = accept(ev.data.fd, ptr::null_mut(), ptr::null_mut());
                if fd < 0 {
                    error(1, errno(), c"failed to accept".as_ptr());
                }
                ev.data.fd = fd;
                if epoll_ctl(epfd, EPOLL_CTL_ADD, fd, &mut ev) != 0 {
                    error(1, errno(), c"failed epoll add".as_ptr());
                }
                continue;
            }
            i = recv(
                ev.data.fd,
                buf.as_mut_ptr() as *mut c_void,
                buf.len(),
                0,
            ) as c_int;
            if i != 1 {
                error(1, errno(), c"failed recv data".as_ptr());
            }
            if epoll_ctl(epfd, EPOLL_CTL_DEL, ev.data.fd, ptr::null_mut()) != 0 {
                error(1, errno(), c"failed epoll del".as_ptr());
            }
            close(ev.data.fd);
            break;
        }
        close(send_fd);
    }
    for i_idx in 0..N_LISTEN {
        close(RCV_FDS[i_idx]);
    }
}

unsafe fn parse_opts(argc: c_int, argv: *mut *mut c_char) {
    let mut c: c_int;

    loop {
        c = getopt(argc, argv, c"46sr".as_ptr());
        if c == -1 {
            break;
        }
        match c as u8 as char {
            '4' => {
                DO_IPV6 = false;
            }
            '6' => {
                DO_IPV6 = true;
            }
            's' => {
                DO_SOCKOPT = true;
            }
            'r' => {
                DO_ROTATE = true;
                KEY_LEN = (KEY_LENGTH * 2) as c_int;
            }
            _ => {
                error(1, 0, c"%s: parse error".as_ptr(), *argv);
            }
        }
    }
}

unsafe fn c_main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    parse_opts(argc, argv);
    PROC_FD = open(PROC_FASTOPEN_KEY.as_ptr() as *const c_char, O_RDWR);
    if PROC_FD < 0 {
        error(
            1,
            errno(),
            c"Unable to open %s".as_ptr(),
            PROC_FASTOPEN_KEY.as_ptr() as *const c_char,
        );
    }
    srand(time(ptr::null_mut()) as c_uint);
    if DO_IPV6 {
        run_one_test(AF_INET6);
    } else {
        run_one_test(AF_INET);
    }
    close(PROC_FD);
    fprintf(stderr, c"PASS\n".as_ptr());
    0
}

fn main() {
    let mut args: Vec<*mut c_char> = std::env::args()
        .map(|arg| {
            std::ffi::CString::new(arg)
                .unwrap()
                .into_raw()
        })
        .collect();
    args.push(ptr::null_mut());
    unsafe {
        std::process::exit(c_main((args.len() - 1) as c_int, args.as_mut_ptr()));
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
