/* Evaluate MSG_ZEROCOPY
 *
 * Send traffic between two processes over one of the supported
 * protocols and modes:
 *
 * PF_INET/PF_INET6
 * - SOCK_STREAM
 * - SOCK_DGRAM
 * - SOCK_DGRAM with UDP_CORK
 * - SOCK_RAW
 * - SOCK_RAW with IP_HDRINCL
 *
 * PF_PACKET
 * - SOCK_DGRAM
 * - SOCK_RAW
 *
 * PF_RDS
 * - SOCK_SEQPACKET
 *
 * Start this program on two connected hosts, one in send mode and
 * the other with option '-r' to put it in receiver mode.
 *
 * If zerocopy mode ('-z') is enabled, the sender will verify that
 * the kernel queues completions on the error queue for all zerocopy
 * transfers.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(static_mut_refs)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

unsafe extern "C" {
    fn error(status: c_int, errnum: c_int, format: *const c_char, ...);
}

const SO_EE_ORIGIN_ZEROCOPY: u8 = 5;
const SO_ZEROCOPY: c_int = 60;
const SO_EE_CODE_ZEROCOPY_COPIED: u8 = 1;
const MSG_ZEROCOPY: c_int = 0x4000000;

const IP_MAXPACKET: usize = 65535;
const PF_UNSPEC: c_int = libc::PF_UNSPEC;
const PF_INET: c_int = libc::PF_INET;
const PF_INET6: c_int = libc::PF_INET6;
const PF_PACKET: c_int = libc::PF_PACKET;
const PF_RDS: c_int = 21;
const AF_INET: c_int = libc::AF_INET;
const AF_INET6: c_int = libc::AF_INET6;
const AF_PACKET: c_int = libc::AF_PACKET;
const SOCK_STREAM: c_int = libc::SOCK_STREAM;
const SOCK_DGRAM: c_int = libc::SOCK_DGRAM;
const SOCK_RAW: c_int = libc::SOCK_RAW;
const SOCK_SEQPACKET: c_int = libc::SOCK_SEQPACKET;
const SOL_SOCKET: c_int = libc::SOL_SOCKET;
const SOL_IP: c_int = libc::SOL_IP;
const SOL_IPV6: c_int = libc::SOL_IPV6;
const SOL_PACKET: c_int = 263;
const SOL_RDS: c_int = 276;
const SO_SNDBUF: c_int = libc::SO_SNDBUF;
const SO_RCVBUF: c_int = libc::SO_RCVBUF;
const SO_RCVLOWAT: c_int = libc::SO_RCVLOWAT;
const SO_REUSEPORT: c_int = libc::SO_REUSEPORT;
const IPPROTO_UDP: c_int = libc::IPPROTO_UDP;
const IPPROTO_EGP: c_int = 8;
const IPPROTO_RAW: c_int = libc::IPPROTO_RAW;
const UDP_CORK: c_int = 1;
const ETH_ALEN: usize = 6;
const ETH_P_IP: u16 = 0x0800;
const ETH_P_IPV6: u16 = 0x86DD;
const IP_RECVERR: c_int = 11;
const IPV6_RECVERR: c_int = 25;
const PACKET_TX_TIMESTAMP: c_int = 16;
const MSG_DONTWAIT: c_int = libc::MSG_DONTWAIT;
const MSG_ERRQUEUE: c_int = libc::MSG_ERRQUEUE;
const MSG_CTRUNC: c_int = libc::MSG_CTRUNC;
const MSG_TRUNC: c_int = libc::MSG_TRUNC;
const POLLIN: c_short = libc::POLLIN;
const POLLOUT: c_short = libc::POLLOUT;
const POLLERR: c_short = libc::POLLERR;
const EAGAIN: c_int = libc::EAGAIN;
const RDS_CMSG_ZCOPY_COOKIE: c_int = 2;
const RDS_CMSG_ZCOPY_COMPLETION: c_int = 3;
const RDS_MAX_ZCOOKIES: c_int = 8;

type c_short = i16;
type socklen_t = libc::socklen_t;

#[repr(C)]
struct ethhdr {
    h_dest: [u8; ETH_ALEN],
    h_source: [u8; ETH_ALEN],
    h_proto: u16,
}

#[repr(C)]
struct rds_zcopy_cookies {
    num: u32,
    cookies: [u32; RDS_MAX_ZCOOKIES as usize],
}

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
struct iphdr {
    ihl_version: u8,
    tos: u8,
    tot_len: u16,
    id: u16,
    frag_off: u16,
    ttl: u8,
    protocol: u8,
    check: u16,
    saddr: u32,
    daddr: u32,
}

impl iphdr {
    unsafe fn set_ihl(&mut self, ihl: u8) {
        self.ihl_version = (self.ihl_version & 0xF0) | (ihl & 0x0F);
    }

    unsafe fn set_version(&mut self, version: u8) {
        self.ihl_version = (self.ihl_version & 0x0F) | ((version & 0x0F) << 4);
    }

    unsafe fn ihl(&self) -> u8 {
        self.ihl_version & 0x0F
    }
}

#[repr(C)]
struct ipv6hdr {
    priority_version: u8,
    flow_lbl: [u8; 3],
    payload_len: u16,
    nexthdr: u8,
    hop_limit: u8,
    saddr: libc::in6_addr,
    daddr: libc::in6_addr,
}

impl ipv6hdr {
    unsafe fn set_version(&mut self, version: u8) {
        self.priority_version = (self.priority_version & 0x0F) | ((version & 0x0F) << 4);
    }
}

#[repr(C)]
union nh_union {
    ip6h: core::mem::ManuallyDrop<ipv6hdr>,
    iph: core::mem::ManuallyDrop<iphdr>,
}

static mut cfg_cork: c_int = 0;
static mut cfg_cork_mixed: bool = false;
static mut cfg_cpu: c_int = -1; /* default: pin to last cpu */
static mut cfg_expect_zerocopy: c_int = -1;
static mut cfg_family: c_int = PF_UNSPEC;
static mut cfg_ifindex: c_int = 1;
static mut cfg_payload_len: c_int = 0;
static mut cfg_port: c_int = 8000;
static mut cfg_rx: bool = false;
static mut cfg_runtime_ms: c_int = 4200;
static mut cfg_verbose: c_int = 0;
static mut cfg_waittime_ms: c_int = 500;
static mut cfg_notification_limit: c_int = 32;
static mut cfg_zerocopy: bool = false;

