// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Cloudflare Ltd.
// Copyright (c) 2020 Isovalent, Inc.

use core::ffi::c_void;
use core::mem::size_of;
use core::ptr;

type __u8 = u8;
type __u16 = u16;
type __u32 = u32;
type __u64 = u64;
type __be16 = __u16;

const BPF_MAP_TYPE_SOCKMAP: __u32 = 15;
const LIBBPF_PIN_BY_NAME: __u32 = 1;
const PIN_GLOBAL_NS: __u32 = 2;
const BPF_F_CURRENT_NETNS: __u64 = -1i32 as __u64;
const BPF_TCP_LISTEN: __u32 = 10;
const ETH_P_IP: __u16 = 0x0800;
const ETH_P_IPV6: __u16 = 0x86DD;
const IPPROTO_TCP: __u8 = 6;
const IPPROTO_UDP: __u8 = 17;
const TC_ACT_OK: i32 = 0;
const TC_ACT_SHOT: i32 = 2;

#[repr(C)]
pub struct __sk_buff {
    pub data: __u32,
    pub data_end: __u32,
}

#[repr(C)]
pub struct bpf_sock {
    pub state: __u32,
}

#[repr(C)]
pub struct ethhdr {
    pub h_dest: [__u8; 6],
    pub h_source: [__u8; 6],
    pub h_proto: __be16,
}

#[repr(C)]
pub struct iphdr {
    pub ihl_version: __u8,
    pub tos: __u8,
    pub tot_len: __be16,
    pub id: __be16,
    pub frag_off: __be16,
    pub ttl: __u8,
    pub protocol: __u8,
    pub check: __be16,
    pub saddr: __u32,
    pub daddr: __u32,
}

impl iphdr {
    unsafe fn ihl(&self) -> __u8 {
        self.ihl_version & 0x0f
    }
}

#[repr(C)]
pub struct ipv6hdr {
    pub priority_version: __u8,
    pub flow_lbl: [__u8; 3],
    pub payload_len: __be16,
    pub nexthdr: __u8,
    pub hop_limit: __u8,
    pub saddr: [__u8; 16],
    pub daddr: [__u8; 16],
}

#[repr(C)]
pub struct bpf_sock_tuple_ipv4 {
    pub saddr: __u32,
    pub daddr: __u32,
    pub sport: __be16,
    pub dport: __be16,
}

#[repr(C)]
pub struct bpf_sock_tuple_ipv6 {
    pub saddr: [__u32; 4],
    pub daddr: [__u32; 4],
    pub sport: __be16,
    pub dport: __be16,
}

#[repr(C)]
pub union bpf_sock_tuple {
    pub ipv4: bpf_sock_tuple_ipv4,
    pub ipv6: bpf_sock_tuple_ipv6,
}

#[repr(C)]
pub struct bpf_elf_map {
    pub type_: __u32,
    pub size_key: __u32,
    pub size_value: __u32,
    pub max_elem: __u32,
    pub flags: __u32,
    pub id: __u32,
    pub pinning: __u32,
}

// Original C uses an IPROUTE2_HAVE_LIBBPF build-time branch:
// - with libbpf, server_map is a new-style BPF_MAP_TYPE_SOCKMAP map in ".maps"
// - otherwise, it is an iproute2 bpf_elf_map pinned under /sys/fs/bpf/tc/globals
#[no_mangle]
#[link_section = "maps"]
pub static mut server_map: bpf_elf_map = bpf_elf_map {
    type_: BPF_MAP_TYPE_SOCKMAP,
    size_key: size_of::<i32>() as __u32,
    size_value: size_of::<__u64>() as __u32,
    max_elem: 1,
    flags: 0,
    id: 0,
    pinning: PIN_GLOBAL_NS,
};

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

extern "C" {
    fn bpf_skc_lookup_tcp(
        skb: *mut __sk_buff,
        tuple: *mut bpf_sock_tuple,
        tuple_size: __u32,
        netns: __u64,
        flags: __u64,
    ) -> *mut bpf_sock;
    fn bpf_sk_lookup_udp(
        skb: *mut __sk_buff,
        tuple: *mut bpf_sock_tuple,
        tuple_size: __u32,
        netns: __u64,
        flags: __u64,
    ) -> *mut bpf_sock;
    fn bpf_map_lookup_elem(map: *mut c_void, key: *const c_void) -> *mut bpf_sock;
    fn bpf_sk_assign(skb: *mut __sk_buff, sk: *mut bpf_sock, flags: __u64) -> i32;
    fn bpf_sk_release(sk: *mut bpf_sock);
}

#[inline(always)]
fn bpf_htons(x: __u16) -> __be16 {
    x.to_be()
}

#[inline(always)]
unsafe fn __sink<T>(_x: T) {}

