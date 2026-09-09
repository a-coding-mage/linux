/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright (c) 2019 Facebook
 *
 * Include file for sample Host Bandwidth Manager (HBM) BPF programs.
 * C header dependencies are supplied by the surrounding BPF environment.
 */

pub const KBUILD_MODNAME: &str = "foo";

pub const DROP_PKT: i32 = 0;
pub const ALLOW_PKT: i32 = 1;
pub const TCP_ECN_OK: i32 = 1;
pub const CWR: i32 = 2;

pub const INITIAL_CREDIT_PACKETS: u64 = 100;
pub const MAX_BYTES_PER_PACKET: u64 = 1500;
pub const MARK_THRESH: u64 = 40 * MAX_BYTES_PER_PACKET;
pub const DROP_THRESH: u64 = 80 * 5 * MAX_BYTES_PER_PACKET;
pub const LARGE_PKT_DROP_THRESH: u64 = DROP_THRESH - 15 * MAX_BYTES_PER_PACKET;
pub const MARK_REGION_SIZE: u64 = LARGE_PKT_DROP_THRESH - MARK_THRESH;
pub const LARGE_PKT_THRESH: u64 = 120;
pub const MAX_CREDIT: u64 = 100 * MAX_BYTES_PER_PACKET;
pub const INIT_CREDIT: u64 = INITIAL_CREDIT_PACKETS * MAX_BYTES_PER_PACKET;

// Time base accounting for fq's EDT.
pub const BURST_SIZE_NS: u64 = 100_000;
pub const MARK_THRESH_NS: u64 = 50_000;
pub const DROP_THRESH_NS: u64 = 500_000;
// Reserve 20us of queuing for small packets (less than 120 bytes).
pub const LARGE_PKT_DROP_THRESH_NS: u64 = DROP_THRESH_NS - 20_000;
pub const MARK_REGION_SIZE_NS: u64 = LARGE_PKT_DROP_THRESH_NS - MARK_THRESH_NS;

#[inline]
pub const fn credit_per_ns(delta: u64, rate: u64) -> u64 {
    delta.wrapping_mul(rate) >> 20
}
#[inline]
pub const fn bytes_per_ns(delta: u64, rate: u64) -> u64 {
    delta.wrapping_mul(rate) >> 20
}
// C's div64_u64 dependency is supplied externally.
extern "C" { pub fn div64_u64(n: u64, d: u64) -> u64; }
#[inline]
pub unsafe fn bytes_to_ns(bytes: u64, rate: u64) -> u64 {
    div64_u64(bytes << 20, rate)
}

#[repr(C)]
pub struct hbm_pkt_info {
    pub cwnd: i32,
    pub rtt: i32,
    pub packets_out: i32,
    pub is_ip: bool,
    pub is_tcp: bool,
    pub ecn: i16,
}

// Types and helpers below are provided by the included BPF/kernel headers.
#[repr(C)] pub struct __sk_buff { pub sk: *mut bpf_sock }
#[repr(C)] pub struct bpf_sock { pub protocol: u32 }
#[repr(C)] pub struct bpf_tcp_sock { pub snd_cwnd: i32, pub srtt_us: u32, pub packets_out: i32 }
#[repr(C)] pub struct iphdr { pub version: u8, pub protocol: u8, pub tos: u8 }
#[repr(C)] pub struct ipv6hdr { pub nexthdr: u8, pub flow_lbl: [u8; 3] }
#[repr(C)] pub struct hbm_vqueue { pub lasttime: u64, pub credit: u64, pub rate: i32 }
#[repr(C)] pub struct hbm_queue_stats {
    pub bytes_total: i64, pub stats: bool, pub firstPacketTime: u64, pub lastPacketTime: u64,
    pub pkts_total: i64, pub pkts_marked: i64, pub bytes_marked: i64,
    pub pkts_dropped: i64, pub bytes_dropped: i64, pub pkts_ecn_ce: i64,
    pub sum_cwnd: i64, pub sum_cwnd_cnt: i64, pub sum_rtt: i64, pub sum_credit: i64,
    pub returnValCount: [i64; 4],
}

extern "C" {
    pub fn bpf_sk_fullsock(sk: *mut bpf_sock) -> *mut bpf_sock;
    pub fn bpf_tcp_sock(sk: *mut bpf_sock) -> *mut bpf_tcp_sock;
    pub fn bpf_skb_load_bytes(skb: *mut __sk_buff, off: u32, to: *mut core::ffi::c_void, len: u32) -> i64;
    pub fn bpf_ktime_get_ns() -> u64;
    pub fn bpf_printk(fmt: *const u8, ...);
}

