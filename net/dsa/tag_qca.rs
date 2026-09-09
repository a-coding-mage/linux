// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2015, The Linux Foundation. All rights reserved.
 */

// Dependencies supplied by the surrounding kernel/DSA translation.

const QCA_NAME: &[u8] = b"qca\0";

unsafe fn qca_tag_xmit(skb: *mut sk_buff, dev: *mut net_device) -> *mut sk_buff {
    let phdr: *mut __be16;
    let mut hdr: u16;

    skb_push(skb, QCA_HDR_LEN);

    dsa_alloc_etype_header(skb, QCA_HDR_LEN);
    phdr = dsa_etype_header_pos_tx(skb);

    /* Set the version field, and set destination port information */
    hdr = FIELD_PREP(QCA_HDR_XMIT_VERSION, QCA_HDR_VERSION);
    hdr |= QCA_HDR_XMIT_FROM_CPU;
    hdr |= FIELD_PREP(QCA_HDR_XMIT_DP_BIT, dsa_xmit_port_mask(skb, dev));

    *phdr = htons(hdr);

    skb
}

unsafe fn qca_tag_rcv(skb: *mut sk_buff, dev: *mut net_device) -> *mut sk_buff {
    let mut tagger_data: *mut qca_tagger_data;
    let dp: *mut dsa_port = (*dev).dsa_ptr;
    let ds: *mut dsa_switch = (*dp).ds;
    let mut ver: u8;
    let mut pk_type: u8;
    let phdr: *mut __be16;
    let port: i32;
    let mut hdr: u16;

    BUILD_BUG_ON!(core::mem::size_of::<qca_mgmt_ethhdr>() != QCA_HDR_MGMT_HEADER_LEN + QCA_HDR_LEN);

    tagger_data = (*ds).tagger_data;

    if unlikely(!pskb_may_pull(skb, QCA_HDR_LEN)) {
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    phdr = dsa_etype_header_pos_rx(skb);
    hdr = ntohs(*phdr);

    /* Make sure the version is correct */
    ver = FIELD_GET(QCA_HDR_RECV_VERSION, hdr);
    if unlikely(ver != QCA_HDR_VERSION) {
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    /* Get pk type */
    pk_type = FIELD_GET(QCA_HDR_RECV_TYPE, hdr);

    /* Ethernet mgmt read/write packet */
    if pk_type == QCA_HDR_RECV_TYPE_RW_REG_ACK {
        if likely(!(*tagger_data).rw_reg_ack_handler.is_null()) {
            ((*tagger_data).rw_reg_ack_handler)(ds, skb);
        }
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    /* Ethernet MIB counter packet */
    if pk_type == QCA_HDR_RECV_TYPE_MIB {
        if likely(!(*tagger_data).mib_autocast_handler.is_null()) {
            ((*tagger_data).mib_autocast_handler)(ds, skb);
        }
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    /* Get source port information */
    port = FIELD_GET(QCA_HDR_RECV_SOURCE_PORT, hdr);

    (*skb).dev = dsa_conduit_find_user(dev, 0, port);
    if (*skb).dev.is_null() {
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    /* Remove QCA tag and recalculate checksum */
    skb_pull_rcsum(skb, QCA_HDR_LEN);
    dsa_strip_etype_header(skb, QCA_HDR_LEN);

    skb
}

unsafe fn qca_tag_connect(ds: *mut dsa_switch) -> i32 {
    let tagger_data: *mut qca_tagger_data;

    tagger_data = kzalloc_obj::<qca_tagger_data>();
    if tagger_data.is_null() {
        return -ENOMEM;
    }

    (*ds).tagger_data = tagger_data;

    0
}

unsafe fn qca_tag_disconnect(ds: *mut dsa_switch) {
    kfree((*ds).tagger_data);
    (*ds).tagger_data = core::ptr::null_mut();
}

static qca_netdev_ops: dsa_device_ops = dsa_device_ops {
    name: QCA_NAME,
    proto: DSA_TAG_PROTO_QCA,
    connect: Some(qca_tag_connect),
    disconnect: Some(qca_tag_disconnect),
    xmit: Some(qca_tag_xmit),
    rcv: Some(qca_tag_rcv),
    needed_headroom: QCA_HDR_LEN,
    promisc_on_conduit: true,
};

// MODULE_DESCRIPTION("DSA tag driver for Qualcomm Atheros QCA8K switches");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS_DSA_TAG_DRIVER(DSA_TAG_PROTO_QCA, QCA_NAME);
// module_dsa_tag_driver(qca_netdev_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
