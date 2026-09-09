// SPDX-License-Identifier: GPL-2.0-only
/*
 * IEEE 802.1Q GARP VLAN Registration Protocol (GVRP)
 *
 * Copyright (c) 2008 Patrick McHardy <kaber@trash.net>
 */

// Dependencies supplied by the surrounding kernel translation.

const GARP_GVRP_ADDRESS: [u8; 6] = [0x01, 0x80, 0xc2, 0x00, 0x00, 0x21];

#[repr(C)]
pub struct NetDevice {
    _private: [u8; 0],
}

#[repr(C)]
pub struct VlanDevPriv {
    pub vlan_id: u16,
    pub vlan_proto: u16,
    pub real_dev: *mut NetDevice,
}

#[repr(C)]
pub struct GarpProto {
    pub group_address: [u8; 6],
}

#[repr(C)]
pub struct GarpApplication {
    pub proto: GarpProto,
    pub maxattr: i32,
    pub r#type: i32,
}

#[repr(C)]
pub struct GarpApplicationOpaque {
    _private: [u8; 0],
}

extern "C" {
    fn vlan_dev_priv(dev: *const NetDevice) -> *const VlanDevPriv;
    fn garp_request_join(
        dev: *mut NetDevice,
        app: *mut GarpApplication,
        value: *const u8,
        len: usize,
        attr: i32,
    ) -> i32;
    fn garp_request_leave(
        dev: *mut NetDevice,
        app: *mut GarpApplication,
        value: *const u8,
        len: usize,
        attr: i32,
    );
    fn garp_init_applicant(dev: *mut NetDevice, app: *mut GarpApplication) -> i32;
    fn garp_uninit_applicant(dev: *mut NetDevice, app: *mut GarpApplication);
    fn garp_register_application(app: *mut GarpApplication) -> i32;
    fn garp_unregister_application(app: *mut GarpApplication);
}

const ETH_P_8021Q: u16 = 0x8100;
const GARP_APPLICATION_GVRP: i32 = 0;

#[repr(i32)]
enum GvrpAttributes {
    GvrpAttrInvalid,
    GvrpAttrVid,
    _GvrpAttrMax,
}

const GVRP_ATTR_MAX: i32 = GvrpAttributes::_GvrpAttrMax as i32 - 1;
const GVRP_ATTR_VID: i32 = GvrpAttributes::GvrpAttrVid as i32;

static mut VLAN_GVRP_APP: GarpApplication = GarpApplication {
    proto: GarpProto {
        group_address: GARP_GVRP_ADDRESS,
    },
    maxattr: GVRP_ATTR_MAX,
    r#type: GARP_APPLICATION_GVRP,
};

#[inline]
unsafe fn htons(value: u16) -> u16 {
    value.to_be()
}

pub unsafe fn vlan_gvrp_request_join(dev: *const NetDevice) -> i32 {
    let vlan = &*vlan_dev_priv(dev);
    let vlan_id: u16 = htons(vlan.vlan_id);

    if vlan.vlan_proto != htons(ETH_P_8021Q) {
        return 0;
    }
    garp_request_join(
        vlan.real_dev,
        &raw mut VLAN_GVRP_APP,
        &vlan_id as *const u16 as *const u8,
        core::mem::size_of::<u16>(),
        GVRP_ATTR_VID,
    )
}

pub unsafe fn vlan_gvrp_request_leave(dev: *const NetDevice) {
    let vlan = &*vlan_dev_priv(dev);
    let vlan_id: u16 = htons(vlan.vlan_id);

    if vlan.vlan_proto != htons(ETH_P_8021Q) {
        return;
    }
    garp_request_leave(
        vlan.real_dev,
        &raw mut VLAN_GVRP_APP,
        &vlan_id as *const u16 as *const u8,
        core::mem::size_of::<u16>(),
        GVRP_ATTR_VID,
    );
}

pub unsafe fn vlan_gvrp_init_applicant(dev: *mut NetDevice) -> i32 {
    garp_init_applicant(dev, &raw mut VLAN_GVRP_APP)
}

pub unsafe fn vlan_gvrp_uninit_applicant(dev: *mut NetDevice) {
    garp_uninit_applicant(dev, &raw mut VLAN_GVRP_APP);
}

pub unsafe fn vlan_gvrp_init() -> i32 {
    garp_register_application(&raw mut VLAN_GVRP_APP)
}

pub unsafe fn vlan_gvrp_uninit() {
    garp_unregister_application(&raw mut VLAN_GVRP_APP);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