static mut cfg_alen: socklen_t = 0;
static mut cfg_dst_addr: libc::sockaddr_storage = unsafe { zeroed() };
static mut cfg_src_addr: libc::sockaddr_storage = unsafe { zeroed() };

static mut exitcode: c_int = 0;
static mut payload: [c_char; IP_MAXPACKET] = [0; IP_MAXPACKET];
static mut packets: c_long = 0;
static mut bytes: c_long = 0;
static mut completions: c_long = 0;
static mut expected_completions: c_long = 0;
static mut next_completion: u32 = 0;
static mut sends_since_notify: u32 = 0;

unsafe fn cmsg_align(len: usize) -> usize {
    (len + size_of::<usize>() - 1) & !(size_of::<usize>() - 1)
}

unsafe fn CMSG_SPACE(len: usize) -> usize {
    cmsg_align(size_of::<libc::cmsghdr>()) + cmsg_align(len)
}

unsafe fn CMSG_LEN(len: usize) -> usize {
    cmsg_align(size_of::<libc::cmsghdr>()) + len
}

unsafe fn CMSG_DATA(cmsg: *mut libc::cmsghdr) -> *mut u8 {
    (cmsg as *mut u8).add(cmsg_align(size_of::<libc::cmsghdr>()))
}

unsafe fn CMSG_FIRSTHDR(msg: *const libc::msghdr) -> *mut libc::cmsghdr {
    if (*msg).msg_controllen as usize >= size_of::<libc::cmsghdr>() {
        (*msg).msg_control as *mut libc::cmsghdr
    } else {
        null_mut()
    }
}

unsafe fn CMSG_NXTHDR(msg: *const libc::msghdr, cmsg: *mut libc::cmsghdr) -> *mut libc::cmsghdr {
    let next = (cmsg as *mut u8).add(cmsg_align((*cmsg).cmsg_len as usize)) as *mut libc::cmsghdr;
    let max = ((*msg).msg_control as *mut u8).add((*msg).msg_controllen as usize);
    if (next as *mut u8).add(size_of::<libc::cmsghdr>()) > max {
        null_mut()
    } else {
        next
    }
}

unsafe fn gettimeofday_ms() -> c_ulong {
    let mut tv: libc::timeval = zeroed();

    libc::gettimeofday(&mut tv, null_mut());
    (tv.tv_sec as c_ulong * 1000) + (tv.tv_usec as c_ulong / 1000)
}

unsafe fn get_ip_csum(start: *const u16, num_words: c_int) -> u16 {
    let mut sum: c_ulong = 0;
    let mut i: c_int;

    i = 0;
    while i < num_words {
        sum = sum.wrapping_add(*start.add(i as usize) as c_ulong);
        i += 1;
    }

    while (sum >> 16) != 0 {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }

    !(sum as u16)
}

unsafe fn do_setcpu(cpu: c_int) -> c_int {
    let mut mask: libc::cpu_set_t = zeroed();

    libc::CPU_ZERO(&mut mask);
    libc::CPU_SET(cpu as usize, &mut mask);
    if libc::sched_setaffinity(0, size_of::<libc::cpu_set_t>(), &mask) != 0 {
        libc::fprintf(libc::stderr, c"cpu: unable to pin, may increase variance.\n".as_ptr());
    } else if cfg_verbose != 0 {
        libc::fprintf(libc::stderr, c"cpu: %u\n".as_ptr(), cpu);
    }

    0
}

unsafe fn do_setsockopt(fd: c_int, level: c_int, optname: c_int, val: c_int) {
    if libc::setsockopt(fd, level, optname, &val as *const c_int as *const c_void, size_of::<c_int>() as socklen_t) != 0 {
        error(1, *libc::__errno_location(), c"setsockopt %d.%d: %d".as_ptr(), level, optname, val);
    }
}

