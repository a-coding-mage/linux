// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2025-2026 NXP
 */

// Dependency intent: declarations are supplied by <linux/dsa/tag_netc.h> and "tag.h".

const NETC_NAME: &str = "nxp_netc";

const NETC_TAG_FORWARD: u8 = 0;
const NETC_TAG_TO_PORT: u8 = 1;
const NETC_TAG_TP_SUBTYPE0: u8 = 0;
const NETC_TAG_TO_HOST: u8 = 2;
const NETC_TAG_TH_SUBTYPE0: u8 = 0;
const NETC_TAG_TH_SUBTYPE1: u8 = 1;
const NETC_TAG_TH_SUBTYPE2: u8 = 2;

const NETC_TAG_FORWARD_LEN: i32 = 6;
const NETC_TAG_TP_SUBTYPE0_LEN: i32 = 6;
const NETC_TAG_TH_SUBTYPE0_LEN: i32 = 6;
const NETC_TAG_TH_SUBTYPE1_LEN: i32 = 14;
const NETC_TAG_TH_SUBTYPE2_LEN: i32 = 14;
const NETC_TAG_CMN_LEN: usize = 5;

const NETC_TAG_SUBTYPE: u8 = 0x0f;
const NETC_TAG_TYPE: u8 = 0xf0;
const NETC_TAG_QV: u8 = 1;
const NETC_TAG_IPV: u8 = 0x1c;
const NETC_TAG_SWITCH: u8 = 0x07;
const NETC_TAG_PORT: u8 = 0xf8;

#[repr(C, packed)]
struct netc_tag_cmn {
    tpid: u16,
    type_: u8,
    qos: u8,
    switch_port: u8,
}

unsafe fn netc_fill_common_tag(
    tag: *mut netc_tag_cmn,
    type_: u8,
    subtype: u8,
    sw_id: u8,
    port: u8,
    ipv: u8,
) {
    (*tag).tpid = htons(ETH_P_NXP_NETC);
    (*tag).type_ = ((type_ << 4) & NETC_TAG_TYPE) | (subtype & NETC_TAG_SUBTYPE);
    (*tag).qos = NETC_TAG_QV | ((ipv << 2) & NETC_TAG_IPV);
    (*tag).switch_port = (sw_id & NETC_TAG_SWITCH) | ((port << 3) & NETC_TAG_PORT);
}

unsafe fn netc_fill_common_tp_tag(
    skb: *mut sk_buff,
    ndev: *mut net_device,
    subtype: u8,
    tag_len: i32,
) -> *mut core::ffi::c_void {
    let dp = dsa_user_to_port(ndev);
    let queue: u16 = skb_get_queue_mapping(skb);
    let mut ipv: i8 = netdev_txq_to_tc(ndev, queue);
    let tag: *mut core::ffi::c_void;

    if ipv < 0 {
        ipv = 0;
    }

    skb_push(skb, tag_len);
    dsa_alloc_etype_header(skb, tag_len);

    tag = dsa_etype_header_pos_tx(skb);
    memset(tag.add(NETC_TAG_CMN_LEN), 0, (tag_len as usize) - NETC_TAG_CMN_LEN);
    /* As 'dsa,member' is a required property for NETC switch, the member
     * is used to specify the switch ID (thus the hardware switch ID and
     * the software switch ID are consistent), its range is 1 ~ 7. The
     * NETC switch driver will check this value, and if it is invalid,
     * the switch driver will fail the probe.
     * In addition, according to the nxp,netc-switch.yaml doc, the port
     * index will not be greater than 0xf.
     */
    netc_fill_common_tag(
        tag as *mut netc_tag_cmn,
        NETC_TAG_TO_PORT,
        subtype,
        (*dp).ds.index,
        (*dp).index,
        ipv as u8,
    );

    tag
}

unsafe fn netc_fill_tp_tag_subtype0(skb: *mut sk_buff, ndev: *mut net_device) {
    netc_fill_common_tp_tag(skb, ndev, NETC_TAG_TP_SUBTYPE0, NETC_TAG_TP_SUBTYPE0_LEN);
}

/* Currently only support To_Port tag, subtype 0 */
unsafe fn netc_xmit(skb: *mut sk_buff, ndev: *mut net_device) -> *mut sk_buff {
    netc_fill_tp_tag_subtype0(skb, ndev);
    skb
}

