// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta */

/* Translated from C implementation source.
 * Original includes supplied kernel/BPF types, helpers, endian helpers, and
 * test_iptunnel_common/bpf_kfunc declarations.
 */

const tcphdr_sz: usize = core::mem::size_of::<tcphdr>();
const udphdr_sz: usize = core::mem::size_of::<udphdr>();
const ethhdr_sz: usize = core::mem::size_of::<ethhdr>();
const iphdr_sz: usize = core::mem::size_of::<iphdr>();
const ipv6hdr_sz: usize = core::mem::size_of::<ipv6hdr>();

/* BPF map declaration translated from:
 * struct { __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY); __uint(max_entries, 256);
 *          __type(key, __u32); __type(value, __u64); } rxcnt SEC(".maps");
 */
#[unsafe(link_section = ".maps")]
static mut rxcnt: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_PERCPU_ARRAY,
    max_entries: 256,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u64>() as __u32,
};

/* BPF map declaration translated from:
 * struct { __uint(type, BPF_MAP_TYPE_HASH);
 *          __uint(max_entries, MAX_IPTNL_ENTRIES);
 *          __type(key, struct vip);
 *          __type(value, struct iptnl_info); } vip2tnl SEC(".maps");
 */
#[unsafe(link_section = ".maps")]
static mut vip2tnl: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: MAX_IPTNL_ENTRIES,
    key_size: core::mem::size_of::<vip>() as __u32,
    value_size: core::mem::size_of::<iptnl_info>() as __u32,
};

#[inline(always)]
unsafe fn count_tx(protocol: __u32) {
    let rxcnt_count: *mut __u64;

    rxcnt_count = bpf_map_lookup_elem(&raw mut rxcnt as *mut _, &protocol as *const _ as *const _) as *mut __u64;
    if !rxcnt_count.is_null() {
        *rxcnt_count = (*rxcnt_count).wrapping_add(1);
    }
}

#[inline(always)]
unsafe fn get_dport(trans_data: *mut core::ffi::c_void, protocol: __u8) -> i32 {
    let th: *mut tcphdr;
    let uh: *mut udphdr;

    match protocol as i32 {
        IPPROTO_TCP => {
            th = trans_data as *mut tcphdr;
            return (*th).dest as i32;
        }
        IPPROTO_UDP => {
            uh = trans_data as *mut udphdr;
            return (*uh).dest as i32;
        }
        _ => {
            return 0;
        }
    }
}

#[inline(always)]
unsafe fn set_ethhdr(
    new_eth: *mut ethhdr,
    old_eth: *const ethhdr,
    tnl: *const iptnl_info,
    h_proto: __be16,
) {
    core::ptr::copy_nonoverlapping(
        (*old_eth).h_dest.as_ptr(),
        (*new_eth).h_source.as_mut_ptr(),
        core::mem::size_of_val(&(*new_eth).h_source),
    );
    core::ptr::copy_nonoverlapping(
        (*tnl).dmac.as_ptr(),
        (*new_eth).h_dest.as_mut_ptr(),
        core::mem::size_of_val(&(*new_eth).h_dest),
    );
    (*new_eth).h_proto = h_proto;
}

