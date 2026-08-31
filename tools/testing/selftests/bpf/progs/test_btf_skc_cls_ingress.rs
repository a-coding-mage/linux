// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

// Dependencies from the original C source:
// #include "bpf_tracing_net.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_endian.h>

const ENOENT: i32 = 2;

#[no_mangle]
pub static mut srv_sa6: sockaddr_in6 = unsafe { core::mem::zeroed() };
#[no_mangle]
pub static mut srv_sa4: sockaddr_in = unsafe { core::mem::zeroed() };
#[no_mangle]
pub static mut listen_tp_sport: __u16 = 0;
#[no_mangle]
pub static mut req_sk_sport: __u16 = 0;
#[no_mangle]
pub static mut recv_cookie: __u32 = 0;
#[no_mangle]
pub static mut gen_cookie: __u32 = 0;
#[no_mangle]
pub static mut mss: __u32 = 0;
#[no_mangle]
pub static mut linum: __u32 = 0;

macro_rules! LOG {
    () => {{
        if unsafe { linum } == 0 {
            unsafe {
                linum = line!();
            }
        }
    }};
}

unsafe fn test_syncookie_helper(
    iphdr: *mut core::ffi::c_void,
    iphdr_size: i32,
    th: *mut tcphdr,
    tp: *mut tcp_sock,
    skb: *mut __sk_buff,
) {
    if (*th).syn() != 0 {
        let mss_cookie: __s64;
        let data_end: *mut core::ffi::c_void;

        data_end = (*skb).data_end as isize as *mut core::ffi::c_void;

        if (*th).doff() * 4 != 40 {
            LOG!();
            return;
        }

        if (th as *mut u8).add(40) as *mut core::ffi::c_void > data_end {
            LOG!();
            return;
        }

        mss_cookie = bpf_tcp_gen_syncookie(tp, iphdr, iphdr_size, th, 40);
        if mss_cookie < 0 {
            if mss_cookie != -(ENOENT as __s64) {
                LOG!();
            }
        } else {
            gen_cookie = mss_cookie as __u32;
            mss = (mss_cookie >> 32) as __u32;
        }
    } else if gen_cookie != 0 {
        /* It was in cookie mode */
        let ret: i32 = bpf_tcp_check_syncookie(
            tp,
            iphdr,
            iphdr_size,
            th,
            core::mem::size_of::<tcphdr>() as __u32,
        );

        if ret < 0 {
            if ret != -ENOENT {
                LOG!();
            }
        } else {
            recv_cookie = bpf_ntohl((*th).ack_seq) - 1;
        }
    }
}