fn netc_get_rx_tag_len(type_: i32, subtype: i32) -> i32 {
    if type_ == NETC_TAG_TO_HOST as i32 {
        if subtype == NETC_TAG_TH_SUBTYPE1 as i32 {
            NETC_TAG_TH_SUBTYPE1_LEN
        } else if subtype == NETC_TAG_TH_SUBTYPE2 as i32 {
            NETC_TAG_TH_SUBTYPE2_LEN
        } else {
            NETC_TAG_TH_SUBTYPE0_LEN
        }
    } else {
        NETC_TAG_FORWARD_LEN
    }
}

unsafe fn netc_rcv(skb: *mut sk_buff, ndev: *mut net_device) -> *mut sk_buff {
    if !pskb_may_pull(skb, NETC_TAG_MAX_LEN) {
        kfree_skb(skb);
        return core::ptr::null_mut();
    }
    let tag_cmn = dsa_etype_header_pos_rx(skb) as *mut netc_tag_cmn;
    if ntohs((*tag_cmn).tpid) != ETH_P_NXP_NETC {
        dev_warn_ratelimited(&(*ndev).dev, "Unknown TPID 0x%04x\n", ntohs((*tag_cmn).tpid));
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    if (*tag_cmn).qos & NETC_TAG_QV != 0 {
        (*skb).priority = ((*tag_cmn).qos & NETC_TAG_IPV) >> 2;
    }

    let sw_id = (*tag_cmn).switch_port & NETC_TAG_SWITCH;
    if sw_id == 0 {
        dev_warn_ratelimited(&(*ndev).dev, "VEPA switch ID is not supported yet\n");
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    let port = ((*tag_cmn).switch_port & NETC_TAG_PORT) >> 3;
    (*skb).dev = dsa_conduit_find_user(ndev, sw_id, port);
    if (*skb).dev.is_null() {
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    let type_ = ((*tag_cmn).type_ & NETC_TAG_TYPE) >> 4;
    let subtype = (*tag_cmn).type_ & NETC_TAG_SUBTYPE;
    if type_ == NETC_TAG_FORWARD {
        dsa_default_offload_fwd_mark(skb);
    } else if type_ == NETC_TAG_TO_HOST {
        if subtype != NETC_TAG_TH_SUBTYPE0 {
            kfree_skb(skb);
            return core::ptr::null_mut();
        }
    } else {
        dev_warn_ratelimited(&(*ndev).dev, "Unexpected  tag type %d\n", type_);
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    let tag_len = netc_get_rx_tag_len(type_ as i32, subtype as i32);
    skb_pull_rcsum(skb, tag_len);
    dsa_strip_etype_header(skb, tag_len);
    skb
}

unsafe fn netc_flow_dissect(skb: *const sk_buff, proto: *mut u16, offset: *mut i32) {
    let tag_cmn = ((*skb).data.sub(2)) as *const netc_tag_cmn;
    let subtype = ((*tag_cmn).type_ & NETC_TAG_SUBTYPE) as i32;
    let type_ = (((*tag_cmn).type_ & NETC_TAG_TYPE) >> 4) as i32;
    let tag_len = netc_get_rx_tag_len(type_, subtype);

    /* The RX minimum frame length of the NETC switch port is 64 bytes,
     * and the frame is received by the ENETC driver. From the hardware
     * perspective, the receive buffer of RX BD is at least 128 bytes,
     * so the switch tag header is guaranteed to be in the linear region
     * of the skb.
     */
    *offset = tag_len;
    *proto = *((*skb).data as *const u16).add((tag_len / 2 - 1) as usize);
}

// The DSA device-ops registration and module metadata are supplied by the kernel integration.
static netc_netdev_ops: dsa_device_ops = dsa_device_ops {
    name: NETC_NAME,
    proto: DSA_TAG_PROTO_NETC,
    xmit: netc_xmit,
    rcv: netc_rcv,
    needed_headroom: NETC_TAG_MAX_LEN,
    flow_dissect: netc_flow_dissect,
};

// MODULE_DESCRIPTION("DSA tag driver for NXP NETC switch family");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS_DSA_TAG_DRIVER(DSA_TAG_PROTO_NETC, NETC_NAME);
// module_dsa_tag_driver(netc_netdev_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
