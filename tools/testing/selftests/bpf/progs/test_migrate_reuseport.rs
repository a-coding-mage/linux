// SPDX-License-Identifier: GPL-2.0
/*
 * Check if we can migrate child sockets.
 *
 *   1. If reuse_md->migrating_sk is NULL (SYN packet),
 *        return SK_PASS without selecting a listener.
 *   2. If reuse_md->migrating_sk is not NULL (socket migration),
 *        select a listener (reuseport_map[migrate_map[cookie]])
 *
 * Author: Kuniyuki Iwashima <kuniyu@amazon.co.jp>
 */

// Dependencies from the original C source:
// <stddef.h>, <string.h>, <linux/bpf.h>, <linux/if_ether.h>, <linux/ip.h>,
// <linux/ipv6.h>, <linux/tcp.h>, <linux/in.h>, <bpf/bpf_endian.h>,
// and <bpf/bpf_helpers.h>.

#[repr(C)]
pub struct xdp_md {
    pub data: u32,
    pub data_end: u32,
}

#[repr(C)]
pub struct sk_reuseport_md {
    pub data: *mut core::ffi::c_void,
    pub data_end: *mut core::ffi::c_void,
    pub len: u32,
    pub eth_protocol: u32,
    pub ip_protocol: u32,
    pub bind_inany: u32,
    pub hash: u32,
    pub sk: *mut bpf_sock,
    pub migrating_sk: *mut bpf_sock,
}

#[repr(C)]
pub struct bpf_sock {
    pub bound_dev_if: u32,
    pub family: u32,
    pub type_: u32,
    pub protocol: u32,
    pub mark: u32,
    pub priority: u32,
    pub src_ip4: u32,
    pub src_ip6: [u32; 4],
    pub src_port: u32,
    pub dst_port: u32,
    pub dst_ip4: u32,
    pub dst_ip6: [u32; 4],
    pub state: u32,
    pub rx_queue_mapping: i32,
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
    pub check: __sum16,
    pub saddr: __be32,
    pub daddr: __be32,
}

impl iphdr {
    #[inline(always)]
    unsafe fn ihl(&self) -> u8 {
        self.ihl_version & 0x0f
    }
}

#[repr(C)]
pub struct ipv6hdr {
    pub priority_version: u8,
    pub flow_lbl: [u8; 3],
    pub payload_len: __be16,
    pub nexthdr: u8,
    pub hop_limit: u8,
    pub saddr: in6_addr,
    pub daddr: in6_addr,
}

#[repr(C)]
pub struct in6_addr {
    pub in6_u: [u8; 16],
}

#[repr(C)]
pub struct tcphdr {
    pub source: __be16,
    pub dest: __be16,
    pub seq: __be32,
    pub ack_seq: __be32,
    pub doff_res_flags: __be16,
    pub window: __be16,
    pub check: __sum16,
    pub urg_ptr: __be16,
}

impl tcphdr {
    #[inline(always)]
    unsafe fn syn(&self) -> bool {
        (u16::from_be(self.doff_res_flags) & 0x0002) != 0
    }

    #[inline(always)]
    unsafe fn ack(&self) -> bool {
        (u16::from_be(self.doff_res_flags) & 0x0010) != 0
    }
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: u32,
    pub key_size: u32,
    pub value_size: u32,
    pub max_entries: u32,
    pub map_flags: u32,
}

pub type __u64 = u64;
pub type __be16 = u16;
pub type __be32 = u32;
pub type __sum16 = u16;

pub const BPF_MAP_TYPE_HASH: u32 = 1;
pub const BPF_MAP_TYPE_REUSEPORT_SOCKARRAY: u32 = 27;
pub const ETH_P_IP: u16 = 0x0800;
pub const ETH_P_IPV6: u16 = 0x86DD;
pub const IPPROTO_TCP: u8 = 6;
pub const XDP_DROP: i32 = 1;
pub const XDP_PASS: i32 = 2;
pub const SK_DROP: i32 = 0;
pub const SK_PASS: i32 = 1;
pub const BPF_TCP_ESTABLISHED: u32 = 1;
pub const BPF_TCP_SYN_RECV: u32 = 3;
pub const BPF_TCP_NEW_SYN_RECV: u32 = 12;

#[link_section = ".maps"]
#[no_mangle]
pub static mut reuseport_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_REUSEPORT_SOCKARRAY,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<__u64>() as u32,
    max_entries: 256,
    map_flags: 0,
};

#[link_section = ".maps"]
#[no_mangle]
pub static mut migrate_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_HASH,
    key_size: core::mem::size_of::<__u64>() as u32,
    value_size: core::mem::size_of::<i32>() as u32,
    max_entries: 256,
    map_flags: 0,
};

