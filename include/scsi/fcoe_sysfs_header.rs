/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2011-2012 Intel Corporation.  All rights reserved.
 *
 * Maintained at www.Open-FCoE.org
 */

// C dependencies: linux/if_ether.h, linux/device.h, scsi/fc/fc_fcoe.h

use core::ffi::c_void;

#[repr(C)]
pub struct fcoe_sysfs_function_template {
    pub get_fcoe_ctlr_link_fail: Option<unsafe extern "C" fn(*mut fcoe_ctlr_device)>,
    pub get_fcoe_ctlr_vlink_fail: Option<unsafe extern "C" fn(*mut fcoe_ctlr_device)>,
    pub get_fcoe_ctlr_miss_fka: Option<unsafe extern "C" fn(*mut fcoe_ctlr_device)>,
    pub get_fcoe_ctlr_symb_err: Option<unsafe extern "C" fn(*mut fcoe_ctlr_device)>,
    pub get_fcoe_ctlr_err_block: Option<unsafe extern "C" fn(*mut fcoe_ctlr_device)>,
    pub get_fcoe_ctlr_fcs_error: Option<unsafe extern "C" fn(*mut fcoe_ctlr_device)>,
    pub set_fcoe_ctlr_mode: Option<unsafe extern "C" fn(*mut fcoe_ctlr_device)>,
    pub set_fcoe_ctlr_enabled:
        Option<unsafe extern "C" fn(*mut fcoe_ctlr_device) -> core::ffi::c_int>,
    pub get_fcoe_fcf_selected: Option<unsafe extern "C" fn(*mut fcoe_fcf_device)>,
    pub get_fcoe_fcf_vlan_id: Option<unsafe extern "C" fn(*mut fcoe_fcf_device)>,
}

// #define dev_to_ctlr(d) container_of((d), struct fcoe_ctlr_device, dev)

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fip_conn_type {
    FIP_CONN_TYPE_UNKNOWN,
    FIP_CONN_TYPE_FABRIC,
    FIP_CONN_TYPE_VN2VN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ctlr_enabled_state {
    FCOE_CTLR_ENABLED,
    FCOE_CTLR_DISABLED,
    FCOE_CTLR_UNUSED,
}

#[repr(C)]
pub struct fcoe_ctlr_device {
    pub id: u32,
    pub dev: device,
    pub f: *mut fcoe_sysfs_function_template,
    pub fcfs: list_head,
    pub work_q: *mut workqueue_struct,
    pub devloss_work_q: *mut workqueue_struct,
    pub lock: mutex,
    pub fcf_dev_loss_tmo: core::ffi::c_int,
    pub mode: fip_conn_type,
    pub enabled: ctlr_enabled_state,
    /* expected in host order for displaying */
    pub lesb: fcoe_fc_els_lesb,
}

#[inline]
pub unsafe fn fcoe_ctlr_device_priv(ctlr: *const fcoe_ctlr_device) -> *mut c_void {
    ctlr.add(1) as *mut c_void
}

/* fcf states */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fcf_state {
    FCOE_FCF_STATE_UNKNOWN,
    FCOE_FCF_STATE_DISCONNECTED,
    FCOE_FCF_STATE_CONNECTED,
    FCOE_FCF_STATE_DELETED,
}

#[repr(C)]
pub struct fcoe_fcf_device {
    pub id: u32,
    pub dev: device,
    pub peers: list_head,
    pub delete_work: work_struct,
    pub dev_loss_work: delayed_work,
    pub dev_loss_tmo: u32,
    pub r#priv: *mut c_void,
    pub state: fcf_state,
    pub fabric_name: u64,
    pub switch_name: u64,
    pub fc_map: u32,
    pub vfid: u16,
    pub mac: [u8; ETH_ALEN],
    pub priority: u8,
    pub fka_period: u32,
    pub selected: u8,
    pub vlan_id: u16,
}

// #define dev_to_fcf(d) container_of((d), struct fcoe_fcf_device, dev)
/* parentage should never be missing */
// #define fcoe_fcf_dev_to_ctlr_dev(x) dev_to_ctlr((x)->dev.parent)
// #define fcoe_fcf_device_priv(x) ((x)->priv)

extern "C" {
    pub fn fcoe_ctlr_device_add(
        parent: *mut device,
        f: *mut fcoe_sysfs_function_template,
        priv_size: core::ffi::c_int,
    ) -> *mut fcoe_ctlr_device;
    pub fn fcoe_ctlr_device_delete(ctlr: *mut fcoe_ctlr_device);
    pub fn fcoe_fcf_device_add(
        ctlr: *mut fcoe_ctlr_device,
        fcf: *mut fcoe_fcf_device,
    ) -> *mut fcoe_fcf_device;
    pub fn fcoe_fcf_device_delete(fcf: *mut fcoe_fcf_device);
    pub fn fcoe_sysfs_setup() -> core::ffi::c_int;
    pub fn fcoe_sysfs_teardown();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
