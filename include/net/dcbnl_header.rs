/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2008, Intel Corporation.
 *
 * Author: Lucy Liu <lucy.liu@intel.com>
 */

// Translated from <linux/dcbnl.h>; dependent declarations are supplied elsewhere.

use core::ffi::c_int;

pub enum net_device {}

#[repr(C)]
pub struct dcb_app_type {
    pub ifindex: c_int,
    pub app: dcb_app,
    pub list: list_head,
    pub dcbx: u8,
}

pub unsafe extern "C" {
    pub fn dcb_getrewr(dev: *mut net_device, app: *mut dcb_app) -> u16;
    pub fn dcb_setrewr(dev: *mut net_device, app: *mut dcb_app) -> c_int;
    pub fn dcb_delrewr(dev: *mut net_device, app: *mut dcb_app) -> c_int;

    pub fn dcb_setapp(dev: *mut net_device, app: *mut dcb_app) -> c_int;
    pub fn dcb_getapp(dev: *mut net_device, app: *mut dcb_app) -> u8;
    pub fn dcb_ieee_setapp(dev: *mut net_device, app: *mut dcb_app) -> c_int;
    pub fn dcb_ieee_delapp(dev: *mut net_device, app: *mut dcb_app) -> c_int;
    pub fn dcb_ieee_getapp_mask(dev: *mut net_device, app: *mut dcb_app) -> u8;
}

#[repr(C)]
pub struct dcb_rewr_prio_pcp_map {
    pub map: [u16; IEEE_8021QAZ_MAX_TCS as usize],
}

pub unsafe extern "C" {
    pub fn dcb_getrewr_prio_pcp_mask_map(
        dev: *const net_device,
        p_map: *mut dcb_rewr_prio_pcp_map,
    );
}

#[repr(C)]
pub struct dcb_ieee_app_prio_map {
    pub map: [u64; IEEE_8021QAZ_MAX_TCS as usize],
}

pub unsafe extern "C" {
    pub fn dcb_ieee_getapp_prio_dscp_mask_map(
        dev: *const net_device,
        p_map: *mut dcb_ieee_app_prio_map,
    );

    pub fn dcb_getrewr_prio_dscp_mask_map(
        dev: *const net_device,
        p_map: *mut dcb_ieee_app_prio_map,
    );
}

#[repr(C)]
pub struct dcb_ieee_app_dscp_map {
    pub map: [u8; 64],
}

pub unsafe extern "C" {
    pub fn dcb_ieee_getapp_dscp_prio_mask_map(
        dev: *const net_device,
        p_map: *mut dcb_ieee_app_dscp_map,
    );
    pub fn dcb_ieee_getapp_default_prio_mask(dev: *const net_device) -> u8;

    pub fn dcbnl_ieee_notify(
        dev: *mut net_device,
        event: c_int,
        cmd: c_int,
        seq: u32,
        pid: u32,
    ) -> c_int;
    pub fn dcbnl_cee_notify(
        dev: *mut net_device,
        event: c_int,
        cmd: c_int,
        seq: u32,
        pid: u32,
    ) -> c_int;
}

/*
 * Ops struct for the netlink callbacks.  Used by DCB-enabled drivers through
 * the netdevice struct.
 */
#[repr(C)]
pub struct dcbnl_rtnl_ops {
    /* IEEE 802.1Qaz std */
    pub ieee_getets: Option<unsafe extern "C" fn(*mut net_device, *mut ieee_ets) -> c_int>,
    pub ieee_setets: Option<unsafe extern "C" fn(*mut net_device, *mut ieee_ets) -> c_int>,
    pub ieee_getmaxrate: Option<unsafe extern "C" fn(*mut net_device, *mut ieee_maxrate) -> c_int>,
    pub ieee_setmaxrate: Option<unsafe extern "C" fn(*mut net_device, *mut ieee_maxrate) -> c_int>,
    pub ieee_getqcn: Option<unsafe extern "C" fn(*mut net_device, *mut ieee_qcn) -> c_int>,
    pub ieee_setqcn: Option<unsafe extern "C" fn(*mut net_device, *mut ieee_qcn) -> c_int>,
    pub ieee_getqcnstats: Option<unsafe extern "C" fn(*mut net_device, *mut ieee_qcn_stats) -> c_int>,
    pub ieee_getpfc: Option<unsafe extern "C" fn(*mut net_device, *mut ieee_pfc) -> c_int>,
    pub ieee_setpfc: Option<unsafe extern "C" fn(*mut net_device, *mut ieee_pfc) -> c_int>,
    pub ieee_getapp: Option<unsafe extern "C" fn(*mut net_device, *mut dcb_app) -> c_int>,
    pub ieee_setapp: Option<unsafe extern "C" fn(*mut net_device, *mut dcb_app) -> c_int>,
    pub ieee_delapp: Option<unsafe extern "C" fn(*mut net_device, *mut dcb_app) -> c_int>,
    pub ieee_peer_getets: Option<unsafe extern "C" fn(*mut net_device, *mut ieee_ets) -> c_int>,
    pub ieee_peer_getpfc: Option<unsafe extern "C" fn(*mut net_device, *mut ieee_pfc) -> c_int>,

