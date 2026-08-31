// SPDX-License-Identifier: GPL-2.0
/*
 * This testsuite provides conformance testing for GRO coalescing.
 *
 * Rust source-level translation of gro.c. C include dependencies are represented
 * as libc-compatible declarations and constants expected from the surrounding
 * repository/build.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type __be16 = u16;
type __u8 = u8;
type socklen_t = u32;
type size_t = usize;
type ssize_t = isize;
type clockid_t = c_int;

const DPORT: c_int = 8000;
const SPORT: c_int = 1500;
const PAYLOAD_LEN: c_int = 100;
const NUM_PACKETS: usize = 4;
const START_SEQ: c_int = 100;
const START_ACK: c_int = 100;
const ETH_P_NONE: c_int = 0;
const ASSUMED_MTU: c_int = 4096;
const MIN_EXTHDR_SIZE: usize = 8;
const EXIT_OVER_COALESCE: c_int = 42;
const CAPACITY_PAYLOAD_LEN: c_int = 200;
const TXTIME_DELAY_MS: u64 = 5;

const ETH_ALEN: usize = 6;
const ETH_HLEN: usize = 14;
const ETH_ZLEN: c_int = 60;
const IP_MAXPACKET: usize = 65535;
const PPPOE_SES_HLEN: usize = 8;
const PPP_IP: c_int = 0x0021;
const PPP_IPV6: c_int = 0x0057;
const TCP_MAXWIN: c_int = 65535;
const TCPOPT_NOP: c_int = 1;
const TCPOPT_WINDOW: c_int = 3;
const TCPOPT_TIMESTAMP: c_int = 8;
const TCPOLEN_WINDOW: c_int = 3;
const TCPOLEN_TIMESTAMP: c_int = 10;
const TCPOLEN_TSTAMP_APPA: c_int = 12;
const TCPOLEN_MAXSEG: c_int = 4;
const IPOPT_TS: c_int = 68;
const IPOPT_TS_TSONLY: c_int = 0;
const IP_DF: c_int = 0x4000;
const TH_CWR: u8 = 0x80;

const AF_PACKET: c_int = 17;
const PF_PACKET: c_int = AF_PACKET;
const AF_INET: c_int = 2;
const PF_INET: c_int = AF_INET;
const AF_INET6: c_int = 10;
const PF_INET6: c_int = AF_INET6;
const SOCK_RAW: c_int = 3;
const IPPROTO_RAW: c_int = 255;
const IPPROTO_TCP: c_int = 6;
const IPPROTO_IPIP: c_int = 4;
const IPPROTO_IPV6: c_int = 41;
const IPPROTO_FRAGMENT: c_int = 44;
const IPPROTO_DSTOPTS: c_int = 60;
const ETH_P_IP: c_int = 0x0800;
const ETH_P_IPV6: c_int = 0x86DD;
const ETH_P_PPP_SES: c_int = 0x8864;
const SOL_SOCKET: c_int = 1;
const SOL_PACKET: c_int = 263;
const SO_ATTACH_FILTER: c_int = 26;
const SO_RCVTIMEO: c_int = 20;
const SO_RCVBUF: c_int = 8;
const SO_SNDBUF: c_int = 7;
const SO_TXTIME: c_int = 61;
const SCM_TXTIME: c_int = SO_TXTIME;
const PACKET_STATISTICS: c_int = 6;
const CLOCK_MONOTONIC: clockid_t = 1;

const BPF_LD: u16 = 0x00;
const BPF_H: u16 = 0x08;
const BPF_B: u16 = 0x10;
const BPF_ABS: u16 = 0x20;
const BPF_JMP: u16 = 0x05;
const BPF_JEQ: u16 = 0x10;
const BPF_K: u16 = 0x00;
const BPF_RET: u16 = 0x06;

const MAX_MSS: usize = ASSUMED_MTU as usize - size_of::<iphdr>() - size_of::<tcphdr>();
const MAX_HDR_LEN: usize = ETH_HLEN + size_of::<ipv6hdr>() * 2 + size_of::<tcphdr>();
const MAX_LARGE_PKT_CNT: usize =
    (IP_MAXPACKET - (MAX_HDR_LEN - ETH_HLEN)) / (ASSUMED_MTU as usize - (MAX_HDR_LEN - ETH_HLEN));
const L2_HLEN_MAX: usize = ETH_HLEN + PPPOE_SES_HLEN;

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
struct ethhdr {
    h_dest: [u8; ETH_ALEN],
    h_source: [u8; ETH_ALEN],
    h_proto: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct iphdr {
    ihl_version: u8,
    tos: u8,
    tot_len: __be16,
    id: __be16,
    frag_off: __be16,
    ttl: u8,
    protocol: u8,
    check: u16,
    saddr: u32,
    daddr: u32,
}
impl iphdr {
    unsafe fn set_ihl(&mut self, v: u8) { self.ihl_version = (self.ihl_version & 0xf0) | (v & 0x0f); }
    unsafe fn ihl(&self) -> u8 { self.ihl_version & 0x0f }
    unsafe fn set_version(&mut self, v: u8) { self.ihl_version = (self.ihl_version & 0x0f) | ((v & 0x0f) << 4); }
    unsafe fn version(&self) -> u8 { self.ihl_version >> 4 }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ipv6hdr {
    priority_version: u8,
    flow_lbl: [u8; 3],
    payload_len: __be16,
    nexthdr: u8,
    hop_limit: u8,
    saddr: in6_addr,
    daddr: in6_addr,
}
impl ipv6hdr {
    unsafe fn set_version(&mut self, v: u8) { self.priority_version = (self.priority_version & 0x0f) | ((v & 0x0f) << 4); }
    unsafe fn version(&self) -> u8 { self.priority_version >> 4 }
    unsafe fn set_priority(&mut self, v: u8) { self.priority_version = (self.priority_version & 0xf0) | (v & 0x0f); }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct tcphdr {
    source: __be16,
    dest: __be16,
    seq: u32,
    ack_seq: u32,
    doff_res1: u8,
    th_flags: u8,
    window: __be16,
    check: u16,
    urg_ptr: __be16,
}
impl tcphdr {
    unsafe fn set_doff(&mut self, v: u8) { self.doff_res1 = (self.doff_res1 & 0x0f) | ((v & 0x0f) << 4); }
    unsafe fn doff(&self) -> u8 { self.doff_res1 >> 4 }
    unsafe fn set_ack(&mut self, v: c_int) { if v != 0 { self.th_flags |= 0x10 } else { self.th_flags &= !0x10 } }
    unsafe fn set_fin(&mut self, v: c_int) { if v != 0 { self.th_flags |= 0x01 } else { self.th_flags &= !0x01 } }
    unsafe fn fin(&self) -> bool { self.th_flags & 0x01 != 0 }
    unsafe fn set_syn(&mut self, v: c_int) { if v != 0 { self.th_flags |= 0x02 } else { self.th_flags &= !0x02 } }
    unsafe fn set_rst(&mut self, v: c_int) { if v != 0 { self.th_flags |= 0x04 } else { self.th_flags &= !0x04 } }
    unsafe fn set_psh(&mut self, v: c_int) { if v != 0 { self.th_flags |= 0x08 } else { self.th_flags &= !0x08 } }
    unsafe fn set_urg(&mut self, v: c_int) { if v != 0 { self.th_flags |= 0x20 } else { self.th_flags &= !0x20 } }
}

#[repr(C)]
struct pppoe_hdr {
    ver_type: u8,
    code: u8,
    sid: __be16,
    length: __be16,
}
impl pppoe_hdr {
    unsafe fn set_type(&mut self, v: u8) { self.ver_type = (self.ver_type & 0xf0) | (v & 0x0f); }
    unsafe fn set_ver(&mut self, v: u8) { self.ver_type = (self.ver_type & 0x0f) | ((v & 0x0f) << 4); }
}

#[repr(C)]
struct ip_timestamp {
    ipt_code: u8,
    ipt_len: u8,
    ipt_ptr: u8,
    ipt_flg: u8,
    data: [u8; 36],
}

#[repr(C)]
struct ip6_ext {
    ip6e_nxt: u8,
    ip6e_len: u8,
}
#[repr(C)]
struct ip6_hbh { ip6h_nxt: u8, ip6h_len: u8 }
#[repr(C)]
struct ip6_dest { ip6d_nxt: u8, ip6d_len: u8 }
#[repr(C)]
struct ip6_frag {
    ip6f_nxt: u8,
    ip6f_reserved: u8,
    ip6f_offlg: __be16,
    ip6f_ident: u32,
}
type ipv6_opt_hdr = ip6_ext;

#[repr(C)]
struct sockaddr_ll {
    sll_family: u16,
    sll_protocol: __be16,
    sll_ifindex: c_int,
    sll_hatype: u16,
    sll_pkttype: u8,
    sll_halen: u8,
    sll_addr: [u8; 8],
}

#[repr(C)]
struct timeval { tv_sec: c_long, tv_usec: c_long }
#[repr(C)]
struct timespec { tv_sec: c_long, tv_nsec: c_long }
#[repr(C)]
struct iovec { iov_base: *mut c_void, iov_len: size_t }
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
struct cmsghdr { cmsg_len: size_t, cmsg_level: c_int, cmsg_type: c_int }
#[repr(C)]
struct sock_filter { code: u16, jt: u8, jf: u8, k: u32 }
#[repr(C)]
struct sock_fprog { len: u16, filter: *mut sock_filter }
#[repr(C)]
struct tpacket_stats { tp_packets: c_uint, tp_drops: c_uint }
#[repr(C)]
struct sock_txtime { clockid: clockid_t, flags: u32 }
#[repr(C)]
struct option { name: *const c_char, has_arg: c_int, flag: *mut c_int, val: c_int }

const no_argument: c_int = 0;
const required_argument: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
enum flush_id_case {
    FLUSH_ID_DF1_INC,
    FLUSH_ID_DF1_FIXED,
    FLUSH_ID_DF0_INC,
    FLUSH_ID_DF0_FIXED,
    FLUSH_ID_DF1_INC_FIXED,
    FLUSH_ID_DF1_FIXED_INC,
}
use flush_id_case::*;

unsafe extern "C" {
    static mut errno: c_int;
    static mut optarg: *mut c_char;
    fn vfprintf(stream: *mut c_void, fmt: *const c_char, ap: VaList) -> c_int;
    static mut stderr: *mut c_void;
    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn error(status: c_int, errnum: c_int, fmt: *const c_char, ...) -> !;
    fn sscanf(s: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memmove(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn htons(hostshort: u16) -> u16;
    fn htonl(hostlong: u32) -> u32;
    fn ntohs(netshort: u16) -> u16;
    fn ntohl(netlong: u32) -> u32;
    fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn setsockopt(fd: c_int, level: c_int, optname: c_int, optval: *const c_void, optlen: socklen_t) -> c_int;
    fn getsockopt(fd: c_int, level: c_int, optname: c_int, optval: *mut c_void, optlen: *mut socklen_t) -> c_int;
    fn bind(fd: c_int, addr: *const c_void, len: socklen_t) -> c_int;
    fn sendmsg(fd: c_int, msg: *const msghdr, flags: c_int) -> ssize_t;
    fn recv(fd: c_int, buf: *mut c_void, len: size_t, flags: c_int) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn if_nametoindex(ifname: *const c_char) -> c_uint;
    fn sleep(seconds: c_uint) -> c_uint;
    fn usleep(usec: c_uint) -> c_int;
    fn clock_gettime(clk_id: clockid_t, tp: *mut timespec) -> c_int;
    fn getopt_long(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char, longopts: *const option, longindex: *mut c_int) -> c_int;
    fn atoi(nptr: *const c_char) -> c_int;
    fn ksft_ready();
}

type VaList = *mut c_void;

static mut addr6_src: *const c_char = c"fdaa::2".as_ptr();
static mut addr6_dst: *const c_char = c"fdaa::1".as_ptr();
static mut addr4_src: *const c_char = c"192.168.1.200".as_ptr();
static mut addr4_dst: *const c_char = c"192.168.1.100".as_ptr();
static mut proto: c_int = -1;
static mut src_mac: [u8; ETH_ALEN] = [0; ETH_ALEN];
static mut dst_mac: [u8; ETH_ALEN] = [0; ETH_ALEN];
static mut testname: *mut c_char = c"data".as_ptr() as *mut c_char;
static mut ifname: *mut c_char = c"eth0".as_ptr() as *mut c_char;
static mut smac: *mut c_char = c"aa:00:00:00:00:02".as_ptr() as *mut c_char;
static mut dmac: *mut c_char = c"aa:00:00:00:00:01".as_ptr() as *mut c_char;
static mut verbose: bool = false;
static mut tx_socket: bool = true;
static mut tcp_offset: c_int = -1;
static mut total_hdr_len: c_int = -1;
static mut ethhdr_proto: c_int = -1;
static mut ipip: bool = false;
static mut ip6ip6: bool = false;
static mut pppoe: bool = false;
static mut txtime_ns: u64 = 0;
static mut num_flows: c_int = 4;
static mut order_check: bool = false;

unsafe fn cmsg_align(len: usize) -> usize { (len + size_of::<usize>() - 1) & !(size_of::<usize>() - 1) }
unsafe fn cmsg_space(len: usize) -> usize { cmsg_align(size_of::<cmsghdr>()) + cmsg_align(len) }
unsafe fn cmsg_len(len: usize) -> usize { cmsg_align(size_of::<cmsghdr>()) + len }
unsafe fn cmsg_firsthdr(msg: *mut msghdr) -> *mut cmsghdr { (*msg).msg_control as *mut cmsghdr }
unsafe fn cmsg_data(cm: *mut cmsghdr) -> *mut u8 { (cm as *mut u8).add(cmsg_align(size_of::<cmsghdr>())) }

macro_rules! BPF_STMT {
    ($code:expr, $k:expr) => { sock_filter { code: $code as u16, jt: 0, jf: 0, k: $k as u32 } };
}
macro_rules! BPF_JUMP {
    ($code:expr, $k:expr, $jt:expr, $jf:expr) => { sock_filter { code: $code as u16, jt: $jt as u8, jf: $jf as u8, k: $k as u32 } };
}
macro_rules! s {
    ($lit:literal) => { concat!($lit, "\0").as_ptr() as *const c_char };
}
unsafe fn streq(p: *const c_char, lit: &'static [u8]) -> bool { strcmp(p, lit.as_ptr() as *const c_char) == 0 }

unsafe fn max_payload() -> c_int { IP_MAXPACKET as c_int - (total_hdr_len - ETH_HLEN as c_int) }
unsafe fn calc_mss() -> c_int { ASSUMED_MTU - (total_hdr_len - ETH_HLEN as c_int) }
unsafe fn num_large_pkt() -> c_int { max_payload() / calc_mss() }

unsafe fn vlog(_fmt: *const c_char) {}

unsafe fn checksum_nofold(data: *mut c_void, len: size_t, mut sum: u32) -> u32 {
    let words = data as *mut u16;
    let mut i = 0usize;
    while i < len / 2 {
        sum = sum.wrapping_add(*words.add(i) as u32);
        i += 1;
    }
    if len & 1 != 0 {
        sum = sum.wrapping_add(*(data as *mut i8).add(len - 1) as u32);
    }
    sum
}

unsafe fn checksum_fold(data: *mut c_void, len: size_t, mut sum: u32) -> u16 {
    sum = checksum_nofold(data, len, sum);
    while sum > 0xffff {
        sum = (sum & 0xffff) + (sum >> 16);
    }
    !(sum as u16)
}

unsafe fn tcp_checksum(buf: *mut c_void, payload_len: c_int) -> u16 {
    #[repr(C)] struct pseudo_header6 { saddr: in6_addr, daddr: in6_addr, protocol: u16, payload_len: u16 }
    #[repr(C)] struct pseudo_header4 { saddr: in_addr, daddr: in_addr, protocol: u16, payload_len: u16 }
    let mut sum = 0u32;
    if proto == PF_INET6 {
        let mut ph6: pseudo_header6 = zeroed();
        if inet_pton(AF_INET6, addr6_src, &mut ph6.saddr as *mut _ as *mut c_void) != 1 { error(1, errno, s!("inet_pton6 source ip pseudo")); }
        if inet_pton(AF_INET6, addr6_dst, &mut ph6.daddr as *mut _ as *mut c_void) != 1 { error(1, errno, s!("inet_pton6 dest ip pseudo")); }
        ph6.protocol = htons(IPPROTO_TCP as u16);
        ph6.payload_len = htons((size_of::<tcphdr>() as c_int + payload_len) as u16);
        sum = checksum_nofold(&mut ph6 as *mut _ as *mut c_void, size_of::<pseudo_header6>(), 0);
    } else if proto == PF_INET {
        let mut ph4: pseudo_header4 = zeroed();
        if inet_pton(AF_INET, addr4_src, &mut ph4.saddr as *mut _ as *mut c_void) != 1 { error(1, errno, s!("inet_pton source ip pseudo")); }
        if inet_pton(AF_INET, addr4_dst, &mut ph4.daddr as *mut _ as *mut c_void) != 1 { error(1, errno, s!("inet_pton dest ip pseudo")); }
        ph4.protocol = htons(IPPROTO_TCP as u16);
        ph4.payload_len = htons((size_of::<tcphdr>() as c_int + payload_len) as u16);
        sum = checksum_nofold(&mut ph4 as *mut _ as *mut c_void, size_of::<pseudo_header4>(), 0);
    }
    checksum_fold(buf, size_of::<tcphdr>() + payload_len as usize, sum)
}

unsafe fn read_MAC(mac_addr: *mut u8, mac: *mut c_char) {
    if sscanf(mac, s!("%hhx:%hhx:%hhx:%hhx:%hhx:%hhx"), mac_addr.add(0), mac_addr.add(1), mac_addr.add(2), mac_addr.add(3), mac_addr.add(4), mac_addr.add(5)) != 6 {
        error(1, 0, s!("sscanf"));
    }
}

unsafe fn fill_datalinklayer(buf: *mut c_void) {
    let eth = buf as *mut ethhdr;
    memcpy((*eth).h_dest.as_mut_ptr() as *mut c_void, dst_mac.as_ptr() as *const c_void, ETH_ALEN);
    memcpy((*eth).h_source.as_mut_ptr() as *mut c_void, src_mac.as_ptr() as *const c_void, ETH_ALEN);
    (*eth).h_proto = ethhdr_proto as u16;
}

unsafe fn fill_networklayer(buf: *mut c_void, payload_len: c_int, protocol: c_int) {
    let ip6h = buf as *mut ipv6hdr;
    let iph = buf as *mut iphdr;
    if proto == PF_INET6 {
        memset(ip6h as *mut c_void, 0, size_of::<ipv6hdr>());
        (*ip6h).set_version(6);
        (*ip6h).payload_len = htons((size_of::<tcphdr>() as c_int + payload_len) as u16);
        (*ip6h).nexthdr = protocol as u8;
        (*ip6h).hop_limit = 8;
        if inet_pton(AF_INET6, addr6_src, &mut (*ip6h).saddr as *mut _ as *mut c_void) != 1 { error(1, errno, s!("inet_pton source ip6")); }
        if inet_pton(AF_INET6, addr6_dst, &mut (*ip6h).daddr as *mut _ as *mut c_void) != 1 { error(1, errno, s!("inet_pton dest ip6")); }
    } else if proto == PF_INET {
        memset(iph as *mut c_void, 0, size_of::<iphdr>());
        (*iph).set_version(4);
        (*iph).set_ihl(5);
        (*iph).ttl = 8;
        (*iph).protocol = protocol as u8;
        (*iph).tot_len = htons((size_of::<tcphdr>() + payload_len as usize + size_of::<iphdr>()) as u16);
        (*iph).frag_off = htons(0x4000);
        if inet_pton(AF_INET, addr4_src, &mut (*iph).saddr as *mut _ as *mut c_void) != 1 { error(1, errno, s!("inet_pton source ip")); }
        if inet_pton(AF_INET, addr4_dst, &mut (*iph).daddr as *mut _ as *mut c_void) != 1 { error(1, errno, s!("inet_pton dest ip")); }
        (*iph).check = checksum_fold(buf, size_of::<iphdr>(), 0);
    }
}

unsafe fn fill_transportlayer(buf: *mut c_void, seq_offset: c_int, ack_offset: c_int, payload_len: c_int, fin: c_int) {
    let tcph = buf as *mut tcphdr;
    memset(tcph as *mut c_void, 0, size_of::<tcphdr>());
    (*tcph).source = htons(SPORT as u16);
    (*tcph).dest = htons(DPORT as u16);
    (*tcph).seq = ntohl((START_SEQ + seq_offset) as u32);
    (*tcph).ack_seq = ntohl((START_ACK + ack_offset) as u32);
    (*tcph).set_ack(1);
    (*tcph).set_fin(fin);
    (*tcph).set_doff(5);
    (*tcph).window = htons(TCP_MAXWIN as u16);
    (*tcph).urg_ptr = 0;
    (*tcph).check = tcp_checksum(tcph as *mut c_void, payload_len);
}

unsafe fn write_packet(fd: c_int, buf: *mut c_char, len: c_int, daddr: *mut sockaddr_ll) {
    let mut control = vec![0u8; cmsg_space(size_of::<u64>())];
    let mut msg: msghdr = zeroed();
    let mut iov: iovec = zeroed();
    iov.iov_base = buf as *mut c_void;
    iov.iov_len = len as usize;
    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1;
    msg.msg_name = daddr as *mut c_void;
    msg.msg_namelen = size_of::<sockaddr_ll>() as socklen_t;
    if txtime_ns != 0 {
        msg.msg_control = control.as_mut_ptr() as *mut c_void;
        msg.msg_controllen = control.len();
        let cm = cmsg_firsthdr(&mut msg);
        (*cm).cmsg_level = SOL_SOCKET;
        (*cm).cmsg_type = SCM_TXTIME;
        (*cm).cmsg_len = cmsg_len(size_of::<u64>());
        memcpy(cmsg_data(cm) as *mut c_void, &txtime_ns as *const _ as *const c_void, size_of::<u64>());
    }
    let ret = sendmsg(fd, &msg, 0);
    if ret == -1 { error(1, errno, s!("sendmsg failure")); }
    if ret != len as isize { error(1, 0, s!("sendmsg wrong length: %d vs %d"), ret as c_int, len); }
}

unsafe fn create_packet(buf: *mut c_void, seq_offset: c_int, ack_offset: c_int, payload_len: c_int, fin: c_int) {
    let ip_hdr_len = if proto == PF_INET { size_of::<iphdr>() } else { size_of::<ipv6hdr>() } as c_int;
    let inner_ip_off = tcp_offset - ip_hdr_len;
    memset(buf, 0, total_hdr_len as usize);
    memset((buf as *mut u8).add(total_hdr_len as usize) as *mut c_void, b'a' as c_int, payload_len as usize);
    fill_transportlayer((buf as *mut u8).add(tcp_offset as usize) as *mut c_void, seq_offset, ack_offset, payload_len, fin);
    fill_networklayer((buf as *mut u8).add(inner_ip_off as usize) as *mut c_void, payload_len, IPPROTO_TCP);
    if inner_ip_off > ETH_HLEN as c_int {
        if pppoe {
            fill_pppoelayer((buf as *mut u8).add(ETH_HLEN) as *mut c_void, payload_len + ip_hdr_len, 0x1234);
        } else {
            let encap_proto = if proto == PF_INET { IPPROTO_IPIP } else { IPPROTO_IPV6 };
            fill_networklayer((buf as *mut u8).add(ETH_HLEN) as *mut c_void, payload_len + ip_hdr_len, encap_proto);
        }
    }
    fill_datalinklayer(buf);
}

#[repr(C)]
struct pppoe_ppp_hdr { eh: pppoe_hdr, proto: __be16 }
unsafe fn fill_pppoelayer(buf: *mut c_void, mut payload_len: c_int, sid: u16) {
    let ph = buf as *mut pppoe_ppp_hdr;
    payload_len += size_of::<tcphdr>() as c_int;
    (*ph).eh.set_type(1);
    (*ph).eh.set_ver(1);
    (*ph).eh.code = 0;
    (*ph).eh.sid = htons(sid);
    (*ph).eh.length = htons((payload_len + size_of::<__be16>() as c_int) as u16);
    (*ph).proto = htons(if proto == PF_INET { PPP_IP } else { PPP_IPV6 } as u16);
}

unsafe fn setup_sock_filter(fd: c_int) {
    let dport_off = tcp_offset + offset_of_tcphdr_dest() as c_int;
    let ethproto_off = 12usize;
    let mut optlen = 0;
    let ipproto_off: c_int;
    let mut opt_ipproto_off: c_int;
    if proto == PF_INET {
        ipproto_off = tcp_offset - size_of::<iphdr>() as c_int + 9;
    } else {
        ipproto_off = tcp_offset - size_of::<ipv6hdr>() as c_int + 6;
    }
    opt_ipproto_off = ipproto_off;
    if streq(testname, b"ip_opt\0") {
        optlen = size_of::<ip_timestamp>() as c_int;
    } else if streq(testname, b"ip_frag6\0") || streq(testname, b"ip_v6ext_same\0") || streq(testname, b"ip_v6ext_diff\0") {
        /* BUILD_BUG_ON checks for IPv6 extension header minimum sizes. */
        optlen = MIN_EXTHDR_SIZE as c_int;
        opt_ipproto_off = ETH_HLEN as c_int + size_of::<ipv6hdr>() as c_int;
    }
    let mut filter = [
        BPF_STMT!(BPF_LD + BPF_H + BPF_ABS, ethproto_off),
        BPF_JUMP!(BPF_JMP + BPF_JEQ + BPF_K, ntohs(ethhdr_proto as u16), 0, 9),
        BPF_STMT!(BPF_LD + BPF_B + BPF_ABS, ipproto_off),
        BPF_JUMP!(BPF_JMP + BPF_JEQ + BPF_K, IPPROTO_TCP, 2, 0),
        BPF_STMT!(BPF_LD + BPF_B + BPF_ABS, opt_ipproto_off),
        BPF_JUMP!(BPF_JMP + BPF_JEQ + BPF_K, IPPROTO_TCP, 0, 5),
        BPF_STMT!(BPF_LD + BPF_H + BPF_ABS, dport_off),
        BPF_JUMP!(BPF_JMP + BPF_JEQ + BPF_K, DPORT, 2, 0),
        BPF_STMT!(BPF_LD + BPF_H + BPF_ABS, dport_off + optlen),
        BPF_JUMP!(BPF_JMP + BPF_JEQ + BPF_K, DPORT, 0, 1),
        BPF_STMT!(BPF_RET + BPF_K, 0xffffffffu32),
        BPF_STMT!(BPF_RET + BPF_K, 0),
    ];
    let bpf = sock_fprog { len: filter.len() as u16, filter: filter.as_mut_ptr() };
    if setsockopt(fd, SOL_SOCKET, SO_ATTACH_FILTER, &bpf as *const _ as *const c_void, size_of::<sock_fprog>() as socklen_t) < 0 {
        error(1, errno, s!("error setting filter"));
    }
}
const fn offset_of_tcphdr_dest() -> usize { 2 }

