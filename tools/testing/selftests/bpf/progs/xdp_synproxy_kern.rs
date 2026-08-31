// SPDX-License-Identifier: LGPL-2.1 OR BSD-2-Clause
/* Copyright (c) 2022, NVIDIA CORPORATION & AFFILIATES. All rights reserved. */

/* C dependencies translated as external Rust dependencies:
 * vmlinux.h, bpf/bpf_helpers.h, bpf/bpf_endian.h, asm/errno.h,
 * bpf_compiler.h.
 */

pub const TC_ACT_OK: i32 = 0;
pub const TC_ACT_SHOT: i32 = 2;

pub const NSEC_PER_SEC: u64 = 1000000000;

pub const ETH_ALEN: usize = 6;
pub const ETH_P_IP: u16 = 0x0800;
pub const ETH_P_IPV6: u16 = 0x86DD;

pub const IP_MF: u16 = 0x2000;
pub const IP_OFFSET: u16 = 0x1fff;

pub const NEXTHDR_TCP: u8 = 6;

pub const TCPOPT_NOP: u32 = 1;
pub const TCPOPT_EOL: u8 = 0;
pub const TCPOPT_MSS: u32 = 2;
pub const TCPOPT_WINDOW: u8 = 3;
pub const TCPOPT_SACK_PERM: u8 = 4;
pub const TCPOPT_TIMESTAMP: u8 = 8;

pub const TCPOLEN_MSS: u32 = 4;
pub const TCPOLEN_WINDOW: u8 = 3;
pub const TCPOLEN_SACK_PERM: u8 = 2;
pub const TCPOLEN_TIMESTAMP: u8 = 10;

pub const TCP_TS_HZ: u64 = 1000;
pub const TS_OPT_WSCALE_MASK: u32 = 0xf;
pub const TS_OPT_SACK: u32 = 1 << 4;
pub const TS_OPT_ECN: u32 = 1 << 5;
pub const TSBITS: u32 = 6;
pub const TSMASK: u32 = ((1u32) << TSBITS) - 1;
pub const TCP_MAX_WSCALE: u8 = 14;

pub const IPV4_MAXLEN: usize = 60;
pub const TCP_MAXLEN: usize = 60;

pub const DEFAULT_MSS4: u16 = 1460;
pub const DEFAULT_MSS6: u16 = 1440;
pub const DEFAULT_WSCALE: u8 = 7;
pub const DEFAULT_TTL: u8 = 64;
pub const MAX_ALLOWED_PORTS: u32 = 8;

pub const MAX_PACKET_OFF: u32 = 0xffff;
pub const BPF_F_CURRENT_NETNS: i32 = -1;

/* BPF maps:
 * values: BPF_MAP_TYPE_ARRAY, key __u32, value __u64, max_entries 2, SEC(".maps")
 * allowed_ports: BPF_MAP_TYPE_ARRAY, key __u32, value __u16,
 *                max_entries MAX_ALLOWED_PORTS, SEC(".maps")
 */
extern "C" {
    static mut values: core::ffi::c_void;
    static mut allowed_ports: core::ffi::c_void;
}

/* Some symbols defined in net/netfilter/nf_conntrack_bpf.c are unavailable in
 * vmlinux.h if CONFIG_NF_CONNTRACK=m, so they are redefined locally.
 */
#[repr(C)]
pub struct bpf_ct_opts___local {
    pub netns_id: s32,
    pub error: s32,
    pub l4proto: u8,
    pub dir: u8,
    pub reserved: [u8; 2],
}

extern "C" {
    fn bpf_xdp_ct_lookup(
        xdp_ctx: *mut xdp_md,
        bpf_tuple: *mut bpf_sock_tuple,
        len_tuple: u32,
        opts: *mut bpf_ct_opts___local,
        len_opts: u32,
    ) -> *mut nf_conn;
    fn bpf_skb_ct_lookup(
        skb_ctx: *mut __sk_buff,
        bpf_tuple: *mut bpf_sock_tuple,
        len_tuple: u32,
        opts: *mut bpf_ct_opts___local,
        len_opts: u32,
    ) -> *mut nf_conn;
    fn bpf_ct_release(ct: *mut nf_conn);
}

#[inline(always)]
unsafe fn tcp_flag_word(tp: *mut tcphdr) -> *mut u32 {
    ((*tp).as_tcp_word_hdr()).words.as_mut_ptr().add(3)
}

#[inline(always)]
unsafe fn swap_eth_addr(a: *mut u8, b: *mut u8) {
    let mut tmp: [u8; ETH_ALEN] = [0; ETH_ALEN];

    core::ptr::copy_nonoverlapping(a, tmp.as_mut_ptr(), ETH_ALEN);
    core::ptr::copy_nonoverlapping(b, a, ETH_ALEN);
    core::ptr::copy_nonoverlapping(tmp.as_ptr(), b, ETH_ALEN);
}

