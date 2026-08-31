/* SPDX-License-Identifier: GPL-2.0
 *  Copyright(c) 2018 Jesper Dangaard Brouer.
 *
 * XDP/TC VLAN manipulation example
 *
 * GOTCHA: Remember to disable NIC hardware offloading of VLANs,
 * else the VLAN tags are NOT inlined in the packet payload:
 *
 *  # ethtool -K ixgbe2 rxvlan off
 *
 * Verify setting:
 *  # ethtool -k ixgbe2 | grep rx-vlan-offload
 *  rx-vlan-offload: off
 *
 */
/* Dependencies from the original C includes:
 * <stddef.h>, <stdbool.h>, <string.h>, <linux/bpf.h>, <linux/if_ether.h>,
 * <linux/if_vlan.h>, <linux/in.h>, <linux/pkt_cls.h>,
 * <bpf/bpf_helpers.h>, <bpf/bpf_endian.h>
 */

/* linux/if_vlan.h have not exposed this as UAPI, thus mirror some here
 *
 *	struct vlan_hdr - vlan header
 *	@h_vlan_TCI: priority and VLAN ID
 *	@h_vlan_encapsulated_proto: packet type ID or len
 */
#[repr(C)]
pub struct _vlan_hdr {
    pub h_vlan_TCI: __be16,
    pub h_vlan_encapsulated_proto: __be16,
}

pub const VLAN_PRIO_MASK: u32 = 0xe000; /* Priority Code Point */
pub const VLAN_PRIO_SHIFT: u32 = 13;
pub const VLAN_CFI_MASK: u32 = 0x1000; /* Canonical Format Indicator */
pub const VLAN_TAG_PRESENT: u32 = VLAN_CFI_MASK;
pub const VLAN_VID_MASK: u32 = 0x0fff; /* VLAN Identifier */
pub const VLAN_N_VID: u32 = 4096;

#[repr(C)]
pub struct parse_pkt {
    pub l3_proto: __u16,
    pub l3_offset: __u16,
    pub vlan_outer: __u16,
    pub vlan_inner: __u16,
    pub vlan_outer_offset: __u8,
    pub vlan_inner_offset: __u8,
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [::core::ffi::c_char; 4] = [b'G' as _, b'P' as _, b'L' as _, 0];

#[inline(always)]
unsafe fn parse_eth_frame(eth: *mut ethhdr, data_end: *mut ::core::ffi::c_void, pkt: *mut parse_pkt) -> bool {
    let mut eth_type: __u16;
    let mut offset: __u8;

    offset = ::core::mem::size_of::<ethhdr>() as __u8;
    /* Make sure packet is large enough for parsing eth + 2 VLAN headers */
    if (eth as *mut ::core::ffi::c_void)
        .add((offset as usize) + (2 * ::core::mem::size_of::<_vlan_hdr>()))
        > data_end
    {
        return false;
    }

    eth_type = (*eth).h_proto;

    /* Handle outer VLAN tag */
    if eth_type == bpf_htons(ETH_P_8021Q as __u16) || eth_type == bpf_htons(ETH_P_8021AD as __u16) {
        let mut vlan_hdr: *mut _vlan_hdr;

        vlan_hdr = (eth as *mut ::core::ffi::c_void).add(offset as usize) as *mut _vlan_hdr;
        (*pkt).vlan_outer_offset = offset;
        (*pkt).vlan_outer = bpf_ntohs((*vlan_hdr).h_vlan_TCI) & (VLAN_VID_MASK as __u16);
        eth_type = (*vlan_hdr).h_vlan_encapsulated_proto;
        offset = offset.wrapping_add(::core::mem::size_of::<_vlan_hdr>() as __u8);
    }

    /* Handle inner (double) VLAN tag */
    if eth_type == bpf_htons(ETH_P_8021Q as __u16) || eth_type == bpf_htons(ETH_P_8021AD as __u16) {
        let mut vlan_hdr: *mut _vlan_hdr;

        vlan_hdr = (eth as *mut ::core::ffi::c_void).add(offset as usize) as *mut _vlan_hdr;
        (*pkt).vlan_inner_offset = offset;
        (*pkt).vlan_inner = bpf_ntohs((*vlan_hdr).h_vlan_TCI) & (VLAN_VID_MASK as __u16);
        eth_type = (*vlan_hdr).h_vlan_encapsulated_proto;
        offset = offset.wrapping_add(::core::mem::size_of::<_vlan_hdr>() as __u8);
    }

    (*pkt).l3_proto = bpf_ntohs(eth_type); /* Convert to host-byte-order */
    (*pkt).l3_offset = offset as __u16;

    true
}

/* Hint, VLANs are chosen to hit network-byte-order issues */
pub const TESTVLAN: u32 = 4011; /* 0xFAB */
// #define TO_VLAN  4000 /* 0xFA0 (hint 0xOA0 = 160) */

#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xdp_drop_vlan_4011(ctx: *mut xdp_md) -> ::core::ffi::c_int {
    let data_end: *mut ::core::ffi::c_void = (*ctx).data_end as usize as *mut ::core::ffi::c_void;
    let data: *mut ::core::ffi::c_void = (*ctx).data as usize as *mut ::core::ffi::c_void;
    let mut pkt: parse_pkt = ::core::mem::zeroed();

