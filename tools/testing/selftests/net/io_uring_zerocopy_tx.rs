/* SPDX-License-Identifier: MIT */
/* based on linux-kernel/tools/testing/selftests/net/msg_zerocopy.c */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

/* C dependencies:
 * assert.h, errno.h, error.h, fcntl.h, limits.h, stdbool.h, stdint.h,
 * stdio.h, stdlib.h, string.h, unistd.h, arpa/inet.h, linux/errqueue.h,
 * linux/if_packet.h, linux/io_uring.h, linux/ipv6.h, linux/socket.h,
 * linux/sockios.h, net/ethernet.h, net/if.h, netinet/in.h, netinet/ip.h,
 * netinet/ip6.h, netinet/tcp.h, netinet/udp.h, sys/ioctl.h, sys/mman.h,
 * sys/resource.h, sys/socket.h, sys/stat.h, sys/time.h, sys/types.h,
 * sys/un.h, sys/wait.h, io_uring/mini_liburing.h.
 */

const NOTIF_TAG: u64 = 0xfffffff_u64;
const NONZC_TAG: u64 = 0;
const ZC_TAG: u64 = 1;

const MODE_NONZC: c_int = 0;
const MODE_ZC: c_int = 1;
const MODE_ZC_FIXED: c_int = 2;
const MODE_MIXED: c_int = 3;

static mut CFG_CORK: bool = false;
static mut CFG_MODE: c_int = MODE_ZC_FIXED;
static mut CFG_NR_REQS: c_int = 8;
static mut CFG_FAMILY: c_int = PF_UNSPEC;
static mut CFG_PAYLOAD_LEN: c_int = 0;
static mut CFG_PORT: c_int = 8000;
static mut CFG_RUNTIME_MS: c_int = 4200;

static mut CFG_ALEN: socklen_t = 0;
static mut CFG_DST_ADDR: sockaddr_storage = sockaddr_storage {
    ss_family: 0,
    __ss_padding: [0; 118],
    __ss_align: 0,
};

#[repr(align(4096))]
struct AlignedPayload([c_char; IP_MAXPACKET]);

static mut PAYLOAD: AlignedPayload = AlignedPayload([0; IP_MAXPACKET]);

type socklen_t = u32;
type size_t = usize;
type ssize_t = isize;
type time_t = i64;
type suseconds_t = i64;

const IP_MAXPACKET: usize = 65535;
const PF_UNSPEC: c_int = 0;
const PF_INET: c_int = 2;
const PF_INET6: c_int = 10;
const AF_INET: c_int = PF_INET;
const AF_INET6: c_int = PF_INET6;
const SOCK_STREAM: c_int = 1;
const SOCK_DGRAM: c_int = 2;
const SOL_SOCKET: c_int = 1;
const SO_SNDBUF: c_int = 7;
const IPPROTO_UDP: c_int = 17;
const UDP_CORK: c_int = 1;
const MSG_WAITALL: c_uint = 0x100;
const EAGAIN: c_int = 11;
const EINVAL: c_int = 22;
const IORING_RECVSEND_FIXED_BUF: c_uint = 1 << 2;
const IORING_CQE_F_MORE: c_uint = 1 << 1;
const IORING_CQE_F_NOTIF: c_uint = 1 << 2;

#[repr(C)]
struct timeval {
    tv_sec: time_t,
    tv_usec: suseconds_t,
}

#[repr(C)]
struct iovec {
    iov_base: *mut c_void,
    iov_len: size_t,
}

#[repr(C)]
struct in_addr {
    s_addr: u32,
}

#[repr(C)]
struct in6_addr {
    s6_addr: [u8; 16],
}

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct sockaddr_in {
    sin_family: u16,
    sin_port: u16,
    sin_addr: in_addr,
    sin_zero: [u8; 8],
}

#[repr(C)]
struct sockaddr_in6 {
    sin6_family: u16,
    sin6_port: u16,
    sin6_flowinfo: u32,
    sin6_addr: in6_addr,
    sin6_scope_id: u32,
}

#[repr(C)]
struct sockaddr_storage {
    ss_family: u16,
    __ss_padding: [u8; 118],
    __ss_align: u64,
}

#[repr(C)]
struct ipv6hdr {
    _private: [u8; 40],
}

#[repr(C)]
struct tcphdr {
    _private: [u8; 20],
}

#[repr(C)]
struct io_uring {
    _private: [u8; 0],
}

#[repr(C)]
struct io_uring_sqe {
    _private0: [u8; 18],
    ioprio: c_uint,
    _private1: [u8; 20],
    user_data: u64,
    buf_index: u16,
}

