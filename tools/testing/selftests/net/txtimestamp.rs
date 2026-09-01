// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2014 Google Inc.
 * Author: willemb@google.com (Willem de Bruijn)
 *
 * Test software tx timestamping, including
 *
 * - SCHED, SND and ACK timestamps
 * - RAW, UDP and TCP
 * - IPv4 and IPv6
 * - various packet sizes (to test GSO and TSO)
 *
 * Consult the command line arguments for help on running
 * the various testcases.
 *
 * This test requires a dummy TCP server.
 * A simple `nc6 [-u] -l -p $DESTPORT` will do
 */

extern crate libc;

use libc::*;
use std::ffi::CString;
use std::mem;
use std::ptr;

const NSEC_PER_USEC: i64 = 1000;
const USEC_PER_SEC: i64 = 1000000;
const NSEC_PER_SEC: i64 = 1000000000;

const SCM_TSTAMP_SCHED: i32 = 0;
const SCM_TSTAMP_SND: i32 = 1;
const SCM_TSTAMP_ACK: i32 = 2;
const SOF_TIMESTAMPING_TX_SOFTWARE: u32 = 1 << 1;
const SOF_TIMESTAMPING_TX_SCHED: u32 = 1 << 8;
const SOF_TIMESTAMPING_TX_ACK: u32 = 1 << 9;
const SOF_TIMESTAMPING_SOFTWARE: u32 = 1 << 4;
const SOF_TIMESTAMPING_OPT_ID: u32 = 1 << 7;
const SOF_TIMESTAMPING_OPT_CMSG: u32 = 1 << 10;
const SOF_TIMESTAMPING_OPT_TSONLY: u32 = 1 << 11;
const SO_TIMESTAMPING: i32 = 37;
const SCM_TIMESTAMPING: i32 = SO_TIMESTAMPING;
const SCM_TS_OPT_ID: i32 = 54;
const SO_EE_ORIGIN_TIMESTAMPING: u8 = 4;
const PACKET_TX_TIMESTAMP: i32 = 16;
const ETH_P_IP: u16 = 0x0800;
const ETH_P_IPV6: u16 = 0x86DD;
const ETH_ALEN: u8 = 6;

#[repr(C)]
struct ScmTimestamping {
    ts: [timespec; 3],
}

#[repr(C)]
struct SockExtendedErr {
    ee_errno: u32,
    ee_origin: u8,
    ee_type: u8,
    ee_code: u8,
    ee_pad: u8,
    ee_info: u32,
    ee_data: u32,
}

#[repr(C)]
struct TimingEvent {
    min: i64,
    max: i64,
    total: i64,
    count: i32,
}

static mut CFG_PROTO: i32 = SOCK_STREAM;
static mut CFG_IPPROTO: i32 = IPPROTO_TCP;
static mut CFG_NUM_PKTS: i32 = 4;
static mut DO_IPV4: i32 = 1;
static mut DO_IPV6: i32 = 1;
static mut CFG_PAYLOAD_LEN: i32 = 10;
static mut CFG_POLL_TIMEOUT: i32 = 100;
static mut CFG_DELAY_SND: i32 = 0;
static mut CFG_DELAY_ACK: i32 = 0;
static mut CFG_DELAY_TOLERANCE_USEC: i32 = 500;
static mut CFG_SHOW_PAYLOAD: bool = false;
static mut CFG_DO_PKTINFO: bool = false;
static mut CFG_BUSY_POLL: bool = false;
static mut CFG_SLEEP_USEC: i32 = 50 * 1000;
static mut CFG_LOOP_NODATA: bool = false;
static mut CFG_USE_CMSG: bool = false;
static mut CFG_USE_PF_PACKET: bool = false;
static mut CFG_USE_EPOLL: bool = false;
static mut CFG_EPOLLET: bool = false;
static mut CFG_DO_LISTEN: bool = false;
static mut DEST_PORT: u16 = 9000;
static mut CFG_PRINT_NSEC: bool = false;
static mut TS_OPT_ID: u32 = 0;
static mut CFG_USE_CMSG_OPT_ID: bool = false;

static mut DADDR: sockaddr_in = unsafe { mem::zeroed() };
static mut DADDR6: sockaddr_in6 = unsafe { mem::zeroed() };
static mut TS_USR: timespec = unsafe { mem::zeroed() };

static mut SAVED_TSKEY: i32 = -1;
static mut SAVED_TSKEY_TYPE: i32 = -1;

static mut USR_ENQ: TimingEvent = TimingEvent { min: 0, max: 0, total: 0, count: 0 };
static mut USR_SND: TimingEvent = TimingEvent { min: 0, max: 0, total: 0, count: 0 };
static mut USR_ACK: TimingEvent = TimingEvent { min: 0, max: 0, total: 0, count: 0 };
static mut TEST_FAILED: bool = false;

unsafe fn cstr(s: &str) -> CString {
    CString::new(s).unwrap()
}

unsafe fn die(status: i32, err: i32, msg: &str) -> ! {
    let cmsg = cstr(msg);
    error(status, err, cmsg.as_ptr());
    std::process::exit(status);
}

unsafe fn timespec_to_ns64(ts: *mut timespec) -> i64 {
    (*ts).tv_sec as i64 * NSEC_PER_SEC + (*ts).tv_nsec as i64
}

unsafe fn timespec_to_us64(ts: *mut timespec) -> i64 {
    (*ts).tv_sec as i64 * USEC_PER_SEC + (*ts).tv_nsec as i64 / NSEC_PER_USEC
}

