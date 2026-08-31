// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

// Translated from bench_xdp_lb.c. C include dependencies are expected to be
// supplied by the surrounding repository/bindings.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{copy_nonoverlapping, null_mut};

type __u8 = u8;
type __u16 = u16;
type __u32 = u32;
type __u64 = u64;
type __be32 = u32;
type error_t = c_int;

const ETH_ALEN: usize = 6;
const BENCH_NR_CPUS: usize = 512;
const DEFAULT_LRU_SIZE: __u32 = 1024;
const CH_RINGS_SIZE: c_int = 65537;
const STATS_SIZE: __u32 = 16;
const STATS_XDP_TX: __u32 = 0;
const STATS_XDP_PASS: __u32 = 1;
const STATS_XDP_DROP: __u32 = 2;
const STATS_LRU: __u32 = 3;
const STATS_LRU_MISS: __u32 = 4;
const BPF_ANY: __u64 = 0;
const BPF_MAP_TYPE_LRU_HASH: c_int = 9;
const XDP_DROP: c_int = 1;
const XDP_PASS: c_int = 2;
const XDP_TX: c_int = 3;
const IPPROTO_TCP: __u8 = 6;
const IPPROTO_UDP: __u8 = 17;
const IPPROTO_ICMP: __u8 = 1;
const IPPROTO_IPV6: __u8 = 41;
const IPPROTO_IPIP: __u8 = 4;
const ETH_P_IP: __u16 = 0x0800;
const ETH_P_IPV6: __u16 = 0x86dd;
const ETH_P_ARP: __u16 = 0x0806;
const F_LRU_BYPASS: __u32 = 1;
const F_IPV6: __u32 = 1;

const fn IP4(a: __u32, b: __u32, c: __u32, d: __u32) -> __u32 {
    (a << 24) | (b << 16) | (c << 8) | d
}

const fn IP6(a: __u32, b: __u32, c: __u32, d: __u32) -> [__u32; 4] {
    [a, b, c, d]
}

const TNL_DST: __u32 = IP4(192, 168, 1, 2);
const REAL_INDEX: __u32 = 1;
const REAL_INDEX_V6: __u32 = 2;
const MAX_PKT_SIZE: usize = 256;
const IP_MF: __u16 = 0x2000;
const MAX_ENCAP_SIZE: usize = MAX_PKT_SIZE + size_of::<ipv6hdr>();

static TNL_DST_V6: [__u32; 4] = [0xfd000000, 0, 0, 2];

static LB_MAC: [__u8; ETH_ALEN] = [0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff];
static CLIENT_MAC: [__u8; ETH_ALEN] = [0x11, 0x22, 0x33, 0x44, 0x55, 0x66];
static ROUTER_MAC: [__u8; ETH_ALEN] = [0xde, 0xad, 0xbe, 0xef, 0x00, 0x01];

#[repr(C)]
#[derive(Clone, Copy)]
enum scenario_id {
    S_TCP_V4_LRU_HIT,
    S_TCP_V4_CH,
    S_TCP_V6_LRU_HIT,
    S_TCP_V6_CH,
    S_UDP_V4_LRU_HIT,
    S_UDP_V6_LRU_HIT,
    S_TCP_V4V6_LRU_HIT,
    S_TCP_V4_LRU_DIVERSE,
    S_TCP_V4_CH_DIVERSE,
    S_TCP_V6_LRU_DIVERSE,
    S_TCP_V6_CH_DIVERSE,
    S_UDP_V4_LRU_DIVERSE,
    S_TCP_V4_LRU_MISS,
    S_UDP_V4_LRU_MISS,
    S_TCP_V4_LRU_WARMUP,
    S_TCP_V4_SYN,
    S_TCP_V4_RST_MISS,
    S_PASS_V4_NO_VIP,
    S_PASS_V6_NO_VIP,
    S_PASS_V4_ICMP,
    S_PASS_NON_IP,
    S_DROP_V4_FRAG,
    S_DROP_V4_OPTIONS,
    S_DROP_V6_FRAG,
    NUM_SCENARIOS,
}

const NUM_SCENARIOS: usize = scenario_id::NUM_SCENARIOS as usize;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum lru_miss_type {
    LRU_MISS_AUTO = 0, /* compute from scenario flags (default) */
    LRU_MISS_NONE,     /* 0 misses (all LRU hits) */
    LRU_MISS_ALL,      /* batch_iters+1 misses (every op misses) */
    LRU_MISS_FIRST,    /* 1 miss (first miss, then hits) */
}

#[repr(C)]
#[derive(Clone, Copy)]
struct test_scenario {
    name: *const c_char,
    description: *const c_char,
    expected_retval: c_int,
    expect_encap: bool,
    is_v6: bool,
    vip_addr: __u32,
    src_addr: __u32,
    tunnel_dst: __u32,
    vip_addr_v6: [__u32; 4],
    src_addr_v6: [__u32; 4],
    tunnel_dst_v6: [__u32; 4],
    dst_port: __u16,
    src_port: __u16,
    ip_proto: __u8,
    vip_flags: __u32,
    vip_num: __u32,
    prepopulate_lru: bool,
    set_frag: bool,
    eth_proto: __u16,
    encap_v6_outer: bool,
    flow_mask: __u32,
    cold_lru: bool,
    set_syn: bool,
    set_rst: bool,
    set_ip_options: bool,
    fixed_batch_iters: __u32, /* 0 = auto-calibrate, >0 = use this value */
    lru_miss: lru_miss_type, /* expected LRU miss pattern */
}

const fn base_encap_v4() -> test_scenario {
    test_scenario {
        name: 0 as *const c_char,
        description: 0 as *const c_char,
        expected_retval: XDP_TX,
        expect_encap: true,
        is_v6: false,
        vip_addr: 0,
        src_addr: 0,
        tunnel_dst: TNL_DST,
        vip_addr_v6: [0; 4],
        src_addr_v6: [0; 4],
        tunnel_dst_v6: [0; 4],
        dst_port: 0,
        src_port: 0,
        ip_proto: 0,
        vip_flags: 0,
        vip_num: 0,
        prepopulate_lru: false,
        set_frag: false,
        eth_proto: 0,
        encap_v6_outer: false,
        flow_mask: 0,
        cold_lru: false,
        set_syn: false,
        set_rst: false,
        set_ip_options: false,
        fixed_batch_iters: 0,
        lru_miss: lru_miss_type::LRU_MISS_AUTO,
    }
}

const fn base_encap_v6() -> test_scenario {
    let mut s = base_encap_v4();
    s.is_v6 = true;
    s.encap_v6_outer = true;
    s.tunnel_dst_v6 = [0xfd000000, 0, 0, 2];
    s
}

const fn base_encap_v4v6() -> test_scenario {
    let mut s = base_encap_v4();
    s.encap_v6_outer = true;
    s.tunnel_dst_v6 = [0xfd000000, 0, 0, 2];
    s
}