#[inline(always)]
fn csum_fold(mut csum: u32) -> u16 {
    csum = (csum & 0xffff).wrapping_add(csum >> 16);
    csum = (csum & 0xffff).wrapping_add(csum >> 16);
    !(csum as u16)
}

#[inline(always)]
fn csum_tcpudp_magic(saddr: __be32, daddr: __be32, len: u32, proto: u8, csum: u32) -> u16 {
    let mut s: u64 = csum as u64;

    s = s.wrapping_add(saddr as u32 as u64);
    s = s.wrapping_add(daddr as u32 as u64);
    #[cfg(target_endian = "big")]
    {
        s = s.wrapping_add(proto as u64 + len as u64);
    }
    #[cfg(target_endian = "little")]
    {
        s = s.wrapping_add(((proto as u32).wrapping_add(len) << 8) as u64);
    }
    s = (s & 0xffffffff).wrapping_add(s >> 32);
    s = (s & 0xffffffff).wrapping_add(s >> 32);

    csum_fold(s as u32)
}

#[inline(always)]
unsafe fn csum_ipv6_magic(
    saddr: *const in6_addr,
    daddr: *const in6_addr,
    len: u32,
    proto: u8,
    csum: u32,
) -> u16 {
    let mut sum: u64 = csum as u64;
    let mut i: i32;

    i = 0;
    while i < 4 {
        sum = sum.wrapping_add((*saddr).in6_u.u6_addr32[i as usize] as u32 as u64);
        i += 1;
    }

    i = 0;
    while i < 4 {
        sum = sum.wrapping_add((*daddr).in6_u.u6_addr32[i as usize] as u32 as u64);
        i += 1;
    }

    /* Don't combine additions to avoid 32-bit overflow. */
    sum = sum.wrapping_add(bpf_htonl(len) as u64);
    sum = sum.wrapping_add(bpf_htonl(proto as u32) as u64);

    sum = (sum & 0xffffffff).wrapping_add(sum >> 32);
    sum = (sum & 0xffffffff).wrapping_add(sum >> 32);

    csum_fold(sum as u32)
}

#[inline(always)]
unsafe fn tcp_clock_ns() -> u64 {
    bpf_ktime_get_ns()
}

#[inline(always)]
unsafe fn tcp_ns_to_ts(ns: u64) -> u32 {
    (ns / (NSEC_PER_SEC / TCP_TS_HZ)) as u32
}

#[inline(always)]
unsafe fn tcp_clock_ms() -> u32 {
    tcp_ns_to_ts(tcp_clock_ns())
}

#[repr(C)]
pub struct tcpopt_context {
    pub data: *mut core::ffi::c_void,
    pub data_end: *mut core::ffi::c_void,
    pub tsecr: *mut __be32,
    pub wscale: u8,
    pub option_timestamp: bool,
    pub option_sack: bool,
    pub off: u32,
}

#[inline(always)]
unsafe fn next(ctx: *mut tcpopt_context, sz: u32) -> *mut u8 {
    let off: u64 = (*ctx).off as u64;
    let data: *mut u8;

    /* Verifier forbids access to packet when offset exceeds MAX_PACKET_OFF */
    if off > MAX_PACKET_OFF.wrapping_sub(sz) as u64 {
        return core::ptr::null_mut();
    }

    data = ((*ctx).data as *mut u8).add(off as usize);
    barrier_var(data);
    if data.add(sz as usize) >= (*ctx).data_end as *mut u8 {
        return core::ptr::null_mut();
    }

    (*ctx).off = (*ctx).off.wrapping_add(sz);
    data
}

unsafe fn tscookie_tcpopt_parse(ctx: *mut tcpopt_context) -> i32 {
    let opcode: *mut u8;
    let opsize: *mut u8;
    let wscale: *mut u8;
    let tsecr: *mut u8;
    let off: u32 = (*ctx).off;

    opcode = next(ctx, 1);
    if opcode.is_null() {
        return 1;
    }

    if *opcode == TCPOPT_EOL {
        return 1;
    }
    if *opcode == TCPOPT_NOP as u8 {
        return 0;
    }

    opsize = next(ctx, 1);
    if opsize.is_null() || *opsize < 2 {
        return 1;
    }

    match *opcode {
        TCPOPT_WINDOW => {
            wscale = next(ctx, 1);
            if wscale.is_null() {
                return 1;
            }
            if *opsize == TCPOLEN_WINDOW {
                (*ctx).wscale = if *wscale < TCP_MAX_WSCALE { *wscale } else { TCP_MAX_WSCALE };
            }
        }
        TCPOPT_TIMESTAMP => {
            tsecr = next(ctx, 4);
            if tsecr.is_null() {
                return 1;
            }
            if *opsize == TCPOLEN_TIMESTAMP {
                (*ctx).option_timestamp = true;
                /* Client's tsval becomes our tsecr. */
                *(*ctx).tsecr = core::ptr::read_unaligned(tsecr as *const __be32);
            }
        }
        TCPOPT_SACK_PERM => {
            if *opsize == TCPOLEN_SACK_PERM {
                (*ctx).option_sack = true;
            }
        }
        _ => {}
    }

    (*ctx).off = off.wrapping_add(*opsize as u32);

    0
}