    if !parse_eth_frame(data as *mut ethhdr, data_end, &mut pkt) {
        return XDP_ABORTED;
    }

    /* Drop specific VLAN ID example */
    if pkt.vlan_outer as u32 == TESTVLAN {
        return XDP_ABORTED;
    }
    /*
     * Using XDP_ABORTED makes it possible to record this event,
     * via tracepoint xdp:xdp_exception like:
     *  # perf record -a -e xdp:xdp_exception
     *  # perf script
     */
    XDP_PASS
}
/*
Commands to setup VLAN on Linux to test packets gets dropped:

 export ROOTDEV=ixgbe2
 export VLANID=4011
 ip link add link $ROOTDEV name $ROOTDEV.$VLANID type vlan id $VLANID
 ip link set dev  $ROOTDEV.$VLANID up

 ip link set dev $ROOTDEV mtu 1508
 ip addr add 100.64.40.11/24 dev $ROOTDEV.$VLANID

Load prog with ip tool:

 ip link set $ROOTDEV xdp off
 ip link set $ROOTDEV xdp object xdp_vlan01_kern.o section xdp_drop_vlan_4011

*/

/* Changing VLAN to zero, have same practical effect as removing the VLAN. */
pub const TO_VLAN: u32 = 0;

#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xdp_vlan_change(ctx: *mut xdp_md) -> ::core::ffi::c_int {
    let data_end: *mut ::core::ffi::c_void = (*ctx).data_end as usize as *mut ::core::ffi::c_void;
    let data: *mut ::core::ffi::c_void = (*ctx).data as usize as *mut ::core::ffi::c_void;
    let mut pkt: parse_pkt = ::core::mem::zeroed();

    if !parse_eth_frame(data as *mut ethhdr, data_end, &mut pkt) {
        return XDP_ABORTED;
    }

    /* Change specific VLAN ID */
    if pkt.vlan_outer as u32 == TESTVLAN {
        let vlan_hdr: *mut _vlan_hdr = data.add(pkt.vlan_outer_offset as usize) as *mut _vlan_hdr;

        /* Modifying VLAN, preserve top 4 bits */
        (*vlan_hdr).h_vlan_TCI =
            bpf_htons((bpf_ntohs((*vlan_hdr).h_vlan_TCI) & 0xf000u16) | (TO_VLAN as __u16));
    }

    XDP_PASS
}

/*
 * Show XDP+TC can cooperate, on creating a VLAN rewriter.
 * 1. Create a XDP prog that can "pop"/remove a VLAN header.
 * 2. Create a TC-bpf prog that egress can add a VLAN header.
 */

/* Original C condition:
 * #ifndef ETH_ALEN
 * #define ETH_ALEN	6	/* bytes */
 * #endif
 */
pub const ETH_ALEN: u32 = 6; /* bytes */
pub const VLAN_HDR_SZ: u32 = 4; /* bytes */