unsafe fn do_poll(fd: c_int, events: c_int) -> c_int {
    let mut pfd: libc::pollfd = zeroed();
    let ret: c_int;

    pfd.events = events as c_short;
    pfd.revents = 0;
    pfd.fd = fd;

    ret = libc::poll(&mut pfd, 1, cfg_waittime_ms);
    if ret == -1 {
        error(1, *libc::__errno_location(), c"poll".as_ptr());
    }

    ((ret != 0) && ((pfd.revents & events as c_short) != 0)) as c_int
}

unsafe fn do_accept(mut fd: c_int) -> c_int {
    let fda: c_int = fd;

    fd = libc::accept(fda, null_mut(), null_mut());
    if fd == -1 {
        error(1, *libc::__errno_location(), c"accept".as_ptr());
    }
    if libc::close(fda) != 0 {
        error(1, *libc::__errno_location(), c"close listen sock".as_ptr());
    }

    fd
}

unsafe fn add_zcopy_cookie(msg: *mut libc::msghdr, cookie: u32) {
    let cm: *mut libc::cmsghdr;

    if (*msg).msg_control.is_null() {
        error(1, *libc::__errno_location(), c"NULL cookie".as_ptr());
    }
    cm = (*msg).msg_control as *mut libc::cmsghdr;
    (*cm).cmsg_len = CMSG_LEN(size_of::<u32>()) as _;
    (*cm).cmsg_level = SOL_RDS;
    (*cm).cmsg_type = RDS_CMSG_ZCOPY_COOKIE;
    libc::memcpy(CMSG_DATA(cm) as *mut c_void, &cookie as *const u32 as *const c_void, size_of::<u32>());
}

unsafe fn do_sendmsg(fd: c_int, msg: *mut libc::msghdr, do_zerocopy: bool, domain: c_int) -> bool {
    static mut cookie: u32 = 0;
    let mut ret: c_int;
    let mut len: c_int;
    let mut i: c_int;
    let mut flags: c_int;
    let mut ckbuf = [0u8; 64];

    len = 0;
    i = 0;
    while i < (*msg).msg_iovlen as c_int {
        len += (*(*msg).msg_iov.add(i as usize)).iov_len as c_int;
        i += 1;
    }

    flags = MSG_DONTWAIT;
    if do_zerocopy {
        flags |= MSG_ZEROCOPY;
        if domain == PF_RDS {
            libc::memset(&mut (*msg).msg_control as *mut *mut c_void as *mut c_void, 0, size_of::<*mut c_void>());
            (*msg).msg_controllen = CMSG_SPACE(size_of::<u32>()) as _;
            (*msg).msg_control = ckbuf.as_mut_ptr() as *mut c_void;
            cookie = cookie.wrapping_add(1);
            add_zcopy_cookie(msg, cookie);
        }
    }

    ret = libc::sendmsg(fd, msg, flags) as c_int;
    if ret == -1 && *libc::__errno_location() == EAGAIN {
        return false;
    }
    if ret == -1 {
        error(1, *libc::__errno_location(), c"send".as_ptr());
    }
    if cfg_verbose != 0 && ret != len {
        libc::fprintf(libc::stderr, c"send: ret=%u != %u\n".as_ptr(), ret, len);
    }
    sends_since_notify = sends_since_notify.wrapping_add(1);

    if len != 0 {
        packets += 1;
        bytes += ret as c_long;
        if do_zerocopy && ret != 0 {
            expected_completions += 1;
        }
    }
    if do_zerocopy && domain == PF_RDS {
        (*msg).msg_control = null_mut();
        (*msg).msg_controllen = 0;
    }

    true
}

unsafe fn do_sendmsg_corked(fd: c_int, msg: *mut libc::msghdr) {
    let mut do_zerocopy: bool = cfg_zerocopy;
    let mut i: c_int;
    let mut payload_len: c_int;
    let mut extra_len: c_int;

    /* split up the packet. for non-multiple, make first buffer longer */
    payload_len = cfg_payload_len / cfg_cork;
    extra_len = cfg_payload_len - (cfg_cork * payload_len);

    do_setsockopt(fd, IPPROTO_UDP, UDP_CORK, 1);

    i = 0;
    while i < cfg_cork {
        /* in mixed-frags mode, alternate zerocopy and copy frags
         * start with non-zerocopy, to ensure attach later works
         */
        if cfg_cork_mixed {
            do_zerocopy = (i & 1) != 0;
        }

        (*(*msg).msg_iov).iov_len = (payload_len + extra_len) as usize;
        extra_len = 0;

        do_sendmsg(
            fd,
            msg,
            do_zerocopy,
            if cfg_dst_addr.ss_family as c_int == AF_INET { PF_INET } else { PF_INET6 },
        );
        i += 1;
    }

    do_setsockopt(fd, IPPROTO_UDP, UDP_CORK, 0);
}

