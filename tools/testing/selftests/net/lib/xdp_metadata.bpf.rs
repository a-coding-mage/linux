// SPDX-License-Identifier: GPL-2.0

// Translated from C. Header-provided BPF/kernel declarations are expected to be
// supplied by the surrounding build/bindings environment.

use core::ffi::c_void;
use core::ptr;
use core::sync::atomic::{AtomicU32, Ordering};

pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;
pub type __s32 = i32;

pub const BPF_MAP_TYPE_ARRAY: __u32 = 2;
pub const BPF_ANY: __u64 = 0;
pub const XDP_PASS: i32 = 2;
pub const ETH_P_IP: __u16 = 0x0800;
pub const ETH_P_IPV6: __u16 = 0x86DD;
pub const IPPROTO_TCP: __u8 = 6;
pub const IPPROTO_UDP: __u8 = 17;

pub type __u64 = u64;

#[repr(C)]
pub struct xdp_md {
    pub data: __u32,
    pub data_end: __u32,
}

#[repr(C)]
pub struct ethhdr {
    pub h_dest: [__u8; 6],
    pub h_source: [__u8; 6],
    pub h_proto: __u16,
}

#[repr(C)]
pub struct iphdr {
    pub ihl_version: __u8,
    pub tos: __u8,
    pub tot_len: __u16,
    pub id: __u16,
    pub frag_off: __u16,
    pub ttl: __u8,
    pub protocol: __u8,
    pub check: __u16,
    pub saddr: __u32,
    pub daddr: __u32,
}

#[repr(C)]
pub struct ipv6hdr {
    pub priority_version: __u8,
    pub flow_lbl: [__u8; 3],
    pub payload_len: __u16,
    pub nexthdr: __u8,
    pub hop_limit: __u8,
    pub saddr: [__u8; 16],
    pub daddr: [__u8; 16],
}

#[repr(C)]
pub struct udphdr {
    pub source: __u16,
    pub dest: __u16,
    pub len: __u16,
    pub check: __u16,
}

#[repr(C)]
pub struct tcphdr {
    pub source: __u16,
    pub dest: __u16,
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: __u32,
    pub max_entries: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
}

pub const XDP_PORT: __u32 = 1;
pub const XDP_PROTO: __u32 = 4;

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut map_xdp_setup: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 5,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__s32>() as __u32,
};

/* RSS hash results: key 0 = hash, key 1 = hash type,
 * key 2 = packet count, key 3 = error count.
 */
pub const RSS_KEY_HASH: __u32 = 0;
pub const RSS_KEY_TYPE: __u32 = 1;
pub const RSS_KEY_PKT_CNT: __u32 = 2;
pub const RSS_KEY_ERR_CNT: __u32 = 3;

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut map_rss: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 4,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u32>() as __u32,
};

/* Mirror of enum xdp_rss_hash_type from include/net/xdp.h.
 * Needed because the enum is not part of UAPI headers.
 */
pub type xdp_rss_hash_type = __u32;
pub const XDP_RSS_L3_IPV4: xdp_rss_hash_type = 1u32 << 0;
pub const XDP_RSS_L3_IPV6: xdp_rss_hash_type = 1u32 << 1;
pub const XDP_RSS_L3_DYNHDR: xdp_rss_hash_type = 1u32 << 2;
pub const XDP_RSS_L4: xdp_rss_hash_type = 1u32 << 3;
pub const XDP_RSS_L4_TCP: xdp_rss_hash_type = 1u32 << 4;
pub const XDP_RSS_L4_UDP: xdp_rss_hash_type = 1u32 << 5;
pub const XDP_RSS_L4_SCTP: xdp_rss_hash_type = 1u32 << 6;
pub const XDP_RSS_L4_IPSEC: xdp_rss_hash_type = 1u32 << 7;
pub const XDP_RSS_L4_ICMP: xdp_rss_hash_type = 1u32 << 8;

unsafe extern "C" {
    #[link_name = "bpf_xdp_metadata_rx_hash"]
    pub fn bpf_xdp_metadata_rx_hash(
        ctx: *const xdp_md,
        hash: *mut __u32,
        rss_type: *mut xdp_rss_hash_type,
    ) -> i32;

    pub fn bpf_map_lookup_elem(map: *mut c_void, key: *const c_void) -> *mut c_void;
    pub fn bpf_map_update_elem(
        map: *mut c_void,
        key: *const c_void,
        value: *const c_void,
        flags: __u64,
    ) -> i64;
}

#[inline(always)]
fn bpf_htons(x: __u16) -> __u16 {
    x.to_be()
}

