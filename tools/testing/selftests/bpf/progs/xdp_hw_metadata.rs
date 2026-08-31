// SPDX-License-Identifier: GPL-2.0

// Translated from C. Original dependencies:
// <vmlinux.h>, "xdp_metadata.h", <bpf/bpf_helpers.h>, <bpf/bpf_endian.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::c_void;
use core::mem::size_of;
use core::ptr;

#[repr(C)]
pub struct bpf_map_def {
    pub type_: __u32,
    pub max_entries: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
}

#[link_section = ".maps"]
#[used]
pub static mut xsk: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_XSKMAP,
    max_entries: 256,
    key_size: size_of::<__u32>() as __u32,
    value_size: size_of::<__u32>() as __u32,
};

pub static mut pkts_skip: __u64 = 0;
pub static mut pkts_fail: __u64 = 0;
pub static mut pkts_redir: __u64 = 0;

extern "C" {
    #[link_name = "bpf_xdp_metadata_rx_timestamp"]
    fn bpf_xdp_metadata_rx_timestamp(ctx: *const xdp_md, timestamp: *mut __u64) -> i32;

    #[link_name = "bpf_xdp_metadata_rx_hash"]
    fn bpf_xdp_metadata_rx_hash(
        ctx: *const xdp_md,
        hash: *mut __u32,
        rss_type: *mut xdp_rss_hash_type,
    ) -> i32;

    #[link_name = "bpf_xdp_metadata_rx_vlan_tag"]
    fn bpf_xdp_metadata_rx_vlan_tag(
        ctx: *const xdp_md,
        vlan_proto: *mut __be16,
        vlan_tci: *mut __u16,
    ) -> i32;

    fn bpf_xdp_adjust_meta(ctx: *mut xdp_md, delta: i32) -> i32;
    fn bpf_ktime_get_tai_ns() -> __u64;
    fn bpf_redirect_map(map: *mut bpf_map_def, key: __u32, flags: __u64) -> i32;
}

unsafe fn __sync_add_and_fetch_u64(ptr: *mut __u64, val: __u64) -> __u64 {
    let old = core::intrinsics::atomic_xadd_seqcst(ptr, val);
    old.wrapping_add(val)
}

#[link_section = "xdp.frags"]
pub unsafe extern "C" fn rx(ctx: *mut xdp_md) -> i32 {
    let mut data: *mut c_void;
    let mut data_meta: *mut c_void;
    let data_end: *mut c_void;
    let mut ip6h: *mut ipv6hdr = ptr::null_mut();
    let mut udp: *mut udphdr = ptr::null_mut();
    let mut iph: *mut iphdr = ptr::null_mut();
    let meta: *mut xdp_meta;
    let mut eth: *mut ethhdr;
    let mut err: i32;

    data = (*ctx).data as usize as *mut c_void;
    data_end = (*ctx).data_end as usize as *mut c_void;
    eth = data as *mut ethhdr;

    if eth.add(1) as *mut c_void as usize < data_end as usize
        && ((*eth).h_proto == bpf_htons(ETH_P_8021AD as __u16)
            || (*eth).h_proto == bpf_htons(ETH_P_8021Q as __u16))
    {
        eth = (eth as *mut u8).add(size_of::<vlan_hdr>()) as *mut ethhdr;
    }

    if eth.add(1) as *mut c_void as usize < data_end as usize
        && (*eth).h_proto == bpf_htons(ETH_P_8021Q as __u16)
    {
        eth = (eth as *mut u8).add(size_of::<vlan_hdr>()) as *mut ethhdr;
    }

    if eth.add(1) as *mut c_void as usize < data_end as usize {
        if (*eth).h_proto == bpf_htons(ETH_P_IP as __u16) {
            iph = eth.add(1) as *mut c_void as *mut iphdr;
            if iph.add(1) as *mut c_void as usize < data_end as usize
                && (*iph).protocol == IPPROTO_UDP as __u8
            {
                udp = iph.add(1) as *mut c_void as *mut udphdr;
            }
        }
        if (*eth).h_proto == bpf_htons(ETH_P_IPV6 as __u16) {
            ip6h = eth.add(1) as *mut c_void as *mut ipv6hdr;
            if ip6h.add(1) as *mut c_void as usize < data_end as usize
                && (*ip6h).nexthdr == IPPROTO_UDP as __u8
            {
                udp = ip6h.add(1) as *mut c_void as *mut udphdr;
            }
        }
        if !udp.is_null() && udp.add(1) as *mut c_void as usize > data_end as usize {
            udp = ptr::null_mut();
        }
    }

    if udp.is_null() {
        __sync_add_and_fetch_u64(ptr::addr_of_mut!(pkts_skip), 1);
        return XDP_PASS;
    }

    /* Forwarding UDP:9091 to AF_XDP */
    if (*udp).dest != bpf_htons(9091) {
        __sync_add_and_fetch_u64(ptr::addr_of_mut!(pkts_skip), 1);
        return XDP_PASS;
    }

    err = bpf_xdp_adjust_meta(ctx, -(size_of::<xdp_meta>() as i32));
    if err != 0 {
        __sync_add_and_fetch_u64(ptr::addr_of_mut!(pkts_fail), 1);
        return XDP_PASS;
    }

    data = (*ctx).data as usize as *mut c_void;
    data_meta = (*ctx).data_meta as usize as *mut c_void;
    meta = data_meta as *mut xdp_meta;

    if meta.add(1) as *mut c_void as usize > data as usize {
        __sync_add_and_fetch_u64(ptr::addr_of_mut!(pkts_fail), 1);
        return XDP_PASS;
    }

    (*meta).hint_valid = 0;

    (*meta).xdp_timestamp = bpf_ktime_get_tai_ns();
    err = bpf_xdp_metadata_rx_timestamp(ctx, ptr::addr_of_mut!((*meta).rx_timestamp));
    if err != 0 {
        (*meta).rx_timestamp_err = err;
    } else {
        (*meta).hint_valid |= XDP_META_FIELD_TS;
    }

    err = bpf_xdp_metadata_rx_hash(
        ctx,
        ptr::addr_of_mut!((*meta).rx_hash),
        ptr::addr_of_mut!((*meta).rx_hash_type),
    );
    if err != 0 {
        (*meta).rx_hash_err = err;
    } else {
        (*meta).hint_valid |= XDP_META_FIELD_RSS;
    }

    err = bpf_xdp_metadata_rx_vlan_tag(
        ctx,
        ptr::addr_of_mut!((*meta).rx_vlan_proto),
        ptr::addr_of_mut!((*meta).rx_vlan_tci),
    );
    if err != 0 {
        (*meta).rx_vlan_tag_err = err;
    } else {
        (*meta).hint_valid |= XDP_META_FIELD_VLAN_TAG;
    }

    __sync_add_and_fetch_u64(ptr::addr_of_mut!(pkts_redir), 1);
    bpf_redirect_map(ptr::addr_of_mut!(xsk), (*ctx).rx_queue_index, XDP_PASS as __u64)
}

#[link_section = "license"]
#[used]
pub static _license: [u8; 4] = *b"GPL\0";