unsafe fn init_timing_event(te: *mut TimingEvent) {
    (*te).min = i64::MAX;
    (*te).max = 0;
    (*te).total = 0;
    (*te).count = 0;
}

unsafe fn add_timing_event(te: *mut TimingEvent, t_start: *mut timespec, t_end: *mut timespec) {
    let ts_delta = timespec_to_ns64(t_end) - timespec_to_ns64(t_start);

    (*te).count += 1;
    if ts_delta < (*te).min {
        (*te).min = ts_delta;
    }
    if ts_delta > (*te).max {
        (*te).max = ts_delta;
    }
    (*te).total += ts_delta;
}

unsafe fn validate_key(tskey: i32, tstype: i32) {
    let mut stepsize: i32;

    /* compare key for each subsequent request
     * must only test for one type, the first one requested
     */
    if SAVED_TSKEY == -1 || CFG_USE_CMSG_OPT_ID {
        SAVED_TSKEY_TYPE = tstype;
    } else if SAVED_TSKEY_TYPE != tstype {
        return;
    }

    stepsize = if CFG_PROTO == SOCK_STREAM { CFG_PAYLOAD_LEN } else { 1 };
    stepsize = if CFG_USE_CMSG_OPT_ID { 0 } else { stepsize };
    if tskey != SAVED_TSKEY + stepsize {
        fprintf(stderr, cstr("ERROR: key %d, expected %d\n").as_ptr(), tskey, SAVED_TSKEY + stepsize);
        TEST_FAILED = true;
    }

    SAVED_TSKEY = tskey;
}

unsafe fn validate_timestamp(cur: *mut timespec, min_delay: i32) {
    let cur64 = timespec_to_us64(cur);
    let start64 = timespec_to_us64(ptr::addr_of_mut!(TS_USR));
    let max_delay = min_delay + CFG_DELAY_TOLERANCE_USEC;

    if cur64 < start64 + min_delay as i64 || cur64 > start64 + max_delay as i64 {
        fprintf(
            stderr,
            cstr("ERROR: %ld us expected between %d and %d\n").as_ptr(),
            cur64 - start64,
            min_delay,
            max_delay,
        );
        if getenv(cstr("KSFT_MACHINE_SLOW").as_ptr()).is_null() {
            TEST_FAILED = true;
        }
    }
}

unsafe fn __print_ts_delta_formatted(ts_delta: i64) {
    if CFG_PRINT_NSEC {
        fprintf(stderr, cstr("%ld ns").as_ptr(), ts_delta);
    } else {
        fprintf(stderr, cstr("%ld us").as_ptr(), ts_delta / NSEC_PER_USEC);
    }
}

unsafe fn __print_timestamp(name: *const c_char, cur: *mut timespec, key: u32, payload_len: i32) {
    let ts_delta: i64;

    if ((*cur).tv_sec | (*cur).tv_nsec) == 0 {
        return;
    }

    if CFG_PRINT_NSEC {
        fprintf(
            stderr,
            cstr("  %s: %lu s %lu ns (seq=%u, len=%u)").as_ptr(),
            name,
            (*cur).tv_sec as c_ulong,
            (*cur).tv_nsec as c_ulong,
            key,
            payload_len as u32,
        );
    } else {
        fprintf(
            stderr,
            cstr("  %s: %lu s %lu us (seq=%u, len=%u)").as_ptr(),
            name,
            (*cur).tv_sec as c_ulong,
            ((*cur).tv_nsec as i64 / NSEC_PER_USEC) as c_ulong,
            key,
            payload_len as u32,
        );
    }

    if cur != ptr::addr_of_mut!(TS_USR) {
        ts_delta = timespec_to_ns64(cur) - timespec_to_ns64(ptr::addr_of_mut!(TS_USR));
        fprintf(stderr, cstr("  (USR +").as_ptr());
        __print_ts_delta_formatted(ts_delta);
        fprintf(stderr, cstr(")").as_ptr());
    }

    fprintf(stderr, cstr("\n").as_ptr());
}

unsafe fn record_timestamp_usr() {
    if clock_gettime(CLOCK_REALTIME, ptr::addr_of_mut!(TS_USR)) != 0 {
        die(1, *__errno_location(), "clock_gettime");
    }
}

unsafe fn print_timestamp(tss: *mut ScmTimestamping, tstype: i32, tskey: i32, payload_len: i32) {
    let tsname: *const c_char;

    validate_key(tskey, tstype);

    match tstype {
        SCM_TSTAMP_SCHED => {
            tsname = cstr("  ENQ").into_raw();
            validate_timestamp(ptr::addr_of_mut!((*tss).ts[0]), 0);
            add_timing_event(ptr::addr_of_mut!(USR_ENQ), ptr::addr_of_mut!(TS_USR), ptr::addr_of_mut!((*tss).ts[0]));
        }
        SCM_TSTAMP_SND => {
            tsname = cstr("  SND").into_raw();
            validate_timestamp(ptr::addr_of_mut!((*tss).ts[0]), CFG_DELAY_SND);
            add_timing_event(ptr::addr_of_mut!(USR_SND), ptr::addr_of_mut!(TS_USR), ptr::addr_of_mut!((*tss).ts[0]));
        }
        SCM_TSTAMP_ACK => {
            tsname = cstr("  ACK").into_raw();
            validate_timestamp(ptr::addr_of_mut!((*tss).ts[0]), CFG_DELAY_ACK);
            add_timing_event(ptr::addr_of_mut!(USR_ACK), ptr::addr_of_mut!(TS_USR), ptr::addr_of_mut!((*tss).ts[0]));
        }
        _ => die(1, 0, "unknown timestamp type: %u"),
    }
    __print_timestamp(tsname, ptr::addr_of_mut!((*tss).ts[0]), tskey as u32, payload_len);
}