/* Fill 'tuple' with L3 info, and attempt to find L4. On fail, return NULL. */
#[inline(always)]
unsafe fn get_tuple(
    skb: *mut __sk_buff,
    ipv4: *mut bool,
    tcp: *mut bool,
) -> *mut bpf_sock_tuple {
    let data_end = (*skb).data_end as usize as *mut c_void;
    let data = (*skb).data as usize as *mut c_void;
    let result: *mut bpf_sock_tuple;
    let eth: *mut ethhdr;
    let mut proto: __u8 = 0;
    let ihl_len: __u64;

    eth = data as *mut ethhdr;
    if eth.add(1) as *mut c_void > data_end {
        return ptr::null_mut();
    }

    if (*eth).h_proto == bpf_htons(ETH_P_IP) {
        let iph = (data as *mut u8).add(size_of::<ethhdr>()) as *mut iphdr;

        if iph.add(1) as *mut c_void > data_end {
            return ptr::null_mut();
        }
        if (*iph).ihl() != 5 {
            /* Options are not supported */
            return ptr::null_mut();
        }
        ihl_len = ((*iph).ihl() as __u64) * 4;
        proto = (*iph).protocol;
        *ipv4 = true;
        result = &mut (*iph).saddr as *mut __u32 as *mut bpf_sock_tuple;
    } else if (*eth).h_proto == bpf_htons(ETH_P_IPV6) {
        let ip6h = (data as *mut u8).add(size_of::<ethhdr>()) as *mut ipv6hdr;

        if ip6h.add(1) as *mut c_void > data_end {
            return ptr::null_mut();
        }
        ihl_len = size_of::<ipv6hdr>() as __u64;
        proto = (*ip6h).nexthdr;
        *ipv4 = false;
        result = &mut (*ip6h).saddr as *mut [__u8; 16] as *mut bpf_sock_tuple;
    } else {
        return data as *mut bpf_sock_tuple;
    }

    if proto != IPPROTO_TCP && proto != IPPROTO_UDP {
        return ptr::null_mut();
    }

    *tcp = proto == IPPROTO_TCP;
    __sink(ihl_len);
    result
}

#[inline(always)]
unsafe fn handle_udp(skb: *mut __sk_buff, tuple: *mut bpf_sock_tuple, ipv4: bool) -> i32 {
    let mut sk: *mut bpf_sock;
    let zero: i32 = 0;
    let tuple_len: usize;
    let dport: __be16;
    let ret: i32;

    tuple_len = if ipv4 {
        size_of::<bpf_sock_tuple_ipv4>()
    } else {
        size_of::<bpf_sock_tuple_ipv6>()
    };
    if (tuple as *mut u8).add(tuple_len) as *mut c_void > (*skb).data_end as usize as *mut c_void {
        return TC_ACT_SHOT;
    }

    sk = bpf_sk_lookup_udp(
        skb,
        tuple,
        tuple_len as __u32,
        BPF_F_CURRENT_NETNS,
        0,
    );
    if !sk.is_null() {
        /* goto assign; */
    } else {
        dport = if ipv4 {
            (*tuple).ipv4.dport
        } else {
            (*tuple).ipv6.dport
        };
        if dport != bpf_htons(4321) {
            return TC_ACT_OK;
        }

        sk = bpf_map_lookup_elem(
            &mut server_map as *mut bpf_elf_map as *mut c_void,
            &zero as *const i32 as *const c_void,
        );
        if sk.is_null() {
            return TC_ACT_SHOT;
        }
    }

    ret = bpf_sk_assign(skb, sk, 0);
    bpf_sk_release(sk);
    ret
}

#[inline(always)]
unsafe fn handle_tcp(skb: *mut __sk_buff, tuple: *mut bpf_sock_tuple, ipv4: bool) -> i32 {
    let mut sk: *mut bpf_sock;
    let zero: i32 = 0;
    let tuple_len: usize;
    let dport: __be16;
    let ret: i32;

    tuple_len = if ipv4 {
        size_of::<bpf_sock_tuple_ipv4>()
    } else {
        size_of::<bpf_sock_tuple_ipv6>()
    };
    if (tuple as *mut u8).add(tuple_len) as *mut c_void > (*skb).data_end as usize as *mut c_void {
        return TC_ACT_SHOT;
    }

    sk = bpf_skc_lookup_tcp(
        skb,
        tuple,
        tuple_len as __u32,
        BPF_F_CURRENT_NETNS,
        0,
    );
    if !sk.is_null() {
        if (*sk).state != BPF_TCP_LISTEN {
            /* goto assign; */
            ret = bpf_sk_assign(skb, sk, 0);
            bpf_sk_release(sk);
            return ret;
        }
        bpf_sk_release(sk);
    }

    dport = if ipv4 {
        (*tuple).ipv4.dport
    } else {
        (*tuple).ipv6.dport
    };
    if dport != bpf_htons(4321) {
        return TC_ACT_OK;
    }

    sk = bpf_map_lookup_elem(
        &mut server_map as *mut bpf_elf_map as *mut c_void,
        &zero as *const i32 as *const c_void,
    );
    if sk.is_null() {
        return TC_ACT_SHOT;
    }

    if (*sk).state != BPF_TCP_LISTEN {
        bpf_sk_release(sk);
        return TC_ACT_SHOT;
    }

    ret = bpf_sk_assign(skb, sk, 0);
    bpf_sk_release(sk);
    ret
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn bpf_sk_assign_test(skb: *mut __sk_buff) -> i32 {
    let tuple: *mut bpf_sock_tuple;
    let mut ipv4: bool = false;
    let mut tcp: bool = false;
    let mut ret: i32 = 0;

    tuple = get_tuple(skb, &mut ipv4, &mut tcp);
    if tuple.is_null() {
        return TC_ACT_SHOT;
    }

    /* Note that the verifier socket return type for bpf_skc_lookup_tcp()
     * differs from bpf_sk_lookup_udp(), so even though the C-level type is
     * the same here, if we try to share the implementations they will
     * fail to verify because we're crossing pointer types.
     */
    if tcp {
        ret = handle_tcp(skb, tuple, ipv4);
    } else {
        ret = handle_udp(skb, tuple, ipv4);
    }

    if ret == 0 {
        TC_ACT_OK
    } else {
        TC_ACT_SHOT
    }
}
