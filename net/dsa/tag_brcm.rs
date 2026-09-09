// SPDX-License-Identifier: GPL-2.0+
/*
 * Broadcom tag support
 *
 * Copyright (C) 2014 Broadcom Corporation
 */

// C kernel includes and build-time CONFIG_NET_DSA_TAG_BRCM conditions are
// supplied by the surrounding kernel translation.

const BRCM_NAME: &str = "brcm";
const BRCM_LEGACY_NAME: &str = "brcm-legacy";
const BRCM_LEGACY_FCS_NAME: &str = "brcm-legacy-fcs";
const BRCM_PREPEND_NAME: &str = "brcm-prepend";

const BRCM_LEG_TAG_LEN: usize = 6;
const BRCM_LEG_TYPE_HI: u8 = 0x88;
const BRCM_LEG_TYPE_LO: u8 = 0x74;
const BRCM_LEG_UNICAST: u8 = 0 << 5;
const BRCM_LEG_MULTICAST: u8 = 1 << 5;
const BRCM_LEG_EGRESS: u8 = 2 << 5;
const BRCM_LEG_INGRESS: u8 = 3 << 5;
const BRCM_LEG_PORT_ID: u8 = 0xf;
const BRCM_TAG_LEN: usize = 4;
const BRCM_OPCODE_SHIFT: u32 = 5;
const BRCM_OPCODE_MASK: u8 = 0x7;
const BRCM_IG_TC_SHIFT: u32 = 2;
const BRCM_IG_TC_MASK: u16 = 0x7;
const BRCM_IG_TE_MASK: u8 = 0x3;
const BRCM_IG_TS_SHIFT: u32 = 7;
const BRCM_IG_DSTMAP2_MASK: u16 = 1;
const BRCM_IG_DSTMAP1_MASK: u16 = 0xff;
const BRCM_EG_CID_MASK: u8 = 0xff;
const BRCM_EG_RC_MASK: u8 = 0xff;
const BRCM_EG_RC_RSVD: u8 = 3 << 6;
const BRCM_EG_RC_EXCEPTION: u8 = 1 << 5;
const BRCM_EG_RC_PROT_SNOOP: u8 = 1 << 4;
const BRCM_EG_RC_PROT_TERM: u8 = 1 << 3;
const BRCM_EG_RC_SWITCH: u8 = 1 << 2;
const BRCM_EG_RC_MAC_LEARN: u8 = 1 << 1;
const BRCM_EG_RC_MIRROR: u8 = 1 << 0;
const BRCM_EG_TC_SHIFT: u32 = 5;
const BRCM_EG_TC_MASK: u8 = 0x7;
const BRCM_EG_PID_MASK: u8 = 0x1f;

#[cfg(any(feature = "CONFIG_NET_DSA_TAG_BRCM", feature = "CONFIG_NET_DSA_TAG_BRCM_PREPEND"))]
unsafe fn brcm_tag_xmit_ll(mut skb: *mut sk_buff, dev: *mut net_device, offset: usize) -> *mut sk_buff {
    let dp = dsa_user_to_port(dev);
    let queue: u16 = skb_get_queue_mapping(skb);
    let port_mask: u16;
    let brcm_tag: *mut u8;

    if skb_put_padto(skb, ETH_ZLEN + BRCM_TAG_LEN) != 0 { return core::ptr::null_mut(); }
    skb_push(skb, BRCM_TAG_LEN);
    if offset != 0 { dsa_alloc_etype_header(skb, BRCM_TAG_LEN); }
    brcm_tag = (*skb).data.add(offset);
    *brcm_tag.add(0) = (1 << BRCM_OPCODE_SHIFT) as u8 | (((queue & BRCM_IG_TC_MASK) as u8) << BRCM_IG_TC_SHIFT);
    *brcm_tag.add(1) = 0;
    port_mask = dsa_xmit_port_mask(skb, dev);
    *brcm_tag.add(2) = ((port_mask >> 8) & BRCM_IG_DSTMAP2_MASK) as u8;
    *brcm_tag.add(3) = (port_mask & BRCM_IG_DSTMAP1_MASK) as u8;
    skb_set_queue_mapping(skb, BRCM_TAG_SET_PORT_QUEUE((*dp).index, queue));
    skb
}