unsafe fn print_timing_event(name: *mut c_char, te: *mut TimingEvent) {
    if (*te).count == 0 {
        return;
    }

    fprintf(stderr, cstr("    %s: count=%d").as_ptr(), name, (*te).count);
    fprintf(stderr, cstr(", avg=").as_ptr());
    __print_ts_delta_formatted((*te).total / (*te).count as i64);
    fprintf(stderr, cstr(", min=").as_ptr());
    __print_ts_delta_formatted((*te).min);
    fprintf(stderr, cstr(", max=").as_ptr());
    __print_ts_delta_formatted((*te).max);
    fprintf(stderr, cstr("\n").as_ptr());
}

/* TODO: convert to check_and_print payload once API is stable */
unsafe fn print_payload(data: *mut c_char, mut len: i32) {
    let mut i: i32;

    if len == 0 {
        return;
    }

    if len > 70 {
        len = 70;
    }

    fprintf(stderr, cstr("payload: ").as_ptr());
    i = 0;
    while i < len {
        fprintf(stderr, cstr("%02hhx ").as_ptr(), *data.offset(i as isize) as c_int);
        i += 1;
    }
    fprintf(stderr, cstr("\n").as_ptr());
}

unsafe fn print_pktinfo(family: i32, ifindex: i32, saddr: *mut c_void, daddr: *mut c_void) {
    let mut sa = [0 as c_char; INET6_ADDRSTRLEN as usize];
    let mut da = [0 as c_char; INET6_ADDRSTRLEN as usize];
    let unknown = cstr("unknown");

    fprintf(
        stderr,
        cstr("         pktinfo: ifindex=%u src=%s dst=%s\n").as_ptr(),
        ifindex,
        if !saddr.is_null() { inet_ntop(family, saddr, sa.as_mut_ptr(), sa.len() as socklen_t) } else { unknown.as_ptr() },
        if !daddr.is_null() { inet_ntop(family, daddr, da.as_mut_ptr(), da.len() as socklen_t) } else { unknown.as_ptr() },
    );
}

unsafe fn __epoll(epfd: i32) {
    let mut events: epoll_event = mem::zeroed();
    let ret = epoll_wait(epfd, &mut events, 1, CFG_POLL_TIMEOUT);
    if ret != 1 {
        die(1, *__errno_location(), "epoll_wait");
    }
}

unsafe fn __poll(fd: i32) {
    let mut pollfd: pollfd = mem::zeroed();
    pollfd.fd = fd;
    let ret = poll(&mut pollfd, 1, CFG_POLL_TIMEOUT);
    if ret != 1 {
        die(1, *__errno_location(), "poll");
    }
}

unsafe fn cmsg_align(len: usize) -> usize {
    (len + mem::size_of::<usize>() - 1) & !(mem::size_of::<usize>() - 1)
}

unsafe fn cmsg_space(len: usize) -> usize {
    cmsg_align(mem::size_of::<cmsghdr>()) + cmsg_align(len)
}

unsafe fn cmsg_len(len: usize) -> usize {
    cmsg_align(mem::size_of::<cmsghdr>()) + len
}

unsafe fn cmsg_data(cmsg: *mut cmsghdr) -> *mut c_uchar {
    (cmsg as *mut c_uchar).add(cmsg_align(mem::size_of::<cmsghdr>()))
}

unsafe fn __recv_errmsg_cmsg(msg: *mut msghdr, payload_len: i32) {
    let mut serr: *mut SockExtendedErr = ptr::null_mut();
    let mut tss: *mut ScmTimestamping = ptr::null_mut();
    let mut cm = CMSG_FIRSTHDR(msg);
    let mut batch = 0;

    while !cm.is_null() && (*cm).cmsg_len != 0 {
        if (*cm).cmsg_level == SOL_SOCKET && (*cm).cmsg_type == SCM_TIMESTAMPING {
            tss = CMSG_DATA(cm) as *mut ScmTimestamping;
        } else if ((*cm).cmsg_level == SOL_IP && (*cm).cmsg_type == IP_RECVERR)
            || ((*cm).cmsg_level == SOL_IPV6 && (*cm).cmsg_type == IPV6_RECVERR)
            || ((*cm).cmsg_level == SOL_PACKET && (*cm).cmsg_type == PACKET_TX_TIMESTAMP)
        {
            serr = CMSG_DATA(cm) as *mut SockExtendedErr;
            if (*serr).ee_errno != ENOMSG as u32 || (*serr).ee_origin != SO_EE_ORIGIN_TIMESTAMPING {
                fprintf(stderr, cstr("unknown ip error %d %d\n").as_ptr(), (*serr).ee_errno, (*serr).ee_origin as c_int);
                serr = ptr::null_mut();
            }
        } else if (*cm).cmsg_level == SOL_IP && (*cm).cmsg_type == IP_PKTINFO {
            let info = CMSG_DATA(cm) as *mut in_pktinfo;
            print_pktinfo(AF_INET, (*info).ipi_ifindex, ptr::addr_of_mut!((*info).ipi_spec_dst) as *mut c_void, ptr::addr_of_mut!((*info).ipi_addr) as *mut c_void);
        } else if (*cm).cmsg_level == SOL_IPV6 && (*cm).cmsg_type == IPV6_PKTINFO {
            let info6 = CMSG_DATA(cm) as *mut in6_pktinfo;
            print_pktinfo(AF_INET6, (*info6).ipi6_ifindex, ptr::null_mut(), ptr::addr_of_mut!((*info6).ipi6_addr) as *mut c_void);
        } else {
            fprintf(stderr, cstr("unknown cmsg %d,%d\n").as_ptr(), (*cm).cmsg_level, (*cm).cmsg_type);
        }

        if !serr.is_null() && !tss.is_null() {
            print_timestamp(tss, (*serr).ee_info as i32, (*serr).ee_data as i32, payload_len);
            serr = ptr::null_mut();
            tss = ptr::null_mut();
            batch += 1;
        }
        cm = CMSG_NXTHDR(msg, cm);
    }

    if batch > 1 {
        fprintf(stderr, cstr("batched %d timestamps\n").as_ptr(), batch);
    } else if batch == 0 {
        fprintf(stderr, cstr("Failed to report timestamps\n").as_ptr());
        TEST_FAILED = true;
    }
}

