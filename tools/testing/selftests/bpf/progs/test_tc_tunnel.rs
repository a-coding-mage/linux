// SPDX-License-Identifier: GPL-2.0

/* In-place tunneling */

// C dependencies translated as external Rust dependencies:
// <vmlinux.h>, <bpf/bpf_helpers.h>, <bpf/bpf_endian.h>,
// <bpf/bpf_core_read.h>, "bpf_tracing_net.h", "bpf_compiler.h"

// Original C suppressed -Waddress-of-packed-member for packed header access.

static cfg_port: i32 = 8000;

static cfg_udp_src: i32 = 20000;

const ETH_P_MPLS_UC: u16 = 0x8847;
const ETH_P_TEB: u16 = 0x6558;

const MPLS_LS_S_MASK: u32 = 0x00000100;

#[inline(always)]
unsafe fn BPF_F_ADJ_ROOM_ENCAP_L2(len: i32) -> u64 {
    (((len as u64) & BPF_ADJ_ROOM_ENCAP_L2_MASK) << BPF_ADJ_ROOM_ENCAP_L2_SHIFT)
}

#[repr(C)]
struct vxlanhdr___local {
    vx_flags: __be32,
    vx_vni: __be32,
}

const L2_PAD_SZ: usize = core::mem::size_of::<vxlanhdr___local>() + ETH_HLEN as usize;

const UDP_PORT: u16 = 5555;
const MPLS_OVER_UDP_PORT: u16 = 6635;
const ETH_OVER_UDP_PORT: u16 = 7777;
const VXLAN_UDP_PORT: u16 = 8472;

const EXTPROTO_VXLAN: u16 = 0x1;

const SKB_GSO_UDP_TUNNEL_MASK: u32 = SKB_GSO_UDP_TUNNEL | SKB_GSO_UDP_TUNNEL_CSUM;

const SKB_GSO_TUNNEL_MASK: u32 = SKB_GSO_UDP_TUNNEL_MASK
    | SKB_GSO_GRE
    | SKB_GSO_GRE_CSUM
    | SKB_GSO_IPXIP4
    | SKB_GSO_IPXIP6
    | SKB_GSO_ESP;

const BPF_F_ADJ_ROOM_DECAP_L4_MASK: u64 =
    BPF_F_ADJ_ROOM_DECAP_L4_UDP | BPF_F_ADJ_ROOM_DECAP_L4_GRE;

const BPF_F_ADJ_ROOM_DECAP_IPXIP_MASK: u64 =
    BPF_F_ADJ_ROOM_DECAP_IPXIP4 | BPF_F_ADJ_ROOM_DECAP_IPXIP6;

#[inline(always)]
unsafe fn VXLAN_FLAGS() -> __be32 {
    bpf_htonl(1 << 27)
}

const VNI_ID: u32 = 1;

#[inline(always)]
unsafe fn VXLAN_VNI() -> __be32 {
    bpf_htonl(VNI_ID << 8)
}

// Defined by linux/in6.h when available in C.
const NEXTHDR_DEST: i32 = 60;

/* MPLS label 1000 with S bit (last label) set and ttl of 255. */
static mpls_label: __u32 = __bpf_constant_htonl((1000 << 12) | MPLS_LS_S_MASK | 0xff);

#[repr(C, packed)]
struct gre_hdr {
    flags: __be16,
    protocol: __be16,
}

#[repr(C)]
union l4hdr {
    udp: udphdr,
    gre: gre_hdr,
}

#[repr(C, packed)]
struct v4hdr {
    ip: iphdr,
    l4hdr: l4hdr,
    pad: [__u8; L2_PAD_SZ], /* space for L2 header / vxlan header ... */
}

#[repr(C, packed)]
struct v6hdr {
    ip: ipv6hdr,
    l4hdr: l4hdr,
    pad: [__u8; L2_PAD_SZ], /* space for L2 header / vxlan header ... */
}

#[inline(always)]
unsafe fn set_ipv4_csum(iph: *mut iphdr) {
    let mut iph16: *mut __u16 = iph as *mut __u16;
    let mut csum: __u32;
    let mut i: i32;

    (*iph).check = 0;

    i = 0;
    csum = 0;
    while i < (core::mem::size_of_val(&*iph) >> 1) as i32 {
        csum = csum.wrapping_add(*iph16 as __u32);
        iph16 = iph16.add(1);
        i += 1;
    }

    (*iph).check = !((csum & 0xffff).wrapping_add(csum >> 16)) as __u16;
}