const fn scenario(mut s: test_scenario, name: *const c_char, description: *const c_char) -> test_scenario {
    s.name = name;
    s.description = description;
    s
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

static SCENARIOS: [test_scenario; NUM_SCENARIOS] = {
    let mut a = [base_encap_v4(); NUM_SCENARIOS];
    a[scenario_id::S_TCP_V4_LRU_HIT as usize] = {
        let mut s = scenario(base_encap_v4(), cstr!("tcp-v4-lru-hit"), cstr!("IPv4 TCP, LRU hit, IPIP encap"));
        s.ip_proto = IPPROTO_TCP; s.vip_addr = IP4(10,10,1,1); s.dst_port = 80; s.src_addr = IP4(10,10,2,1); s.src_port = 12345; s.prepopulate_lru = true; s.lru_miss = lru_miss_type::LRU_MISS_NONE; s
    };
    a[scenario_id::S_TCP_V4_CH as usize] = {
        let mut s = scenario(base_encap_v4(), cstr!("tcp-v4-ch"), cstr!("IPv4 TCP, CH (LRU bypass), IPIP encap"));
        s.ip_proto = IPPROTO_TCP; s.vip_addr = IP4(10,10,1,2); s.dst_port = 80; s.src_addr = IP4(10,10,2,2); s.src_port = 54321; s.vip_flags = F_LRU_BYPASS; s.vip_num = 1; s.lru_miss = lru_miss_type::LRU_MISS_ALL; s
    };
    a[scenario_id::S_TCP_V6_LRU_HIT as usize] = {
        let mut s = scenario(base_encap_v6(), cstr!("tcp-v6-lru-hit"), cstr!("IPv6 TCP, LRU hit, IP6IP6 encap"));
        s.ip_proto = IPPROTO_TCP; s.vip_addr_v6 = IP6(0xfd000100,0,0,1); s.dst_port = 80; s.src_addr_v6 = IP6(0xfd000200,0,0,1); s.src_port = 12345; s.vip_num = 10; s.prepopulate_lru = true; s.lru_miss = lru_miss_type::LRU_MISS_NONE; s
    };
    a[scenario_id::S_TCP_V6_CH as usize] = {
        let mut s = scenario(base_encap_v6(), cstr!("tcp-v6-ch"), cstr!("IPv6 TCP, CH (LRU bypass), IP6IP6 encap"));
        s.ip_proto = IPPROTO_TCP; s.vip_addr_v6 = IP6(0xfd000100,0,0,2); s.dst_port = 80; s.src_addr_v6 = IP6(0xfd000200,0,0,2); s.src_port = 54321; s.vip_flags = F_LRU_BYPASS; s.vip_num = 12; s.lru_miss = lru_miss_type::LRU_MISS_ALL; s
    };
    a[scenario_id::S_UDP_V4_LRU_HIT as usize] = {
        let mut s = scenario(base_encap_v4(), cstr!("udp-v4-lru-hit"), cstr!("IPv4 UDP, LRU hit, IPIP encap"));
        s.ip_proto = IPPROTO_UDP; s.vip_addr = IP4(10,10,1,1); s.dst_port = 443; s.src_addr = IP4(10,10,3,1); s.src_port = 11111; s.vip_num = 2; s.prepopulate_lru = true; s.lru_miss = lru_miss_type::LRU_MISS_NONE; s
    };
    a[scenario_id::S_UDP_V6_LRU_HIT as usize] = {
        let mut s = scenario(base_encap_v6(), cstr!("udp-v6-lru-hit"), cstr!("IPv6 UDP, LRU hit, IP6IP6 encap"));
        s.ip_proto = IPPROTO_UDP; s.vip_addr_v6 = IP6(0xfd000100,0,0,1); s.dst_port = 443; s.src_addr_v6 = IP6(0xfd000200,0,0,3); s.src_port = 22222; s.vip_num = 14; s.prepopulate_lru = true; s.lru_miss = lru_miss_type::LRU_MISS_NONE; s
    };
    a[scenario_id::S_TCP_V4V6_LRU_HIT as usize] = {
        let mut s = scenario(base_encap_v4v6(), cstr!("tcp-v4v6-lru-hit"), cstr!("IPv4 TCP, LRU hit, IPv4-in-IPv6 encap"));
        s.ip_proto = IPPROTO_TCP; s.vip_addr = IP4(10,10,1,4); s.dst_port = 80; s.src_addr = IP4(10,10,2,4); s.src_port = 12347; s.vip_num = 13; s.prepopulate_lru = true; s.lru_miss = lru_miss_type::LRU_MISS_NONE; s
    };
    a[scenario_id::S_TCP_V4_LRU_DIVERSE as usize] = { let mut s = a[scenario_id::S_TCP_V4_LRU_HIT as usize]; s.name = cstr!("tcp-v4-lru-diverse"); s.description = cstr!("IPv4 TCP, diverse flows, warm LRU"); s.flow_mask = 0xfff; s };
    a[scenario_id::S_TCP_V4_CH_DIVERSE as usize] = { let mut s = a[scenario_id::S_TCP_V4_CH as usize]; s.name = cstr!("tcp-v4-ch-diverse"); s.description = cstr!("IPv4 TCP, diverse flows, CH (LRU bypass)"); s.flow_mask = 0xfff; s };
    a[scenario_id::S_TCP_V6_LRU_DIVERSE as usize] = { let mut s = a[scenario_id::S_TCP_V6_LRU_HIT as usize]; s.name = cstr!("tcp-v6-lru-diverse"); s.description = cstr!("IPv6 TCP, diverse flows, warm LRU"); s.flow_mask = 0xfff; s };
    a[scenario_id::S_TCP_V6_CH_DIVERSE as usize] = { let mut s = a[scenario_id::S_TCP_V6_CH as usize]; s.name = cstr!("tcp-v6-ch-diverse"); s.description = cstr!("IPv6 TCP, diverse flows, CH (LRU bypass)"); s.flow_mask = 0xfff; s };
    a[scenario_id::S_UDP_V4_LRU_DIVERSE as usize] = { let mut s = a[scenario_id::S_UDP_V4_LRU_HIT as usize]; s.name = cstr!("udp-v4-lru-diverse"); s.description = cstr!("IPv4 UDP, diverse flows, warm LRU"); s.flow_mask = 0xfff; s };
    a[scenario_id::S_TCP_V4_LRU_MISS as usize] = { let mut s = a[scenario_id::S_TCP_V4_LRU_HIT as usize]; s.name = cstr!("tcp-v4-lru-miss"); s.description = cstr!("IPv4 TCP, LRU miss (16M flow space), CH lookup"); s.prepopulate_lru = false; s.flow_mask = 0xffffff; s.cold_lru = true; s.lru_miss = lru_miss_type::LRU_MISS_FIRST; s };
    a[scenario_id::S_UDP_V4_LRU_MISS as usize] = { let mut s = a[scenario_id::S_UDP_V4_LRU_HIT as usize]; s.name = cstr!("udp-v4-lru-miss"); s.description = cstr!("IPv4 UDP, LRU miss (16M flow space), CH lookup"); s.prepopulate_lru = false; s.flow_mask = 0xffffff; s.cold_lru = true; s.lru_miss = lru_miss_type::LRU_MISS_FIRST; s };
    a[scenario_id::S_TCP_V4_LRU_WARMUP as usize] = { let mut s = a[scenario_id::S_TCP_V4_LRU_HIT as usize]; s.name = cstr!("tcp-v4-lru-warmup"); s.description = cstr!("IPv4 TCP, 4K flows, ~50% LRU miss"); s.prepopulate_lru = false; s.flow_mask = 0xfff; s.cold_lru = true; s.fixed_batch_iters = 6500; s.lru_miss = lru_miss_type::LRU_MISS_FIRST; s };
    a[scenario_id::S_TCP_V4_SYN as usize] = { let mut s = a[scenario_id::S_TCP_V4_LRU_HIT as usize]; s.name = cstr!("tcp-v4-syn"); s.description = cstr!("IPv4 TCP SYN, skip LRU, CH + LRU insert"); s.src_addr = IP4(10,10,8,2); s.src_port = 60001; s.prepopulate_lru = false; s.set_syn = true; s.lru_miss = lru_miss_type::LRU_MISS_ALL; s };
    a[scenario_id::S_TCP_V4_RST_MISS as usize] = { let mut s = a[scenario_id::S_TCP_V4_LRU_HIT as usize]; s.name = cstr!("tcp-v4-rst-miss"); s.description = cstr!("IPv4 TCP RST, CH lookup, no LRU insert"); s.src_addr = IP4(10,10,8,1); s.src_port = 60000; s.prepopulate_lru = false; s.flow_mask = 0xffffff; s.cold_lru = true; s.set_rst = true; s.lru_miss = lru_miss_type::LRU_MISS_ALL; s };
    a[scenario_id::S_PASS_V4_NO_VIP as usize] = { let mut s = base_encap_v4(); s.name = cstr!("pass-v4-no-vip"); s.description = cstr!("IPv4 TCP, unknown VIP, XDP_PASS"); s.expected_retval = XDP_PASS; s.expect_encap = false; s.ip_proto = IPPROTO_TCP; s.vip_addr = IP4(10,10,9,9); s.dst_port = 80; s.src_addr = IP4(10,10,4,1); s.src_port = 33333; s };
    a[scenario_id::S_PASS_V6_NO_VIP as usize] = { let mut s = base_encap_v4(); s.name = cstr!("pass-v6-no-vip"); s.description = cstr!("IPv6 TCP, unknown VIP, XDP_PASS"); s.expected_retval = XDP_PASS; s.expect_encap = false; s.is_v6 = true; s.ip_proto = IPPROTO_TCP; s.vip_addr_v6 = IP6(0xfd009900,0,0,1); s.dst_port = 80; s.src_addr_v6 = IP6(0xfd000400,0,0,1); s.src_port = 33333; s };
    a[scenario_id::S_PASS_V4_ICMP as usize] = { let mut s = base_encap_v4(); s.name = cstr!("pass-v4-icmp"); s.description = cstr!("IPv4 ICMP, non-TCP/UDP protocol, XDP_PASS"); s.expected_retval = XDP_PASS; s.expect_encap = false; s.ip_proto = IPPROTO_ICMP; s.vip_addr = IP4(10,10,1,1); s.src_addr = IP4(10,10,6,1); s };
    a[scenario_id::S_PASS_NON_IP as usize] = { let mut s = base_encap_v4(); s.name = cstr!("pass-non-ip"); s.description = cstr!("Non-IP (ARP), earliest XDP_PASS exit"); s.expected_retval = XDP_PASS; s.expect_encap = false; s.eth_proto = ETH_P_ARP; s };
    a[scenario_id::S_DROP_V4_FRAG as usize] = { let mut s = base_encap_v4(); s.name = cstr!("drop-v4-frag"); s.description = cstr!("IPv4 fragmented, XDP_DROP"); s.expected_retval = XDP_DROP; s.expect_encap = false; s.ip_proto = IPPROTO_TCP; s.vip_addr = IP4(10,10,1,1); s.dst_port = 80; s.src_addr = IP4(10,10,5,1); s.src_port = 44444; s.set_frag = true; s };
    a[scenario_id::S_DROP_V4_OPTIONS as usize] = { let mut s = base_encap_v4(); s.name = cstr!("drop-v4-options"); s.description = cstr!("IPv4 with IP options (ihl>5), XDP_DROP"); s.expected_retval = XDP_DROP; s.expect_encap = false; s.ip_proto = IPPROTO_TCP; s.vip_addr = IP4(10,10,1,1); s.dst_port = 80; s.src_addr = IP4(10,10,7,1); s.src_port = 55555; s.set_ip_options = true; s };
    a[scenario_id::S_DROP_V6_FRAG as usize] = { let mut s = base_encap_v4(); s.name = cstr!("drop-v6-frag"); s.description = cstr!("IPv6 fragment extension header, XDP_DROP"); s.expected_retval = XDP_DROP; s.expect_encap = false; s.is_v6 = true; s.ip_proto = IPPROTO_TCP; s.vip_addr_v6 = IP6(0xfd000100,0,0,1); s.dst_port = 80; s.src_addr_v6 = IP6(0xfd000500,0,0,1); s.src_port = 44444; s.set_frag = true; s };
    a
};

#[repr(C)] struct xdp_lb_bench { maps: xdp_lb_bench_maps, progs: xdp_lb_bench_progs, bss: *mut xdp_lb_bench_bss }
#[repr(C)] struct xdp_lb_bench_maps { vip_map: *mut c_void, lru_mapping: *mut c_void, ch_rings: *mut c_void, ctl_array: *mut c_void, reals: *mut c_void, vip_miss_stats: *mut c_void, stats: *mut c_void }
#[repr(C)] struct xdp_lb_bench_progs { xdp_lb_bench: *mut c_void }
#[repr(C)] struct xdp_lb_bench_bss { batch_iters: __u64, cold_lru: c_int, flow_mask: __u32 }
#[repr(C)] struct bpf_bench_timing { batch_iters: __u64, machine_readable: bool }
#[repr(C)] struct bench_res { _unused: [u8; 0] }
#[repr(C)] struct bpf_map_create_opts { sz: usize }
#[repr(C)] struct bpf_test_run_opts { sz: usize, data_in: *mut c_void, data_size_in: __u32, data_out: *mut c_void, data_size_out: __u32, retval: __u32, repeat: __u32 }
#[repr(C)] struct argp_state { _unused: [u8; 0] }
#[repr(C)] struct argp_option { name: *const c_char, key: c_int, arg: *const c_char, flags: c_int, doc: *const c_char, group: c_int }
#[repr(C)] struct argp { options: *const argp_option, parser: Option<unsafe extern "C" fn(c_int, *mut c_char, *mut argp_state) -> error_t> }
#[repr(C)] struct bench { name: *const c_char, argp: *const argp, validate: Option<unsafe extern "C" fn()>, setup: Option<unsafe extern "C" fn()>, producer_thread: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>, measure: Option<unsafe extern "C" fn(*mut bench_res)>, report_final: Option<unsafe extern "C" fn(*mut bench_res, c_int)> }
#[repr(C)] struct env_t { consumer_cnt: c_int, duration_sec: c_int, quiet: bool }
#[repr(C)] struct timespec { tv_sec: i64, tv_nsec: i64 }
#[repr(C)] struct ethhdr { h_dest: [__u8; ETH_ALEN], h_source: [__u8; ETH_ALEN], h_proto: __u16 }
#[repr(C)] struct in6_addr { s6_addr32: [__be32; 4] }
#[repr(C)] struct ipv6hdr { priority_version: __u8, flow_lbl: [__u8; 3], payload_len: __u16, nexthdr: __u8, hop_limit: __u8, saddr: in6_addr, daddr: in6_addr }
#[repr(C)] struct iphdr { ihl_version: __u8, tos: __u8, tot_len: __u16, id: __u16, frag_off: __u16, ttl: __u8, protocol: __u8, check: __u16, saddr: __u32, daddr: __u32 }
#[repr(C)] struct tcphdr { source: __u16, dest: __u16, seq: __u32, ack_seq: __u32, doff_res_flags: __u16, window: __u16, check: __u16, urg_ptr: __u16 }
#[repr(C)] struct udphdr { source: __u16, dest: __u16, len: __u16, check: __u16 }
#[repr(C)] struct flow_key { src: __be32, dst: __be32, srcv6: [__be32; 4], dstv6: [__be32; 4], proto: __u8, port16: [__u16; 2] }
#[repr(C)] struct real_pos_lru { pos: __u32, atime: __u64 }
#[repr(C)] struct vip_definition { vip: __be32, vipv6: [__be32; 4], port: __u16, proto: __u8 }
#[repr(C)] struct vip_meta { flags: __u32, vip_num: __u32 }
#[repr(C)] struct real_definition { dst: __be32, dstv6: [__be32; 4], flags: __u32 }
#[repr(C)] struct ctl_value { mac: [__u8; ETH_ALEN] }
#[repr(C)] struct lb_stats { v1: __u64, v2: __u64 }

unsafe extern "C" {
    static mut errno: c_int;
    static mut env: env_t;
    static stderr: *mut c_void;
    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn exit(status: c_int) -> !;
    fn close(fd: c_int) -> c_int;
    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn argp_usage(state: *mut argp_state);
    fn setup_libbpf();
    fn xdp_lb_bench__open() -> *mut xdp_lb_bench;
    fn xdp_lb_bench__load(skel: *mut xdp_lb_bench) -> c_int;
    fn xdp_lb_bench__destroy(skel: *mut xdp_lb_bench);
    fn bpf_map__fd(map: *mut c_void) -> c_int;
    fn bpf_program__fd(prog: *mut c_void) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: __u64) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_num_possible_cpus() -> c_uint;
    fn bpf_map_create(map_type: c_int, name: *const c_char, key_size: __u32, value_size: __u32, max_entries: __u32, opts: *const bpf_map_create_opts) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn create_encap_ipv6_src(port: __u16, addr: __be32, dst: *mut __be32);
    fn create_encap_ipv4_src(port: __u16, addr: __be32) -> __be32;
    fn bpf_bench_calibrate(timing: *mut bpf_bench_timing, cb: unsafe extern "C" fn(*mut c_void), arg: *mut c_void);
    fn bpf_bench_timing_measure(timing: *mut bpf_bench_timing, res: *mut bench_res);
    fn bpf_bench_timing_report(timing: *mut bpf_bench_timing, name: *const c_char, description: *const c_char);
}

