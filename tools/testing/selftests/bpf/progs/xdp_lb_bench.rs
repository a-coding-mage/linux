// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

/*
 * C dependencies translated as external Rust dependencies:
 * <stddef.h>, <stdbool.h>, <linux/bpf.h>, <linux/if_ether.h>,
 * <linux/ip.h>, <linux/ipv6.h>, <linux/in.h>, <linux/tcp.h>,
 * <linux/udp.h>, <bpf/bpf_helpers.h>, <bpf/bpf_endian.h>,
 * "bpf_compiler.h", "xdp_lb_bench_common.h",
 * "bench_bpf_timing.bpf.h"
 */

type __u8 = u8;
type __u16 = u16;
type __u32 = u32;
type __u64 = u64;
type __be32 = u32;

const IPPROTO_FRAGMENT: __u8 = 44;
const JHASH_INITVAL: __u32 = 0xdeadbeef;

/* jhash helpers */

#[inline(always)]
unsafe fn rol32(word: __u32, shift: u32) -> __u32 {
    word.wrapping_shl(shift) | word.wrapping_shr((0u32.wrapping_sub(shift)) & 31)
}

#[inline(always)]
unsafe fn __jhash_mix(a: &mut __u32, b: &mut __u32, c: &mut __u32) {
    *a = (*a).wrapping_sub(*c);
    *a ^= rol32(*c, 4);
    *c = (*c).wrapping_add(*b);
    *b = (*b).wrapping_sub(*a);
    *b ^= rol32(*a, 6);
    *a = (*a).wrapping_add(*c);
    *c = (*c).wrapping_sub(*b);
    *c ^= rol32(*b, 8);
    *b = (*b).wrapping_add(*a);
    *a = (*a).wrapping_sub(*c);
    *a ^= rol32(*c, 16);
    *c = (*c).wrapping_add(*b);
    *b = (*b).wrapping_sub(*a);
    *b ^= rol32(*a, 19);
    *a = (*a).wrapping_add(*c);
    *c = (*c).wrapping_sub(*b);
    *c ^= rol32(*b, 4);
    *b = (*b).wrapping_add(*a);
}

#[inline(always)]
unsafe fn __jhash_final(a: &mut __u32, b: &mut __u32, c: &mut __u32) {
    *c ^= *b;
    *c = (*c).wrapping_sub(rol32(*b, 14));
    *a ^= *c;
    *a = (*a).wrapping_sub(rol32(*c, 11));
    *b ^= *a;
    *b = (*b).wrapping_sub(rol32(*a, 25));
    *c ^= *b;
    *c = (*c).wrapping_sub(rol32(*b, 16));
    *a ^= *c;
    *a = (*a).wrapping_sub(rol32(*c, 4));
    *b ^= *a;
    *b = (*b).wrapping_sub(rol32(*a, 14));
    *c ^= *b;
    *c = (*c).wrapping_sub(rol32(*b, 24));
}

#[inline(always)]
unsafe fn __jhash_nwords(mut a: __u32, mut b: __u32, mut c: __u32, initval: __u32) -> __u32 {
    a = a.wrapping_add(initval);
    b = b.wrapping_add(initval);
    c = c.wrapping_add(initval);
    __jhash_final(&mut a, &mut b, &mut c);
    c
}

#[inline(always)]
unsafe fn jhash_2words(a: __u32, b: __u32, initval: __u32) -> __u32 {
    __jhash_nwords(a, b, 0, initval.wrapping_add(JHASH_INITVAL).wrapping_add(2 << 2))
}

#[inline(always)]
unsafe fn jhash2_4words(k: *const __u32, initval: __u32) -> __u32 {
    let mut a: __u32;
    let mut b: __u32;
    let mut c: __u32;

    c = JHASH_INITVAL.wrapping_add(4 << 2).wrapping_add(initval);
    b = c;
    a = b;

    a = a.wrapping_add(*k.add(0));
    b = b.wrapping_add(*k.add(1));
    c = c.wrapping_add(*k.add(2));
    __jhash_mix(&mut a, &mut b, &mut c);

    a = a.wrapping_add(*k.add(3));
    __jhash_final(&mut a, &mut b, &mut c);

    c
}

