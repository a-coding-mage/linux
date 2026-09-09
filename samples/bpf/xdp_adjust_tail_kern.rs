/* SPDX-License-Identifier: GPL-2.0
 * Copyright (c) 2018 Facebook
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General Public
 * License as published by the Free Software Foundation.
 *
 * This program shows how to use bpf_xdp_adjust_tail() by
 * generating ICMPv4 "packet to big" (unreachable/ df bit set frag needed
 * to be more preice in case of v4) where receiving packets bigger then
 * 600 bytes.
 */

// C headers provide the BPF, networking, and helper declarations used below.

const DEFAULT_TTL: u8 = 64;
const MAX_PCKT_SIZE: u32 = 600;
const ICMP_TOOBIG_SIZE: usize = 98;
const ICMP_TOOBIG_PAYLOAD_SIZE: i32 = 92;

/* volatile to prevent compiler optimizations */
static mut max_pcktsz: u32 = MAX_PCKT_SIZE;

// The map definition is supplied by the BPF toolchain.
#[allow(non_upper_case_globals)]
static mut icmpcnt: BpfArrayMap<u32, u64, 1> = BpfArrayMap::new();

#[inline(always)]
unsafe fn count_icmp() {
    let key: u64 = 0;
    let icmp_count: *mut u64 = bpf_map_lookup_elem(
        &mut icmpcnt as *mut _ as *mut core::ffi::c_void,
        &key as *const _ as *const core::ffi::c_void,
    ) as *mut u64;
    if !icmp_count.is_null() {
        *icmp_count = (*icmp_count).wrapping_add(1);
    }
}

#[inline(always)]
unsafe fn swap_mac(data: *mut core::ffi::c_void, orig_eth: *const ethhdr) {
    let eth = data as *mut ethhdr;
    core::ptr::copy_nonoverlapping((*orig_eth).h_dest.as_ptr(), (*eth).h_source.as_mut_ptr(), ETH_ALEN);
    core::ptr::copy_nonoverlapping((*orig_eth).h_source.as_ptr(), (*eth).h_dest.as_mut_ptr(), ETH_ALEN);
    (*eth).h_proto = (*orig_eth).h_proto;
}

#[inline(always)]
fn csum_fold_helper(mut csum: u32) -> u16 {
    csum = (csum & 0xffff).wrapping_add(csum >> 16);
    (!(csum & 0xffff).wrapping_add(csum >> 16)) as u16
}

#[inline(always)]
unsafe fn ipv4_csum(data_start: *const core::ffi::c_void, data_size: i32, csum: &mut u32) {
    *csum = bpf_csum_diff(core::ptr::null(), 0, data_start, data_size, *csum);
    *csum = csum_fold_helper(*csum) as u32;
}

#[inline(always)]
unsafe fn send_icmp4_too_big(xdp: *mut xdp_md) -> i32 {
    let headroom: i32 = core::mem::size_of::<iphdr>() as i32 + core::mem::size_of::<icmphdr>() as i32;

    if bpf_xdp_adjust_head(xdp, 0i32.wrapping_sub(headroom)) != 0 {
        return XDP_DROP;
    }
    let data = (*xdp).data as *mut u8;
    let data_end = (*xdp).data_end as *mut u8;
    if data.add(ICMP_TOOBIG_SIZE + headroom as usize) > data_end {
        return XDP_DROP;
    }

    let mut off: usize = 0;
    let mut csum: u32 = 0;
    let orig_eth = data.add(headroom as usize) as *const ethhdr;
    swap_mac(data as *mut core::ffi::c_void, orig_eth);
    off += core::mem::size_of::<ethhdr>();
    let iph = data.add(off) as *mut iphdr;
    off += core::mem::size_of::<iphdr>();
    let icmp_hdr = data.add(off) as *mut icmphdr;
    off += core::mem::size_of::<icmphdr>();
    let orig_iph = data.add(off) as *const iphdr;
    (*icmp_hdr).type_ = ICMP_DEST_UNREACH;
    (*icmp_hdr).code = ICMP_FRAG_NEEDED;
    (*icmp_hdr).un.frag.mtu = htons(max_pcktsz.wrapping_sub(core::mem::size_of::<ethhdr>() as u32));
    (*icmp_hdr).checksum = 0;
    ipv4_csum(icmp_hdr as *const _, ICMP_TOOBIG_PAYLOAD_SIZE, &mut csum);
    (*icmp_hdr).checksum = csum as _;
    (*iph).ttl = DEFAULT_TTL;
    (*iph).daddr = (*orig_iph).saddr;
    (*iph).saddr = (*orig_iph).daddr;
    (*iph).version = 4;
    (*iph).ihl = 5;
    (*iph).protocol = IPPROTO_ICMP;
    (*iph).tos = 0;
    (*iph).tot_len = htons((ICMP_TOOBIG_SIZE + headroom as usize - core::mem::size_of::<ethhdr>()) as u16);
    (*iph).check = 0;
    csum = 0;
    ipv4_csum(iph as *const _, core::mem::size_of::<iphdr>() as i32, &mut csum);
    (*iph).check = csum as _;
    count_icmp();
    XDP_TX
}

#[inline(always)]
unsafe fn handle_ipv4(xdp: *mut xdp_md) -> i32 {
    let data_end = (*xdp).data_end as *mut u8;
    let data = (*xdp).data as *mut u8;
    let pckt_size = data_end.offset_from(data) as i32;
    if pckt_size as u32 > core::cmp::max(max_pcktsz, ICMP_TOOBIG_SIZE as u32) {
        let offset = pckt_size - ICMP_TOOBIG_SIZE as i32;
        if bpf_xdp_adjust_tail(xdp, 0i32.wrapping_sub(offset)) != 0 {
            return XDP_PASS;
        }
        return send_icmp4_too_big(xdp);
    }
    XDP_PASS
}

#[no_mangle]
pub unsafe extern "C" fn _xdp_icmp(xdp: *mut xdp_md) -> i32 {
    let data_end = (*xdp).data_end as *mut u8;
    let data = (*xdp).data as *mut u8;
    let eth = data as *const ethhdr;
    if eth.add(1) as *const u8 > data_end {
        return XDP_DROP;
    }
    let h_proto = (*eth).h_proto;
    if h_proto == htons(ETH_P_IP) { handle_ipv4(xdp) } else { XDP_PASS }
}

#[used]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
