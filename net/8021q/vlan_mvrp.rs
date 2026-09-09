// SPDX-License-Identifier: GPL-2.0-only
/*
 *	IEEE 802.1Q Multiple VLAN Registration Protocol (MVRP)
 *
 *	Copyright (c) 2012 Massachusetts Institute of Technology
 *
 *	Adapted from code in net/8021q/vlan_gvrp.c
 *	Copyright (c) 2008 Patrick McHardy <kaber@trash.net>
 */

// Linux and local headers from the original translation unit provide these
// types, constants, and functions.

const MRP_MVRP_ADDRESS: [u8; 6] = [0x01, 0x80, 0xc2, 0x00, 0x00, 0x21];

#[repr(i32)]
enum mvrp_attributes {
    MVRP_ATTR_INVALID,
    MVRP_ATTR_VID,
    __MVRP_ATTR_MAX,
}

const MVRP_ATTR_MAX: i32 = (__MVRP_ATTR_MAX as i32) - 1;

static mut vlan_mrp_app: mrp_application = mrp_application {
    type_: MRP_APPLICATION_MVRP,
    maxattr: MVRP_ATTR_MAX,
    pkttype: mrp_application_pkttype { type_: htons(ETH_P_MVRP) },
    group_address: MRP_MVRP_ADDRESS,
    version: 0,
};

pub unsafe fn vlan_mvrp_request_join(dev: *const net_device) -> i32 {
    let vlan: *const vlan_dev_priv = vlan_dev_priv(dev);
    let vlan_id: __be16 = htons((*vlan).vlan_id);

    if (*vlan).vlan_proto != htons(ETH_P_8021Q) {
        return 0;
    }
    mrp_request_join(
        (*vlan).real_dev,
        &raw mut vlan_mrp_app,
        &vlan_id,
        core::mem::size_of::<__be16>(),
        MVRP_ATTR_VID,
    )
}

pub unsafe fn vlan_mvrp_request_leave(dev: *const net_device) {
    let vlan: *const vlan_dev_priv = vlan_dev_priv(dev);
    let vlan_id: __be16 = htons((*vlan).vlan_id);

    if (*vlan).vlan_proto != htons(ETH_P_8021Q) {
        return;
    }
    mrp_request_leave(
        (*vlan).real_dev,
        &raw mut vlan_mrp_app,
        &vlan_id,
        core::mem::size_of::<__be16>(),
        MVRP_ATTR_VID,
    );
}

pub unsafe fn vlan_mvrp_init_applicant(dev: *mut net_device) -> i32 {
    mrp_init_applicant(dev, &raw mut vlan_mrp_app)
}

pub unsafe fn vlan_mvrp_uninit_applicant(dev: *mut net_device) {
    mrp_uninit_applicant(dev, &raw mut vlan_mrp_app);
}

pub unsafe fn vlan_mvrp_init() -> i32 {
    mrp_register_application(&raw mut vlan_mrp_app)
}

pub unsafe fn vlan_mvrp_uninit() {
    mrp_unregister_application(&raw mut vlan_mrp_app);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