unsafe fn recv_errmsg(fd: i32) -> i32 {
    static mut CTRL: [c_char; 1024] = [0; 1024 /* overprovision*/];
    static mut MSG: msghdr = msghdr {
        msg_name: ptr::null_mut(),
        msg_namelen: 0,
        msg_iov: ptr::null_mut(),
        msg_iovlen: 0,
        msg_control: ptr::null_mut(),
        msg_controllen: 0,
        msg_flags: 0,
    };
    static mut DATA: *mut c_char = ptr::null_mut();
    let mut entry: iovec = mem::zeroed();
    let mut ret = 0;

    DATA = malloc(CFG_PAYLOAD_LEN as usize) as *mut c_char;
    if DATA.is_null() {
        die(1, 0, "malloc");
    }

    ptr::write_bytes(ptr::addr_of_mut!(MSG), 0, 1);
    ptr::write_bytes(&mut entry, 0, 1);
    ptr::write_bytes(CTRL.as_mut_ptr(), 0, CTRL.len());

    entry.iov_base = DATA as *mut c_void;
    entry.iov_len = CFG_PAYLOAD_LEN as usize;
    MSG.msg_iov = &mut entry;
    MSG.msg_iovlen = 1;
    MSG.msg_name = ptr::null_mut();
    MSG.msg_namelen = 0;
    MSG.msg_control = CTRL.as_mut_ptr() as *mut c_void;
    MSG.msg_controllen = CTRL.len();

    ret = recvmsg(fd, ptr::addr_of_mut!(MSG), MSG_ERRQUEUE);
    if ret == -1 && *__errno_location() != EAGAIN {
        die(1, *__errno_location(), "recvmsg");
    }

    if ret >= 0 {
        __recv_errmsg_cmsg(ptr::addr_of_mut!(MSG), ret as i32);
        if CFG_SHOW_PAYLOAD {
            print_payload(DATA, CFG_PAYLOAD_LEN);
        }
    }

    free(DATA as *mut c_void);
    (ret == -1) as i32
}

unsafe fn get_ip_csum(start: *const u16, num_words: i32, mut sum: c_ulong) -> u16 {
    let mut i = 0;

    while i < num_words {
        sum = sum.wrapping_add(*start.offset(i as isize) as c_ulong);
        i += 1;
    }

    while (sum >> 16) != 0 {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }

    !(sum as u16)
}

unsafe fn get_udp_csum(udph: *const udphdr, alen: i32) -> u16 {
    let mut pseudo_sum: c_ulong;
    let csum_len: c_ulong;
    let mut csum_start = udph as *const c_void;

    pseudo_sum = htons(IPPROTO_UDP as u16) as c_ulong;
    pseudo_sum = pseudo_sum.wrapping_add((*udph).len as c_ulong);

    /* checksum ip(v6) addresses + udp header + payload */
    csum_start = (csum_start as *const u8).offset(-(alen * 2) as isize) as *const c_void;
    csum_len = ntohs((*udph).len) as c_ulong + (alen * 2) as c_ulong;

    get_ip_csum(csum_start as *const u16, (csum_len >> 1) as i32, pseudo_sum)
}

unsafe fn fill_header_ipv4(p: *mut c_void) -> i32 {
    let iph = p as *mut iphdr;

    ptr::write_bytes(iph, 0, 1);

    (*iph).set_ihl(5);
    (*iph).set_version(4);
    (*iph).ttl = 2;
    (*iph).saddr = DADDR.sin_addr.s_addr; /* set for udp csum calc */
    (*iph).daddr = DADDR.sin_addr.s_addr;
    (*iph).protocol = IPPROTO_UDP as u8;

    /* kernel writes saddr, csum, len */

    mem::size_of::<iphdr>() as i32
}