#[inline(always)]
unsafe fn ipv4_csum(iph: *mut iphdr) {
    let mut next_iph = iph as *mut __u16;
    let mut csum: __u32 = 0;
    let mut i: i32 = 0;

    while i < (core::mem::size_of::<iphdr>() >> 1) as i32 {
        csum = csum.wrapping_add(*next_iph as __u32);
        next_iph = next_iph.add(1);
        i += 1;
    }

    csum = (csum & 0xffff).wrapping_add(csum >> 16);
    csum = (csum & 0xffff).wrapping_add(csum >> 16);
    (*iph).check = !(csum as __u16);
}

/* BPF map declarations from the C source, preserving section intent. */
#[sec(".maps")]
static mut vip_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 64,
};

#[repr(C)]
struct lru_inner_map {
    type_: __u32,
    max_entries: __u32,
}

#[sec(".maps")]
static mut lru_inner: lru_inner_map = lru_inner_map {
    type_: BPF_MAP_TYPE_LRU_HASH,
    max_entries: DEFAULT_LRU_SIZE,
};

#[sec(".maps")]
static mut lru_mapping: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY_OF_MAPS,
    max_entries: BENCH_NR_CPUS,
};

#[sec(".maps")]
static mut ch_rings: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: CH_RINGS_SIZE,
};

#[sec(".maps")]
static mut reals: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: MAX_REALS,
};

#[sec(".maps")]
static mut stats: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_PERCPU_ARRAY,
    max_entries: STATS_SIZE,
};

#[sec(".maps")]
static mut reals_stats: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_PERCPU_ARRAY,
    max_entries: MAX_REALS,
};

#[sec(".maps")]
static mut ctl_array: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
};

#[sec(".maps")]
static mut vip_miss_stats: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
};

#[sec(".maps")]
static mut lru_miss_stats: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_PERCPU_ARRAY,
    max_entries: MAX_REALS,
};

static mut flow_mask: __u32 = 0;
static mut cold_lru: __u32 = 0;
static mut batch_gen: __u32 = 0;

/*
 * old_eth MUST be read BEFORE writing the outer header because
 * bpf_xdp_adjust_head makes them overlap.
 */
#[inline(always)]
unsafe fn encap_v4(
    xdp: *mut xdp_md,
    saddr: __be32,
    daddr: __be32,
    payload_len: __u16,
    dst_mac: *const __u8,
) -> i32 {
    let mut data: *mut core::ffi::c_void;
    let mut data_end: *mut core::ffi::c_void;
    let new_eth: *mut ethhdr;
    let old_eth: *mut ethhdr;
    let iph: *mut iphdr;

    if bpf_xdp_adjust_head(xdp, -(core::mem::size_of::<iphdr>() as i32)) != 0 {
        return -1;
    }

    data = (*xdp).data as usize as *mut core::ffi::c_void;
    data_end = (*xdp).data_end as usize as *mut core::ffi::c_void;

    new_eth = data as *mut ethhdr;
    iph = (data as *mut __u8).add(core::mem::size_of::<ethhdr>()) as *mut iphdr;
    old_eth = (data as *mut __u8).add(core::mem::size_of::<iphdr>()) as *mut ethhdr;

    if new_eth.add(1) as *mut core::ffi::c_void > data_end
        || old_eth.add(1) as *mut core::ffi::c_void > data_end
        || iph.add(1) as *mut core::ffi::c_void > data_end
    {
        return -1;
    }

    core::ptr::copy_nonoverlapping((*old_eth).h_dest.as_ptr(), (*new_eth).h_source.as_mut_ptr(), (*new_eth).h_source.len());
    core::ptr::copy_nonoverlapping(dst_mac, (*new_eth).h_dest.as_mut_ptr(), (*new_eth).h_dest.len());
    (*new_eth).h_proto = bpf_htons(ETH_P_IP as __u16);

    core::ptr::write_bytes(iph as *mut u8, 0, core::mem::size_of::<iphdr>());
    (*iph).version = 4;
    (*iph).ihl = (core::mem::size_of::<iphdr>() >> 2) as __u8;
    (*iph).protocol = IPPROTO_IPIP;
    (*iph).tot_len = bpf_htons(payload_len.wrapping_add(core::mem::size_of::<iphdr>() as __u16));
    (*iph).ttl = 64;
    (*iph).saddr = saddr;
    (*iph).daddr = daddr;
    ipv4_csum(iph);

    0
}

