// SPDX-License-Identifier: GPL-2.0
/* Toeplitz test
 *
 * 1. Read packets and their rx_hash using PF_PACKET/TPACKET_V3
 * 2. Compute the rx_hash in software based on the packet contents
 * 3. Compare the two
 *
 * Optionally, either '-C $rx_irq_cpu_list' or '-r $rps_bitmap' may be given.
 *
 * If '-C $rx_irq_cpu_list' is given, also
 *
 * 4. Identify the cpu on which the packet arrived with PACKET_FANOUT_CPU
 * 5. Compute the rxqueue that RSS would select based on this rx_hash
 * 6. Using the $rx_irq_cpu_list map, identify the arriving cpu based on rxq irq
 * 7. Compare the cpus from 4 and 6
 *
 * Else if '-r $rps_bitmap' is given, also
 *
 * 4. Identify the cpu on which the packet arrived with PACKET_FANOUT_CPU
 * 5. Compute the cpu that RPS should select based on rx_hash and $rps_bitmap
 * 6. Compare the cpus from 4 and 5
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type uint8_t = u8;
type uint16_t = u16;
type uint32_t = u32;
type uint64_t = u64;
type socklen_t = u32;
type size_t = usize;
type ssize_t = isize;
type off_t = c_long;

const TOEPLITZ_KEY_MIN_LEN: usize = 40;
const TOEPLITZ_KEY_MAX_LEN: usize = 256;

const fn TOEPLITZ_STR_LEN(k: usize) -> usize {
    (k * 3) - 1
}
const TOEPLITZ_STR_MIN_LEN: usize = TOEPLITZ_STR_LEN(TOEPLITZ_KEY_MIN_LEN);
const TOEPLITZ_STR_MAX_LEN: usize = TOEPLITZ_STR_LEN(TOEPLITZ_KEY_MAX_LEN);

const INET6_ADDRSTRLEN: usize = 46;
const FOUR_TUPLE_MAX_LEN: usize = (size_of::<in6_addr>() * 2) + (size_of::<uint16_t>() * 2);

const RSS_MAX_CPUS: usize = 1 << 16; /* real constraint is PACKET_FANOUT_MAX */
const RSS_MAX_INDIR: usize = 1 << 16;

const RPS_MAX_CPUS: usize = 16; /* must be a power of 2 */

const MIN_PKT_SAMPLES: c_int = 40; /* minimum number of packets to receive */

const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const AF_PACKET: c_int = 17;
const PF_PACKET: c_int = AF_PACKET;
const SOCK_STREAM: c_int = 1;
const SOCK_DGRAM: c_int = 2;
const SOL_SOCKET: c_int = 1;
const SOL_PACKET: c_int = 263;
const SO_ATTACH_FILTER: c_int = 26;
const SO_RCVBUFFORCE: c_int = 33;
const PACKET_RX_RING: c_int = 5;
const PACKET_VERSION: c_int = 10;
const PACKET_FANOUT: c_int = 18;
const PACKET_FANOUT_CPU: c_int = 2;
const PACKET_HOST: c_uint = 0;
const TPACKET_V3: c_int = 2;
const TP_FT_REQ_FILL_RXHASH: c_uint = 1;
const TP_STATUS_USER: c_uint = 1;
const TP_STATUS_KERNEL: c_uint = 0;
const ETH_P_IP: c_int = 0x0800;
const ETH_P_IPV6: c_int = 0x86DD;
const IPPROTO_TCP: uint8_t = 6;
const IPPROTO_UDP: uint8_t = 17;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_SHARED: c_int = 0x01;
const MAP_LOCKED: c_int = 0x2000;
const MAP_POPULATE: c_int = 0x8000;
const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

const BPF_LD: u16 = 0x00;
const BPF_B: u16 = 0x10;
const BPF_H: u16 = 0x08;
const BPF_ABS: u16 = 0x20;
const BPF_JMP: u16 = 0x05;
const BPF_JEQ: u16 = 0x10;
const BPF_K: u16 = 0x00;
const BPF_RET: u16 = 0x06;
const SKF_AD_OFF: u32 = 0xfffff000;
const SKF_AD_PKTTYPE: u32 = 4;

#[repr(C)]
struct in_addr {
    s_addr: uint32_t,
}

#[repr(C)]
struct in6_addr {
    s6_addr: [uint8_t; 16],
}

#[repr(C)]
struct iphdr {
    ihl_version: uint8_t,
    tos: uint8_t,
    tot_len: uint16_t,
    id: uint16_t,
    frag_off: uint16_t,
    ttl: uint8_t,
    protocol: uint8_t,
    check: uint16_t,
    saddr: uint32_t,
    daddr: uint32_t,
}

