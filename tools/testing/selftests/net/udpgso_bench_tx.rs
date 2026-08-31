// SPDX-License-Identifier: GPL-2.0

// C dependency intent: _GNU_SOURCE plus Linux/POSIX networking headers and kselftest.h.

use libc::*;
use std::ffi::CString;
use std::mem;
use std::ptr;

const ETH_MAX_MTU: usize = 0xFFFF;
const UDP_SEGMENT: c_int = 103;
const SO_ZEROCOPY: c_int = 60;
const SO_EE_ORIGIN_ZEROCOPY: u8 = 5;
const MSG_ZEROCOPY: c_int = 0x4000000;
const ENOTSUPP: c_int = 524;
const NUM_PKT: usize = 100;
const KSFT_SKIP: c_int = 4;

const SOF_TIMESTAMPING_TX_HARDWARE: u32 = 1 << 0;
const SOF_TIMESTAMPING_TX_SOFTWARE: u32 = 1 << 1;
const SOF_TIMESTAMPING_SOFTWARE: c_int = 1 << 4;
const SOF_TIMESTAMPING_RAW_HARDWARE: c_int = 1 << 6;
const SOF_TIMESTAMPING_OPT_ID: c_int = 1 << 7;
const SOF_TIMESTAMPING_OPT_CMSG: c_int = 1 << 10;
const SOF_TIMESTAMPING_OPT_TSONLY: c_int = 1 << 11;

const SO_EE_ORIGIN_LOCAL: u8 = 1;
const SO_EE_ORIGIN_ICMP: u8 = 2;
const SO_EE_ORIGIN_ICMP6: u8 = 3;
const SO_EE_ORIGIN_TIMESTAMPING: u8 = 4;

const ETH_DATA_LEN: c_int = 1500;

#[repr(C)]
struct sock_extended_err {
    ee_errno: u32,
    ee_origin: u8,
    ee_type: u8,
    ee_code: u8,
    ee_pad: u8,
    ee_info: u32,
    ee_data: u32,
}

#[repr(C)]
struct scm_timestamping {
    ts: [timespec; 3],
}

unsafe extern "C" {
    fn error(status: c_int, errnum: c_int, format: *const c_char, ...);
}

static mut CFG_CACHE_TRASH: bool = false;
static mut CFG_CPU: c_int = -1;
static mut CFG_CONNECTED: c_int = 1;
static mut CFG_FAMILY: c_int = PF_UNSPEC;
static mut CFG_MSS: u16 = 0;
static mut CFG_PAYLOAD_LEN: c_int = 1472 * 42;
static mut CFG_PORT: c_int = 8000;
static mut CFG_RUNTIME_MS: c_int = -1;
static mut CFG_POLL: bool = false;
static mut CFG_POLL_LOOP_TIMEOUT_MS: c_int = 2000;
static mut CFG_SEGMENT: bool = false;
static mut CFG_SENDMMSG: bool = false;
static mut CFG_TCP: bool = false;
static mut CFG_TX_TS: u32 = SOF_TIMESTAMPING_TX_SOFTWARE;
static mut CFG_TX_TSTAMP: bool = false;
static mut CFG_AUDIT: bool = false;
static mut CFG_VERBOSE: bool = false;
static mut CFG_ZEROCOPY: bool = false;
static mut CFG_MSG_NR: c_int = 0;
static mut CFG_GSO_SIZE: u16 = 0;
static mut TOTAL_NUM_MSGS: c_ulong = 0;
static mut TOTAL_NUM_SENDS: c_ulong = 0;
static mut STAT_TX_TS: c_ulong = 0;
static mut STAT_TX_TS_ERRORS: c_ulong = 0;
static mut TSTART: c_ulong = 0;
static mut TEND: c_ulong = 0;
static mut STAT_ZCOPIES: c_ulong = 0;

static mut CFG_ALEN: socklen_t = 0;
static mut CFG_DST_ADDR: sockaddr_storage = unsafe { mem::zeroed() };

