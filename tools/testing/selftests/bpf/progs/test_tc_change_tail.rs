// SPDX-License-Identifier: GPL-2.0
// Source dependencies from C: "vmlinux.h" and <bpf/bpf_helpers.h>.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

const PAGE_SIZE: usize = __PAGE_SIZE as usize;
const BPF_SKB_MAX_LEN: i32 = (PAGE_SIZE << 2) as i32;

#[no_mangle]
pub static mut change_tail_ret: i64 = 1;

#[inline(always)]
unsafe fn parse_ip_header(skb: *mut __sk_buff, ip_proto: *mut i32) -> *mut iphdr {
    let data_end = (*skb).data_end as usize as *mut core::ffi::c_void;
    let data = (*skb).data as usize as *mut core::ffi::c_void;
    let eth = data as *mut ethhdr;
    let iph: *mut iphdr;

    /* Verify Ethernet header */
    if (data as *mut u8).add(core::mem::size_of_val(&*eth)) as *mut core::ffi::c_void > data_end {
        return core::ptr::null_mut();
    }

    /* Skip Ethernet header to get to IP header */
    iph = (data as *mut u8).add(core::mem::size_of::<ethhdr>()) as *mut iphdr;

    /* Verify IP header */
    if (data as *mut u8)
        .add(core::mem::size_of::<ethhdr>() + core::mem::size_of_val(&*iph))
        as *mut core::ffi::c_void
        > data_end
    {
        return core::ptr::null_mut();
    }

    /* Basic IP header validation */
    if (*iph).version != 4 {
        /* Only support IPv4 */
        return core::ptr::null_mut();
    }

    if (*iph).ihl < 5 {
        /* Minimum IP header length */
        return core::ptr::null_mut();
    }

    *ip_proto = (*iph).protocol as i32;
    iph
}

#[inline(always)]
unsafe fn parse_udp_header(skb: *mut __sk_buff, iph: *mut iphdr) -> *mut udphdr {
    let data_end = (*skb).data_end as usize as *mut core::ffi::c_void;
    let mut hdr = iph as *mut core::ffi::c_void;
    let udp: *mut udphdr;

    /* Calculate UDP header position */
    udp = (hdr as *mut u8).add(((*iph).ihl as usize) * 4) as *mut udphdr;
    hdr = udp as *mut core::ffi::c_void;

    /* Verify UDP header bounds */
    if (hdr as *mut u8).add(core::mem::size_of_val(&*udp)) as *mut core::ffi::c_void > data_end {
        return core::ptr::null_mut();
    }

    udp
}

#[no_mangle]
#[link_section = "tc/ingress"]
pub unsafe extern "C" fn change_tail(skb: *mut __sk_buff) -> i32 {
    let len = (*skb).len as i32;
    let mut udp: *mut udphdr;
    let mut iph: *mut iphdr;
    let data_end: *mut core::ffi::c_void;
    let payload: *mut i8;
    let mut ip_proto: i32 = 0;

    bpf_skb_pull_data(skb, len as u32);

    data_end = (*skb).data_end as usize as *mut core::ffi::c_void;
    iph = parse_ip_header(skb, &mut ip_proto);
    if iph.is_null() {
        return TCX_PASS;
    }

    if ip_proto != IPPROTO_UDP {
        return TCX_PASS;
    }

    udp = parse_udp_header(skb, iph);
    if udp.is_null() {
        return TCX_PASS;
    }

    payload = (udp as *mut u8).add(core::mem::size_of::<udphdr>()) as *mut i8;
    if payload.add(1) > data_end as *mut i8 {
        return TCX_PASS;
    }

    if *payload == b'T' as i8 {
        /* Trim the packet */
        change_tail_ret = bpf_skb_change_tail(skb, (len - 1) as u32, 0) as i64;
        if change_tail_ret == 0 {
            bpf_skb_change_tail(skb, len as u32, 0);
        }
        return TCX_PASS;
    } else if *payload == b'G' as i8 {
        /* Grow the packet */
        change_tail_ret = bpf_skb_change_tail(skb, (len + 1) as u32, 0) as i64;
        if change_tail_ret == 0 {
            bpf_skb_change_tail(skb, len as u32, 0);
        }
        return TCX_PASS;
    } else if *payload == b'E' as i8 {
        /* Error */
        change_tail_ret = bpf_skb_change_tail(skb, BPF_SKB_MAX_LEN as u32, 0) as i64;
        return TCX_PASS;
    } else if *payload == b'Z' as i8 {
        /* Zero */
        change_tail_ret = bpf_skb_change_tail(skb, 0, 0) as i64;
        return TCX_PASS;
    }
    TCX_DROP
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";