#[inline(always)]
unsafe fn __encap_ipv4(
    skb: *mut __sk_buff,
    encap_proto: __u8,
    l2_proto: __u16,
    ext_proto: __u16,
) -> i32 {
    let mut iph_inner: iphdr = core::mem::zeroed();
    let mut udp_dst: __u16 = UDP_PORT;
    let mut h_outer: v4hdr = core::mem::zeroed();
    let mut tcph: tcphdr = core::mem::zeroed();
    let mut olen: i32;
    let mut l2_len: i32;
    let mut l2_hdr: *mut __u8 = core::ptr::null_mut();
    let tcp_off: i32;
    let mut flags: __u64;

    /* Most tests encapsulate a packet into a tunnel with the same
     * network protocol, and derive the outer header fields from
     * the inner header.
     *
     * The 6in4 case tests different inner and outer protocols. As
     * the inner is ipv6, but the outer expects an ipv4 header as
     * input, manually build a struct iphdr based on the ipv6hdr.
     */
    if encap_proto == IPPROTO_IPV6 {
        const saddr: __u32 = (192 << 24) | (168 << 16) | (1 << 8) | 1;
        const daddr: __u32 = (192 << 24) | (168 << 16) | (1 << 8) | 2;
        let mut iph6_inner: ipv6hdr = core::mem::zeroed();

        /* Read the IPv6 header */
        if bpf_skb_load_bytes(
            skb,
            ETH_HLEN as i32,
            &mut iph6_inner as *mut _ as *mut _,
            core::mem::size_of_val(&iph6_inner) as u32,
        ) < 0
        {
            return TC_ACT_OK;
        }

        /* Derive the IPv4 header fields from the IPv6 header */
        iph_inner.version = 4;
        iph_inner.ihl = 5;
        iph_inner.tot_len = bpf_htons(
            (core::mem::size_of_val(&iph6_inner) as u16).wrapping_add(bpf_ntohs(iph6_inner.payload_len)),
        );
        iph_inner.ttl = iph6_inner.hop_limit.wrapping_sub(1);
        iph_inner.protocol = iph6_inner.nexthdr;
        iph_inner.saddr = __bpf_constant_htonl(saddr);
        iph_inner.daddr = __bpf_constant_htonl(daddr);

        tcp_off = core::mem::size_of_val(&iph6_inner) as i32;
    } else {
        if bpf_skb_load_bytes(
            skb,
            ETH_HLEN as i32,
            &mut iph_inner as *mut _ as *mut _,
            core::mem::size_of_val(&iph_inner) as u32,
        ) < 0
        {
            return TC_ACT_OK;
        }

        tcp_off = core::mem::size_of_val(&iph_inner) as i32;
    }

    /* filter only packets we want */
    if iph_inner.ihl != 5 || iph_inner.protocol != IPPROTO_TCP {
        return TC_ACT_OK;
    }

    if bpf_skb_load_bytes(
        skb,
        ETH_HLEN as i32 + tcp_off,
        &mut tcph as *mut _ as *mut _,
        core::mem::size_of_val(&tcph) as u32,
    ) < 0
    {
        return TC_ACT_OK;
    }

    if tcph.dest != __bpf_constant_htons(cfg_port as u16) {
        return TC_ACT_OK;
    }

    olen = core::mem::size_of_val(&h_outer.ip) as i32;
    l2_len = 0;

    flags = BPF_F_ADJ_ROOM_FIXED_GSO | BPF_F_ADJ_ROOM_ENCAP_L3_IPV4;

    match l2_proto {
        ETH_P_MPLS_UC => {
            l2_len = core::mem::size_of_val(&mpls_label) as i32;
            udp_dst = MPLS_OVER_UDP_PORT;
        }
        ETH_P_TEB => {
            l2_len = ETH_HLEN as i32;
            if (ext_proto & EXTPROTO_VXLAN) != 0 {
                udp_dst = VXLAN_UDP_PORT;
                l2_len += core::mem::size_of::<vxlanhdr___local>() as i32;
            } else {
                udp_dst = ETH_OVER_UDP_PORT;
            }
        }
        _ => {}
    }
    flags |= BPF_F_ADJ_ROOM_ENCAP_L2(l2_len);

    match encap_proto {
        IPPROTO_GRE => {
            flags |= BPF_F_ADJ_ROOM_ENCAP_L4_GRE;
            olen += core::mem::size_of::<gre_hdr>() as i32;
            h_outer.l4hdr.gre.protocol = bpf_htons(l2_proto);
            h_outer.l4hdr.gre.flags = 0;
        }
        IPPROTO_UDP => {
            flags |= BPF_F_ADJ_ROOM_ENCAP_L4_UDP;
            olen += core::mem::size_of::<udphdr>() as i32;
            h_outer.l4hdr.udp.source = __bpf_constant_htons(cfg_udp_src as u16);
            h_outer.l4hdr.udp.dest = bpf_htons(udp_dst);
            h_outer.l4hdr.udp.check = 0;
            h_outer.l4hdr.udp.len = bpf_htons(
                bpf_ntohs(iph_inner.tot_len)
                    .wrapping_add(core::mem::size_of::<udphdr>() as u16)
                    .wrapping_add(l2_len as u16),
            );
        }
        IPPROTO_IPIP | IPPROTO_IPV6 => {}
        _ => return TC_ACT_OK,
    }

    /* add L2 encap (if specified) */
    l2_hdr = (&mut h_outer as *mut v4hdr as *mut __u8).add(olen as usize);
    match l2_proto {
        ETH_P_MPLS_UC => {
            *(l2_hdr as *mut __u32) = mpls_label;
        }
        ETH_P_TEB => {
            flags |= BPF_F_ADJ_ROOM_ENCAP_L2_ETH;

            if (ext_proto & EXTPROTO_VXLAN) != 0 {
                let vxlan_hdr: *mut vxlanhdr___local = l2_hdr as *mut vxlanhdr___local;

                (*vxlan_hdr).vx_flags = VXLAN_FLAGS();
                (*vxlan_hdr).vx_vni = VXLAN_VNI();

                l2_hdr = l2_hdr.add(core::mem::size_of::<vxlanhdr___local>());
            }

            if bpf_skb_load_bytes(skb, 0, l2_hdr as *mut _, ETH_HLEN as u32) != 0 {
                return TC_ACT_SHOT;
            }
        }
        _ => {}
    }
    olen += l2_len;

    /* add room between mac and network header */
    if bpf_skb_adjust_room(skb, olen, BPF_ADJ_ROOM_MAC, flags) != 0 {
        return TC_ACT_SHOT;
    }

    /* prepare new outer network header */
    h_outer.ip = iph_inner;
    h_outer.ip.tot_len = bpf_htons((olen as u16).wrapping_add(bpf_ntohs(h_outer.ip.tot_len)));
    h_outer.ip.protocol = encap_proto;

    set_ipv4_csum(&mut h_outer.ip as *mut _ as *mut _);

    /* store new outer network header */
    if bpf_skb_store_bytes(
        skb,
        ETH_HLEN as i32,
        &mut h_outer as *mut _ as *mut _,
        olen as u32,
        BPF_F_INVALIDATE_HASH,
    ) < 0
    {
        return TC_ACT_SHOT;
    }

    /* if changing outer proto type, update eth->h_proto */
    if encap_proto == IPPROTO_IPV6 {
        let mut eth: ethhdr = core::mem::zeroed();

        if bpf_skb_load_bytes(
            skb,
            0,
            &mut eth as *mut _ as *mut _,
            core::mem::size_of_val(&eth) as u32,
        ) < 0
        {
            return TC_ACT_SHOT;
        }
        eth.h_proto = bpf_htons(ETH_P_IP);
        if bpf_skb_store_bytes(
            skb,
            0,
            &mut eth as *mut _ as *mut _,
            core::mem::size_of_val(&eth) as u32,
            0,
        ) < 0
        {
            return TC_ACT_SHOT;
        }
    }

    TC_ACT_OK
}