unsafe fn create_capacity_packet(buf: *mut c_void, flow_id: c_int, pkt_idx: c_int, psh: c_int) {
    let seq_offset = pkt_idx * CAPACITY_PAYLOAD_LEN;
    create_packet(buf, seq_offset, 0, CAPACITY_PAYLOAD_LEN, 0);
    memset((buf as *mut u8).add(total_hdr_len as usize) as *mut c_void, b'a' as c_int + flow_id, CAPACITY_PAYLOAD_LEN as usize);
    let tcph = (buf as *mut u8).add(tcp_offset as usize) as *mut tcphdr;
    (*tcph).source = htons((SPORT + flow_id) as u16);
    (*tcph).set_psh(psh);
    (*tcph).check = 0;
    (*tcph).check = tcp_checksum(tcph as *mut c_void, CAPACITY_PAYLOAD_LEN);
}

unsafe fn send_capacity(fd: c_int, daddr: *mut sockaddr_ll) {
    static mut BUF: [c_char; MAX_HDR_LEN + CAPACITY_PAYLOAD_LEN as usize] = [0; MAX_HDR_LEN + CAPACITY_PAYLOAD_LEN as usize];
    let pkt_size = total_hdr_len + CAPACITY_PAYLOAD_LEN;
    let mut i = 0;
    while i < num_flows { create_capacity_packet(BUF.as_mut_ptr() as *mut c_void, i, 0, 0); write_packet(fd, BUF.as_mut_ptr(), pkt_size, daddr); i += 1; }
    i = 0;
    while i < num_flows { create_capacity_packet(BUF.as_mut_ptr() as *mut c_void, i, 1, 1); write_packet(fd, BUF.as_mut_ptr(), pkt_size, daddr); i += 1; }
}

