// SPDX-License-Identifier: GPL-2.0
/* Copyright 2019 NXP
 */

// Dependency declarations from <linux/dsa/ocelot.h> and "tag.h" are supplied
// by the surrounding translation unit.

const OCELOT_NAME: *const core::ffi::c_char = b"ocelot\0".as_ptr() as *const _;
const SEVILLE_NAME: *const core::ffi::c_char = b"seville\0".as_ptr() as *const _;

unsafe fn ocelot_xmit_common(
    skb: *mut sk_buff,
    netdev: *mut net_device,
    ifh_prefix: __be32,
    ifh: *mut *mut core::ffi::c_void,
) {
    let dp = dsa_user_to_port(netdev);
    let ds = (*dp).ds;
    let mut vlan_tci: u64 = 0;
    let mut tag_type: u64 = 0;
    let injection: *mut core::ffi::c_void;
    let prefix: *mut __be32;
    let mut rew_op: u32 = 0;
    let qos_class: u64;

    ocelot_xmit_get_vlan_info(
        skb,
        dsa_port_bridge_dev_get(dp),
        &mut vlan_tci,
        &mut tag_type,
    );

    qos_class = if netdev_get_num_tc(netdev) != 0 {
        netdev_get_prio_tc_map(netdev, (*skb).priority)
    } else {
        (*skb).priority as u64
    };

    injection = skb_push(skb, OCELOT_TAG_LEN);
    prefix = skb_push(skb, OCELOT_SHORT_PREFIX_LEN) as *mut __be32;

    *prefix = ifh_prefix;
    core::ptr::write_bytes(injection, 0, OCELOT_TAG_LEN as usize);
    ocelot_ifh_set_bypass(injection, 1);
    ocelot_ifh_set_src(injection, (*ds).num_ports);
    ocelot_ifh_set_qos_class(injection, qos_class);
    ocelot_ifh_set_vlan_tci(injection, vlan_tci);
    ocelot_ifh_set_tag_type(injection, tag_type);

    rew_op = ocelot_ptp_rew_op(skb);
    if rew_op != 0 {
        ocelot_ifh_set_rew_op(injection, rew_op);
    }

    *ifh = injection;
}

unsafe fn ocelot_xmit(skb: *mut sk_buff, netdev: *mut net_device) -> *mut sk_buff {
    let mut injection: *mut core::ffi::c_void = core::ptr::null_mut();

    ocelot_xmit_common(skb, netdev, cpu_to_be32(0x8880000a), &mut injection);
    ocelot_ifh_set_dest(injection, dsa_xmit_port_mask(skb, netdev));

    skb
}

unsafe fn seville_xmit(skb: *mut sk_buff, netdev: *mut net_device) -> *mut sk_buff {
    let mut injection: *mut core::ffi::c_void = core::ptr::null_mut();

    ocelot_xmit_common(skb, netdev, cpu_to_be32(0x88800005), &mut injection);
    seville_ifh_set_dest(injection, dsa_xmit_port_mask(skb, netdev));

    skb
}

unsafe fn ocelot_rcv(skb: *mut sk_buff, netdev: *mut net_device) -> *mut sk_buff {
    let mut src_port: u64 = 0;
    let mut qos_class: u64 = 0;
    let mut vlan_tci: u64 = 0;
    let mut tag_type: u64 = 0;
    let start = (*skb).data;
    let mut dp: *mut dsa_port;
    let extraction: *mut u8;
    let mut vlan_tpid: u16;
    let mut rew_val: u64 = 0;

    // Revert skb->data by the amount consumed by the DSA conduit.
    skb_push(skb, ETH_HLEN);
    // Discard the short prefix by moving it into the headroom.
    skb_pull(skb, OCELOT_SHORT_PREFIX_LEN);
    extraction = (*skb).data;
    skb_pull(skb, OCELOT_TAG_LEN);
    skb_reset_mac_header(skb);
    skb_reset_mac_len(skb);
    skb_pull(skb, ETH_HLEN);

    skb_postpull_rcsum(skb, start, OCELOT_TOTAL_TAG_LEN);

    ocelot_xfh_get_src_port(extraction, &mut src_port);
    ocelot_xfh_get_qos_class(extraction, &mut qos_class);
    ocelot_xfh_get_tag_type(extraction, &mut tag_type);
    ocelot_xfh_get_vlan_tci(extraction, &mut vlan_tci);
    ocelot_xfh_get_rew_val(extraction, &mut rew_val);

    (*skb).dev = dsa_conduit_find_user(netdev, 0, src_port);
    if (*skb).dev.is_null() {
        // Ignore reflected frames from sockets on the bare DSA conduit.
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    dsa_default_offload_fwd_mark(skb);
    (*skb).priority = qos_class;
    (*OCELOT_SKB_CB(skb)).tstamp_lo = rew_val;

    dp = dsa_user_to_port((*skb).dev);
    vlan_tpid = if tag_type != 0 { ETH_P_8021AD } else { ETH_P_8021Q };

    if dsa_port_is_vlan_filtering(dp) && (*eth_hdr(skb)).h_proto == htons(vlan_tpid) {
        let mut dummy_vlan_tci: u16 = 0;

        skb_push_rcsum(skb, ETH_HLEN);
        __skb_vlan_pop(skb, &mut dummy_vlan_tci);
        skb_pull_rcsum(skb, ETH_HLEN);
        __vlan_hwaccel_put_tag(skb, htons(vlan_tpid), vlan_tci);
    }

    skb
}

// The following DSA registration declarations correspond to the C macros
// DSA_TAG_DRIVER, MODULE_ALIAS_DSA_TAG_DRIVER, and module_dsa_tag_drivers.
static ocelot_netdev_ops: dsa_device_ops = dsa_device_ops {
    name: OCELOT_NAME,
    proto: DSA_TAG_PROTO_OCELOT,
    xmit: ocelot_xmit,
    rcv: ocelot_rcv,
    needed_headroom: OCELOT_TOTAL_TAG_LEN,
    promisc_on_conduit: true,
};

static seville_netdev_ops: dsa_device_ops = dsa_device_ops {
    name: SEVILLE_NAME,
    proto: DSA_TAG_PROTO_SEVILLE,
    xmit: seville_xmit,
    rcv: ocelot_rcv,
    needed_headroom: OCELOT_TOTAL_TAG_LEN,
    promisc_on_conduit: true,
};

static ocelot_tag_driver_array: [*mut dsa_tag_driver; 2] = [
    &DSA_TAG_DRIVER_NAME(ocelot_netdev_ops),
    &DSA_TAG_DRIVER_NAME(seville_netdev_ops),
];

// MODULE_DESCRIPTION("DSA tag driver for Ocelot family of switches, using NPI port");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