#[repr(C)]
struct io_uring_cqe {
    user_data: u64,
    res: c_int,
    flags: c_uint,
}

unsafe extern "C" {
    static mut errno: c_int;
    static mut optarg: *mut c_char;
    static mut optind: c_int;
    static mut stderr: *mut c_void;

    fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;
    fn setsockopt(
        fd: c_int,
        level: c_int,
        optname: c_int,
        optval: *const c_void,
        optlen: socklen_t,
    ) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn connect(fd: c_int, addr: *const sockaddr, len: socklen_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn error(status: c_int, errnum: c_int, format: *const c_char, ...) -> !;
    fn rand() -> c_int;
    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> u64;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> i64;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn htons(hostshort: u16) -> u16;
    fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;

    fn io_uring_queue_init(entries: c_uint, ring: *mut io_uring, flags: c_uint) -> c_int;
    fn io_uring_register_buffers(
        ring: *mut io_uring,
        iovecs: *const iovec,
        nr_iovecs: c_uint,
    ) -> c_int;
    fn io_uring_get_sqe(ring: *mut io_uring) -> *mut io_uring_sqe;
    fn io_uring_prep_send(
        sqe: *mut io_uring_sqe,
        sockfd: c_int,
        buf: *const c_void,
        len: size_t,
        flags: c_int,
    );
    fn io_uring_prep_sendzc(
        sqe: *mut io_uring_sqe,
        sockfd: c_int,
        buf: *const c_void,
        len: size_t,
        flags: c_int,
        zc_flags: c_uint,
    );
    fn io_uring_submit(ring: *mut io_uring) -> c_int;
    fn io_uring_wait_cqe(ring: *mut io_uring, cqe_ptr: *mut *mut io_uring_cqe) -> c_int;
    fn io_uring_cqe_seen(ring: *mut io_uring, cqe: *mut io_uring_cqe);
}

unsafe fn gettimeofday_ms() -> u64 {
    let mut tv: timeval = zeroed();

    gettimeofday(&mut tv, ptr::null_mut());
    (tv.tv_sec as u64 * 1000) + (tv.tv_usec as u64 / 1000)
}

unsafe fn do_setsockopt(fd: c_int, level: c_int, optname: c_int, val: c_int) {
    if setsockopt(
        fd,
        level,
        optname,
        &val as *const c_int as *const c_void,
        size_of::<c_int>() as socklen_t,
    ) != 0
    {
        error(
            1,
            errno,
            c"setsockopt %d.%d: %d".as_ptr(),
            level,
            optname,
            val,
        );
    }
}

unsafe fn do_setup_tx(domain: c_int, type_: c_int, protocol: c_int) -> c_int {
    let fd: c_int;

    fd = socket(domain, type_, protocol);
    if fd == -1 {
        error(1, errno, c"socket t".as_ptr());
    }

    do_setsockopt(fd, SOL_SOCKET, SO_SNDBUF, 1 << 21);

    if connect(
        fd,
        &CFG_DST_ADDR as *const sockaddr_storage as *const sockaddr,
        CFG_ALEN,
    ) != 0
    {
        error(1, errno, c"connect".as_ptr());
    }
    fd
}

unsafe fn do_tx(domain: c_int, type_: c_int, protocol: c_int) {
    let mut sqe: *mut io_uring_sqe;
    let mut cqe: *mut io_uring_cqe;
    let mut packets: u64 = 0;
    let mut bytes: u64 = 0;
    let mut ring: io_uring = zeroed();
    let mut iov: iovec = zeroed();
    let tstop: u64;
    let mut i: c_int;
    let fd: c_int;
    let mut ret: c_int;
    let mut compl_cqes: c_int = 0;

    fd = do_setup_tx(domain, type_, protocol);

    ret = io_uring_queue_init(512, &mut ring, 0);
    if ret != 0 {
        error(1, -ret, c"io_uring: queue init".as_ptr());
    }

    iov.iov_base = PAYLOAD.0.as_mut_ptr() as *mut c_void;
    iov.iov_len = CFG_PAYLOAD_LEN as size_t;

    ret = io_uring_register_buffers(&mut ring, &iov, 1);
    if ret != 0 {
        error(1, -ret, c"io_uring: buffer registration".as_ptr());
    }

    tstop = gettimeofday_ms().wrapping_add(CFG_RUNTIME_MS as u64);
    loop {
        if CFG_CORK {
            do_setsockopt(fd, IPPROTO_UDP, UDP_CORK, 1);
        }

        i = 0;
        while i < CFG_NR_REQS {
            let zc_flags: c_uint = 0;
            let buf_idx: c_uint = 0;
            let mut mode: c_uint = CFG_MODE as c_uint;
            let msg_flags: c_uint = MSG_WAITALL;

            if CFG_MODE == MODE_MIXED {
                mode = (rand() % 3) as c_uint;
            }

            sqe = io_uring_get_sqe(&mut ring);

            if mode == MODE_NONZC as c_uint {
                io_uring_prep_send(
                    sqe,
                    fd,
                    PAYLOAD.0.as_ptr() as *const c_void,
                    CFG_PAYLOAD_LEN as size_t,
                    msg_flags as c_int,
                );
                (*sqe).user_data = NONZC_TAG;
            } else {
                io_uring_prep_sendzc(
                    sqe,
                    fd,
                    PAYLOAD.0.as_ptr() as *const c_void,
                    CFG_PAYLOAD_LEN as size_t,
                    msg_flags as c_int,
                    zc_flags,
                );
                if mode == MODE_ZC_FIXED as c_uint {
                    (*sqe).ioprio |= IORING_RECVSEND_FIXED_BUF;
                    (*sqe).buf_index = buf_idx as u16;
                }
                (*sqe).user_data = ZC_TAG;
            }
            i += 1;
        }

        ret = io_uring_submit(&mut ring);
        if ret != CFG_NR_REQS {
            error(1, -ret, c"submit".as_ptr());
        }

        if CFG_CORK {
            do_setsockopt(fd, IPPROTO_UDP, UDP_CORK, 0);
        }
        i = 0;
        while i < CFG_NR_REQS {
            cqe = ptr::null_mut();
            ret = io_uring_wait_cqe(&mut ring, &mut cqe);
            if ret != 0 {
                error(1, -ret, c"wait cqe".as_ptr());
            }

            if (*cqe).user_data != NONZC_TAG && (*cqe).user_data != ZC_TAG {
                error(1, EINVAL, c"invalid cqe->user_data".as_ptr());
            }

            if ((*cqe).flags & IORING_CQE_F_NOTIF) != 0 {
                if ((*cqe).flags & IORING_CQE_F_MORE) != 0 {
                    error(1, EINVAL, c"invalid notif flags".as_ptr());
                }
                if compl_cqes <= 0 {
                    error(1, EINVAL, c"notification mismatch".as_ptr());
                }
                compl_cqes -= 1;
                i -= 1;
                io_uring_cqe_seen(&mut ring, cqe);
                i += 1;
                continue;
            }
            if ((*cqe).flags & IORING_CQE_F_MORE) != 0 {
                if (*cqe).user_data != ZC_TAG {
                    error(1, -(*cqe).res, c"unexpected F_MORE".as_ptr());
                }
                compl_cqes += 1;
            }
            if (*cqe).res >= 0 {
                packets = packets.wrapping_add(1);
                bytes = bytes.wrapping_add((*cqe).res as u64);
            } else if (*cqe).res != -EAGAIN {
                error(1, -(*cqe).res, c"send failed".as_ptr());
            }
            io_uring_cqe_seen(&mut ring, cqe);
            i += 1;
        }
        if gettimeofday_ms() >= tstop {
            break;
        }
    }

    while compl_cqes != 0 {
        cqe = ptr::null_mut();
        ret = io_uring_wait_cqe(&mut ring, &mut cqe);
        if ret != 0 {
            error(1, -ret, c"wait cqe".as_ptr());
        }
        if ((*cqe).flags & IORING_CQE_F_MORE) != 0 {
            error(1, EINVAL, c"invalid notif flags".as_ptr());
        }
        if ((*cqe).flags & IORING_CQE_F_NOTIF) == 0 {
            error(1, EINVAL, c"missing notif flag".as_ptr());
        }

        io_uring_cqe_seen(&mut ring, cqe);
        compl_cqes -= 1;
    }

    fprintf(
        stderr,
        c"tx=%lu (MB=%lu), tx/s=%lu (MB/s=%lu)\n".as_ptr(),
        packets,
        bytes >> 20,
        packets / (CFG_RUNTIME_MS as u64 / 1000),
        (bytes >> 20) / (CFG_RUNTIME_MS as u64 / 1000),
    );

    if close(fd) != 0 {
        error(1, errno, c"close".as_ptr());
    }
}

unsafe fn do_test(domain: c_int, type_: c_int, protocol: c_int) {
    let mut i: c_int;

    i = 0;
    while i < IP_MAXPACKET as c_int {
        PAYLOAD.0[i as usize] = (b'a' + (i % 26) as u8) as c_char;
        i += 1;
    }
    do_tx(domain, type_, protocol);
}

unsafe fn usage(filepath: *const c_char) {
    error(
        1,
        0,
        c"Usage: %s (-4|-6) (udp|tcp) -D<dst_ip> [-s<payload size>] [-t<time s>] [-n<batch>] [-p<port>] [-m<mode>]".as_ptr(),
        filepath,
    );
}

unsafe fn parse_opts(argc: c_int, argv: *mut *mut c_char) {
    let max_payload_len: c_int = (size_of::<AlignedPayload>()
        - size_of::<ipv6hdr>()
        - size_of::<tcphdr>()
        - 40 /* max tcp options */) as c_int;
    let addr6: *mut sockaddr_in6 = &mut CFG_DST_ADDR as *mut sockaddr_storage as *mut sockaddr_in6;
    let addr4: *mut sockaddr_in = &mut CFG_DST_ADDR as *mut sockaddr_storage as *mut sockaddr_in;
    let mut daddr: *mut c_char = ptr::null_mut();
    let mut c: c_int;

    if argc <= 1 {
        usage(*argv.add(0));
    }
    CFG_PAYLOAD_LEN = max_payload_len;

    loop {
        c = getopt(argc, argv, c"46D:p:s:t:n:c:m:".as_ptr());
        if c == -1 {
            break;
        }
        match c as u8 as char {
            '4' => {
                if CFG_FAMILY != PF_UNSPEC {
                    error(1, 0, c"Pass one of -4 or -6".as_ptr());
                }
                CFG_FAMILY = PF_INET;
                CFG_ALEN = size_of::<sockaddr_in>() as socklen_t;
            }
            '6' => {
                if CFG_FAMILY != PF_UNSPEC {
                    error(1, 0, c"Pass one of -4 or -6".as_ptr());
                }
                CFG_FAMILY = PF_INET6;
                CFG_ALEN = size_of::<sockaddr_in6>() as socklen_t;
            }
            'D' => {
                daddr = optarg;
            }
            'p' => {
                CFG_PORT = strtoul(optarg, ptr::null_mut(), 0) as c_int;
            }
            's' => {
                CFG_PAYLOAD_LEN = strtoul(optarg, ptr::null_mut(), 0) as c_int;
            }
            't' => {
                CFG_RUNTIME_MS = 200 + (strtoul(optarg, ptr::null_mut(), 10) as c_int) * 1000;
            }
            'n' => {
                CFG_NR_REQS = strtoul(optarg, ptr::null_mut(), 0) as c_int;
            }
            'c' => {
                CFG_CORK = strtol(optarg, ptr::null_mut(), 0) != 0;
            }
            'm' => {
                CFG_MODE = strtol(optarg, ptr::null_mut(), 0) as c_int;
            }
            _ => {}
        }
    }

    match CFG_FAMILY {
        PF_INET => {
            memset(
                addr4 as *mut c_void,
                0,
                size_of::<sockaddr_in>() as size_t,
            );
            (*addr4).sin_family = AF_INET as u16;
            (*addr4).sin_port = htons(CFG_PORT as u16);
            if !daddr.is_null()
                && inet_pton(
                    AF_INET,
                    daddr,
                    &mut (*addr4).sin_addr as *mut in_addr as *mut c_void,
                ) != 1
            {
                error(1, 0, c"ipv4 parse error: %s".as_ptr(), daddr);
            }
        }
        PF_INET6 => {
            memset(
                addr6 as *mut c_void,
                0,
                size_of::<sockaddr_in6>() as size_t,
            );
            (*addr6).sin6_family = AF_INET6 as u16;
            (*addr6).sin6_port = htons(CFG_PORT as u16);
            if !daddr.is_null()
                && inet_pton(
                    AF_INET6,
                    daddr,
                    &mut (*addr6).sin6_addr as *mut in6_addr as *mut c_void,
                ) != 1
            {
                error(1, 0, c"ipv6 parse error: %s".as_ptr(), daddr);
            }
        }
        _ => {
            error(1, 0, c"illegal domain".as_ptr());
        }
    }

    if CFG_PAYLOAD_LEN > max_payload_len {
        error(
            1,
            0,
            c"-s: payload exceeds max (%d)".as_ptr(),
            max_payload_len,
        );
    }
    if optind != argc - 1 {
        usage(*argv.add(0));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let cfg_test: *const c_char = *argv.add((argc - 1) as usize);

    parse_opts(argc, argv);

    if strcmp(cfg_test, c"tcp".as_ptr()) == 0 {
        do_test(CFG_FAMILY, SOCK_STREAM, 0);
    } else if strcmp(cfg_test, c"udp".as_ptr()) == 0 {
        do_test(CFG_FAMILY, SOCK_DGRAM, 0);
    } else {
        error(1, 0, c"unknown cfg_test %s".as_ptr(), cfg_test);
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