unsafe fn set_flags(tcph: *mut tcphdr, payload_len: c_int, psh: c_int, syn: c_int, rst: c_int, urg: c_int, cwr: c_int) {
    (*tcph).set_psh(psh); (*tcph).set_syn(syn); (*tcph).set_rst(rst); (*tcph).set_urg(urg);
    if cwr != 0 { (*tcph).th_flags |= TH_CWR; } else { (*tcph).th_flags &= !TH_CWR; }
    (*tcph).check = 0;
    (*tcph).check = tcp_checksum(tcph as *mut c_void, payload_len);
}

unsafe fn send_flags(fd: c_int, daddr: *mut sockaddr_ll, psh: c_int, syn: c_int, rst: c_int, urg: c_int, cwr: c_int) {
    static mut FLAG_BUF: [[c_char; MAX_HDR_LEN + PAYLOAD_LEN as usize]; 2] = [[0; MAX_HDR_LEN + PAYLOAD_LEN as usize]; 2];
    static mut BUF: [c_char; MAX_HDR_LEN + PAYLOAD_LEN as usize] = [0; MAX_HDR_LEN + PAYLOAD_LEN as usize];
    let payload_len = PAYLOAD_LEN * ((psh != 0 || cwr != 0) as c_int);
    let pkt_size = total_hdr_len + payload_len;
    let flag = [NUM_PACKETS as c_int / 2, NUM_PACKETS as c_int / 2 - 1];
    let mut i = 0;
    while i < 2 {
        if flag[i] > 0 {
            create_packet(FLAG_BUF[i].as_mut_ptr() as *mut c_void, flag[i] * payload_len, 0, payload_len, 0);
            let tcph = FLAG_BUF[i].as_mut_ptr().add(tcp_offset as usize) as *mut tcphdr;
            set_flags(tcph, payload_len, psh, syn, rst, urg, cwr);
        }
        i += 1;
    }
    i = 0;
    while i < NUM_PACKETS + 1 {
        if i as c_int == flag[0] { write_packet(fd, FLAG_BUF[0].as_mut_ptr(), pkt_size, daddr); }
        else if i as c_int == flag[1] && cwr != 0 { write_packet(fd, FLAG_BUF[1].as_mut_ptr(), pkt_size, daddr); }
        else { create_packet(BUF.as_mut_ptr() as *mut c_void, i as c_int * PAYLOAD_LEN, 0, PAYLOAD_LEN, 0); write_packet(fd, BUF.as_mut_ptr(), total_hdr_len + PAYLOAD_LEN, daddr); }
        i += 1;
    }
}

unsafe fn send_data_pkts(fd: c_int, daddr: *mut sockaddr_ll, payload_len1: c_int, payload_len2: c_int) {
    static mut BUF: [c_char; L2_HLEN_MAX + IP_MAXPACKET] = [0; L2_HLEN_MAX + IP_MAXPACKET];
    create_packet(BUF.as_mut_ptr() as *mut c_void, 0, 0, payload_len1, 0);
    write_packet(fd, BUF.as_mut_ptr(), total_hdr_len + payload_len1, daddr);
    create_packet(BUF.as_mut_ptr() as *mut c_void, payload_len1, 0, payload_len2, 0);
    write_packet(fd, BUF.as_mut_ptr(), total_hdr_len + payload_len2, daddr);
}

unsafe fn send_large(fd: c_int, daddr: *mut sockaddr_ll, remainder: c_int) {
    static mut PKTS: [[c_char; MAX_HDR_LEN + MAX_MSS]; MAX_LARGE_PKT_CNT] = [[0; MAX_HDR_LEN + MAX_MSS]; MAX_LARGE_PKT_CNT];
    static mut NEW_SEG: [c_char; MAX_HDR_LEN + MAX_MSS] = [0; MAX_HDR_LEN + MAX_MSS];
    static mut LAST: [c_char; MAX_HDR_LEN + MAX_MSS] = [0; MAX_HDR_LEN + MAX_MSS];
    let num_pkt = num_large_pkt();
    let mss = calc_mss();
    let mut i = 0;
    while i < num_pkt { create_packet(PKTS[i as usize].as_mut_ptr() as *mut c_void, i * mss, 0, mss, 0); i += 1; }
    create_packet(LAST.as_mut_ptr() as *mut c_void, num_pkt * mss, 0, remainder, 0);
    create_packet(NEW_SEG.as_mut_ptr() as *mut c_void, (num_pkt + 1) * mss, 0, remainder, 0);
    i = 0;
    while i < num_pkt { write_packet(fd, PKTS[i as usize].as_mut_ptr(), total_hdr_len + mss, daddr); i += 1; }
    write_packet(fd, LAST.as_mut_ptr(), total_hdr_len + remainder, daddr);
    write_packet(fd, NEW_SEG.as_mut_ptr(), total_hdr_len + remainder, daddr);
}

unsafe fn send_ack(fd: c_int, daddr: *mut sockaddr_ll) {
    static mut BUF: [c_char; MAX_HDR_LEN] = [0; MAX_HDR_LEN];
    create_packet(BUF.as_mut_ptr() as *mut c_void, 0, 0, 0, 0);
    write_packet(fd, BUF.as_mut_ptr(), total_hdr_len, daddr);
    write_packet(fd, BUF.as_mut_ptr(), total_hdr_len, daddr);
    create_packet(BUF.as_mut_ptr() as *mut c_void, 0, 1, 0, 0);
    write_packet(fd, BUF.as_mut_ptr(), total_hdr_len, daddr);
}

unsafe fn recompute_packet(buf: *mut c_char, no_ext: *mut c_char, extlen: c_int) {
    let tcphdr = buf.add(tcp_offset as usize) as *mut tcphdr;
    memmove(buf as *mut c_void, no_ext as *const c_void, total_hdr_len as usize);
    memmove(buf.add((total_hdr_len + extlen) as usize) as *mut c_void, no_ext.add(total_hdr_len as usize) as *const c_void, PAYLOAD_LEN as usize);
    (*tcphdr).set_doff((*tcphdr).doff() + (extlen / 4) as u8);
    (*tcphdr).check = 0;
    (*tcphdr).check = tcp_checksum(tcphdr as *mut c_void, PAYLOAD_LEN + extlen);
    let mut off = ETH_HLEN as c_int;
    if proto == PF_INET {
        while off < tcp_offset {
            let iph = buf.add(off as usize) as *mut iphdr;
            (*iph).tot_len = htons(ntohs((*iph).tot_len).wrapping_add(extlen as u16));
            (*iph).check = 0;
            (*iph).check = checksum_fold(iph as *mut c_void, size_of::<iphdr>(), 0);
            off += size_of::<iphdr>() as c_int;
        }
    } else {
        while off < tcp_offset {
            let ip6h = buf.add(off as usize) as *mut ipv6hdr;
            (*ip6h).payload_len = htons(ntohs((*ip6h).payload_len).wrapping_add(extlen as u16));
            off += size_of::<ipv6hdr>() as c_int;
        }
    }
}