unsafe extern "C" fn tscookie_tcpopt_parse_batch(_index: u32, context: *mut core::ffi::c_void) -> i32 {
    let mut i: i32;

    i = 0;
    while i < 7 {
        if tscookie_tcpopt_parse(context as *mut tcpopt_context) != 0 {
            return 1;
        }
        i += 1;
    }
    0
}

#[inline(always)]
unsafe fn tscookie_init(
    tcp_header: *mut tcphdr,
    _tcp_len: u16,
    tsval: *mut __be32,
    tsecr: *mut __be32,
    data: *mut core::ffi::c_void,
    data_end: *mut core::ffi::c_void,
) -> bool {
    let mut loop_ctx = tcpopt_context {
        data,
        data_end,
        tsecr,
        wscale: TS_OPT_WSCALE_MASK as u8,
        option_timestamp: false,
        option_sack: false,
        /* Note: currently verifier would track .off as unbound scalar.
         *       In case if verifier would at some point get smarter and
         *       compute bounded value for this var, beware that it might
         *       hinder bpf_loop() convergence validation.
         */
        off: (tcp_header.add(1) as *mut u8).offset_from(data as *mut u8) as u32,
    };
    let mut cookie: u32;

    bpf_loop(6, Some(tscookie_tcpopt_parse_batch), &mut loop_ctx as *mut _ as *mut core::ffi::c_void, 0);

    if !loop_ctx.option_timestamp {
        return false;
    }

    cookie = tcp_clock_ms() & !TSMASK;
    cookie |= loop_ctx.wscale as u32 & TS_OPT_WSCALE_MASK;
    if loop_ctx.option_sack {
        cookie |= TS_OPT_SACK;
    }
    if (*tcp_header).ece != 0 && (*tcp_header).cwr != 0 {
        cookie |= TS_OPT_ECN;
    }
    *tsval = bpf_htonl(cookie);

    true
}

#[inline(always)]
unsafe fn values_get_tcpipopts(mss: *mut u16, wscale: *mut u8, ttl: *mut u8, ipv6: bool) {
    let mut key: u32 = 0;
    let value: *mut u64;

    value = bpf_map_lookup_elem(&mut values as *mut _ as *mut core::ffi::c_void, &mut key as *mut _ as *mut core::ffi::c_void) as *mut u64;
    if !value.is_null() && *value != 0 {
        if ipv6 {
            *mss = ((*value >> 32) & 0xffff) as u16;
        } else {
            *mss = (*value & 0xffff) as u16;
        }
        *wscale = ((*value >> 16) & 0xf) as u8;
        *ttl = ((*value >> 24) & 0xff) as u8;
        return;
    }

    *mss = if ipv6 { DEFAULT_MSS6 } else { DEFAULT_MSS4 };
    *wscale = DEFAULT_WSCALE;
    *ttl = DEFAULT_TTL;
}

#[inline(always)]
unsafe fn values_inc_synacks() {
    let mut key: u32 = 1;
    let value: *mut u64;

    value = bpf_map_lookup_elem(&mut values as *mut _ as *mut core::ffi::c_void, &mut key as *mut _ as *mut core::ffi::c_void) as *mut u64;
    if !value.is_null() {
        core::intrinsics::atomic_xadd(value, 1);
    }
}

#[inline(always)]
unsafe fn check_port_allowed(port: u16) -> bool {
    let mut i: u32;

    i = 0;
    while i < MAX_ALLOWED_PORTS {
        let mut key: u32 = i;
        let value: *mut u16;

        value = bpf_map_lookup_elem(&mut allowed_ports as *mut _ as *mut core::ffi::c_void, &mut key as *mut _ as *mut core::ffi::c_void) as *mut u16;

        if value.is_null() {
            break;
        }
        /* 0 is a terminator value. Check it first to avoid matching on
         * a forbidden port == 0 and returning true.
         */
        if *value == 0 {
            break;
        }

        if *value == port {
            return true;
        }
        i += 1;
    }

    false
}

#[repr(C)]
pub struct header_pointers {
    pub eth: *mut ethhdr,
    pub ipv4: *mut iphdr,
    pub ipv6: *mut ipv6hdr,
    pub tcp: *mut tcphdr,
    pub tcp_len: u16,
}

