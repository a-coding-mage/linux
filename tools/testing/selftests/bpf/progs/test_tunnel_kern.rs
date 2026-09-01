// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2016 VMware
 * Copyright (c) 2016 Facebook
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General Public
 * License as published by the Free Software Foundation.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

// C dependencies removed from executable Rust:
// BPF_NO_KFUNC_PROTOTYPES, vmlinux.h, bpf_core_read.h, bpf_helpers.h,
// bpf_endian.h, bpf_kfuncs.h, bpf_tracing_net.h.

const VXLAN_UDP_PORT: u16 = 4789;
const ETH_P_IP: u16 = 0x0800;
const PACKET_HOST: u32 = 0;
const TUNNEL_CSUM: u16 = bpf_htons_const(0x01);
const TUNNEL_KEY: u16 = bpf_htons_const(0x04);

/* Only IPv4 address assigned to veth1.
 * 172.16.1.200
 */
const ASSIGNED_ADDR_VETH1: u32 = 0xac1001c8;

#[repr(C)]
pub struct bpf_fou_encap___local {
    pub sport: __be16,
    pub dport: __be16,
}

#[repr(C)]
pub enum bpf_fou_encap_type___local {
    FOU_BPF_ENCAP_FOU___local,
    FOU_BPF_ENCAP_GUE___local,
}