#[inline(always)]
unsafe fn encap_ipv4(skb: *mut __sk_buff, encap_proto: __u8, l2_proto: __u16) -> i32 {
    __encap_ipv4(skb, encap_proto, l2_proto, 0)
}

#[inline(always)]
unsafe fn __encap_ipv6(
    skb: *mut __sk_buff,
    encap_proto: __u8,
    l2_proto: __u16,
    ext_proto: __u16,
) -> i32 {
    let mut udp_dst: __u16 = UDP_PORT;
    let mut iph_inner: ipv6hdr = core::mem::zeroed();
    let mut h_outer: v6hdr = core::mem::zeroed();
    let mut tcph: tcphdr = core::mem::zeroed();
    let mut olen: i32;
    let mut l2_len: i32;
    let mut l2_hdr: *mut __u8 = core::ptr::null_mut();
    let mut tot_len: __u16;
    let mut flags: __u64;

    if bpf_skb_load_bytes(
        skb,
        ETH_HLEN as i32,
        &mut iph_inner as *mut _ as *mut _,
        core::mem::size_of_val(&iph_inner) as u32,
    ) < 0
    {
        return TC_ACT_OK;
    }

    /* filter only packets we want */
    if bpf_skb_load_bytes(
        skb,
        ETH_HLEN as i32 + core::mem::size_of_val(&iph_inner) as i32,
        &mut tcph as *mut _ as *mut _,
        core::mem::size_of_val(&tcph) as u32,
    ) < 0
    {
        return TC_ACT_OK;
    }

    if tcph.dest != __bpf_constant_htons(cfg_port as u16) {
        return TC_ACT_OK;
    }

    olen = core::mem::size_of_val(&h_outer.ip) as i32;
    l2_len = 0;

    flags = BPF_F_ADJ_ROOM_FIXED_GSO | BPF_F_ADJ_ROOM_ENCAP_L3_IPV6;

    match l2_proto {
        ETH_P_MPLS_UC => {
            l2_len = core::mem::size_of_val(&mpls_label) as i32;
            udp_dst = MPLS_OVER_UDP_PORT;
        }
        ETH_P_TEB => {
            l2_len = ETH_HLEN as i32;
            if (ext_proto & EXTPROTO_VXLAN) != 0 {
                udp_dst = VXLAN_UDP_PORT;
                l2_len += core::mem::size_of::<vxlanhdr___local>() as i32;
            } else {
                udp_dst = ETH_OVER_UDP_PORT;
            }
        }
        _ => {}
    }
    flags |= BPF_F_ADJ_ROOM_ENCAP_L2(l2_len);

    match encap_proto {
        IPPROTO_GRE => {
            flags |= BPF_F_ADJ_ROOM_ENCAP_L4_GRE;
            olen += core::mem::size_of::<gre_hdr>() as i32;
            h_outer.l4hdr.gre.protocol = bpf_htons(l2_proto);
            h_outer.l4hdr.gre.flags = 0;
        }
        IPPROTO_UDP => {
            flags |= BPF_F_ADJ_ROOM_ENCAP_L4_UDP;
            olen += core::mem::size_of::<udphdr>() as i32;
            h_outer.l4hdr.udp.source = __bpf_constant_htons(cfg_udp_src as u16);
            h_outer.l4hdr.udp.dest = bpf_htons(udp_dst);
            tot_len = bpf_ntohs(iph_inner.payload_len)
                .wrapping_add(core::mem::size_of_val(&iph_inner) as u16)
                .wrapping_add(core::mem::size_of::<udphdr>() as u16)
                .wrapping_add(l2_len as u16);
            h_outer.l4hdr.udp.check = 0;
            h_outer.l4hdr.udp.len = bpf_htons(tot_len);
        }
        IPPROTO_IPV6 => {}
        _ => return TC_ACT_OK,
    }

    /* add L2 encap (if specified) */
    l2_hdr = (&mut h_outer as *mut v6hdr as *mut __u8).add(olen as usize);
    match l2_proto {
        ETH_P_MPLS_UC => {
            *(l2_hdr as *mut __u32) = mpls_label;
        }
        ETH_P_TEB => {
            flags |= BPF_F_ADJ_ROOM_ENCAP_L2_ETH;

            if (ext_proto & EXTPROTO_VXLAN) != 0 {
                let vxlan_hdr: *mut vxlanhdr___local = l2_hdr as *mut vxlanhdr___local;

                (*vxlan_hdr).vx_flags = VXLAN_FLAGS();
                (*vxlan_hdr).vx_vni = VXLAN_VNI();

                l2_hdr = l2_hdr.add(core::mem::size_of::<vxlanhdr___local>());
            }

            if bpf_skb_load_bytes(skb, 0, l2_hdr as *mut _, ETH_HLEN as u32) != 0 {
                return TC_ACT_SHOT;
            }
        }
        _ => {}
    }
    olen += l2_len;

    /* add room between mac and network header */
    if bpf_skb_adjust_room(skb, olen, BPF_ADJ_ROOM_MAC, flags) != 0 {
        return TC_ACT_SHOT;
    }

    /* prepare new outer network header */
    h_outer.ip = iph_inner;
    h_outer.ip.payload_len =
        bpf_htons((olen as u16).wrapping_add(bpf_ntohs(h_outer.ip.payload_len)));

    h_outer.ip.nexthdr = encap_proto;

    /* store new outer network header */
    if bpf_skb_store_bytes(
        skb,
        ETH_HLEN as i32,
        &mut h_outer as *mut _ as *mut _,
        olen as u32,
        BPF_F_INVALIDATE_HASH,
    ) < 0
    {
        return TC_ACT_SHOT;
    }

    TC_ACT_OK
}

