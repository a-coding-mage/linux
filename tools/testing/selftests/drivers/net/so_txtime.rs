// SPDX-License-Identifier: GPL-2.0
/*
 * Test the SO_TXTIME API
 *
 * Takes a stream of { payload, delivery time }[], to be sent across two
 * processes. Start this program on two separate network namespaces or
 * connected hosts, one instance in transmit mode and the other in receive
 * mode using the '-r' option. Receiver will compare arrival timestamps to
 * the expected stream. Sender will read transmit timestamps from the error
 * queue. The streams can differ due to out-of-order delivery and drops.
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

type uint16_t = u16;
type uint64_t = u64;
type int64_t = i64;
type clockid_t = libc::clockid_t;
type socklen_t = libc::socklen_t;

const MAX_NUM_PKT: usize = 8;
const KSFT_PASS: c_int = 0;
const KSFT_FAIL: c_int = 1;

/* Constants supplied by Linux headers in the C source. */
const SO_TXTIME: c_int = 61;
const SCM_TXTIME: c_int = SO_TXTIME;
const SOF_TXTIME_REPORT_ERRORS: u32 = 1 << 1;
const SO_EE_ORIGIN_TXTIME: u8 = 6;
const SO_EE_CODE_TXTIME_INVALID_PARAM: u8 = 1;
const SO_EE_CODE_TXTIME_MISSED: u8 = 2;

