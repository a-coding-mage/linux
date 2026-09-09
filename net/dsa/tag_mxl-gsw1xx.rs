// SPDX-License-Identifier: GPL-2.0+
/*
 * DSA driver Special Tag support for MaxLinear GSW1xx switch chips
 *
 * Copyright (C) 2025 Daniel Golle <daniel@makrotopia.org>
 * Copyright (C) 2023 - 2024 MaxLinear Inc.
 */

// Linux kernel dependencies supplied by the surrounding tree.

pub const GSW1XX_TAG_NAME: &str = "gsw1xx";
pub const GSW1XX_HEADER_LEN: usize = 8;

pub const GSW1XX_TX_PORT_MAP: u16 = 0xff;
pub const GSW1XX_TX_PORT_MAP_EN: u16 = 1 << 15;
pub const GSW1XX_TX_CLASS_EN: u16 = 1 << 14;
pub const GSW1XX_TX_TIME_STAMP_EN: u16 = 1 << 13;
pub const GSW1XX_TX_LRN_DIS: u16 = 1 << 12;
pub const GSW1XX_TX_CLASS: u16 = 0xf << 8;
pub const GSW1XX_RX_PORT_MAP: u16 = 0xff00;

extern "C" {
    pub fn skb_push(skb: *mut sk_buff, len: usize);
    pub fn dsa_alloc_etype_header(skb: *mut sk_buff, len: usize);
    pub fn dsa_etype_header_pos_tx(skb: *mut sk_buff) -> *mut u16;
    pub fn dsa_etype_header_pos_rx(skb: *mut sk_buff) -> *mut u16;
    pub fn dsa_xmit_port_mask(skb: *mut sk_buff, dev: *mut net_device) -> u16;
    pub fn pskb_may_pull(skb: *mut sk_buff, len: usize) -> bool;
    pub fn dsa_conduit_find_user(
        dev: *mut net_device,
        tree_index: i32,
        port: i32,
    ) -> *mut net_device;
    pub fn skb_pull_rcsum(skb: *mut sk_buff, len: usize);
    pub fn dsa_strip_etype_header(skb: *mut sk_buff, len: usize);
    pub fn kfree_skb(skb: *mut sk_buff);
}

#[repr(C)]
pub struct sk_buff {
    pub dev: *mut net_device,
}

#[repr(C)]
pub struct net_device {
    pub dev: device,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dsa_device_ops {
    pub name: *const u8,
    pub proto: i32,
    pub xmit: Option<unsafe extern "C" fn(*mut sk_buff, *mut net_device) -> *mut sk_buff>,
    pub rcv: Option<unsafe extern "C" fn(*mut sk_buff, *mut net_device) -> *mut sk_buff>,
    pub needed_headroom: usize,
}

extern "C" {
    pub fn dev_warn_ratelimited(dev: *mut device, fmt: *const u8, ...);
}

pub const ETH_P_MXLGSW: u16 = 0x88c3;
pub const DSA_TAG_PROTO_MXL_GSW1XX: i32 = 0;

#[inline]
unsafe extern "C" fn gsw1xx_tag_xmit(
    skb: *mut sk_buff,
    dev: *mut net_device,
) -> *mut sk_buff {
    let gsw1xx_tag: *mut u16;
    let tag: u16;

    skb_push(skb, GSW1XX_HEADER_LEN);
    dsa_alloc_etype_header(skb, GSW1XX_HEADER_LEN);

    gsw1xx_tag = dsa_etype_header_pos_tx(skb);
    *gsw1xx_tag.add(0) = ETH_P_MXLGSW.to_be();

    tag = (dsa_xmit_port_mask(skb, dev) & GSW1XX_TX_PORT_MAP)
        | GSW1XX_TX_PORT_MAP_EN
        | GSW1XX_TX_LRN_DIS;
    *gsw1xx_tag.add(1) = tag.to_be();
    *gsw1xx_tag.add(2) = 0;
    *gsw1xx_tag.add(3) = 0;

    skb
}

#[inline]
unsafe extern "C" fn gsw1xx_tag_rcv(
    skb: *mut sk_buff,
    dev: *mut net_device,
) -> *mut sk_buff {
    let port: i32;
    let gsw1xx_tag: *mut u16;

    if !pskb_may_pull(skb, GSW1XX_HEADER_LEN) {
        dev_warn_ratelimited(
            &mut (*dev).dev,
            b"Dropping packet, cannot pull SKB\0".as_ptr(),
        );
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    gsw1xx_tag = dsa_etype_header_pos_rx(skb);

    if u16::from_be(*gsw1xx_tag.add(0)) != ETH_P_MXLGSW {
        dev_warn_ratelimited(
            &mut (*dev).dev,
            b"Dropping packet due to invalid special tag\0".as_ptr(),
        );
        dev_warn_ratelimited(&mut (*dev).dev, b"Tag: %8ph\n\0".as_ptr(), gsw1xx_tag);
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    port = ((u16::from_be(*gsw1xx_tag.add(1)) & GSW1XX_RX_PORT_MAP) >> 8) as i32;
    (*skb).dev = dsa_conduit_find_user(dev, 0, port);
    if (*skb).dev.is_null() {
        dev_warn_ratelimited(
            &mut (*dev).dev,
            b"Dropping packet due to invalid source port\0".as_ptr(),
        );
        dev_warn_ratelimited(&mut (*dev).dev, b"Tag: %8ph\n\0".as_ptr(), gsw1xx_tag);
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    skb_pull_rcsum(skb, GSW1XX_HEADER_LEN);
    dsa_strip_etype_header(skb, GSW1XX_HEADER_LEN);

    skb
}

#[no_mangle]
pub static gsw1xx_netdev_ops: dsa_device_ops = dsa_device_ops {
    name: GSW1XX_TAG_NAME.as_bytes().as_ptr(),
    proto: DSA_TAG_PROTO_MXL_GSW1XX,
    xmit: Some(gsw1xx_tag_xmit),
    rcv: Some(gsw1xx_tag_rcv),
    needed_headroom: GSW1XX_HEADER_LEN,
};

// MODULE_DESCRIPTION("DSA tag driver for MaxLinear GSW1xx 8 byte protocol");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS_DSA_TAG_DRIVER(DSA_TAG_PROTO_MXL_GSW1XX, GSW1XX_TAG_NAME);
// module_dsa_tag_driver(gsw1xx_netdev_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