static mut INTERRUPTED: bool = false;
static mut BUF: [[c_char; ETH_MAX_MTU]; NUM_PKT] = [[0; ETH_MAX_MTU]; NUM_PKT];

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

unsafe extern "C" fn sigint_handler(signum: c_int) {
    if signum == SIGINT {
        INTERRUPTED = true;
    }
}

unsafe fn gettimeofday_ms() -> c_ulong {
    let mut tv: timeval = mem::zeroed();

    gettimeofday(&mut tv, ptr::null_mut());
    (tv.tv_sec as c_ulong * 1000) + (tv.tv_usec as c_ulong / 1000)
}

unsafe fn set_cpu(cpu: c_int) -> c_int {
    let mut mask: cpu_set_t = mem::zeroed();

    CPU_ZERO(&mut mask);
    CPU_SET(cpu as usize, &mut mask);
    if sched_setaffinity(0, mem::size_of_val(&mask), &mask) != 0 {
        error(1, 0, cstr!("setaffinity %d"), cpu);
    }

    0
}

unsafe fn setup_sockaddr(domain: c_int, str_addr: *const c_char, sockaddr: *mut c_void) {
    let addr6 = sockaddr as *mut sockaddr_in6;
    let addr4 = sockaddr as *mut sockaddr_in;

    match domain {
        PF_INET => {
            (*addr4).sin_family = AF_INET as sa_family_t;
            (*addr4).sin_port = htons(CFG_PORT as u16);
            if inet_pton(AF_INET, str_addr, &mut (*addr4).sin_addr as *mut _ as *mut c_void) != 1 {
                error(1, 0, cstr!("ipv4 parse error: %s"), str_addr);
            }
        }
        PF_INET6 => {
            (*addr6).sin6_family = AF_INET6 as sa_family_t;
            (*addr6).sin6_port = htons(CFG_PORT as u16);
            if inet_pton(AF_INET6, str_addr, &mut (*addr6).sin6_addr as *mut _ as *mut c_void) != 1 {
                error(1, 0, cstr!("ipv6 parse error: %s"), str_addr);
            }
        }
        _ => {
            error(1, 0, cstr!("illegal domain"));
        }
    }
}

unsafe fn flush_cmsg(cmsg: *mut cmsghdr) {
    let mut err: *mut sock_extended_err;
    let mut tss: *mut scm_timestamping;
    let lo: u32;
    let hi: u32;
    let i: c_int;

    match (*cmsg).cmsg_level {
        SOL_SOCKET => {
            if (*cmsg).cmsg_type == SO_TIMESTAMPING {
                i = if CFG_TX_TS == SOF_TIMESTAMPING_TX_HARDWARE { 2 } else { 0 };
                tss = CMSG_DATA(cmsg) as *mut scm_timestamping;
                if (*tss).ts[i as usize].tv_sec == 0 {
                    STAT_TX_TS_ERRORS += 1;
                }
            } else {
                error(1, 0, cstr!("unknown SOL_SOCKET cmsg type=%u\n"), (*cmsg).cmsg_type);
            }
        }
        SOL_IP | SOL_IPV6 => {
            match (*cmsg).cmsg_type {
                IP_RECVERR | IPV6_RECVERR => {
                    err = CMSG_DATA(cmsg) as *mut sock_extended_err;
                    match (*err).ee_origin {
                        SO_EE_ORIGIN_TIMESTAMPING => {
                            /* Got a TX timestamp from error queue */
                            STAT_TX_TS += 1;
                        }
                        SO_EE_ORIGIN_ICMP | SO_EE_ORIGIN_ICMP6 => {
                            if CFG_VERBOSE {
                                fprintf(
                                    stderr,
                                    cstr!("received ICMP error: type=%u, code=%u\n"),
                                    (*err).ee_type as c_uint,
                                    (*err).ee_code as c_uint,
                                );
                            }
                        }
                        SO_EE_ORIGIN_ZEROCOPY => {
                            lo = (*err).ee_info;
                            hi = (*err).ee_data;
                            /* range of IDs acknowledged */
                            STAT_ZCOPIES += (hi - lo + 1) as c_ulong;
                        }
                        SO_EE_ORIGIN_LOCAL => {
                            if CFG_VERBOSE {
                                fprintf(
                                    stderr,
                                    cstr!("received packet with local origin: %u\n"),
                                    (*err).ee_origin as c_uint,
                                );
                            }
                        }
                        _ => {
                            error(0, 1, cstr!("received packet with origin: %u"), (*err).ee_origin as c_uint);
                        }
                    }
                }
                _ => {
                    error(0, 1, cstr!("unknown IP msg type=%u\n"), (*cmsg).cmsg_type);
                }
            }
        }
        _ => {
            error(0, 1, cstr!("unknown cmsg level=%u\n"), (*cmsg).cmsg_level);
        }
    }
}