#[inline(always)]
unsafe fn encap_v6(
    xdp: *mut xdp_md,
    saddr: *const __be32,
    daddr: *const __be32,
    nexthdr: __u8,
    payload_len: __u16,
    dst_mac: *const __u8,
) -> i32 {
    let mut data: *mut core::ffi::c_void;
    let mut data_end: *mut core::ffi::c_void;
    let new_eth: *mut ethhdr;
    let old_eth: *mut ethhdr;
    let ip6h: *mut ipv6hdr;

    if bpf_xdp_adjust_head(xdp, -(core::mem::size_of::<ipv6hdr>() as i32)) != 0 {
        return -1;
    }

    data = (*xdp).data as usize as *mut core::ffi::c_void;
    data_end = (*xdp).data_end as usize as *mut core::ffi::c_void;

    new_eth = data as *mut ethhdr;
    ip6h = (data as *mut __u8).add(core::mem::size_of::<ethhdr>()) as *mut ipv6hdr;
    old_eth = (data as *mut __u8).add(core::mem::size_of::<ipv6hdr>()) as *mut ethhdr;

    if new_eth.add(1) as *mut core::ffi::c_void > data_end
        || old_eth.add(1) as *mut core::ffi::c_void > data_end
        || ip6h.add(1) as *mut core::ffi::c_void > data_end
    {
        return -1;
    }

    core::ptr::copy_nonoverlapping((*old_eth).h_dest.as_ptr(), (*new_eth).h_source.as_mut_ptr(), (*new_eth).h_source.len());
    core::ptr::copy_nonoverlapping(dst_mac, (*new_eth).h_dest.as_mut_ptr(), (*new_eth).h_dest.len());
    (*new_eth).h_proto = bpf_htons(ETH_P_IPV6 as __u16);

    core::ptr::write_bytes(ip6h as *mut u8, 0, core::mem::size_of::<ipv6hdr>());
    (*ip6h).version = 6;
    (*ip6h).nexthdr = nexthdr;
    (*ip6h).payload_len = bpf_htons(payload_len);
    (*ip6h).hop_limit = 64;
    core::ptr::copy_nonoverlapping(saddr as *const u8, &mut (*ip6h).saddr as *mut _ as *mut u8, core::mem::size_of_val(&(*ip6h).saddr));
    core::ptr::copy_nonoverlapping(daddr as *const u8, &mut (*ip6h).daddr as *mut _ as *mut u8, core::mem::size_of_val(&(*ip6h).daddr));

    0
}

#[inline(always)]
unsafe fn update_stats(map: *mut core::ffi::c_void, key: __u32, bytes: __u16) {
    let st = bpf_map_lookup_elem(map, &key as *const _ as *const core::ffi::c_void) as *mut lb_stats;

    if !st.is_null() {
        (*st).v1 = (*st).v1.wrapping_add(1);
        (*st).v2 = (*st).v2.wrapping_add(bytes as __u64);
    }
}

#[inline(always)]
unsafe fn count_action(action: i32) {
    let st: *mut lb_stats;
    let key: __u32;

    if action == XDP_TX {
        key = STATS_XDP_TX;
    } else if action == XDP_PASS {
        key = STATS_XDP_PASS;
    } else {
        key = STATS_XDP_DROP;
    }

    st = bpf_map_lookup_elem(&raw mut stats as *mut _, &key as *const _ as *const core::ffi::c_void) as *mut lb_stats;
    if !st.is_null() {
        (*st).v1 = (*st).v1.wrapping_add(1);
    }
}

#[inline(always)]
unsafe fn is_under_flood() -> bool {
    let key: __u32 = STATS_NEW_CONN;
    let conn_st = bpf_map_lookup_elem(&raw mut stats as *mut _, &key as *const _ as *const core::ffi::c_void) as *mut lb_stats;
    let cur_time: __u64;

    if conn_st.is_null() {
        return true;
    }

    cur_time = bpf_ktime_get_ns();
    if cur_time.wrapping_sub((*conn_st).v2) > ONE_SEC {
        (*conn_st).v1 = 1;
        (*conn_st).v2 = cur_time;
    } else {
        (*conn_st).v1 = (*conn_st).v1.wrapping_add(1);
        if (*conn_st).v1 > MAX_CONN_RATE {
            return true;
        }
    }
    false
}

