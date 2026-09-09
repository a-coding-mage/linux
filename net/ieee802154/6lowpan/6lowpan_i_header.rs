/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies:
// linux/list.h, net/ieee802154_netdev.h, net/inet_frag.h, net/6lowpan.h

use core::ffi::c_void;

pub type lowpan_rx_result = u32;

pub const RX_CONTINUE: lowpan_rx_result = 0u32;
pub const RX_DROP_UNUSABLE: lowpan_rx_result = 1u32;
pub const RX_DROP: lowpan_rx_result = 2u32;
pub const RX_QUEUED: lowpan_rx_result = 3u32;

pub const LOWPAN_DISPATCH_FRAG1: u8 = 0xc0;
pub const LOWPAN_DISPATCH_FRAGN: u8 = 0xe0;

#[repr(C)]
pub struct frag_lowpan_compare_key {
    pub tag: u16,
    pub d_size: u16,
    pub src: ieee802154_addr,
    pub dst: ieee802154_addr,
}

/* Equivalent of ipv4 struct ipq
 */
#[repr(C)]
pub struct lowpan_frag_queue {
    pub q: inet_frag_queue,
}

extern "C" {
    pub fn lowpan_frag_rcv(skb: *mut sk_buff, frag_type: u8) -> i32;
    pub fn lowpan_net_frag_exit();
    pub fn lowpan_net_frag_init() -> i32;

    pub fn lowpan_rx_init();
    pub fn lowpan_rx_exit();

    pub fn lowpan_header_create(
        skb: *mut sk_buff,
        dev: *mut net_device,
        type_: u16,
        daddr: *const c_void,
        saddr: *const c_void,
        len: u32,
    ) -> i32;
    pub fn lowpan_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t;

    pub fn lowpan_iphc_decompress(skb: *mut sk_buff) -> i32;
    pub fn lowpan_rx_h_ipv6(skb: *mut sk_buff) -> lowpan_rx_result;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