unsafe fn flush_errqueue_recv(fd: c_int) {
    let mut control = [0 as c_char; 128];
    let mut msg: msghdr = mem::zeroed();
    let mut cmsg: *mut cmsghdr;
    let mut ret: c_int;

    loop {
        msg.msg_control = control.as_mut_ptr() as *mut c_void;
        msg.msg_controllen = control.len();
        ret = recvmsg(fd, &mut msg, MSG_ERRQUEUE);
        if ret == -1 && *__errno_location() == EAGAIN {
            break;
        }
        if ret == -1 {
            error(1, *__errno_location(), cstr!("errqueue"));
        }
        if msg.msg_flags != MSG_ERRQUEUE {
            error(1, 0, cstr!("errqueue: flags 0x%x\n"), msg.msg_flags);
        }
        if CFG_AUDIT {
            cmsg = CMSG_FIRSTHDR(&msg);
            while !cmsg.is_null() {
                flush_cmsg(cmsg);
                cmsg = CMSG_NXTHDR(&msg, cmsg);
            }
        }
        msg.msg_flags = 0;
    }
}

unsafe fn flush_errqueue(fd: c_int, do_poll: bool, poll_timeout: c_ulong, poll_err: bool) {
    if do_poll {
        let mut fds: pollfd = mem::zeroed();
        let ret: c_int;

        fds.fd = fd;
        ret = poll(&mut fds, 1, poll_timeout as c_int);
        if ret == 0 {
            if CFG_VERBOSE && poll_err {
                fprintf(stderr, cstr!("poll timeout\n"));
            }
        } else if ret < 0 {
            error(1, *__errno_location(), cstr!("poll"));
        }
    }

    flush_errqueue_recv(fd);
}

unsafe fn flush_errqueue_retry(fd: c_int, num_sends: c_ulong) {
    let mut tnow: c_ulong;
    let tstop: c_ulong;
    let mut first_try = true;

    tnow = gettimeofday_ms();
    tstop = tnow + CFG_POLL_LOOP_TIMEOUT_MS as c_ulong;
    loop {
        flush_errqueue(fd, true, tstop - tnow, first_try);
        first_try = false;
        tnow = gettimeofday_ms();
        if !((STAT_ZCOPIES != num_sends) && (tnow < tstop)) {
            break;
        }
    }
}

unsafe fn send_tcp(fd: c_int, data: *mut c_char) -> c_int {
    let mut ret: c_int;
    let mut done: c_int = 0;
    let mut count: c_int = 0;

    while done < CFG_PAYLOAD_LEN {
        ret = send(
            fd,
            data.add(done as usize) as *const c_void,
            (CFG_PAYLOAD_LEN - done) as usize,
            if CFG_ZEROCOPY { MSG_ZEROCOPY } else { 0 },
        ) as c_int;
        if ret == -1 {
            error(1, *__errno_location(), cstr!("write"));
        }

        done += ret;
        count += 1;
    }

    count
}

