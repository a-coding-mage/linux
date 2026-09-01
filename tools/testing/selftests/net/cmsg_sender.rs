// SPDX-License-Identifier: GPL-2.0-or-later
// Translated from testing/selftests/net/cmsg_sender.c.
// C include dependencies: errno.h, error.h, netdb.h, stdbool.h, stdio.h,
// stdlib.h, string.h, time.h, unistd.h, linux/errqueue.h, linux/icmp.h,
// linux/icmpv6.h, linux/net_tstamp.h, linux/types.h, linux/udp.h,
// sys/socket.h, and "kselftest.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(static_mut_refs)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{copy_nonoverlapping, null_mut};

type size_t = usize;
type socklen_t = u32;
type sa_family_t = u16;
type time_t = c_long;
type __u32 = u32;
type __u64 = u64;

const ERN_SUCCESS: c_int = 0;
/* Well defined errors, callers may depend on these */
const ERN_SEND: c_int = 1;
/* Informational, can reorder */
const ERN_HELP: c_int = 2;
const ERN_SEND_SHORT: c_int = 3;
const ERN_SOCK_CREATE: c_int = 4;
const ERN_RESOLVE: c_int = 5;
const ERN_CMSG_WR: c_int = 6;
const ERN_SOCKOPT: c_int = 7;
const ERN_GETTIME: c_int = 8;
const ERN_RECVERR: c_int = 9;
const ERN_CMSG_RD: c_int = 10;
const ERN_CMSG_RCV: c_int = 11;
const ERN_SEND_MORE: c_int = 12;

const AF_UNSPEC: c_int = 0;
const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const SOCK_DGRAM: c_int = 2;
const SOCK_RAW: c_int = 3;
const SOL_SOCKET: c_int = 1;
const SOL_IP: c_int = 0;
const SOL_IPV6: c_int = 41;
const IPPROTO_UDP: c_int = 17;
const IPPROTO_ICMP: c_int = 1;
const IPPROTO_ICMPV6: c_int = 58;
const SO_MARK: c_int = 36;
const SO_PRIORITY: c_int = 12;
const SO_TIMESTAMPING: c_int = 37;
const SO_TIMESTAMPING_OLD: c_int = 37;
const SO_TXTIME: c_int = 61;
const SCM_TXTIME: c_int = SO_TXTIME;
const IP_TOS: c_int = 1;
const IP_TTL: c_int = 2;
const IP_RECVERR: c_int = 11;
const IPV6_DONTFRAG: c_int = 62;
const IPV6_TCLASS: c_int = 67;
const IPV6_HOPLIMIT: c_int = 52;
const IPV6_UNICAST_HOPS: c_int = 16;
const IPV6_RECVERR: c_int = 25;
const IPV6_HOPOPTS: c_int = 54;
const IPV6_DSTOPTS: c_int = 59;
const IPV6_RTHDRDSTOPTS: c_int = 55;
const CLOCK_REALTIME: c_int = 0;
const CLOCK_MONOTONIC: c_int = 1;
const MSG_ERRQUEUE: c_int = 0x2000;
const MSG_MORE: c_int = 0x8000;
const EAGAIN: c_int = 11;
const EFAULT: c_int = 14;
const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const ICMP_ECHO: u8 = 8;
const ICMPV6_ECHO_REQUEST: u8 = 128;
const SOF_TIMESTAMPING_TX_SOFTWARE: __u32 = 1 << 1;
const SOF_TIMESTAMPING_TX_SCHED: __u32 = 1 << 8;
const SOF_TIMESTAMPING_SOFTWARE: __u32 = 1 << 4;
const SOF_TIMESTAMPING_OPT_TSONLY: __u32 = 1 << 11;
const SCM_TSTAMP_SND: usize = 0;
const SCM_TSTAMP_SCHED: usize = 1;
const SCM_TSTAMP_ACK: usize = 2;

#[repr(C)]
struct timespec {
    tv_sec: time_t,
    tv_nsec: c_long,
}

#[repr(C)]
struct timeval {
    tv_sec: time_t,
    tv_usec: c_long,
}

#[repr(C)]
struct sock_txtime {
    clockid: c_int,
    flags: __u32,
}