#[inline(always)]
unsafe fn connection_table_lookup(
    lru_map: *mut core::ffi::c_void,
    flow: *mut flow_key,
    out_pos: *mut __u32,
) -> *mut real_definition {
    let dst_lru: *mut real_pos_lru;
    let real: *mut real_definition;
    let key: __u32;

    dst_lru = bpf_map_lookup_elem(lru_map, flow as *const core::ffi::c_void) as *mut real_pos_lru;
    if dst_lru.is_null() {
        return core::ptr::null_mut();
    }

    /* UDP connections use atime-based timeout instead of FIN/RST */
    if (*flow).proto == IPPROTO_UDP {
        let cur_time = bpf_ktime_get_ns();

        if cur_time.wrapping_sub((*dst_lru).atime) > LRU_UDP_TIMEOUT {
            return core::ptr::null_mut();
        }
        (*dst_lru).atime = cur_time;
    }

    key = (*dst_lru).pos;
    *out_pos = key;
    real = bpf_map_lookup_elem(&raw mut reals as *mut _, &key as *const _ as *const core::ffi::c_void) as *mut real_definition;
    real
}

#[inline(always)]
unsafe fn get_packet_dst(
    real: *mut *mut real_definition,
    flow: *mut flow_key,
    vip_info: *mut vip_meta,
    is_v6: bool,
    lru_map: *mut core::ffi::c_void,
    is_rst: bool,
    out_pos: *mut __u32,
) -> bool {
    let under_flood: bool;
    let hash: __u32;
    let ch_key: __u32;
    let ch_val: *mut __u32;
    let real_pos: __u32;

    under_flood = is_under_flood();

    if is_v6 {
        let src_hash = jhash2_4words((*flow).srcv6.as_ptr() as *const __u32, MAX_VIPS);

        hash = jhash_2words(src_hash, (*flow).ports, CH_RING_SIZE);
    } else {
        hash = jhash_2words((*flow).src, (*flow).ports, CH_RING_SIZE);
    }

    ch_key = CH_RING_SIZE
        .wrapping_mul((*vip_info).vip_num)
        .wrapping_add(hash % CH_RING_SIZE);
    ch_val = bpf_map_lookup_elem(&raw mut ch_rings as *mut _, &ch_key as *const _ as *const core::ffi::c_void) as *mut __u32;
    if ch_val.is_null() {
        return false;
    }
    real_pos = *ch_val;

    *real = bpf_map_lookup_elem(&raw mut reals as *mut _, &real_pos as *const _ as *const core::ffi::c_void) as *mut real_definition;
    if (*real).is_null() {
        return false;
    }

    if ((*vip_info).flags & F_LRU_BYPASS) == 0 && !under_flood && !is_rst {
        let mut new_lru = real_pos_lru {
            pos: real_pos,
            atime: 0,
        };

        if (*flow).proto == IPPROTO_UDP {
            new_lru.atime = bpf_ktime_get_ns();
        }
        bpf_map_update_elem(
            lru_map,
            flow as *const core::ffi::c_void,
            &new_lru as *const _ as *const core::ffi::c_void,
            BPF_ANY,
        );
    }

    *out_pos = real_pos;
    true
}

#[inline(always)]
unsafe fn update_vip_lru_miss_stats(vip: *mut vip_definition, is_v6: bool, real_idx: __u32) {
    let miss_vip: *mut vip_definition;
    let key: __u32 = 0;
    let cnt: *mut __u32;

    miss_vip = bpf_map_lookup_elem(&raw mut vip_miss_stats as *mut _, &key as *const _ as *const core::ffi::c_void) as *mut vip_definition;
    if miss_vip.is_null() {
        return;
    }

    if is_v6 {
        if (*miss_vip).vipv6[0] != (*vip).vipv6[0]
            || (*miss_vip).vipv6[1] != (*vip).vipv6[1]
            || (*miss_vip).vipv6[2] != (*vip).vipv6[2]
            || (*miss_vip).vipv6[3] != (*vip).vipv6[3]
        {
            return;
        }
    } else if (*miss_vip).vip != (*vip).vip {
        return;
    }

    if (*miss_vip).port != (*vip).port || (*miss_vip).proto != (*vip).proto {
        return;
    }

    cnt = bpf_map_lookup_elem(&raw mut lru_miss_stats as *mut _, &real_idx as *const _ as *const core::ffi::c_void) as *mut __u32;
    if !cnt.is_null() {
        *cnt = (*cnt).wrapping_add(1);
    }
}