pub unsafe fn get_tcp_info(skb: *mut __sk_buff, pkti: *mut hbm_pkt_info) -> i32 {
    let mut sk = (*skb).sk;
    if !sk.is_null() {
        sk = bpf_sk_fullsock(sk);
        if !sk.is_null() && (*sk).protocol == 6 {
            let tp = bpf_tcp_sock(sk);
            if !tp.is_null() {
                (*pkti).cwnd = (*tp).snd_cwnd;
                (*pkti).rtt = ((*tp).srtt_us >> 3) as i32;
                (*pkti).packets_out = (*tp).packets_out;
                return 0;
            }
        }
    }
    (*pkti).cwnd = 0; (*pkti).rtt = 0; (*pkti).packets_out = 0; 1
}

pub unsafe fn hbm_get_pkt_info(skb: *mut __sk_buff, pkti: *mut hbm_pkt_info) {
    let mut iph = core::mem::MaybeUninit::<iphdr>::zeroed().assume_init();
    (*pkti).cwnd = 0; (*pkti).rtt = 0;
    bpf_skb_load_bytes(skb, 0, &mut iph as *mut _ as *mut core::ffi::c_void, 12);
    if iph.version == 6 {
        let ip6h = &*((&iph as *const iphdr) as *const ipv6hdr);
        (*pkti).is_ip = true; (*pkti).is_tcp = ip6h.nexthdr == 6;
        (*pkti).ecn = ((ip6h.flow_lbl[0] >> 4) & 3) as i16;
    } else if iph.version == 4 {
        (*pkti).is_ip = true; (*pkti).is_tcp = iph.protocol == 6; (*pkti).ecn = (iph.tos & 3) as i16;
    } else { (*pkti).is_ip = false; (*pkti).is_tcp = false; (*pkti).ecn = 0; }
    if (*pkti).is_tcp { get_tcp_info(skb, pkti); }
}

pub unsafe fn hbm_init_vqueue(qdp: *mut hbm_vqueue, rate: i32) {
    (*qdp).lasttime = bpf_ktime_get_ns(); (*qdp).credit = INIT_CREDIT; (*qdp).rate = rate * 128;
}
pub unsafe fn hbm_init_edt_vqueue(qdp: *mut hbm_vqueue, rate: i32) {
    let curtime = bpf_ktime_get_ns(); (*qdp).lasttime = curtime - BURST_SIZE_NS; (*qdp).credit = 0; (*qdp).rate = rate * 128;
}

// Atomic statistic updates and BPF map declarations are represented by the
// corresponding volatile operations and external map symbols in the BPF build.
pub unsafe fn hbm_update_stats(qsp: *mut hbm_queue_stats, len: i32, curtime: u64,
    congestion_flag: bool, drop_flag: bool, cwr_flag: bool, ecn_ce_flag: bool,
    pkti: *mut hbm_pkt_info, credit: i32) {
    if qsp.is_null() { return; }
    (*qsp).bytes_total = (*qsp).bytes_total.wrapping_add(len as i64);
    if !(*qsp).stats { return; }
    if (*qsp).firstPacketTime == 0 { (*qsp).firstPacketTime = curtime; }
    (*qsp).lastPacketTime = curtime; (*qsp).pkts_total += 1;
    if congestion_flag { (*qsp).pkts_marked += 1; (*qsp).bytes_marked += len as i64; }
    if drop_flag { (*qsp).pkts_dropped += 1; (*qsp).bytes_dropped += len as i64; }
    if ecn_ce_flag { (*qsp).pkts_ecn_ce += 1; }
    if (*pkti).cwnd != 0 { (*qsp).sum_cwnd += (*pkti).cwnd as i64; (*qsp).sum_cwnd_cnt += 1; }
    if (*pkti).rtt != 0 { (*qsp).sum_rtt += (*pkti).rtt as i64; }
    (*qsp).sum_credit += credit as i64;
    let mut rv = if drop_flag { DROP_PKT } else { ALLOW_PKT };
    if cwr_flag { rv |= 2; }
    if (0..=3).contains(&rv) { (*qsp).returnValCount[rv as usize] += 1; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