unsafe fn encap_ipv6_ipip6(skb: *mut __sk_buff) -> i32 {
    let mut h_outer: v6hdr = core::mem::zeroed();
    let mut iph_inner: iphdr = core::mem::zeroed();
    let mut tcph: tcphdr = core::mem::zeroed();
    let mut eth: ethhdr = core::mem::zeroed();
    let mut flags: __u64;
    let olen: i32;

    if bpf_skb_load_bytes(
        skb,
        ETH_HLEN as i32,
        &mut iph_inner as *mut _ as *mut _,
        core::mem::size_of_val(&iph_inner) as u32,
    ) < 0
    {
        return TC_ACT_OK;
    }

    /* filter only packets we want */
    if bpf_skb_load_bytes(
        skb,
        ETH_HLEN as i32 + ((iph_inner.ihl as i32) << 2),
        &mut tcph as *mut _ as *mut _,
        core::mem::size_of_val(&tcph) as u32,
    ) < 0
    {
        return TC_ACT_OK;
    }

    if tcph.dest != __bpf_constant_htons(cfg_port as u16) {
        return TC_ACT_OK;
    }

    olen = core::mem::size_of_val(&h_outer.ip) as i32;

    flags = BPF_F_ADJ_ROOM_FIXED_GSO | BPF_F_ADJ_ROOM_ENCAP_L3_IPV6;

    /* add room between mac and network header */
    if bpf_skb_adjust_room(skb, olen, BPF_ADJ_ROOM_MAC, flags) != 0 {
        return TC_ACT_SHOT;
    }

    /* prepare new outer network header */
    h_outer.ip.version = 6;
    h_outer.ip.hop_limit = iph_inner.ttl;
    h_outer.ip.saddr.in6_u.u6_addr8[1] = 0xfd;
    h_outer.ip.saddr.in6_u.u6_addr8[15] = 1;
    h_outer.ip.daddr.in6_u.u6_addr8[1] = 0xfd;
    h_outer.ip.daddr.in6_u.u6_addr8[15] = 2;
    h_outer.ip.payload_len = iph_inner.tot_len;
    h_outer.ip.nexthdr = IPPROTO_IPIP;

    /* store new outer network header */
    if bpf_skb_store_bytes(
        skb,
        ETH_HLEN as i32,
        &mut h_outer as *mut _ as *mut _,
        olen as u32,
        BPF_F_INVALIDATE_HASH,
    ) < 0
    {
        return TC_ACT_SHOT;
    }

    /* update eth->h_proto */
    if bpf_skb_load_bytes(
        skb,
        0,
        &mut eth as *mut _ as *mut _,
        core::mem::size_of_val(&eth) as u32,
    ) < 0
    {
        return TC_ACT_SHOT;
    }
    eth.h_proto = bpf_htons(ETH_P_IPV6);
    if bpf_skb_store_bytes(
        skb,
        0,
        &mut eth as *mut _ as *mut _,
        core::mem::size_of_val(&eth) as u32,
        0,
    ) < 0
    {
        return TC_ACT_SHOT;
    }

    TC_ACT_OK
}

