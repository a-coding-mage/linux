// SPDX-License-Identifier: GPL-2.0-only
/*
 * This code is taken from the Android Open Source Project and the author
 * (Maciej Żenczykowski) has gave permission to relicense it under the
 * GPLv2. Therefore this program is free software;
 * You can redistribute it and/or modify it under the terms of the GNU
 * General Public License version 2 as published by the Free Software
 * Foundation

 * The original headers, including the original license headers, are
 * included below for completeness.
 *
 * Copyright (C) 2019 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// Depends on Linux and BPF definitions corresponding to:
// <linux/bpf.h>, <linux/if.h>, <linux/if_ether.h>, <linux/if_packet.h>,
// <linux/in.h>, <linux/in6.h>, <linux/ip.h>, <linux/ipv6.h>,
// <linux/pkt_cls.h>, <linux/swab.h>, <linux/udp.h>,
// <bpf/bpf_helpers.h>, and <bpf/bpf_endian.h>.

const IP_DF: u16 = 0x4000; // Flag: "Don't Fragment"

#[no_mangle]
#[link_section = "schedcls/ingress6/nat_6"]
pub unsafe extern "C" fn sched_cls_ingress6_nat_6_prog(skb: *mut __sk_buff) -> i32 {
    let l2_header_size: i32 = core::mem::size_of::<ethhdr>() as i32;
    let mut data: *mut core::ffi::c_void = (*skb).data as isize as *mut core::ffi::c_void;
    let mut data_end: *const core::ffi::c_void =
        (*skb).data_end as isize as *const core::ffi::c_void;
    let eth: *const ethhdr = data as *const ethhdr; // used iff is_ethernet
    let ip6: *const ipv6hdr = eth.add(1) as *const core::ffi::c_void as *const ipv6hdr;

    // Require ethernet dst mac address to be our unicast address.
    if (*skb).pkt_type != PACKET_HOST {
        return TC_ACT_OK;
    }

    // Must be meta-ethernet IPv6 frame
    if (*skb).protocol != bpf_htons(ETH_P_IPV6 as u16) {
        return TC_ACT_OK;
    }

    // Must have (ethernet and) ipv6 header
    if (data as *mut u8).add(l2_header_size as usize + core::mem::size_of_val(&*ip6))
        > data_end as *mut u8
    {
        return TC_ACT_OK;
    }

    // Ethertype - if present - must be IPv6
    if (*eth).h_proto != bpf_htons(ETH_P_IPV6 as u16) {
        return TC_ACT_OK;
    }

    // IP version must be 6
    if (*ip6).version != 6 {
        return TC_ACT_OK;
    }
    // Maximum IPv6 payload length that can be translated to IPv4
    if bpf_ntohs((*ip6).payload_len) as usize > 0xFFFFusize - core::mem::size_of::<iphdr>() {
        return TC_ACT_OK;
    }
    match (*ip6).nexthdr as i32 {
        IPPROTO_TCP => {
            // For TCP & UDP the checksum neutrality of the chosen IPv6
        }
        IPPROTO_UDP => {
            // address means there is no need to update their checksums.
        }
        IPPROTO_GRE => {
            // We do not need to bother looking at GRE/ESP headers,
        }
        IPPROTO_ESP => {
            // since there is never a checksum to update.
        }
        _ => {
            // do not know how to handle anything else
            return TC_ACT_OK;
        }
    }

    let mut eth2: ethhdr; // used iff is_ethernet

    eth2 = *eth; // Copy over the ethernet header (src/dst mac)
    eth2.h_proto = bpf_htons(ETH_P_IP as u16); // But replace the ethertype

    let mut ip = iphdr {
        version: 4, // u4
        ihl: (core::mem::size_of::<iphdr>() / core::mem::size_of::<u32>()) as _, // u4
        tos: ((*ip6).priority << 4) + ((*ip6).flow_lbl[0] >> 4), // u8
        tot_len: bpf_htons(
            (bpf_ntohs((*ip6).payload_len) as usize + core::mem::size_of::<iphdr>()) as u16,
        ), // u16
        id: 0, // u16
        frag_off: bpf_htons(IP_DF), // u16
        ttl: (*ip6).hop_limit, // u8
        protocol: (*ip6).nexthdr, // u8
        check: 0, // u16
        saddr: 0x0201a8c0, // u32
        daddr: 0x0101a8c0, // u32
    };

    // Calculate the IPv4 one's complement checksum of the IPv4 header.
    let mut sum4: __wsum = 0;

    let mut i = 0usize;
    while i < core::mem::size_of_val(&ip) / core::mem::size_of::<u16>() {
        sum4 = sum4.wrapping_add(*((&ip as *const iphdr as *const u16).add(i)) as __wsum);
        i += 1;
    }

    // Note that sum4 is guaranteed to be non-zero by virtue of ip.version == 4
    sum4 = (sum4 & 0xFFFF).wrapping_add(sum4 >> 16); // collapse u32 into range 1 .. 0x1FFFE
    sum4 = (sum4 & 0xFFFF).wrapping_add(sum4 >> 16); // collapse any potential carry into u16
    ip.check = !(sum4 as u16); // sum4 cannot be zero, so this is never 0xFFFF

    // Calculate the *negative* IPv6 16-bit one's complement checksum of the IPv6 header.
    let mut sum6: __wsum = 0;
    // We'll end up with a non-zero sum due to ip6->version == 6 (which has '0' bits)
    let mut i = 0usize;
    while i < core::mem::size_of_val(&*ip6) / core::mem::size_of::<u16>() {
        sum6 = sum6.wrapping_add(!*((ip6 as *const u16).add(i)) as __wsum); // note the bitwise negation
        i += 1;
    }

    // Note that there is no L4 checksum update: we are relying on the checksum neutrality
    // of the ipv6 address chosen by netd's ClatdController.

    // Packet mutations begin - point of no return, but if this first modification fails
    // the packet is probably still pristine, so let clatd handle it.
    if bpf_skb_change_proto(skb, bpf_htons(ETH_P_IP as u16), 0) != 0 {
        return TC_ACT_OK;
    }
    bpf_csum_update(skb, sum6);

    data = (*skb).data as isize as *mut core::ffi::c_void;
    data_end = (*skb).data_end as isize as *const core::ffi::c_void;
    if (data as *mut u8).add(l2_header_size as usize + core::mem::size_of::<iphdr>())
        > data_end as *mut u8
    {
        return TC_ACT_SHOT;
    }

    let new_eth: *mut ethhdr = data as *mut ethhdr;

    // Copy over the updated ethernet header
    *new_eth = eth2;

    // Copy over the new ipv4 header.
    *(new_eth.add(1) as *mut iphdr) = ip;
    return bpf_redirect((*skb).ifindex, BPF_F_INGRESS);
}

#[no_mangle]
#[link_section = "schedcls/egress4/snat4"]
pub unsafe extern "C" fn sched_cls_egress4_snat4_prog(skb: *mut __sk_buff) -> i32 {
    let l2_header_size: i32 = core::mem::size_of::<ethhdr>() as i32;
    let mut data: *mut core::ffi::c_void = (*skb).data as isize as *mut core::ffi::c_void;
    let mut data_end: *const core::ffi::c_void =
        (*skb).data_end as isize as *const core::ffi::c_void;
    let eth: *const ethhdr = data as *const ethhdr; // used iff is_ethernet
    let ip4: *const iphdr = eth.add(1) as *const core::ffi::c_void as *const iphdr;

    // Must be meta-ethernet IPv4 frame
    if (*skb).protocol != bpf_htons(ETH_P_IP as u16) {
        return TC_ACT_OK;
    }

    // Must have ipv4 header
    if (data as *mut u8).add(l2_header_size as usize + core::mem::size_of::<ipv6hdr>())
        > data_end as *mut u8
    {
        return TC_ACT_OK;
    }

    // Ethertype - if present - must be IPv4
    if (*eth).h_proto != bpf_htons(ETH_P_IP as u16) {
        return TC_ACT_OK;
    }

    // IP version must be 4
    if (*ip4).version != 4 {
        return TC_ACT_OK;
    }

    // We cannot handle IP options, just standard 20 byte == 5 dword minimal IPv4 header
    if (*ip4).ihl != 5 {
        return TC_ACT_OK;
    }

    // Maximum IPv6 payload length that can be translated to IPv4
    if bpf_htons((*ip4).tot_len) as usize > 0xFFFFusize - core::mem::size_of::<ipv6hdr>() {
        return TC_ACT_OK;
    }

    // Calculate the IPv4 one's complement checksum of the IPv4 header.
    let mut sum4: __wsum = 0;

    let mut i = 0usize;
    while i < core::mem::size_of_val(&*ip4) / core::mem::size_of::<u16>() {
        sum4 = sum4.wrapping_add(*((ip4 as *const u16).add(i)) as __wsum);
        i += 1;
    }

    // Note that sum4 is guaranteed to be non-zero by virtue of ip4->version == 4
    sum4 = (sum4 & 0xFFFF).wrapping_add(sum4 >> 16); // collapse u32 into range 1 .. 0x1FFFE
    sum4 = (sum4 & 0xFFFF).wrapping_add(sum4 >> 16); // collapse any potential carry into u16
    // for a correct checksum we should get *a* zero, but sum4 must be positive, ie 0xFFFF
    if sum4 != 0xFFFF {
        return TC_ACT_OK;
    }

    // Minimum IPv4 total length is the size of the header
    if bpf_ntohs((*ip4).tot_len) as usize < core::mem::size_of_val(&*ip4) {
        return TC_ACT_OK;
    }

    // We are incapable of dealing with IPv4 fragments
    if ((*ip4).frag_off & !bpf_htons(IP_DF)) != 0 {
        return TC_ACT_OK;
    }

    match (*ip4).protocol as i32 {
        IPPROTO_TCP => {
            // For TCP & UDP the checksum neutrality of the chosen IPv6
        }
        IPPROTO_GRE => {
            // address means there is no need to update their checksums.
        }
        IPPROTO_ESP => {
            // We do not need to bother looking at GRE/ESP headers,
            // since there is never a checksum to update.
        }

        IPPROTO_UDP => {
            // See above comment, but must also have UDP header...
            if (data as *mut u8).add(core::mem::size_of_val(&*ip4) + core::mem::size_of::<udphdr>())
                > data_end as *mut u8
            {
                return TC_ACT_OK;
            }
            let uh: *const udphdr = ip4.add(1) as *const udphdr;
            // If IPv4/UDP checksum is 0 then fallback to clatd so it can calculate the
            // checksum.  Otherwise the network or more likely the NAT64 gateway might
            // drop the packet because in most cases IPv6/UDP packets with a zero checksum
            // are invalid. See RFC 6935.  TODO: calculate checksum via bpf_csum_diff()
            if (*uh).check == 0 {
                return TC_ACT_OK;
            }
        }

        _ => {
            // do not know how to handle anything else
            return TC_ACT_OK;
        }
    }
    let mut eth2: ethhdr; // used iff is_ethernet

    eth2 = *eth; // Copy over the ethernet header (src/dst mac)
    eth2.h_proto = bpf_htons(ETH_P_IPV6 as u16); // But replace the ethertype

    let mut ip6 = ipv6hdr {
        version: 6, // __u8:4
        priority: (*ip4).tos >> 4, // __u8:4
        flow_lbl: [((*ip4).tos & 0xF) << 4, 0, 0], // __u8[3]
        payload_len: bpf_htons(bpf_ntohs((*ip4).tot_len) - 20), // __be16
        nexthdr: (*ip4).protocol, // __u8
        hop_limit: (*ip4).ttl, // __u8
        ..core::mem::zeroed()
    };
    ip6.saddr.in6_u.u6_addr32[0] = bpf_htonl(0x20010db8);
    ip6.saddr.in6_u.u6_addr32[1] = 0;
    ip6.saddr.in6_u.u6_addr32[2] = 0;
    ip6.saddr.in6_u.u6_addr32[3] = bpf_htonl(1);
    ip6.daddr.in6_u.u6_addr32[0] = bpf_htonl(0x20010db8);
    ip6.daddr.in6_u.u6_addr32[1] = 0;
    ip6.daddr.in6_u.u6_addr32[2] = 0;
    ip6.daddr.in6_u.u6_addr32[3] = bpf_htonl(2);

    // Calculate the IPv6 16-bit one's complement checksum of the IPv6 header.
    let mut sum6: __wsum = 0;
    // We'll end up with a non-zero sum due to ip6.version == 6
    let mut i = 0usize;
    while i < core::mem::size_of_val(&ip6) / core::mem::size_of::<u16>() {
        sum6 = sum6.wrapping_add(*((&ip6 as *const ipv6hdr as *const u16).add(i)) as __wsum);
        i += 1;
    }

    // Packet mutations begin - point of no return, but if this first modification fails
    // the packet is probably still pristine, so let clatd handle it.
    if bpf_skb_change_proto(skb, bpf_htons(ETH_P_IPV6 as u16), 0) != 0 {
        return TC_ACT_OK;
    }

    // This takes care of updating the skb->csum field for a CHECKSUM_COMPLETE packet.
    // In such a case, skb->csum is a 16-bit one's complement sum of the entire payload,
    // thus we need to subtract out the ipv4 header's sum, and add in the ipv6 header's sum.
    // However, we've already verified the ipv4 checksum is correct and thus 0.
    // Thus we only need to add the ipv6 header's sum.
    //
    // bpf_csum_update() always succeeds if the skb is CHECKSUM_COMPLETE and returns an error
    // (-ENOTSUPP) if it isn't.  So we just ignore the return code (see above for more details).
    bpf_csum_update(skb, sum6);

    // bpf_skb_change_proto() invalidates all pointers - reload them.
    data = (*skb).data as isize as *mut core::ffi::c_void;
    data_end = (*skb).data_end as isize as *const core::ffi::c_void;

    // I cannot think of any valid way for this error condition to trigger, however I do
    // believe the explicit check is required to keep the in kernel ebpf verifier happy.
    if (data as *mut u8).add(l2_header_size as usize + core::mem::size_of_val(&ip6))
        > data_end as *mut u8
    {
        return TC_ACT_SHOT;
    }

    let new_eth: *mut ethhdr = data as *mut ethhdr;

    // Copy over the updated ethernet header
    *new_eth = eth2;
    // Copy over the new ipv4 header.
    *(new_eth.add(1) as *mut ipv6hdr) = ip6;
    return TC_ACT_OK;
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";