#[repr(C)]
#[derive(Copy, Clone)]
struct sock_txtime {
    clockid: clockid_t,
    flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct sock_extended_err {
    ee_errno: u32,
    ee_origin: u8,
    ee_type: u8,
    ee_code: u8,
    ee_pad: u8,
    ee_info: u32,
    ee_data: u32,
}

/* encode one timed transmission (of a 1B payload) */
#[repr(C)]
#[derive(Copy, Clone)]
struct timed_send {
    data: c_char,
    delay_us: int64_t,
}

static mut cfg_clockid: c_int = libc::CLOCK_TAI;
static mut cfg_port: uint16_t = 8000;
static mut cfg_variance_us: c_int = 8000;
static mut cfg_machine_slow: bool = false;
static mut cfg_start_time_ns: uint64_t = 0;
static mut cfg_mark: c_int = 0;
static mut cfg_rx: bool = false;

static mut glob_tstart: uint64_t = 0;
static mut tdeliver_max: uint64_t = 0;

static mut errors: c_int = 0;

static mut cfg_buf: [timed_send; MAX_NUM_PKT] = [timed_send {
    data: 0,
    delay_us: 0,
}; MAX_NUM_PKT];
static mut cfg_num_pkt: c_int = 0;

static mut cfg_errq_level: c_int = 0;
static mut cfg_errq_type: c_int = 0;

static mut cfg_dst_addr: libc::sockaddr_storage = unsafe { mem::zeroed() };
static mut cfg_src_addr: libc::sockaddr_storage = unsafe { mem::zeroed() };
static mut cfg_alen: socklen_t = 0;

unsafe extern "C" {
    fn error(status: c_int, errnum: c_int, format: *const c_char, ...);
    static mut optarg: *mut c_char;
    static mut optind: c_int;
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

unsafe fn cmsg_align(len: usize) -> usize {
    (len + mem::size_of::<usize>() - 1) & !(mem::size_of::<usize>() - 1)
}

unsafe fn cmsg_space(len: usize) -> usize {
    cmsg_align(mem::size_of::<libc::cmsghdr>()) + cmsg_align(len)
}

unsafe fn cmsg_len(len: usize) -> usize {
    cmsg_align(mem::size_of::<libc::cmsghdr>()) + len
}

unsafe fn cmsg_firsthdr(mhdr: *mut libc::msghdr) -> *mut libc::cmsghdr {
    if (*mhdr).msg_controllen < mem::size_of::<libc::cmsghdr>() {
        ptr::null_mut()
    } else {
        (*mhdr).msg_control as *mut libc::cmsghdr
    }
}

unsafe fn cmsg_data(cmsg: *mut libc::cmsghdr) -> *mut c_uchar {
    (cmsg as *mut c_uchar).add(cmsg_align(mem::size_of::<libc::cmsghdr>()))
}

type c_uchar = u8;

unsafe fn gettime_ns(clock: clockid_t) -> uint64_t {
    let mut ts: libc::timespec = mem::zeroed();

    if libc::clock_gettime(clock, &mut ts) != 0 {
        error(1, *libc::__errno_location(), cstr!("gettime"));
    }

    ts.tv_sec as uint64_t * (1000u64 * 1000 * 1000) + ts.tv_nsec as uint64_t
}

unsafe fn do_send_one(fdt: c_int, ts: *mut timed_send) {
    let mut control = [0u8; 64];
    let mut msg: libc::msghdr = mem::zeroed();
    let mut iov: libc::iovec = mem::zeroed();
    let mut tdeliver: uint64_t;
    let mut ret: c_int;

    iov.iov_base = &mut (*ts).data as *mut _ as *mut c_void;
    iov.iov_len = 1;

    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1;
    msg.msg_name = &mut cfg_dst_addr as *mut _ as *mut c_void;
    msg.msg_namelen = cfg_alen;

    if (*ts).delay_us >= 0 {
        ptr::write_bytes(control.as_mut_ptr(), 0, control.len());
        msg.msg_control = control.as_mut_ptr() as *mut c_void;
        msg.msg_controllen = cmsg_space(mem::size_of::<uint64_t>());

        tdeliver = glob_tstart + ((*ts).delay_us * 1000) as uint64_t;
        tdeliver_max = if tdeliver_max > tdeliver {
            tdeliver_max
        } else {
            tdeliver
        };

        let cm = cmsg_firsthdr(&mut msg);
        (*cm).cmsg_level = libc::SOL_SOCKET;
        (*cm).cmsg_type = SCM_TXTIME;
        (*cm).cmsg_len = cmsg_len(mem::size_of_val(&tdeliver)) as _;
        ptr::copy_nonoverlapping(
            &tdeliver as *const _ as *const u8,
            cmsg_data(cm),
            mem::size_of_val(&tdeliver),
        );
    }

    ret = libc::sendmsg(fdt, &msg, 0) as c_int;
    if ret == -1 {
        error(1, *libc::__errno_location(), cstr!("write"));
    }
    if ret == 0 {
        error(1, 0, cstr!("write: 0B"));
    }
}

unsafe fn do_recv_one(fdr: c_int, ts: *mut timed_send) {
    let mut tstop: int64_t;
    let texpect: int64_t;
    let mut rbuf = [0 as c_char; 2];
    let ret: c_int;

    ret = libc::recv(fdr, rbuf.as_mut_ptr() as *mut c_void, rbuf.len(), 0) as c_int;
    if ret == -1 && *libc::__errno_location() == libc::EAGAIN {
        error(1, libc::EAGAIN, cstr!("recv: timeout"));
    }
    if ret == -1 {
        error(1, *libc::__errno_location(), cstr!("read"));
    }
    if ret != 1 {
        error(1, 0, cstr!("read: %dB"), ret);
    }

    tstop = ((gettime_ns(cfg_clockid) - glob_tstart) / 1000) as int64_t;
    texpect = if (*ts).delay_us >= 0 { (*ts).delay_us } else { 0 };

    libc::fprintf(
        libc::stderr,
        cstr!("payload:%c delay:%lld expected:%lld (us)\n"),
        rbuf[0] as c_int,
        tstop as libc::c_longlong,
        texpect as libc::c_longlong,
    );

    if rbuf[0] != (*ts).data {
        libc::fprintf(
            libc::stderr,
            cstr!("payload mismatch. expected %c\n"),
            (*ts).data as c_int,
        );
        errors += 1;
    }

    if libc::llabs(tstop - texpect) > cfg_variance_us as libc::c_longlong {
        libc::fprintf(
            libc::stderr,
            cstr!("exceeds variance (%d us)\n"),
            cfg_variance_us,
        );
        if !cfg_machine_slow {
            errors += 1;
        }
    }
}

unsafe fn do_recv_verify_empty(fdr: c_int) {
    let mut rbuf = [0 as c_char; 1];
    let ret: c_int;

    ret = libc::recv(fdr, rbuf.as_mut_ptr() as *mut c_void, rbuf.len(), 0) as c_int;
    if ret != -1 || *libc::__errno_location() != libc::EAGAIN {
        error(
            1,
            0,
            cstr!("recv: not empty as expected (%d, %d)"),
            ret,
            *libc::__errno_location(),
        );
    }
}

unsafe fn do_recv_errqueue_timeout(fdt: c_int) -> c_int {
    let mut control =
        [0u8; 64 + 128];
    let mut data =
        [0u8; 14 + 40 + 8 + 1];
    let mut ret: c_int;
    let mut num_tstamp: c_int = 0;
    let mut msg: libc::msghdr = mem::zeroed();
    let mut iov: libc::iovec = mem::zeroed();
    let mut tstamp: int64_t = 0;

    iov.iov_base = data.as_mut_ptr() as *mut c_void;
    iov.iov_len = data.len();

    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1;

    msg.msg_control = control.as_mut_ptr() as *mut c_void;
    msg.msg_controllen = control.len();

    loop {
        let mut reason: *const c_char = ptr::null();

        ret = libc::recvmsg(fdt, &mut msg, libc::MSG_ERRQUEUE) as c_int;
        if ret == -1 && *libc::__errno_location() == libc::EAGAIN {
            break;
        }
        if ret == -1 {
            error(1, *libc::__errno_location(), cstr!("errqueue"));
        }
        if msg.msg_flags != libc::MSG_ERRQUEUE {
            error(1, 0, cstr!("errqueue: flags 0x%x\n"), msg.msg_flags);
        }

        let cm = cmsg_firsthdr(&mut msg);
        if (*cm).cmsg_level != cfg_errq_level || (*cm).cmsg_type != cfg_errq_type {
            error(
                1,
                0,
                cstr!("errqueue: type 0x%x.0x%x\n"),
                (*cm).cmsg_level,
                (*cm).cmsg_type,
            );
        }

        let err = cmsg_data(cm) as *mut sock_extended_err;
        if (*err).ee_origin != SO_EE_ORIGIN_TXTIME {
            error(1, 0, cstr!("errqueue: origin 0x%x\n"), (*err).ee_origin as c_uint);
        }

        match (*err).ee_errno as c_int {
            libc::ECANCELED => {
                if (*err).ee_code != SO_EE_CODE_TXTIME_MISSED {
                    error(
                        1,
                        0,
                        cstr!("errqueue: unknown ECANCELED %u\n"),
                        (*err).ee_code as c_uint,
                    );
                }
                reason = cstr!("missed txtime");
            }
            libc::EINVAL => {
                if (*err).ee_code != SO_EE_CODE_TXTIME_INVALID_PARAM {
                    error(
                        1,
                        0,
                        cstr!("errqueue: unknown EINVAL %u\n"),
                        (*err).ee_code as c_uint,
                    );
                }
                reason = cstr!("invalid txtime");
            }
            _ => {
                error(
                    1,
                    0,
                    cstr!("errqueue: errno %u code %u\n"),
                    (*err).ee_errno,
                    (*err).ee_code as c_uint,
                );
            }
        }

        tstamp = (((*err).ee_data as int64_t) << 32) | (*err).ee_info as int64_t;
        tstamp -= glob_tstart as int64_t;
        tstamp /= 1000 * 1000;
        libc::fprintf(
            libc::stderr,
            cstr!("send: pkt %c at %lldms dropped: %s\n"),
            data[(ret - 1) as usize] as c_int,
            tstamp as libc::c_longlong,
            reason,
        );

        msg.msg_flags = 0;
        msg.msg_controllen = control.len();
        num_tstamp += 1;
    }

    num_tstamp
}

unsafe fn recv_errqueue_msgs(fdt: c_int) {
    let mut pfd = libc::pollfd {
        fd: fdt,
        events: libc::POLLERR,
        revents: 0,
    };
    let timeout_ms: c_int = 10;
    let mut ret: c_int;
    let mut num_tstamp: c_int = 0;

    loop {
        ret = libc::poll(&mut pfd, 1, timeout_ms);
        if ret == -1 {
            error(1, *libc::__errno_location(), cstr!("poll"));
        }

        if ret != 0 && (pfd.revents & libc::POLLERR) != 0 {
            num_tstamp += do_recv_errqueue_timeout(fdt);
        }

        if num_tstamp == cfg_num_pkt {
            break;
        }

        if gettime_ns(cfg_clockid) >= tdeliver_max {
            break;
        }
    }
}

unsafe fn start_time_wait() {
    let now: uint64_t;
    let err: c_int;

    if cfg_start_time_ns == 0 {
        return;
    }

    now = gettime_ns(libc::CLOCK_REALTIME);
    if cfg_start_time_ns < now {
        libc::fprintf(libc::stderr, cstr!("FAIL: start time already passed\n"));
        if !cfg_machine_slow {
            errors += 1;
        }
        return;
    }

    err = libc::usleep(((cfg_start_time_ns - now) / 1000) as libc::useconds_t);
    if err != 0 {
        error(1, *libc::__errno_location(), cstr!("usleep"));
    }
}

unsafe fn setsockopt_txtime(fd: c_int) {
    let mut so_txtime_val = sock_txtime {
        clockid: cfg_clockid,
        flags: 0,
    };
    let mut so_txtime_val_read: sock_txtime = mem::zeroed();
    let mut vallen: socklen_t = mem::size_of_val(&so_txtime_val) as socklen_t;

    so_txtime_val.flags = SOF_TXTIME_REPORT_ERRORS;

    if libc::setsockopt(
        fd,
        libc::SOL_SOCKET,
        SO_TXTIME,
        &so_txtime_val as *const _ as *const c_void,
        mem::size_of_val(&so_txtime_val) as socklen_t,
    ) != 0
    {
        error(1, *libc::__errno_location(), cstr!("setsockopt txtime"));
    }

    if libc::getsockopt(
        fd,
        libc::SOL_SOCKET,
        SO_TXTIME,
        &mut so_txtime_val_read as *mut _ as *mut c_void,
        &mut vallen,
    ) != 0
    {
        error(1, *libc::__errno_location(), cstr!("getsockopt txtime"));
    }

    if vallen != mem::size_of_val(&so_txtime_val) as socklen_t
        || libc::memcmp(
            &so_txtime_val as *const _ as *const c_void,
            &so_txtime_val_read as *const _ as *const c_void,
            vallen as usize,
        ) != 0
    {
        error(1, 0, cstr!("getsockopt txtime: mismatch"));
    }
}

unsafe fn setup_tx(addr: *mut libc::sockaddr, alen: socklen_t) -> c_int {
    let fd: c_int;

    fd = libc::socket((*addr).sa_family as c_int, libc::SOCK_DGRAM, 0);
    if fd == -1 {
        error(1, *libc::__errno_location(), cstr!("socket t"));
    }

    if libc::connect(fd, addr, alen) != 0 {
        error(1, *libc::__errno_location(), cstr!("connect"));
    }

    setsockopt_txtime(fd);

    if cfg_mark != 0
        && libc::setsockopt(
            fd,
            libc::SOL_SOCKET,
            libc::SO_MARK,
            &cfg_mark as *const _ as *const c_void,
            mem::size_of_val(&cfg_mark) as socklen_t,
        ) != 0
    {
        error(1, *libc::__errno_location(), cstr!("setsockopt mark"));
    }

    fd
}

unsafe fn setup_rx(addr: *mut libc::sockaddr, alen: socklen_t) -> c_int {
    let mut tv = libc::timeval {
        tv_sec: 0,
        tv_usec: 100 * 1000,
    };
    let fd: c_int;

    fd = libc::socket((*addr).sa_family as c_int, libc::SOCK_DGRAM, 0);
    if fd == -1 {
        error(1, *libc::__errno_location(), cstr!("socket r"));
    }

    if libc::bind(fd, addr, alen) != 0 {
        error(1, *libc::__errno_location(), cstr!("bind"));
    }

    if cfg_machine_slow {
        tv.tv_sec = 2;
    }

    if libc::setsockopt(
        fd,
        libc::SOL_SOCKET,
        libc::SO_RCVTIMEO,
        &tv as *const _ as *const c_void,
        mem::size_of_val(&tv) as socklen_t,
    ) != 0
    {
        error(1, *libc::__errno_location(), cstr!("setsockopt rcv timeout"));
    }

    fd
}

unsafe fn do_test_tx(addr: *mut libc::sockaddr, alen: socklen_t) {
    let fdt: c_int;
    let mut i: c_int;

    libc::fprintf(
        libc::stderr,
        cstr!("\nSO_TXTIME ipv%c clock %s\n"),
        if (*addr).sa_family as c_int == libc::PF_INET {
            '4' as c_int
        } else {
            '6' as c_int
        },
        if cfg_clockid == libc::CLOCK_TAI {
            cstr!("tai")
        } else {
            cstr!("monotonic")
        },
    );

    fdt = setup_tx(addr, alen);

    start_time_wait();
    glob_tstart = gettime_ns(cfg_clockid);

    i = 0;
    while i < cfg_num_pkt {
        do_send_one(fdt, cfg_buf.as_mut_ptr().add(i as usize));
        i += 1;
    }

    recv_errqueue_msgs(fdt);

    if libc::close(fdt) != 0 {
        error(1, *libc::__errno_location(), cstr!("close t"));
    }
}

unsafe fn do_test_rx(addr: *mut libc::sockaddr, alen: socklen_t) {
    let fdr: c_int;
    let mut i: c_int;

    fdr = setup_rx(addr, alen);

    start_time_wait();
    glob_tstart = gettime_ns(cfg_clockid);

    i = 0;
    while i < cfg_num_pkt {
        do_recv_one(fdr, cfg_buf.as_mut_ptr().add(i as usize));
        i += 1;
    }

    do_recv_verify_empty(fdr);

    if libc::close(fdr) != 0 {
        error(1, *libc::__errno_location(), cstr!("close r"));
    }
}

unsafe fn setup_sockaddr(
    domain: c_int,
    str_addr: *const c_char,
    sockaddr: *mut libc::sockaddr_storage,
) {
    let addr6 = sockaddr as *mut libc::sockaddr_in6;
    let addr4 = sockaddr as *mut libc::sockaddr_in;

    match domain {
        libc::PF_INET => {
            ptr::write_bytes(addr4, 0, 1);
            (*addr4).sin_family = libc::AF_INET as libc::sa_family_t;
            (*addr4).sin_port = libc::htons(cfg_port);
            if !str_addr.is_null()
                && libc::inet_pton(
                    libc::AF_INET,
                    str_addr,
                    &mut (*addr4).sin_addr as *mut _ as *mut c_void,
                ) != 1
            {
                error(1, 0, cstr!("ipv4 parse error: %s"), str_addr);
            }
        }
        libc::PF_INET6 => {
            ptr::write_bytes(addr6, 0, 1);
            (*addr6).sin6_family = libc::AF_INET6 as libc::sa_family_t;
            (*addr6).sin6_port = libc::htons(cfg_port);
            if !str_addr.is_null()
                && libc::inet_pton(
                    libc::AF_INET6,
                    str_addr,
                    &mut (*addr6).sin6_addr as *mut _ as *mut c_void,
                ) != 1
            {
                error(1, 0, cstr!("ipv6 parse error: %s"), str_addr);
            }
        }
        _ => {}
    }
}

unsafe fn parse_io(optarg_: *const c_char, mut array: *mut timed_send) -> c_int {
    let mut arg: *mut c_char;
    let mut tok: *mut c_char;
    let mut aoff: c_int = 0;

    arg = libc::strdup(optarg_);
    if arg.is_null() {
        error(1, *libc::__errno_location(), cstr!("strdup"));
    }

    loop {
        tok = libc::strtok(arg, cstr!(","));
        if tok.is_null() {
            break;
        }
        arg = ptr::null_mut(); /* only pass non-zero on first call */

        if aoff / 2 == MAX_NUM_PKT as c_int {
            error(1, 0, cstr!("exceeds max pkt count (%d)"), MAX_NUM_PKT as c_int);
        }

        if (aoff & 1) != 0 {
            /* parse delay */
            (*array).delay_us = libc::strtol(tok, ptr::null_mut(), 0) * 1000;
            array = array.add(1);
        } else {
            /* parse character */
            (*array).data = *tok;
        }

        aoff += 1;
    }

    libc::free(arg as *mut c_void);

    aoff / 2
}

unsafe fn usage(progname: *const c_char) {
    libc::fprintf(
        libc::stderr,
        cstr!(
            "\nUsage: %s [options] <payload>\n\
Options:\n\
  -4            only IPv4\n\
  -6            only IPv6\n\
  -c <clock>    monotonic or tai (default)\n\
  -D <addr>     destination IP address (server)\n\
  -S <addr>     source IP address (client)\n\
  -r            run rx mode\n\
  -t <nsec>     start time (UTC nanoseconds)\n\
  -m <mark>     socket mark\n\
\n"
        ),
        progname,
    );
    libc::exit(1);
}

unsafe fn parse_opts(argc: c_int, argv: *mut *mut c_char) {
    let mut daddr: *mut c_char = ptr::null_mut();
    let mut saddr: *mut c_char = ptr::null_mut();
    let mut domain: c_int = libc::PF_UNSPEC;
    let mut c: c_int;

    loop {
        c = libc::getopt(argc, argv, cstr!("46c:S:D:rt:m:"));
        if c == -1 {
            break;
        }
        match c as u8 as char {
            '4' => {
                if domain != libc::PF_UNSPEC {
                    error(1, 0, cstr!("Pass one of -4 or -6"));
                }
                domain = libc::PF_INET;
                cfg_alen = mem::size_of::<libc::sockaddr_in>() as socklen_t;
                cfg_errq_level = libc::SOL_IP;
                cfg_errq_type = libc::IP_RECVERR;
            }
            '6' => {
                if domain != libc::PF_UNSPEC {
                    error(1, 0, cstr!("Pass one of -4 or -6"));
                }
                domain = libc::PF_INET6;
                cfg_alen = mem::size_of::<libc::sockaddr_in6>() as socklen_t;
                cfg_errq_level = libc::SOL_IPV6;
                cfg_errq_type = libc::IPV6_RECVERR;
            }
            'c' => {
                if libc::strcmp(optarg, cstr!("tai")) == 0 {
                    cfg_clockid = libc::CLOCK_TAI;
                } else if libc::strcmp(optarg, cstr!("monotonic")) == 0
                    || libc::strcmp(optarg, cstr!("mono")) == 0
                {
                    cfg_clockid = libc::CLOCK_MONOTONIC;
                } else {
                    error(1, 0, cstr!("unknown clock id %s"), optarg);
                }
            }
            'S' => {
                saddr = optarg;
            }
            'D' => {
                daddr = optarg;
            }
            'r' => {
                cfg_rx = true;
            }
            't' => {
                cfg_start_time_ns = libc::strtoll(optarg, ptr::null_mut(), 0) as uint64_t;
            }
            'm' => {
                cfg_mark = libc::strtol(optarg, ptr::null_mut(), 0) as c_int;
            }
            _ => {
                usage(*argv.add(0));
            }
        }
    }

    if argc - optind != 1 {
        usage(*argv.add(0));
    }

    if domain == libc::PF_UNSPEC {
        error(1, 0, cstr!("Pass one of -4 or -6"));
    }
    if daddr.is_null() {
        error(1, 0, cstr!("-D <server addr> required\n"));
    }
    if !cfg_rx && saddr.is_null() {
        error(1, 0, cstr!("-S <client addr> required\n"));
    }

    setup_sockaddr(domain, daddr, &mut cfg_dst_addr);
    setup_sockaddr(domain, saddr, &mut cfg_src_addr);

    cfg_num_pkt = parse_io(*argv.add(optind as usize), cfg_buf.as_mut_ptr());

    cfg_machine_slow = !libc::getenv(cstr!("KSFT_MACHINE_SLOW")).is_null();
}

fn main() {
    unsafe {
        let mut argc: c_int = 0;
        let mut argv_vec: Vec<*mut c_char> = std::env::args()
            .map(|arg| std::ffi::CString::new(arg).unwrap().into_raw())
            .collect();
        argv_vec.push(ptr::null_mut());
        argc = (argv_vec.len() - 1) as c_int;

        parse_opts(argc, argv_vec.as_mut_ptr());

        if cfg_rx {
            do_test_rx(&mut cfg_dst_addr as *mut _ as *mut libc::sockaddr, cfg_alen);
        } else {
            do_test_tx(&mut cfg_src_addr as *mut _ as *mut libc::sockaddr, cfg_alen);
        }

        for arg in argv_vec.into_iter().take(argc as usize) {
            let _ = std::ffi::CString::from_raw(arg);
        }

        if errors != 0 {
            libc::fprintf(libc::stderr, cstr!("FAIL: %d errors\n"), errors);
            std::process::exit(KSFT_FAIL);
        }

        std::process::exit(KSFT_PASS);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