unsafe fn setup_iph(iph: *mut iphdr, payload_len: u16) -> c_int {
    let daddr = &mut *(&mut cfg_dst_addr as *mut libc::sockaddr_storage as *mut libc::sockaddr_in);
    let saddr = &mut *(&mut cfg_src_addr as *mut libc::sockaddr_storage as *mut libc::sockaddr_in);

    libc::memset(iph as *mut c_void, 0, size_of::<iphdr>());

    (*iph).set_version(4);
    (*iph).tos = 0;
    (*iph).set_ihl(5);
    (*iph).ttl = 2;
    (*iph).saddr = saddr.sin_addr.s_addr;
    (*iph).daddr = daddr.sin_addr.s_addr;
    (*iph).protocol = IPPROTO_EGP as u8;
    (*iph).tot_len = libc::htons((size_of::<iphdr>() + payload_len as usize) as u16);
    (*iph).check = get_ip_csum(iph as *const u16, ((*iph).ihl() as c_int) << 1);

    size_of::<iphdr>() as c_int
}

unsafe fn setup_ip6h(ip6h: *mut ipv6hdr, payload_len: u16) -> c_int {
    let daddr = &mut *(&mut cfg_dst_addr as *mut libc::sockaddr_storage as *mut libc::sockaddr_in6);
    let saddr = &mut *(&mut cfg_src_addr as *mut libc::sockaddr_storage as *mut libc::sockaddr_in6);

    libc::memset(ip6h as *mut c_void, 0, size_of::<ipv6hdr>());

    (*ip6h).set_version(6);
    (*ip6h).payload_len = libc::htons(payload_len);
    (*ip6h).nexthdr = IPPROTO_EGP as u8;
    (*ip6h).hop_limit = 2;
    (*ip6h).saddr = saddr.sin6_addr;
    (*ip6h).daddr = daddr.sin6_addr;

    size_of::<ipv6hdr>() as c_int
}

unsafe fn setup_sockaddr(domain: c_int, str_addr: *const c_char, sockaddr: *mut libc::sockaddr_storage) {
    let addr6 = sockaddr as *mut libc::sockaddr_in6;
    let addr4 = sockaddr as *mut libc::sockaddr_in;

    match domain {
        PF_INET => {
            libc::memset(addr4 as *mut c_void, 0, size_of::<libc::sockaddr_in>());
            (*addr4).sin_family = AF_INET as _;
            (*addr4).sin_port = libc::htons(cfg_port as u16);
            if !str_addr.is_null()
                && libc::inet_pton(AF_INET, str_addr, &mut (*addr4).sin_addr as *mut _ as *mut c_void) != 1
            {
                error(1, 0, c"ipv4 parse error: %s".as_ptr(), str_addr);
            }
        }
        PF_INET6 => {
            libc::memset(addr6 as *mut c_void, 0, size_of::<libc::sockaddr_in6>());
            (*addr6).sin6_family = AF_INET6 as _;
            (*addr6).sin6_port = libc::htons(cfg_port as u16);
            if !str_addr.is_null()
                && libc::inet_pton(AF_INET6, str_addr, &mut (*addr6).sin6_addr as *mut _ as *mut c_void) != 1
            {
                error(1, 0, c"ipv6 parse error: %s".as_ptr(), str_addr);
            }
        }
        _ => error(1, 0, c"illegal domain".as_ptr()),
    }
}

unsafe fn do_setup_tx(domain: c_int, type_: c_int, protocol: c_int) -> c_int {
    let fd: c_int;

    fd = libc::socket(domain, type_, protocol);
    if fd == -1 {
        error(1, *libc::__errno_location(), c"socket t".as_ptr());
    }

    do_setsockopt(fd, SOL_SOCKET, SO_SNDBUF, 1 << 21);
    if cfg_zerocopy {
        do_setsockopt(fd, SOL_SOCKET, SO_ZEROCOPY, 1);
    }

    if domain != PF_PACKET && domain != PF_RDS {
        if libc::connect(fd, &cfg_dst_addr as *const _ as *const libc::sockaddr, cfg_alen) != 0 {
            error(1, *libc::__errno_location(), c"connect".as_ptr());
        }
    }

    if domain == PF_RDS {
        if libc::bind(fd, &cfg_src_addr as *const _ as *const libc::sockaddr, cfg_alen) != 0 {
            error(1, *libc::__errno_location(), c"bind".as_ptr());
        }
    }

    fd
}

unsafe fn do_process_zerocopy_cookies(ck: *mut rds_zcopy_cookies) -> u32 {
    let mut i: c_int;

    if (*ck).num > RDS_MAX_ZCOOKIES as u32 {
        error(
            1,
            0,
            c"Returned %d cookies, max expected %d\n".as_ptr(),
            (*ck).num,
            RDS_MAX_ZCOOKIES,
        );
    }
    i = 0;
    while i < (*ck).num as c_int {
        if cfg_verbose >= 2 {
            libc::fprintf(libc::stderr, c"%d\n".as_ptr(), (*ck).cookies[i as usize]);
        }
        i += 1;
    }
    (*ck).num
}