unsafe fn fill_header_ipv6(p: *mut c_void) -> i32 {
    let ip6h = p as *mut ip6_hdr;

    ptr::write_bytes(ip6h, 0, 1);

    (*ip6h).ip6_ctlun.ip6_un1.ip6_un1_flow = htonl(6u32 << 28);
    (*ip6h).ip6_ctlun.ip6_un1.ip6_un1_plen = htons((mem::size_of::<udphdr>() as i32 + CFG_PAYLOAD_LEN) as u16);
    (*ip6h).ip6_ctlun.ip6_un1.ip6_un1_nxt = IPPROTO_UDP as u8;
    (*ip6h).ip6_ctlun.ip6_un1.ip6_un1_hlim = 64;

    (*ip6h).ip6_src = DADDR6.sin6_addr;
    (*ip6h).ip6_dst = DADDR6.sin6_addr;

    /* kernel does not write saddr in case of ipv6 */

    mem::size_of::<ip6_hdr>() as i32
}

unsafe fn fill_header_udp(p: *mut c_void, is_ipv4: bool) {
    let udph = p as *mut udphdr;

    (*udph).source = ntohs(DEST_PORT + 1); /* spoof */
    (*udph).dest = ntohs(DEST_PORT);
    (*udph).len = ntohs((mem::size_of::<udphdr>() as i32 + CFG_PAYLOAD_LEN) as u16);
    (*udph).check = 0;

    (*udph).check = get_udp_csum(
        udph,
        if is_ipv4 {
            mem::size_of::<in_addr>() as i32
        } else {
            mem::size_of::<in6_addr>() as i32
        },
    );
}