unsafe fn handle_ip_tcp(eth: *mut ethhdr, skb: *mut __sk_buff) -> i32 {
    let mut tuple: *mut bpf_sock_tuple = core::ptr::null_mut();
    let mut tuple_len: u32 = 0;
    let bpf_skc: *mut bpf_sock;
    let data_end: *mut core::ffi::c_void;
    let iphdr: *mut core::ffi::c_void;
    let ip6h: *mut ipv6hdr;
    let ip4h: *mut iphdr;
    let th: *mut tcphdr;
    let iphdr_size: i32;

    data_end = (*skb).data_end as isize as *mut core::ffi::c_void;

    match (*eth).h_proto {
        x if x == bpf_htons(ETH_P_IP as __u16) => {
            ip4h = eth.add(1) as *mut iphdr;
            if ip4h.add(1) as *mut core::ffi::c_void > data_end {
                return TC_ACT_OK;
            }
            if (*ip4h).protocol != IPPROTO_TCP as __u8 {
                return TC_ACT_OK;
            }
            th = ip4h.add(1) as *mut tcphdr;
            if th.add(1) as *mut core::ffi::c_void > data_end {
                return TC_ACT_OK;
            }
            /* Is it the testing traffic? */
            if (*th).dest != srv_sa4.sin_port {
                return TC_ACT_OK;
            }
            tuple_len = core::mem::size_of_val(&(*tuple).ipv4) as u32;
            tuple = &mut (*ip4h).saddr as *mut _ as *mut bpf_sock_tuple;
            iphdr = ip4h as *mut core::ffi::c_void;
            iphdr_size = core::mem::size_of::<iphdr>() as i32;
        }
        x if x == bpf_htons(ETH_P_IPV6 as __u16) => {
            ip6h = eth.add(1) as *mut ipv6hdr;
            if ip6h.add(1) as *mut core::ffi::c_void > data_end {
                return TC_ACT_OK;
            }
            if (*ip6h).nexthdr != IPPROTO_TCP as __u8 {
                return TC_ACT_OK;
            }
            th = ip6h.add(1) as *mut tcphdr;
            if th.add(1) as *mut core::ffi::c_void > data_end {
                return TC_ACT_OK;
            }
            /* Is it the testing traffic? */
            if (*th).dest != srv_sa6.sin6_port {
                return TC_ACT_OK;
            }
            tuple_len = core::mem::size_of_val(&(*tuple).ipv6) as u32;
            tuple = &mut (*ip6h).saddr as *mut _ as *mut bpf_sock_tuple;
            iphdr = ip6h as *mut core::ffi::c_void;
            iphdr_size = core::mem::size_of::<ipv6hdr>() as i32;
        }
        _ => {
            return TC_ACT_OK;
        }
    }

    if (tuple as *mut u8).add(tuple_len as usize) as *mut core::ffi::c_void > data_end {
        LOG!();
        return TC_ACT_OK;
    }

    bpf_skc = bpf_skc_lookup_tcp(skb, tuple, tuple_len, BPF_F_CURRENT_NETNS, 0);
    if bpf_skc.is_null() {
        LOG!();
        return TC_ACT_OK;
    }

    if (*bpf_skc).state == BPF_TCP_NEW_SYN_RECV {
        let req_sk: *mut request_sock;

        req_sk = bpf_skc_to_tcp_request_sock(bpf_skc) as *mut request_sock;
        if req_sk.is_null() {
            LOG!();
            bpf_sk_release(bpf_skc as *mut core::ffi::c_void);
            return TC_ACT_OK;
        }

        if bpf_sk_assign(skb, req_sk as *mut core::ffi::c_void, 0) != 0 {
            LOG!();
            bpf_sk_release(bpf_skc as *mut core::ffi::c_void);
            return TC_ACT_OK;
        }

        req_sk_sport = (*req_sk).__req_common.skc_num;

        bpf_sk_release(req_sk as *mut core::ffi::c_void);
        return TC_ACT_OK;
    } else if (*bpf_skc).state == BPF_TCP_LISTEN {
        let tp: *mut tcp_sock;

        tp = bpf_skc_to_tcp_sock(bpf_skc);
        if tp.is_null() {
            LOG!();
            bpf_sk_release(bpf_skc as *mut core::ffi::c_void);
            return TC_ACT_OK;
        }

        if bpf_sk_assign(skb, tp as *mut core::ffi::c_void, 0) != 0 {
            LOG!();
            bpf_sk_release(bpf_skc as *mut core::ffi::c_void);
            return TC_ACT_OK;
        }

        listen_tp_sport = (*tp).inet_conn.icsk_inet.sk.__sk_common.skc_num;

        test_syncookie_helper(iphdr, iphdr_size, th, tp, skb);
        bpf_sk_release(tp as *mut core::ffi::c_void);
        return TC_ACT_OK;
    }

    if bpf_sk_assign(skb, bpf_skc as *mut core::ffi::c_void, 0) != 0 {
        LOG!();
    }

    bpf_sk_release(bpf_skc as *mut core::ffi::c_void);
    TC_ACT_OK
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn cls_ingress(skb: *mut __sk_buff) -> i32 {
    let eth: *mut ethhdr;
    let data_end: *mut core::ffi::c_void;

    data_end = (*skb).data_end as isize as *mut core::ffi::c_void;

    eth = (*skb).data as isize as *mut ethhdr;
    if eth.add(1) as *mut core::ffi::c_void > data_end {
        return TC_ACT_OK;
    }

    if (*eth).h_proto != bpf_htons(ETH_P_IP as __u16)
        && (*eth).h_proto != bpf_htons(ETH_P_IPV6 as __u16)
    {
        return TC_ACT_OK;
    }

    handle_ip_tcp(eth, skb)
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";