#[cfg(any(feature = "CONFIG_NET_DSA_TAG_BRCM", feature = "CONFIG_NET_DSA_TAG_BRCM_PREPEND"))]
unsafe fn brcm_tag_rcv_ll(skb: *mut sk_buff, dev: *mut net_device, offset: usize) -> *mut sk_buff {
    let source_port: i32;
    let brcm_tag: *mut u8;
    if unlikely(pskb_may_pull(skb, BRCM_TAG_LEN) == 0) { kfree_skb(skb); return core::ptr::null_mut(); }
    brcm_tag = (*skb).data.sub(offset);
    if unlikely(((*brcm_tag >> BRCM_OPCODE_SHIFT) & BRCM_OPCODE_MASK) != 0) { kfree_skb(skb); return core::ptr::null_mut(); }
    if unlikely((*brcm_tag.add(2) & BRCM_EG_RC_RSVD) != 0) { kfree_skb(skb); return core::ptr::null_mut(); }
    source_port = (*brcm_tag.add(3) & BRCM_EG_PID_MASK) as i32;
    (*skb).dev = dsa_conduit_find_user(dev, 0, source_port);
    if (*skb).dev.is_null() { kfree_skb(skb); return core::ptr::null_mut(); }
    skb_pull_rcsum(skb, BRCM_TAG_LEN);
    if likely(!is_link_local_ether_addr((*eth_hdr(skb)).h_dest.as_ptr())) { dsa_default_offload_fwd_mark(skb); }
    skb
}

#[cfg(feature = "CONFIG_NET_DSA_TAG_BRCM")]
unsafe fn brcm_tag_xmit(skb: *mut sk_buff, dev: *mut net_device) -> *mut sk_buff { brcm_tag_xmit_ll(skb, dev, 2 * ETH_ALEN) }

#[cfg(feature = "CONFIG_NET_DSA_TAG_BRCM")]
unsafe fn brcm_tag_rcv(skb: *mut sk_buff, dev: *mut net_device) -> *mut sk_buff {
    let nskb = brcm_tag_rcv_ll(skb, dev, 2);
    if nskb.is_null() { return nskb; }
    dsa_strip_etype_header(skb, BRCM_TAG_LEN);
    nskb
}

// The following driver registrations preserve the C driver's interfaces;
// their concrete kernel types and registration macros are external.
#[cfg(feature = "CONFIG_NET_DSA_TAG_BRCM")]
static brcm_netdev_ops: dsa_device_ops = dsa_device_ops { name: BRCM_NAME, proto: DSA_TAG_PROTO_BRCM, xmit: brcm_tag_xmit, rcv: brcm_tag_rcv, needed_headroom: BRCM_TAG_LEN };

#[cfg(any(feature = "CONFIG_NET_DSA_TAG_BRCM_LEGACY", feature = "CONFIG_NET_DSA_TAG_BRCM_LEGACY_FCS"))]
unsafe fn brcm_leg_tag_rcv(mut skb: *mut sk_buff, dev: *mut net_device) -> *mut sk_buff {
    let mut len = BRCM_LEG_TAG_LEN;
    if unlikely(pskb_may_pull(skb, BRCM_LEG_TAG_LEN + VLAN_HLEN) == 0) { kfree_skb(skb); return core::ptr::null_mut(); }
    let brcm_tag = dsa_etype_header_pos_rx(skb);
    let proto = (brcm_tag.add(BRCM_LEG_TAG_LEN)) as *mut __be16;
    let source_port = (*brcm_tag.add(5) & BRCM_LEG_PORT_ID) as i32;
    (*skb).dev = dsa_conduit_find_user(dev, 0, source_port);
    if (*skb).dev.is_null() { kfree_skb(skb); return core::ptr::null_mut(); }
    if *proto == htons(ETH_P_8021Q) && *proto.add(1) == 0 { len += VLAN_HLEN; }
    skb_pull_rcsum(skb, len);
    if likely(!is_link_local_ether_addr((*eth_hdr(skb)).h_dest.as_ptr())) { dsa_default_offload_fwd_mark(skb); }
    dsa_strip_etype_header(skb, len);
    skb
}