unsafe fn tcp_write_options(buf: *mut c_char, kind: c_int, ts: c_int) {
    #[repr(C)] struct tcp_option_ts { kind: u8, len: u8, tsval: u32, tsecr: u32 }
    #[repr(C)] struct tcp_option_window { kind: u8, len: u8, shift: u8 }
    match kind {
        TCPOPT_NOP => *buf = TCPOPT_NOP as c_char,
        TCPOPT_WINDOW => {
            let opt_window = buf as *mut tcp_option_window;
            memset(opt_window as *mut c_void, 0, size_of::<tcp_option_window>());
            (*opt_window).kind = TCPOPT_WINDOW as u8; (*opt_window).len = TCPOLEN_WINDOW as u8; (*opt_window).shift = 0;
        }
        TCPOPT_TIMESTAMP => {
            let opt_ts = buf as *mut tcp_option_ts;
            memset(opt_ts as *mut c_void, 0, size_of::<tcp_option_ts>());
            (*opt_ts).kind = TCPOPT_TIMESTAMP as u8; (*opt_ts).len = TCPOLEN_TIMESTAMP as u8; (*opt_ts).tsval = ts as u32; (*opt_ts).tsecr = 0;
        }
        _ => error(1, 0, s!("unimplemented TCP option")),
    }
}

unsafe fn add_standard_tcp_options(buf: *mut c_char, no_ext: *mut c_char, ts: c_int, order: c_int) {
    match order {
        0 => { tcp_write_options(buf.add(total_hdr_len as usize), TCPOPT_NOP, 0); tcp_write_options(buf.add(total_hdr_len as usize + 1), TCPOPT_NOP, 0); tcp_write_options(buf.add(total_hdr_len as usize + 2), TCPOPT_TIMESTAMP, ts); }
        1 => { tcp_write_options(buf.add(total_hdr_len as usize), TCPOPT_NOP, 0); tcp_write_options(buf.add(total_hdr_len as usize + 1), TCPOPT_TIMESTAMP, ts); tcp_write_options(buf.add(total_hdr_len as usize + 1 + TCPOLEN_TIMESTAMP as usize), TCPOPT_NOP, 0); }
        2 => { tcp_write_options(buf.add(total_hdr_len as usize), TCPOPT_TIMESTAMP, ts); tcp_write_options(buf.add(total_hdr_len as usize + TCPOLEN_TIMESTAMP as usize + 1), TCPOPT_NOP, 0); tcp_write_options(buf.add(total_hdr_len as usize + TCPOLEN_TIMESTAMP as usize + 2), TCPOPT_NOP, 0); }
        _ => error(1, 0, s!("unknown order")),
    }
    recompute_packet(buf, no_ext, TCPOLEN_TSTAMP_APPA);
}

unsafe fn send_changed_checksum(fd: c_int, daddr: *mut sockaddr_ll) {
    static mut BUF: [c_char; MAX_HDR_LEN + PAYLOAD_LEN as usize] = [0; MAX_HDR_LEN + PAYLOAD_LEN as usize];
    let pkt_size = total_hdr_len + PAYLOAD_LEN;
    create_packet(BUF.as_mut_ptr() as *mut c_void, 0, 0, PAYLOAD_LEN, 0); write_packet(fd, BUF.as_mut_ptr(), pkt_size, daddr);
    create_packet(BUF.as_mut_ptr() as *mut c_void, PAYLOAD_LEN, 0, PAYLOAD_LEN, 0);
    let tcph = BUF.as_mut_ptr().add(tcp_offset as usize) as *mut tcphdr;
    (*tcph).check = (*tcph).check.wrapping_sub(1);
    write_packet(fd, BUF.as_mut_ptr(), pkt_size, daddr);
}

unsafe fn send_changed_ip_checksum(fd: c_int, daddr: *mut sockaddr_ll) {
    static mut BUF: [c_char; MAX_HDR_LEN + PAYLOAD_LEN as usize] = [0; MAX_HDR_LEN + PAYLOAD_LEN as usize];
    let pkt_size = total_hdr_len + PAYLOAD_LEN;
    create_packet(BUF.as_mut_ptr() as *mut c_void, 0, 0, PAYLOAD_LEN, 0); write_packet(fd, BUF.as_mut_ptr(), pkt_size, daddr);
    create_packet(BUF.as_mut_ptr() as *mut c_void, PAYLOAD_LEN, 0, PAYLOAD_LEN, 0);
    let iph = BUF.as_mut_ptr().add(ETH_HLEN) as *mut iphdr; (*iph).check = (*iph).check.wrapping_sub(1);
    write_packet(fd, BUF.as_mut_ptr(), pkt_size, daddr);
    create_packet(BUF.as_mut_ptr() as *mut c_void, PAYLOAD_LEN * 2, 0, PAYLOAD_LEN, 0); write_packet(fd, BUF.as_mut_ptr(), pkt_size, daddr);
}

unsafe fn send_changed_seq(fd: c_int, daddr: *mut sockaddr_ll) {
    static mut BUF: [c_char; MAX_HDR_LEN + PAYLOAD_LEN as usize] = [0; MAX_HDR_LEN + PAYLOAD_LEN as usize];
    let pkt_size = total_hdr_len + PAYLOAD_LEN;
    create_packet(BUF.as_mut_ptr() as *mut c_void, 0, 0, PAYLOAD_LEN, 0); write_packet(fd, BUF.as_mut_ptr(), pkt_size, daddr);
    create_packet(BUF.as_mut_ptr() as *mut c_void, PAYLOAD_LEN, 0, PAYLOAD_LEN, 0);
    let tcph = BUF.as_mut_ptr().add(tcp_offset as usize) as *mut tcphdr;
    (*tcph).seq = ntohl(htonl((*tcph).seq).wrapping_add(1)); (*tcph).check = 0; (*tcph).check = tcp_checksum(tcph as *mut c_void, PAYLOAD_LEN);
    write_packet(fd, BUF.as_mut_ptr(), pkt_size, daddr);
}

unsafe fn send_changed_ts(fd: c_int, daddr: *mut sockaddr_ll) {
    static mut BUF: [c_char; MAX_HDR_LEN + PAYLOAD_LEN as usize] = [0; MAX_HDR_LEN + PAYLOAD_LEN as usize];
    static mut EXTPKT: [c_char; MAX_HDR_LEN + PAYLOAD_LEN as usize + TCPOLEN_TSTAMP_APPA as usize] = [0; MAX_HDR_LEN + PAYLOAD_LEN as usize + TCPOLEN_TSTAMP_APPA as usize];
    let pkt_size = total_hdr_len + PAYLOAD_LEN + TCPOLEN_TSTAMP_APPA;
    for (i, ts, order) in [(0,0,0),(1,0,0),(2,100,0),(3,100,1),(4,100,2)] {
        create_packet(BUF.as_mut_ptr() as *mut c_void, PAYLOAD_LEN * i, 0, PAYLOAD_LEN, 0);
        add_standard_tcp_options(EXTPKT.as_mut_ptr(), BUF.as_mut_ptr(), ts, order);
        write_packet(fd, EXTPKT.as_mut_ptr(), pkt_size, daddr);
    }
}

unsafe fn send_diff_opt(fd: c_int, daddr: *mut sockaddr_ll) {
    static mut BUF: [c_char; MAX_HDR_LEN + PAYLOAD_LEN as usize] = [0; MAX_HDR_LEN + PAYLOAD_LEN as usize];
    static mut EXTPKT1: [c_char; MAX_HDR_LEN + PAYLOAD_LEN as usize + TCPOLEN_TSTAMP_APPA as usize] = [0; MAX_HDR_LEN + PAYLOAD_LEN as usize + TCPOLEN_TSTAMP_APPA as usize];
    static mut EXTPKT2: [c_char; MAX_HDR_LEN + PAYLOAD_LEN as usize + TCPOLEN_MAXSEG as usize] = [0; MAX_HDR_LEN + PAYLOAD_LEN as usize + TCPOLEN_MAXSEG as usize];
    let extpkt1_size = total_hdr_len + PAYLOAD_LEN + TCPOLEN_TSTAMP_APPA;
    let extpkt2_size = total_hdr_len + PAYLOAD_LEN + TCPOLEN_MAXSEG;
    create_packet(BUF.as_mut_ptr() as *mut c_void, 0, 0, PAYLOAD_LEN, 0); add_standard_tcp_options(EXTPKT1.as_mut_ptr(), BUF.as_mut_ptr(), 0, 0); write_packet(fd, EXTPKT1.as_mut_ptr(), extpkt1_size, daddr);
    create_packet(BUF.as_mut_ptr() as *mut c_void, PAYLOAD_LEN, 0, PAYLOAD_LEN, 0); add_standard_tcp_options(EXTPKT1.as_mut_ptr(), BUF.as_mut_ptr(), 0, 0); write_packet(fd, EXTPKT1.as_mut_ptr(), extpkt1_size, daddr);
    create_packet(BUF.as_mut_ptr() as *mut c_void, PAYLOAD_LEN * 2, 0, PAYLOAD_LEN, 0);
    tcp_write_options(EXTPKT2.as_mut_ptr().add(MAX_HDR_LEN), TCPOPT_NOP, 0);
    tcp_write_options(EXTPKT2.as_mut_ptr().add(MAX_HDR_LEN + 1), TCPOPT_WINDOW, 0);
    recompute_packet(EXTPKT2.as_mut_ptr(), BUF.as_mut_ptr(), TCPOLEN_WINDOW + 1);
    write_packet(fd, EXTPKT2.as_mut_ptr(), extpkt2_size, daddr);
}

unsafe fn add_ipv4_ts_option(buf: *mut c_void, optpkt: *mut c_void) {
    let ts = (optpkt as *mut u8).add(tcp_offset as usize) as *mut ip_timestamp;
    let optlen = size_of::<ip_timestamp>() as c_int;
    if optlen % 4 != 0 { error(1, 0, s!("ipv4 timestamp length is not a multiple of 4B")); }
    (*ts).ipt_code = IPOPT_TS as u8; (*ts).ipt_len = optlen as u8; (*ts).ipt_ptr = 5; (*ts).ipt_flg = IPOPT_TS_TSONLY as u8;
    memcpy(optpkt, buf, tcp_offset as usize);
    memcpy((optpkt as *mut u8).add((tcp_offset + optlen) as usize) as *mut c_void, (buf as *mut u8).add(tcp_offset as usize) as *const c_void, size_of::<tcphdr>() + PAYLOAD_LEN as usize);
    let iph = (optpkt as *mut u8).add(ETH_HLEN) as *mut iphdr;
    (*iph).set_ihl(5 + (optlen / 4) as u8);
    (*iph).tot_len = htons(ntohs((*iph).tot_len).wrapping_add(optlen as u16));
    (*iph).check = 0; (*iph).check = checksum_fold(iph as *mut c_void, size_of::<iphdr>() + optlen as usize, 0);
}

unsafe fn add_ipv6_exthdr(buf: *mut c_void, optpkt: *mut c_void, exthdr_type: __u8, ext_payload: *mut c_char) {
    let exthdr = (optpkt as *mut u8).add(tcp_offset as usize) as *mut ipv6_opt_hdr;
    let iph = (optpkt as *mut u8).add(ETH_HLEN) as *mut ipv6hdr;
    (*exthdr).ip6e_len = 0; (*exthdr).ip6e_nxt = IPPROTO_TCP as u8;
    memcpy(exthdr.add(1) as *mut c_void, ext_payload as *const c_void, MIN_EXTHDR_SIZE - size_of::<ipv6_opt_hdr>());
    memcpy(optpkt, buf, tcp_offset as usize);
    memcpy((optpkt as *mut u8).add(tcp_offset as usize + MIN_EXTHDR_SIZE) as *mut c_void, (buf as *mut u8).add(tcp_offset as usize) as *const c_void, size_of::<tcphdr>() + PAYLOAD_LEN as usize);
    (*iph).nexthdr = exthdr_type; (*iph).payload_len = htons(ntohs((*iph).payload_len).wrapping_add(MIN_EXTHDR_SIZE as u16));
}

unsafe fn fix_ip4_checksum(iph: *mut iphdr) { (*iph).check = 0; (*iph).check = checksum_fold(iph as *mut c_void, size_of::<iphdr>(), 0); }