#[inline(never)]
unsafe fn process_packet(xdp: *mut xdp_md) -> i32 {
    let mut data = (*xdp).data as usize as *mut core::ffi::c_void;
    let data_end = (*xdp).data_end as usize as *mut core::ffi::c_void;
    let eth = data as *mut ethhdr;
    let mut dst: *mut real_definition = core::ptr::null_mut();
    let mut vip_def: vip_definition = core::mem::zeroed();
    let cval: *mut ctl_value;
    let mut flow: flow_key = core::mem::zeroed();
    let vip_info: *mut vip_meta;
    let data_stats: *mut lb_stats;
    let uh: *mut udphdr;
    let mut tnl_src: [__be32; 4] = [0; 4];
    let lru_map: *mut core::ffi::c_void;
    let l4: *mut core::ffi::c_void;
    let payload_len: __u16;
    let mut real_pos: __u32 = 0;
    let cpu_num: __u32;
    let mut key: __u32;
    let proto: __u8;
    let mut action: i32 = XDP_DROP;
    let is_v6: bool;
    let mut is_syn: bool = false;
    let mut is_rst: bool = false;

    if eth.add(1) as *mut core::ffi::c_void > data_end {
        count_action(action);
        return action;
    }

    if (*eth).h_proto == bpf_htons(ETH_P_IPV6 as __u16) {
        is_v6 = true;
    } else if (*eth).h_proto == bpf_htons(ETH_P_IP as __u16) {
        is_v6 = false;
    } else {
        action = XDP_PASS;
        count_action(action);
        return action;
    }

    if is_v6 {
        let ip6h = (eth.add(1)) as *mut ipv6hdr;

        if ip6h.add(1) as *mut core::ffi::c_void > data_end {
            count_action(action);
            return action;
        }
        if (*ip6h).nexthdr == IPPROTO_FRAGMENT {
            count_action(action);
            return action;
        }

        payload_len = (core::mem::size_of::<ipv6hdr>() as __u16).wrapping_add(bpf_ntohs((*ip6h).payload_len));
        proto = (*ip6h).nexthdr;

        core::ptr::copy_nonoverlapping(&(*ip6h).saddr as *const _ as *const u8, flow.srcv6.as_mut_ptr() as *mut u8, core::mem::size_of_val(&flow.srcv6));
        core::ptr::copy_nonoverlapping(&(*ip6h).daddr as *const _ as *const u8, flow.dstv6.as_mut_ptr() as *mut u8, core::mem::size_of_val(&flow.dstv6));
        core::ptr::copy_nonoverlapping(&(*ip6h).daddr as *const _ as *const u8, vip_def.vipv6.as_mut_ptr() as *mut u8, core::mem::size_of_val(&vip_def.vipv6));
        l4 = ip6h.add(1) as *mut core::ffi::c_void;
    } else {
        let iph = (eth.add(1)) as *mut iphdr;

        if iph.add(1) as *mut core::ffi::c_void > data_end {
            count_action(action);
            return action;
        }
        if (*iph).ihl != 5 {
            count_action(action);
            return action;
        }
        if ((*iph).frag_off & bpf_htons(PCKT_FRAGMENTED as __u16)) != 0 {
            count_action(action);
            return action;
        }

        payload_len = bpf_ntohs((*iph).tot_len);
        proto = (*iph).protocol;

        flow.src = (*iph).saddr;
        flow.dst = (*iph).daddr;
        vip_def.vip = (*iph).daddr;
        l4 = iph.add(1) as *mut core::ffi::c_void;
    }

    /* TCP and UDP share the same port layout at offset 0 */
    if proto != IPPROTO_TCP && proto != IPPROTO_UDP {
        action = XDP_PASS;
        count_action(action);
        return action;
    }

    uh = l4 as *mut udphdr;
    if uh.add(1) as *mut core::ffi::c_void > data_end {
        count_action(action);
        return action;
    }
    flow.port16[0] = (*uh).source;
    flow.port16[1] = (*uh).dest;

    if proto == IPPROTO_TCP {
        let th = l4 as *mut tcphdr;

        if th.add(1) as *mut core::ffi::c_void > data_end {
            count_action(action);
            return action;
        }
        is_syn = (*th).syn != 0;
        is_rst = (*th).rst != 0;
    }

    flow.proto = proto;
    vip_def.port = flow.port16[1];
    vip_def.proto = proto;

    vip_info = bpf_map_lookup_elem(&raw mut vip_map as *mut _, &vip_def as *const _ as *const core::ffi::c_void) as *mut vip_meta;
    if vip_info.is_null() {
        action = XDP_PASS;
        count_action(action);
        return action;
    }

    key = STATS_LRU;
    data_stats = bpf_map_lookup_elem(&raw mut stats as *mut _, &key as *const _ as *const core::ffi::c_void) as *mut lb_stats;
    if data_stats.is_null() {
        count_action(action);
        return action;
    }
    (*data_stats).v1 = (*data_stats).v1.wrapping_add(1);

    cpu_num = bpf_get_smp_processor_id();
    lru_map = bpf_map_lookup_elem(&raw mut lru_mapping as *mut _, &cpu_num as *const _ as *const core::ffi::c_void);
    if lru_map.is_null() {
        count_action(action);
        return action;
    }

    if ((*vip_info).flags & F_LRU_BYPASS) == 0 && !is_syn {
        dst = connection_table_lookup(lru_map, &mut flow, &mut real_pos);
    }

    if dst.is_null() {
        if flow.proto == IPPROTO_TCP {
            let miss_st: *mut lb_stats;

            key = STATS_LRU_MISS;
            miss_st = bpf_map_lookup_elem(&raw mut stats as *mut _, &key as *const _ as *const core::ffi::c_void) as *mut lb_stats;
            if !miss_st.is_null() {
                (*miss_st).v1 = (*miss_st).v1.wrapping_add(1);
            }
        }

        if !get_packet_dst(&mut dst, &mut flow, vip_info, is_v6, lru_map, is_rst, &mut real_pos) {
            count_action(action);
            return action;
        }

        update_vip_lru_miss_stats(&mut vip_def, is_v6, real_pos);
        (*data_stats).v2 = (*data_stats).v2.wrapping_add(1);
    }

    key = 0;
    cval = bpf_map_lookup_elem(&raw mut ctl_array as *mut _, &key as *const _ as *const core::ffi::c_void) as *mut ctl_value;
    if cval.is_null() {
        count_action(action);
        return action;
    }

    update_stats(&raw mut stats as *mut _, (*vip_info).vip_num, payload_len);
    update_stats(&raw mut reals_stats as *mut _, real_pos, payload_len);

    if is_v6 {
        create_encap_ipv6_src(flow.port16[0], flow.srcv6[0], tnl_src.as_mut_ptr());
        if encap_v6(xdp, tnl_src.as_ptr(), (*dst).dstv6.as_ptr(), IPPROTO_IPV6, payload_len, (*cval).mac.as_ptr()) != 0 {
            count_action(action);
            return action;
        }
    } else if ((*dst).flags & F_IPV6) != 0 {
        create_encap_ipv6_src(flow.port16[0], flow.src, tnl_src.as_mut_ptr());
        if encap_v6(xdp, tnl_src.as_ptr(), (*dst).dstv6.as_ptr(), IPPROTO_IPIP, payload_len, (*cval).mac.as_ptr()) != 0 {
            count_action(action);
            return action;
        }
    } else if encap_v4(
        xdp,
        create_encap_ipv4_src(flow.port16[0], flow.src),
        (*dst).dst,
        payload_len,
        (*cval).mac.as_ptr(),
    ) != 0 {
        count_action(action);
        return action;
    }

    action = XDP_TX;

    count_action(action);
    action
}