unsafe fn send_udp(fd: c_int, data: *mut c_char) -> c_int {
    let mut ret: c_int;
    let mut total_len: c_int;
    let mut len: c_int;
    let mut count: c_int = 0;

    total_len = CFG_PAYLOAD_LEN;

    while total_len != 0 {
        len = if total_len < CFG_MSS as c_int { total_len } else { CFG_MSS as c_int };

        ret = sendto(
            fd,
            data as *const c_void,
            len as usize,
            if CFG_ZEROCOPY { MSG_ZEROCOPY } else { 0 },
            if CFG_CONNECTED != 0 { ptr::null() } else { &CFG_DST_ADDR as *const _ as *const sockaddr },
            if CFG_CONNECTED != 0 { 0 } else { CFG_ALEN },
        ) as c_int;
        if ret == -1 {
            error(1, *__errno_location(), cstr!("write"));
        }
        if ret != len {
            error(1, *__errno_location(), cstr!("write: %uB != %uB\n"), ret, len);
        }

        total_len -= len;
        count += 1;
    }

    count
}

unsafe fn send_ts_cmsg(cm: *mut cmsghdr) {
    let valp: *mut u32;

    (*cm).cmsg_level = SOL_SOCKET;
    (*cm).cmsg_type = SO_TIMESTAMPING;
    (*cm).cmsg_len = CMSG_LEN(mem::size_of_val(&CFG_TX_TS) as c_uint) as usize;
    valp = CMSG_DATA(cm) as *mut u32;
    *valp = CFG_TX_TS;
}

unsafe fn send_udp_sendmmsg(fd: c_int, data: *mut c_char) -> c_int {
    let mut control = [0 as c_char; 64];
    let max_nr_msg: c_int = ETH_MAX_MTU as c_int / ETH_DATA_LEN;
    let mut mmsgs: [mmsghdr; ETH_MAX_MTU / ETH_DATA_LEN as usize] = mem::zeroed();
    let mut iov: [iovec; ETH_MAX_MTU / ETH_DATA_LEN as usize] = mem::zeroed();
    let mut off: c_uint = 0;
    let mut left: c_uint;
    let mut msg_controllen: usize = 0;
    let mut i: c_int = 0;
    let ret: c_int;

    if CFG_TX_TSTAMP {
        let mut msg: msghdr = mem::zeroed();
        let cmsg: *mut cmsghdr;

        msg.msg_control = control.as_mut_ptr() as *mut c_void;
        msg.msg_controllen = control.len();
        cmsg = CMSG_FIRSTHDR(&msg);
        send_ts_cmsg(cmsg);
        msg_controllen += CMSG_SPACE(mem::size_of_val(&CFG_TX_TS) as c_uint) as usize;
    }

    left = CFG_PAYLOAD_LEN as c_uint;
    while left != 0 {
        if i == max_nr_msg {
            error(1, 0, cstr!("sendmmsg: exceeds max_nr_msg"));
        }

        iov[i as usize].iov_base = data.add(off as usize) as *mut c_void;
        iov[i as usize].iov_len = if (CFG_MSS as c_uint) < left { CFG_MSS as usize } else { left as usize };

        mmsgs[i as usize].msg_hdr.msg_iov = iov.as_mut_ptr().add(i as usize);
        mmsgs[i as usize].msg_hdr.msg_iovlen = 1;

        mmsgs[i as usize].msg_hdr.msg_name = &CFG_DST_ADDR as *const _ as *mut c_void;
        mmsgs[i as usize].msg_hdr.msg_namelen = CFG_ALEN;
        if msg_controllen != 0 {
            mmsgs[i as usize].msg_hdr.msg_control = control.as_mut_ptr() as *mut c_void;
            mmsgs[i as usize].msg_hdr.msg_controllen = msg_controllen;
        }

        off += iov[i as usize].iov_len as c_uint;
        left -= iov[i as usize].iov_len as c_uint;
        i += 1;
    }

    ret = sendmmsg(fd, mmsgs.as_mut_ptr(), i as c_uint, if CFG_ZEROCOPY { MSG_ZEROCOPY } else { 0 });
    if ret == -1 {
        error(1, *__errno_location(), cstr!("sendmmsg"));
    }

    ret
}

