// SPDX-License-Identifier: GPL-2.0
/*
 * Mediatek DSA Tag support
 * Copyright (C) 2017 Landen Chao <landen.chao@mediatek.com>
 *                     Sean Wang <sean.wang@mediatek.com>
 */

// Dependencies supplied by the surrounding kernel/DSA implementation.

const MTK_NAME: &str = "mtk";

const MTK_HDR_LEN: usize = 4;
const MTK_HDR_XMIT_UNTAGGED: u8 = 0;
const MTK_HDR_XMIT_TAGGED_TPID_8100: u8 = 1;
const MTK_HDR_XMIT_TAGGED_TPID_88A8: u8 = 2;
const MTK_HDR_RECV_SOURCE_PORT_MASK: u16 = (1u16 << (2 + 1)) - 1;
const MTK_HDR_XMIT_DP_BIT_MASK: u8 = (1u8 << (5 + 1)) - 1;
const MTK_HDR_XMIT_SA_DIS: u8 = 1u8 << 6;

unsafe fn mtk_tag_xmit(skb: *mut sk_buff, dev: *mut net_device) -> *mut sk_buff {
    let dp = dsa_user_to_port(dev);
    let mut xmit_tpid: u8;
    let mut mtk_tag: *mut u8;

    skb_set_queue_mapping(skb, (*dp).index);

    /* Build the special tag after the MAC Source Address. If VLAN header
     * is present, it's required that VLAN header and special tag is
     * being combined. Only in this way we can allow the switch can parse
     * the both special and VLAN tag at the same time and then look up VLAN
     * table with VID.
     */
    match (*skb).protocol {
        p if p == htons(ETH_P_8021Q) => {
            xmit_tpid = MTK_HDR_XMIT_TAGGED_TPID_8100;
        }
        p if p == htons(ETH_P_8021AD) => {
            xmit_tpid = MTK_HDR_XMIT_TAGGED_TPID_88A8;
        }
        _ => {
            xmit_tpid = MTK_HDR_XMIT_UNTAGGED;
            skb_push(skb, MTK_HDR_LEN);
            dsa_alloc_etype_header(skb, MTK_HDR_LEN);
        }
    }

    mtk_tag = dsa_etype_header_pos_tx(skb);

    /* Mark tag attribute on special tag insertion to notify hardware
     * whether that's a combined special tag with 802.1Q header.
     */
    *mtk_tag.add(0) = xmit_tpid;
    *mtk_tag.add(1) = field_prep(MTK_HDR_XMIT_DP_BIT_MASK,
                                 dsa_xmit_port_mask(skb, dev));

    /* Tag control information is kept for 802.1Q */
    if xmit_tpid == MTK_HDR_XMIT_UNTAGGED {
        *mtk_tag.add(2) = 0;
        *mtk_tag.add(3) = 0;
    }

    skb
}

unsafe fn mtk_tag_rcv(skb: *mut sk_buff, dev: *mut net_device) -> *mut sk_buff {
    let hdr: u16;
    let port: i32;
    let phdr: *mut __be16;

    if !pskb_may_pull(skb, MTK_HDR_LEN) {
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    phdr = dsa_etype_header_pos_rx(skb);
    hdr = ntohs(*phdr);

    /* Remove MTK tag and recalculate checksum. */
    skb_pull_rcsum(skb, MTK_HDR_LEN);

    dsa_strip_etype_header(skb, MTK_HDR_LEN);

    /* Get source port information */
    port = (hdr & MTK_HDR_RECV_SOURCE_PORT_MASK) as i32;

    (*skb).dev = dsa_conduit_find_user(dev, 0, port);
    if (*skb).dev.is_null() {
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    dsa_default_offload_fwd_mark(skb);

    skb
}

static mtk_netdev_ops: dsa_device_ops = dsa_device_ops {
    name: MTK_NAME,
    proto: DSA_TAG_PROTO_MTK,
    xmit: Some(mtk_tag_xmit),
    rcv: Some(mtk_tag_rcv),
    needed_headroom: MTK_HDR_LEN,
};

// MODULE_DESCRIPTION("DSA tag driver for Mediatek switches");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS_DSA_TAG_DRIVER(DSA_TAG_PROTO_MTK, MTK_NAME);
// module_dsa_tag_driver(mtk_netdev_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
