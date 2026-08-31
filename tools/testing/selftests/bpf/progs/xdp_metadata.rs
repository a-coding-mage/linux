// SPDX-License-Identifier: GPL-2.0

// Dependencies from the original C source:
// <vmlinux.h>, "xdp_metadata.h", <bpf/bpf_helpers.h>, <bpf/bpf_endian.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

pub type __u16 = u16;
pub type __u32 = u32;
pub type __u64 = u64;
pub type __be16 = u16;
pub type u64 = __u64;

pub const BPF_MAP_TYPE_XSKMAP: __u32 = 17;
pub const BPF_MAP_TYPE_PROG_ARRAY: __u32 = 3;
pub const BPF_MAP_TYPE_DEVMAP: __u32 = 14;
pub const ETH_P_IP: __u16 = 0x0800;
pub const ETH_P_IPV6: __u16 = 0x86DD;
pub const IPPROTO_UDP: u8 = 17;
pub const XDP_DROP: i32 = 1;
pub const XDP_PASS: i32 = 2;

#[repr(C)]
pub struct xdp_md {
    pub data: __u32,
    pub data_end: __u32,
    pub data_meta: __u32,
    pub ingress_ifindex: __u32,
    pub rx_queue_index: __u32,
    pub egress_ifindex: __u32,
}

#[repr(C)]
pub struct ethhdr {
    pub h_dest: [u8; 6],
    pub h_source: [u8; 6],
    pub h_proto: __be16,
}

#[repr(C)]
pub struct iphdr {
    pub ihl_version: u8,
    pub tos: u8,
    pub tot_len: __be16,
    pub id: __be16,
    pub frag_off: __be16,
    pub ttl: u8,
    pub protocol: u8,
    pub check: __u16,
    pub saddr: __u32,
    pub daddr: __u32,
}

#[repr(C)]
pub struct ipv6hdr {
    pub priority_version: u8,
    pub flow_lbl: [u8; 3],
    pub payload_len: __be16,
    pub nexthdr: u8,
    pub hop_limit: u8,
    pub saddr: [u8; 16],
    pub daddr: [u8; 16],
}

#[repr(C)]
pub struct udphdr {
    pub source: __be16,
    pub dest: __be16,
    pub len: __be16,
    pub check: __u16,
}

#[repr(C)]
pub struct xdp_meta {
    pub rx_timestamp: __u64,
    pub rx_hash: __u32,
    pub rx_hash_type: xdp_rss_hash_type,
    pub rx_vlan_proto: __be16,
    pub rx_vlan_tci: __u16,
}

pub type xdp_rss_hash_type = __u32;