unsafe fn send_flush_id_case(fd: c_int, daddr: *mut sockaddr_ll, tcase: flush_id_case) {
    static mut BUF1: [c_char; MAX_HDR_LEN + PAYLOAD_LEN as usize] = [0; MAX_HDR_LEN + PAYLOAD_LEN as usize];
    static mut BUF2: [c_char; MAX_HDR_LEN + PAYLOAD_LEN as usize] = [0; MAX_HDR_LEN + PAYLOAD_LEN as usize];
    static mut BUF3: [c_char; MAX_HDR_LEN + PAYLOAD_LEN as usize] = [0; MAX_HDR_LEN + PAYLOAD_LEN as usize];
    let mut send_three = false;
    let iph1 = BUF1.as_mut_ptr().add(ETH_HLEN) as *mut iphdr;
    let iph2 = BUF2.as_mut_ptr().add(ETH_HLEN) as *mut iphdr;
    let iph3 = BUF3.as_mut_ptr().add(ETH_HLEN) as *mut iphdr;
    create_packet(BUF1.as_mut_ptr() as *mut c_void, 0, 0, PAYLOAD_LEN, 0);
    create_packet(BUF2.as_mut_ptr() as *mut c_void, PAYLOAD_LEN, 0, PAYLOAD_LEN, 0);
    create_packet(BUF3.as_mut_ptr() as *mut c_void, PAYLOAD_LEN * 2, 0, PAYLOAD_LEN, 0);
    match tcase {
        FLUSH_ID_DF1_INC => { (*iph1).frag_off |= htons(IP_DF as u16); (*iph1).id = htons(8); (*iph2).frag_off |= htons(IP_DF as u16); (*iph2).id = htons(9); }
        FLUSH_ID_DF1_FIXED => { (*iph1).frag_off |= htons(IP_DF as u16); (*iph1).id = htons(8); (*iph2).frag_off |= htons(IP_DF as u16); (*iph2).id = htons(8); }
        FLUSH_ID_DF0_INC => { (*iph1).frag_off &= !htons(IP_DF as u16); (*iph1).id = htons(8); (*iph2).frag_off &= !htons(IP_DF as u16); (*iph2).id = htons(9); }
        FLUSH_ID_DF0_FIXED => { (*iph1).frag_off &= !htons(IP_DF as u16); (*iph1).id = htons(8); (*iph2).frag_off &= !htons(IP_DF as u16); (*iph2).id = htons(8); }
        FLUSH_ID_DF1_INC_FIXED => { (*iph1).frag_off |= htons(IP_DF as u16); (*iph1).id = htons(8); (*iph2).frag_off |= htons(IP_DF as u16); (*iph2).id = htons(9); (*iph3).frag_off |= htons(IP_DF as u16); (*iph3).id = htons(9); send_three = true; }
        FLUSH_ID_DF1_FIXED_INC => { (*iph1).frag_off |= htons(IP_DF as u16); (*iph1).id = htons(8); (*iph2).frag_off |= htons(IP_DF as u16); (*iph2).id = htons(8); (*iph3).frag_off |= htons(IP_DF as u16); (*iph3).id = htons(9); send_three = true; }
    }
    fix_ip4_checksum(iph1); fix_ip4_checksum(iph2);
    write_packet(fd, BUF1.as_mut_ptr(), total_hdr_len + PAYLOAD_LEN, daddr);
    write_packet(fd, BUF2.as_mut_ptr(), total_hdr_len + PAYLOAD_LEN, daddr);
    if send_three { fix_ip4_checksum(iph3); write_packet(fd, BUF3.as_mut_ptr(), total_hdr_len + PAYLOAD_LEN, daddr); }
}

unsafe fn send_ipv6_exthdr(fd: c_int, daddr: *mut sockaddr_ll, ext_data1: *mut c_char, ext_data2: *mut c_char) {
    static mut BUF: [c_char; MAX_HDR_LEN + PAYLOAD_LEN as usize] = [0; MAX_HDR_LEN + PAYLOAD_LEN as usize];
    static mut EXTHDR_PCK: [c_char; MAX_HDR_LEN + PAYLOAD_LEN as usize + MIN_EXTHDR_SIZE] = [0; MAX_HDR_LEN + PAYLOAD_LEN as usize + MIN_EXTHDR_SIZE];
    create_packet(BUF.as_mut_ptr() as *mut c_void, 0, 0, PAYLOAD_LEN, 0); add_ipv6_exthdr(BUF.as_mut_ptr() as *mut c_void, EXTHDR_PCK.as_mut_ptr() as *mut c_void, IPPROTO_DSTOPTS as u8, ext_data1); write_packet(fd, EXTHDR_PCK.as_mut_ptr(), total_hdr_len + PAYLOAD_LEN + MIN_EXTHDR_SIZE as c_int, daddr);
    create_packet(BUF.as_mut_ptr() as *mut c_void, PAYLOAD_LEN, 0, PAYLOAD_LEN, 0); add_ipv6_exthdr(BUF.as_mut_ptr() as *mut c_void, EXTHDR_PCK.as_mut_ptr() as *mut c_void, IPPROTO_DSTOPTS as u8, ext_data2); write_packet(fd, EXTHDR_PCK.as_mut_ptr(), total_hdr_len + PAYLOAD_LEN + MIN_EXTHDR_SIZE as c_int, daddr);
}

unsafe fn send_ip_options(fd: c_int, daddr: *mut sockaddr_ll) {
    static mut BUF: [c_char; MAX_HDR_LEN + PAYLOAD_LEN as usize] = [0; MAX_HDR_LEN + PAYLOAD_LEN as usize];
    static mut OPTPKT: [c_char; MAX_HDR_LEN + PAYLOAD_LEN as usize + size_of::<ip_timestamp>()] = [0; MAX_HDR_LEN + PAYLOAD_LEN as usize + size_of::<ip_timestamp>()];
    let pkt_size = total_hdr_len + PAYLOAD_LEN + size_of::<ip_timestamp>() as c_int;
    create_packet(BUF.as_mut_ptr() as *mut c_void, 0, 0, PAYLOAD_LEN, 0); write_packet(fd, BUF.as_mut_ptr(), total_hdr_len + PAYLOAD_LEN, daddr);
    create_packet(BUF.as_mut_ptr() as *mut c_void, PAYLOAD_LEN, 0, PAYLOAD_LEN, 0); add_ipv4_ts_option(BUF.as_mut_ptr() as *mut c_void, OPTPKT.as_mut_ptr() as *mut c_void); write_packet(fd, OPTPKT.as_mut_ptr(), pkt_size, daddr);
    create_packet(BUF.as_mut_ptr() as *mut c_void, PAYLOAD_LEN * 2, 0, PAYLOAD_LEN, 0); write_packet(fd, BUF.as_mut_ptr(), total_hdr_len + PAYLOAD_LEN, daddr);
}

unsafe fn send_fragment4(fd: c_int, daddr: *mut sockaddr_ll) {
    static mut BUF: [c_char; IP_MAXPACKET] = [0; IP_MAXPACKET];
    let iph = BUF.as_mut_ptr().add(ETH_HLEN) as *mut iphdr;
    let pkt_size = total_hdr_len + PAYLOAD_LEN;
    create_packet(BUF.as_mut_ptr() as *mut c_void, 0, 0, PAYLOAD_LEN, 0); write_packet(fd, BUF.as_mut_ptr(), pkt_size, daddr);
    memset(BUF.as_mut_ptr().add(total_hdr_len as usize) as *mut c_void, b'a' as c_int, (PAYLOAD_LEN * 2) as usize);
    fill_transportlayer(BUF.as_mut_ptr().add(tcp_offset as usize) as *mut c_void, PAYLOAD_LEN, 0, PAYLOAD_LEN * 2, 0);
    fill_networklayer(BUF.as_mut_ptr().add(ETH_HLEN) as *mut c_void, PAYLOAD_LEN, IPPROTO_TCP); fill_datalinklayer(BUF.as_mut_ptr() as *mut c_void);
    (*iph).frag_off = htons(0x6000); (*iph).check = 0; (*iph).check = checksum_fold(iph as *mut c_void, size_of::<iphdr>(), 0);
    write_packet(fd, BUF.as_mut_ptr(), pkt_size, daddr);
}

unsafe fn send_changed_ttl(fd: c_int, daddr: *mut sockaddr_ll) {
    static mut BUF: [c_char; MAX_HDR_LEN + PAYLOAD_LEN as usize] = [0; MAX_HDR_LEN + PAYLOAD_LEN as usize];
    let pkt_size = total_hdr_len + PAYLOAD_LEN; let iph = BUF.as_mut_ptr().add(ETH_HLEN) as *mut iphdr;
    create_packet(BUF.as_mut_ptr() as *mut c_void, 0, 0, PAYLOAD_LEN, 0); write_packet(fd, BUF.as_mut_ptr(), pkt_size, daddr);
    create_packet(BUF.as_mut_ptr() as *mut c_void, PAYLOAD_LEN, 0, PAYLOAD_LEN, 0); (*iph).ttl = 7; (*iph).check = 0; (*iph).check = checksum_fold(iph as *mut c_void, size_of::<iphdr>(), 0); write_packet(fd, BUF.as_mut_ptr(), pkt_size, daddr);
}

unsafe fn send_changed_tos(fd: c_int, daddr: *mut sockaddr_ll) {
    static mut BUF: [c_char; MAX_HDR_LEN + PAYLOAD_LEN as usize] = [0; MAX_HDR_LEN + PAYLOAD_LEN as usize];
    let pkt_size = total_hdr_len + PAYLOAD_LEN; let iph = BUF.as_mut_ptr().add(ETH_HLEN) as *mut iphdr; let ip6h = BUF.as_mut_ptr().add(ETH_HLEN) as *mut ipv6hdr;
    create_packet(BUF.as_mut_ptr() as *mut c_void, 0, 0, PAYLOAD_LEN, 0); write_packet(fd, BUF.as_mut_ptr(), pkt_size, daddr);
    create_packet(BUF.as_mut_ptr() as *mut c_void, PAYLOAD_LEN, 0, PAYLOAD_LEN, 0);
    if proto == PF_INET { (*iph).tos = 1; (*iph).check = 0; (*iph).check = checksum_fold(iph as *mut c_void, size_of::<iphdr>(), 0); } else if proto == PF_INET6 { (*ip6h).set_priority(0xf); }
    write_packet(fd, BUF.as_mut_ptr(), pkt_size, daddr);
}

unsafe fn send_changed_ECN(fd: c_int, daddr: *mut sockaddr_ll) {
    static mut BUF: [c_char; MAX_HDR_LEN + PAYLOAD_LEN as usize] = [0; MAX_HDR_LEN + PAYLOAD_LEN as usize];
    let pkt_size = total_hdr_len + PAYLOAD_LEN; let iph = BUF.as_mut_ptr().add(ETH_HLEN) as *mut iphdr;
    create_packet(BUF.as_mut_ptr() as *mut c_void, 0, 0, PAYLOAD_LEN, 0); write_packet(fd, BUF.as_mut_ptr(), pkt_size, daddr);
    create_packet(BUF.as_mut_ptr() as *mut c_void, PAYLOAD_LEN, 0, PAYLOAD_LEN, 0);
    if proto == PF_INET { *BUF.as_mut_ptr().add(ETH_HLEN + 1) ^= 0x2; (*iph).check = 0; (*iph).check = checksum_fold(iph as *mut c_void, size_of::<iphdr>(), 0); } else { *BUF.as_mut_ptr().add(ETH_HLEN + 1) ^= 0x20; }
    write_packet(fd, BUF.as_mut_ptr(), pkt_size, daddr);
}