#[repr(C)]
struct sockaddr {
    sa_family: sa_family_t,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct sockaddr_in6 {
    sin6_family: sa_family_t,
    sin6_port: u16,
    sin6_flowinfo: __u32,
    sin6_addr: [u8; 16],
    sin6_scope_id: __u32,
}

#[repr(C)]
struct addrinfo {
    ai_flags: c_int,
    ai_family: c_int,
    ai_socktype: c_int,
    ai_protocol: c_int,
    ai_addrlen: socklen_t,
    ai_addr: *mut sockaddr,
    ai_canonname: *mut c_char,
    ai_next: *mut addrinfo,
}

#[repr(C)]
struct iovec {
    iov_base: *mut c_void,
    iov_len: size_t,
}

#[repr(C)]
struct msghdr {
    msg_name: *mut c_void,
    msg_namelen: socklen_t,
    msg_iov: *mut iovec,
    msg_iovlen: size_t,
    msg_control: *mut c_void,
    msg_controllen: size_t,
    msg_flags: c_int,
}

#[repr(C)]
struct cmsghdr {
    cmsg_len: size_t,
    cmsg_level: c_int,
    cmsg_type: c_int,
}

#[repr(C)]
struct sock_extended_err {
    ee_errno: __u32,
    ee_origin: u8,
    ee_type: u8,
    ee_code: u8,
    ee_pad: u8,
    ee_info: __u32,
    ee_data: __u32,
}

#[repr(C)]
struct scm_timestamping {
    ts: [timespec; 3],
}

#[repr(C)]
struct udphdr {
    source: u16,
    dest: u16,
    len: u16,
    check: u16,
}

#[repr(C)]
struct option_cmsg_u32 {
    ena: bool,
    val: c_uint,
}

#[repr(C)]
struct options_sockopt {
    mark: c_uint,
    dontfrag: c_uint,
    tclass: c_uint,
    hlimit: c_uint,
    priority: c_uint,
}

#[repr(C)]
struct options_sock {
    family: c_uint,
    type_: c_uint,
    proto: c_uint,
}

#[repr(C)]
struct options_txtime {
    ena: bool,
    delay: c_uint,
}

#[repr(C)]
struct options_ts {
    ena: bool,
}

#[repr(C)]
struct options_cmsg {
    dontfrag: option_cmsg_u32,
    tclass: option_cmsg_u32,
    hlimit: option_cmsg_u32,
    exthdr: option_cmsg_u32,
}

#[repr(C)]
struct options {
    silent_send: bool,
    host: *const c_char,
    service: *const c_char,
    size: c_uint,
    num_pkt: c_uint,
    msg_more: bool,
    sockopt: options_sockopt,
    sock: options_sock,
    mark: option_cmsg_u32,
    priority: option_cmsg_u32,
    txtime: options_txtime,
    ts: options_ts,
    cmsg: options_cmsg,
}

static mut opt: options = options {
    silent_send: false,
    host: null_mut(),
    service: null_mut(),
    size: 13,
    num_pkt: 1,
    msg_more: false,
    sockopt: options_sockopt {
        mark: 0,
        dontfrag: 0,
        tclass: 0,
        hlimit: 0,
        priority: 0,
    },
    sock: options_sock {
        family: AF_UNSPEC as c_uint,
        type_: SOCK_DGRAM as c_uint,
        proto: IPPROTO_UDP as c_uint,
    },
    mark: option_cmsg_u32 { ena: false, val: 0 },
    priority: option_cmsg_u32 { ena: false, val: 0 },
    txtime: options_txtime { ena: false, delay: 0 },
    ts: options_ts { ena: false },
    cmsg: options_cmsg {
        dontfrag: option_cmsg_u32 { ena: false, val: 0 },
        tclass: option_cmsg_u32 { ena: false, val: 0 },
        hlimit: option_cmsg_u32 { ena: false, val: 0 },
        exthdr: option_cmsg_u32 { ena: false, val: 0 },
    },
};

static mut time_start_real: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
static mut time_start_mono: timespec = timespec { tv_sec: 0, tv_nsec: 0 };

unsafe extern "C" {
    static mut optarg: *mut c_char;
    static mut optind: c_int;
    fn __errno_location() -> *mut c_int;
    fn atoi(nptr: *const c_char) -> c_int;
    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn error(status: c_int, errnum: c_int, format: *const c_char, ...);
    fn exit(status: c_int) -> !;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn free(ptr: *mut c_void);
    fn freeaddrinfo(res: *mut addrinfo);
    fn getaddrinfo(
        node: *const c_char,
        service: *const c_char,
        hints: *const addrinfo,
        res: *mut *mut addrinfo,
    ) -> c_int;
    fn getopt(argc: c_int, argv: *const *mut c_char, optstring: *const c_char) -> c_int;
    fn htons(hostshort: u16) -> u16;
    fn malloc(size: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn printf(format: *const c_char, ...) -> c_int;
    fn rand() -> c_int;
    fn recvmsg(sockfd: c_int, msg: *mut msghdr, flags: c_int) -> isize;
    fn sendmsg(sockfd: c_int, msg: *const msghdr, flags: c_int) -> isize;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: socklen_t,
    ) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn usleep(usec: c_uint) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> isize;
    static mut stderr: *mut FILE;
}

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

unsafe fn errno() -> c_int {
    *__errno_location()
}

const fn cmsg_align(len: size_t) -> size_t {
    (len + size_of::<size_t>() - 1) & !(size_of::<size_t>() - 1)
}

const fn CMSG_SPACE(len: size_t) -> size_t {
    cmsg_align(size_of::<cmsghdr>()) + cmsg_align(len)
}

const fn CMSG_LEN(len: size_t) -> size_t {
    cmsg_align(size_of::<cmsghdr>()) + len
}

unsafe fn CMSG_DATA(cmsg: *mut cmsghdr) -> *mut u8 {
    (cmsg as *mut u8).add(cmsg_align(size_of::<cmsghdr>()))
}

unsafe fn CMSG_FIRSTHDR(msg: *mut msghdr) -> *mut cmsghdr {
    if (*msg).msg_controllen >= size_of::<cmsghdr>() {
        (*msg).msg_control as *mut cmsghdr
    } else {
        null_mut()
    }
}

unsafe fn CMSG_NXTHDR(msg: *mut msghdr, cmsg: *mut cmsghdr) -> *mut cmsghdr {
    let next = (cmsg as *mut u8).add(cmsg_align((*cmsg).cmsg_len)) as *mut cmsghdr;
    let max = ((*msg).msg_control as *mut u8).add((*msg).msg_controllen);
    if (next as *mut u8).add(size_of::<cmsghdr>()) > max {
        null_mut()
    } else {
        next
    }
}

unsafe fn cs_usage(bin: *const c_char) -> ! {
    printf(c"Usage: %s [opts] <dst host> <dst port / service>\n".as_ptr(), bin);
    printf(
        c"Options:\n\
\t\t-s      Silent send() failures\n\
\t\t-S      send() size\n\
\t\t-4/-6   Force IPv4 / IPv6 only\n\
\t\t-p prot Socket protocol\n\
\t\t        (u = UDP (default); i = ICMP; r = RAW;\n\
\t\t         U = UDP with MSG_MORE)\n\
\n\
\t\t-m val  Set SO_MARK with given value\n\
\t\t-M val  Set SO_MARK via setsockopt\n\
\t\t-P val  Set SO_PRIORITY via setsockopt\n\
\t\t-Q val  Set SO_PRIORITY via cmsg\n\
\t\t-d val  Set SO_TXTIME with given delay (usec)\n\
\t\t-t      Enable time stamp reporting\n\
\t\t-f val  Set don't fragment via cmsg\n\
\t\t-F val  Set don't fragment via setsockopt\n\
\t\t-c val  Set TOS/TCLASS via cmsg\n\
\t\t-C val  Set TOS/TCLASS via setsockopt\n\
\t\t-l val  Set TTL/HOPLIMIT via cmsg\n\
\t\t-L val  Set TTL/HOPLIMIT via setsockopt\n\
\t\t-H type Add an IPv6 header option\n\
\t\t        (h = HOP; d = DST; r = RTDST)\n\
\n"
        .as_ptr(),
    );
    exit(ERN_HELP);
}

unsafe fn cs_parse_args(argc: c_int, argv: *mut *mut c_char) {
    let mut o: c_int;

    loop {
        o = getopt(argc, argv, c"46sS:p:P:m:M:n:d:tf:F:c:C:l:L:H:Q:".as_ptr());
        if o == -1 {
            break;
        }
        match o as u8 as char {
            's' => opt.silent_send = true,
            'S' => opt.size = atoi(optarg) as c_uint,
            '4' => opt.sock.family = AF_INET as c_uint,
            '6' => opt.sock.family = AF_INET6 as c_uint,
            'p' => {
                if *optarg == b'u' as c_char {
                    opt.sock.proto = IPPROTO_UDP as c_uint;
                } else if *optarg == b'U' as c_char {
                    opt.sock.proto = IPPROTO_UDP as c_uint;
                    opt.msg_more = true;
                } else if *optarg == b'i' as c_char || *optarg == b'I' as c_char {
                    opt.sock.proto = IPPROTO_ICMP as c_uint;
                } else if *optarg == b'r' as c_char {
                    opt.sock.type_ = SOCK_RAW as c_uint;
                } else {
                    printf(c"Error: unknown protocol: %s\n".as_ptr(), optarg);
                    cs_usage(*argv);
                }
            }
            'P' => opt.sockopt.priority = atoi(optarg) as c_uint,
            'm' => {
                opt.mark.ena = true;
                opt.mark.val = atoi(optarg) as c_uint;
            }
            'Q' => {
                opt.priority.ena = true;
                opt.priority.val = atoi(optarg) as c_uint;
            }
            'M' => opt.sockopt.mark = atoi(optarg) as c_uint,
            'n' => opt.num_pkt = atoi(optarg) as c_uint,
            'd' => {
                opt.txtime.ena = true;
                opt.txtime.delay = atoi(optarg) as c_uint;
            }
            't' => opt.ts.ena = true,
            'f' => {
                opt.cmsg.dontfrag.ena = true;
                opt.cmsg.dontfrag.val = atoi(optarg) as c_uint;
            }
            'F' => opt.sockopt.dontfrag = atoi(optarg) as c_uint,
            'c' => {
                opt.cmsg.tclass.ena = true;
                opt.cmsg.tclass.val = atoi(optarg) as c_uint;
            }
            'C' => opt.sockopt.tclass = atoi(optarg) as c_uint,
            'l' => {
                opt.cmsg.hlimit.ena = true;
                opt.cmsg.hlimit.val = atoi(optarg) as c_uint;
            }
            'L' => opt.sockopt.hlimit = atoi(optarg) as c_uint,
            'H' => {
                opt.cmsg.exthdr.ena = true;
                match *optarg as u8 as char {
                    'h' => opt.cmsg.exthdr.val = IPV6_HOPOPTS as c_uint,
                    'd' => opt.cmsg.exthdr.val = IPV6_DSTOPTS as c_uint,
                    'r' => opt.cmsg.exthdr.val = IPV6_RTHDRDSTOPTS as c_uint,
                    _ => {
                        printf(c"Error: hdr type: %s\n".as_ptr(), optarg);
                    }
                }
            }
            _ => {}
        }
    }

    if optind != argc - 2 {
        cs_usage(*argv);
    }

    opt.host = *argv.add(optind as usize);
    opt.service = *argv.add((optind + 1) as usize);
}

unsafe fn memrnd(s: *mut c_void, mut n: size_t) {
    let mut dword = s as *mut c_int;
    let mut byte: *mut c_char;

    while n >= 4 {
        *dword = rand();
        dword = dword.add(1);
        n -= 4;
    }
    byte = dword as *mut c_char;
    while n != 0 {
        *byte = rand() as c_char;
        byte = byte.add(1);
        n -= 1;
    }
}

unsafe fn ca_write_cmsg_u32(
    cbuf: *mut c_char,
    cbuf_sz: size_t,
    cmsg_len: *mut size_t,
    level: c_int,
    optname: c_int,
    uopt: *mut option_cmsg_u32,
) {
    let cmsg: *mut cmsghdr;

    if !(*uopt).ena {
        return;
    }

    cmsg = cbuf.add(*cmsg_len) as *mut cmsghdr;
    *cmsg_len += CMSG_SPACE(size_of::<__u32>());
    if cbuf_sz < *cmsg_len {
        error(ERN_CMSG_WR, EFAULT, c"cmsg buffer too small".as_ptr());
    }

    (*cmsg).cmsg_level = level;
    (*cmsg).cmsg_type = optname;
    (*cmsg).cmsg_len = CMSG_LEN(size_of::<__u32>());
    *(CMSG_DATA(cmsg) as *mut __u32) = (*uopt).val;
}

unsafe fn cs_write_cmsg(fd: c_int, msg: *mut msghdr, cbuf: *mut c_char, cbuf_sz: size_t) {
    let mut cmsg: *mut cmsghdr;
    let mut cmsg_len: size_t;

    let _ = fd;
    (*msg).msg_control = cbuf as *mut c_void;
    cmsg_len = 0;

    ca_write_cmsg_u32(cbuf, cbuf_sz, &mut cmsg_len, SOL_SOCKET, SO_MARK, &raw mut opt.mark);
    ca_write_cmsg_u32(
        cbuf,
        cbuf_sz,
        &mut cmsg_len,
        SOL_SOCKET,
        SO_PRIORITY,
        &raw mut opt.priority,
    );

    if opt.sock.family == AF_INET as c_uint {
        ca_write_cmsg_u32(cbuf, cbuf_sz, &mut cmsg_len, SOL_IP, IP_TOS, &raw mut opt.cmsg.tclass);
        ca_write_cmsg_u32(cbuf, cbuf_sz, &mut cmsg_len, SOL_IP, IP_TTL, &raw mut opt.cmsg.hlimit);
    } else {
        ca_write_cmsg_u32(
            cbuf,
            cbuf_sz,
            &mut cmsg_len,
            SOL_IPV6,
            IPV6_DONTFRAG,
            &raw mut opt.cmsg.dontfrag,
        );
        ca_write_cmsg_u32(
            cbuf,
            cbuf_sz,
            &mut cmsg_len,
            SOL_IPV6,
            IPV6_TCLASS,
            &raw mut opt.cmsg.tclass,
        );
        ca_write_cmsg_u32(
            cbuf,
            cbuf_sz,
            &mut cmsg_len,
            SOL_IPV6,
            IPV6_HOPLIMIT,
            &raw mut opt.cmsg.hlimit,
        );
    }

    if opt.txtime.ena {
        let txtime: __u64;

        txtime = (time_start_mono.tv_sec as __u64) * (1000_u64 * 1000 * 1000)
            + time_start_mono.tv_nsec as __u64
            + (opt.txtime.delay as __u64) * 1000;

        cmsg = cbuf.add(cmsg_len) as *mut cmsghdr;
        cmsg_len += CMSG_SPACE(size_of::<__u64>());
        if cbuf_sz < cmsg_len {
            error(ERN_CMSG_WR, EFAULT, c"cmsg buffer too small".as_ptr());
        }

        (*cmsg).cmsg_level = SOL_SOCKET;
        (*cmsg).cmsg_type = SCM_TXTIME;
        (*cmsg).cmsg_len = CMSG_LEN(size_of::<__u64>());
        memcpy(
            CMSG_DATA(cmsg) as *mut c_void,
            &txtime as *const __u64 as *const c_void,
            size_of::<__u64>(),
        );
    }
    if opt.ts.ena {
        cmsg = cbuf.add(cmsg_len) as *mut cmsghdr;
        cmsg_len += CMSG_SPACE(size_of::<__u32>());
        if cbuf_sz < cmsg_len {
            error(ERN_CMSG_WR, EFAULT, c"cmsg buffer too small".as_ptr());
        }

        (*cmsg).cmsg_level = SOL_SOCKET;
        (*cmsg).cmsg_type = SO_TIMESTAMPING;
        (*cmsg).cmsg_len = CMSG_LEN(size_of::<__u32>());
        *(CMSG_DATA(cmsg) as *mut __u32) =
            SOF_TIMESTAMPING_TX_SCHED | SOF_TIMESTAMPING_TX_SOFTWARE;
    }
    if opt.cmsg.exthdr.ena {
        cmsg = cbuf.add(cmsg_len) as *mut cmsghdr;
        cmsg_len += CMSG_SPACE(8);
        if cbuf_sz < cmsg_len {
            error(ERN_CMSG_WR, EFAULT, c"cmsg buffer too small".as_ptr());
        }

        (*cmsg).cmsg_level = SOL_IPV6;
        (*cmsg).cmsg_type = opt.cmsg.exthdr.val as c_int;
        (*cmsg).cmsg_len = CMSG_LEN(8);
        *(CMSG_DATA(cmsg) as *mut __u64) = 0;
    }

    if cmsg_len != 0 {
        (*msg).msg_controllen = cmsg_len;
    } else {
        (*msg).msg_control = null_mut();
    }
}

unsafe fn cs_ts_info2str(info: c_uint) -> *const c_char {
    static N0: &[u8] = b"SND\0";
    static N1: &[u8] = b"SCHED\0";
    static N2: &[u8] = b"ACK\0";
    static UNKNOWN: &[u8] = b"unknown\0";
    static NAMES: [*const c_char; 3] = [
        N0.as_ptr() as *const c_char,
        N1.as_ptr() as *const c_char,
        N2.as_ptr() as *const c_char,
    ];

    if (info as usize) < NAMES.len() {
        return NAMES[info as usize];
    }
    UNKNOWN.as_ptr() as *const c_char
}

unsafe fn cs_read_cmsg(fd: c_int, msg: *mut msghdr, cbuf: *mut c_char, cbuf_sz: size_t) -> c_ulong {
    let mut see: *mut sock_extended_err;
    let mut ts: *mut scm_timestamping;
    let mut ts_seen: c_ulong = 0;
    let mut cmsg: *mut cmsghdr;
    let mut i: c_int;
    let mut err: c_int;

    if !opt.ts.ena {
        return 0;
    }
    (*msg).msg_control = cbuf as *mut c_void;
    (*msg).msg_controllen = cbuf_sz;

    loop {
        ts = null_mut();
        see = null_mut();
        memset(cbuf as *mut c_void, 0, cbuf_sz);

        err = recvmsg(fd, msg, MSG_ERRQUEUE) as c_int;
        if err < 0 {
            if errno() == EAGAIN {
                break;
            }
            error(ERN_RECVERR, errno(), c"recvmsg ERRQ".as_ptr());
        }

        cmsg = CMSG_FIRSTHDR(msg);
        while !cmsg.is_null() {
            if (*cmsg).cmsg_level == SOL_SOCKET && (*cmsg).cmsg_type == SO_TIMESTAMPING_OLD {
                if (*cmsg).cmsg_len < size_of::<scm_timestamping>() {
                    error(ERN_CMSG_RD, EINVAL, c"TS cmsg".as_ptr());
                }

                ts = CMSG_DATA(cmsg) as *mut scm_timestamping;
            }
            if ((*cmsg).cmsg_level == SOL_IP && (*cmsg).cmsg_type == IP_RECVERR)
                || ((*cmsg).cmsg_level == SOL_IPV6 && (*cmsg).cmsg_type == IPV6_RECVERR)
            {
                if (*cmsg).cmsg_len < size_of::<sock_extended_err>() {
                    error(ERN_CMSG_RD, EINVAL, c"sock_err cmsg".as_ptr());
                }

                see = CMSG_DATA(cmsg) as *mut sock_extended_err;
            }
            cmsg = CMSG_NXTHDR(msg, cmsg);
        }

        if ts.is_null() {
            error(ERN_CMSG_RCV, ENOENT, c"TS cmsg not found".as_ptr());
        }
        if see.is_null() {
            error(ERN_CMSG_RCV, ENOENT, c"sock_err cmsg not found".as_ptr());
        }

        i = 0;
        while i < 3 {
            let rel_time: u64;

            if (*ts).ts[i as usize].tv_sec == 0 && (*ts).ts[i as usize].tv_nsec == 0 {
                i += 1;
                continue;
            }

            rel_time = (((*ts).ts[i as usize].tv_sec - time_start_real.tv_sec) as u64)
                * (1000_u64 * 1000)
                + (((*ts).ts[i as usize].tv_nsec - time_start_real.tv_nsec) as u64) / 1000;
            printf(
                c" %5s ts%d %lluus\n".as_ptr(),
                cs_ts_info2str((*see).ee_info),
                i,
                rel_time,
            );
            ts_seen |= 1 << (*see).ee_info;
            i += 1;
        }
    }

    ts_seen
}

unsafe fn ca_set_sockopts(fd: c_int) {
    if opt.sockopt.mark != 0
        && setsockopt(
            fd,
            SOL_SOCKET,
            SO_MARK,
            &opt.sockopt.mark as *const c_uint as *const c_void,
            size_of::<c_uint>() as socklen_t,
        ) != 0
    {
        error(ERN_SOCKOPT, errno(), c"setsockopt SO_MARK".as_ptr());
    }
    if opt.sockopt.priority != 0
        && setsockopt(
            fd,
            SOL_SOCKET,
            SO_PRIORITY,
            &opt.sockopt.priority as *const c_uint as *const c_void,
            size_of::<c_uint>() as socklen_t,
        ) != 0
    {
        error(ERN_SOCKOPT, errno(), c"setsockopt SO_PRIORITY".as_ptr());
    }

    if opt.sock.family == AF_INET as c_uint {
        if opt.sockopt.tclass != 0
            && setsockopt(
                fd,
                SOL_IP,
                IP_TOS,
                &opt.sockopt.tclass as *const c_uint as *const c_void,
                size_of::<c_uint>() as socklen_t,
            ) != 0
        {
            error(ERN_SOCKOPT, errno(), c"setsockopt IP_TOS".as_ptr());
        }
        if opt.sockopt.hlimit != 0
            && setsockopt(
                fd,
                SOL_IP,
                IP_TTL,
                &opt.sockopt.hlimit as *const c_uint as *const c_void,
                size_of::<c_uint>() as socklen_t,
            ) != 0
        {
            error(ERN_SOCKOPT, errno(), c"setsockopt IP_TTL".as_ptr());
        }
    } else {
        if opt.sockopt.dontfrag != 0
            && setsockopt(
                fd,
                SOL_IPV6,
                IPV6_DONTFRAG,
                &opt.sockopt.dontfrag as *const c_uint as *const c_void,
                size_of::<c_uint>() as socklen_t,
            ) != 0
        {
            error(ERN_SOCKOPT, errno(), c"setsockopt IPV6_DONTFRAG".as_ptr());
        }
        if opt.sockopt.tclass != 0
            && setsockopt(
                fd,
                SOL_IPV6,
                IPV6_TCLASS,
                &opt.sockopt.tclass as *const c_uint as *const c_void,
                size_of::<c_uint>() as socklen_t,
            ) != 0
        {
            error(ERN_SOCKOPT, errno(), c"setsockopt IPV6_TCLASS".as_ptr());
        }
        if opt.sockopt.hlimit != 0
            && setsockopt(
                fd,
                SOL_IPV6,
                IPV6_UNICAST_HOPS,
                &opt.sockopt.hlimit as *const c_uint as *const c_void,
                size_of::<c_uint>() as socklen_t,
            ) != 0
        {
            error(ERN_SOCKOPT, errno(), c"setsockopt IPV6_HOPLIMIT".as_ptr());
        }
    }

    if opt.txtime.ena {
        let so_txtime = sock_txtime {
            clockid: CLOCK_MONOTONIC,
            flags: 0,
        };

        if setsockopt(
            fd,
            SOL_SOCKET,
            SO_TXTIME,
            &so_txtime as *const sock_txtime as *const c_void,
            size_of::<sock_txtime>() as socklen_t,
        ) != 0
        {
            error(ERN_SOCKOPT, errno(), c"setsockopt TXTIME".as_ptr());
        }
    }
    if opt.ts.ena {
        let val: __u32 = SOF_TIMESTAMPING_SOFTWARE | SOF_TIMESTAMPING_OPT_TSONLY;

        if setsockopt(
            fd,
            SOL_SOCKET,
            SO_TIMESTAMPING,
            &val as *const __u32 as *const c_void,
            size_of::<__u32>() as socklen_t,
        ) != 0
        {
            error(ERN_SOCKOPT, errno(), c"setsockopt TIMESTAMPING".as_ptr());
        }
    }
}

unsafe fn c_main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut hints: addrinfo = zeroed();
    let mut ai: *mut addrinfo;
    let mut iov: [iovec; 1] = [zeroed()];
    let mut buf: *mut u8;
    let mut msg: msghdr = zeroed();
    let mut cbuf: [c_char; 1024] = [0; 1024];
    let mut err: c_int;
    let fd: c_int;
    let mut i: c_int;

