// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Motorcomm YT921x Switch Extended CPU Port Tagging
 *
 * Copyright (c) 2025 David Yang <mmyangfl@gmail.com>
 *
 * +----+----+-------+-----+----+---------
 * | DA | SA | TagET | Tag | ET | Payload ...
 * +----+----+-------+-----+----+---------
 *   6    6      2      6    2       N
 *
 * Tag Ethertype: CPU_TAG_TPID_TPID (default: ETH_P_YT921X = 0x9988)
 *   * Hardcoded for the moment, but still configurable. Discuss it if there
 *     are conflicts somewhere and/or you want to change it for some reason.
 * Tag:
 *   2: VLAN Tag
 *   2:
 *     15b: Rx Port Valid
 *     14b-11b: Rx Port
 *     10b-8b: Tx/Rx Priority
 *     7b: Tx/Rx Code Valid
 *     6b-1b: Tx/Rx Code
 *     0b: ? (unset)
 *   2:
 *     15b: Tx Port(s) Valid
 *     10b-0b: Tx Port(s) Mask
 */

// Dependency declarations supplied by the surrounding kernel/DSA code.
use core::ffi::c_void;

#[repr(C)]
pub struct sk_buff {
    pub priority: u32,
    pub dev: *mut net_device,
}
#[repr(C)]
pub struct net_device { pub dev: c_void }
#[repr(C)]
pub struct dsa_device_ops {
    pub name: *const u8,
    pub proto: i32,
    pub xmit: Option<unsafe extern "C" fn(*mut sk_buff, *mut net_device) -> *mut sk_buff>,
    pub rcv: Option<unsafe extern "C" fn(*mut sk_buff, *mut net_device) -> *mut sk_buff>,
    pub needed_headroom: usize,
}

extern "C" {
    fn skb_push(skb: *mut sk_buff, len: usize);
    fn dsa_alloc_etype_header(skb: *mut sk_buff, len: usize);
    fn dsa_etype_header_pos_tx(skb: *mut sk_buff) -> *mut u16;
    fn dsa_etype_header_pos_rx(skb: *mut sk_buff) -> *mut u16;
    fn dsa_xmit_port_mask(skb: *mut sk_buff, netdev: *mut net_device) -> u16;
    fn pskb_may_pull(skb: *mut sk_buff, len: usize) -> bool;
    fn kfree_skb(skb: *mut sk_buff);
    fn dsa_conduit_find_user(netdev: *mut net_device, index: u32, port: u32) -> *mut net_device;
    fn dsa_default_offload_fwd_mark(skb: *mut sk_buff);
    fn skb_pull_rcsum(skb: *mut sk_buff, len: usize);
    fn dsa_strip_etype_header(skb: *mut sk_buff, len: usize);
}

const YT921X_TAG_NAME: &[u8] = b"yt921x\0";
const YT921X_TAG_LEN: usize = 8;
const YT921X_TAG_PORT_EN: u16 = 1 << 15;
const YT921X_TAG_RX_PORT_M: u16 = 0x7800;
const YT921X_TAG_PRIO_M: u16 = 0x0700;
const YT921X_TAG_CODE_EN: u16 = 1 << 7;
const YT921X_TAG_CODE_M: u16 = 0x007e;
const YT921X_TAG_TX_PORTS_M: u16 = 0x07ff;
const ETH_P_YT921X: u16 = 0x9988;
const DSA_TAG_PROTO_YT921X: i32 = 0;

#[inline]
const fn yt921x_tag_prio(x: u16) -> u16 { (x << 8) & YT921X_TAG_PRIO_M }
#[inline]
const fn yt921x_tag_code(x: u16) -> u16 { (x << 1) & YT921X_TAG_CODE_M }
#[inline]
const fn yt921x_tag_tx_ports(x: u16) -> u16 { x & YT921X_TAG_TX_PORTS_M }
#[inline]
const fn field_get(mask: u16, value: u16) -> u16 { (value & mask) >> mask.trailing_zeros() }
#[inline]
const fn htons(x: u16) -> u16 { x.to_be() }
#[inline]
const fn ntohs(x: u16) -> u16 { u16::from_be(x) }

