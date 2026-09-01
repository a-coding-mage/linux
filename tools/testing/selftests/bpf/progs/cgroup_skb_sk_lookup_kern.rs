// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020 Facebook

// C dependencies: linux/bpf.h, bpf/bpf_endian.h, bpf/bpf_helpers.h,
// linux/if_ether.h, linux/in.h, linux/in6.h, linux/ipv6.h, linux/tcp.h,
// sys/types.h, sys/socket.h.

pub type __u16 = u16;
pub type __u32 = u32;
pub type __u64 = u64;

pub const ETH_P_IPV6: __u16 = 0x86DD;
pub const IPPROTO_TCP: u8 = 6;
pub const BPF_F_CURRENT_NETNS: __u64 = -1i32 as __u64;

#[repr(C)]
pub union in6_addr__bindgen_ty_1 {
    pub u6_addr8: [u8; 16],
    pub u6_addr16: [__u16; 8],
    pub u6_addr32: [__u32; 4],
}

#[repr(C)]
pub struct in6_addr {
    pub in6_u: in6_addr__bindgen_ty_1,
}

#[repr(C)]
pub struct ipv6hdr {
    pub priority_version: u8,
    pub flow_lbl: [u8; 3],
    pub payload_len: __u16,
    pub nexthdr: u8,
    pub hop_limit: u8,
    pub saddr: in6_addr,
    pub daddr: in6_addr,
}

#[repr(C)]
pub struct tcphdr {
    pub source: __u16,
    pub dest: __u16,
}

#[repr(C)]
pub struct bpf_sock_tuple_ipv6 {
    pub saddr: [__u32; 4],
    pub daddr: [__u32; 4],
    pub sport: __u16,
    pub dport: __u16,
}

#[repr(C)]
pub union bpf_sock_tuple {
    pub ipv6: bpf_sock_tuple_ipv6,
}

#[repr(C)]
pub struct __sk_buff {
    pub len: __u32,
    pub pkt_type: __u32,
    pub mark: __u32,
    pub queue_mapping: __u32,
    pub protocol: __u32,
}

#[repr(C)]
pub struct bpf_sock {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn bpf_skb_load_bytes(
        skb: *const __sk_buff,
        offset: __u32,
        to: *mut core::ffi::c_void,
        len: __u32,
    ) -> i32;
    pub fn bpf_sk_lookup_tcp(
        ctx: *mut __sk_buff,
        tuple: *mut bpf_sock_tuple,
        tuple_size: __u32,
        netns: __u64,
        flags: __u64,
    ) -> *mut bpf_sock;
    pub fn bpf_skb_cgroup_id(skb: *mut __sk_buff) -> __u64;
    pub fn bpf_sk_cgroup_id(sk: *mut bpf_sock) -> __u64;
    pub fn bpf_skb_ancestor_cgroup_id(skb: *mut __sk_buff, ancestor_level: i32) -> __u64;
    pub fn bpf_sk_ancestor_cgroup_id(sk: *mut bpf_sock, ancestor_level: i32) -> __u64;
    pub fn bpf_sk_release(sk: *mut bpf_sock);
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut g_serv_port: __u16 = 0;

#[inline(always)]
unsafe fn bpf_htons(x: __u16) -> __u16 {
    x.to_be()
}

#[inline(always)]
unsafe fn set_ip(dst: *mut __u32, src: *const in6_addr) {
    unsafe {
        *dst.add(0) = (*src).in6_u.u6_addr32[0];
        *dst.add(1) = (*src).in6_u.u6_addr32[1];
        *dst.add(2) = (*src).in6_u.u6_addr32[2];
        *dst.add(3) = (*src).in6_u.u6_addr32[3];
    }
}

#[inline(always)]
unsafe fn set_tuple(tuple: *mut bpf_sock_tuple, ip6h: *const ipv6hdr, tcph: *const tcphdr) {
    unsafe {
        set_ip((*tuple).ipv6.saddr.as_mut_ptr(), &(*ip6h).daddr);
        set_ip((*tuple).ipv6.daddr.as_mut_ptr(), &(*ip6h).saddr);
        (*tuple).ipv6.sport = (*tcph).dest;
        (*tuple).ipv6.dport = (*tcph).source;
    }
}

#[inline(always)]
unsafe fn is_allowed_peer_cg(
    skb: *mut __sk_buff,
    ip6h: *const ipv6hdr,
    tcph: *const tcphdr,
) -> i32 {
    let cgid: __u64;
    let acgid: __u64;
    let peer_cgid: __u64;
    let peer_acgid: __u64;
    let mut tuple: bpf_sock_tuple = unsafe {
        core::mem::zeroed()
    };
    let tuple_len = core::mem::size_of::<bpf_sock_tuple_ipv6>();
    let peer_sk: *mut bpf_sock;

    unsafe {
        set_tuple(&mut tuple, ip6h, tcph);

        peer_sk = bpf_sk_lookup_tcp(
            skb,
            &mut tuple,
            tuple_len as __u32,
            BPF_F_CURRENT_NETNS,
            0,
        );
        if peer_sk.is_null() {
            return 0;
        }

        cgid = bpf_skb_cgroup_id(skb);
        peer_cgid = bpf_sk_cgroup_id(peer_sk);

        acgid = bpf_skb_ancestor_cgroup_id(skb, 2);
        peer_acgid = bpf_sk_ancestor_cgroup_id(peer_sk, 2);

        bpf_sk_release(peer_sk);
    }

    (cgid != 0 && cgid == peer_cgid && acgid != 0 && acgid == peer_acgid) as i32
}

#[unsafe(link_section = "cgroup_skb/ingress")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ingress_lookup(skb: *mut __sk_buff) -> i32 {
    let mut ip6h: ipv6hdr = unsafe {
        core::mem::zeroed()
    };
    let mut tcph: tcphdr = unsafe {
        core::mem::zeroed()
    };

    unsafe {
        if (*skb).protocol != bpf_htons(ETH_P_IPV6) as __u32 {
            return 1;
        }

        /* For SYN packets coming to listening socket skb->remote_port will be
         * zero, so IPv6/TCP headers are loaded to identify remote peer
         * instead.
         */
        if bpf_skb_load_bytes(
            skb,
            0,
            &mut ip6h as *mut _ as *mut core::ffi::c_void,
            core::mem::size_of::<ipv6hdr>() as __u32,
        ) != 0 {
            return 1;
        }

        if ip6h.nexthdr != IPPROTO_TCP {
            return 1;
        }

        if bpf_skb_load_bytes(
            skb,
            core::mem::size_of::<ipv6hdr>() as __u32,
            &mut tcph as *mut _ as *mut core::ffi::c_void,
            core::mem::size_of::<tcphdr>() as __u32,
        ) != 0 {
            return 1;
        }

        if g_serv_port == 0 {
            return 0;
        }

        if tcph.dest != g_serv_port {
            return 1;
        }

        is_allowed_peer_cg(skb, &ip6h, &tcph)
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