unsafe fn do_recvmsg_completion(fd: c_int) -> bool {
    let mut cmsgbuf = [0u8; 128];
    let mut ck: *mut rds_zcopy_cookies;
    let mut cmsg: *mut libc::cmsghdr;
    let mut msg: libc::msghdr = zeroed();
    let mut ret: bool = false;

    libc::memset(&mut msg as *mut _ as *mut c_void, 0, size_of::<libc::msghdr>());
    msg.msg_control = cmsgbuf.as_mut_ptr() as *mut c_void;
    msg.msg_controllen = core::mem::size_of_val(&cmsgbuf) as _;

    if libc::recvmsg(fd, &mut msg, MSG_DONTWAIT) != 0 {
        return ret;
    }

    if (msg.msg_flags & MSG_CTRUNC) != 0 {
        error(1, *libc::__errno_location(), c"recvmsg notification: truncated".as_ptr());
    }

    cmsg = CMSG_FIRSTHDR(&msg);
    while !cmsg.is_null() {
        if (*cmsg).cmsg_level == SOL_RDS && (*cmsg).cmsg_type == RDS_CMSG_ZCOPY_COMPLETION {
            ck = CMSG_DATA(cmsg) as *mut rds_zcopy_cookies;
            completions += do_process_zerocopy_cookies(ck) as c_long;
            ret = true;
            break;
        }
        error(
            0,
            0,
            c"ignoring cmsg at level %d type %d\n".as_ptr(),
            (*cmsg).cmsg_level,
            (*cmsg).cmsg_type,
        );
        cmsg = CMSG_NXTHDR(&msg, cmsg);
    }
    ret
}

unsafe fn do_recv_completion(fd: c_int, domain: c_int) -> bool {
    let serr: *mut sock_extended_err;
    let mut msg: libc::msghdr = zeroed();
    let cm: *mut libc::cmsghdr;
    let mut hi: u32;
    let mut lo: u32;
    let range: u32;
    let ret: c_int;
    let zerocopy: c_int;
    let mut control = [0u8; 100];

    if domain == PF_RDS {
        return do_recvmsg_completion(fd);
    }

    msg.msg_control = control.as_mut_ptr() as *mut c_void;
    msg.msg_controllen = core::mem::size_of_val(&control) as _;

    ret = libc::recvmsg(fd, &mut msg, MSG_ERRQUEUE) as c_int;
    if ret == -1 && *libc::__errno_location() == EAGAIN {
        return false;
    }
    if ret == -1 {
        error(1, *libc::__errno_location(), c"recvmsg notification".as_ptr());
    }
    if (msg.msg_flags & MSG_CTRUNC) != 0 {
        error(1, *libc::__errno_location(), c"recvmsg notification: truncated".as_ptr());
    }

    cm = CMSG_FIRSTHDR(&msg);
    if cm.is_null() {
        error(1, 0, c"cmsg: no cmsg".as_ptr());
    }
    if !(((*cm).cmsg_level == SOL_IP && (*cm).cmsg_type == IP_RECVERR)
        || ((*cm).cmsg_level == SOL_IPV6 && (*cm).cmsg_type == IPV6_RECVERR)
        || ((*cm).cmsg_level == SOL_PACKET && (*cm).cmsg_type == PACKET_TX_TIMESTAMP))
    {
        error(1, 0, c"serr: wrong type: %d.%d".as_ptr(), (*cm).cmsg_level, (*cm).cmsg_type);
    }

    serr = CMSG_DATA(cm) as *mut sock_extended_err;

    if (*serr).ee_origin != SO_EE_ORIGIN_ZEROCOPY {
        error(1, 0, c"serr: wrong origin: %u".as_ptr(), (*serr).ee_origin as c_uint);
    }
    if (*serr).ee_errno != 0 {
        error(1, 0, c"serr: wrong error code: %u".as_ptr(), (*serr).ee_errno);
    }

    hi = (*serr).ee_data;
    lo = (*serr).ee_info;
    range = hi.wrapping_sub(lo).wrapping_add(1);

    /* Detect notification gaps. These should not happen often, if at all.
     * Gaps can occur due to drops, reordering and retransmissions.
     */
    if cfg_verbose != 0 && lo != next_completion {
        libc::fprintf(
            libc::stderr,
            c"gap: %u..%u does not append to %u\n".as_ptr(),
            lo,
            hi,
            next_completion,
        );
    }
    next_completion = hi.wrapping_add(1);

    zerocopy = (((*serr).ee_code & SO_EE_CODE_ZEROCOPY_COPIED) == 0) as c_int;
    if cfg_expect_zerocopy != -1 && cfg_expect_zerocopy != zerocopy {
        libc::fprintf(
            libc::stderr,
            c"serr: ee_code: %u != expected %u\n".as_ptr(),
            zerocopy,
            cfg_expect_zerocopy,
        );
        exitcode = 1;
        /* suppress repeated messages */
        cfg_expect_zerocopy = zerocopy;
    }

    if cfg_verbose >= 2 {
        libc::fprintf(libc::stderr, c"completed: %u (h=%u l=%u)\n".as_ptr(), range, hi, lo);
    }

    completions += range as c_long;
    true
}

/* Read all outstanding messages on the errqueue */
unsafe fn do_recv_completions(fd: c_int, domain: c_int) {
    while do_recv_completion(fd, domain) {}
    sends_since_notify = 0;
}