unsafe fn do_test(family: i32, report_opt: u32) {
    let mut control = [0 as c_char; 2 * 32];
    let mut laddr: sockaddr_ll = mem::zeroed();
    let mut sock_opt: u32;
    let mut cmsg: *mut cmsghdr;
    let mut msg: msghdr = mem::zeroed();
    let mut iov: iovec = mem::zeroed();
    let mut buf: *mut c_char;
    let mut fd: i32;
    let mut i: i32;
    let mut val: i32 = 1;
    let mut total_len: i32;
    let mut epfd: i32 = 0;

    init_timing_event(ptr::addr_of_mut!(USR_ENQ));
    init_timing_event(ptr::addr_of_mut!(USR_SND));
    init_timing_event(ptr::addr_of_mut!(USR_ACK));

    total_len = CFG_PAYLOAD_LEN;
    if CFG_USE_PF_PACKET || CFG_PROTO == SOCK_RAW {
        total_len += mem::size_of::<udphdr>() as i32;
        if CFG_USE_PF_PACKET || CFG_IPPROTO == IPPROTO_RAW {
            if family == PF_INET {
                total_len += mem::size_of::<iphdr>() as i32;
            } else {
                total_len += mem::size_of::<ip6_hdr>() as i32;
            }
        }
        /* special case, only rawv6_sendmsg:
         * pass proto in sin6_port if not connected
         * also see ANK comment in net/ipv4/raw.c
         */
        DADDR6.sin6_port = htons(CFG_IPPROTO as u16);
    }

    buf = malloc(total_len as usize) as *mut c_char;
    if buf.is_null() {
        die(1, 0, "malloc");
    }

    fd = socket(if CFG_USE_PF_PACKET { PF_PACKET } else { family }, CFG_PROTO, CFG_IPPROTO);
    if fd < 0 {
        die(1, *__errno_location(), "socket");
    }

    if CFG_USE_EPOLL {
        let mut ev: epoll_event = mem::zeroed();

        ev.u64 = fd as u64;
        if CFG_EPOLLET {
            ev.events |= EPOLLET as u32;
        }
        epfd = epoll_create(1);
        if epfd <= 0 {
            die(1, *__errno_location(), "epoll_create");
        }
        if epoll_ctl(epfd, EPOLL_CTL_ADD, fd, &mut ev) != 0 {
            die(1, *__errno_location(), "epoll_ctl");
        }
    }

    /* reset expected key on each new socket */
    SAVED_TSKEY = -1;

    if CFG_PROTO == SOCK_STREAM {
        if setsockopt(fd, IPPROTO_TCP, TCP_NODELAY, &val as *const _ as *const c_void, mem::size_of_val(&val) as socklen_t) != 0 {
            die(1, 0, "setsockopt no nagle");
        }

        if family == PF_INET {
            if connect(fd, ptr::addr_of!(DADDR) as *const sockaddr, mem::size_of::<sockaddr_in>() as socklen_t) != 0 {
                die(1, *__errno_location(), "connect ipv4");
            }
        } else if connect(fd, ptr::addr_of!(DADDR6) as *const sockaddr, mem::size_of::<sockaddr_in6>() as socklen_t) != 0 {
            die(1, *__errno_location(), "connect ipv6");
        }
    }

    if CFG_DO_PKTINFO {
        if family == AF_INET6 {
            if setsockopt(fd, SOL_IPV6, IPV6_RECVPKTINFO, &val as *const _ as *const c_void, mem::size_of_val(&val) as socklen_t) != 0 {
                die(1, *__errno_location(), "setsockopt pktinfo ipv6");
            }
        } else if setsockopt(fd, SOL_IP, IP_PKTINFO, &val as *const _ as *const c_void, mem::size_of_val(&val) as socklen_t) != 0 {
            die(1, *__errno_location(), "setsockopt pktinfo ipv4");
        }
    }

    sock_opt = SOF_TIMESTAMPING_SOFTWARE | SOF_TIMESTAMPING_OPT_CMSG | SOF_TIMESTAMPING_OPT_ID;

    if !CFG_USE_CMSG {
        sock_opt |= report_opt;
    }

    if CFG_LOOP_NODATA {
        sock_opt |= SOF_TIMESTAMPING_OPT_TSONLY;
    }

    if setsockopt(fd, SOL_SOCKET, SO_TIMESTAMPING, &sock_opt as *const _ as *const c_void, mem::size_of_val(&sock_opt) as socklen_t) != 0 {
        die(1, 0, "setsockopt timestamping");
    }

    i = 0;
    while i < CFG_NUM_PKTS {
        ptr::write_bytes(&mut msg, 0, 1);
        ptr::write_bytes(buf, ('a' as i32 + i) as u8, total_len as usize);

        if CFG_USE_PF_PACKET || CFG_PROTO == SOCK_RAW {
            let mut off = 0;

            if CFG_USE_PF_PACKET || CFG_IPPROTO == IPPROTO_RAW {
                if family == PF_INET {
                    off = fill_header_ipv4(buf as *mut c_void);
                } else {
                    off = fill_header_ipv6(buf as *mut c_void);
                }
            }

            fill_header_udp(buf.offset(off as isize) as *mut c_void, family == PF_INET);
        }

        iov.iov_base = buf as *mut c_void;
        iov.iov_len = total_len as usize;

        if CFG_PROTO != SOCK_STREAM {
            if CFG_USE_PF_PACKET {
                ptr::write_bytes(&mut laddr, 0, 1);

                laddr.sll_family = AF_PACKET as u16;
                laddr.sll_ifindex = 1;
                laddr.sll_protocol = htons(if family == AF_INET { ETH_P_IP } else { ETH_P_IPV6 });
                laddr.sll_halen = ETH_ALEN;

                msg.msg_name = &mut laddr as *mut _ as *mut c_void;
                msg.msg_namelen = mem::size_of::<sockaddr_ll>() as socklen_t;
            } else if family == PF_INET {
                msg.msg_name = ptr::addr_of_mut!(DADDR) as *mut c_void;
                msg.msg_namelen = mem::size_of::<sockaddr_in>() as socklen_t;
            } else {
                msg.msg_name = ptr::addr_of_mut!(DADDR6) as *mut c_void;
                msg.msg_namelen = mem::size_of::<sockaddr_in6>() as socklen_t;
            }
        }

        msg.msg_iov = &mut iov;
        msg.msg_iovlen = 1;

        if CFG_USE_CMSG || CFG_USE_CMSG_OPT_ID {
            ptr::write_bytes(control.as_mut_ptr(), 0, control.len());

            msg.msg_control = control.as_mut_ptr() as *mut c_void;
            msg.msg_controllen = (CFG_USE_CMSG as usize) * cmsg_space(mem::size_of::<u32>());
            msg.msg_controllen += (CFG_USE_CMSG_OPT_ID as usize) * cmsg_space(mem::size_of::<u32>());

            cmsg = ptr::null_mut();
            if CFG_USE_CMSG {
                cmsg = CMSG_FIRSTHDR(&mut msg);
                (*cmsg).cmsg_level = SOL_SOCKET;
                (*cmsg).cmsg_type = SO_TIMESTAMPING;
                (*cmsg).cmsg_len = cmsg_len(mem::size_of::<u32>());

                *(CMSG_DATA(cmsg) as *mut u32) = report_opt;
            }
            if CFG_USE_CMSG_OPT_ID {
                cmsg = if !cmsg.is_null() { CMSG_NXTHDR(&mut msg, cmsg) } else { CMSG_FIRSTHDR(&mut msg) };
                (*cmsg).cmsg_level = SOL_SOCKET;
                (*cmsg).cmsg_type = SCM_TS_OPT_ID;
                (*cmsg).cmsg_len = cmsg_len(mem::size_of::<u32>());

                *(CMSG_DATA(cmsg) as *mut u32) = TS_OPT_ID;
                SAVED_TSKEY = TS_OPT_ID as i32;
            }
        }

        record_timestamp_usr();

        val = sendmsg(fd, &msg, 0) as i32;
        if val != total_len {
            die(1, *__errno_location(), "send");
        }

        __print_timestamp(cstr("  USR").as_ptr(), ptr::addr_of_mut!(TS_USR), 0, 0);

        /* wait for all errors to be queued, else ACKs arrive OOO */
        if CFG_SLEEP_USEC != 0 {
            usleep(CFG_SLEEP_USEC as useconds_t);
        }

        if !CFG_BUSY_POLL {
            if CFG_USE_EPOLL {
                __epoll(epfd);
            } else {
                __poll(fd);
            }
        }

        while recv_errmsg(fd) == 0 {}
        i += 1;
    }

    print_timing_event(cstr("USR-ENQ").as_ptr() as *mut c_char, ptr::addr_of_mut!(USR_ENQ));
    print_timing_event(cstr("USR-SND").as_ptr() as *mut c_char, ptr::addr_of_mut!(USR_SND));
    print_timing_event(cstr("USR-ACK").as_ptr() as *mut c_char, ptr::addr_of_mut!(USR_ACK));

    if close(fd) != 0 {
        die(1, *__errno_location(), "close");
    }

    free(buf as *mut c_void);
    usleep((100 * NSEC_PER_USEC) as useconds_t);
}