#[repr(i32)]
enum yt921x_tag_code {
    YT921X_TAG_CODE_FORWARD = 0,
    YT921X_TAG_CODE_ACL = 0x17,
    YT921X_TAG_CODE_UNK_UCAST = 0x19,
    YT921X_TAG_CODE_UNK_MCAST = 0x1a,
    YT921X_TAG_CODE_PORT_COPY = 0x1b,
    YT921X_TAG_CODE_FDB_COPY = 0x1c,
}

unsafe extern "C" fn yt921x_tag_xmit(skb: *mut sk_buff, netdev: *mut net_device) -> *mut sk_buff {
    skb_push(skb, YT921X_TAG_LEN);
    dsa_alloc_etype_header(skb, YT921X_TAG_LEN);
    let tag = dsa_etype_header_pos_tx(skb);
    *tag.add(0) = htons(ETH_P_YT921X);
    // VLAN tag unrelated when TX
    *tag.add(1) = 0;
    let ctrl = yt921x_tag_code(0) | YT921X_TAG_CODE_EN |
        yt921x_tag_prio((*skb).priority as u16);
    *tag.add(2) = htons(ctrl);
    let ctrl = yt921x_tag_tx_ports(dsa_xmit_port_mask(skb, netdev)) | YT921X_TAG_PORT_EN;
    *tag.add(3) = htons(ctrl);
    skb
}

unsafe extern "C" fn yt921x_tag_rcv(skb: *mut sk_buff, netdev: *mut net_device) -> *mut sk_buff {
    if !pskb_may_pull(skb, YT921X_TAG_LEN) { kfree_skb(skb); return core::ptr::null_mut(); }
    let tag = dsa_etype_header_pos_rx(skb);
    if *tag != htons(ETH_P_YT921X) { kfree_skb(skb); return core::ptr::null_mut(); }
    // Locate which port this is coming from
    let rx = ntohs(*tag.add(2));
    if rx & YT921X_TAG_PORT_EN == 0 { kfree_skb(skb); return core::ptr::null_mut(); }
    let port = field_get(YT921X_TAG_RX_PORT_M, rx) as u32;
    (*skb).dev = dsa_conduit_find_user(netdev, 0, port);
    if (*skb).dev.is_null() { kfree_skb(skb); return core::ptr::null_mut(); }
    (*skb).priority = field_get(YT921X_TAG_PRIO_M, rx) as u32;
    if rx & YT921X_TAG_CODE_EN == 0 {
        // Tag code not enabled in rx packet
    } else {
        match field_get(YT921X_TAG_CODE_M, rx) as i32 {
            0 | 0x1b | 0x1c => {
                // Already forwarded by hardware
                dsa_default_offload_fwd_mark(skb)
            },
            0x17 | 0x19 | 0x1a => {},
            _ => {},
        }
    }
    // Remove YT921x tag and update checksum
    skb_pull_rcsum(skb, YT921X_TAG_LEN);
    dsa_strip_etype_header(skb, YT921X_TAG_LEN);
    skb
}

#[no_mangle]
pub static yt921x_netdev_ops: dsa_device_ops = dsa_device_ops {
    name: YT921X_TAG_NAME.as_ptr(), proto: DSA_TAG_PROTO_YT921X,
    xmit: Some(yt921x_tag_xmit), rcv: Some(yt921x_tag_rcv), needed_headroom: YT921X_TAG_LEN,
};

// MODULE_DESCRIPTION("DSA tag driver for Motorcomm YT921x switches");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS_DSA_TAG_DRIVER(DSA_TAG_PROTO_YT921X, YT921X_TAG_NAME);
// module_dsa_tag_driver(yt921x_netdev_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