static mut PKT_BUF: [[__u8; MAX_PKT_SIZE]; NUM_SCENARIOS] = [[0; MAX_PKT_SIZE]; NUM_SCENARIOS];
static mut PKT_LEN: [__u32; NUM_SCENARIOS] = [0; NUM_SCENARIOS];
static mut EXPECTED_BUF: [[__u8; MAX_ENCAP_SIZE]; NUM_SCENARIOS] = [[0; MAX_ENCAP_SIZE]; NUM_SCENARIOS];
static mut EXPECTED_LEN: [__u32; NUM_SCENARIOS] = [0; NUM_SCENARIOS];
static mut LRU_INNER_FDS: [c_int; BENCH_NR_CPUS] = [0; BENCH_NR_CPUS];
static mut NR_INNER_MAPS: c_int = 0;

#[repr(C)]
struct ctx_t { skel: *mut xdp_lb_bench, timing: bpf_bench_timing, prog_fd: c_int }
static mut ctx: ctx_t = ctx_t { skel: null_mut(), timing: bpf_bench_timing { batch_iters: 0, machine_readable: false }, prog_fd: 0 };

#[repr(C)]
struct args_t { scenario: c_int, machine_readable: bool }
static mut args: args_t = args_t { scenario: -1, machine_readable: false };