#[repr(C)]
pub struct bpf_devmap_val {
    pub ifindex: __u32,
    pub bpf_prog: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct bpf_map_def_xsk {
    pub type_: __u32,
    pub max_entries: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
}

#[repr(C)]
pub struct bpf_map_def_prog_arr {
    pub type_: __u32,
    pub max_entries: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
}

#[repr(C)]
pub struct bpf_map_def_dev_map {
    pub type_: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
    pub max_entries: __u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut xsk: bpf_map_def_xsk = bpf_map_def_xsk {
    type_: BPF_MAP_TYPE_XSKMAP,
    max_entries: 4,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u32>() as __u32,
};

#[link_section = ".maps"]
#[no_mangle]
pub static mut prog_arr: bpf_map_def_prog_arr = bpf_map_def_prog_arr {
    type_: BPF_MAP_TYPE_PROG_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u32>() as __u32,
};

#[link_section = ".maps"]
#[no_mangle]
pub static mut dev_map: bpf_map_def_dev_map = bpf_map_def_dev_map {
    type_: BPF_MAP_TYPE_DEVMAP,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<bpf_devmap_val>() as __u32,
    max_entries: 1,
};

extern "C" {
    #[link_name = "bpf_xdp_metadata_rx_timestamp"]
    pub fn bpf_xdp_metadata_rx_timestamp(
        ctx: *const xdp_md,
        timestamp: *mut __u64,
    ) -> i32;
    #[link_name = "bpf_xdp_metadata_rx_hash"]
    pub fn bpf_xdp_metadata_rx_hash(
        ctx: *const xdp_md,
        hash: *mut __u32,
        rss_type: *mut xdp_rss_hash_type,
    ) -> i32;
    #[link_name = "bpf_xdp_metadata_rx_vlan_tag"]
    pub fn bpf_xdp_metadata_rx_vlan_tag(
        ctx: *const xdp_md,
        vlan_proto: *mut __be16,
        vlan_tci: *mut __u16,
    ) -> i32;
    pub fn bpf_xdp_adjust_meta(ctx: *mut xdp_md, delta: i32) -> i32;
    pub fn bpf_redirect_map(
        map: *mut core::ffi::c_void,
        key: __u64,
        flags: __u64,
    ) -> i32;
}

#[inline]
pub const fn bpf_htons(x: __u16) -> __be16 {
    x.to_be()
}

#[link_section = "xdp"]
#[no_mangle]
pub unsafe extern "C" fn rx(ctx: *mut xdp_md) -> i32 {
    let mut data: *mut core::ffi::c_void;
    let mut data_meta: *mut core::ffi::c_void;
    let data_end: *mut core::ffi::c_void;
    let mut ip6h: *mut ipv6hdr = core::ptr::null_mut();
    let mut eth: *mut ethhdr = core::ptr::null_mut();
    let mut udp: *mut udphdr = core::ptr::null_mut();
    let mut iph: *mut iphdr = core::ptr::null_mut();
    let meta: *mut xdp_meta;
    let mut timestamp: u64 = -1i64 as u64;
    let mut ret: i32;

    data = (*ctx).data as usize as *mut core::ffi::c_void;
    data_end = (*ctx).data_end as usize as *mut core::ffi::c_void;
    eth = data as *mut ethhdr;
    if eth.add(1) < data_end as *mut ethhdr {
        if (*eth).h_proto == bpf_htons(ETH_P_IP) {
            iph = eth.add(1) as *mut core::ffi::c_void as *mut iphdr;
            if iph.add(1) < data_end as *mut iphdr && (*iph).protocol == IPPROTO_UDP {
                udp = iph.add(1) as *mut core::ffi::c_void as *mut udphdr;
            }
        }
        if (*eth).h_proto == bpf_htons(ETH_P_IPV6) {
            ip6h = eth.add(1) as *mut core::ffi::c_void as *mut ipv6hdr;
            if ip6h.add(1) < data_end as *mut ipv6hdr && (*ip6h).nexthdr == IPPROTO_UDP {
                udp = ip6h.add(1) as *mut core::ffi::c_void as *mut udphdr;
            }
        }
        if !udp.is_null() && udp.add(1) > data_end as *mut udphdr {
            udp = core::ptr::null_mut();
        }
    }

    if udp.is_null() {
        return XDP_PASS;
    }

    /* Forwarding UDP:8080 to AF_XDP */
    if (*udp).dest != bpf_htons(8080) {
        return XDP_PASS;
    }

    /* Reserve enough for all custom metadata. */

    ret = bpf_xdp_adjust_meta(ctx, -(core::mem::size_of::<xdp_meta>() as i32));
    if ret != 0 {
        return XDP_DROP;
    }

    data = (*ctx).data as usize as *mut core::ffi::c_void;
    data_meta = (*ctx).data_meta as usize as *mut core::ffi::c_void;

    if (data_meta as *mut u8).add(core::mem::size_of::<xdp_meta>()) > data as *mut u8 {
        return XDP_DROP;
    }

    meta = data_meta as *mut xdp_meta;

    /* Export metadata. */

    /* We expect veth bpf_xdp_metadata_rx_timestamp to return 0 HW
     * timestamp, so put some non-zero value into AF_XDP frame for
     * the userspace.
     */
    bpf_xdp_metadata_rx_timestamp(ctx, &mut timestamp);
    if timestamp == 0 {
        (*meta).rx_timestamp = 1;
    }

    bpf_xdp_metadata_rx_hash(ctx, &mut (*meta).rx_hash, &mut (*meta).rx_hash_type);
    bpf_xdp_metadata_rx_vlan_tag(
        ctx,
        &mut (*meta).rx_vlan_proto,
        &mut (*meta).rx_vlan_tci,
    );

    return bpf_redirect_map(
        &mut xsk as *mut bpf_map_def_xsk as *mut core::ffi::c_void,
        (*ctx).rx_queue_index as __u64,
        XDP_PASS as __u64,
    );
}

#[link_section = "xdp"]
#[no_mangle]
pub unsafe extern "C" fn redirect(ctx: *mut xdp_md) -> i32 {
    return bpf_redirect_map(
        &mut dev_map as *mut bpf_map_def_dev_map as *mut core::ffi::c_void,
        (*ctx).rx_queue_index as __u64,
        XDP_PASS as __u64,
    );
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";