unsafe fn send_fragment6(fd: c_int, daddr: *mut sockaddr_ll) {
    static mut BUF: [c_char; MAX_HDR_LEN + PAYLOAD_LEN as usize] = [0; MAX_HDR_LEN + PAYLOAD_LEN as usize];
    static mut EXTPKT: [c_char; MAX_HDR_LEN + PAYLOAD_LEN as usize + size_of::<ip6_frag>()] = [0; MAX_HDR_LEN + PAYLOAD_LEN as usize + size_of::<ip6_frag>()];
    let ip6h = BUF.as_mut_ptr().add(ETH_HLEN) as *mut ipv6hdr;
    let frag = EXTPKT.as_mut_ptr().add(tcp_offset as usize) as *mut ip6_frag;
    let extlen = size_of::<ip6_frag>() as c_int; let bufpkt_len = total_hdr_len + PAYLOAD_LEN; let extpkt_len = bufpkt_len + extlen;
    let mut i = 0; while i < 2 { create_packet(BUF.as_mut_ptr() as *mut c_void, PAYLOAD_LEN * i, 0, PAYLOAD_LEN, 0); write_packet(fd, BUF.as_mut_ptr(), bufpkt_len, daddr); i += 1; }
    sleep(1); create_packet(BUF.as_mut_ptr() as *mut c_void, PAYLOAD_LEN * 2, 0, PAYLOAD_LEN, 0); memset(EXTPKT.as_mut_ptr() as *mut c_void, 0, extpkt_len as usize);
    (*ip6h).nexthdr = IPPROTO_FRAGMENT as u8; (*ip6h).payload_len = htons(ntohs((*ip6h).payload_len).wrapping_add(extlen as u16)); (*frag).ip6f_nxt = IPPROTO_TCP as u8;
    memcpy(EXTPKT.as_mut_ptr() as *mut c_void, BUF.as_mut_ptr() as *const c_void, tcp_offset as usize);
    memcpy(EXTPKT.as_mut_ptr().add(tcp_offset as usize + extlen as usize) as *mut c_void, BUF.as_mut_ptr().add(tcp_offset as usize) as *const c_void, size_of::<tcphdr>() + PAYLOAD_LEN as usize);
    write_packet(fd, EXTPKT.as_mut_ptr(), extpkt_len, daddr);
    create_packet(BUF.as_mut_ptr() as *mut c_void, PAYLOAD_LEN * 3, 0, PAYLOAD_LEN, 0); write_packet(fd, BUF.as_mut_ptr(), bufpkt_len, daddr);
}

unsafe fn send_changed_pppoe_sid(fd: c_int, daddr: *mut sockaddr_ll) {
    static mut BUF: [c_char; MAX_HDR_LEN + PAYLOAD_LEN as usize] = [0; MAX_HDR_LEN + PAYLOAD_LEN as usize];
    let pkt_size = total_hdr_len + PAYLOAD_LEN; let hdr = BUF.as_mut_ptr().add(ETH_HLEN) as *mut pppoe_hdr;
    create_packet(BUF.as_mut_ptr() as *mut c_void, 0, 0, PAYLOAD_LEN, 0); write_packet(fd, BUF.as_mut_ptr(), pkt_size, daddr);
    create_packet(BUF.as_mut_ptr() as *mut c_void, PAYLOAD_LEN, 0, PAYLOAD_LEN, 0); (*hdr).sid = htons(0x4321); write_packet(fd, BUF.as_mut_ptr(), pkt_size, daddr);
}

unsafe fn bind_packetsocket(fd: c_int) {
    let mut daddr: sockaddr_ll = zeroed();
    daddr.sll_family = AF_PACKET as u16; daddr.sll_protocol = ethhdr_proto as u16; daddr.sll_ifindex = if_nametoindex(ifname) as c_int;
    if daddr.sll_ifindex == 0 { error(1, errno, s!("if_nametoindex")); }
    if bind(fd, &daddr as *const _ as *const c_void, size_of::<sockaddr_ll>() as socklen_t) < 0 { error(1, errno, s!("could not bind socket")); }
}
unsafe fn set_timeout(fd: c_int) {
    let timeout = timeval { tv_sec: 3, tv_usec: 0 };
    if setsockopt(fd, SOL_SOCKET, SO_RCVTIMEO, &timeout as *const _ as *const c_void, size_of::<timeval>() as socklen_t) < 0 { error(1, errno, s!("cannot set timeout, setsockopt failed")); }
}
unsafe fn set_rcvbuf(fd: c_int) {
    let bufsize: c_int = 1024 * 1024;
    if setsockopt(fd, SOL_SOCKET, SO_RCVBUF, &bufsize as *const _ as *const c_void, size_of::<c_int>() as socklen_t) != 0 { error(1, errno, s!("cannot set rcvbuf size, setsockopt failed")); }
}
unsafe fn recv_error(fd: c_int, rcv_errno: c_int) {
    let mut stats: tpacket_stats = zeroed(); let mut len = size_of::<tpacket_stats>() as socklen_t;
    if getsockopt(fd, SOL_PACKET, PACKET_STATISTICS, &mut stats as *mut _ as *mut c_void, &mut len) != 0 { error(1, errno, s!("can't get stats")); }
    fprintf(stderr, s!("Socket stats: packets=%u, drops=%u\n"), stats.tp_packets, stats.tp_drops);
    error(1, rcv_errno, s!("could not receive"));
}

unsafe fn check_recv_pkts(fd: c_int, correct_payload: *mut c_int, correct_num_pkts: c_int) {
    static mut BUFFER: [c_char; IP_MAXPACKET + L2_HLEN_MAX + 1] = [0; IP_MAXPACKET + L2_HLEN_MAX + 1];
    let nhoff = ETH_HLEN + if pppoe { PPPOE_SES_HLEN } else { 0 };
    let iph = BUFFER.as_mut_ptr().add(nhoff) as *mut iphdr; let ip6h = BUFFER.as_mut_ptr().add(nhoff) as *mut ipv6hdr;
    let mut bad_packet = false; let mut bytes_expected = 0; let mut bytes_received = 0; let mut num_pkt = 0;
    let mut i = 0; while i < correct_num_pkts { bytes_expected += *correct_payload.add(i as usize); i += 1; }
    loop {
        let mut ip_ext_len = 0; let pkt_size = recv(fd, BUFFER.as_mut_ptr() as *mut c_void, BUFFER.len(), 0) as c_int;
        if pkt_size < 0 { recv_error(fd, errno); }
        if (*iph).version() == 4 { ip_ext_len = ((*iph).ihl() as c_int - 5) * 4; }
        else if (*ip6h).version() == 6 && !ip6ip6 && (*ip6h).nexthdr != IPPROTO_TCP as u8 { ip_ext_len = MIN_EXTHDR_SIZE as c_int; }
        let tcph = BUFFER.as_mut_ptr().add((tcp_offset + ip_ext_len) as usize) as *mut tcphdr;
        if (*tcph).fin() { break; }
        let tcp_ext_len = ((*tcph).doff() as c_int - 5) * 4;
        let mut data_len = pkt_size - total_hdr_len - tcp_ext_len - ip_ext_len;
        if pkt_size == ETH_ZLEN && (*iph).version() == 4 { data_len = ntohs((*iph).tot_len) as c_int - size_of::<tcphdr>() as c_int - size_of::<iphdr>() as c_int; }
        if data_len != *correct_payload.add(num_pkt as usize) { bad_packet = true; }
        bytes_received += data_len; num_pkt += 1;
    }
    if num_pkt < correct_num_pkts && bytes_received == bytes_expected { error(EXIT_OVER_COALESCE, 0, s!("over-coalesced: got %d pkts vs expected %d (%d B)"), num_pkt, correct_num_pkts, bytes_received); }
    if num_pkt != correct_num_pkts { error(1, 0, s!("incorrect number of packets")); }
    if bad_packet { error(1, 0, s!("incorrect packet geometry")); }
    printf(s!("Test succeeded\n\n"));
}

unsafe fn check_capacity_pkts(fd: c_int) {
    static mut BUFFER: [c_char; IP_MAXPACKET + L2_HLEN_MAX + 1] = [0; IP_MAXPACKET + L2_HLEN_MAX + 1];
    let nhoff = ETH_HLEN + if pppoe { PPPOE_SES_HLEN } else { 0 };
    let iph = BUFFER.as_mut_ptr().add(nhoff) as *mut iphdr; let ip6h = BUFFER.as_mut_ptr().add(nhoff) as *mut ipv6hdr;
    let mut num_pkt = 0; let mut num_coal = 0; let mut fail_reason: *const c_char = ptr::null();
    let mut flow_order = vec![-1; (num_flows * 2) as usize]; let mut coalesced = vec![0; num_flows as usize];
    loop {
        let mut ip_ext_len = 0; let pkt_size = recv(fd, BUFFER.as_mut_ptr() as *mut c_void, BUFFER.len(), 0) as c_int;
        if pkt_size < 0 { recv_error(fd, errno); }
        if (*iph).version() == 4 { ip_ext_len = ((*iph).ihl() as c_int - 5) * 4; }
        else if (*ip6h).version() == 6 && !ip6ip6 && (*ip6h).nexthdr != IPPROTO_TCP as u8 { ip_ext_len = MIN_EXTHDR_SIZE as c_int; }
        let tcph = BUFFER.as_mut_ptr().add((tcp_offset + ip_ext_len) as usize) as *mut tcphdr;
        if (*tcph).fin() { break; }
        let sport = ntohs((*tcph).source) as c_int; let flow_id = sport - SPORT;
        if flow_id < 0 || flow_id >= num_flows { if fail_reason.is_null() { fail_reason = s!("invalid packet"); } continue; }
        let data_len = if pkt_size == ETH_ZLEN && (*iph).version() == 4 { ntohs((*iph).tot_len) as c_int - size_of::<tcphdr>() as c_int - size_of::<iphdr>() as c_int } else { pkt_size - total_hdr_len - ip_ext_len };
        if num_pkt < num_flows * 2 { flow_order[num_pkt as usize] = flow_id; } else if num_pkt == num_flows * 2 && fail_reason.is_null() { fail_reason = s!("too many packets"); }
        coalesced[flow_id as usize] = data_len;
        if data_len == CAPACITY_PAYLOAD_LEN * 2 { num_coal += 1; } else if fail_reason.is_null() { fail_reason = s!("not coalesced"); }
        num_pkt += 1;
    }
    let mut pkt_idx = 0; let mut flow_id = 0;
    while order_check && flow_id < num_flows {
        if coalesced[flow_id as usize] <= CAPACITY_PAYLOAD_LEN {
            if flow_order[pkt_idx as usize] != flow_id && fail_reason.is_null() { fail_reason = s!("bad packet order (1)"); }
            pkt_idx += 1;
        }
        flow_id += 1;
    }
    flow_id = 0;
    while order_check && flow_id < num_flows {
        if flow_order[pkt_idx as usize] != flow_id && fail_reason.is_null() { fail_reason = s!("bad packet order (2)"); }
        pkt_idx += 1; flow_id += 1;
    }
    if fail_reason.is_null() { printf(s!("Test succeeded\n\n")); } else { printf(s!("FAILED\n")); }
    printf(s!("STATS: received=%d wire=%d coalesced=%d\n"), num_pkt, num_pkt + num_coal, num_coal);
    if !fail_reason.is_null() { error(1, 0, s!("capacity test failed %s"), fail_reason); }
}

macro_rules! send_case {
    ($fd:expr, $daddr:expr, $fin:expr, $delay:expr, $body:block) => {{ $body; if $delay { usleep(100 * 1000); } write_packet($fd, $fin, total_hdr_len, $daddr); }};
}