fn htons(x: __u16) -> __u16 { x.to_be() }
fn htonl(x: __u32) -> __u32 { x.to_be() }

unsafe fn iphdr_set_version_ihl(iph: *mut iphdr, version: __u8, ihl: __u8) {
    (*iph).ihl_version = (ihl & 0x0f) | (version << 4);
}

unsafe fn ipv6hdr_set_version(ip6h: *mut ipv6hdr, version: __u8) {
    (*ip6h).priority_version = version << 4;
}

unsafe fn tcphdr_set_doff(tcp: *mut tcphdr, doff: __u16) {
    (*tcp).doff_res_flags = ((*tcp).doff_res_flags & 0x00ff) | (doff << 12);
}

unsafe fn tcphdr_set_syn(tcp: *mut tcphdr, v: __u16) {
    if v != 0 { (*tcp).doff_res_flags |= 0x0002; } else { (*tcp).doff_res_flags &= !0x0002; }
}

unsafe fn tcphdr_set_rst(tcp: *mut tcphdr, v: __u16) {
    if v != 0 { (*tcp).doff_res_flags |= 0x0004; } else { (*tcp).doff_res_flags &= !0x0004; }
}

unsafe fn ip_checksum(hdr: *const c_void, len: c_int) -> __u16 {
    let p = hdr as *const __u16;
    let mut csum: __u32 = 0;
    let mut i: c_int = 0;
    while i < len / 2 {
        csum = csum.wrapping_add(*p.add(i as usize) as __u32);
        i += 1;
    }
    while (csum >> 16) != 0 {
        csum = (csum & 0xffff).wrapping_add(csum >> 16);
    }
    !(csum as __u16)
}

unsafe fn htonl_v6(dst: *mut __be32, src: *const __u32) {
    let mut i = 0;
    while i < 4 {
        *dst.add(i) = htonl(*src.add(i));
        i += 1;
    }
}

unsafe fn build_flow_key(fk: *mut flow_key, sc: *const test_scenario) {
    memset(fk as *mut c_void, 0, size_of::<flow_key>());
    if (*sc).is_v6 {
        htonl_v6((*fk).srcv6.as_mut_ptr(), (*sc).src_addr_v6.as_ptr());
        htonl_v6((*fk).dstv6.as_mut_ptr(), (*sc).vip_addr_v6.as_ptr());
    } else {
        (*fk).src = htonl((*sc).src_addr);
        (*fk).dst = htonl((*sc).vip_addr);
    }
    (*fk).proto = (*sc).ip_proto;
    (*fk).port16[0] = htons((*sc).src_port);
    (*fk).port16[1] = htons((*sc).dst_port);
}

unsafe fn build_l4(sc: *const test_scenario, p: *mut __u8, off: *mut __u32) {
    if (*sc).ip_proto == IPPROTO_TCP {
        let mut tcp: tcphdr = zeroed();
        tcp.source = htons((*sc).src_port);
        tcp.dest = htons((*sc).dst_port);
        tcphdr_set_doff(&mut tcp, 5);
        tcphdr_set_syn(&mut tcp, if (*sc).set_syn { 1 } else { 0 });
        tcphdr_set_rst(&mut tcp, if (*sc).set_rst { 1 } else { 0 });
        tcp.window = htons(8192);
        memcpy(p.add(*off as usize) as *mut c_void, &tcp as *const _ as *const c_void, size_of::<tcphdr>());
        *off += size_of::<tcphdr>() as __u32;
    } else if (*sc).ip_proto == IPPROTO_UDP {
        let mut udp: udphdr = zeroed();
        udp.source = htons((*sc).src_port);
        udp.dest = htons((*sc).dst_port);
        udp.len = htons((size_of::<udphdr>() + 16) as __u16);
        memcpy(p.add(*off as usize) as *mut c_void, &udp as *const _ as *const c_void, size_of::<udphdr>());
        *off += size_of::<udphdr>() as __u32;
    }
}