#[inline(always)]
unsafe fn tcp_dissect(data: *mut core::ffi::c_void, data_end: *mut core::ffi::c_void, hdr: *mut header_pointers) -> i32 {
    (*hdr).eth = data as *mut ethhdr;
    if (*hdr).eth.add(1) as *mut core::ffi::c_void > data_end {
        return XDP_DROP;
    }

    match bpf_ntohs((*(*hdr).eth).h_proto) {
        ETH_P_IP => {
            (*hdr).ipv6 = core::ptr::null_mut();

            (*hdr).ipv4 = ((*hdr).eth as *mut u8).add(core::mem::size_of::<ethhdr>()) as *mut iphdr;
            if (*hdr).ipv4.add(1) as *mut core::ffi::c_void > data_end {
                return XDP_DROP;
            }
            if ((*(*hdr).ipv4).ihl as usize) * 4 < core::mem::size_of::<iphdr>() {
                return XDP_DROP;
            }
            if (*(*hdr).ipv4).version != 4 {
                return XDP_DROP;
            }

            if (*(*hdr).ipv4).protocol != IPPROTO_TCP {
                return XDP_PASS;
            }

            (*hdr).tcp = ((*hdr).ipv4 as *mut u8).add((*(*hdr).ipv4).ihl as usize * 4) as *mut tcphdr;
        }
        ETH_P_IPV6 => {
            (*hdr).ipv4 = core::ptr::null_mut();

            (*hdr).ipv6 = ((*hdr).eth as *mut u8).add(core::mem::size_of::<ethhdr>()) as *mut ipv6hdr;
            if (*hdr).ipv6.add(1) as *mut core::ffi::c_void > data_end {
                return XDP_DROP;
            }
            if (*(*hdr).ipv6).version != 6 {
                return XDP_DROP;
            }

            /* XXX: Extension headers are not supported and could circumvent
             * XDP SYN flood protection.
             */
            if (*(*hdr).ipv6).nexthdr != NEXTHDR_TCP {
                return XDP_PASS;
            }

            (*hdr).tcp = ((*hdr).ipv6 as *mut u8).add(core::mem::size_of::<ipv6hdr>()) as *mut tcphdr;
        }
        _ => {
            /* XXX: VLANs will circumvent XDP SYN flood protection. */
            return XDP_PASS;
        }
    }

    if (*hdr).tcp.add(1) as *mut core::ffi::c_void > data_end {
        return XDP_DROP;
    }
    (*hdr).tcp_len = ((*(*hdr).tcp).doff as u16) * 4;
    if ((*hdr).tcp_len as usize) < core::mem::size_of::<tcphdr>() {
        return XDP_DROP;
    }

    XDP_TX
}

#[inline(always)]
unsafe fn tcp_lookup(ctx: *mut core::ffi::c_void, hdr: *mut header_pointers, xdp: bool) -> i32 {
    let mut ct_lookup_opts = bpf_ct_opts___local {
        netns_id: BPF_F_CURRENT_NETNS,
        error: 0,
        l4proto: IPPROTO_TCP,
        dir: 0,
        reserved: [0; 2],
    };
    let mut tup: bpf_sock_tuple = core::mem::zeroed();
    let ct: *mut nf_conn;
    let tup_size: u32;

    if !(*hdr).ipv4.is_null() {
        /* TCP doesn't normally use fragments, and XDP can't reassemble
         * them.
         */
        if ((*(*hdr).ipv4).frag_off & bpf_htons((IP_MF | IP_OFFSET) as u16)) != 0 {
            return XDP_DROP;
        }

        tup.ipv4.saddr = (*(*hdr).ipv4).saddr;
        tup.ipv4.daddr = (*(*hdr).ipv4).daddr;
        tup.ipv4.sport = (*(*hdr).tcp).source;
        tup.ipv4.dport = (*(*hdr).tcp).dest;
        tup_size = core::mem::size_of_val(&tup.ipv4) as u32;
    } else if !(*hdr).ipv6.is_null() {
        core::ptr::copy_nonoverlapping(
            &(*(*hdr).ipv6).saddr as *const _ as *const u8,
            tup.ipv6.saddr.as_mut_ptr() as *mut u8,
            core::mem::size_of_val(&tup.ipv6.saddr),
        );
        core::ptr::copy_nonoverlapping(
            &(*(*hdr).ipv6).daddr as *const _ as *const u8,
            tup.ipv6.daddr.as_mut_ptr() as *mut u8,
            core::mem::size_of_val(&tup.ipv6.daddr),
        );
        tup.ipv6.sport = (*(*hdr).tcp).source;
        tup.ipv6.dport = (*(*hdr).tcp).dest;
        tup_size = core::mem::size_of_val(&tup.ipv6) as u32;
    } else {
        /* The verifier can't track that either ipv4 or ipv6 is not
         * NULL.
         */
        return XDP_ABORTED;
    }
    if xdp {
        ct = bpf_xdp_ct_lookup(ctx as *mut xdp_md, &mut tup, tup_size, &mut ct_lookup_opts, core::mem::size_of::<bpf_ct_opts___local>() as u32);
    } else {
        ct = bpf_skb_ct_lookup(ctx as *mut __sk_buff, &mut tup, tup_size, &mut ct_lookup_opts, core::mem::size_of::<bpf_ct_opts___local>() as u32);
    }
    if !ct.is_null() {
        let status: core::ffi::c_ulong = (*ct).status;

        bpf_ct_release(ct);
        if status & IPS_CONFIRMED != 0 {
            return XDP_PASS;
        }
    } else if ct_lookup_opts.error != -ENOENT {
        return XDP_ABORTED;
    }

    /* error == -ENOENT || !(status & IPS_CONFIRMED) */
    XDP_TX
}