unsafe fn send_udp_segment_cmsg(cm: *mut cmsghdr) {
    let valp: *mut u16;

    (*cm).cmsg_level = SOL_UDP;
    (*cm).cmsg_type = UDP_SEGMENT;
    (*cm).cmsg_len = CMSG_LEN(mem::size_of_val(&CFG_GSO_SIZE) as c_uint) as usize;
    valp = CMSG_DATA(cm) as *mut u16;
    *valp = CFG_GSO_SIZE;
}

unsafe fn send_udp_segment(fd: c_int, data: *mut c_char) -> c_int {
    let mut control = [0 as c_char; 128];
    let mut msg: msghdr = mem::zeroed();
    let mut iov: iovec = mem::zeroed();
    let mut msg_controllen: usize;
    let mut cmsg: *mut cmsghdr;
    let ret: c_int;

    iov.iov_base = data as *mut c_void;
    iov.iov_len = CFG_PAYLOAD_LEN as usize;

    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1;

    msg.msg_control = control.as_mut_ptr() as *mut c_void;
    msg.msg_controllen = control.len();
    cmsg = CMSG_FIRSTHDR(&msg);
    send_udp_segment_cmsg(cmsg);
    msg_controllen = CMSG_SPACE(mem::size_of_val(&CFG_MSS) as c_uint) as usize;
    if CFG_TX_TSTAMP {
        cmsg = CMSG_NXTHDR(&msg, cmsg);
        send_ts_cmsg(cmsg);
        msg_controllen += CMSG_SPACE(mem::size_of_val(&CFG_TX_TS) as c_uint) as usize;
    }

    msg.msg_controllen = msg_controllen;
    msg.msg_name = &CFG_DST_ADDR as *const _ as *mut c_void;
    msg.msg_namelen = CFG_ALEN;

    ret = sendmsg(fd, &msg, if CFG_ZEROCOPY { MSG_ZEROCOPY } else { 0 }) as c_int;
    if ret == -1 {
        error(1, *__errno_location(), cstr!("sendmsg"));
    }
    if ret as usize != iov.iov_len {
        error(1, 0, cstr!("sendmsg: %u != %llu\n"), ret, iov.iov_len as c_ulonglong);
    }

    1
}

unsafe fn usage(filepath: *const c_char) {
    error(
        1,
        0,
        cstr!("Usage: %s [-46acmHPtTuvz] [-C cpu] [-D dst ip] [-l secs] [-L secs] [-M messagenr] [-p port] [-s sendsize] [-S gsosize]"),
        filepath,
    );
}

