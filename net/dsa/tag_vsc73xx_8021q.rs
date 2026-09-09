// SPDX-License-Identifier: GPL-2.0 OR MIT
/* Copyright (C) 2024 Pawel Dembicki <paweldembicki@gmail.com>
 */
// Translated from tag_vsc73xx_8021q.c.
// Dependencies supplied by the Linux DSA/networking environment are referenced
// here as external types, functions, constants, and macros.

const VSC73XX_8021Q_NAME: &str = "vsc73xx-8021q";

extern "C" {
    fn dsa_user_to_port(netdev: *mut net_device) -> *mut dsa_port;
    fn skb_get_queue_mapping(skb: *mut sk_buff) -> u16;
    fn dsa_tag_8021q_standalone_vid(dp: *mut dsa_port) -> u16;
    fn dsa_port_bridge_num_get(dp: *mut dsa_port) -> core::ffi::c_uint;
    fn dsa_port_bridge_dev_get(dp: *mut dsa_port) -> *mut net_device;
    fn br_vlan_enabled(br: *mut net_device) -> bool;
    fn dsa_tag_8021q_bridge_vid(bridge_num: core::ffi::c_uint) -> u16;
    fn netdev_txq_to_tc(netdev: *mut net_device, queue_mapping: u16) -> u8;
    fn dsa_8021q_xmit(
        skb: *mut sk_buff,
        netdev: *mut net_device,
        protocol: u16,
        tci: u16,
    ) -> *mut sk_buff;
    fn dsa_8021q_rcv(
        skb: *mut sk_buff,
        src_port: *mut core::ffi::c_int,
        switch_id: *mut core::ffi::c_int,
        vbid: *mut core::ffi::c_int,
        vid: *mut core::ffi::c_int,
    );
    fn dsa_tag_8021q_find_user(
        netdev: *mut net_device,
        src_port: core::ffi::c_int,
        switch_id: core::ffi::c_int,
        vid: core::ffi::c_int,
        vbid: core::ffi::c_int,
    ) -> *mut net_device;
    fn dev_warn_ratelimited(dev: *mut device, message: *const core::ffi::c_char);
    fn kfree_skb(skb: *mut sk_buff);
    fn dsa_default_offload_fwd_mark(skb: *mut sk_buff);
}

extern "C" {
    static ETH_P_8021Q: u16;
    static VLAN_PRIO_SHIFT: u32;
    static VLAN_HLEN: u16;
    static DSA_TAG_PROTO_VSC73XX_8021Q: core::ffi::c_int;
}

#[repr(C)]
struct sk_buff {
    dev: *mut net_device,
    offload_fwd_mark: bool,
}

#[repr(C)]
struct net_device {
    dev: device,
}

#[repr(C)]
struct device;

#[repr(C)]
struct dsa_port;

#[repr(C)]
struct dsa_device_ops {
    name: *const core::ffi::c_char,
    proto: core::ffi::c_int,
    xmit: Option<unsafe extern "C" fn(*mut sk_buff, *mut net_device) -> *mut sk_buff>,
    rcv: Option<unsafe extern "C" fn(*mut sk_buff, *mut net_device) -> *mut sk_buff>,
    needed_headroom: u16,
    promisc_on_conduit: bool,
}

unsafe extern "C" fn vsc73xx_xmit(
    skb: *mut sk_buff,
    netdev: *mut net_device,
) -> *mut sk_buff {
    let dp = dsa_user_to_port(netdev);
    let queue_mapping = skb_get_queue_mapping(skb);
    let mut tx_vid = dsa_tag_8021q_standalone_vid(dp);
    let pcp: u8;

    if (*skb).offload_fwd_mark {
        let bridge_num = dsa_port_bridge_num_get(dp);
        let br = dsa_port_bridge_dev_get(dp);

        if br_vlan_enabled(br) {
            return skb;
        }

        tx_vid = dsa_tag_8021q_bridge_vid(bridge_num);
    }

    pcp = netdev_txq_to_tc(netdev, queue_mapping);

    dsa_8021q_xmit(
        skb,
        netdev,
        ETH_P_8021Q,
        ((pcp as u16) << VLAN_PRIO_SHIFT) | tx_vid,
    )
}

unsafe extern "C" fn vsc73xx_rcv(
    skb: *mut sk_buff,
    netdev: *mut net_device,
) -> *mut sk_buff {
    let mut src_port: core::ffi::c_int = -1;
    let mut switch_id: core::ffi::c_int = -1;
    let mut vbid: core::ffi::c_int = -1;
    let mut vid: core::ffi::c_int = -1;

    dsa_8021q_rcv(skb, &mut src_port, &mut switch_id, &mut vbid, &mut vid);

    (*skb).dev = dsa_tag_8021q_find_user(netdev, src_port, switch_id, vid, vbid);
    if (*skb).dev.is_null() {
        // "Couldn't decode source port\n"
        dev_warn_ratelimited(
            &mut (*netdev).dev,
            b"Couldn't decode source port\n\0".as_ptr() as *const core::ffi::c_char,
        );
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    dsa_default_offload_fwd_mark(skb);

    skb
}

#[no_mangle]
static vsc73xx_8021q_netdev_ops: dsa_device_ops = dsa_device_ops {
    name: b"vsc73xx-8021q\0".as_ptr() as *const core::ffi::c_char,
    proto: DSA_TAG_PROTO_VSC73XX_8021Q,
    xmit: Some(vsc73xx_xmit),
    rcv: Some(vsc73xx_rcv),
    needed_headroom: VLAN_HLEN,
    promisc_on_conduit: true,
};

// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("DSA tag driver for VSC73XX family of switches, using VLAN");
// MODULE_ALIAS_DSA_TAG_DRIVER(DSA_TAG_PROTO_VSC73XX_8021Q, VSC73XX_8021Q_NAME);
// module_dsa_tag_driver(vsc73xx_8021q_netdev_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