unsafe fn build_packet(idx: c_int) {
    let sc = SCENARIOS.as_ptr().add(idx as usize);
    let p = PKT_BUF[idx as usize].as_mut_ptr();
    let mut eth: ethhdr = zeroed();
    let proto: __u16;
    let mut off: __u32 = 0;

    memcpy(eth.h_dest.as_mut_ptr() as *mut c_void, LB_MAC.as_ptr() as *const c_void, ETH_ALEN);
    memcpy(eth.h_source.as_mut_ptr() as *mut c_void, CLIENT_MAC.as_ptr() as *const c_void, ETH_ALEN);
    if (*sc).eth_proto != 0 { proto = (*sc).eth_proto; } else if (*sc).is_v6 { proto = ETH_P_IPV6; } else { proto = ETH_P_IP; }
    eth.h_proto = htons(proto);
    memcpy(p as *mut c_void, &eth as *const _ as *const c_void, size_of::<ethhdr>());
    off += size_of::<ethhdr>() as __u32;

    if proto != ETH_P_IP && proto != ETH_P_IPV6 {
        memcpy(p.add(off as usize) as *mut c_void, cstr!("bench___payload!") as *const c_void, 16);
        off += 16;
        PKT_LEN[idx as usize] = off;
        return;
    }

    if (*sc).is_v6 {
        let mut ip6h: ipv6hdr = zeroed();
        let ip6_off = off;
        ipv6hdr_set_version(&mut ip6h, 6);
        ip6h.nexthdr = if (*sc).set_frag { 44 } else { (*sc).ip_proto };
        ip6h.hop_limit = 64;
        htonl_v6(ip6h.saddr.s6_addr32.as_mut_ptr(), (*sc).src_addr_v6.as_ptr());
        htonl_v6(ip6h.daddr.s6_addr32.as_mut_ptr(), (*sc).vip_addr_v6.as_ptr());
        off += size_of::<ipv6hdr>() as __u32;
        if (*sc).set_frag {
            memset(p.add(off as usize) as *mut c_void, 0, 8);
            *p.add(off as usize) = (*sc).ip_proto;
            off += 8;
        }
        build_l4(sc, p, &mut off);
        memcpy(p.add(off as usize) as *mut c_void, cstr!("bench___payload!") as *const c_void, 16);
        off += 16;
        ip6h.payload_len = htons((off - ip6_off - size_of::<ipv6hdr>() as __u32) as __u16);
        memcpy(p.add(ip6_off as usize) as *mut c_void, &ip6h as *const _ as *const c_void, size_of::<ipv6hdr>());
    } else {
        let mut iph: iphdr = zeroed();
        let ip_off = off;
        iphdr_set_version_ihl(&mut iph, 4, if (*sc).set_ip_options { 6 } else { 5 });
        iph.ttl = 64;
        iph.protocol = (*sc).ip_proto;
        iph.saddr = htonl((*sc).src_addr);
        iph.daddr = htonl((*sc).vip_addr);
        iph.frag_off = if (*sc).set_frag { htons(IP_MF) } else { 0 };
        off += size_of::<iphdr>() as __u32;
        if (*sc).set_ip_options {
            /* NOP option padding (4 bytes = 1 word) */
            let nop: __u32 = htonl(0x01010101);
            memcpy(p.add(off as usize) as *mut c_void, &nop as *const _ as *const c_void, size_of::<__u32>());
            off += size_of::<__u32>() as __u32;
        }
        build_l4(sc, p, &mut off);
        memcpy(p.add(off as usize) as *mut c_void, cstr!("bench___payload!") as *const c_void, 16);
        off += 16;
        iph.tot_len = htons((off - ip_off) as __u16);
        iph.check = ip_checksum(&iph as *const _ as *const c_void, size_of::<iphdr>() as c_int);
        memcpy(p.add(ip_off as usize) as *mut c_void, &iph as *const _ as *const c_void, size_of::<iphdr>());
    }
    PKT_LEN[idx as usize] = off;
}

unsafe fn populate_vip(skel: *mut xdp_lb_bench, sc: *const test_scenario) {
    let mut key: vip_definition = zeroed();
    let mut val: vip_meta = zeroed();
    if (*sc).is_v6 { htonl_v6(key.vipv6.as_mut_ptr(), (*sc).vip_addr_v6.as_ptr()); } else { key.vip = htonl((*sc).vip_addr); }
    key.port = htons((*sc).dst_port);
    key.proto = (*sc).ip_proto;
    val.flags = (*sc).vip_flags;
    val.vip_num = (*sc).vip_num;
    let err = bpf_map_update_elem(bpf_map__fd((*skel).maps.vip_map), &key as *const _ as *const c_void, &val as *const _ as *const c_void, BPF_ANY);
    if err != 0 { fprintf(stderr, cstr!("vip_map [%s]: %s\n"), (*sc).name, strerror(errno)); exit(1); }
}

unsafe fn create_per_cpu_lru_maps(skel: *mut xdp_lb_bench) {
    let outer_fd = bpf_map__fd((*skel).maps.lru_mapping);
    let mut nr_cpus = bpf_num_possible_cpus();
    if nr_cpus > BENCH_NR_CPUS as c_uint { nr_cpus = BENCH_NR_CPUS as c_uint; }
    let mut i: c_int = 0;
    while i < nr_cpus as c_int {
        let opts = bpf_map_create_opts { sz: size_of::<bpf_map_create_opts>() };
        let inner_fd = bpf_map_create(BPF_MAP_TYPE_LRU_HASH, cstr!("lru_inner"), size_of::<flow_key>() as __u32, size_of::<real_pos_lru>() as __u32, DEFAULT_LRU_SIZE, &opts);
        if inner_fd < 0 { fprintf(stderr, cstr!("lru_inner[%d]: %s\n"), i, strerror(errno)); exit(1); }
        let cpu: __u32 = i as __u32;
        let err = bpf_map_update_elem(outer_fd, &cpu as *const _ as *const c_void, &inner_fd as *const _ as *const c_void, BPF_ANY);
        if err != 0 { fprintf(stderr, cstr!("lru_mapping[%d]: %s\n"), i, strerror(errno)); close(inner_fd); exit(1); }
        LRU_INNER_FDS[i as usize] = inner_fd;
        i += 1;
    }
    NR_INNER_MAPS = nr_cpus as c_int;
}

unsafe fn ktime_get_ns() -> __u64 {
    const CLOCK_MONOTONIC: c_int = 1;
    let mut ts: timespec = zeroed();
    clock_gettime(CLOCK_MONOTONIC, &mut ts);
    ts.tv_sec as __u64 * 1000000000u64 + ts.tv_nsec as __u64
}

unsafe fn populate_lru(sc: *const test_scenario, real_idx: __u32) {
    let mut lru = real_pos_lru { pos: real_idx, atime: 0 };
    let mut fk: flow_key = zeroed();
    if (*sc).ip_proto == IPPROTO_UDP { lru.atime = ktime_get_ns(); }
    build_flow_key(&mut fk, sc);
    /* Insert into every per-CPU inner LRU so the entry is found
     * regardless of which CPU runs the BPF program.
     */
    let mut i: c_int = 0;
    while i < NR_INNER_MAPS {
        let err = bpf_map_update_elem(LRU_INNER_FDS[i as usize], &fk as *const _ as *const c_void, &lru as *const _ as *const c_void, BPF_ANY);
        if err != 0 { fprintf(stderr, cstr!("lru_inner[%d] [%s]: %s\n"), i, (*sc).name, strerror(errno)); exit(1); }
        i += 1;
    }
}

