/* Broadcom NetXtreme-C/E network driver.
 *
 * Copyright (c) 2016-2018 Broadcom Limited
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation.
 */

use core::ffi::c_void;

// Dependency supplied by the Linux auxiliary-bus implementation.
#[repr(C)]
pub struct auxiliary_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hwrm_async_event_cmpl {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bnxt {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

pub const BNXT_MIN_ROCE_CP_RINGS: i32 = 2;
pub const BNXT_MIN_ROCE_STAT_CTXS: i32 = 1;
pub const BNXT_MAX_ROCE_MSIX_VF: i32 = 2;
pub const BNXT_MAX_ROCE_MSIX_NPAR_PF: i32 = 5;
pub const BNXT_MAX_ROCE_MSIX: usize = 64;

#[repr(C)]
#[derive Copy, Clone, PartialEq, Eq)]
pub enum bnxt_auxdev_type {
    BNXT_AUXDEV_RDMA = 0,
    BNXT_AUXDEV_FWCTL,
    __BNXT_AUXDEV_MAX,
}

#[repr(C)]
pub struct bnxt_aux_priv {
    pub aux_dev: auxiliary_device,
    pub edev: *mut bnxt_en_dev,
    pub id: i32,
}

#[repr(C)]
pub struct bnxt_msix_entry {
    pub vector: u32,
    pub ring_idx: u32,
    pub db_offset: u32,
}

#[repr(C)]
pub struct bnxt_ulp_ops {
    /* async_notifier() cannot sleep (in BH context) */
    pub ulp_async_notifier:
        Option<unsafe extern "C" fn(*mut c_void, *mut hwrm_async_event_cmpl)>,
    pub ulp_irq_stop: Option<unsafe extern "C" fn(*mut c_void, bool)>,
    pub ulp_irq_restart: Option<unsafe extern "C" fn(*mut c_void, *mut bnxt_msix_entry)>,
}

#[repr(C)]
pub struct bnxt_fw_msg {
    pub msg: *mut c_void,
    pub msg_len: i32,
    pub resp: *mut c_void,
    pub resp_max_len: i32,
    pub timeout: i32,
}

#[repr(C)]
pub struct bnxt_ulp {
    pub handle: *mut c_void,
    pub ulp_ops: *mut bnxt_ulp_ops,
    pub async_events_bmap: *mut usize,
    pub max_async_event_id: u16,
    pub msix_requested: u16,
}

#[repr(C)]
pub struct bnxt_en_dev {
    pub net: *mut net_device,
    pub pdev: *mut pci_dev,
    pub msix_entries: [bnxt_msix_entry; BNXT_MAX_ROCE_MSIX],
    pub flags: u32,
    pub ulp_tbl: *mut bnxt_ulp,
    pub l2_db_size: i32, // Doorbell BAR size in bytes mapped by L2 driver.
    pub l2_db_size_nc: i32, // Doorbell BAR size in bytes mapped as non-cacheable.
    pub l2_db_offset: i32, // Doorbell offset in bytes within l2_db_size_nc.
    pub chip_num: u16,
    pub hw_ring_stats_size: u16,
    pub pf_port_id: u16,
    pub en_state: usize, // Could be checked in RoCE driver suspend mode only. Will be updated in resume.
    pub bar0: *mut c_void,
    pub ulp_num_msix_vec: u16,
    pub ulp_num_ctxs: u16,
    // serialize ulp operations
    pub en_dev_lock: mutex,
}

pub const BNXT_EN_FLAG_ROCEV1_CAP: u32 = 0x1;
pub const BNXT_EN_FLAG_ROCEV2_CAP: u32 = 0x2;
pub const BNXT_EN_FLAG_ROCE_CAP: u32 = BNXT_EN_FLAG_ROCEV1_CAP | BNXT_EN_FLAG_ROCEV2_CAP;
pub const BNXT_EN_FLAG_ULP_STOPPED: u32 = 0x8;
pub const BNXT_EN_FLAG_VF: u32 = 0x10;
pub const BNXT_EN_FLAG_ROCE_VF_RES_MGMT: u32 = 0x20;
pub const BNXT_EN_FLAG_SW_RES_LMT: u32 = 0x40;

#[inline]
pub unsafe fn BNXT_EN_VF(edev: *const bnxt_en_dev) -> u32 {
    (*edev).flags & BNXT_EN_FLAG_VF
}

#[inline]
pub unsafe fn BNXT_EN_SW_RES_LMT(edev: *const bnxt_en_dev) -> u32 {
    (*edev).flags & BNXT_EN_FLAG_SW_RES_LMT
}

extern "C" {
    pub fn rcu_access_pointer(ulp_ops: *mut bnxt_ulp_ops) -> *mut bnxt_ulp_ops;

    pub fn bnxt_get_ulp_msix_num(bp: *mut bnxt) -> i32;
    pub fn bnxt_get_ulp_msix_num_in_use(bp: *mut bnxt) -> i32;
    pub fn bnxt_set_ulp_msix_num(bp: *mut bnxt, num: i32);
    pub fn bnxt_get_ulp_stat_ctxs(bp: *mut bnxt) -> i32;
    pub fn bnxt_set_ulp_stat_ctxs(bp: *mut bnxt, num_ctxs: i32);
    pub fn bnxt_get_ulp_stat_ctxs_in_use(bp: *mut bnxt) -> i32;
    pub fn bnxt_set_dflt_ulp_stat_ctxs(bp: *mut bnxt);
    pub fn bnxt_ulp_stop(bp: *mut bnxt);
    pub fn bnxt_ulp_start(bp: *mut bnxt);
    pub fn bnxt_ulp_sriov_cfg(bp: *mut bnxt, num_vfs: i32);
    pub fn bnxt_ulp_irq_stop(bp: *mut bnxt);
    pub fn bnxt_ulp_irq_restart(bp: *mut bnxt, err: i32);
    pub fn bnxt_ulp_async_events(bp: *mut bnxt, cmpl: *mut hwrm_async_event_cmpl);
    pub fn bnxt_aux_devices_uninit(bp: *mut bnxt);
    pub fn bnxt_aux_devices_del(bp: *mut bnxt);
    pub fn bnxt_aux_devices_add(bp: *mut bnxt);
    pub fn bnxt_aux_devices_init(bp: *mut bnxt);
    pub fn bnxt_register_dev(edev: *mut bnxt_en_dev, ulp_ops: *mut bnxt_ulp_ops, handle: *mut c_void) -> i32;
    pub fn bnxt_unregister_dev(edev: *mut bnxt_en_dev);
    pub fn bnxt_send_msg(edev: *mut bnxt_en_dev, fw_msg: *mut bnxt_fw_msg) -> i32;
    pub fn bnxt_register_async_events(edev: *mut bnxt_en_dev, events_bmap: *mut usize, max_id: u16);
    pub fn bnxt_auxdev_id_alloc(bp: *mut bnxt) -> i32;
    pub fn bnxt_auxdev_id_free(bp: *mut bnxt, id: i32);
}

#[inline]
pub unsafe fn bnxt_ulp_registered(edev: *mut bnxt_en_dev) -> bool {
    !edev.is_null() && !(*(*edev).ulp_tbl).ulp_ops.is_null()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
