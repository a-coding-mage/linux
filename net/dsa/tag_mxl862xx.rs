// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * DSA Special Tag for MaxLinear 862xx switch chips
 *
 * Copyright (C) 2025 Daniel Golle <daniel@makrotopia.org>
 * Copyright (C) 2024 MaxLinear Inc.
 */

// Dependencies supplied by the surrounding kernel/DSA implementation.

const MXL862_NAME: &str = "mxl862xx";
const MXL862_HEADER_LEN: usize = 8;

/* Word 0 -> EtherType */

/* Word 2 */
const MXL862_SUBIF_ID: u16 = 0x1f;

/* Word 3 */
const MXL862_IGP_EGP: u16 = 0x0f;

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

#[repr(C)]
struct dsa_device_ops {
    name: *const u8,
    proto: u32,
    xmit: Option<unsafe extern "C" fn(*mut sk_buff, *mut net_device) -> *mut sk_buff>,
    rcv: Option<unsafe extern "C" fn(*mut sk_buff, *mut net_device) -> *mut sk_buff>,
    needed_headroom: usize,
}

unsafe extern "C" {
    fn dsa_user_to_port(dev: *mut net_device) -> *mut dsa_port;
    fn dsa_alloc_etype_header(skb: *mut sk_buff, len: usize);
    fn dsa_etype_header_pos_tx(skb: *mut sk_buff) -> *mut u16;
    fn dsa_etype_header_pos_rx(skb: *mut sk_buff) -> *mut u16;
    fn skb_push(skb: *mut sk_buff, len: usize) -> *mut u8;
    fn pskb_may_pull(skb: *mut sk_buff, len: usize) -> bool;
    fn kfree_skb(skb: *mut sk_buff);
    fn dsa_conduit_find_user(dev: *mut net_device, conduit: i32, port: i32) -> *mut net_device;
    fn is_link_local_ether_addr(addr: *const u8) -> bool;
    fn eth_hdr(skb: *mut sk_buff) -> *mut ethhdr;
    fn dsa_default_offload_fwd_mark(skb: *mut sk_buff);
    fn skb_pull_rcsum(skb: *mut sk_buff, len: usize) -> *mut u8;
    fn dsa_strip_etype_header(skb: *mut sk_buff, len: usize);
    fn htons(value: u16) -> u16;
    fn ntohs(value: u16) -> u16;
}

#[repr(C)]
struct dsa_port {
    _private: [u8; 0],
}

#[repr(C)]
struct ethhdr {
    h_dest: [u8; 6],
    _rest: [u8; 0],
}

const ETH_P_MXLGSW: u16 = 0;
const DSA_TAG_PROTO_MXL862: u32 = 0;

#[inline]
unsafe fn field_prep(mask: u16, value: u16) -> u16 {
    (value & mask) // MXL862 fields start at bit zero.
}

unsafe extern "C" fn mxl862_tag_xmit(
    skb: *mut sk_buff,
    dev: *mut net_device,
) -> *mut sk_buff {
    let dp = dsa_user_to_port(dev);
    let cpu_dp = *(dp as *mut *mut dsa_port).add(1);
    let cpu_port = *(cpu_dp as *mut u32);
    let dp_index = *(dp as *mut u32);
    let sub_interface = dp_index.wrapping_add(16).wrapping_sub(cpu_port);

    /* provide additional space 'MXL862_HEADER_LEN' bytes */
    skb_push(skb, MXL862_HEADER_LEN);

    /* shift MAC address to the beginning of the enlarged buffer,
     * releasing the space required for DSA tag (between MAC address and
     * Ethertype)
     */
    dsa_alloc_etype_header(skb, MXL862_HEADER_LEN);

    /* special tag ingress (from the perspective of the switch) */
    let mxl862_tag = dsa_etype_header_pos_tx(skb);
    *mxl862_tag.add(0) = htons(ETH_P_MXLGSW);
    *mxl862_tag.add(1) = 0;
    *mxl862_tag.add(2) = htons(field_prep(MXL862_SUBIF_ID, sub_interface as u16));
    *mxl862_tag.add(3) = htons(field_prep(MXL862_IGP_EGP, cpu_port as u16));

    skb
}

unsafe extern "C" fn mxl862_tag_rcv(
    skb: *mut sk_buff,
    dev: *mut net_device,
) -> *mut sk_buff {
    if !pskb_may_pull(skb, MXL862_HEADER_LEN) {
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    let mxl862_tag = dsa_etype_header_pos_rx(skb);
    if *mxl862_tag.add(0) != htons(ETH_P_MXLGSW) {
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    /* Get source port information */
    let port = (ntohs(*mxl862_tag.add(3)) & MXL862_IGP_EGP) as i32;
    let skb_dev = dsa_conduit_find_user(dev, 0, port);
    *(skb as *mut *mut net_device).add(0) = skb_dev;
    if skb_dev.is_null() {
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    let hdr = eth_hdr(skb);
    if !is_link_local_ether_addr((*hdr).h_dest.as_ptr()) {
        dsa_default_offload_fwd_mark(skb);
    }

    /* remove the MxL862xx special tag between the MAC addresses and the
     * current ethertype field.
     */
    skb_pull_rcsum(skb, MXL862_HEADER_LEN);
    dsa_strip_etype_header(skb, MXL862_HEADER_LEN);

    skb
}

static mxl862_netdev_ops: dsa_device_ops = dsa_device_ops {
    name: MXL862_NAME.as_ptr(),
    proto: DSA_TAG_PROTO_MXL862,
    xmit: Some(mxl862_tag_xmit),
    rcv: Some(mxl862_tag_rcv),
    needed_headroom: MXL862_HEADER_LEN,
};

// MODULE_ALIAS_DSA_TAG_DRIVER(DSA_TAG_PROTO_MXL862, MXL862_NAME);
// MODULE_DESCRIPTION("DSA tag driver for MaxLinear MxL862xx switches");
// MODULE_LICENSE("GPL");
// module_dsa_tag_driver(mxl862_netdev_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