unsafe fn parse_opts(argc: c_int, argv: *mut *mut c_char) {
    let mut bind_addr: *const c_char = ptr::null();
    let max_len: c_int;
    let hdrlen: c_int;
    let mut c: c_int;

    loop {
        c = getopt(argc, argv, cstr!("46acC:D:Hl:L:mM:p:s:PS:tTuvz"));
        if c == -1 {
            break;
        }
        match c as u8 as char {
            '4' => {
                if CFG_FAMILY != PF_UNSPEC {
                    error(1, 0, cstr!("Pass one of -4 or -6"));
                }
                CFG_FAMILY = PF_INET;
                CFG_ALEN = mem::size_of::<sockaddr_in>() as socklen_t;
            }
            '6' => {
                if CFG_FAMILY != PF_UNSPEC {
                    error(1, 0, cstr!("Pass one of -4 or -6"));
                }
                CFG_FAMILY = PF_INET6;
                CFG_ALEN = mem::size_of::<sockaddr_in6>() as socklen_t;
            }
            'a' => CFG_AUDIT = true,
            'c' => CFG_CACHE_TRASH = true,
            'C' => CFG_CPU = strtol(optarg, ptr::null_mut(), 0) as c_int,
            'D' => bind_addr = optarg,
            'l' => CFG_RUNTIME_MS = (strtoul(optarg, ptr::null_mut(), 10) * 1000) as c_int,
            'L' => CFG_POLL_LOOP_TIMEOUT_MS = (strtoul(optarg, ptr::null_mut(), 10) * 1000) as c_int,
            'm' => CFG_SENDMMSG = true,
            'M' => CFG_MSG_NR = strtoul(optarg, ptr::null_mut(), 10) as c_int,
            'p' => CFG_PORT = strtoul(optarg, ptr::null_mut(), 0) as c_int,
            'P' => CFG_POLL = true,
            's' => CFG_PAYLOAD_LEN = strtoul(optarg, ptr::null_mut(), 0) as c_int,
            'S' => {
                CFG_GSO_SIZE = strtoul(optarg, ptr::null_mut(), 0) as u16;
                CFG_SEGMENT = true;
            }
            'H' => {
                CFG_TX_TS = SOF_TIMESTAMPING_TX_HARDWARE;
                CFG_TX_TSTAMP = true;
            }
            't' => CFG_TCP = true,
            'T' => CFG_TX_TSTAMP = true,
            'u' => CFG_CONNECTED = 0,
            'v' => CFG_VERBOSE = true,
            'z' => CFG_ZEROCOPY = true,
            _ => exit(1),
        }
    }

    let default_v6 = cstr!("::");
    let default_v4 = cstr!("0.0.0.0");
    if bind_addr.is_null() {
        bind_addr = if CFG_FAMILY == PF_INET6 { default_v6 } else { default_v4 };
    }

    setup_sockaddr(CFG_FAMILY, bind_addr, &mut CFG_DST_ADDR as *mut _ as *mut c_void);

    if optind != argc {
        usage(*argv.add(0));
    }

    if CFG_FAMILY == PF_UNSPEC {
        error(1, 0, cstr!("must pass one of -4 or -6"));
    }
    if CFG_TCP && CFG_CONNECTED == 0 {
        error(1, 0, cstr!("connectionless tcp makes no sense"));
    }
    if CFG_SEGMENT && CFG_SENDMMSG {
        error(1, 0, cstr!("cannot combine segment offload and sendmmsg"));
    }
    if CFG_TX_TSTAMP && !(CFG_SEGMENT || CFG_SENDMMSG) {
        error(1, 0, cstr!("Options -T and -H require either -S or -m option"));
    }

    if CFG_FAMILY == PF_INET {
        hdrlen = mem::size_of::<iphdr>() as c_int + mem::size_of::<udphdr>() as c_int;
    } else {
        hdrlen = mem::size_of::<ip6_hdr>() as c_int + mem::size_of::<udphdr>() as c_int;
    }

    CFG_MSS = (ETH_DATA_LEN - hdrlen) as u16;
    max_len = ETH_MAX_MTU as c_int - hdrlen;
    if CFG_GSO_SIZE == 0 {
        CFG_GSO_SIZE = CFG_MSS;
    }

    if CFG_PAYLOAD_LEN > max_len {
        error(1, 0, cstr!("payload length %u exceeds max %u"), CFG_PAYLOAD_LEN, max_len);
    }
}

unsafe fn set_pmtu_discover(fd: c_int, is_ipv4: bool) {
    let level: c_int;
    let name: c_int;
    let val: c_int;

    if is_ipv4 {
        level = SOL_IP;
        name = IP_MTU_DISCOVER;
        val = IP_PMTUDISC_DO;
    } else {
        level = SOL_IPV6;
        name = IPV6_MTU_DISCOVER;
        val = IPV6_PMTUDISC_DO;
    }

    if setsockopt(fd, level, name, &val as *const _ as *const c_void, mem::size_of_val(&val) as socklen_t) != 0 {
        error(1, *__errno_location(), cstr!("setsockopt path mtu"));
    }
}