unsafe fn populate_maps(skel: *mut xdp_lb_bench) {
    let mut real_v4: real_definition = zeroed();
    let mut real_v6: real_definition = zeroed();
    let mut cval: ctl_value = zeroed();
    let mut key: __u32;
    let real_idx: __u32 = REAL_INDEX;
    if SCENARIOS[args.scenario as usize].expect_encap { populate_vip(skel, &SCENARIOS[args.scenario as usize]); }
    let ch_fd = bpf_map__fd((*skel).maps.ch_rings);
    let mut i: c_int = 0;
    while i < CH_RINGS_SIZE {
        let k: __u32 = i as __u32;
        let err = bpf_map_update_elem(ch_fd, &k as *const _ as *const c_void, &real_idx as *const _ as *const c_void, BPF_ANY);
        if err != 0 { fprintf(stderr, cstr!("ch_rings[%d]: %s\n"), i, strerror(errno)); exit(1); }
        i += 1;
    }
    memcpy(cval.mac.as_mut_ptr() as *mut c_void, ROUTER_MAC.as_ptr() as *const c_void, ETH_ALEN);
    key = 0;
    let mut err = bpf_map_update_elem(bpf_map__fd((*skel).maps.ctl_array), &key as *const _ as *const c_void, &cval as *const _ as *const c_void, BPF_ANY);
    if err != 0 { fprintf(stderr, cstr!("ctl_array: %s\n"), strerror(errno)); exit(1); }
    key = REAL_INDEX;
    real_v4.dst = htonl(TNL_DST);
    htonl_v6(real_v4.dstv6.as_mut_ptr(), TNL_DST_V6.as_ptr());
    err = bpf_map_update_elem(bpf_map__fd((*skel).maps.reals), &key as *const _ as *const c_void, &real_v4 as *const _ as *const c_void, BPF_ANY);
    if err != 0 { fprintf(stderr, cstr!("reals[%d]: %s\n"), REAL_INDEX, strerror(errno)); exit(1); }
    key = REAL_INDEX_V6;
    htonl_v6(real_v6.dstv6.as_mut_ptr(), TNL_DST_V6.as_ptr());
    real_v6.flags = F_IPV6;
    err = bpf_map_update_elem(bpf_map__fd((*skel).maps.reals), &key as *const _ as *const c_void, &real_v6 as *const _ as *const c_void, BPF_ANY);
    if err != 0 { fprintf(stderr, cstr!("reals[%d]: %s\n"), REAL_INDEX_V6, strerror(errno)); exit(1); }
    create_per_cpu_lru_maps(skel);
    if SCENARIOS[args.scenario as usize].prepopulate_lru {
        let sc = &SCENARIOS[args.scenario as usize] as *const test_scenario;
        let ridx = if (*sc).encap_v6_outer { REAL_INDEX_V6 } else { REAL_INDEX };
        populate_lru(sc, ridx);
    }
    if SCENARIOS[args.scenario as usize].expect_encap {
        let sc = &SCENARIOS[args.scenario as usize] as *const test_scenario;
        let mut miss_vip: vip_definition = zeroed();
        if (*sc).is_v6 { htonl_v6(miss_vip.vipv6.as_mut_ptr(), (*sc).vip_addr_v6.as_ptr()); } else { miss_vip.vip = htonl((*sc).vip_addr); }
        miss_vip.port = htons((*sc).dst_port);
        miss_vip.proto = (*sc).ip_proto;
        key = 0;
        err = bpf_map_update_elem(bpf_map__fd((*skel).maps.vip_miss_stats), &key as *const _ as *const c_void, &miss_vip as *const _ as *const c_void, BPF_ANY);
        if err != 0 { fprintf(stderr, cstr!("vip_miss_stats: %s\n"), strerror(errno)); exit(1); }
    }
}

unsafe fn build_expected_packet(idx: c_int) {
    let sc = &SCENARIOS[idx as usize] as *const test_scenario;
    let p = EXPECTED_BUF[idx as usize].as_mut_ptr();
    let mut eth: ethhdr = zeroed();
    let input = PKT_BUF[idx as usize].as_ptr();
    let in_len = PKT_LEN[idx as usize];
    let mut off: __u32 = 0;
    let inner_len = in_len - size_of::<ethhdr>() as __u32;
    if (*sc).expected_retval == XDP_DROP { EXPECTED_LEN[idx as usize] = 0; return; }
    if (*sc).expected_retval == XDP_PASS {
        memcpy(p as *mut c_void, input as *const c_void, in_len as usize);
        EXPECTED_LEN[idx as usize] = in_len;
        return;
    }
    memcpy(eth.h_dest.as_mut_ptr() as *mut c_void, ROUTER_MAC.as_ptr() as *const c_void, ETH_ALEN);
    memcpy(eth.h_source.as_mut_ptr() as *mut c_void, LB_MAC.as_ptr() as *const c_void, ETH_ALEN);
    eth.h_proto = htons(if (*sc).encap_v6_outer { ETH_P_IPV6 } else { ETH_P_IP });
    memcpy(p as *mut c_void, &eth as *const _ as *const c_void, size_of::<ethhdr>());
    off += size_of::<ethhdr>() as __u32;
    if (*sc).encap_v6_outer {
        let mut ip6h: ipv6hdr = zeroed();
        let nexthdr = if (*sc).is_v6 { IPPROTO_IPV6 } else { IPPROTO_IPIP };
        ipv6hdr_set_version(&mut ip6h, 6);
        ip6h.nexthdr = nexthdr;
        ip6h.payload_len = htons(inner_len as __u16);
        ip6h.hop_limit = 64;
        create_encap_ipv6_src(htons((*sc).src_port), if (*sc).is_v6 { htonl((*sc).src_addr_v6[0]) } else { htonl((*sc).src_addr) }, ip6h.saddr.s6_addr32.as_mut_ptr());
        htonl_v6(ip6h.daddr.s6_addr32.as_mut_ptr(), (*sc).tunnel_dst_v6.as_ptr());
        memcpy(p.add(off as usize) as *mut c_void, &ip6h as *const _ as *const c_void, size_of::<ipv6hdr>());
        off += size_of::<ipv6hdr>() as __u32;
    } else {
        let mut iph: iphdr = zeroed();
        iphdr_set_version_ihl(&mut iph, 4, (size_of::<iphdr>() >> 2) as __u8);
        iph.protocol = IPPROTO_IPIP;
        iph.tot_len = htons((inner_len + size_of::<iphdr>() as __u32) as __u16);
        iph.ttl = 64;
        iph.saddr = create_encap_ipv4_src(htons((*sc).src_port), htonl((*sc).src_addr));
        iph.daddr = htonl((*sc).tunnel_dst);
        iph.check = ip_checksum(&iph as *const _ as *const c_void, size_of::<iphdr>() as c_int);
        memcpy(p.add(off as usize) as *mut c_void, &iph as *const _ as *const c_void, size_of::<iphdr>());
        off += size_of::<iphdr>() as __u32;
    }
    memcpy(p.add(off as usize) as *mut c_void, input.add(size_of::<ethhdr>()) as *const c_void, inner_len as usize);
    off += inner_len;
    EXPECTED_LEN[idx as usize] = off;
}