    cs_parse_args(argc, argv);

    buf = malloc(opt.size as size_t) as *mut u8;
    memrnd(buf as *mut c_void, opt.size as size_t);

    memset(
        &mut hints as *mut addrinfo as *mut c_void,
        0,
        size_of::<addrinfo>(),
    );
    hints.ai_family = opt.sock.family as c_int;

    ai = null_mut();
    err = getaddrinfo(opt.host, opt.service, &hints, &mut ai);
    if err != 0 {
        fprintf(
            stderr,
            c"Can't resolve address [%s]:%s\n".as_ptr(),
            opt.host,
            opt.service,
        );
        err = ERN_SOCK_CREATE;
        goto_err_free_buff(buf, ai, -1, err);
    }

    if (*ai).ai_family == AF_INET6 && opt.sock.proto == IPPROTO_ICMP as c_uint {
        opt.sock.proto = IPPROTO_ICMPV6 as c_uint;
    }

    fd = socket((*ai).ai_family, opt.sock.type_ as c_int, opt.sock.proto as c_int);
    if fd < 0 {
        fprintf(stderr, c"Can't open socket: %s\n".as_ptr(), strerror(errno()));
        err = ERN_RESOLVE;
        goto_err_free_info(buf, ai, err);
    }

    if opt.sock.proto == IPPROTO_ICMP as c_uint {
        *buf.add(0) = ICMP_ECHO;
        *buf.add(1) = 0;
    } else if opt.sock.proto == IPPROTO_ICMPV6 as c_uint {
        *buf.add(0) = ICMPV6_ECHO_REQUEST;
        *buf.add(1) = 0;
    } else if opt.sock.type_ == SOCK_RAW as c_uint {
        let hdr = udphdr {
            source: 1,
            dest: 2,
            len: htons(opt.size as u16),
            check: 0,
        };
        let sin6 = (*ai).ai_addr as *mut sockaddr_in6;

        memcpy(
            buf as *mut c_void,
            &hdr as *const udphdr as *const c_void,
            size_of::<udphdr>(),
        );
        (*sin6).sin6_port = htons(opt.sock.proto as u16);
    }