unsafe extern "C" {
    fn bpf_skb_set_fou_encap(
        skb_ctx: *mut __sk_buff,
        encap: *mut bpf_fou_encap___local,
        type_: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn bpf_skb_get_fou_encap(
        skb_ctx: *mut __sk_buff,
        encap: *mut bpf_fou_encap___local,
    ) -> ::core::ffi::c_int;
    fn bpf_xdp_get_xfrm_state(
        ctx: *mut xdp_md,
        opts: *mut bpf_xfrm_state_opts,
        opts__sz: u32,
    ) -> *mut xfrm_state;
    fn bpf_xdp_xfrm_state_release(x: *mut xfrm_state);
}

#[repr(C)]
pub struct local_ip_map_def {
    // Original C map definition:
    // __uint(type, BPF_MAP_TYPE_ARRAY);
    // __uint(max_entries, 1);
    // __type(key, __u32);
    // __type(value, __u32);
    _private: [u8; 0],
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut local_ip_map: local_ip_map_def = local_ip_map_def { _private: [] };

#[inline(always)]
const fn bpf_htons_const(x: u16) -> u16 {
    x.to_be()
}

#[inline(always)]
unsafe fn log_err(ret: ::core::ffi::c_int, line: u32) {
    bpf_printk(c"ERROR line:%d ret:%d\n".as_ptr(), line, ret);
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn gre_set_tunnel(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int;
    let mut key: bpf_tunnel_key = ::core::mem::zeroed();

    key.remote_ipv4 = 0xac100164; /* 172.16.1.100 */
    key.tunnel_id = 2;
    key.tunnel_tos = 0;
    key.tunnel_ttl = 64;

    ret = bpf_skb_set_tunnel_key(
        skb,
        &mut key,
        ::core::mem::size_of_val(&key) as u32,
        BPF_F_ZERO_CSUM_TX | BPF_F_SEQ_NUMBER,
    );
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    TC_ACT_OK
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn gre_set_tunnel_no_key(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int;
    let mut key: bpf_tunnel_key = ::core::mem::zeroed();

    key.remote_ipv4 = 0xac100164; /* 172.16.1.100 */
    key.tunnel_ttl = 64;

    ret = bpf_skb_set_tunnel_key(
        skb,
        &mut key,
        ::core::mem::size_of_val(&key) as u32,
        BPF_F_ZERO_CSUM_TX | BPF_F_SEQ_NUMBER | BPF_F_NO_TUNNEL_KEY,
    );
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    TC_ACT_OK
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn gre_get_tunnel(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int;
    let mut key: bpf_tunnel_key = ::core::mem::zeroed();

    ret = bpf_skb_get_tunnel_key(skb, &mut key, ::core::mem::size_of_val(&key) as u32, 0);
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    bpf_printk(c"key %d remote ip 0x%x\n".as_ptr(), key.tunnel_id, key.remote_ipv4);
    TC_ACT_OK
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn ip6gretap_set_tunnel(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let mut key: bpf_tunnel_key = ::core::mem::zeroed();
    let mut ret: ::core::ffi::c_int;

    key.remote_ipv6[3] = bpf_htonl(0x11); /* ::11 */
    key.tunnel_id = 2;
    key.tunnel_tos = 0;
    key.tunnel_ttl = 64;
    key.tunnel_label = 0xabcde;

    ret = bpf_skb_set_tunnel_key(
        skb,
        &mut key,
        ::core::mem::size_of_val(&key) as u32,
        BPF_F_TUNINFO_IPV6 | BPF_F_ZERO_CSUM_TX | BPF_F_SEQ_NUMBER,
    );
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    TC_ACT_OK
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn ip6gretap_get_tunnel(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let mut key: bpf_tunnel_key = ::core::mem::zeroed();
    let mut ret: ::core::ffi::c_int;

    ret = bpf_skb_get_tunnel_key(
        skb,
        &mut key,
        ::core::mem::size_of_val(&key) as u32,
        BPF_F_TUNINFO_IPV6,
    );
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    bpf_printk(
        c"key %d remote ip6 ::%x label %x\n".as_ptr(),
        key.tunnel_id,
        key.remote_ipv6[3],
        key.tunnel_label,
    );

    TC_ACT_OK
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn erspan_set_tunnel(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let mut key: bpf_tunnel_key = ::core::mem::zeroed();
    let mut md: erspan_metadata = ::core::mem::zeroed();
    let mut ret: ::core::ffi::c_int;

    key.remote_ipv4 = 0xac100164; /* 172.16.1.100 */
    key.tunnel_id = 2;
    key.tunnel_tos = 0;
    key.tunnel_ttl = 64;

    ret = bpf_skb_set_tunnel_key(
        skb,
        &mut key,
        ::core::mem::size_of_val(&key) as u32,
        BPF_F_ZERO_CSUM_TX,
    );
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    // Original C conditional:
    // #ifdef ERSPAN_V1
    // md.version = 1;
    // md.u.index = bpf_htonl(123);
    // #else
    let direction: __u8 = 1;
    let hwid: __u8 = 7;

    md.version = 2;
    BPF_CORE_WRITE_BITFIELD!(&mut md.u.md2, dir, direction);
    BPF_CORE_WRITE_BITFIELD!(&mut md.u.md2, hwid, hwid & 0xf);
    BPF_CORE_WRITE_BITFIELD!(&mut md.u.md2, hwid_upper, (hwid >> 4) & 0x3);
    // #endif

    ret = bpf_skb_set_tunnel_opt(skb, &mut md, ::core::mem::size_of_val(&md) as u32);
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    TC_ACT_OK
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn erspan_get_tunnel(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let mut key: bpf_tunnel_key = ::core::mem::zeroed();
    let mut md: erspan_metadata = ::core::mem::zeroed();
    let mut ret: ::core::ffi::c_int;

    ret = bpf_skb_get_tunnel_key(skb, &mut key, ::core::mem::size_of_val(&key) as u32, 0);
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    ret = bpf_skb_get_tunnel_opt(skb, &mut md, ::core::mem::size_of_val(&md) as u32);
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    bpf_printk(
        c"key %d remote ip 0x%x erspan version %d\n".as_ptr(),
        key.tunnel_id,
        key.remote_ipv4,
        md.version,
    );

    // Original C had an ERSPAN_V1 branch reading md.u.index.
    bpf_printk(
        c"\tdirection %d hwid %x timestamp %u\n".as_ptr(),
        BPF_CORE_READ_BITFIELD!(&md.u.md2, dir),
        (BPF_CORE_READ_BITFIELD!(&md.u.md2, hwid_upper) << 4)
            + BPF_CORE_READ_BITFIELD!(&md.u.md2, hwid),
        bpf_ntohl(md.u.md2.timestamp),
    );

    TC_ACT_OK
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn ip4ip6erspan_set_tunnel(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let mut key: bpf_tunnel_key = ::core::mem::zeroed();
    let mut md: erspan_metadata = ::core::mem::zeroed();
    let mut ret: ::core::ffi::c_int;

    key.remote_ipv6[3] = bpf_htonl(0x11);
    key.tunnel_id = 2;
    key.tunnel_tos = 0;
    key.tunnel_ttl = 64;

    ret = bpf_skb_set_tunnel_key(
        skb,
        &mut key,
        ::core::mem::size_of_val(&key) as u32,
        BPF_F_TUNINFO_IPV6,
    );
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    // Original C conditional:
    // #ifdef ERSPAN_V1
    // md.u.index = bpf_htonl(123);
    // md.version = 1;
    // #else
    let direction: __u8 = 0;
    let hwid: __u8 = 17;

    md.version = 2;
    BPF_CORE_WRITE_BITFIELD!(&mut md.u.md2, dir, direction);
    BPF_CORE_WRITE_BITFIELD!(&mut md.u.md2, hwid, hwid & 0xf);
    BPF_CORE_WRITE_BITFIELD!(&mut md.u.md2, hwid_upper, (hwid >> 4) & 0x3);
    // #endif

    ret = bpf_skb_set_tunnel_opt(skb, &mut md, ::core::mem::size_of_val(&md) as u32);
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    TC_ACT_OK
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn ip4ip6erspan_get_tunnel(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let mut key: bpf_tunnel_key = ::core::mem::zeroed();
    let mut md: erspan_metadata = ::core::mem::zeroed();
    let mut ret: ::core::ffi::c_int;

    ret = bpf_skb_get_tunnel_key(
        skb,
        &mut key,
        ::core::mem::size_of_val(&key) as u32,
        BPF_F_TUNINFO_IPV6,
    );
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    ret = bpf_skb_get_tunnel_opt(skb, &mut md, ::core::mem::size_of_val(&md) as u32);
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    bpf_printk(
        c"ip6erspan get key %d remote ip6 ::%x erspan version %d\n".as_ptr(),
        key.tunnel_id,
        key.remote_ipv4,
        md.version,
    );

    // Original C had an ERSPAN_V1 branch reading md.u.index.
    bpf_printk(
        c"\tdirection %d hwid %x timestamp %u\n".as_ptr(),
        BPF_CORE_READ_BITFIELD!(&md.u.md2, dir),
        (BPF_CORE_READ_BITFIELD!(&md.u.md2, hwid_upper) << 4)
            + BPF_CORE_READ_BITFIELD!(&md.u.md2, hwid),
        bpf_ntohl(md.u.md2.timestamp),
    );

    TC_ACT_OK
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn vxlan_set_tunnel_dst(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let mut key: bpf_tunnel_key = ::core::mem::zeroed();
    let mut md: vxlan_metadata = ::core::mem::zeroed();
    let mut index: __u32 = 0;
    let mut ret: ::core::ffi::c_int = 0;

    let local_ip = bpf_map_lookup_elem(&mut local_ip_map as *mut _ as *mut _, &mut index as *mut _ as *mut _)
        as *mut __u32;
    if local_ip.is_null() {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    key.local_ipv4 = 0xac100164; /* 172.16.1.100 */
    key.remote_ipv4 = *local_ip;
    key.tunnel_id = 2;
    key.tunnel_tos = 0;
    key.tunnel_ttl = 64;

    ret = bpf_skb_set_tunnel_key(
        skb,
        &mut key,
        ::core::mem::size_of_val(&key) as u32,
        BPF_F_ZERO_CSUM_TX,
    );
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    md.gbp = 0x800FF; /* Set VXLAN Group Policy extension */
    ret = bpf_skb_set_tunnel_opt(skb, &mut md, ::core::mem::size_of_val(&md) as u32);
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    TC_ACT_OK
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn vxlan_set_tunnel_src(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let mut key: bpf_tunnel_key = ::core::mem::zeroed();
    let mut md: vxlan_metadata = ::core::mem::zeroed();
    let mut index: __u32 = 0;
    let mut ret: ::core::ffi::c_int = 0;

    let local_ip = bpf_map_lookup_elem(&mut local_ip_map as *mut _ as *mut _, &mut index as *mut _ as *mut _)
        as *mut __u32;
    if local_ip.is_null() {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    key.local_ipv4 = *local_ip;
    key.remote_ipv4 = 0xac100164; /* 172.16.1.100 */
    key.tunnel_id = 2;
    key.tunnel_tos = 0;
    key.tunnel_ttl = 64;

    ret = bpf_skb_set_tunnel_key(
        skb,
        &mut key,
        ::core::mem::size_of_val(&key) as u32,
        BPF_F_ZERO_CSUM_TX,
    );
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    md.gbp = 0x800FF; /* Set VXLAN Group Policy extension */
    ret = bpf_skb_set_tunnel_opt(skb, &mut md, ::core::mem::size_of_val(&md) as u32);
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    TC_ACT_OK
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn vxlan_get_tunnel_src(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int;
    let mut key: bpf_tunnel_key = ::core::mem::zeroed();
    let mut md: vxlan_metadata = ::core::mem::zeroed();

    ret = bpf_skb_get_tunnel_key(
        skb,
        &mut key,
        ::core::mem::size_of_val(&key) as u32,
        BPF_F_TUNINFO_FLAGS,
    );
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    ret = bpf_skb_get_tunnel_opt(skb, &mut md, ::core::mem::size_of_val(&md) as u32);
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    if key.local_ipv4 != ASSIGNED_ADDR_VETH1
        || md.gbp != 0x800FF
        || (key.tunnel_flags & TUNNEL_KEY) == 0
        || (key.tunnel_flags & TUNNEL_CSUM) != 0
    {
        bpf_printk(
            c"vxlan key %d local ip 0x%x remote ip 0x%x gbp 0x%x flags 0x%x\n".as_ptr(),
            key.tunnel_id,
            key.local_ipv4,
            key.remote_ipv4,
            md.gbp,
            bpf_ntohs(key.tunnel_flags),
        );
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    TC_ACT_OK
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn veth_set_outer_dst(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let eth = (*skb).data as usize as *mut ethhdr;
    let mut assigned_ip: __u32 = bpf_htonl(ASSIGNED_ADDR_VETH1);
    let data_end = (*skb).data_end as usize as *mut ::core::ffi::c_void;
    let mut ret: ::core::ffi::c_int = 0;
    let csum: __s64;

    if (eth as *mut u8).add(::core::mem::size_of::<ethhdr>()) > data_end as *mut u8 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    if (*eth).h_proto != bpf_htons(ETH_P_IP) {
        return TC_ACT_OK;
    }

    let iph = eth.add(1) as *mut iphdr;
    if (iph as *mut u8).add(::core::mem::size_of::<iphdr>()) > data_end as *mut u8 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }
    if (*iph).protocol != IPPROTO_UDP {
        return TC_ACT_OK;
    }

    let udph = iph.add(1) as *mut udphdr;
    if (udph as *mut u8).add(::core::mem::size_of::<udphdr>()) > data_end as *mut u8 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }
    if (*udph).dest != bpf_htons(VXLAN_UDP_PORT) {
        return TC_ACT_OK;
    }

    if (*iph).daddr != assigned_ip {
        csum = bpf_csum_diff(
            &mut (*iph).daddr,
            ::core::mem::size_of::<__u32>() as u32,
            &mut assigned_ip,
            ::core::mem::size_of::<__u32>() as u32,
            0,
        );
        if bpf_skb_store_bytes(
            skb,
            ETH_HLEN + offset_of!(iphdr, daddr) as u32,
            &mut assigned_ip as *mut _ as *mut _,
            ::core::mem::size_of::<__u32>() as u32,
            0,
        ) < 0
        {
            log_err(ret, line!());
            return TC_ACT_SHOT;
        }
        if bpf_l3_csum_replace(
            skb,
            ETH_HLEN + offset_of!(iphdr, check) as u32,
            0,
            csum,
            0,
        ) < 0
        {
            log_err(ret, line!());
            return TC_ACT_SHOT;
        }
        bpf_skb_change_type(skb, PACKET_HOST);
    }
    TC_ACT_OK
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn ip6vxlan_set_tunnel_dst(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let mut key: bpf_tunnel_key = ::core::mem::zeroed();
    let mut index: __u32 = 0;
    let mut ret: ::core::ffi::c_int = 0;

    let local_ip = bpf_map_lookup_elem(&mut local_ip_map as *mut _ as *mut _, &mut index as *mut _ as *mut _)
        as *mut __u32;
    if local_ip.is_null() {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    key.local_ipv6[3] = bpf_htonl(0x11); /* ::11 */
    key.remote_ipv6[3] = bpf_htonl(*local_ip);
    key.tunnel_id = 22;
    key.tunnel_tos = 0;
    key.tunnel_ttl = 64;

    ret = bpf_skb_set_tunnel_key(
        skb,
        &mut key,
        ::core::mem::size_of_val(&key) as u32,
        BPF_F_TUNINFO_IPV6,
    );
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    TC_ACT_OK
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn ip6vxlan_set_tunnel_src(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let mut key: bpf_tunnel_key = ::core::mem::zeroed();
    let mut index: __u32 = 0;
    let mut ret: ::core::ffi::c_int = 0;

    let local_ip = bpf_map_lookup_elem(&mut local_ip_map as *mut _ as *mut _, &mut index as *mut _ as *mut _)
        as *mut __u32;
    if local_ip.is_null() {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    key.local_ipv6[3] = bpf_htonl(*local_ip);
    key.remote_ipv6[3] = bpf_htonl(0x11); /* ::11 */
    key.tunnel_id = 22;
    key.tunnel_tos = 0;
    key.tunnel_ttl = 64;

    ret = bpf_skb_set_tunnel_key(
        skb,
        &mut key,
        ::core::mem::size_of_val(&key) as u32,
        BPF_F_TUNINFO_IPV6,
    );
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    TC_ACT_OK
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn ip6vxlan_get_tunnel_src(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let mut key: bpf_tunnel_key = ::core::mem::zeroed();
    let mut index: __u32 = 0;
    let mut ret: ::core::ffi::c_int = 0;

    let local_ip = bpf_map_lookup_elem(&mut local_ip_map as *mut _ as *mut _, &mut index as *mut _ as *mut _)
        as *mut __u32;
    if local_ip.is_null() {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    ret = bpf_skb_get_tunnel_key(
        skb,
        &mut key,
        ::core::mem::size_of_val(&key) as u32,
        BPF_F_TUNINFO_IPV6 | BPF_F_TUNINFO_FLAGS,
    );
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    if bpf_ntohl(key.local_ipv6[3]) != *local_ip
        || (key.tunnel_flags & TUNNEL_KEY) == 0
        || (key.tunnel_flags & TUNNEL_CSUM) == 0
    {
        bpf_printk(
            c"ip6vxlan key %d local ip6 ::%x remote ip6 ::%x label 0x%x flags 0x%x\n".as_ptr(),
            key.tunnel_id,
            bpf_ntohl(key.local_ipv6[3]),
            bpf_ntohl(key.remote_ipv6[3]),
            key.tunnel_label,
            bpf_ntohs(key.tunnel_flags),
        );
        bpf_printk(c"local_ip 0x%x\n".as_ptr(), *local_ip);
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    TC_ACT_OK
}

#[repr(C)]
pub struct local_geneve_opt {
    pub gopt: geneve_opt,
    pub data: ::core::ffi::c_int,
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn geneve_set_tunnel(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int;
    let mut key: bpf_tunnel_key = ::core::mem::zeroed();
    let mut local_gopt: local_geneve_opt = ::core::mem::zeroed();
    let gopt = &mut local_gopt as *mut local_geneve_opt as *mut geneve_opt;

    key.remote_ipv4 = 0xac100164; /* 172.16.1.100 */
    key.tunnel_id = 2;
    key.tunnel_tos = 0;
    key.tunnel_ttl = 64;

    ::core::ptr::write_bytes(gopt as *mut u8, 0, ::core::mem::size_of_val(&local_gopt));
    (*gopt).opt_class = bpf_htons(0x102); /* Open Virtual Networking (OVN) */
    (*gopt).type_ = 0x08;
    (*gopt).r1 = 0;
    (*gopt).r2 = 0;
    (*gopt).r3 = 0;
    (*gopt).length = 2; /* 4-byte multiple */
    *(&mut (*gopt).opt_data as *mut _ as *mut ::core::ffi::c_int) = bpf_htonl(0xdeadbeef) as _;

    ret = bpf_skb_set_tunnel_key(
        skb,
        &mut key,
        ::core::mem::size_of_val(&key) as u32,
        BPF_F_ZERO_CSUM_TX,
    );
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    ret = bpf_skb_set_tunnel_opt(skb, gopt as *mut _, ::core::mem::size_of_val(&local_gopt) as u32);
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    TC_ACT_OK
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn geneve_get_tunnel(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int;
    let mut key: bpf_tunnel_key = ::core::mem::zeroed();
    let mut gopt: geneve_opt = ::core::mem::zeroed();

    ret = bpf_skb_get_tunnel_key(skb, &mut key, ::core::mem::size_of_val(&key) as u32, 0);
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    ret = bpf_skb_get_tunnel_opt(skb, &mut gopt, ::core::mem::size_of_val(&gopt) as u32);
    if ret < 0 {
        gopt.opt_class = 0;
    }

    bpf_printk(
        c"key %d remote ip 0x%x geneve class 0x%x\n".as_ptr(),
        key.tunnel_id,
        key.remote_ipv4,
        gopt.opt_class,
    );
    TC_ACT_OK
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn ip6geneve_set_tunnel(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let mut key: bpf_tunnel_key = ::core::mem::zeroed();
    let mut local_gopt: local_geneve_opt = ::core::mem::zeroed();
    let gopt = &mut local_gopt as *mut local_geneve_opt as *mut geneve_opt;
    let mut ret: ::core::ffi::c_int;

    key.remote_ipv6[3] = bpf_htonl(0x11); /* ::11 */
    key.tunnel_id = 22;
    key.tunnel_tos = 0;
    key.tunnel_ttl = 64;

    ret = bpf_skb_set_tunnel_key(
        skb,
        &mut key,
        ::core::mem::size_of_val(&key) as u32,
        BPF_F_TUNINFO_IPV6,
    );
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    ::core::ptr::write_bytes(gopt as *mut u8, 0, ::core::mem::size_of_val(&local_gopt));
    (*gopt).opt_class = bpf_htons(0x102); /* Open Virtual Networking (OVN) */
    (*gopt).type_ = 0x08;
    (*gopt).r1 = 0;
    (*gopt).r2 = 0;
    (*gopt).r3 = 0;
    (*gopt).length = 2; /* 4-byte multiple */
    *(&mut (*gopt).opt_data as *mut _ as *mut ::core::ffi::c_int) = bpf_htonl(0xfeedbeef) as _;

    ret = bpf_skb_set_tunnel_opt(skb, gopt as *mut _, ::core::mem::size_of_val(&gopt) as u32);
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    TC_ACT_OK
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn ip6geneve_get_tunnel(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let mut key: bpf_tunnel_key = ::core::mem::zeroed();
    let mut gopt: geneve_opt = ::core::mem::zeroed();
    let mut ret: ::core::ffi::c_int;

    ret = bpf_skb_get_tunnel_key(
        skb,
        &mut key,
        ::core::mem::size_of_val(&key) as u32,
        BPF_F_TUNINFO_IPV6,
    );
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    ret = bpf_skb_get_tunnel_opt(skb, &mut gopt, ::core::mem::size_of_val(&gopt) as u32);
    if ret < 0 {
        gopt.opt_class = 0;
    }

    bpf_printk(
        c"key %d remote ip 0x%x geneve class 0x%x\n".as_ptr(),
        key.tunnel_id,
        key.remote_ipv4,
        gopt.opt_class,
    );

    TC_ACT_OK
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn ipip_set_tunnel(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let mut key: bpf_tunnel_key = ::core::mem::zeroed();
    let data = (*skb).data as usize as *mut ::core::ffi::c_void;
    let iph = data as *mut iphdr;
    let data_end = (*skb).data_end as usize as *mut ::core::ffi::c_void;
    let mut ret: ::core::ffi::c_int;

    /* single length check */
    if (data as *mut u8).add(::core::mem::size_of::<iphdr>()) > data_end as *mut u8 {
        log_err(1, line!());
        return TC_ACT_SHOT;
    }

    key.tunnel_ttl = 64;
    if (*iph).protocol == IPPROTO_ICMP {
        key.remote_ipv4 = 0xac100164; /* 172.16.1.100 */
    }

    ret = bpf_skb_set_tunnel_key(skb, &mut key, ::core::mem::size_of_val(&key) as u32, 0);
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    TC_ACT_OK
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn ipip_get_tunnel(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int;
    let mut key: bpf_tunnel_key = ::core::mem::zeroed();

    ret = bpf_skb_get_tunnel_key(skb, &mut key, ::core::mem::size_of_val(&key) as u32, 0);
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    bpf_printk(c"remote ip 0x%x\n".as_ptr(), key.remote_ipv4);
    TC_ACT_OK
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn ipip_gue_set_tunnel(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let mut key: bpf_tunnel_key = ::core::mem::zeroed();
    let mut encap: bpf_fou_encap___local = ::core::mem::zeroed();
    let data = (*skb).data as usize as *mut ::core::ffi::c_void;
    let iph = data as *mut iphdr;
    let data_end = (*skb).data_end as usize as *mut ::core::ffi::c_void;
    let mut ret: ::core::ffi::c_int;

    if (data as *mut u8).add(::core::mem::size_of::<iphdr>()) > data_end as *mut u8 {
        log_err(1, line!());
        return TC_ACT_SHOT;
    }

    key.tunnel_ttl = 64;
    if (*iph).protocol == IPPROTO_ICMP {
        key.remote_ipv4 = 0xac100164; /* 172.16.1.100 */
    }

    ret = bpf_skb_set_tunnel_key(skb, &mut key, ::core::mem::size_of_val(&key) as u32, 0);
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    encap.sport = 0;
    encap.dport = bpf_htons(5555);

    ret = bpf_skb_set_fou_encap(
        skb,
        &mut encap,
        bpf_fou_encap_type___local::FOU_BPF_ENCAP_GUE___local as ::core::ffi::c_int,
    );
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    TC_ACT_OK
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn ipip_fou_set_tunnel(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let mut key: bpf_tunnel_key = ::core::mem::zeroed();
    let mut encap: bpf_fou_encap___local = ::core::mem::zeroed();
    let data = (*skb).data as usize as *mut ::core::ffi::c_void;
    let iph = data as *mut iphdr;
    let data_end = (*skb).data_end as usize as *mut ::core::ffi::c_void;
    let mut ret: ::core::ffi::c_int;

    if (data as *mut u8).add(::core::mem::size_of::<iphdr>()) > data_end as *mut u8 {
        log_err(1, line!());
        return TC_ACT_SHOT;
    }

    key.tunnel_ttl = 64;
    if (*iph).protocol == IPPROTO_ICMP {
        key.remote_ipv4 = 0xac100164; /* 172.16.1.100 */
    }

    ret = bpf_skb_set_tunnel_key(skb, &mut key, ::core::mem::size_of_val(&key) as u32, 0);
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    encap.sport = 0;
    encap.dport = bpf_htons(5555);

    ret = bpf_skb_set_fou_encap(
        skb,
        &mut encap,
        bpf_fou_encap_type___local::FOU_BPF_ENCAP_FOU___local as ::core::ffi::c_int,
    );
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    TC_ACT_OK
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn ipip_encap_get_tunnel(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int;
    let mut key: bpf_tunnel_key = ::core::mem::zeroed();
    let mut encap: bpf_fou_encap___local = ::core::mem::zeroed();

    ret = bpf_skb_get_tunnel_key(skb, &mut key, ::core::mem::size_of_val(&key) as u32, 0);
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    ret = bpf_skb_get_fou_encap(skb, &mut encap);
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    if bpf_ntohs(encap.dport) != 5555 {
        return TC_ACT_SHOT;
    }

    bpf_printk(
        c"%d remote ip 0x%x, sport %d, dport %d\n".as_ptr(),
        ret,
        key.remote_ipv4,
        bpf_ntohs(encap.sport),
        bpf_ntohs(encap.dport),
    );
    TC_ACT_OK
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn ipip6_set_tunnel(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let mut key: bpf_tunnel_key = ::core::mem::zeroed();
    let data = (*skb).data as usize as *mut ::core::ffi::c_void;
    let iph = data as *mut iphdr;
    let data_end = (*skb).data_end as usize as *mut ::core::ffi::c_void;
    let mut ret: ::core::ffi::c_int;

    /* single length check */
    if (data as *mut u8).add(::core::mem::size_of::<iphdr>()) > data_end as *mut u8 {
        log_err(1, line!());
        return TC_ACT_SHOT;
    }

    key.tunnel_ttl = 64;
    if (*iph).protocol == IPPROTO_ICMP {
        key.remote_ipv6[3] = bpf_htonl(0x11); /* ::11 */
    }

    ret = bpf_skb_set_tunnel_key(
        skb,
        &mut key,
        ::core::mem::size_of_val(&key) as u32,
        BPF_F_TUNINFO_IPV6,
    );
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    TC_ACT_OK
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn ipip6_get_tunnel(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int;
    let mut key: bpf_tunnel_key = ::core::mem::zeroed();

    ret = bpf_skb_get_tunnel_key(
        skb,
        &mut key,
        ::core::mem::size_of_val(&key) as u32,
        BPF_F_TUNINFO_IPV6,
    );
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    bpf_printk(
        c"remote ip6 %x::%x\n".as_ptr(),
        bpf_htonl(key.remote_ipv6[0]),
        bpf_htonl(key.remote_ipv6[3]),
    );
    TC_ACT_OK
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn ip6ip6_set_tunnel(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let mut key: bpf_tunnel_key = ::core::mem::zeroed();
    let data = (*skb).data as usize as *mut ::core::ffi::c_void;
    let iph = data as *mut ipv6hdr;
    let data_end = (*skb).data_end as usize as *mut ::core::ffi::c_void;
    let mut ret: ::core::ffi::c_int;

    /* single length check */
    if (data as *mut u8).add(::core::mem::size_of::<ipv6hdr>()) > data_end as *mut u8 {
        log_err(1, line!());
        return TC_ACT_SHOT;
    }

    key.tunnel_ttl = 64;
    if (*iph).nexthdr == 58 {
        /* NEXTHDR_ICMP */
        key.remote_ipv6[3] = bpf_htonl(0x11); /* ::11 */
    }

    ret = bpf_skb_set_tunnel_key(
        skb,
        &mut key,
        ::core::mem::size_of_val(&key) as u32,
        BPF_F_TUNINFO_IPV6,
    );
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    TC_ACT_OK
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn ip6ip6_get_tunnel(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int;
    let mut key: bpf_tunnel_key = ::core::mem::zeroed();

    ret = bpf_skb_get_tunnel_key(
        skb,
        &mut key,
        ::core::mem::size_of_val(&key) as u32,
        BPF_F_TUNINFO_IPV6,
    );
    if ret < 0 {
        log_err(ret, line!());
        return TC_ACT_SHOT;
    }

    bpf_printk(
        c"remote ip6 %x::%x\n".as_ptr(),
        bpf_htonl(key.remote_ipv6[0]),
        bpf_htonl(key.remote_ipv6[3]),
    );
    TC_ACT_OK
}

#[no_mangle]
pub static mut xfrm_reqid: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut xfrm_spi: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut xfrm_remote_ip: ::core::ffi::c_int = 0;

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn xfrm_get_state(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let mut x: bpf_xfrm_state = ::core::mem::zeroed();
    let mut ret: ::core::ffi::c_int;

    ret = bpf_skb_get_xfrm_state(skb, 0, &mut x, ::core::mem::size_of_val(&x) as u32, 0);
    if ret < 0 {
        return TC_ACT_OK;
    }

    xfrm_reqid = x.reqid as _;
    xfrm_spi = bpf_ntohl(x.spi) as _;
    xfrm_remote_ip = bpf_ntohl(x.remote_ipv4) as _;

    TC_ACT_OK
}

#[no_mangle]
pub static mut xfrm_replay_window: ::core::ffi::c_int = 0;

#[link_section = "xdp"]
#[no_mangle]
pub unsafe extern "C" fn xfrm_get_state_xdp(xdp: *mut xdp_md) -> ::core::ffi::c_int {
    let mut opts: bpf_xfrm_state_opts = ::core::mem::zeroed();
    let mut x: *mut xfrm_state = ::core::ptr::null_mut();
    let mut ptr: bpf_dynptr = ::core::mem::zeroed();
    let mut esph_buf: [u8; 8] = [0; 8];
    let mut iph_buf: [u8; 20] = [0; 20];
    let mut off: u32;

    'out: {
        if bpf_dynptr_from_xdp(xdp, 0, &mut ptr) != 0 {
            break 'out;
        }

        off = ::core::mem::size_of::<ethhdr>() as u32;
        let iph = bpf_dynptr_slice(
            &mut ptr,
            off,
            iph_buf.as_mut_ptr() as *mut _,
            iph_buf.len() as u32,
        ) as *mut iphdr;
        if iph.is_null() || (*iph).protocol != IPPROTO_ESP {
            break 'out;
        }

        off += ::core::mem::size_of::<iphdr>() as u32;
        let esph = bpf_dynptr_slice(
            &mut ptr,
            off,
            esph_buf.as_mut_ptr() as *mut _,
            esph_buf.len() as u32,
        ) as *mut ip_esp_hdr;
        if esph.is_null() {
            break 'out;
        }

        opts.netns_id = BPF_F_CURRENT_NETNS;
        opts.daddr.a4 = (*iph).daddr;
        opts.spi = (*esph).spi;
        opts.proto = IPPROTO_ESP;
        opts.family = AF_INET;

        x = bpf_xdp_get_xfrm_state(xdp, &mut opts, ::core::mem::size_of_val(&opts) as u32);
        if x.is_null() {
            break 'out;
        }

        if (*x).replay_esn.is_null() {
            break 'out;
        }

        xfrm_replay_window = (*(*x).replay_esn).replay_window as _;
    }
    if !x.is_null() {
        bpf_xdp_xfrm_state_release(x);
    }
    XDP_PASS
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [::core::ffi::c_char; 4] = [b'G' as _, b'P' as _, b'L' as _, 0];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