unsafe fn gro_sender() {
    let bufsize: c_int = 4 * 1024 * 1024;
    static mut FIN_PKT: [c_char; MAX_HDR_LEN] = [0; MAX_HDR_LEN];
    let txfd = socket(PF_PACKET, SOCK_RAW, IPPROTO_RAW);
    if txfd < 0 { error(1, errno, s!("socket creation")); }
    if setsockopt(txfd, SOL_SOCKET, SO_SNDBUF, &bufsize as *const _ as *const c_void, size_of::<c_int>() as socklen_t) != 0 { error(1, errno, s!("cannot set sndbuf size, setsockopt failed")); }
    if strcmp(testname, s!("single")) != 0 && strcmp(testname, s!("capacity")) != 0 {
        let so_txtime = sock_txtime { clockid: CLOCK_MONOTONIC, flags: 0 }; let mut ts: timespec = zeroed();
        if setsockopt(txfd, SOL_SOCKET, SO_TXTIME, &so_txtime as *const _ as *const c_void, size_of::<sock_txtime>() as socklen_t) != 0 { error(1, errno, s!("setsockopt SO_TXTIME")); }
        if clock_gettime(CLOCK_MONOTONIC, &mut ts) != 0 { error(1, errno, s!("clock_gettime")); }
        txtime_ns = ts.tv_sec as u64 * 1000000000u64 + ts.tv_nsec as u64 + TXTIME_DELAY_MS * 1000000u64;
    }
    let mut daddr: sockaddr_ll = zeroed();
    daddr.sll_ifindex = if_nametoindex(ifname) as c_int; if daddr.sll_ifindex == 0 { error(1, errno, s!("if_nametoindex")); }
    daddr.sll_family = AF_PACKET as u16; memcpy(daddr.sll_addr.as_mut_ptr() as *mut c_void, dst_mac.as_ptr() as *const c_void, ETH_ALEN); daddr.sll_halen = ETH_ALEN as u8;
    create_packet(FIN_PKT.as_mut_ptr() as *mut c_void, PAYLOAD_LEN * 2, 0, 0, 1);
    if streq(testname, b"data_same\0") { send_case!(txfd, &mut daddr, FIN_PKT.as_mut_ptr(), false, { send_data_pkts(txfd, &mut daddr, PAYLOAD_LEN, PAYLOAD_LEN) }); }
    else if streq(testname, b"data_lrg_sml\0") { send_case!(txfd, &mut daddr, FIN_PKT.as_mut_ptr(), false, { send_data_pkts(txfd, &mut daddr, PAYLOAD_LEN, PAYLOAD_LEN / 2) }); }
    else if streq(testname, b"data_lrg_1byte\0") { send_case!(txfd, &mut daddr, FIN_PKT.as_mut_ptr(), false, { send_data_pkts(txfd, &mut daddr, PAYLOAD_LEN, 1) }); }
    else if streq(testname, b"data_sml_lrg\0") { send_case!(txfd, &mut daddr, FIN_PKT.as_mut_ptr(), false, { send_data_pkts(txfd, &mut daddr, PAYLOAD_LEN / 2, PAYLOAD_LEN) }); }
    else if streq(testname, b"data_burst\0") { static mut BUF: [c_char; MAX_HDR_LEN + PAYLOAD_LEN as usize] = [0; MAX_HDR_LEN + PAYLOAD_LEN as usize]; create_packet(BUF.as_mut_ptr() as *mut c_void,0,0,PAYLOAD_LEN,0); write_packet(txfd,BUF.as_mut_ptr(),total_hdr_len+PAYLOAD_LEN,&mut daddr); create_packet(BUF.as_mut_ptr() as *mut c_void,PAYLOAD_LEN,0,PAYLOAD_LEN,0); write_packet(txfd,BUF.as_mut_ptr(),total_hdr_len+PAYLOAD_LEN,&mut daddr); usleep(100*1000); create_packet(BUF.as_mut_ptr() as *mut c_void,PAYLOAD_LEN*2,0,PAYLOAD_LEN,0); write_packet(txfd,BUF.as_mut_ptr(),total_hdr_len+PAYLOAD_LEN,&mut daddr); create_packet(BUF.as_mut_ptr() as *mut c_void,PAYLOAD_LEN*3,0,PAYLOAD_LEN,0); write_packet(txfd,BUF.as_mut_ptr(),total_hdr_len+PAYLOAD_LEN,&mut daddr); write_packet(txfd, FIN_PKT.as_mut_ptr(), total_hdr_len, &mut daddr); }
    else if streq(testname, b"ack\0") { send_case!(txfd, &mut daddr, FIN_PKT.as_mut_ptr(), false, { send_ack(txfd, &mut daddr) }); }
    else if streq(testname, b"flags_psh\0") { send_case!(txfd, &mut daddr, FIN_PKT.as_mut_ptr(), false, { send_flags(txfd, &mut daddr, 1,0,0,0,0) }); }
    else if streq(testname, b"flags_syn\0") { send_case!(txfd, &mut daddr, FIN_PKT.as_mut_ptr(), false, { send_flags(txfd, &mut daddr, 0,1,0,0,0) }); }
    else if streq(testname, b"flags_rst\0") { send_case!(txfd, &mut daddr, FIN_PKT.as_mut_ptr(), false, { send_flags(txfd, &mut daddr, 0,0,1,0,0) }); }
    else if streq(testname, b"flags_urg\0") { send_case!(txfd, &mut daddr, FIN_PKT.as_mut_ptr(), false, { send_flags(txfd, &mut daddr, 0,0,0,1,0) }); }
    else if streq(testname, b"flags_cwr\0") { send_case!(txfd, &mut daddr, FIN_PKT.as_mut_ptr(), false, { send_flags(txfd, &mut daddr, 0,0,0,0,1) }); }
    else if streq(testname, b"tcp_csum\0") { send_case!(txfd, &mut daddr, FIN_PKT.as_mut_ptr(), true, { send_changed_checksum(txfd, &mut daddr) }); }
    else if streq(testname, b"tcp_seq\0") { send_case!(txfd, &mut daddr, FIN_PKT.as_mut_ptr(), true, { send_changed_seq(txfd, &mut daddr) }); }
    else if streq(testname, b"tcp_ts\0") { send_case!(txfd, &mut daddr, FIN_PKT.as_mut_ptr(), true, { send_changed_ts(txfd, &mut daddr) }); }
    else if streq(testname, b"tcp_opt\0") { send_case!(txfd, &mut daddr, FIN_PKT.as_mut_ptr(), true, { send_diff_opt(txfd, &mut daddr) }); }
    else if streq(testname, b"ip_ecn\0") { send_case!(txfd, &mut daddr, FIN_PKT.as_mut_ptr(), false, { send_changed_ECN(txfd, &mut daddr) }); }
    else if streq(testname, b"ip_tos\0") { send_case!(txfd, &mut daddr, FIN_PKT.as_mut_ptr(), false, { send_changed_tos(txfd, &mut daddr) }); }
    else if streq(testname, b"ip_csum\0") { send_case!(txfd, &mut daddr, FIN_PKT.as_mut_ptr(), true, { send_changed_ip_checksum(txfd, &mut daddr) }); }
    else if streq(testname, b"ip_ttl\0") { send_case!(txfd, &mut daddr, FIN_PKT.as_mut_ptr(), false, { send_changed_ttl(txfd, &mut daddr) }); }
    else if streq(testname, b"ip_opt\0") { send_case!(txfd, &mut daddr, FIN_PKT.as_mut_ptr(), true, { send_ip_options(txfd, &mut daddr) }); }
    else if streq(testname, b"ip_frag4\0") { send_case!(txfd, &mut daddr, FIN_PKT.as_mut_ptr(), true, { send_fragment4(txfd, &mut daddr) }); }
    else if streq(testname, b"ip_id_df1_inc\0") { send_case!(txfd, &mut daddr, FIN_PKT.as_mut_ptr(), true, { send_flush_id_case(txfd, &mut daddr, FLUSH_ID_DF1_INC) }); }
    else if streq(testname, b"ip_id_df1_fixed\0") { send_case!(txfd, &mut daddr, FIN_PKT.as_mut_ptr(), true, { send_flush_id_case(txfd, &mut daddr, FLUSH_ID_DF1_FIXED) }); }
    else if streq(testname, b"ip_id_df0_inc\0") { send_case!(txfd, &mut daddr, FIN_PKT.as_mut_ptr(), true, { send_flush_id_case(txfd, &mut daddr, FLUSH_ID_DF0_INC) }); }
    else if streq(testname, b"ip_id_df0_fixed\0") { send_case!(txfd, &mut daddr, FIN_PKT.as_mut_ptr(), true, { send_flush_id_case(txfd, &mut daddr, FLUSH_ID_DF0_FIXED) }); }
    else if streq(testname, b"ip_id_df1_inc_fixed\0") { send_case!(txfd, &mut daddr, FIN_PKT.as_mut_ptr(), true, { send_flush_id_case(txfd, &mut daddr, FLUSH_ID_DF1_INC_FIXED) }); }
    else if streq(testname, b"ip_id_df1_fixed_inc\0") { send_case!(txfd, &mut daddr, FIN_PKT.as_mut_ptr(), true, { send_flush_id_case(txfd, &mut daddr, FLUSH_ID_DF1_FIXED_INC) }); }
    else if streq(testname, b"ip_frag6\0") { send_case!(txfd, &mut daddr, FIN_PKT.as_mut_ptr(), true, { send_fragment6(txfd, &mut daddr) }); }
    else if streq(testname, b"ip_v6ext_same\0") { send_case!(txfd, &mut daddr, FIN_PKT.as_mut_ptr(), true, { send_ipv6_exthdr(txfd, &mut daddr, s!("\x00\x00\x00\x00\x00\x00") as *mut c_char, s!("\x00\x00\x00\x00\x00\x00") as *mut c_char) }); }
    else if streq(testname, b"ip_v6ext_diff\0") { send_case!(txfd, &mut daddr, FIN_PKT.as_mut_ptr(), true, { send_ipv6_exthdr(txfd, &mut daddr, s!("\x00\x00\x00\x00\x00\x00") as *mut c_char, s!("\x11\x11\x11\x11\x11\x11") as *mut c_char) }); }
    else if streq(testname, b"large_max\0") { let remainder = max_payload() % calc_mss(); send_case!(txfd, &mut daddr, FIN_PKT.as_mut_ptr(), false, { send_large(txfd, &mut daddr, remainder) }); }
    else if streq(testname, b"large_rem\0") { let remainder = max_payload() % calc_mss(); send_case!(txfd, &mut daddr, FIN_PKT.as_mut_ptr(), false, { send_large(txfd, &mut daddr, remainder + 1) }); }
    else if streq(testname, b"single\0") { static mut BUF: [c_char; MAX_HDR_LEN + PAYLOAD_LEN as usize] = [0; MAX_HDR_LEN + PAYLOAD_LEN as usize]; create_packet(BUF.as_mut_ptr() as *mut c_void,0,0,PAYLOAD_LEN,0); write_packet(txfd,BUF.as_mut_ptr(),total_hdr_len+PAYLOAD_LEN,&mut daddr); write_packet(txfd, FIN_PKT.as_mut_ptr(), total_hdr_len, &mut daddr); }
    else if streq(testname, b"capacity\0") { send_case!(txfd, &mut daddr, FIN_PKT.as_mut_ptr(), true, { send_capacity(txfd, &mut daddr) }); }
    else if streq(testname, b"pppoe_sid\0") { send_case!(txfd, &mut daddr, FIN_PKT.as_mut_ptr(), true, { send_changed_pppoe_sid(txfd, &mut daddr) }); }
    else { error(1, 0, s!("Unknown testcase: %s"), testname); }
    if close(txfd) != 0 { error(1, errno, s!("socket close")); }
}

unsafe fn expect_payload(rxfd: c_int, vals: &[c_int], msg: *const c_char) {
    let mut correct_payload = [0; NUM_PACKETS];
    for (i, v) in vals.iter().enumerate() { correct_payload[i] = *v; }
    if !msg.is_null() { printf(msg); }
    check_recv_pkts(rxfd, correct_payload.as_mut_ptr(), vals.len() as c_int);
}