unsafe fn set_tx_timestamping(fd: c_int) {
    let mut val: c_int = SOF_TIMESTAMPING_OPT_CMSG | SOF_TIMESTAMPING_OPT_ID | SOF_TIMESTAMPING_OPT_TSONLY;

    if CFG_TX_TS == SOF_TIMESTAMPING_TX_SOFTWARE {
        val |= SOF_TIMESTAMPING_SOFTWARE;
    } else {
        val |= SOF_TIMESTAMPING_RAW_HARDWARE;
    }

    if setsockopt(fd, SOL_SOCKET, SO_TIMESTAMPING, &val as *const _ as *const c_void, mem::size_of_val(&val) as socklen_t) != 0 {
        error(1, *__errno_location(), cstr!("setsockopt tx timestamping"));
    }
}

unsafe fn print_audit_report(num_msgs: c_ulong, num_sends: c_ulong) {
    let tdelta: c_ulong;

    tdelta = TEND - TSTART;
    if tdelta == 0 {
        return;
    }

    fprintf(stderr, cstr!("Summary over %lu.%03lu seconds...\n"), tdelta / 1000, tdelta % 1000);
    fprintf(
        stderr,
        cstr!("sum %s tx: %6lu MB/s %10lu calls (%lu/s) %10lu msgs (%lu/s)\n"),
        if CFG_TCP { cstr!("tcp") } else { cstr!("udp") },
        ((num_msgs * CFG_PAYLOAD_LEN as c_ulong) >> 10) / tdelta,
        num_sends,
        num_sends * 1000 / tdelta,
        num_msgs,
        num_msgs * 1000 / tdelta,
    );

    if CFG_TX_TSTAMP {
        if STAT_TX_TS_ERRORS != 0 {
            error(
                1,
                0,
                cstr!("Expected clean TX Timestamps: %9lu msgs received %6lu errors"),
                STAT_TX_TS,
                STAT_TX_TS_ERRORS,
            );
        }
        if STAT_TX_TS != num_sends {
            error(
                1,
                0,
                cstr!("Unexpected number of TX Timestamps: %9lu expected %9lu received"),
                num_sends,
                STAT_TX_TS,
            );
        }
        fprintf(stderr, cstr!("Tx Timestamps: %19lu received %17lu errors\n"), STAT_TX_TS, STAT_TX_TS_ERRORS);
    }

    if CFG_ZEROCOPY {
        if STAT_ZCOPIES != num_sends {
            error(
                1,
                0,
                cstr!("Unexpected number of Zerocopy completions: %9lu expected %9lu received"),
                num_sends,
                STAT_ZCOPIES,
            );
        }
        fprintf(stderr, cstr!("Zerocopy acks: %19lu\n"), STAT_ZCOPIES);
    }
}

unsafe fn print_report(num_msgs: c_ulong, num_sends: c_ulong) {
    fprintf(
        stderr,
        cstr!("%s tx: %6lu MB/s %8lu calls/s %6lu msg/s\n"),
        if CFG_TCP { cstr!("tcp") } else { cstr!("udp") },
        (num_msgs * CFG_PAYLOAD_LEN as c_ulong) >> 20,
        num_sends,
        num_msgs,
    );

    if CFG_AUDIT {
        TOTAL_NUM_MSGS += num_msgs;
        TOTAL_NUM_SENDS += num_sends;
    }
}