unsafe fn usage(filepath: *const c_char) -> ! {
    fprintf(
        stderr,
        cstr("\nUsage: %s [options] hostname\n\nwhere options are:\n  -4:   only IPv4\n  -6:   only IPv6\n  -h:   show this message\n  -b:   busy poll to read from error queue\n  -c N: number of packets for each test\n  -C:   use cmsg to set tstamp recording options\n  -e:   use level-triggered epoll() instead of poll()\n  -E:   use event-triggered epoll() instead of poll()\n  -F:   poll()/epoll() waits forever for an event\n  -I:   request PKTINFO\n  -l N: send N bytes at a time\n  -L    listen on hostname and port\n  -n:   set no-payload option\n  -N:   print timestamps and durations in nsec (instead of usec)\n  -o N: use SCM_TS_OPT_ID control message to provide N as tskey\n  -p N: connect to port N\n  -P:   use PF_PACKET\n  -r:   use raw\n  -R:   use raw (IP_HDRINCL)\n  -S N: usec to sleep before reading error queue\n  -t N: tolerance (usec) for timestamp validation\n  -u:   use udp\n  -v:   validate SND delay (usec)\n  -V:   validate ACK delay (usec)\n  -x:   show payload (up to 70 bytes)\n").as_ptr(),
        filepath,
    );
    exit(1);
}

unsafe fn parse_opt(argc: i32, argv: *mut *mut c_char) {
    let mut proto_count = 0;
    let mut c: i32;

    loop {
        c = getopt(argc, argv, cstr("46bc:CeEFhIl:LnNo:p:PrRS:t:uv:V:x").as_ptr());
        if c == -1 {
            break;
        }
        match c as u8 as char {
            '4' => DO_IPV6 = 0,
            '6' => DO_IPV4 = 0,
            'b' => CFG_BUSY_POLL = true,
            'c' => CFG_NUM_PKTS = strtoul(optarg, ptr::null_mut(), 10) as i32,
            'C' => CFG_USE_CMSG = true,
            'e' => CFG_USE_EPOLL = true,
            'E' => {
                CFG_USE_EPOLL = true;
                CFG_EPOLLET = true;
                CFG_POLL_TIMEOUT = -1;
            }
            'F' => CFG_POLL_TIMEOUT = -1,
            'I' => CFG_DO_PKTINFO = true,
            'l' => CFG_PAYLOAD_LEN = strtoul(optarg, ptr::null_mut(), 10) as i32,
            'L' => CFG_DO_LISTEN = true,
            'n' => CFG_LOOP_NODATA = true,
            'N' => CFG_PRINT_NSEC = true,
            'o' => {
                TS_OPT_ID = strtoul(optarg, ptr::null_mut(), 10) as u32;
                CFG_USE_CMSG_OPT_ID = true;
            }
            'p' => DEST_PORT = strtoul(optarg, ptr::null_mut(), 10) as u16,
            'P' => {
                proto_count += 1;
                CFG_USE_PF_PACKET = true;
                CFG_PROTO = SOCK_DGRAM;
                CFG_IPPROTO = 0;
            }
            'r' => {
                proto_count += 1;
                CFG_PROTO = SOCK_RAW;
                CFG_IPPROTO = IPPROTO_UDP;
            }
            'R' => {
                proto_count += 1;
                CFG_PROTO = SOCK_RAW;
                CFG_IPPROTO = IPPROTO_RAW;
            }
            'S' => CFG_SLEEP_USEC = strtoul(optarg, ptr::null_mut(), 10) as i32,
            't' => CFG_DELAY_TOLERANCE_USEC = strtoul(optarg, ptr::null_mut(), 10) as i32,
            'u' => {
                proto_count += 1;
                CFG_PROTO = SOCK_DGRAM;
                CFG_IPPROTO = IPPROTO_UDP;
            }
            'v' => CFG_DELAY_SND = strtoul(optarg, ptr::null_mut(), 10) as i32,
            'V' => CFG_DELAY_ACK = strtoul(optarg, ptr::null_mut(), 10) as i32,
            'x' => CFG_SHOW_PAYLOAD = true,
            'h' | _ => usage(*argv.offset(0)),
        }
    }

    if CFG_PAYLOAD_LEN == 0 {
        die(1, 0, "payload may not be nonzero");
    }
    if CFG_PROTO != SOCK_STREAM && CFG_PAYLOAD_LEN > 1472 {
        die(1, 0, "udp packet might exceed expected MTU");
    }
    if DO_IPV4 == 0 && DO_IPV6 == 0 {
        die(1, 0, "pass -4 or -6, not both");
    }
    if proto_count > 1 {
        die(1, 0, "pass -P, -r, -R or -u, not multiple");
    }
    if CFG_DO_PKTINFO && CFG_USE_PF_PACKET {
        die(1, 0, "cannot ask for pktinfo over pf_packet");
    }
    if CFG_BUSY_POLL && CFG_USE_EPOLL {
        die(1, 0, "pass epoll or busy_poll, not both");
    }
    if CFG_PROTO == SOCK_STREAM && CFG_USE_CMSG_OPT_ID {
        die(1, 0, "TCP sockets don't support SCM_TS_OPT_ID");
    }

    if optind != argc - 1 {
        die(1, 0, "missing required hostname argument");
    }
}