#[inline(always)]
unsafe fn handle_ipv4(xdp: *mut xdp_md, xdp_ptr: *mut bpf_dynptr) -> i32 {
    let mut eth_buffer: [__u8; ethhdr_sz + iphdr_sz + ethhdr_sz] = [0; ethhdr_sz + iphdr_sz + ethhdr_sz];
    let mut iph_buffer_tcp: [__u8; iphdr_sz + tcphdr_sz] = [0; iphdr_sz + tcphdr_sz];
    let mut iph_buffer_udp: [__u8; iphdr_sz + udphdr_sz] = [0; iphdr_sz + udphdr_sz];
    let mut new_xdp_ptr: bpf_dynptr = core::mem::zeroed();
    let tnl: *mut iptnl_info;
    let new_eth: *mut ethhdr;
    let old_eth: *mut ethhdr;
    let mut iph: *mut iphdr;
    let mut next_iph: *mut __u16;
    let payload_len: __u16;
    let mut vip: vip = core::mem::zeroed();
    let dport: i32;
    let mut csum: __u32 = 0;
    let mut i: i32;

    if ethhdr_sz + iphdr_sz + tcphdr_sz > ((*xdp).data_end).wrapping_sub((*xdp).data) as usize {
        iph = bpf_dynptr_slice(
            xdp_ptr,
            ethhdr_sz as __u32,
            iph_buffer_udp.as_mut_ptr() as *mut _,
            core::mem::size_of_val(&iph_buffer_udp) as __u32,
        ) as *mut iphdr;
    } else {
        iph = bpf_dynptr_slice(
            xdp_ptr,
            ethhdr_sz as __u32,
            iph_buffer_tcp.as_mut_ptr() as *mut _,
            core::mem::size_of_val(&iph_buffer_tcp) as __u32,
        ) as *mut iphdr;
    }

    if iph.is_null() {
        return XDP_DROP;
    }

    dport = get_dport(iph.add(1) as *mut core::ffi::c_void, (*iph).protocol);
    if dport == -1 {
        return XDP_DROP;
    }

    vip.protocol = (*iph).protocol;
    vip.family = AF_INET;
    vip.daddr.v4 = (*iph).daddr;
    vip.dport = dport;
    payload_len = bpf_ntohs((*iph).tot_len);

    tnl = bpf_map_lookup_elem(&raw mut vip2tnl as *mut _, &vip as *const _ as *const _) as *mut iptnl_info;
    /* It only does v4-in-v4 */
    if tnl.is_null() || (*tnl).family != AF_INET {
        return XDP_PASS;
    }

    if bpf_xdp_adjust_head(xdp, 0 - iphdr_sz as i32) != 0 {
        return XDP_DROP;
    }

    bpf_dynptr_from_xdp(xdp, 0, &mut new_xdp_ptr);
    new_eth = bpf_dynptr_slice_rdwr(
        &mut new_xdp_ptr,
        0,
        eth_buffer.as_mut_ptr() as *mut _,
        core::mem::size_of_val(&eth_buffer) as __u32,
    ) as *mut ethhdr;
    if new_eth.is_null() {
        return XDP_DROP;
    }

    iph = new_eth.add(1) as *mut iphdr;
    old_eth = iph.add(1) as *mut ethhdr;

    set_ethhdr(new_eth, old_eth, tnl, bpf_htons(ETH_P_IP));

    if new_eth == eth_buffer.as_mut_ptr() as *mut ethhdr {
        bpf_dynptr_write(
            &mut new_xdp_ptr,
            0,
            eth_buffer.as_ptr() as *const _,
            core::mem::size_of_val(&eth_buffer) as __u32,
            0,
        );
    }

    (*iph).version = 4;
    (*iph).ihl = (iphdr_sz >> 2) as __u8;
    (*iph).frag_off = 0;
    (*iph).protocol = IPPROTO_IPIP as __u8;
    (*iph).check = 0;
    (*iph).tos = 0;
    (*iph).tot_len = bpf_htons(payload_len.wrapping_add(iphdr_sz as __u16));
    (*iph).daddr = (*tnl).daddr.v4;
    (*iph).saddr = (*tnl).saddr.v4;
    (*iph).ttl = 8;

    next_iph = iph as *mut __u16;
    i = 0;
    while i < (iphdr_sz >> 1) as i32 {
        csum = csum.wrapping_add(*next_iph as __u32);
        next_iph = next_iph.add(1);
        i += 1;
    }

    (*iph).check = !((csum & 0xffff).wrapping_add(csum >> 16)) as __u16;

    count_tx(vip.protocol as __u32);

    XDP_TX
}