/* Wait for all remaining completions on the errqueue */
unsafe fn do_recv_remaining_completions(fd: c_int, domain: c_int) {
    let tstop: i64 = gettimeofday_ms() as i64 + cfg_waittime_ms as i64;

    while completions < expected_completions && (gettimeofday_ms() as i64) < tstop {
        if do_poll(fd, if domain == PF_RDS { POLLIN as c_int } else { POLLERR as c_int }) != 0 {
            do_recv_completions(fd, domain);
        }
    }

    if completions < expected_completions {
        libc::fprintf(
            libc::stderr,
            c"missing notifications: %lu < %lu\n".as_ptr(),
            completions as c_ulong,
            expected_completions as c_ulong,
        );
    }
}

unsafe fn do_tx(domain: c_int, type_: c_int, protocol: c_int) {
    let mut iov: [libc::iovec; 3] = zeroed();
    let mut laddr: libc::sockaddr_ll = zeroed();
    let mut msg: libc::msghdr = zeroed();
    let mut eth: ethhdr = zeroed();
    let mut nh: nh_union = zeroed();
    let tstop: u64;
    let fd: c_int;

    fd = do_setup_tx(domain, type_, protocol);

    if domain == PF_PACKET {
        let proto: u16 = if cfg_family == PF_INET { ETH_P_IP } else { ETH_P_IPV6 };

        /* sock_raw passes ll header as data */
        if type_ == SOCK_RAW {
            libc::memset(eth.h_dest.as_mut_ptr() as *mut c_void, 0x06, ETH_ALEN);
            libc::memset(eth.h_source.as_mut_ptr() as *mut c_void, 0x02, ETH_ALEN);
            eth.h_proto = libc::htons(proto);
            iov[0].iov_base = &mut eth as *mut _ as *mut c_void;
            iov[0].iov_len = size_of::<ethhdr>();
            msg.msg_iovlen += 1;
        }

        /* both sock_raw and sock_dgram expect name */
        libc::memset(&mut laddr as *mut _ as *mut c_void, 0, size_of::<libc::sockaddr_ll>());
        laddr.sll_family = AF_PACKET as _;
        laddr.sll_ifindex = cfg_ifindex;
        laddr.sll_protocol = libc::htons(proto);
        laddr.sll_halen = ETH_ALEN as _;

        libc::memset(laddr.sll_addr.as_mut_ptr() as *mut c_void, 0x06, ETH_ALEN);

        msg.msg_name = &mut laddr as *mut _ as *mut c_void;
        msg.msg_namelen = size_of::<libc::sockaddr_ll>() as _;
    }

    /* packet and raw sockets with hdrincl must pass network header */
    if domain == PF_PACKET || protocol == IPPROTO_RAW {
        if cfg_family == PF_INET {
            iov[1].iov_len = setup_iph((&mut nh.iph as *mut _ as *mut iphdr), cfg_payload_len as u16) as usize;
        } else {
            iov[1].iov_len = setup_ip6h((&mut nh.ip6h as *mut _ as *mut ipv6hdr), cfg_payload_len as u16) as usize;
        }

        iov[1].iov_base = &mut nh as *mut _ as *mut c_void;
        msg.msg_iovlen += 1;
    }

    if domain == PF_RDS {
        msg.msg_name = &mut cfg_dst_addr as *mut _ as *mut c_void;
        msg.msg_namelen = if cfg_dst_addr.ss_family as c_int == AF_INET {
            size_of::<libc::sockaddr_in>()
        } else {
            size_of::<libc::sockaddr_in6>()
        } as _;
    }

    iov[2].iov_base = payload.as_mut_ptr() as *mut c_void;
    iov[2].iov_len = cfg_payload_len as usize;
    msg.msg_iovlen += 1;
    msg.msg_iov = iov.as_mut_ptr().add(3 - msg.msg_iovlen);

    tstop = gettimeofday_ms() as u64 + cfg_runtime_ms as u64;
    loop {
        if cfg_cork != 0 {
            do_sendmsg_corked(fd, &mut msg);
        } else {
            do_sendmsg(fd, &mut msg, cfg_zerocopy, domain);
        }

        if cfg_zerocopy && sends_since_notify >= cfg_notification_limit as u32 {
            do_recv_completions(fd, domain);
        }

        while do_poll(fd, POLLOUT as c_int) == 0 {
            if cfg_zerocopy {
                do_recv_completions(fd, domain);
            }
        }

        if gettimeofday_ms() as u64 >= tstop {
            break;
        }
    }

    if cfg_zerocopy {
        do_recv_remaining_completions(fd, domain);
    }

    if libc::close(fd) != 0 {
        error(1, *libc::__errno_location(), c"close".as_ptr());
    }

    libc::fprintf(
        libc::stderr,
        c"tx=%lu (%lu MB) txc=%lu zc=%c\n".as_ptr(),
        packets as c_ulong,
        (bytes >> 20) as c_ulong,
        completions as c_ulong,
        if cfg_zerocopy && cfg_expect_zerocopy == 1 { b'y' as c_int } else { b'n' as c_int },
    );
}