unsafe fn resolve_hostname(hostname: *const c_char) {
    let mut hints: addrinfo = mem::zeroed();
    let mut addrs: *mut addrinfo = ptr::null_mut();
    let mut cur: *mut addrinfo;
    let mut have_ipv4 = 0;
    let mut have_ipv6 = 0;

    hints.ai_family = if DO_IPV4 != 0 { AF_INET } else { AF_INET6 };

    loop {
        if getaddrinfo(hostname, ptr::null(), &hints, &mut addrs) != 0 {
            die(1, *__errno_location(), "getaddrinfo");
        }

        cur = addrs;
        while !cur.is_null() && have_ipv4 == 0 && have_ipv6 == 0 {
            if have_ipv4 == 0 && (*cur).ai_family == AF_INET {
                ptr::copy_nonoverlapping((*cur).ai_addr as *const u8, ptr::addr_of_mut!(DADDR) as *mut u8, mem::size_of::<sockaddr_in>());
                DADDR.sin_port = htons(DEST_PORT);
                have_ipv4 = 1;
            } else if have_ipv6 == 0 && (*cur).ai_family == AF_INET6 {
                ptr::copy_nonoverlapping((*cur).ai_addr as *const u8, ptr::addr_of_mut!(DADDR6) as *mut u8, mem::size_of::<sockaddr_in6>());
                DADDR6.sin6_port = htons(DEST_PORT);
                have_ipv6 = 1;
            }
            cur = (*cur).ai_next;
        }
        if !addrs.is_null() {
            freeaddrinfo(addrs);
        }

        if DO_IPV6 != 0 && hints.ai_family != AF_INET6 {
            hints.ai_family = AF_INET6;
            continue;
        }
        break;
    }

    DO_IPV4 &= have_ipv4;
    DO_IPV6 &= have_ipv6;
}

unsafe fn do_listen(family: i32, addr: *mut c_void, alen: i32) {
    let fd: i32;
    let type_: i32;

    type_ = if CFG_PROTO == SOCK_RAW { SOCK_DGRAM } else { CFG_PROTO };

    fd = socket(family, type_, 0);
    if fd == -1 {
        die(1, *__errno_location(), "socket rx");
    }

    if bind(fd, addr as *const sockaddr, alen as socklen_t) != 0 {
        die(1, *__errno_location(), "bind rx");
    }

    if type_ == SOCK_STREAM && listen(fd, 10) != 0 {
        die(1, *__errno_location(), "listen rx");
    }

    /* leave fd open, will be closed on process exit.
     * this enables connect() to succeed and avoids icmp replies
     */
}

unsafe fn do_main(family: i32) {
    fprintf(
        stderr,
        cstr("family:       %s %s\n").as_ptr(),
        if family == PF_INET { cstr("INET").as_ptr() } else { cstr("INET6").as_ptr() },
        if CFG_USE_PF_PACKET { cstr("(PF_PACKET)").as_ptr() } else { cstr("").as_ptr() },
    );

    fprintf(stderr, cstr("test SND\n").as_ptr());
    do_test(family, SOF_TIMESTAMPING_TX_SOFTWARE);

    fprintf(stderr, cstr("test ENQ\n").as_ptr());
    do_test(family, SOF_TIMESTAMPING_TX_SCHED);

    fprintf(stderr, cstr("test ENQ + SND\n").as_ptr());
    do_test(family, SOF_TIMESTAMPING_TX_SCHED | SOF_TIMESTAMPING_TX_SOFTWARE);

    if CFG_PROTO == SOCK_STREAM {
        fprintf(stderr, cstr("\ntest ACK\n").as_ptr());
        do_test(family, SOF_TIMESTAMPING_TX_ACK);

        fprintf(stderr, cstr("\ntest SND + ACK\n").as_ptr());
        do_test(family, SOF_TIMESTAMPING_TX_SOFTWARE | SOF_TIMESTAMPING_TX_ACK);

        fprintf(stderr, cstr("\ntest ENQ + SND + ACK\n").as_ptr());
        do_test(family, SOF_TIMESTAMPING_TX_SCHED | SOF_TIMESTAMPING_TX_SOFTWARE | SOF_TIMESTAMPING_TX_ACK);
    }
}

static SOCK_NAMES: [*const c_char; 4] = [
    ptr::null(),
    b"TCP\0".as_ptr() as *const c_char,
    b"UDP\0".as_ptr() as *const c_char,
    b"RAW\0".as_ptr() as *const c_char,
];

fn main() {
    unsafe {
        let mut args: Vec<CString> = std::env::args().map(|s| CString::new(s).unwrap()).collect();
        let mut argv: Vec<*mut c_char> = args.iter_mut().map(|s| s.as_ptr() as *mut c_char).collect();
        argv.push(ptr::null_mut());
        let argc = (argv.len() - 1) as i32;

        if argc == 1 {
            usage(argv[0]);
        }

        parse_opt(argc, argv.as_mut_ptr());
        resolve_hostname(argv[(argc - 1) as usize]);

        fprintf(stderr, cstr("protocol:     %s\n").as_ptr(), SOCK_NAMES[CFG_PROTO as usize]);
        fprintf(stderr, cstr("payload:      %u\n").as_ptr(), CFG_PAYLOAD_LEN);
        fprintf(stderr, cstr("server port:  %u\n").as_ptr(), DEST_PORT as c_uint);
        fprintf(stderr, cstr("\n").as_ptr());

        if DO_IPV4 != 0 {
            if CFG_DO_LISTEN {
                do_listen(PF_INET, ptr::addr_of_mut!(DADDR) as *mut c_void, mem::size_of::<sockaddr_in>() as i32);
            }
            do_main(PF_INET);
        }

        if DO_IPV6 != 0 {
            if CFG_DO_LISTEN {
                do_listen(PF_INET6, ptr::addr_of_mut!(DADDR6) as *mut c_void, mem::size_of::<sockaddr_in6>() as i32);
            }
            do_main(PF_INET6);
        }

        std::process::exit(TEST_FAILED as i32);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