#[inline(always)]
unsafe fn tcp_mkoptions(mut buf: *mut __be32, tsopt: *mut __be32, mss: u16, wscale: u8) -> u8 {
    let start: *mut __be32 = buf;

    *buf = bpf_htonl((TCPOPT_MSS << 24) | (TCPOLEN_MSS << 16) | mss as u32);
    buf = buf.add(1);

    if tsopt.is_null() {
        return buf.offset_from(start) as u8;
    }

    if *tsopt.add(0) & bpf_htonl(1 << 4) != 0 {
        *buf = bpf_htonl((TCPOPT_SACK_PERM << 24) | ((TCPOLEN_SACK_PERM as u32) << 16) | ((TCPOPT_TIMESTAMP as u32) << 8) | TCPOLEN_TIMESTAMP as u32);
        buf = buf.add(1);
    } else {
        *buf = bpf_htonl((TCPOPT_NOP << 24) | (TCPOPT_NOP << 16) | ((TCPOPT_TIMESTAMP as u32) << 8) | TCPOLEN_TIMESTAMP as u32);
        buf = buf.add(1);
    }
    *buf = *tsopt.add(0);
    buf = buf.add(1);
    *buf = *tsopt.add(1);
    buf = buf.add(1);

    if (*tsopt.add(0) & bpf_htonl(0xf)) != bpf_htonl(0xf) {
        *buf = bpf_htonl((TCPOPT_NOP << 24) | ((TCPOPT_WINDOW as u32) << 16) | ((TCPOLEN_WINDOW as u32) << 8) | wscale as u32);
        buf = buf.add(1);
    }

    buf.offset_from(start) as u8
}

#[inline(always)]
unsafe fn tcp_gen_synack(tcp_header: *mut tcphdr, cookie: u32, tsopt: *mut __be32, mss: u16, wscale: u8) {
    let tcp_options: *mut core::ffi::c_void;

    *tcp_flag_word(tcp_header) = TCP_FLAG_SYN | TCP_FLAG_ACK;
    if !tsopt.is_null() && (*tsopt.add(0) & bpf_htonl(1 << 5)) != 0 {
        *tcp_flag_word(tcp_header) |= TCP_FLAG_ECE;
    }
    (*tcp_header).doff = 5; /* doff is part of tcp_flag_word. */
    core::mem::swap(&mut (*tcp_header).source, &mut (*tcp_header).dest);
    (*tcp_header).ack_seq = bpf_htonl(bpf_ntohl((*tcp_header).seq).wrapping_add(1));
    (*tcp_header).seq = bpf_htonl(cookie);
    (*tcp_header).window = 0;
    (*tcp_header).urg_ptr = 0;
    (*tcp_header).check = 0; /* Calculate checksum later. */

    tcp_options = tcp_header.add(1) as *mut core::ffi::c_void;
    (*tcp_header).doff = (*tcp_header).doff.wrapping_add(tcp_mkoptions(tcp_options as *mut __be32, tsopt, mss, wscale));
}

#[inline(always)]
unsafe fn tcpv4_gen_synack(hdr: *mut header_pointers, cookie: u32, tsopt: *mut __be32) {
    let mut wscale: u8 = 0;
    let mut mss: u16 = 0;
    let mut ttl: u8 = 0;

    values_get_tcpipopts(&mut mss, &mut wscale, &mut ttl, false);

    swap_eth_addr((*(*hdr).eth).h_source.as_mut_ptr(), (*(*hdr).eth).h_dest.as_mut_ptr());

    core::mem::swap(&mut (*(*hdr).ipv4).saddr, &mut (*(*hdr).ipv4).daddr);
    (*(*hdr).ipv4).check = 0; /* Calculate checksum later. */
    (*(*hdr).ipv4).tos = 0;
    (*(*hdr).ipv4).id = 0;
    (*(*hdr).ipv4).ttl = ttl;

    tcp_gen_synack((*hdr).tcp, cookie, tsopt, mss, wscale);

    (*hdr).tcp_len = ((*(*hdr).tcp).doff as u16) * 4;
    (*(*hdr).ipv4).tot_len = bpf_htons((core::mem::size_of::<iphdr>() + (*hdr).tcp_len as usize) as u16);
}