unsafe fn do_setup_rx(domain: c_int, type_: c_int, protocol: c_int) -> c_int {
    let mut fd: c_int;

    /* If tx over PF_PACKET, rx over PF_INET(6)/SOCK_RAW,
     * to recv the only copy of the packet, not a clone
     */
    if domain == PF_PACKET {
        error(1, 0, c"Use PF_INET/SOCK_RAW to read".as_ptr());
    }

    if type_ == SOCK_RAW && protocol == IPPROTO_RAW {
        error(1, 0, c"IPPROTO_RAW: not supported on Rx".as_ptr());
    }

    fd = libc::socket(domain, type_, protocol);
    if fd == -1 {
        error(1, *libc::__errno_location(), c"socket r".as_ptr());
    }

    do_setsockopt(fd, SOL_SOCKET, SO_RCVBUF, 1 << 21);
    do_setsockopt(fd, SOL_SOCKET, SO_RCVLOWAT, 1 << 16);
    do_setsockopt(fd, SOL_SOCKET, SO_REUSEPORT, 1);

    if libc::bind(fd, &cfg_dst_addr as *const _ as *const libc::sockaddr, cfg_alen) != 0 {
        error(1, *libc::__errno_location(), c"bind".as_ptr());
    }

    if type_ == SOCK_STREAM {
        if libc::listen(fd, 1) != 0 {
            error(1, *libc::__errno_location(), c"listen".as_ptr());
        }
        fd = do_accept(fd);
    }

    fd
}

/* Flush all outstanding bytes for the tcp receive queue */
unsafe fn do_flush_tcp(fd: c_int) {
    let ret: c_int;

    /* MSG_TRUNC flushes up to len bytes */
    ret = libc::recv(fd, null_mut(), 1 << 21, MSG_TRUNC | MSG_DONTWAIT) as c_int;
    if ret == -1 && *libc::__errno_location() == EAGAIN {
        return;
    }
    if ret == -1 {
        error(1, *libc::__errno_location(), c"flush".as_ptr());
    }
    if ret == 0 {
        return;
    }

    packets += 1;
    bytes += ret as c_long;
}

/* Flush all outstanding datagrams. Verify first few bytes of each. */
unsafe fn do_flush_datagram(fd: c_int, type_: c_int) {
    let mut ret: c_int;
    let mut off: c_int = 0;
    let mut buf = [0 as c_char; 64];

    /* MSG_TRUNC will return full datagram length */
    ret = libc::recv(
        fd,
        buf.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&buf),
        MSG_DONTWAIT | MSG_TRUNC,
    ) as c_int;
    if ret == -1 && *libc::__errno_location() == EAGAIN {
        return;
    }

    /* raw ipv4 return with header, raw ipv6 without */
    if cfg_family == PF_INET && type_ == SOCK_RAW {
        off += size_of::<iphdr>() as c_int;
        ret -= size_of::<iphdr>() as c_int;
    }

    if ret == -1 {
        error(1, *libc::__errno_location(), c"recv".as_ptr());
    }
    if ret != cfg_payload_len {
        error(1, 0, c"recv: ret=%u != %u".as_ptr(), ret, cfg_payload_len);
    }
    if ret > (core::mem::size_of_val(&buf) as c_int) - off {
        ret = (core::mem::size_of_val(&buf) as c_int) - off;
    }
    if libc::memcmp(
        buf.as_ptr().add(off as usize) as *const c_void,
        payload.as_ptr() as *const c_void,
        ret as usize,
    ) != 0
    {
        error(1, 0, c"recv: data mismatch".as_ptr());
    }

    packets += 1;
    bytes += cfg_payload_len as c_long;
}

unsafe fn do_rx(domain: c_int, type_: c_int, protocol: c_int) {
    const cfg_receiver_wait_ms: c_int = 400;
    let tstop: u64;
    let fd: c_int;

    fd = do_setup_rx(domain, type_, protocol);

    tstop = gettimeofday_ms() as u64 + cfg_runtime_ms as u64 + cfg_receiver_wait_ms as u64;
    loop {
        if type_ == SOCK_STREAM {
            do_flush_tcp(fd);
        } else {
            do_flush_datagram(fd, type_);
        }

        do_poll(fd, POLLIN as c_int);

        if gettimeofday_ms() as u64 >= tstop {
            break;
        }
    }

    if libc::close(fd) != 0 {
        error(1, *libc::__errno_location(), c"close".as_ptr());
    }

    libc::fprintf(
        libc::stderr,
        c"rx=%lu (%lu MB)\n".as_ptr(),
        packets as c_ulong,
        (bytes >> 20) as c_ulong,
    );
}

unsafe fn do_test(domain: c_int, type_: c_int, protocol: c_int) {
    let mut i: c_int;

    if cfg_cork != 0 && (domain == PF_PACKET || type_ != SOCK_DGRAM) {
        error(1, 0, c"can only cork udp sockets".as_ptr());
    }

    do_setcpu(cfg_cpu);

    i = 0;
    while i < IP_MAXPACKET as c_int {
        payload[i as usize] = (b'a' + (i % 26) as u8) as c_char;
        i += 1;
    }

    if cfg_rx {
        do_rx(domain, type_, protocol);
    } else {
        do_tx(domain, type_, protocol);
    }
}