#[repr(C)]
struct ip6_hdr {
    ip6_flow: uint32_t,
    ip6_plen: uint16_t,
    ip6_nxt: uint8_t,
    ip6_hlim: uint8_t,
    ip6_src: in6_addr,
    ip6_dst: in6_addr,
}

#[repr(C)]
struct tcphdr {
    source: uint16_t,
    dest: uint16_t,
}

#[repr(C)]
struct timeval {
    tv_sec: c_long,
    tv_usec: c_long,
}

#[repr(C)]
struct sock_filter {
    code: u16,
    jt: u8,
    jf: u8,
    k: u32,
}

#[repr(C)]
struct sock_fprog {
    len: u16,
    filter: *mut sock_filter,
}

#[repr(C)]
struct sockaddr_ll {
    sll_family: u16,
    sll_protocol: u16,
    sll_ifindex: c_int,
    sll_hatype: u16,
    sll_pkttype: u8,
    sll_halen: u8,
    sll_addr: [u8; 8],
}

#[repr(C)]
struct tpacket_req3 {
    tp_block_size: c_uint,
    tp_block_nr: c_uint,
    tp_frame_size: c_uint,
    tp_frame_nr: c_uint,
    tp_retire_blk_tov: c_uint,
    tp_sizeof_priv: c_uint,
    tp_feature_req_word: c_uint,
}

#[repr(C)]
struct tpacket_hdr_variant1 {
    tp_rxhash: uint32_t,
    tp_vlan_tci: uint32_t,
    tp_vlan_tpid: uint16_t,
    tp_padding: uint16_t,
}

#[repr(C)]
struct tpacket3_hdr {
    tp_next_offset: uint32_t,
    tp_sec: uint32_t,
    tp_nsec: uint32_t,
    tp_snaplen: uint32_t,
    tp_len: uint32_t,
    tp_status: uint32_t,
    tp_mac: uint16_t,
    tp_net: uint16_t,
    hv1: tpacket_hdr_variant1,
}

#[repr(C)]
struct tpacket_bd_ts {
    ts_sec: c_uint,
    ts_usec: c_uint,
}

#[repr(C)]
struct tpacket_hdr_v1 {
    block_status: uint32_t,
    num_pkts: uint32_t,
    offset_to_first_pkt: uint32_t,
    blk_len: uint32_t,
    seq_num: uint64_t,
    ts_first_pkt: tpacket_bd_ts,
    ts_last_pkt: tpacket_bd_ts,
}

#[repr(C)]
union tpacket_bd_header_u {
    bh1: core::mem::ManuallyDrop<tpacket_hdr_v1>,
}

#[repr(C)]
struct tpacket_block_desc {
    version: uint32_t,
    offset_to_priv: uint32_t,
    hdr: tpacket_bd_header_u,
}

#[repr(C)]
struct fanout_args {
    id: uint16_t,
    type_flags: uint16_t,
    max_num_members: uint32_t,
}

#[repr(C)]
struct ynl_sock_err {
    code: c_int,
    msg: *const c_char,
}

#[repr(C)]
struct ynl_sock {
    err: ynl_sock_err,
}

#[repr(C)]
struct ethtool_rss_get_req {
    _private: [u8; 0],
}

#[repr(C)]
struct ethtool_rss_get_rsp_len {
    hkey: c_uint,
}

#[repr(C)]
struct ethtool_rss_get_rsp_count {
    indir: c_uint,
}

#[repr(C)]
struct ethtool_rss_get_rsp {
    _len: ethtool_rss_get_rsp_len,
    _count: ethtool_rss_get_rsp_count,
    hkey: *mut uint8_t,
    indir: *mut c_uint,
}

#[repr(C)]
struct option {
    name: *const c_char,
    has_arg: c_int,
    flag: *mut c_int,
    val: c_int,
}

const no_argument: c_int = 0;
const required_argument: c_int = 1;

unsafe extern "C" {
    static mut errno: c_int;
    static mut optarg: *mut c_char;
    static mut optopt: c_int;
    static ynl_ethtool_family: c_void;

    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn inet_ntop(af: c_int, src: *const c_void, dst: *mut c_char, size: socklen_t) -> *const c_char;
    fn ntohl(netlong: uint32_t) -> uint32_t;
    fn ntohs(netshort: uint16_t) -> uint16_t;
    fn htons(hostshort: uint16_t) -> uint16_t;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn setsockopt(socket: c_int, level: c_int, option_name: c_int, option_value: *const c_void, option_len: socklen_t) -> c_int;
    fn bind(sockfd: c_int, addr: *const c_void, addrlen: socklen_t) -> c_int;
    fn mmap(addr: *mut c_void, length: size_t, prot: c_int, flags: c_int, fd: c_int, offset: off_t) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn if_nametoindex(ifname: *const c_char) -> c_uint;
    fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;
    fn usleep(usec: c_uint) -> c_int;
    fn get_nprocs() -> c_int;
    fn getopt_long(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char, longopts: *const option, longindex: *mut c_int) -> c_int;
    fn error(status: c_int, errnum: c_int, format: *const c_char, ...) -> !;
    fn ksft_ready();

    fn ynl_sock_create(family: *const c_void, config: *mut c_void) -> *mut ynl_sock;
    fn ynl_sock_destroy(ys: *mut ynl_sock);
    fn ethtool_rss_get_req_alloc() -> *mut ethtool_rss_get_req;
    fn ethtool_rss_get_req_free(req: *mut ethtool_rss_get_req);
    fn ethtool_rss_get_req_set_header_dev_name(req: *mut ethtool_rss_get_req, dev_name: *const c_char);
    fn ethtool_rss_get(ys: *mut ynl_sock, req: *mut ethtool_rss_get_req) -> *mut ethtool_rss_get_rsp;
    fn ethtool_rss_get_rsp_free(rsp: *mut ethtool_rss_get_rsp);
}