unsafe fn gro_receiver() {
    let rxfd = socket(PF_PACKET, SOCK_RAW, htons(ETH_P_NONE as u16) as c_int);
    if rxfd < 0 { error(1, 0, s!("socket creation")); }
    setup_sock_filter(rxfd); set_timeout(rxfd); set_rcvbuf(rxfd); bind_packetsocket(rxfd); ksft_ready();
    if streq(testname,b"data_same\0") { expect_payload(rxfd,&[PAYLOAD_LEN*2],s!("pure data packet of same size: ")); }
    else if streq(testname,b"data_lrg_sml\0") { expect_payload(rxfd,&[(PAYLOAD_LEN as f64 * 1.5) as c_int],s!("large data packets followed by a smaller one: ")); }
    else if streq(testname,b"data_lrg_1byte\0") { expect_payload(rxfd,&[PAYLOAD_LEN+1],s!("large data packet followed by a 1 byte one: ")); }
    else if streq(testname,b"data_sml_lrg\0") { expect_payload(rxfd,&[PAYLOAD_LEN/2,PAYLOAD_LEN],s!("small data packets followed by a larger one: ")); }
    else if streq(testname,b"data_burst\0") { expect_payload(rxfd,&[PAYLOAD_LEN*2,PAYLOAD_LEN*2],s!("two bursts of two data packets: ")); }
    else if streq(testname,b"ack\0") { expect_payload(rxfd,&[0,0,0],s!("duplicate ack and pure ack: ")); }
    else if streq(testname,b"flags_psh\0") { expect_payload(rxfd,&[PAYLOAD_LEN*3,PAYLOAD_LEN*2],s!("psh flag ends coalescing: ")); }
    else if streq(testname,b"flags_syn\0") { expect_payload(rxfd,&[PAYLOAD_LEN*2,0,PAYLOAD_LEN*2],s!("syn flag ends coalescing: ")); }
    else if streq(testname,b"flags_rst\0") { expect_payload(rxfd,&[PAYLOAD_LEN*2,0,PAYLOAD_LEN*2],s!("rst flag ends coalescing: ")); }
    else if streq(testname,b"flags_urg\0") { expect_payload(rxfd,&[PAYLOAD_LEN*2,0,PAYLOAD_LEN*2],s!("urg flag ends coalescing: ")); }
    else if streq(testname,b"flags_cwr\0") { expect_payload(rxfd,&[PAYLOAD_LEN,PAYLOAD_LEN*2,PAYLOAD_LEN*2],s!("cwr flag ends coalescing: ")); }
    else if streq(testname,b"tcp_csum\0") { expect_payload(rxfd,&[PAYLOAD_LEN,PAYLOAD_LEN],s!("changed checksum does not coalesce: ")); }
    else if streq(testname,b"tcp_seq\0") { expect_payload(rxfd,&[PAYLOAD_LEN,PAYLOAD_LEN],s!("Wrong Seq number doesn't coalesce: ")); }
    else if streq(testname,b"tcp_ts\0") { expect_payload(rxfd,&[PAYLOAD_LEN*2,PAYLOAD_LEN,PAYLOAD_LEN,PAYLOAD_LEN],s!("Different timestamp doesn't coalesce: ")); }
    else if streq(testname,b"tcp_opt\0") { expect_payload(rxfd,&[PAYLOAD_LEN*2,PAYLOAD_LEN],s!("Different options doesn't coalesce: ")); }
    else if streq(testname,b"ip_ecn\0") { expect_payload(rxfd,&[PAYLOAD_LEN,PAYLOAD_LEN],s!("different ECN doesn't coalesce: ")); }
    else if streq(testname,b"ip_tos\0") { expect_payload(rxfd,&[PAYLOAD_LEN,PAYLOAD_LEN],s!("different tos doesn't coalesce: ")); }
    else if streq(testname,b"ip_csum\0") { expect_payload(rxfd,&[PAYLOAD_LEN,PAYLOAD_LEN,PAYLOAD_LEN],s!("bad ip checksum doesn't coalesce: ")); }
    else if streq(testname,b"ip_ttl\0") { expect_payload(rxfd,&[PAYLOAD_LEN,PAYLOAD_LEN],s!("different ttl doesn't coalesce: ")); }
    else if streq(testname,b"ip_opt\0") { expect_payload(rxfd,&[PAYLOAD_LEN,PAYLOAD_LEN,PAYLOAD_LEN],s!("ip options doesn't coalesce: ")); }
    else if streq(testname,b"ip_frag4\0") { expect_payload(rxfd,&[PAYLOAD_LEN,PAYLOAD_LEN],s!("fragmented ip4 doesn't coalesce: ")); }
    else if streq(testname,b"ip_id_df1_inc\0") { expect_payload(rxfd,&[PAYLOAD_LEN*2],s!("DF=1, Incrementing - should coalesce: ")); }
    else if streq(testname,b"ip_id_df1_fixed\0") { expect_payload(rxfd,&[PAYLOAD_LEN*2],s!("DF=1, Fixed - should coalesce: ")); }
    else if streq(testname,b"ip_id_df0_inc\0") { expect_payload(rxfd,&[PAYLOAD_LEN*2],s!("DF=0, Incrementing - should coalesce: ")); }
    else if streq(testname,b"ip_id_df0_fixed\0") { expect_payload(rxfd,&[PAYLOAD_LEN*2],s!("DF=0, Fixed - should coalesce: ")); }
    else if streq(testname,b"ip_id_df1_inc_fixed\0") { expect_payload(rxfd,&[PAYLOAD_LEN*2,PAYLOAD_LEN],s!("DF=1, 2 Incrementing and one fixed - should coalesce only first 2 packets: ")); }
    else if streq(testname,b"ip_id_df1_fixed_inc\0") { expect_payload(rxfd,&[PAYLOAD_LEN*2,PAYLOAD_LEN],s!("DF=1, 2 Fixed and one incrementing - should coalesce only first 2 packets: ")); }
    else if streq(testname,b"ip_frag6\0") { expect_payload(rxfd,&[PAYLOAD_LEN*2,PAYLOAD_LEN,PAYLOAD_LEN],s!("fragmented ip6 doesn't coalesce: ")); }
    else if streq(testname,b"ip_v6ext_same\0") { expect_payload(rxfd,&[PAYLOAD_LEN*2],s!("ipv6 with ext header does coalesce: ")); }
    else if streq(testname,b"ip_v6ext_diff\0") { expect_payload(rxfd,&[PAYLOAD_LEN,PAYLOAD_LEN],s!("ipv6 with ext header with different payloads doesn't coalesce: ")); }
    else if streq(testname,b"large_max\0") { let remainder=max_payload()%calc_mss(); expect_payload(rxfd,&[max_payload(),remainder],s!("Shouldn't coalesce if exceed IP max pkt size: ")); }
    else if streq(testname,b"large_rem\0") { let remainder=max_payload()%calc_mss(); expect_payload(rxfd,&[max_payload()-remainder,remainder+1,remainder+1],s!("last segment sent individually: ")); }
    else if streq(testname,b"single\0") { expect_payload(rxfd,&[PAYLOAD_LEN],s!("single data packet: ")); }
    else if streq(testname,b"capacity\0") { check_capacity_pkts(rxfd); }
    else if streq(testname,b"pppoe_sid\0") { expect_payload(rxfd,&[PAYLOAD_LEN,PAYLOAD_LEN],s!("different PPPoE session ID doesn't coalesce: ")); }
    else { error(1, 0, s!("Test case error: unknown testname %s"), testname); }
    if close(rxfd) != 0 { error(1, 0, s!("socket close")); }
}

unsafe fn parse_args(argc: c_int, argv: *mut *mut c_char) {
    let opts = [
        option { name: s!("daddr"), has_arg: required_argument, flag: ptr::null_mut(), val: 'd' as c_int },
        option { name: s!("dmac"), has_arg: required_argument, flag: ptr::null_mut(), val: 'D' as c_int },
        option { name: s!("iface"), has_arg: required_argument, flag: ptr::null_mut(), val: 'i' as c_int },
        option { name: s!("ipv4"), has_arg: no_argument, flag: ptr::null_mut(), val: '4' as c_int },
        option { name: s!("ipv6"), has_arg: no_argument, flag: ptr::null_mut(), val: '6' as c_int },
        option { name: s!("ipip"), has_arg: no_argument, flag: ptr::null_mut(), val: 'e' as c_int },
        option { name: s!("ip6ip6"), has_arg: no_argument, flag: ptr::null_mut(), val: 'E' as c_int },
        option { name: s!("pppoev4"), has_arg: no_argument, flag: ptr::null_mut(), val: 'p' as c_int },
        option { name: s!("pppoev6"), has_arg: no_argument, flag: ptr::null_mut(), val: 'P' as c_int },
        option { name: s!("num-flows"), has_arg: required_argument, flag: ptr::null_mut(), val: 'n' as c_int },
        option { name: s!("rx"), has_arg: no_argument, flag: ptr::null_mut(), val: 'r' as c_int },
        option { name: s!("saddr"), has_arg: required_argument, flag: ptr::null_mut(), val: 's' as c_int },
        option { name: s!("smac"), has_arg: required_argument, flag: ptr::null_mut(), val: 'S' as c_int },
        option { name: s!("test"), has_arg: required_argument, flag: ptr::null_mut(), val: 't' as c_int },
        option { name: s!("order-check"), has_arg: no_argument, flag: ptr::null_mut(), val: 'o' as c_int },
        option { name: s!("verbose"), has_arg: no_argument, flag: ptr::null_mut(), val: 'v' as c_int },
        option { name: ptr::null(), has_arg: 0, flag: ptr::null_mut(), val: 0 },
    ];
    loop {
        let c = getopt_long(argc, argv, s!("46d:D:eEi:n:pPrs:S:t:ov"), opts.as_ptr(), ptr::null_mut());
        if c == -1 { break; }
        match c {
            x if x == '4' as c_int => { proto = PF_INET; ethhdr_proto = htons(ETH_P_IP as u16) as c_int; }
            x if x == '6' as c_int => { proto = PF_INET6; ethhdr_proto = htons(ETH_P_IPV6 as u16) as c_int; }
            x if x == 'e' as c_int => { ipip = true; proto = PF_INET; ethhdr_proto = htons(ETH_P_IP as u16) as c_int; }
            x if x == 'E' as c_int => { ip6ip6 = true; proto = PF_INET6; ethhdr_proto = htons(ETH_P_IPV6 as u16) as c_int; }
            x if x == 'p' as c_int => { pppoe = true; proto = PF_INET; ethhdr_proto = htons(ETH_P_PPP_SES as u16) as c_int; }
            x if x == 'P' as c_int => { pppoe = true; proto = PF_INET6; ethhdr_proto = htons(ETH_P_PPP_SES as u16) as c_int; }
            x if x == 'd' as c_int => { addr4_dst = optarg; addr6_dst = optarg; }
            x if x == 'D' as c_int => dmac = optarg,
            x if x == 'i' as c_int => ifname = optarg,
            x if x == 'n' as c_int => num_flows = atoi(optarg),
            x if x == 'r' as c_int => tx_socket = false,
            x if x == 's' as c_int => { addr4_src = optarg; addr6_src = optarg; }
            x if x == 'S' as c_int => smac = optarg,
            x if x == 't' as c_int => testname = optarg,
            x if x == 'o' as c_int => order_check = true,
            x if x == 'v' as c_int => verbose = true,
            _ => error(1, 0, s!("%s invalid option %c\n"), s!("parse_args"), c),
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    parse_args(argc, argv);
    if ipip {
        tcp_offset = ETH_HLEN as c_int + size_of::<iphdr>() as c_int * 2;
    } else if ip6ip6 {
        tcp_offset = ETH_HLEN as c_int + size_of::<ipv6hdr>() as c_int * 2;
    } else if pppoe {
        tcp_offset = ETH_HLEN as c_int + PPPOE_SES_HLEN as c_int + if proto == PF_INET { size_of::<iphdr>() as c_int } else { size_of::<ipv6hdr>() as c_int };
    } else if proto == PF_INET {
        tcp_offset = ETH_HLEN as c_int + size_of::<iphdr>() as c_int;
    } else if proto == PF_INET6 {
        tcp_offset = ETH_HLEN as c_int + size_of::<ipv6hdr>() as c_int;
    } else {
        error(1, 0, s!("Protocol family is not ipv4 or ipv6"));
    }
    total_hdr_len = tcp_offset + size_of::<tcphdr>() as c_int;
    read_MAC(src_mac.as_mut_ptr(), smac);
    read_MAC(dst_mac.as_mut_ptr(), dmac);
    if tx_socket { gro_sender(); } else { gro_receiver(); fprintf(stderr, s!("Gro::%s test passed.\n"), testname); }
    0
}