#[no_mangle]
pub static mut migrated_at_close: i32 = 0;
#[no_mangle]
pub static mut migrated_at_close_fastopen: i32 = 0;
#[no_mangle]
pub static mut migrated_at_send_synack: i32 = 0;
#[no_mangle]
pub static mut migrated_at_recv_ack: i32 = 0;
#[no_mangle]
pub static mut server_port: __be16 = 0;

extern "C" {
    fn bpf_map_lookup_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn bpf_get_socket_cookie(sk: *mut bpf_sock) -> __u64;
    fn bpf_sk_select_reuseport(
        reuse_md: *mut sk_reuseport_md,
        map: *mut core::ffi::c_void,
        key: *mut core::ffi::c_void,
        flags: u64,
    ) -> i64;
}

#[inline(always)]
fn bpf_ntohs(x: __be16) -> u16 {
    u16::from_be(x)
}

#[link_section = "xdp"]
#[no_mangle]
pub unsafe extern "C" fn drop_ack(xdp: *mut xdp_md) -> i32 {
    let data_end = (*xdp).data_end as usize as *mut core::ffi::c_void;
    let data = (*xdp).data as usize as *mut core::ffi::c_void;
    let eth = data as *mut ethhdr;
    let mut tcp: *mut tcphdr = core::ptr::null_mut();

    if eth.add(1) as *mut core::ffi::c_void > data_end {
        return XDP_PASS;
    }

    match bpf_ntohs((*eth).h_proto) {
        ETH_P_IP => {
            let ip = eth.add(1) as *mut iphdr;

            if ip.add(1) as *mut core::ffi::c_void > data_end {
                return XDP_PASS;
            }

            if (*ip).protocol != IPPROTO_TCP {
                return XDP_PASS;
            }

            tcp = (ip as *mut u8).add((*ip).ihl() as usize * 4) as *mut tcphdr;
        }
        ETH_P_IPV6 => {
            let ipv6 = eth.add(1) as *mut ipv6hdr;

            if ipv6.add(1) as *mut core::ffi::c_void > data_end {
                return XDP_PASS;
            }

            if (*ipv6).nexthdr != IPPROTO_TCP {
                return XDP_PASS;
            }

            tcp = ipv6.add(1) as *mut tcphdr;
        }
        _ => {
            return XDP_PASS;
        }
    }

    if tcp.add(1) as *mut core::ffi::c_void > data_end {
        return XDP_PASS;
    }

    if (*tcp).dest != server_port {
        return XDP_PASS;
    }

    if !(*tcp).syn() && (*tcp).ack() {
        return XDP_DROP;
    }

    XDP_PASS
}

#[link_section = "sk_reuseport/migrate"]
#[no_mangle]
pub unsafe extern "C" fn migrate_reuseport(reuse_md: *mut sk_reuseport_md) -> i32 {
    let flags: i32 = 0;
    let state: i32;
    let err: i32;
    let cookie: __u64;
    let key: *mut i32;

    if (*reuse_md).migrating_sk.is_null() {
        return SK_PASS;
    }

    state = (*(*reuse_md).migrating_sk).state as i32;
    cookie = bpf_get_socket_cookie((*reuse_md).sk);

    key = bpf_map_lookup_elem(
        &raw mut migrate_map as *mut _ as *mut core::ffi::c_void,
        &cookie as *const _ as *const core::ffi::c_void,
    ) as *mut i32;
    if key.is_null() {
        return SK_DROP;
    }

    err = bpf_sk_select_reuseport(
        reuse_md,
        &raw mut reuseport_map as *mut _ as *mut core::ffi::c_void,
        key as *mut core::ffi::c_void,
        flags as u64,
    ) as i32;
    if err != 0 {
        return SK_PASS;
    }

    match state as u32 {
        BPF_TCP_ESTABLISHED => {
            core::sync::atomic::AtomicI32::from_ptr(&raw mut migrated_at_close)
                .fetch_add(1, core::sync::atomic::Ordering::SeqCst);
        }
        BPF_TCP_SYN_RECV => {
            core::sync::atomic::AtomicI32::from_ptr(&raw mut migrated_at_close_fastopen)
                .fetch_add(1, core::sync::atomic::Ordering::SeqCst);
        }
        BPF_TCP_NEW_SYN_RECV => {
            if (*reuse_md).len == 0 {
                core::sync::atomic::AtomicI32::from_ptr(&raw mut migrated_at_send_synack)
                    .fetch_add(1, core::sync::atomic::Ordering::SeqCst);
            } else {
                core::sync::atomic::AtomicI32::from_ptr(&raw mut migrated_at_recv_ack)
                    .fetch_add(1, core::sync::atomic::Ordering::SeqCst);
            }
        }
        _ => {}
    }

    SK_PASS
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";
