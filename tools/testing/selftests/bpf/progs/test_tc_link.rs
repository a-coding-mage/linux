// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Isovalent */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type bool_ = bool;

const ETH_P_IP: u16 = 0x0800;
const PACKET_HOST: u32 = 0;
const PACKET_MULTICAST: u32 = 2;
const TCX_NEXT: i32 = -1;
const TCX_PASS: i32 = 0;

#[repr(C)]
pub struct __sk_buff {
    pub len: u32,
    pub pkt_type: u32,
    pub mark: u32,
    pub queue_mapping: u32,
    pub protocol: u32,
    pub vlan_present: u32,
    pub vlan_tci: u32,
    pub vlan_proto: u32,
    pub priority: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethhdr {
    pub h_dest: [u8; 6],
    pub h_source: [u8; 6],
    pub h_proto: u16,
}

impl Default for ethhdr {
    fn default() -> Self {
        Self {
            h_dest: [0; 6],
            h_source: [0; 6],
            h_proto: 0,
        }
    }
}

extern "C" {
    fn bpf_skb_load_bytes(skb: *mut __sk_buff, offset: i32, to: *mut core::ffi::c_void, len: u32) -> i32;
    fn bpf_skb_store_bytes(
        skb: *mut __sk_buff,
        offset: i32,
        from: *const core::ffi::c_void,
        len: u32,
        flags: u64,
    ) -> i32;
    fn bpf_skb_change_type(skb: *mut __sk_buff, type_: u32) -> i32;
}

#[inline]
const fn __bpf_constant_htons(x: u16) -> u16 {
    x.to_be()
}

#[inline]
const fn bpf_htons(x: u16) -> u16 {
    x.to_be()
}

#[no_mangle]
#[link_section = "license"]
pub static LICENSE: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut seen_tc1: bool_ = false;
#[no_mangle]
pub static mut seen_tc2: bool_ = false;
#[no_mangle]
pub static mut seen_tc3: bool_ = false;
#[no_mangle]
pub static mut seen_tc4: bool_ = false;
#[no_mangle]
pub static mut seen_tc5: bool_ = false;
#[no_mangle]
pub static mut seen_tc6: bool_ = false;
#[no_mangle]
pub static mut seen_tc7: bool_ = false;
#[no_mangle]
pub static mut seen_tc8: bool_ = false;

#[no_mangle]
pub static mut set_type: bool_ = false;

#[no_mangle]
pub static mut seen_eth: bool_ = false;
#[no_mangle]
pub static mut seen_host: bool_ = false;
#[no_mangle]
pub static mut seen_mcast: bool_ = false;

#[no_mangle]
pub static mut mark: i32 = 0;
#[no_mangle]
pub static mut prio: i32 = 0;
#[no_mangle]
pub static mut headroom: u16 = 0;
#[no_mangle]
pub static mut tailroom: u16 = 0;

#[no_mangle]
#[link_section = "tc/ingress"]
pub unsafe extern "C" fn tc1(skb: *mut __sk_buff) -> i32 {
    let mut eth: ethhdr = ethhdr::default();

    if (*skb).protocol != __bpf_constant_htons(ETH_P_IP) as u32 {
        seen_tc1 = true;
        return TCX_NEXT;
    }
    if bpf_skb_load_bytes(
        skb,
        0,
        &mut eth as *mut ethhdr as *mut core::ffi::c_void,
        core::mem::size_of::<ethhdr>() as u32,
    ) != 0 {
        seen_tc1 = true;
        return TCX_NEXT;
    }
    seen_eth = eth.h_proto == bpf_htons(ETH_P_IP);
    seen_host = (*skb).pkt_type == PACKET_HOST;
    if seen_host && set_type {
        eth.h_dest[0] = 4;
        if bpf_skb_store_bytes(
            skb,
            0,
            &eth as *const ethhdr as *const core::ffi::c_void,
            core::mem::size_of::<ethhdr>() as u32,
            0,
        ) != 0 {
            return TCX_NEXT;
        }
        bpf_skb_change_type(skb, PACKET_MULTICAST);
    }
    seen_tc1 = true;
    TCX_NEXT
}

#[no_mangle]
#[link_section = "tc/egress"]
pub unsafe extern "C" fn tc2(_skb: *mut __sk_buff) -> i32 {
    seen_tc2 = true;
    TCX_NEXT
}

#[no_mangle]
#[link_section = "tc/egress"]
pub unsafe extern "C" fn tc3(_skb: *mut __sk_buff) -> i32 {
    seen_tc3 = true;
    TCX_NEXT
}

#[no_mangle]
#[link_section = "tc/egress"]
pub unsafe extern "C" fn tc4(_skb: *mut __sk_buff) -> i32 {
    seen_tc4 = true;
    TCX_NEXT
}

#[no_mangle]
#[link_section = "tc/egress"]
pub unsafe extern "C" fn tc5(_skb: *mut __sk_buff) -> i32 {
    seen_tc5 = true;
    TCX_PASS
}

#[no_mangle]
#[link_section = "tc/egress"]
pub unsafe extern "C" fn tc6(_skb: *mut __sk_buff) -> i32 {
    seen_tc6 = true;
    TCX_PASS
}

#[no_mangle]
#[link_section = "tc/ingress"]
pub unsafe extern "C" fn tc7(skb: *mut __sk_buff) -> i32 {
    let mut eth: ethhdr = ethhdr::default();

    if (*skb).protocol != __bpf_constant_htons(ETH_P_IP) as u32 {
        seen_tc7 = true;
        return TCX_PASS;
    }
    if bpf_skb_load_bytes(
        skb,
        0,
        &mut eth as *mut ethhdr as *mut core::ffi::c_void,
        core::mem::size_of::<ethhdr>() as u32,
    ) != 0 {
        seen_tc7 = true;
        return TCX_PASS;
    }
    if eth.h_dest[0] == 4 && set_type {
        seen_mcast = (*skb).pkt_type == PACKET_MULTICAST;
        bpf_skb_change_type(skb, PACKET_HOST);
    }
    seen_tc7 = true;
    TCX_PASS
}

#[repr(C)]
pub struct sk_buff {
    pub dev: *mut net_device,
}

#[repr(C)]
pub struct net_device {
    pub needed_headroom: u16,
    pub needed_tailroom: u16,
}

#[no_mangle]
#[link_section = "tc/egress"]
pub unsafe extern "C" fn tc8(skb: *mut __sk_buff) -> i32 {
    /* BPF_CORE_READ((struct sk_buff *)skb, dev) */
    let dev: *mut net_device = (*(skb as *mut sk_buff)).dev;

    seen_tc8 = true;
    mark = (*skb).mark as i32;
    prio = (*skb).priority as i32;
    /* BPF_CORE_READ(dev, needed_headroom) */
    headroom = (*dev).needed_headroom;
    /* BPF_CORE_READ(dev, needed_tailroom) */
    tailroom = (*dev).needed_tailroom;
    TCX_PASS
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