#[cfg(feature = "CONFIG_NET_DSA_TAG_BRCM_LEGACY")]
unsafe fn brcm_leg_tag_xmit(skb: *mut sk_buff, dev: *mut net_device) -> *mut sk_buff {
    let dp = dsa_user_to_port(dev);
    if skb_put_padto(skb, ETH_ZLEN + BRCM_LEG_TAG_LEN) != 0 { return core::ptr::null_mut(); }
    skb_push(skb, BRCM_LEG_TAG_LEN); dsa_alloc_etype_header(skb, BRCM_LEG_TAG_LEN);
    let tag = (*skb).data.add(2 * ETH_ALEN);
    *tag.add(0) = BRCM_LEG_TYPE_HI; *tag.add(1) = BRCM_LEG_TYPE_LO; *tag.add(2) = BRCM_LEG_EGRESS;
    *tag.add(3) = 0; *tag.add(4) = 0; *tag.add(5) = ((*dp).index as u8) & BRCM_LEG_PORT_ID; skb
}

#[cfg(feature = "CONFIG_NET_DSA_TAG_BRCM_LEGACY_FCS")]
unsafe fn brcm_leg_fcs_tag_xmit(skb: *mut sk_buff, dev: *mut net_device) -> *mut sk_buff {
    let dp = dsa_user_to_port(dev);
    if skb_put_padto(skb, ETH_ZLEN + BRCM_LEG_TAG_LEN) != 0 { return core::ptr::null_mut(); }
    let fcs_len = (*skb).len;
    let fcs_val: __le32 = cpu_to_le32(crc32_le(!0, (*skb).data, fcs_len) ^ !0);
    skb_push(skb, BRCM_LEG_TAG_LEN); dsa_alloc_etype_header(skb, BRCM_LEG_TAG_LEN);
    let tag = (*skb).data.add(2 * ETH_ALEN);
    *tag.add(0) = BRCM_LEG_TYPE_HI; *tag.add(1) = BRCM_LEG_TYPE_LO;
    *tag.add(2) = BRCM_LEG_EGRESS | (((fcs_len >> 8) & 0x7) as u8);
    *tag.add(3) = (fcs_len & 0xff) as u8; *tag.add(4) = 0;
    *tag.add(5) = ((*dp).index as u8) & BRCM_LEG_PORT_ID;
    if skb_pad(skb, ETH_FCS_LEN) != 0 { return core::ptr::null_mut(); }
    skb_put_data(skb, &fcs_val as *const __le32 as *const core::ffi::c_void, ETH_FCS_LEN); skb
}

#[cfg(feature = "CONFIG_NET_DSA_TAG_BRCM_PREPEND")]
unsafe fn brcm_tag_xmit_prepend(skb: *mut sk_buff, dev: *mut net_device) -> *mut sk_buff { brcm_tag_xmit_ll(skb, dev, 0) }
#[cfg(feature = "CONFIG_NET_DSA_TAG_BRCM_PREPEND")]
unsafe fn brcm_tag_rcv_prepend(skb: *mut sk_buff, dev: *mut net_device) -> *mut sk_buff { brcm_tag_rcv_ll(skb, dev, ETH_HLEN) }

// CONFIG_NET_DSA_TAG_BRCM_LEGACY_FCS transmit and all DSA_TAG_DRIVER,
// MODULE_ALIAS_DSA_TAG_DRIVER, module_dsa_tag_drivers, and module metadata
// declarations remain external kernel registration constructs.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