#[inline(always)]
unsafe fn tcpv6_gen_synack(hdr: *mut header_pointers, cookie: u32, tsopt: *mut __be32) {
    let mut wscale: u8 = 0;
    let mut mss: u16 = 0;
    let mut ttl: u8 = 0;

    values_get_tcpipopts(&mut mss, &mut wscale, &mut ttl, true);

    swap_eth_addr((*(*hdr).eth).h_source.as_mut_ptr(), (*(*hdr).eth).h_dest.as_mut_ptr());

    core::mem::swap(&mut (*(*hdr).ipv6).saddr, &mut (*(*hdr).ipv6).daddr);
    *((*hdr).ipv6 as *mut __be32) = bpf_htonl(0x60000000);
    (*(*hdr).ipv6).hop_limit = ttl;

    tcp_gen_synack((*hdr).tcp, cookie, tsopt, mss, wscale);

    (*hdr).tcp_len = ((*(*hdr).tcp).doff as u16) * 4;
    (*(*hdr).ipv6).payload_len = bpf_htons((*hdr).tcp_len);
}

#[inline(always)]
unsafe fn syncookie_handle_syn(
    hdr: *mut header_pointers,
    ctx: *mut core::ffi::c_void,
    data: *mut core::ffi::c_void,
    data_end: *mut core::ffi::c_void,
    xdp: bool,
) -> i32 {
    let old_pkt_size: u32;
    let new_pkt_size: u32;
    /* Unlike clang 10, clang 11 and 12 generate code that doesn't pass the
     * BPF verifier if tsopt is not volatile. Volatile forces it to store
     * the pointer value and use it directly, otherwise tcp_mkoptions is
     * (mis)compiled like this:
     *   if (!tsopt)
     *       return buf - start;
     *   reg = stored_return_value_of_tscookie_init;
     *   if (reg)
     *       tsopt = tsopt_buf;
     *   else
     *       tsopt = NULL;
     *   ...
     *   *buf++ = tsopt[1];
     * It creates a dead branch where tsopt is assigned NULL, but the
     * verifier can't prove it's dead and blocks the program.
     */
    let mut tsopt: *mut __be32 = core::ptr::null_mut();
    let mut tsopt_buf: [__be32; 2] = [0; 2];
    let ip_len: u16;
    let cookie: u32;
    let mut value: i64;

    /* Checksum is not yet verified, but both checksum failure and TCP
     * header checks return XDP_DROP, so the order doesn't matter.
     */
    if (*(*hdr).tcp).fin != 0 || (*(*hdr).tcp).rst != 0 {
        return XDP_DROP;
    }

    /* Issue SYN cookies on allowed ports, drop SYN packets on blocked
     * ports.
     */
    if !check_port_allowed(bpf_ntohs((*(*hdr).tcp).dest)) {
        return XDP_DROP;
    }

    if !(*hdr).ipv4.is_null() {
        /* Check the IPv4 and TCP checksums before creating a SYNACK. */
        value = bpf_csum_diff(core::ptr::null_mut(), 0, (*hdr).ipv4 as *mut core::ffi::c_void, (*(*hdr).ipv4).ihl as u32 * 4, 0);
        if value < 0 {
            return XDP_ABORTED;
        }
        if csum_fold(value as u32) != 0 {
            return XDP_DROP; /* Bad IPv4 checksum. */
        }

        value = bpf_csum_diff(core::ptr::null_mut(), 0, (*hdr).tcp as *mut core::ffi::c_void, (*hdr).tcp_len as u32, 0);
        if value < 0 {
            return XDP_ABORTED;
        }
        if csum_tcpudp_magic((*(*hdr).ipv4).saddr, (*(*hdr).ipv4).daddr, (*hdr).tcp_len as u32, IPPROTO_TCP, value as u32) != 0 {
            return XDP_DROP; /* Bad TCP checksum. */
        }

        ip_len = core::mem::size_of::<iphdr>() as u16;

        value = bpf_tcp_raw_gen_syncookie_ipv4((*hdr).ipv4, (*hdr).tcp, (*hdr).tcp_len as u32);
    } else if !(*hdr).ipv6.is_null() {
        /* Check the TCP checksum before creating a SYNACK. */
        value = bpf_csum_diff(core::ptr::null_mut(), 0, (*hdr).tcp as *mut core::ffi::c_void, (*hdr).tcp_len as u32, 0);
        if value < 0 {
            return XDP_ABORTED;
        }
        if csum_ipv6_magic(&(*(*hdr).ipv6).saddr, &(*(*hdr).ipv6).daddr, (*hdr).tcp_len as u32, IPPROTO_TCP, value as u32) != 0 {
            return XDP_DROP; /* Bad TCP checksum. */
        }

        ip_len = core::mem::size_of::<ipv6hdr>() as u16;

        value = bpf_tcp_raw_gen_syncookie_ipv6((*hdr).ipv6, (*hdr).tcp, (*hdr).tcp_len as u32);
    } else {
        return XDP_ABORTED;
    }

    if value < 0 {
        return XDP_ABORTED;
    }
    cookie = value as u32;

    if tscookie_init((*hdr).tcp as *mut tcphdr, (*hdr).tcp_len, &mut tsopt_buf[0], &mut tsopt_buf[1], data, data_end) {
        core::ptr::write_volatile(&mut tsopt, tsopt_buf.as_mut_ptr());
    }

    /* Check that there is enough space for a SYNACK. It also covers
     * the check that the destination of the __builtin_memmove below
     * doesn't overflow.
     */
    if (data as *mut u8).add(core::mem::size_of::<ethhdr>() + ip_len as usize + TCP_MAXLEN) > data_end as *mut u8 {
        return XDP_ABORTED;
    }

    if !(*hdr).ipv4.is_null() {
        if ((*(*hdr).ipv4).ihl as usize) * 4 > core::mem::size_of::<iphdr>() {
            let new_tcp_header: *mut tcphdr;

            new_tcp_header = (data as *mut u8).add(core::mem::size_of::<ethhdr>() + core::mem::size_of::<iphdr>()) as *mut tcphdr;
            core::ptr::copy((*hdr).tcp, new_tcp_header, 1);
            (*hdr).tcp = new_tcp_header;

            (*(*hdr).ipv4).ihl = (core::mem::size_of::<iphdr>() / 4) as u8;
        }

        tcpv4_gen_synack(hdr, cookie, core::ptr::read_volatile(&tsopt));
    } else if !(*hdr).ipv6.is_null() {
        tcpv6_gen_synack(hdr, cookie, core::ptr::read_volatile(&tsopt));
    } else {
        return XDP_ABORTED;
    }

    /* Recalculate checksums. */
    (*(*hdr).tcp).check = 0;
    value = bpf_csum_diff(core::ptr::null_mut(), 0, (*hdr).tcp as *mut core::ffi::c_void, (*hdr).tcp_len as u32, 0);
    if value < 0 {
        return XDP_ABORTED;
    }
    if !(*hdr).ipv4.is_null() {
        (*(*hdr).tcp).check = csum_tcpudp_magic((*(*hdr).ipv4).saddr, (*(*hdr).ipv4).daddr, (*hdr).tcp_len as u32, IPPROTO_TCP, value as u32);

        (*(*hdr).ipv4).check = 0;
        value = bpf_csum_diff(core::ptr::null_mut(), 0, (*hdr).ipv4 as *mut core::ffi::c_void, core::mem::size_of::<iphdr>() as u32, 0);
        if value < 0 {
            return XDP_ABORTED;
        }
        (*(*hdr).ipv4).check = csum_fold(value as u32);
    } else if !(*hdr).ipv6.is_null() {
        (*(*hdr).tcp).check = csum_ipv6_magic(&(*(*hdr).ipv6).saddr, &(*(*hdr).ipv6).daddr, (*hdr).tcp_len as u32, IPPROTO_TCP, value as u32);
    } else {
        return XDP_ABORTED;
    }

    /* Set the new packet size. */
    old_pkt_size = (data_end as *mut u8).offset_from(data as *mut u8) as u32;
    new_pkt_size = (core::mem::size_of::<ethhdr>() + ip_len as usize + (*(*hdr).tcp).doff as usize * 4) as u32;
    if xdp {
        if bpf_xdp_adjust_tail(ctx as *mut xdp_md, new_pkt_size.wrapping_sub(old_pkt_size) as i32) != 0 {
            return XDP_ABORTED;
        }
    } else if bpf_skb_change_tail(ctx as *mut __sk_buff, new_pkt_size, 0) != 0 {
        return XDP_ABORTED;
    }

    values_inc_synacks();

    XDP_TX
}