unsafe fn usage(filepath: *const c_char) {
    error(1, 0, c"Usage: %s [options] <test>".as_ptr(), filepath);
}

unsafe fn parse_opts(argc: c_int, argv: *mut *mut c_char) {
    let max_payload_len: c_int = (core::mem::size_of_val(&payload)
        - size_of::<ipv6hdr>()
        - size_of::<libc::tcphdr>()
        - 40 /* max tcp options */) as c_int;
    let mut c: c_int;
    let mut daddr: *mut c_char = null_mut();
    let mut saddr: *mut c_char = null_mut();
    let cfg_test: *mut c_char;

    cfg_payload_len = max_payload_len;

    loop {
        c = libc::getopt(argc, argv, c"46c:C:D:i:l:mp:rs:S:t:vzZ:".as_ptr());
        if c == -1 {
            break;
        }
        match c as u8 as char {
            '4' => {
                if cfg_family != PF_UNSPEC {
                    error(1, 0, c"Pass one of -4 or -6".as_ptr());
                }
                cfg_family = PF_INET;
                cfg_alen = size_of::<libc::sockaddr_in>() as _;
            }
            '6' => {
                if cfg_family != PF_UNSPEC {
                    error(1, 0, c"Pass one of -4 or -6".as_ptr());
                }
                cfg_family = PF_INET6;
                cfg_alen = size_of::<libc::sockaddr_in6>() as _;
            }
            'c' => cfg_cork = libc::strtol(libc::optarg, null_mut(), 0) as c_int,
            'C' => cfg_cpu = libc::strtol(libc::optarg, null_mut(), 0) as c_int,
            'D' => daddr = libc::optarg,
            'i' => {
                cfg_ifindex = libc::if_nametoindex(libc::optarg) as c_int;
                if cfg_ifindex == 0 {
                    error(1, *libc::__errno_location(), c"invalid iface: %s".as_ptr(), libc::optarg);
                }
            }
            'l' => cfg_notification_limit = libc::strtoul(libc::optarg, null_mut(), 0) as c_int,
            'm' => cfg_cork_mixed = true,
            'p' => cfg_port = libc::strtoul(libc::optarg, null_mut(), 0) as c_int,
            'r' => cfg_rx = true,
            's' => cfg_payload_len = libc::strtoul(libc::optarg, null_mut(), 0) as c_int,
            'S' => saddr = libc::optarg,
            't' => cfg_runtime_ms = 200 + libc::strtoul(libc::optarg, null_mut(), 10) as c_int * 1000,
            'v' => cfg_verbose += 1,
            'z' => cfg_zerocopy = true,
            'Z' => cfg_expect_zerocopy = (libc::atoi(libc::optarg) != 0) as c_int,
            _ => {}
        }
    }

    cfg_test = *argv.add((argc - 1) as usize);
    if libc::strcmp(cfg_test, c"rds".as_ptr()) == 0 {
        if daddr.is_null() {
            error(1, 0, c"-D <server addr> required for PF_RDS\n".as_ptr());
        }
        if !cfg_rx && saddr.is_null() {
            error(1, 0, c"-S <client addr> required for PF_RDS\n".as_ptr());
        }
    }
    setup_sockaddr(cfg_family, daddr, &mut cfg_dst_addr);
    setup_sockaddr(cfg_family, saddr, &mut cfg_src_addr);

    if cfg_payload_len > max_payload_len {
        error(1, 0, c"-s: payload exceeds max (%d)".as_ptr(), max_payload_len);
    }
    if cfg_cork_mixed && (!cfg_zerocopy || cfg_cork == 0) {
        error(1, 0, c"-m: cork_mixed requires corking and zerocopy".as_ptr());
    }

    if libc::optind != argc - 1 {
        usage(*argv);
    }
}

unsafe fn cstr_eq(a: *const c_char, b: *const c_char) -> bool {
    libc::strcmp(a, b) == 0
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let cfg_test: *const c_char;

    parse_opts(argc, argv);

    cfg_test = *argv.add((argc - 1) as usize);

    if cstr_eq(cfg_test, c"packet".as_ptr()) {
        do_test(PF_PACKET, SOCK_RAW, 0);
    } else if cstr_eq(cfg_test, c"packet_dgram".as_ptr()) {
        do_test(PF_PACKET, SOCK_DGRAM, 0);
    } else if cstr_eq(cfg_test, c"raw".as_ptr()) {
        do_test(cfg_family, SOCK_RAW, IPPROTO_EGP);
    } else if cstr_eq(cfg_test, c"raw_hdrincl".as_ptr()) {
        do_test(cfg_family, SOCK_RAW, IPPROTO_RAW);
    } else if cstr_eq(cfg_test, c"tcp".as_ptr()) {
        do_test(cfg_family, SOCK_STREAM, 0);
    } else if cstr_eq(cfg_test, c"udp".as_ptr()) {
        do_test(cfg_family, SOCK_DGRAM, 0);
    } else if cstr_eq(cfg_test, c"rds".as_ptr()) {
        do_test(PF_RDS, SOCK_SEQPACKET, 0);
    } else {
        error(1, 0, c"unknown cfg_test %s".as_ptr(), cfg_test);
    }

    exitcode
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