unsafe fn print_hex_diff(name: *const c_char, got: *const __u8, got_len: __u32, exp: *const __u8, exp_len: __u32) {
    let max_len = if got_len > exp_len { got_len } else { exp_len };
    let mut i: __u32 = 0;
    let mut ndiffs: __u32 = 0;
    fprintf(stderr, cstr!("  [%s] got %u bytes, expected %u bytes\n"), name, got_len, exp_len);
    while i < max_len && ndiffs < 8 {
        let g = if i < got_len { *got.add(i as usize) } else { 0 };
        let e = if i < exp_len { *exp.add(i as usize) } else { 0 };
        if g != e || i >= got_len || i >= exp_len {
            fprintf(stderr, cstr!("    offset 0x%03x: got 0x%02x  expected 0x%02x\n"), i, g as c_int, e as c_int);
            ndiffs += 1;
        }
        i += 1;
    }
    if ndiffs >= 8 && i < max_len { fprintf(stderr, cstr!("    ... (more differences)\n")); }
}

unsafe fn read_stat(stats_fd: c_int, key: __u32, v1_out: *mut __u64, v2_out: *mut __u64) {
    let mut values: [lb_stats; BENCH_NR_CPUS] = zeroed();
    let mut nr_cpus = bpf_num_possible_cpus();
    let mut v1: __u64 = 0;
    let mut v2: __u64 = 0;
    if nr_cpus > BENCH_NR_CPUS as c_uint { nr_cpus = BENCH_NR_CPUS as c_uint; }
    if bpf_map_lookup_elem(stats_fd, &key as *const _ as *const c_void, values.as_mut_ptr() as *mut c_void) == 0 {
        let mut i: c_uint = 0;
        while i < nr_cpus {
            v1 += values[i as usize].v1;
            v2 += values[i as usize].v2;
            i += 1;
        }
    }
    *v1_out = v1;
    *v2_out = v2;
}

unsafe fn reset_stats(stats_fd: c_int) {
    let mut zeros: [lb_stats; BENCH_NR_CPUS] = zeroed();
    memset(zeros.as_mut_ptr() as *mut c_void, 0, size_of::<[lb_stats; BENCH_NR_CPUS]>());
    let mut key: __u32 = 0;
    while key < STATS_SIZE {
        bpf_map_update_elem(stats_fd, &key as *const _ as *const c_void, zeros.as_ptr() as *const c_void, BPF_ANY);
        key += 1;
    }
}

unsafe fn validate_counters(idx: c_int) -> bool {
    let sc = &SCENARIOS[idx as usize] as *const test_scenario;
    let stats_fd = bpf_map__fd((*ctx.skel).maps.stats);
    let (mut xdp_tx, mut xdp_pass, mut xdp_drop, mut lru_pkts, mut lru_misses, mut tcp_misses, mut dummy) = (0, 0, 0, 0, 0, 0, 0);
    /*
     * BENCH_BPF_LOOP runs batch_iters timed + 1 untimed iteration.
     * Each iteration calls process_packet -> count_action, so all
     * counters are incremented (batch_iters + 1) times.
     */
    let n = ctx.timing.batch_iters + 1;
    let mut pass = true;
    read_stat(stats_fd, STATS_XDP_TX, &mut xdp_tx, &mut dummy);
    read_stat(stats_fd, STATS_XDP_PASS, &mut xdp_pass, &mut dummy);
    read_stat(stats_fd, STATS_XDP_DROP, &mut xdp_drop, &mut dummy);
    read_stat(stats_fd, STATS_LRU, &mut lru_pkts, &mut lru_misses);
    read_stat(stats_fd, STATS_LRU_MISS, &mut tcp_misses, &mut dummy);
    if (*sc).expected_retval == XDP_TX && xdp_tx != n { fprintf(stderr, cstr!("  [%s] COUNTER FAIL: STATS_XDP_TX=%llu, expected %llu\n"), (*sc).name, xdp_tx, n); pass = false; }
    if (*sc).expected_retval == XDP_PASS && xdp_pass != n { fprintf(stderr, cstr!("  [%s] COUNTER FAIL: STATS_XDP_PASS=%llu, expected %llu\n"), (*sc).name, xdp_pass, n); pass = false; }
    if (*sc).expected_retval == XDP_DROP && xdp_drop != n { fprintf(stderr, cstr!("  [%s] COUNTER FAIL: STATS_XDP_DROP=%llu, expected %llu\n"), (*sc).name, xdp_drop, n); pass = false; }
    if !(*sc).expect_encap { reset_stats(stats_fd); return pass; }
    if lru_pkts != n { fprintf(stderr, cstr!("  [%s] COUNTER FAIL: STATS_LRU.v1=%llu, expected %llu\n"), (*sc).name, lru_pkts, n); pass = false; }
    let expected_misses = match (*sc).lru_miss {
        lru_miss_type::LRU_MISS_NONE => 0,
        lru_miss_type::LRU_MISS_ALL => n,
        lru_miss_type::LRU_MISS_FIRST => 1,
        _ => {
            /* LRU_MISS_AUTO: compute from scenario flags */
            if (*sc).prepopulate_lru && !(*sc).set_syn { 0 } else if (*sc).set_syn || (*sc).set_rst || ((*sc).vip_flags & F_LRU_BYPASS) != 0 { n } else if (*sc).cold_lru { 1 } else { n }
        }
    };
    if lru_misses != expected_misses { fprintf(stderr, cstr!("  [%s] COUNTER FAIL: LRU misses=%llu, expected %llu\n"), (*sc).name, lru_misses, expected_misses); pass = false; }
    if (*sc).ip_proto == IPPROTO_TCP && lru_misses > 0 && tcp_misses != lru_misses { fprintf(stderr, cstr!("  [%s] COUNTER FAIL: TCP LRU misses=%llu, expected %llu\n"), (*sc).name, tcp_misses, lru_misses); pass = false; }
    reset_stats(stats_fd);
    pass
}

unsafe fn xdp_action_str(action: c_int) -> *const c_char {
    match action {
        XDP_DROP => cstr!("XDP_DROP"),
        XDP_PASS => cstr!("XDP_PASS"),
        XDP_TX => cstr!("XDP_TX"),
        _ => cstr!("UNKNOWN"),
    }
}

unsafe fn validate_scenario(idx: c_int) -> bool {
    let mut topts = bpf_test_run_opts { sz: size_of::<bpf_test_run_opts>(), data_in: null_mut(), data_size_in: 0, data_out: null_mut(), data_size_out: 0, retval: 0, repeat: 0 };
    let sc = &SCENARIOS[idx as usize] as *const test_scenario;
    let mut out: [__u8; MAX_ENCAP_SIZE] = [0; MAX_ENCAP_SIZE];
    topts.data_in = PKT_BUF[idx as usize].as_mut_ptr() as *mut c_void;
    topts.data_size_in = PKT_LEN[idx as usize];
    topts.data_out = out.as_mut_ptr() as *mut c_void;
    topts.data_size_out = size_of::<[__u8; MAX_ENCAP_SIZE]>() as __u32;
    topts.repeat = 1;
    let err = bpf_prog_test_run_opts(ctx.prog_fd, &mut topts);
    if err != 0 { fprintf(stderr, cstr!("  [%s] FAIL: test_run: %s\n"), (*sc).name, strerror(errno)); return false; }
    if topts.retval as c_int != (*sc).expected_retval {
        fprintf(stderr, cstr!("  [%s] FAIL: retval %s, expected %s\n"), (*sc).name, xdp_action_str(topts.retval as c_int), xdp_action_str((*sc).expected_retval));
        return false;
    }
    /*
     * Compare output packet when it's deterministic.
     * Skip for XDP_DROP (no output) and cold_lru (source IP poisoned).
     */
    if (*sc).expected_retval != XDP_DROP && !(*sc).cold_lru {
        if topts.data_size_out != EXPECTED_LEN[idx as usize] ||
           memcmp(out.as_ptr() as *const c_void, EXPECTED_BUF[idx as usize].as_ptr() as *const c_void, EXPECTED_LEN[idx as usize] as usize) != 0 {
            fprintf(stderr, cstr!("  [%s] FAIL: output packet mismatch\n"), (*sc).name);
            print_hex_diff((*sc).name, out.as_ptr(), topts.data_size_out, EXPECTED_BUF[idx as usize].as_ptr(), EXPECTED_LEN[idx as usize]);
            return false;
        }
    }
    if !validate_counters(idx) { return false; }
    true
}