    ca_set_sockopts(fd);

    if clock_gettime(CLOCK_REALTIME, &raw mut time_start_real) != 0 {
        error(ERN_GETTIME, errno(), c"gettime REALTIME".as_ptr());
    }
    if clock_gettime(CLOCK_MONOTONIC, &raw mut time_start_mono) != 0 {
        error(ERN_GETTIME, errno(), c"gettime MONOTONIC".as_ptr());
    }

    iov[0].iov_base = buf as *mut c_void;
    iov[0].iov_len = opt.size as size_t;

    memset(
        &mut msg as *mut msghdr as *mut c_void,
        0,
        size_of::<msghdr>(),
    );
    msg.msg_name = (*ai).ai_addr as *mut c_void;
    msg.msg_namelen = (*ai).ai_addrlen;
    msg.msg_iov = iov.as_mut_ptr();
    msg.msg_iovlen = 1;

    cs_write_cmsg(fd, &mut msg, cbuf.as_mut_ptr(), size_of::<[c_char; 1024]>());

    i = 0;
    while i < opt.num_pkt as c_int {
        err = sendmsg(fd, &msg, if opt.msg_more { MSG_MORE } else { 0 }) as c_int;
        if err < 0 {
            if !opt.silent_send {
                fprintf(stderr, c"send failed: %s\n".as_ptr(), strerror(errno()));
            }
            err = ERN_SEND;
            goto_err_out(buf, ai, fd, err);
        } else if err != opt.size as c_int {
            fprintf(stderr, c"short send\n".as_ptr());
            err = ERN_SEND_SHORT;
            goto_err_out(buf, ai, fd, err);
        }
        if opt.msg_more {
            err = write(fd, null_mut(), 0) as c_int;
            if err < 0 {
                fprintf(stderr, c"send more: %s\n".as_ptr(), strerror(errno()));
                err = ERN_SEND_MORE;
                goto_err_out(buf, ai, fd, err);
            }
        }
        i += 1;
    }
    err = ERN_SUCCESS;