unsafe fn main_0(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut num_msgs: c_ulong;
    let mut num_sends: c_ulong;
    let mut tnow: c_ulong;
    let mut treport: c_ulong;
    let tstop: c_ulong;
    let fd: c_int;
    let mut i: c_int;
    let mut val: c_int;
    let ret: c_int;

    parse_opts(argc, argv);

    if CFG_CPU > 0 {
        set_cpu(CFG_CPU);
    }

    i = 0;
    while (i as usize) < BUF[0].len() {
        BUF[0][i as usize] = (b'a' + (i % 26) as u8) as c_char;
        i += 1;
    }
    i = 1;
    while i < NUM_PKT as c_int {
        ptr::copy_nonoverlapping(BUF[0].as_ptr(), BUF[i as usize].as_mut_ptr(), BUF[0].len());
        i += 1;
    }

    signal(SIGINT, sigint_handler as sighandler_t);

    fd = socket(CFG_FAMILY, if CFG_TCP { SOCK_STREAM } else { SOCK_DGRAM }, 0);
    if fd == -1 {
        error(1, *__errno_location(), cstr!("socket"));
    }

    if CFG_ZEROCOPY {
        val = 1;

        ret = setsockopt(fd, SOL_SOCKET, SO_ZEROCOPY, &val as *const _ as *const c_void, mem::size_of_val(&val) as socklen_t);
        if ret != 0 {
            if *__errno_location() == ENOPROTOOPT || *__errno_location() == ENOTSUPP {
                fprintf(stderr, cstr!("SO_ZEROCOPY not supported"));
                exit(KSFT_SKIP);
            }
            error(1, *__errno_location(), cstr!("setsockopt zerocopy"));
        }
    }

    if CFG_CONNECTED != 0 && connect(fd, &CFG_DST_ADDR as *const _ as *const sockaddr, CFG_ALEN) != 0 {
        error(1, *__errno_location(), cstr!("connect"));
    }

    if CFG_SEGMENT {
        set_pmtu_discover(fd, CFG_FAMILY == PF_INET);
    }

    if CFG_TX_TSTAMP {
        set_tx_timestamping(fd);
    }

    num_msgs = 0;
    num_sends = 0;
    tnow = gettimeofday_ms();
    TSTART = tnow;
    TEND = tnow;
    tstop = tnow + CFG_RUNTIME_MS as c_ulong;
    treport = tnow + 1000;

    i = 0;
    loop {
        if CFG_TCP {
            num_sends += send_tcp(fd, BUF[i as usize].as_mut_ptr()) as c_ulong;
        } else if CFG_SEGMENT {
            num_sends += send_udp_segment(fd, BUF[i as usize].as_mut_ptr()) as c_ulong;
        } else if CFG_SENDMMSG {
            num_sends += send_udp_sendmmsg(fd, BUF[i as usize].as_mut_ptr()) as c_ulong;
        } else {
            num_sends += send_udp(fd, BUF[i as usize].as_mut_ptr()) as c_ulong;
        }
        num_msgs += 1;
        if (CFG_ZEROCOPY && ((num_msgs & 0xF) == 0)) || CFG_TX_TSTAMP {
            flush_errqueue(fd, CFG_POLL, 500, true);
        }

        if CFG_MSG_NR != 0 && num_msgs >= CFG_MSG_NR as c_ulong {
            break;
        }

        tnow = gettimeofday_ms();
        if tnow >= treport {
            print_report(num_msgs, num_sends);
            num_msgs = 0;
            num_sends = 0;
            treport = tnow + 1000;
        }

        /* cold cache when writing buffer */
        if CFG_CACHE_TRASH {
            i += 1;
            i = if i < NUM_PKT as c_int { i } else { 0 };
        }

        if INTERRUPTED || !(CFG_RUNTIME_MS == -1 || tnow < tstop) {
            break;
        }
    }

    if CFG_ZEROCOPY || CFG_TX_TSTAMP {
        flush_errqueue_retry(fd, num_sends);
    }

    if close(fd) != 0 {
        error(1, *__errno_location(), cstr!("close"));
    }

    if CFG_AUDIT {
        TEND = tnow;
        TOTAL_NUM_MSGS += num_msgs;
        TOTAL_NUM_SENDS += num_sends;
        print_audit_report(TOTAL_NUM_MSGS, TOTAL_NUM_SENDS);
    }

    0
}

fn main() {
    unsafe {
        let args: Vec<CString> = std::env::args()
            .map(|arg| CString::new(arg).unwrap())
            .collect();
        let mut argv: Vec<*mut c_char> = args.iter().map(|arg| arg.as_ptr() as *mut c_char).collect();
        argv.push(ptr::null_mut());
        std::process::exit(main_0((argv.len() - 1) as c_int, argv.as_mut_ptr()));
    }
}