unsafe fn find_scenario(name: *const c_char) -> c_int {
    let mut i: c_int = 0;
    while i < NUM_SCENARIOS as c_int {
        if strcmp(SCENARIOS[i as usize].name, name) == 0 { return i; }
        i += 1;
    }
    -1
}

unsafe extern "C" fn xdp_lb_validate() {
    if env.consumer_cnt != 0 { fprintf(stderr, cstr!("benchmark doesn't support consumers\n")); exit(1); }
    if bpf_num_possible_cpus() > BENCH_NR_CPUS as c_uint {
        fprintf(stderr, cstr!("too many CPUs (%d > %d), increase BENCH_NR_CPUS\n"), bpf_num_possible_cpus(), BENCH_NR_CPUS as c_int);
        exit(1);
    }
}

unsafe extern "C" fn xdp_lb_run_once(_unused: *mut c_void) {
    let idx = args.scenario;
    let mut topts = bpf_test_run_opts { sz: size_of::<bpf_test_run_opts>(), data_in: PKT_BUF[idx as usize].as_mut_ptr() as *mut c_void, data_size_in: PKT_LEN[idx as usize], data_out: null_mut(), data_size_out: 0, retval: 0, repeat: 1 };
    bpf_prog_test_run_opts(ctx.prog_fd, &mut topts);
}

unsafe extern "C" fn xdp_lb_setup() {
    if args.scenario < 0 { fprintf(stderr, cstr!("--scenario is required. Use --list-scenarios to see options.\n")); exit(1); }
    setup_libbpf();
    let skel = xdp_lb_bench__open();
    if skel.is_null() { fprintf(stderr, cstr!("failed to open skeleton\n")); exit(1); }
    let err = xdp_lb_bench__load(skel);
    if err != 0 {
        fprintf(stderr, cstr!("failed to load skeleton: %s\n"), strerror(-err));
        xdp_lb_bench__destroy(skel);
        exit(1);
    }
    ctx.skel = skel;
    ctx.prog_fd = bpf_program__fd((*skel).progs.xdp_lb_bench);
    build_packet(args.scenario);
    build_expected_packet(args.scenario);
    populate_maps(skel);
    ctx.timing = bpf_bench_timing { batch_iters: 0, machine_readable: false };
    ctx.timing.machine_readable = args.machine_readable;
    if SCENARIOS[args.scenario as usize].fixed_batch_iters != 0 {
        ctx.timing.batch_iters = SCENARIOS[args.scenario as usize].fixed_batch_iters as __u64;
        (*(*skel).bss).batch_iters = ctx.timing.batch_iters;
    } else {
        bpf_bench_calibrate(&mut ctx.timing, xdp_lb_run_once, null_mut());
    }
    env.duration_sec = 600;
    /*
     * Enable cold_lru before validation so LRU miss counters are
     * correct.  Seed the LRU with one run so the original flow is
     * present; validation then sees exactly 1 miss (the poisoned
     * flow) regardless of whether calibration ran.
     */
    if SCENARIOS[args.scenario as usize].cold_lru {
        (*(*skel).bss).cold_lru = 1;
        xdp_lb_run_once(null_mut());
    }
    reset_stats(bpf_map__fd((*skel).maps.stats));
    if !validate_scenario(args.scenario) {
        fprintf(stderr, cstr!("Validation FAILED - aborting benchmark\n"));
        exit(1);
    }
    if SCENARIOS[args.scenario as usize].flow_mask != 0 {
        (*(*skel).bss).flow_mask = SCENARIOS[args.scenario as usize].flow_mask;
    }
}

unsafe extern "C" fn xdp_lb_producer(_input: *mut c_void) -> *mut c_void {
    loop { xdp_lb_run_once(null_mut()); }
}

unsafe extern "C" fn xdp_lb_measure(res: *mut bench_res) {
    bpf_bench_timing_measure(&mut ctx.timing, res);
}

unsafe extern "C" fn xdp_lb_report_final(_res: *mut bench_res, _res_cnt: c_int) {
    bpf_bench_timing_report(&mut ctx.timing, SCENARIOS[args.scenario as usize].name, SCENARIOS[args.scenario as usize].description);
}

const ARG_SCENARIO: c_int = 9001;
const ARG_LIST_SCENARIOS: c_int = 9002;
const ARG_MACHINE_READABLE: c_int = 9003;
const ARGP_ERR_UNKNOWN: error_t = -1;

static OPTS: [argp_option; 4] = [
    argp_option { name: cstr!("scenario"), key: ARG_SCENARIO, arg: cstr!("NAME"), flags: 0, doc: cstr!("Scenario to benchmark (required)"), group: 0 },
    argp_option { name: cstr!("list-scenarios"), key: ARG_LIST_SCENARIOS, arg: 0 as *const c_char, flags: 0, doc: cstr!("List available scenarios and exit"), group: 0 },
    argp_option { name: cstr!("machine-readable"), key: ARG_MACHINE_READABLE, arg: 0 as *const c_char, flags: 0, doc: cstr!("Print only a machine-readable RESULT line"), group: 0 },
    argp_option { name: 0 as *const c_char, key: 0, arg: 0 as *const c_char, flags: 0, doc: 0 as *const c_char, group: 0 },
];

unsafe extern "C" fn parse_arg(key: c_int, arg: *mut c_char, state: *mut argp_state) -> error_t {
    match key {
        ARG_SCENARIO => {
            args.scenario = find_scenario(arg);
            if args.scenario < 0 {
                fprintf(stderr, cstr!("unknown scenario: '%s'\n"), arg);
                fprintf(stderr, cstr!("use --list-scenarios to see options\n"));
                argp_usage(state);
            }
        }
        ARG_LIST_SCENARIOS => {
            printf(cstr!("Available scenarios:\n"));
            let mut i: c_int = 0;
            while i < NUM_SCENARIOS as c_int {
                printf(cstr!("  %-20s  %s\n"), SCENARIOS[i as usize].name, SCENARIOS[i as usize].description);
                i += 1;
            }
            exit(0);
        }
        ARG_MACHINE_READABLE => {
            args.machine_readable = true;
            env.quiet = true;
        }
        _ => return ARGP_ERR_UNKNOWN,
    }
    0
}

#[unsafe(no_mangle)]
pub static bench_xdp_lb_argp: argp = argp {
    options: OPTS.as_ptr(),
    parser: Some(parse_arg),
};

#[unsafe(no_mangle)]
pub static bench_xdp_lb: bench = bench {
    name: cstr!("xdp-lb"),
    argp: &bench_xdp_lb_argp,
    validate: Some(xdp_lb_validate),
    setup: Some(xdp_lb_setup),
    producer_thread: Some(xdp_lb_producer),
    measure: Some(xdp_lb_measure),
    report_final: Some(xdp_lb_report_final),
};