    if opt.ts.ena {
        let mut seen: c_ulong;
        let mut i: c_int;

        /* Make sure all timestamps have time to loop back */
        i = 0;
        while i < 40 {
            seen = cs_read_cmsg(fd, &mut msg, cbuf.as_mut_ptr(), size_of::<[c_char; 1024]>());
            if (seen & (1 << SCM_TSTAMP_SND)) != 0 {
                break;
            }
            usleep(opt.txtime.delay / 20);
            i += 1;
        }
    }

    close(fd);
    freeaddrinfo(ai);
    free(buf as *mut c_void);
    err
}

unsafe fn goto_err_out(buf: *mut u8, ai: *mut addrinfo, fd: c_int, err: c_int) -> ! {
    close(fd);
    goto_err_free_info(buf, ai, err);
}

unsafe fn goto_err_free_info(buf: *mut u8, ai: *mut addrinfo, err: c_int) -> ! {
    freeaddrinfo(ai);
    goto_err_free_buff(buf, ai, err);
}

unsafe fn goto_err_free_buff(buf: *mut u8, _ai: *mut addrinfo, err: c_int) -> ! {
    free(buf as *mut c_void);
    exit(err);
}

fn main() {
    unsafe extern "C" {
        static mut __libc_argv: *mut *mut c_char;
        static mut __libc_argc: c_int;
    }

    unsafe {
        let code = c_main(__libc_argc, __libc_argv);
        if code != 0 {
            exit(code);
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
