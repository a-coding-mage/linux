// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2026, Joris Vaisvila <joey@tinyisr.com>
 * MT7628 switch tag support
 */

// The Linux kernel headers and "tag.h" provide the types and symbols used
// below.

/*
 * The MT7628 tag is encoded in the VLAN TPID field.
 * On TX the lower 6 bits encode the destination port bitmask.
 * On RX the lower 3 bits encode the source port number.
 *
 * The switch hardware will not modify the TPID of an incoming packet if it is
 * already VLAN tagged. To work around this the switch is configured to always
 * append a tag_8021q standalone VLAN tag for each port. That means we can
 * safely strip the outer VLAN tag after parsing it.
 *
 * A VLAN tag is constructed on egress to target the standalone VLAN and
 * destination port.
 */

const MT7628_TAG_NAME: &str = "mt7628";

const MT7628_TAG_TX_PORT: u16 = ((1u16 << 6) - 1);
const MT7628_TAG_RX_PORT: u16 = ((1u16 << 3) - 1);
const MT7628_TAG_LEN: usize = 4;

unsafe fn mt7628_tag_xmit(
    skb: *mut sk_buff,
    dev: *mut net_device,
) -> *mut sk_buff {
    let dp: *mut dsa_port;
    let mut xmit_vlan: u16;
    let tag: *mut __be16;

    dp = dsa_user_to_port(dev);
    xmit_vlan = dsa_tag_8021q_standalone_vid(dp);

    skb_push(skb, MT7628_TAG_LEN);
    dsa_alloc_etype_header(skb, MT7628_TAG_LEN);

    tag = dsa_etype_header_pos_tx(skb);

    *tag.add(0) = htons(
        ETH_P_8021Q |
            field_prep(MT7628_TAG_TX_PORT, dsa_xmit_port_mask(skb, dev)),
    );
    *tag.add(1) = htons(xmit_vlan);

    skb
}

unsafe fn mt7628_tag_rcv(
    skb: *mut sk_buff,
    dev: *mut net_device,
) -> *mut sk_buff {
    let phdr: *mut __be16;

    if unlikely(!pskb_may_pull(skb, MT7628_TAG_LEN)) {
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    phdr = dsa_etype_header_pos_rx(skb);
    (*skb).dev = dsa_conduit_find_user(
        dev,
        0,
        field_get(MT7628_TAG_RX_PORT, ntohs(*phdr)),
    );
    if (*skb).dev.is_null() {
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    skb_pull_rcsum(skb, MT7628_TAG_LEN);
    dsa_strip_etype_header(skb, MT7628_TAG_LEN);
    dsa_default_offload_fwd_mark(skb);
    skb
}

static MT7628_TAG_OPS: dsa_device_ops = dsa_device_ops {
    name: MT7628_TAG_NAME,
    proto: DSA_TAG_PROTO_MT7628,
    xmit: Some(mt7628_tag_xmit),
    rcv: Some(mt7628_tag_rcv),
    needed_headroom: MT7628_TAG_LEN,
};

module_dsa_tag_driver!(MT7628_TAG_OPS);

module_alias_dsa_tag_driver!(DSA_TAG_PROTO_MT7628, MT7628_TAG_NAME);
module_description!("DSA tag driver for MT7628 switch");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