#[inline(always)]
unsafe fn encap_ipv6(skb: *mut __sk_buff, encap_proto: __u8, l2_proto: __u16) -> i32 {
    __encap_ipv6(skb, encap_proto, l2_proto, 0)
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn __encap_ipip_none(skb: *mut __sk_buff) -> i32 {
    if (*skb).protocol == __bpf_constant_htons(ETH_P_IP) {
        encap_ipv4(skb, IPPROTO_IPIP, ETH_P_IP)
    } else {
        TC_ACT_OK
    }
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn __encap_gre_none(skb: *mut __sk_buff) -> i32 {
    if (*skb).protocol == __bpf_constant_htons(ETH_P_IP) {
        encap_ipv4(skb, IPPROTO_GRE, ETH_P_IP)
    } else {
        TC_ACT_OK
    }
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn __encap_gre_mpls(skb: *mut __sk_buff) -> i32 {
    if (*skb).protocol == __bpf_constant_htons(ETH_P_IP) {
        encap_ipv4(skb, IPPROTO_GRE, ETH_P_MPLS_UC)
    } else {
        TC_ACT_OK
    }
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn __encap_gre_eth(skb: *mut __sk_buff) -> i32 {
    if (*skb).protocol == __bpf_constant_htons(ETH_P_IP) {
        encap_ipv4(skb, IPPROTO_GRE, ETH_P_TEB)
    } else {
        TC_ACT_OK
    }
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn __encap_udp_none(skb: *mut __sk_buff) -> i32 {
    if (*skb).protocol == __bpf_constant_htons(ETH_P_IP) {
        encap_ipv4(skb, IPPROTO_UDP, ETH_P_IP)
    } else {
        TC_ACT_OK
    }
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn __encap_udp_mpls(skb: *mut __sk_buff) -> i32 {
    if (*skb).protocol == __bpf_constant_htons(ETH_P_IP) {
        encap_ipv4(skb, IPPROTO_UDP, ETH_P_MPLS_UC)
    } else {
        TC_ACT_OK
    }
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn __encap_udp_eth(skb: *mut __sk_buff) -> i32 {
    if (*skb).protocol == __bpf_constant_htons(ETH_P_IP) {
        encap_ipv4(skb, IPPROTO_UDP, ETH_P_TEB)
    } else {
        TC_ACT_OK
    }
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn __encap_vxlan_eth(skb: *mut __sk_buff) -> i32 {
    if (*skb).protocol == __bpf_constant_htons(ETH_P_IP) {
        __encap_ipv4(skb, IPPROTO_UDP, ETH_P_TEB, EXTPROTO_VXLAN)
    } else {
        TC_ACT_OK
    }
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn __encap_sit_none(skb: *mut __sk_buff) -> i32 {
    if (*skb).protocol == __bpf_constant_htons(ETH_P_IPV6) {
        encap_ipv4(skb, IPPROTO_IPV6, ETH_P_IP)
    } else {
        TC_ACT_OK
    }
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn __encap_ip6tnl_none(skb: *mut __sk_buff) -> i32 {
    if (*skb).protocol == __bpf_constant_htons(ETH_P_IPV6) {
        encap_ipv6(skb, IPPROTO_IPV6, ETH_P_IPV6)
    } else {
        TC_ACT_OK
    }
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn __encap_ipip6_none(skb: *mut __sk_buff) -> i32 {
    if (*skb).protocol == __bpf_constant_htons(ETH_P_IP) {
        encap_ipv6_ipip6(skb)
    } else {
        TC_ACT_OK
    }
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn __encap_ip6gre_none(skb: *mut __sk_buff) -> i32 {
    if (*skb).protocol == __bpf_constant_htons(ETH_P_IPV6) {
        encap_ipv6(skb, IPPROTO_GRE, ETH_P_IPV6)
    } else {
        TC_ACT_OK
    }
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn __encap_ip6gre_mpls(skb: *mut __sk_buff) -> i32 {
    if (*skb).protocol == __bpf_constant_htons(ETH_P_IPV6) {
        encap_ipv6(skb, IPPROTO_GRE, ETH_P_MPLS_UC)
    } else {
        TC_ACT_OK
    }
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn __encap_ip6gre_eth(skb: *mut __sk_buff) -> i32 {
    if (*skb).protocol == __bpf_constant_htons(ETH_P_IPV6) {
        encap_ipv6(skb, IPPROTO_GRE, ETH_P_TEB)
    } else {
        TC_ACT_OK
    }
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn __encap_ip6udp_none(skb: *mut __sk_buff) -> i32 {
    if (*skb).protocol == __bpf_constant_htons(ETH_P_IPV6) {
        encap_ipv6(skb, IPPROTO_UDP, ETH_P_IPV6)
    } else {
        TC_ACT_OK
    }
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn __encap_ip6udp_mpls(skb: *mut __sk_buff) -> i32 {
    if (*skb).protocol == __bpf_constant_htons(ETH_P_IPV6) {
        encap_ipv6(skb, IPPROTO_UDP, ETH_P_MPLS_UC)
    } else {
        TC_ACT_OK
    }
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn __encap_ip6udp_eth(skb: *mut __sk_buff) -> i32 {
    if (*skb).protocol == __bpf_constant_htons(ETH_P_IPV6) {
        encap_ipv6(skb, IPPROTO_UDP, ETH_P_TEB)
    } else {
        TC_ACT_OK
    }
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn __encap_ip6vxlan_eth(skb: *mut __sk_buff) -> i32 {
    if (*skb).protocol == __bpf_constant_htons(ETH_P_IPV6) {
        __encap_ipv6(skb, IPPROTO_UDP, ETH_P_TEB, EXTPROTO_VXLAN)
    } else {
        TC_ACT_OK
    }
}

unsafe fn decap_internal(
    skb: *mut __sk_buff,
    off: i32,
    len: i32,
    proto: i8,
    ipxip_flag: __u64,
) -> i32 {
    let mut flags: __u64 = BPF_F_ADJ_ROOM_FIXED_GSO;
    let mut kskb: *mut sk_buff;
    let mut shinfo: *mut skb_shared_info;
    let mut ip6_opt_hdr: ipv6_opt_hdr = core::mem::zeroed();
    let mut greh: gre_hdr = core::mem::zeroed();
    let mut udph: udphdr = core::mem::zeroed();
    let mut olen: i32 = len;

    match proto as i32 {
        IPPROTO_IPIP => {
            flags |= BPF_F_ADJ_ROOM_DECAP_L3_IPV4 | ipxip_flag;
        }
        IPPROTO_IPV6 => {
            flags |= BPF_F_ADJ_ROOM_DECAP_L3_IPV6 | ipxip_flag;
        }
        NEXTHDR_DEST => {
            if bpf_skb_load_bytes(
                skb,
                off + len,
                &mut ip6_opt_hdr as *mut _ as *mut _,
                core::mem::size_of_val(&ip6_opt_hdr) as u32,
            ) < 0
            {
                return TC_ACT_OK;
            }
            match ip6_opt_hdr.nexthdr as i32 {
                IPPROTO_IPIP => {
                    flags |= BPF_F_ADJ_ROOM_DECAP_L3_IPV4 | ipxip_flag;
                }
                IPPROTO_IPV6 => {
                    flags |= BPF_F_ADJ_ROOM_DECAP_L3_IPV6 | ipxip_flag;
                }
                _ => return TC_ACT_OK,
            }
        }
        IPPROTO_GRE => {
            olen += core::mem::size_of::<gre_hdr>() as i32;
            if !bpf_core_enum_value_exists(
                bpf_adj_room_flags::BPF_F_ADJ_ROOM_DECAP_L4_GRE,
            ) {
                return TC_ACT_SHOT;
            }
            flags |= BPF_F_ADJ_ROOM_DECAP_L4_GRE;

            if bpf_skb_load_bytes(
                skb,
                off + len,
                &mut greh as *mut _ as *mut _,
                core::mem::size_of_val(&greh) as u32,
            ) < 0
            {
                return TC_ACT_OK;
            }
            match bpf_ntohs(greh.protocol) {
                ETH_P_MPLS_UC => {
                    olen += core::mem::size_of_val(&mpls_label) as i32;
                }
                ETH_P_TEB => {
                    olen += ETH_HLEN as i32;
                }
                _ => {}
            }
        }
        IPPROTO_UDP => {
            olen += core::mem::size_of::<udphdr>() as i32;
            if !bpf_core_enum_value_exists(
                bpf_adj_room_flags::BPF_F_ADJ_ROOM_DECAP_L4_UDP,
            ) {
                return TC_ACT_SHOT;
            }
            flags |= BPF_F_ADJ_ROOM_DECAP_L4_UDP;
            if bpf_skb_load_bytes(
                skb,
                off + len,
                &mut udph as *mut _ as *mut _,
                core::mem::size_of_val(&udph) as u32,
            ) < 0
            {
                return TC_ACT_OK;
            }
            match bpf_ntohs(udph.dest) {
                MPLS_OVER_UDP_PORT => {
                    olen += core::mem::size_of_val(&mpls_label) as i32;
                }
                ETH_OVER_UDP_PORT => {
                    olen += ETH_HLEN as i32;
                }
                VXLAN_UDP_PORT => {
                    olen += ETH_HLEN as i32 + core::mem::size_of::<vxlanhdr___local>() as i32;
                }
                _ => {}
            }
        }
        _ => return TC_ACT_OK,
    }

    if bpf_skb_adjust_room(skb, -olen, BPF_ADJ_ROOM_MAC, flags) != 0 {
        return TC_ACT_SHOT;
    }

    kskb = bpf_cast_to_kern_ctx(skb);
    shinfo = bpf_core_cast((*kskb).head.add((*kskb).end as usize), skb_shared_info);
    if (*shinfo).gso_size != 0 {
        if (flags & BPF_F_ADJ_ROOM_DECAP_L4_UDP) != 0
            && ((*shinfo).gso_type & SKB_GSO_UDP_TUNNEL_MASK) != 0
        {
            return TC_ACT_SHOT;
        }

        if (flags & BPF_F_ADJ_ROOM_DECAP_L4_GRE) != 0
            && ((*shinfo).gso_type & (SKB_GSO_GRE | SKB_GSO_GRE_CSUM)) != 0
        {
            return TC_ACT_SHOT;
        }

        if (flags & BPF_F_ADJ_ROOM_DECAP_IPXIP4) != 0
            && ((*shinfo).gso_type & SKB_GSO_IPXIP4) != 0
        {
            return TC_ACT_SHOT;
        }

        if (flags & BPF_F_ADJ_ROOM_DECAP_IPXIP6) != 0
            && ((*shinfo).gso_type & SKB_GSO_IPXIP6) != 0
        {
            return TC_ACT_SHOT;
        }

        if (flags & (BPF_F_ADJ_ROOM_DECAP_L4_MASK | BPF_F_ADJ_ROOM_DECAP_IPXIP_MASK)) != 0 {
            if ((*shinfo).gso_type & SKB_GSO_TUNNEL_MASK) != 0 && !(*kskb).encapsulation {
                return TC_ACT_SHOT;
            }
            if ((*shinfo).gso_type & SKB_GSO_TUNNEL_MASK) == 0 && (*kskb).encapsulation {
                return TC_ACT_SHOT;
            }
        }
    } else if (flags & (BPF_F_ADJ_ROOM_DECAP_L4_MASK | BPF_F_ADJ_ROOM_DECAP_IPXIP_MASK)) != 0
        && (*kskb).encapsulation
    {
        return TC_ACT_SHOT;
    }

    TC_ACT_OK
}

unsafe fn decap_ipv4(skb: *mut __sk_buff) -> i32 {
    let mut iph_outer: iphdr = core::mem::zeroed();

    if !bpf_core_enum_value_exists(bpf_adj_room_flags::BPF_F_ADJ_ROOM_DECAP_IPXIP4) {
        return TC_ACT_SHOT;
    }

    if bpf_skb_load_bytes(
        skb,
        ETH_HLEN as i32,
        &mut iph_outer as *mut _ as *mut _,
        core::mem::size_of_val(&iph_outer) as u32,
    ) < 0
    {
        return TC_ACT_OK;
    }

    if iph_outer.ihl != 5 {
        return TC_ACT_OK;
    }

    decap_internal(
        skb,
        ETH_HLEN as i32,
        core::mem::size_of_val(&iph_outer) as i32,
        iph_outer.protocol as i8,
        BPF_F_ADJ_ROOM_DECAP_IPXIP4,
    )
}

unsafe fn decap_ipv6(skb: *mut __sk_buff) -> i32 {
    let mut iph_outer: ipv6hdr = core::mem::zeroed();

    if !bpf_core_enum_value_exists(bpf_adj_room_flags::BPF_F_ADJ_ROOM_DECAP_IPXIP6) {
        return TC_ACT_SHOT;
    }

    if bpf_skb_load_bytes(
        skb,
        ETH_HLEN as i32,
        &mut iph_outer as *mut _ as *mut _,
        core::mem::size_of_val(&iph_outer) as u32,
    ) < 0
    {
        return TC_ACT_OK;
    }

    decap_internal(
        skb,
        ETH_HLEN as i32,
        core::mem::size_of_val(&iph_outer) as i32,
        iph_outer.nexthdr as i8,
        BPF_F_ADJ_ROOM_DECAP_IPXIP6,
    )
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn decap_f(skb: *mut __sk_buff) -> i32 {
    match (*skb).protocol {
        x if x == __bpf_constant_htons(ETH_P_IP) => decap_ipv4(skb),
        x if x == __bpf_constant_htons(ETH_P_IPV6) => decap_ipv6(skb),
        _ => {
            /* does not match, ignore */
            TC_ACT_OK
        }
    }
}

#[no_mangle]
#[link_section = "license"]
pub static mut __license: [u8; 4] = *b"GPL\0";