#[inline(always)]
unsafe fn strip_encap(xdp: *mut xdp_md, saved_eth: *const ethhdr) -> i32 {
    let mut data = (*xdp).data as usize as *mut core::ffi::c_void;
    let mut data_end = (*xdp).data_end as usize as *mut core::ffi::c_void;
    let mut eth = data as *mut ethhdr;
    let hdr_sz: i32;

    if eth.add(1) as *mut core::ffi::c_void > data_end {
        return -1;
    }

    hdr_sz = if (*eth).h_proto == bpf_htons(ETH_P_IPV6 as __u16) {
        core::mem::size_of::<ipv6hdr>() as i32
    } else {
        core::mem::size_of::<iphdr>() as i32
    };

    if bpf_xdp_adjust_head(xdp, hdr_sz) != 0 {
        return -1;
    }

    data = (*xdp).data as usize as *mut core::ffi::c_void;
    data_end = (*xdp).data_end as usize as *mut core::ffi::c_void;
    eth = data as *mut ethhdr;

    if eth.add(1) as *mut core::ffi::c_void > data_end {
        return -1;
    }

    core::ptr::copy_nonoverlapping(saved_eth, eth, 1);
    0
}

#[inline(always)]
unsafe fn randomize_src(xdp: *mut xdp_md, saddr_off: i32, rand_state: *mut __u32) {
    let data = (*xdp).data as usize as *mut core::ffi::c_void;
    let data_end = (*xdp).data_end as usize as *mut core::ffi::c_void;
    let saddr = (data as *mut __u8).add(saddr_off as usize) as *mut __u32;

    *rand_state ^= (*rand_state).wrapping_shl(13);
    *rand_state ^= (*rand_state).wrapping_shr(17);
    *rand_state ^= (*rand_state).wrapping_shl(5);

    if saddr.add(1) as *mut core::ffi::c_void <= data_end {
        *saddr = *rand_state & flow_mask;
    }
}