#[inline(always)]
unsafe fn syncookie_handle_ack(hdr: *mut header_pointers) -> i32 {
    let err: i32;

    if (*(*hdr).tcp).rst != 0 {
        return XDP_DROP;
    }

    if !(*hdr).ipv4.is_null() {
        err = bpf_tcp_raw_check_syncookie_ipv4((*hdr).ipv4, (*hdr).tcp);
    } else if !(*hdr).ipv6.is_null() {
        err = bpf_tcp_raw_check_syncookie_ipv6((*hdr).ipv6, (*hdr).tcp);
    } else {
        return XDP_ABORTED;
    }
    if err != 0 {
        return XDP_DROP;
    }

    XDP_PASS
}

#[inline(always)]
unsafe fn syncookie_part1(
    ctx: *mut core::ffi::c_void,
    data: *mut core::ffi::c_void,
    data_end: *mut core::ffi::c_void,
    hdr: *mut header_pointers,
    xdp: bool,
) -> i32 {
    let mut ret: i32;

    ret = tcp_dissect(data, data_end, hdr);
    if ret != XDP_TX {
        return ret;
    }

    ret = tcp_lookup(ctx, hdr, xdp);
    if ret != XDP_TX {
        return ret;
    }

    /* Packet is TCP and doesn't belong to an established connection. */

    if (((*(*hdr).tcp).syn as u8) ^ ((*(*hdr).tcp).ack as u8)) != 1 {
        return XDP_DROP;
    }

    /* Grow the TCP header to TCP_MAXLEN to be able to pass any hdr->tcp_len
     * to bpf_tcp_raw_gen_syncookie_ipv{4,6} and pass the verifier.
     */
    if xdp {
        if bpf_xdp_adjust_tail(ctx as *mut xdp_md, (TCP_MAXLEN as i32).wrapping_sub((*hdr).tcp_len as i32)) != 0 {
            return XDP_ABORTED;
        }
    } else {
        /* Without volatile the verifier throws this error:
         * R9 32-bit pointer arithmetic prohibited
         */
        let old_len: u64 = (data_end as *mut u8).offset_from(data as *mut u8) as u64;

        if bpf_skb_change_tail(ctx as *mut __sk_buff, (old_len + TCP_MAXLEN as u64 - (*hdr).tcp_len as u64) as u32, 0) != 0 {
            return XDP_ABORTED;
        }
    }

    XDP_TX
}