unsafe extern "C" {
    static mut stderr: *mut c_void;
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! log_verbose {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {{
        unsafe {
            if cfg_verbose {
                fprintf(stderr, cstr!($fmt), $($arg),*);
            }
        }
    }};
}

const fn offsetof_ip_saddr() -> usize {
    12
}
const fn offsetof_ip_protocol() -> usize {
    9
}
const fn offsetof_ip6_src() -> usize {
    8
}
const fn offsetof_ip6_nxt() -> usize {
    6
}
const fn offsetof_tcphdr_dest() -> usize {
    2
}

const fn BPF_STMT(code: u16, k: u32) -> sock_filter {
    sock_filter { code, jt: 0, jf: 0, k }
}

const fn BPF_JUMP(code: u16, k: u32, jt: u8, jf: u8) -> sock_filter {
    sock_filter { code, jt, jf, k }
}

/* configuration options (cmdline arguments) */
static mut cfg_dport: uint16_t = 8000;
static mut cfg_family: c_int = AF_INET6;
static mut cfg_ifname: *mut c_char = cstr!("eth0") as *mut c_char;
static mut cfg_num_queues: c_int = 0;
static mut cfg_num_rps_cpus: c_int = 0;
static mut cfg_sink: bool = false;
static mut cfg_type: c_int = SOCK_STREAM;
static mut cfg_timeout_msec: c_int = 1000;
static mut cfg_verbose: bool = false;

/* global vars */
static mut num_cpus: c_int = 0;
static mut ring_block_nr: c_int = 0;
static mut ring_block_sz: c_int = 0;

/* stats */
static mut frames_received: c_int = 0;
static mut frames_nohash: c_int = 0;
static mut frames_error: c_int = 0;

/* tpacket ring */
#[repr(C)]
struct ring_state {
    fd: c_int,
    mmap: *mut c_char,
    idx: c_int,
    cpu: c_int,
}

static mut rx_irq_cpus: [c_uint; RSS_MAX_CPUS] = [0; RSS_MAX_CPUS]; /* map from rxq to cpu */
static mut rps_silo_to_cpu: [c_int; RPS_MAX_CPUS] = [0; RPS_MAX_CPUS];
static mut toeplitz_key: [c_uchar_alias; TOEPLITZ_KEY_MAX_LEN] = [0; TOEPLITZ_KEY_MAX_LEN];
type c_uchar_alias = u8;
static mut rss_indir_tbl: [c_uint; RSS_MAX_INDIR] = [0; RSS_MAX_INDIR];
static mut rss_indir_tbl_size: c_uint = 0;
static mut rings: [ring_state; RSS_MAX_CPUS] = [const { ring_state { fd: 0, mmap: ptr::null_mut(), idx: 0, cpu: 0 } }; RSS_MAX_CPUS];

unsafe fn toeplitz(mut four_tuple: *const c_uchar_alias, mut key: *const c_uchar_alias) -> uint32_t {
    let mut ret: c_int = 0;
    let mut key32: uint32_t;

    key32 = ntohl(*(key as *const uint32_t));
    key = key.add(4);

    let mut i = 0;
    while i < FOUR_TUPLE_MAX_LEN {
        let mut bit: c_int = 7;
        while bit >= 0 {
            if (*four_tuple.add(i) & ((1 as c_int) << bit) as u8) != 0 {
                ret ^= key32 as c_int;
            }

            key32 = key32.wrapping_shl(1);
            key32 |= ((*key & ((1 as c_int) << bit) as u8) != 0) as uint32_t;
            bit -= 1;
        }
        key = key.add(1);
        i += 1;
    }

    ret as uint32_t
}

/* Compare computed cpu with arrival cpu from packet_fanout_cpu */
unsafe fn verify_rss(rx_hash: uint32_t, cpu: c_int) {
    let queue: c_int;

    if rss_indir_tbl_size != 0 {
        queue = rss_indir_tbl[(rx_hash % rss_indir_tbl_size) as usize] as c_int;
    } else {
        queue = (rx_hash % cfg_num_queues as u32) as c_int;
    }

    log_verbose!(" rxq %d (cpu %d)", queue, rx_irq_cpus[queue as usize]);
    if rx_irq_cpus[queue as usize] != cpu as c_uint {
        log_verbose!(". error: rss cpu mismatch (%d)", cpu);
        frames_error += 1;
    }
}

unsafe fn verify_rps(rx_hash: uint64_t, cpu: c_int) {
    let silo: c_int = ((rx_hash.wrapping_mul(cfg_num_rps_cpus as u64)) >> 32) as c_int;

    log_verbose!(" silo %d (cpu %d)", silo, rps_silo_to_cpu[silo as usize]);
    if rps_silo_to_cpu[silo as usize] != cpu {
        log_verbose!(". error: rps cpu mismatch (%d)", cpu);
        frames_error += 1;
    }
}

unsafe fn log_rxhash(cpu: c_int, rx_hash: uint32_t, addrs: *const c_char, addr_len: c_int) {
    let mut saddr: [c_char; INET6_ADDRSTRLEN] = [0; INET6_ADDRSTRLEN];
    let mut daddr: [c_char; INET6_ADDRSTRLEN] = [0; INET6_ADDRSTRLEN];
    let ports: *mut uint16_t;

    if inet_ntop(cfg_family, addrs as *const c_void, saddr.as_mut_ptr(), size_of::<[c_char; INET6_ADDRSTRLEN]>() as socklen_t).is_null()
        || inet_ntop(cfg_family, addrs.add(addr_len as usize) as *const c_void, daddr.as_mut_ptr(), size_of::<[c_char; INET6_ADDRSTRLEN]>() as socklen_t).is_null()
    {
        error(1, 0, cstr!("address parse error"));
    }

    ports = addrs.add((addr_len * 2) as usize) as *mut uint16_t;
    log_verbose!(
        "cpu %d: rx_hash 0x%08x [saddr %s daddr %s sport %02hu dport %02hu]",
        cpu,
        rx_hash,
        saddr.as_ptr(),
        daddr.as_ptr(),
        ntohs(*ports.add(0)) as c_int,
        ntohs(*ports.add(1)) as c_int
    );
}

/* Compare computed rxhash with rxhash received from tpacket_v3 */
unsafe fn verify_rxhash(pkt: *const c_char, rx_hash: uint32_t, cpu: c_int) {
    let mut four_tuple: [c_uchar_alias; FOUR_TUPLE_MAX_LEN] = [0; FOUR_TUPLE_MAX_LEN];
    let rx_hash_sw: uint32_t;
    let addrs: *const c_char;
    let addr_len: c_int;

    if cfg_family == AF_INET {
        addr_len = size_of::<in_addr>() as c_int;
        addrs = pkt.add(offsetof_ip_saddr());
    } else {
        addr_len = size_of::<in6_addr>() as c_int;
        addrs = pkt.add(offsetof_ip6_src());
    }

    memcpy(
        four_tuple.as_mut_ptr() as *mut c_void,
        addrs as *const c_void,
        ((addr_len * 2) as usize) + (size_of::<uint16_t>() * 2),
    );
    rx_hash_sw = toeplitz(four_tuple.as_ptr(), toeplitz_key.as_ptr());

    if cfg_verbose {
        log_rxhash(cpu, rx_hash, addrs, addr_len);
    }

    if rx_hash != rx_hash_sw {
        log_verbose!(" != expected 0x%x\n", rx_hash_sw);
        frames_error += 1;
        return;
    }

    log_verbose!(" OK");
    if cfg_num_queues != 0 {
        verify_rss(rx_hash, cpu);
    } else if cfg_num_rps_cpus != 0 {
        verify_rps(rx_hash as uint64_t, cpu);
    }
    log_verbose!("\n");
}

unsafe fn recv_frame(ring: *const ring_state, frame: *mut c_char) -> *mut c_char {
    let hdr: *mut tpacket3_hdr = frame as *mut tpacket3_hdr;

    if (*hdr).hv1.tp_rxhash != 0 {
        verify_rxhash(frame.add((*hdr).tp_net as usize), (*hdr).hv1.tp_rxhash, (*ring).cpu);
    } else {
        frames_nohash += 1;
    }

    frame.add((*hdr).tp_next_offset as usize)
}

/* A single TPACKET_V3 block can hold multiple frames */
unsafe fn recv_block(ring: *mut ring_state) -> bool {
    let block: *mut tpacket_block_desc;
    let mut frame: *mut c_char;
    let mut i: c_int;

    block = (*ring).mmap.add(((*ring).idx * ring_block_sz) as usize) as *mut tpacket_block_desc;
    if ((*(*block).hdr.bh1).block_status & TP_STATUS_USER) == 0 {
        return false;
    }

    frame = block as *mut c_char;
    frame = frame.add((*(*block).hdr.bh1).offset_to_first_pkt as usize);

    i = 0;
    while i < (*(*block).hdr.bh1).num_pkts as c_int {
        frame = recv_frame(ring, frame);
        frames_received += 1;
        i += 1;
    }

    (*(*block).hdr.bh1).block_status = TP_STATUS_KERNEL;
    (*ring).idx = ((*ring).idx + 1) % ring_block_nr;

    true
}

/* simple test: process all rings until MIN_PKT_SAMPLES packets are received,
 * or the test times out.
 */
unsafe fn process_rings() {
    let mut start: timeval = core::mem::zeroed();
    let mut now: timeval = core::mem::zeroed();
    let mut pkts_found: bool = true;
    let mut elapsed_usec: c_long;
    let mut i: c_int;

    gettimeofday(&mut start, ptr::null_mut());

    loop {
        if !pkts_found {
            usleep(100);
        }

        pkts_found = false;
        i = 0;
        while i < num_cpus {
            pkts_found |= recv_block(&mut rings[i as usize]);
            i += 1;
        }

        gettimeofday(&mut now, ptr::null_mut());
        elapsed_usec = (now.tv_sec - start.tv_sec) * 1000000 + (now.tv_usec - start.tv_usec);

        if !(frames_received - frames_nohash < MIN_PKT_SAMPLES && elapsed_usec < (cfg_timeout_msec * 1000) as c_long) {
            break;
        }
    }

    fprintf(
        stderr,
        cstr!("count: pass=%u nohash=%u fail=%u\n"),
        frames_received - frames_nohash - frames_error,
        frames_nohash,
        frames_error,
    );
}

unsafe fn setup_ring(fd: c_int) -> *mut c_char {
    let mut req3: tpacket_req3 = core::mem::zeroed();
    let ring: *mut c_void;

    req3.tp_retire_blk_tov = (cfg_timeout_msec / 8) as c_uint;
    req3.tp_feature_req_word = TP_FT_REQ_FILL_RXHASH;

    req3.tp_frame_size = 2048;
    req3.tp_frame_nr = 1 << 10;
    req3.tp_block_nr = 16;

    req3.tp_block_size = req3.tp_frame_size * req3.tp_frame_nr;
    req3.tp_block_size /= req3.tp_block_nr;

    if setsockopt(fd, SOL_PACKET, PACKET_RX_RING, &req3 as *const _ as *const c_void, size_of::<tpacket_req3>() as socklen_t) != 0 {
        error(1, errno, cstr!("setsockopt PACKET_RX_RING"));
    }

    ring_block_sz = req3.tp_block_size as c_int;
    ring_block_nr = req3.tp_block_nr as c_int;

    ring = mmap(
        ptr::null_mut(),
        (req3.tp_block_size * req3.tp_block_nr) as size_t,
        PROT_READ | PROT_WRITE,
        MAP_SHARED | MAP_LOCKED | MAP_POPULATE,
        fd,
        0,
    );
    if ring == MAP_FAILED {
        error(1, 0, cstr!("mmap failed"));
    }

    ring as *mut c_char
}

unsafe fn __set_filter(fd: c_int, off_proto: c_int, proto: uint8_t, off_dport: c_int) {
    let mut filter = [
        BPF_STMT(BPF_LD + BPF_B + BPF_ABS, SKF_AD_OFF + SKF_AD_PKTTYPE),
        BPF_JUMP(BPF_JMP + BPF_JEQ + BPF_K, PACKET_HOST, 0, 4),
        BPF_STMT(BPF_LD + BPF_B + BPF_ABS, off_proto as u32),
        BPF_JUMP(BPF_JMP + BPF_JEQ + BPF_K, proto as u32, 0, 2),
        BPF_STMT(BPF_LD + BPF_H + BPF_ABS, off_dport as u32),
        BPF_JUMP(BPF_JMP + BPF_JEQ + BPF_K, cfg_dport as u32, 1, 0),
        BPF_STMT(BPF_RET + BPF_K, 0),
        BPF_STMT(BPF_RET + BPF_K, 0xFFFF),
    ];
    let mut prog: sock_fprog = core::mem::zeroed();

    prog.filter = filter.as_mut_ptr();
    prog.len = filter.len() as u16;
    if setsockopt(fd, SOL_SOCKET, SO_ATTACH_FILTER, &prog as *const _ as *const c_void, size_of::<sock_fprog>() as socklen_t) != 0 {
        error(1, errno, cstr!("setsockopt filter"));
    }
}

/* filter on transport protocol and destination port */
unsafe fn set_filter(fd: c_int) {
    let off_dport: c_int = offsetof_tcphdr_dest() as c_int; /* same for udp */
    let proto: uint8_t;

    proto = if cfg_type == SOCK_STREAM { IPPROTO_TCP } else { IPPROTO_UDP };
    if cfg_family == AF_INET {
        __set_filter(fd, offsetof_ip_protocol() as c_int, proto, size_of::<iphdr>() as c_int + off_dport);
    } else {
        __set_filter(fd, offsetof_ip6_nxt() as c_int, proto, size_of::<ip6_hdr>() as c_int + off_dport);
    }
}

/* drop everything: used temporarily during setup */
unsafe fn set_filter_null(fd: c_int) {
    let mut filter = [
        BPF_STMT(BPF_RET + BPF_K, 0),
    ];
    let mut prog: sock_fprog = core::mem::zeroed();

    prog.filter = filter.as_mut_ptr();
    prog.len = filter.len() as u16;
    if setsockopt(fd, SOL_SOCKET, SO_ATTACH_FILTER, &prog as *const _ as *const c_void, size_of::<sock_fprog>() as socklen_t) != 0 {
        error(1, errno, cstr!("setsockopt filter"));
    }
}

unsafe fn create_ring(ring: *mut *mut c_char) -> c_int {
    let args = fanout_args {
        id: 1,
        type_flags: PACKET_FANOUT_CPU as uint16_t,
        max_num_members: RSS_MAX_CPUS as uint32_t,
    };
    let mut ll: sockaddr_ll = core::mem::zeroed();
    let fd: c_int;
    let mut val: c_int;

    fd = socket(PF_PACKET, SOCK_DGRAM, 0);
    if fd == -1 {
        error(1, errno, cstr!("socket creation failed"));
    }

    val = TPACKET_V3;
    if setsockopt(fd, SOL_PACKET, PACKET_VERSION, &val as *const _ as *const c_void, size_of::<c_int>() as socklen_t) != 0 {
        error(1, errno, cstr!("setsockopt PACKET_VERSION"));
    }
    *ring = setup_ring(fd);

    /* block packets until all rings are added to the fanout group:
     * else packets can arrive during setup and get misclassified
     */
    set_filter_null(fd);

    ll.sll_family = AF_PACKET as u16;
    ll.sll_ifindex = if_nametoindex(cfg_ifname) as c_int;
    ll.sll_protocol = if cfg_family == AF_INET { htons(ETH_P_IP as uint16_t) } else { htons(ETH_P_IPV6 as uint16_t) };
    if bind(fd, &ll as *const _ as *const c_void, size_of::<sockaddr_ll>() as socklen_t) != 0 {
        error(1, errno, cstr!("bind"));
    }

    /* must come after bind: verifies all programs in group match */
    if setsockopt(fd, SOL_PACKET, PACKET_FANOUT, &args as *const _ as *const c_void, size_of::<fanout_args>() as socklen_t) != 0 {
        /* on failure, retry using old API if that is sufficient:
         * it has a hard limit of 256 sockets, so only try if
         * (a) only testing rxhash, not RSS or (b) <= 256 cpus.
         * in this API, the third argument is left implicit.
         */
        if cfg_num_queues != 0
            || num_cpus > 256
            || setsockopt(fd, SOL_PACKET, PACKET_FANOUT, &args as *const _ as *const c_void, size_of::<uint32_t>() as socklen_t) != 0
        {
            error(1, errno, cstr!("setsockopt PACKET_FANOUT cpu"));
        }
    }

    fd
}

/* setup inet(6) socket to blackhole the test traffic, if arg '-s' */
unsafe fn setup_sink() -> c_int {
    let fd: c_int;
    let mut val: c_int;

    fd = socket(cfg_family, cfg_type, 0);
    if fd == -1 {
        error(1, errno, cstr!("socket %d.%d"), cfg_family, cfg_type);
    }

    val = 1 << 20;
    if setsockopt(fd, SOL_SOCKET, SO_RCVBUFFORCE, &val as *const _ as *const c_void, size_of::<c_int>() as socklen_t) != 0 {
        error(1, errno, cstr!("setsockopt rcvbuf"));
    }

    fd
}

unsafe fn setup_rings() {
    let mut i: c_int = 0;

    while i < num_cpus {
        rings[i as usize].cpu = i;
        rings[i as usize].fd = create_ring(&mut rings[i as usize].mmap);
        i += 1;
    }

    /* accept packets once all rings in the fanout group are up */
    i = 0;
    while i < num_cpus {
        set_filter(rings[i as usize].fd);
        i += 1;
    }
}

unsafe fn cleanup_rings() {
    let mut i: c_int = 0;

    while i < num_cpus {
        if munmap(rings[i as usize].mmap as *mut c_void, (ring_block_nr * ring_block_sz) as size_t) != 0 {
            error(1, errno, cstr!("munmap"));
        }
        if close(rings[i as usize].fd) != 0 {
            error(1, errno, cstr!("close"));
        }
        i += 1;
    }
}

unsafe fn parse_cpulist(mut arg: *const c_char) {
    loop {
        rx_irq_cpus[cfg_num_queues as usize] = strtol(arg, ptr::null_mut(), 10) as c_uint;
        cfg_num_queues += 1;

        arg = strchr(arg, ',' as c_int);
        if arg.is_null() {
            break;
        }
        arg = arg.add(1); // skip ','
    }
}

unsafe fn show_cpulist() {
    let mut i: c_int = 0;

    while i < cfg_num_queues {
        fprintf(stderr, cstr!("rxq %d: cpu %d\n"), i, rx_irq_cpus[i as usize]);
        i += 1;
    }
}

unsafe fn show_silos() {
    let mut i: c_int = 0;

    while i < cfg_num_rps_cpus {
        fprintf(stderr, cstr!("silo %d: cpu %d\n"), i, rps_silo_to_cpu[i as usize]);
        i += 1;
    }
}

unsafe fn parse_toeplitz_key(str_: *const c_char, slen: c_int, key: *mut c_uchar_alias) {
    let mut i: c_int;
    let mut ret: c_int;
    let mut off: c_int;

    if slen < TOEPLITZ_STR_MIN_LEN as c_int || slen > (TOEPLITZ_STR_MAX_LEN + 1) as c_int {
        error(1, 0, cstr!("invalid toeplitz key"));
    }

    i = 0;
    off = 0;
    while off < slen {
        ret = sscanf(str_.add(off as usize), cstr!("%hhx"), key.add(i as usize));
        if ret != 1 {
            error(1, 0, cstr!("key parse error at %d off %d len %d"), i, off, slen);
        }
        i += 1;
        off += 3;
    }
}

unsafe fn parse_rps_bitmap(arg: *const c_char) {
    let bitmap: c_ulong;
    let mut i: c_int;

    bitmap = strtoul(arg, ptr::null_mut(), 0);

    if (bitmap & !(((1 as c_ulong) << RPS_MAX_CPUS) - 1)) != 0 {
        error(1, 0, cstr!("rps bitmap 0x%lx out of bounds, max cpu %lu"), bitmap, (RPS_MAX_CPUS - 1) as c_ulong);
    }

    i = 0;
    while i < RPS_MAX_CPUS as c_int {
        if (bitmap & ((1 as c_ulong) << i)) != 0 {
            rps_silo_to_cpu[cfg_num_rps_cpus as usize] = i;
            cfg_num_rps_cpus += 1;
        }
        i += 1;
    }
}

unsafe fn read_rss_dev_info_ynl() {
    let req: *mut ethtool_rss_get_req;
    let rsp: *mut ethtool_rss_get_rsp;
    let ys: *mut ynl_sock;

    ys = ynl_sock_create(&ynl_ethtool_family as *const _ as *const c_void, ptr::null_mut());
    if ys.is_null() {
        error(1, errno, cstr!("ynl_sock_create failed"));
    }

    req = ethtool_rss_get_req_alloc();
    if req.is_null() {
        error(1, errno, cstr!("ethtool_rss_get_req_alloc failed"));
    }

    ethtool_rss_get_req_set_header_dev_name(req, cfg_ifname);

    rsp = ethtool_rss_get(ys, req);
    if rsp.is_null() {
        error(1, (*ys).err.code, cstr!("YNL: %s"), (*ys).err.msg);
    }

    if (*rsp)._len.hkey == 0 {
        error(1, 0, cstr!("RSS key not available for %s"), cfg_ifname);
    }

    if (*rsp)._len.hkey < TOEPLITZ_KEY_MIN_LEN as c_uint || (*rsp)._len.hkey > TOEPLITZ_KEY_MAX_LEN as c_uint {
        error(
            1,
            0,
            cstr!("RSS key length %u out of bounds [%u, %u]"),
            (*rsp)._len.hkey,
            TOEPLITZ_KEY_MIN_LEN as c_uint,
            TOEPLITZ_KEY_MAX_LEN as c_uint,
        );
    }

    memcpy(toeplitz_key.as_mut_ptr() as *mut c_void, (*rsp).hkey as *const c_void, (*rsp)._len.hkey as size_t);

    if (*rsp)._count.indir > RSS_MAX_INDIR as c_uint {
        error(
            1,
            0,
            cstr!("RSS indirection table too large (%u > %u)"),
            (*rsp)._count.indir,
            RSS_MAX_INDIR as c_uint,
        );
    }

    /* If indir table not available we'll fallback to simple modulo math */
    if (*rsp)._count.indir != 0 {
        memcpy(
            rss_indir_tbl.as_mut_ptr() as *mut c_void,
            (*rsp).indir as *const c_void,
            ((*rsp)._count.indir as usize) * size_of::<c_uint>(),
        );
        rss_indir_tbl_size = (*rsp)._count.indir;

        log_verbose!("RSS indirection table size: %u\n", rss_indir_tbl_size);
    }

    ethtool_rss_get_rsp_free(rsp);
    ethtool_rss_get_req_free(req);
    ynl_sock_destroy(ys);
}

unsafe fn parse_opts(argc: c_int, argv: *mut *mut c_char) {
    let mut long_options = [
        option { name: cstr!("dport"), has_arg: required_argument, flag: ptr::null_mut(), val: 'd' as c_int },
        option { name: cstr!("cpus"), has_arg: required_argument, flag: ptr::null_mut(), val: 'C' as c_int },
        option { name: cstr!("key"), has_arg: required_argument, flag: ptr::null_mut(), val: 'k' as c_int },
        option { name: cstr!("iface"), has_arg: required_argument, flag: ptr::null_mut(), val: 'i' as c_int },
        option { name: cstr!("ipv4"), has_arg: no_argument, flag: ptr::null_mut(), val: '4' as c_int },
        option { name: cstr!("ipv6"), has_arg: no_argument, flag: ptr::null_mut(), val: '6' as c_int },
        option { name: cstr!("sink"), has_arg: no_argument, flag: ptr::null_mut(), val: 's' as c_int },
        option { name: cstr!("tcp"), has_arg: no_argument, flag: ptr::null_mut(), val: 't' as c_int },
        option { name: cstr!("timeout"), has_arg: required_argument, flag: ptr::null_mut(), val: 'T' as c_int },
        option { name: cstr!("udp"), has_arg: no_argument, flag: ptr::null_mut(), val: 'u' as c_int },
        option { name: cstr!("verbose"), has_arg: no_argument, flag: ptr::null_mut(), val: 'v' as c_int },
        option { name: cstr!("rps"), has_arg: required_argument, flag: ptr::null_mut(), val: 'r' as c_int },
        option { name: ptr::null(), has_arg: 0, flag: ptr::null_mut(), val: 0 },
    ];
    let mut have_toeplitz: bool = false;
    let mut index: c_int = 0;
    let mut c: c_int;

    loop {
        c = getopt_long(argc, argv, cstr!("46C:d:i:k:r:stT:uv"), long_options.as_mut_ptr(), &mut index);
        if c == -1 {
            break;
        }
        match c {
            x if x == '4' as c_int => {
                cfg_family = AF_INET;
            }
            x if x == '6' as c_int => {
                cfg_family = AF_INET6;
            }
            x if x == 'C' as c_int => {
                parse_cpulist(optarg);
            }
            x if x == 'd' as c_int => {
                cfg_dport = strtol(optarg, ptr::null_mut(), 0) as uint16_t;
            }
            x if x == 'i' as c_int => {
                cfg_ifname = optarg;
            }
            x if x == 'k' as c_int => {
                parse_toeplitz_key(optarg, strlen(optarg) as c_int, toeplitz_key.as_mut_ptr());
                have_toeplitz = true;
            }
            x if x == 'r' as c_int => {
                parse_rps_bitmap(optarg);
            }
            x if x == 's' as c_int => {
                cfg_sink = true;
            }
            x if x == 't' as c_int => {
                cfg_type = SOCK_STREAM;
            }
            x if x == 'T' as c_int => {
                cfg_timeout_msec = strtol(optarg, ptr::null_mut(), 0) as c_int;
            }
            x if x == 'u' as c_int => {
                cfg_type = SOCK_DGRAM;
            }
            x if x == 'v' as c_int => {
                cfg_verbose = true;
            }
            _ => {
                error(1, 0, cstr!("unknown option %c"), optopt);
            }
        }
    }

    if !have_toeplitz {
        read_rss_dev_info_ynl();
    }

    num_cpus = get_nprocs();
    if num_cpus > RSS_MAX_CPUS as c_int {
        error(1, 0, cstr!("increase RSS_MAX_CPUS"));
    }

    if cfg_num_queues != 0 && cfg_num_rps_cpus != 0 {
        error(1, 0, cstr!("Can't supply both RSS cpus ('-C') and RPS map ('-r')"));
    }
    if cfg_verbose {
        show_cpulist();
        show_silos();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let min_tests: c_int = 10;
    let mut fd_sink: c_int = -1;

    parse_opts(argc, argv);

    if cfg_sink {
        fd_sink = setup_sink();
    }

    setup_rings();

    /* Signal to test framework that we're ready to receive */
    ksft_ready();

    process_rings();
    cleanup_rings();

    if cfg_sink && close(fd_sink) != 0 {
        error(1, errno, cstr!("close sink"));
    }

    if frames_received - frames_nohash < min_tests {
        error(1, 0, cstr!("too few frames for verification"));
    }

    frames_error
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