#[inline(always)]
unsafe fn handle_ipv6(xdp: *mut xdp_md, xdp_ptr: *mut bpf_dynptr) -> i32 {
    let mut eth_buffer: [__u8; ethhdr_sz + ipv6hdr_sz + ethhdr_sz] = [0; ethhdr_sz + ipv6hdr_sz + ethhdr_sz];
    let mut ip6h_buffer_tcp: [__u8; ipv6hdr_sz + tcphdr_sz] = [0; ipv6hdr_sz + tcphdr_sz];
    let mut ip6h_buffer_udp: [__u8; ipv6hdr_sz + udphdr_sz] = [0; ipv6hdr_sz + udphdr_sz];
    let mut new_xdp_ptr: bpf_dynptr = core::mem::zeroed();
    let tnl: *mut iptnl_info;
    let new_eth: *mut ethhdr;
    let old_eth: *mut ethhdr;
    let mut ip6h: *mut ipv6hdr;
    let payload_len: __u16;
    let mut vip: vip = core::mem::zeroed();
    let dport: i32;

    if ethhdr_sz + iphdr_sz + tcphdr_sz > ((*xdp).data_end).wrapping_sub((*xdp).data) as usize {
        ip6h = bpf_dynptr_slice(
            xdp_ptr,
            ethhdr_sz as __u32,
            ip6h_buffer_udp.as_mut_ptr() as *mut _,
            core::mem::size_of_val(&ip6h_buffer_udp) as __u32,
        ) as *mut ipv6hdr;
    } else {
        ip6h = bpf_dynptr_slice(
            xdp_ptr,
            ethhdr_sz as __u32,
            ip6h_buffer_tcp.as_mut_ptr() as *mut _,
            core::mem::size_of_val(&ip6h_buffer_tcp) as __u32,
        ) as *mut ipv6hdr;
    }

    if ip6h.is_null() {
        return XDP_DROP;
    }

    dport = get_dport(ip6h.add(1) as *mut core::ffi::c_void, (*ip6h).nexthdr);
    if dport == -1 {
        return XDP_DROP;
    }

    vip.protocol = (*ip6h).nexthdr;
    vip.family = AF_INET6;
    core::ptr::copy_nonoverlapping(
        (*ip6h).daddr.s6_addr32.as_ptr(),
        vip.daddr.v6.as_mut_ptr(),
        core::mem::size_of_val(&vip.daddr) / core::mem::size_of::<__u32>(),
    );
    vip.dport = dport;
    payload_len = (*ip6h).payload_len;

    tnl = bpf_map_lookup_elem(&raw mut vip2tnl as *mut _, &vip as *const _ as *const _) as *mut iptnl_info;
    /* It only does v6-in-v6 */
    if tnl.is_null() || (*tnl).family != AF_INET6 {
        return XDP_PASS;
    }

    if bpf_xdp_adjust_head(xdp, 0 - ipv6hdr_sz as i32) != 0 {
        return XDP_DROP;
    }

    bpf_dynptr_from_xdp(xdp, 0, &mut new_xdp_ptr);
    new_eth = bpf_dynptr_slice_rdwr(
        &mut new_xdp_ptr,
        0,
        eth_buffer.as_mut_ptr() as *mut _,
        core::mem::size_of_val(&eth_buffer) as __u32,
    ) as *mut ethhdr;
    if new_eth.is_null() {
        return XDP_DROP;
    }

    ip6h = new_eth.add(1) as *mut ipv6hdr;
    old_eth = ip6h.add(1) as *mut ethhdr;

    set_ethhdr(new_eth, old_eth, tnl, bpf_htons(ETH_P_IPV6));

    if new_eth == eth_buffer.as_mut_ptr() as *mut ethhdr {
        bpf_dynptr_write(
            &mut new_xdp_ptr,
            0,
            eth_buffer.as_ptr() as *const _,
            core::mem::size_of_val(&eth_buffer) as __u32,
            0,
        );
    }

    (*ip6h).version = 6;
    (*ip6h).priority = 0;
    (*ip6h).flow_lbl = [0; core::mem::size_of_val(&(*ip6h).flow_lbl)];
    (*ip6h).payload_len = bpf_htons(bpf_ntohs(payload_len).wrapping_add(ipv6hdr_sz as __u16));
    (*ip6h).nexthdr = IPPROTO_IPV6 as __u8;
    (*ip6h).hop_limit = 8;
    core::ptr::copy_nonoverlapping(
        (*tnl).saddr.v6.as_ptr(),
        (*ip6h).saddr.s6_addr32.as_mut_ptr(),
        core::mem::size_of_val(&(*tnl).saddr.v6) / core::mem::size_of::<__u32>(),
    );
    core::ptr::copy_nonoverlapping(
        (*tnl).daddr.v6.as_ptr(),
        (*ip6h).daddr.s6_addr32.as_mut_ptr(),
        core::mem::size_of_val(&(*tnl).daddr.v6) / core::mem::size_of::<__u32>(),
    );

    count_tx(vip.protocol as __u32);

    XDP_TX
}

#[unsafe(link_section = "xdp")]
pub unsafe extern "C" fn _xdp_tx_iptunnel(xdp: *mut xdp_md) -> i32 {
    let mut buffer: [__u8; ethhdr_sz] = [0; ethhdr_sz];
    let mut ptr: bpf_dynptr = core::mem::zeroed();
    let eth: *mut ethhdr;
    let h_proto: __u16;

    bpf_dynptr_from_xdp(xdp, 0, &mut ptr);
    eth = bpf_dynptr_slice(
        &mut ptr,
        0,
        buffer.as_mut_ptr() as *mut _,
        core::mem::size_of_val(&buffer) as __u32,
    ) as *mut ethhdr;
    if eth.is_null() {
        return XDP_DROP;
    }

    h_proto = (*eth).h_proto;

    if h_proto == bpf_htons(ETH_P_IP) {
        return handle_ipv4(xdp, &mut ptr);
    } else if h_proto == bpf_htons(ETH_P_IPV6) {
        return handle_ipv6(xdp, &mut ptr);
    } else {
        return XDP_DROP;
    }
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";