    /* CEE std */
    pub getstate: Option<unsafe extern "C" fn(*mut net_device) -> u8>,
    pub setstate: Option<unsafe extern "C" fn(*mut net_device, u8) -> u8>,
    pub getpermhwaddr: Option<unsafe extern "C" fn(*mut net_device, *mut u8)>,
    pub setpgtccfgtx: Option<unsafe extern "C" fn(*mut net_device, c_int, u8, u8, u8, u8)>,
    pub setpgbwgcfgtx: Option<unsafe extern "C" fn(*mut net_device, c_int, u8)>,
    pub setpgtccfgrx: Option<unsafe extern "C" fn(*mut net_device, c_int, u8, u8, u8, u8)>,
    pub setpgbwgcfgrx: Option<unsafe extern "C" fn(*mut net_device, c_int, u8)>,
    pub getpgtccfgtx: Option<unsafe extern "C" fn(*mut net_device, c_int, *mut u8, *mut u8, *mut u8, *mut u8)>,
    pub getpgbwgcfgtx: Option<unsafe extern "C" fn(*mut net_device, c_int, *mut u8)>,
    pub getpgtccfgrx: Option<unsafe extern "C" fn(*mut net_device, c_int, *mut u8, *mut u8, *mut u8, *mut u8)>,
    pub getpgbwgcfgrx: Option<unsafe extern "C" fn(*mut net_device, c_int, *mut u8)>,
    pub setpfccfg: Option<unsafe extern "C" fn(*mut net_device, c_int, u8)>,
    pub getpfccfg: Option<unsafe extern "C" fn(*mut net_device, c_int, *mut u8)>,
    pub setall: Option<unsafe extern "C" fn(*mut net_device) -> u8>,
    pub getcap: Option<unsafe extern "C" fn(*mut net_device, c_int, *mut u8) -> u8>,
    pub getnumtcs: Option<unsafe extern "C" fn(*mut net_device, c_int, *mut u8) -> c_int>,
    pub setnumtcs: Option<unsafe extern "C" fn(*mut net_device, c_int, u8) -> c_int>,
    pub getpfcstate: Option<unsafe extern "C" fn(*mut net_device) -> u8>,
    pub setpfcstate: Option<unsafe extern "C" fn(*mut net_device, u8)>,
    pub getbcncfg: Option<unsafe extern "C" fn(*mut net_device, c_int, *mut u32)>,
    pub setbcncfg: Option<unsafe extern "C" fn(*mut net_device, c_int, u32)>,
    pub getbcnrp: Option<unsafe extern "C" fn(*mut net_device, c_int, *mut u8)>,
    pub setbcnrp: Option<unsafe extern "C" fn(*mut net_device, c_int, u8)>,
    pub setapp: Option<unsafe extern "C" fn(*mut net_device, u8, u16, u8) -> c_int>,
    pub getapp: Option<unsafe extern "C" fn(*mut net_device, u8, u16) -> c_int>,
    pub getfeatcfg: Option<unsafe extern "C" fn(*mut net_device, c_int, *mut u8) -> u8>,
    pub setfeatcfg: Option<unsafe extern "C" fn(*mut net_device, c_int, u8) -> u8>,

    /* DCBX configuration */
    pub getdcbx: Option<unsafe extern "C" fn(*mut net_device) -> u8>,
    pub setdcbx: Option<unsafe extern "C" fn(*mut net_device, u8) -> u8>,

    /* peer apps */
    pub peer_getappinfo: Option<unsafe extern "C" fn(*mut net_device, *mut dcb_peer_app_info, *mut u16) -> c_int>,
    pub peer_getapptable: Option<unsafe extern "C" fn(*mut net_device, *mut dcb_app) -> c_int>,

    /* CEE peer */
    pub cee_peer_getpg: Option<unsafe extern "C" fn(*mut net_device, *mut cee_pg) -> c_int>,
    pub cee_peer_getpfc: Option<unsafe extern "C" fn(*mut net_device, *mut cee_pfc) -> c_int>,

    /* buffer settings */
    pub dcbnl_getbuffer: Option<unsafe extern "C" fn(*mut net_device, *mut dcbnl_buffer) -> c_int>,
    pub dcbnl_setbuffer: Option<unsafe extern "C" fn(*mut net_device, *mut dcbnl_buffer) -> c_int>,

    /* apptrust */
    pub dcbnl_setapptrust: Option<unsafe extern "C" fn(*mut net_device, *mut u8, c_int) -> c_int>,
    pub dcbnl_getapptrust: Option<unsafe extern "C" fn(*mut net_device, *mut u8, *mut c_int) -> c_int>,

    /* rewrite */
    pub dcbnl_setrewr: Option<unsafe extern "C" fn(*mut net_device, *mut dcb_app) -> c_int>,
    pub dcbnl_delrewr: Option<unsafe extern "C" fn(*mut net_device, *mut dcb_app) -> c_int>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
