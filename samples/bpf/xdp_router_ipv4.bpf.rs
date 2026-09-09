/* Copyright (C) 2017 Cavium, Inc.
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of version 2 of the GNU General Public License
 * as published by the Free Software Foundation.
 */

// Dependencies supplied by vmlinux.h, xdp_sample.bpf.h, and xdp_sample_shared.h.

pub const ETH_ALEN: usize = 6;
pub const ETH_P_8021Q: u16 = 0x8100;
pub const ETH_P_8021AD: u16 = 0x88A8;

#[repr(C)]
pub struct trie_value {
    pub prefix: [u8; 4],
    pub value: u64,
    pub ifindex: i32,
    pub metric: i32,
    pub gw: u32,
}

#[repr(C)]
pub union key_4 {
    pub b32: [u32; 2],
    pub b8: [u8; 8],
}

#[repr(C)]
pub struct arp_entry {
    pub mac: u64,
    pub dst: u32,
}

#[repr(C)]
pub struct direct_map {
    pub arp: arp_entry,
    pub ifindex: i32,
    pub mac: u64,
}

// Map for trie implementation: BPF_MAP_TYPE_LPM_TRIE, key_size 8,
// value_size sizeof(struct trie_value), max_entries 50,
// map_flags BPF_F_NO_PREALLOC, SEC(".maps").
pub static mut lpm_map: core::ffi::c_void = core::ffi::c_void::zeroed();

// Map for ARP table: BPF_MAP_TYPE_HASH, key __be32, value __be64,
// max_entries 50, SEC(".maps").
pub static mut arp_table: core::ffi::c_void = core::ffi::c_void::zeroed();

// Map to keep the exact match entries in the route table:
// BPF_MAP_TYPE_HASH, key __be32, value struct direct_map, max_entries 50,
// SEC(".maps").
pub static mut exact_match: core::ffi::c_void = core::ffi::c_void::zeroed();

// BPF_MAP_TYPE_DEVMAP, key_size sizeof(int), value_size sizeof(int),
// max_entries 100, SEC(".maps").
pub static mut tx_port: core::ffi::c_void = core::ffi::c_void::zeroed();

#[allow(non_snake_case)]
pub unsafe fn xdp_router_ipv4_prog(ctx: *mut xdp_md) -> i32 {
    let data_end = ctx.data_end as *mut core::ffi::c_void;
    let data = ctx.data as *mut core::ffi::c_void;
    let eth = data as *mut ethhdr;
    let mut nh_off: u64 = core::mem::size_of::<ethhdr>() as u64;
    let mut rec: *mut datarec;
    let mut h_proto: u16;
    let key: u32 = 0;

    rec = bpf_map_lookup_elem(&mut rx_cnt as *mut _ as *mut _, &key);
    if !rec.is_null() {
        NO_TEAR_INC!((*rec).processed);
    }

    if (data as usize).wrapping_add(nh_off as usize) > data_end as usize {
        goto_drop!();
    }

    h_proto = (*eth).h_proto;
    if h_proto == bpf_htons(ETH_P_8021Q) || h_proto == bpf_htons(ETH_P_8021AD) {
        let vhdr = (data as usize).wrapping_add(nh_off as usize) as *mut vlan_hdr;
        nh_off += core::mem::size_of::<vlan_hdr>() as u64;
        if (data as usize).wrapping_add(nh_off as usize) > data_end as usize {
            goto_drop!();
        }
        h_proto = (*vhdr).h_vlan_encapsulated_proto;
    }

    match bpf_ntohs(h_proto) {
        ETH_P_ARP => {
            if !rec.is_null() { NO_TEAR_INC!((*rec).xdp_pass); }
            return XDP_PASS;
        }
        ETH_P_IP => {
            let iph = (data as usize).wrapping_add(nh_off as usize) as *mut iphdr;
            let mut src_mac: *mut u64;
            let mut dest_mac: *mut u64;
            let forward_to: i32;
            if (iph as usize).wrapping_add(core::mem::size_of::<iphdr>()) > data_end as usize {
                goto_drop!();
            }
            let direct_entry = bpf_map_lookup_elem(&mut exact_match as *mut _ as *mut _, &(*iph).daddr) as *mut direct_map;
            if !direct_entry.is_null() && (*direct_entry).mac != 0 && (*direct_entry).arp.mac != 0 {
                src_mac = &mut (*direct_entry).mac;
                dest_mac = &mut (*direct_entry).arp.mac;
                forward_to = (*direct_entry).ifindex;
            } else {
                let key4 = key_4 { b32: [32, 0] };
                (*(&key4 as *const _ as *mut key_4)).b8[4] = ((*iph).daddr & 0xff) as u8;
                (*(&key4 as *const _ as *mut key_4)).b8[5] = (((*iph).daddr >> 8) & 0xff) as u8;
                (*(&key4 as *const _ as *mut key_4)).b8[6] = (((*iph).daddr >> 16) & 0xff) as u8;
                (*(&key4 as *const _ as *mut key_4)).b8[7] = (((*iph).daddr >> 24) & 0xff) as u8;
                let prefix_value = bpf_map_lookup_elem(&mut lpm_map as *mut _ as *mut _, &key4) as *mut trie_value;
                if prefix_value.is_null() { goto_drop!(); }
                forward_to = (*prefix_value).ifindex;
                src_mac = &mut (*prefix_value).value;
                if src_mac.is_null() { goto_drop!(); }
                dest_mac = bpf_map_lookup_elem(&mut arp_table as *mut _ as *mut _, &(*iph).daddr) as *mut u64;
                if dest_mac.is_null() {
                    if (*prefix_value).gw == 0 { goto_drop!(); }
                    dest_mac = bpf_map_lookup_elem(&mut arp_table as *mut _ as *mut _, &(*prefix_value).gw) as *mut u64;
                    if dest_mac.is_null() {
                        if !rec.is_null() { NO_TEAR_INC!((*rec).xdp_pass); }
                        return XDP_PASS;
                    }
                }
            }
            if !src_mac.is_null() && !dest_mac.is_null() {
                __builtin_memcpy!((*eth).h_dest.as_mut_ptr(), dest_mac, ETH_ALEN);
                __builtin_memcpy!((*eth).h_source.as_mut_ptr(), src_mac, ETH_ALEN);
                let ret = bpf_redirect_map(&mut tx_port, forward_to, 0);
                if ret == XDP_REDIRECT {
                    if !rec.is_null() { NO_TEAR_INC!((*rec).xdp_redirect); }
                    return ret;
                }
            }
        }
        _ => {}
    }
    if !rec.is_null() { NO_TEAR_INC!((*rec).xdp_drop); }
    XDP_DROP
}

pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