#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xdp_vlan_remove_outer(ctx: *mut xdp_md) -> ::core::ffi::c_int {
    let data_end: *mut ::core::ffi::c_void = (*ctx).data_end as usize as *mut ::core::ffi::c_void;
    let data: *mut ::core::ffi::c_void = (*ctx).data as usize as *mut ::core::ffi::c_void;
    let mut pkt: parse_pkt = ::core::mem::zeroed();
    let mut dest: *mut ::core::ffi::c_char;

    if !parse_eth_frame(data as *mut ethhdr, data_end, &mut pkt) {
        return XDP_ABORTED;
    }

    /* Skip packet if no outer VLAN was detected */
    if pkt.vlan_outer_offset == 0 {
        return XDP_PASS;
    }

    /* Moving Ethernet header, dest overlap with src, memmove handle this */
    dest = data as *mut ::core::ffi::c_char;
    dest = dest.add(VLAN_HDR_SZ as usize);
    /*
     * Notice: Taking over vlan_hdr->h_vlan_encapsulated_proto, by
     * only moving two MAC addrs (12 bytes), not overwriting last 2 bytes
     */
    __builtin_memmove(dest as *mut ::core::ffi::c_void, data, (ETH_ALEN * 2) as usize);
    /* Note: LLVM built-in memmove inlining require size to be constant */

    /* Move start of packet header seen by Linux kernel stack */
    bpf_xdp_adjust_head(ctx, VLAN_HDR_SZ as ::core::ffi::c_int);

    XDP_PASS
}

#[inline(always)]
unsafe fn shift_mac_4bytes_32bit(data: *mut ::core::ffi::c_void) {
    let p: *mut __u32 = data as *mut __u32;

    /* Assuming VLAN hdr present. The 4 bytes in p[3] that gets
     * overwritten, is ethhdr->h_proto and vlan_hdr->h_vlan_TCI.
     * The vlan_hdr->h_vlan_encapsulated_proto take over role as
     * ethhdr->h_proto.
     */
    *p.add(3) = *p.add(2);
    *p.add(2) = *p.add(1);
    *p.add(1) = *p.add(0);
}

#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xdp_vlan_remove_outer2(ctx: *mut xdp_md) -> ::core::ffi::c_int {
    let data_end: *mut ::core::ffi::c_void = (*ctx).data_end as usize as *mut ::core::ffi::c_void;
    let data: *mut ::core::ffi::c_void = (*ctx).data as usize as *mut ::core::ffi::c_void;
    let orig_eth: *mut ethhdr = data as *mut ethhdr;
    let mut pkt: parse_pkt = ::core::mem::zeroed();

    if !parse_eth_frame(orig_eth, data_end, &mut pkt) {
        return XDP_ABORTED;
    }

    /* Skip packet if no outer VLAN was detected */
    if pkt.vlan_outer_offset == 0 {
        return XDP_PASS;
    }

    /* Simply shift down MAC addrs 4 bytes, overwrite h_proto + TCI */
    shift_mac_4bytes_32bit(data);

    /* Move start of packet header seen by Linux kernel stack */
    bpf_xdp_adjust_head(ctx, VLAN_HDR_SZ as ::core::ffi::c_int);

    XDP_PASS
}

/*=====================================
 *  BELOW: TC-hook based ebpf programs
 * ====================================
 * The TC-clsact eBPF programs (currently) need to be attach via TC commands
 */

#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tc_vlan_push(ctx: *mut __sk_buff) -> ::core::ffi::c_int {
    bpf_skb_vlan_push(ctx, bpf_htons(ETH_P_8021Q as __u16), TESTVLAN as __u16);

    TC_ACT_OK
}
/*
Commands to setup TC to use above bpf prog:

export ROOTDEV=ixgbe2
export FILE=xdp_vlan01_kern.o

# Re-attach clsact to clear/flush existing role
tc qdisc del dev $ROOTDEV clsact 2> /dev/null ;\
tc qdisc add dev $ROOTDEV clsact

# Attach BPF prog EGRESS
tc filter add dev $ROOTDEV egress \
  prio 1 handle 1 bpf da obj $FILE sec tc_vlan_push

tc filter show dev $ROOTDEV egress
*/