#[inline(always)]
fn bpf_ntohs(x: __u16) -> __u16 {
    __u16::from_be(x)
}

#[inline(always)]
unsafe fn get_dest_port(l4: *mut c_void, data_end: *mut c_void, protocol: __u8) -> __u16 {
    if protocol == IPPROTO_UDP {
        let udp = l4 as *mut udphdr;

        if udp.add(1) as *mut c_void > data_end {
            return 0;
        }
        return (*udp).dest;
    } else if protocol == IPPROTO_TCP {
        let tcp = l4 as *mut tcphdr;

        if tcp.add(1) as *mut c_void > data_end {
            return 0;
        }
        return (*tcp).dest;
    }

    0
}

#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xdp_rss_hash(ctx: *mut xdp_md) -> i32 {
    let data_end = (*ctx).data_end as usize as *mut c_void;
    let data = (*ctx).data as usize as *mut c_void;
    let mut rss_type: xdp_rss_hash_type = 0;
    let eth = data as *mut ethhdr;
    let mut l4_proto: __u8 = 0;
    let mut hash: __u32 = 0;
    let mut key: __u32;
    let mut val: __u32;
    let mut l4: *mut c_void = ptr::null_mut();
    let mut cnt: *mut __u32;
    let ret: i32;

    if eth.add(1) as *mut c_void > data_end {
        return XDP_PASS;
    }

    if (*eth).h_proto == bpf_htons(ETH_P_IP) {
        let iph = eth.add(1) as *mut c_void as *mut iphdr;

        if iph.add(1) as *mut c_void > data_end {
            return XDP_PASS;
        }
        l4_proto = (*iph).protocol;
        l4 = iph.add(1) as *mut c_void;
    } else if (*eth).h_proto == bpf_htons(ETH_P_IPV6) {
        let ip6h = eth.add(1) as *mut c_void as *mut ipv6hdr;

        if ip6h.add(1) as *mut c_void > data_end {
            return XDP_PASS;
        }
        l4_proto = (*ip6h).nexthdr;
        l4 = ip6h.add(1) as *mut c_void;
    }

    if l4.is_null() {
        return XDP_PASS;
    }

    /* Filter on the configured protocol (map_xdp_setup key XDP_PROTO).
     * When set, only process packets matching the requested L4 protocol.
     */
    key = XDP_PROTO;
    let proto_cfg =
        bpf_map_lookup_elem(&raw mut map_xdp_setup as *mut c_void, &key as *const _ as *const c_void)
            as *mut __s32;

    if !proto_cfg.is_null() && *proto_cfg != 0 && l4_proto != *proto_cfg as __u8 {
        return XDP_PASS;
    }

    /* Filter on the configured port (map_xdp_setup key XDP_PORT).
     * Only applies to protocols with ports (UDP, TCP).
     */
    key = XDP_PORT;
    let port_cfg =
        bpf_map_lookup_elem(&raw mut map_xdp_setup as *mut c_void, &key as *const _ as *const c_void)
            as *mut __s32;

    if !port_cfg.is_null() && *port_cfg != 0 {
        let dest: __u16 = get_dest_port(l4, data_end, l4_proto);

        if dest == 0 || bpf_ntohs(dest) != *port_cfg as __u16 {
            return XDP_PASS;
        }
    }

    ret = bpf_xdp_metadata_rx_hash(ctx, &mut hash, &mut rss_type);
    if ret < 0 {
        key = RSS_KEY_ERR_CNT;
        cnt = bpf_map_lookup_elem(
            &raw mut map_rss as *mut c_void,
            &key as *const _ as *const c_void,
        ) as *mut __u32;
        if !cnt.is_null() {
            (*(cnt as *mut AtomicU32)).fetch_add(1, Ordering::SeqCst);
        }
        return XDP_PASS;
    }

    key = RSS_KEY_HASH;
    bpf_map_update_elem(
        &raw mut map_rss as *mut c_void,
        &key as *const _ as *const c_void,
        &hash as *const _ as *const c_void,
        BPF_ANY,
    );

    key = RSS_KEY_TYPE;
    val = rss_type as __u32;
    bpf_map_update_elem(
        &raw mut map_rss as *mut c_void,
        &key as *const _ as *const c_void,
        &val as *const _ as *const c_void,
        BPF_ANY,
    );

    key = RSS_KEY_PKT_CNT;
    cnt = bpf_map_lookup_elem(
        &raw mut map_rss as *mut c_void,
        &key as *const _ as *const c_void,
    ) as *mut __u32;
    if !cnt.is_null() {
        (*(cnt as *mut AtomicU32)).fetch_add(1, Ordering::SeqCst);
    }

    XDP_PASS
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