#[inline(always)]
unsafe fn syncookie_part2(
    ctx: *mut core::ffi::c_void,
    data: *mut core::ffi::c_void,
    data_end: *mut core::ffi::c_void,
    hdr: *mut header_pointers,
    xdp: bool,
) -> i32 {
    if !(*hdr).ipv4.is_null() {
        (*hdr).eth = data as *mut ethhdr;
        (*hdr).ipv4 = ((*hdr).eth as *mut u8).add(core::mem::size_of::<ethhdr>()) as *mut iphdr;
        /* IPV4_MAXLEN is needed when calculating checksum.
         * At least sizeof(struct iphdr) is needed here to access ihl.
         */
        if ((*hdr).ipv4 as *mut u8).add(IPV4_MAXLEN) > data_end as *mut u8 {
            return XDP_ABORTED;
        }
        (*hdr).tcp = ((*hdr).ipv4 as *mut u8).add((*(*hdr).ipv4).ihl as usize * 4) as *mut tcphdr;
    } else if !(*hdr).ipv6.is_null() {
        (*hdr).eth = data as *mut ethhdr;
        (*hdr).ipv6 = ((*hdr).eth as *mut u8).add(core::mem::size_of::<ethhdr>()) as *mut ipv6hdr;
        (*hdr).tcp = ((*hdr).ipv6 as *mut u8).add(core::mem::size_of::<ipv6hdr>()) as *mut tcphdr;
    } else {
        return XDP_ABORTED;
    }

    if ((*hdr).tcp as *mut u8).add(TCP_MAXLEN) > data_end as *mut u8 {
        return XDP_ABORTED;
    }

    /* We run out of registers, tcp_len gets spilled to the stack, and the
     * verifier forgets its min and max values checked above in tcp_dissect.
     */
    (*hdr).tcp_len = ((*(*hdr).tcp).doff as u16) * 4;
    if ((*hdr).tcp_len as usize) < core::mem::size_of::<tcphdr>() {
        return XDP_ABORTED;
    }

    if (*(*hdr).tcp).syn != 0 {
        syncookie_handle_syn(hdr, ctx, data, data_end, xdp)
    } else {
        syncookie_handle_ack(hdr)
    }
}

#[no_mangle]
#[link_section = "xdp"]
pub unsafe extern "C" fn syncookie_xdp(ctx: *mut xdp_md) -> i32 {
    let mut data_end: *mut core::ffi::c_void = (*ctx).data_end as isize as *mut core::ffi::c_void;
    let mut data: *mut core::ffi::c_void = (*ctx).data as isize as *mut core::ffi::c_void;
    let mut hdr: header_pointers = core::mem::zeroed();
    let ret: i32;

    ret = syncookie_part1(ctx as *mut core::ffi::c_void, data, data_end, &mut hdr, true);
    if ret != XDP_TX {
        return ret;
    }

    data_end = (*ctx).data_end as isize as *mut core::ffi::c_void;
    data = (*ctx).data as isize as *mut core::ffi::c_void;

    syncookie_part2(ctx as *mut core::ffi::c_void, data, data_end, &mut hdr, true)
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn syncookie_tc(skb: *mut __sk_buff) -> i32 {
    let mut data_end: *mut core::ffi::c_void = (*skb).data_end as isize as *mut core::ffi::c_void;
    let mut data: *mut core::ffi::c_void = (*skb).data as isize as *mut core::ffi::c_void;
    let mut hdr: header_pointers = core::mem::zeroed();
    let mut ret: i32;

    ret = syncookie_part1(skb as *mut core::ffi::c_void, data, data_end, &mut hdr, false);
    if ret != XDP_TX {
        return if ret == XDP_PASS { TC_ACT_OK } else { TC_ACT_SHOT };
    }

    data_end = (*skb).data_end as isize as *mut core::ffi::c_void;
    data = (*skb).data as isize as *mut core::ffi::c_void;

    ret = syncookie_part2(skb as *mut core::ffi::c_void, data, data_end, &mut hdr, false);
    match ret {
        XDP_PASS => TC_ACT_OK,
        XDP_TX => bpf_redirect((*skb).ifindex, 0),
        _ => TC_ACT_SHOT,
    }
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";