#[sec("xdp")]
pub unsafe extern "C" fn xdp_lb_bench(xdp: *mut xdp_md) -> i32 {
    let data = (*xdp).data as usize as *mut core::ffi::c_void;
    let data_end = (*xdp).data_end as usize as *mut core::ffi::c_void;
    let eth = data as *mut ethhdr;
    let mut saved_eth: ethhdr = core::mem::zeroed();
    let mut rand_state: __u32 = 0;
    let mut batch_hash: __u32 = 0;
    let mut saddr_off: i32 = 0;
    let is_v6: bool;

    if eth.add(1) as *mut core::ffi::c_void > data_end {
        return XDP_DROP;
    }

    core::ptr::copy_nonoverlapping(eth, &mut saved_eth, 1);

    is_v6 = saved_eth.h_proto == bpf_htons(ETH_P_IPV6 as __u16);

    saddr_off = core::mem::size_of::<ethhdr>() as i32
        + if is_v6 {
            offset_of!(ipv6hdr, saddr) as i32
        } else {
            offset_of!(iphdr, saddr) as i32
        };

    if flow_mask != 0 {
        rand_state = bpf_get_prandom_u32() | 1;
    }

    if cold_lru != 0 {
        let saddr = (data as *mut __u8).add(saddr_off as usize) as *mut __u32;

        batch_gen = batch_gen.wrapping_add(1);
        batch_hash = batch_gen
            .wrapping_add(bpf_get_smp_processor_id())
            .wrapping_mul(KNUTH_HASH_MULT);
        if saddr.add(1) as *mut core::ffi::c_void <= data_end {
            *saddr ^= batch_hash;
        }
    }

    /*
     * BENCH_BPF_LOOP(
     *     process_packet(xdp),
     *     post-iteration block below
     * )
     */
    BENCH_BPF_LOOP!(process_packet(xdp), {
        if __bench_result == XDP_TX {
            if strip_encap(xdp, &saved_eth) != 0 {
                return XDP_DROP;
            }
            if rand_state != 0 {
                randomize_src(xdp, saddr_off, &mut rand_state);
            }
        }
        if cold_lru != 0 {
            let d = (*xdp).data as usize as *mut core::ffi::c_void;
            let de = (*xdp).data_end as usize as *mut core::ffi::c_void;
            let __sa = (d as *mut __u8).add(saddr_off as usize) as *mut __u32;

            if __sa.add(1) as *mut core::ffi::c_void <= de {
                *__sa ^= batch_hash;
            }
        }
    })
}

#[sec("license")]
static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
