/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * NFC Digital Protocol stack
 * Copyright (c) 2013, Intel Corporation.
 */

// Dependencies supplied by the surrounding kernel translation.

/** Configuration types for in_configure_hw and tg_configure_hw. */
pub const NFC_DIGITAL_CONFIG_RF_TECH: i32 = 0;
pub const NFC_DIGITAL_CONFIG_FRAMING: i32 = 1;

/** RF technology values passed as param argument to configuration functions. */
pub const NFC_DIGITAL_RF_TECH_106A: i32 = 0;
pub const NFC_DIGITAL_RF_TECH_212F: i32 = 1;
pub const NFC_DIGITAL_RF_TECH_424F: i32 = 2;
pub const NFC_DIGITAL_RF_TECH_ISO15693: i32 = 3;
pub const NFC_DIGITAL_RF_TECH_106B: i32 = 4;
pub const NFC_DIGITAL_RF_TECH_LAST: i32 = 5;

/** Framing configuration values. */
pub const NFC_DIGITAL_FRAMING_NFCA_SHORT: i32 = 0;
pub const NFC_DIGITAL_FRAMING_NFCA_STANDARD: i32 = 1;
pub const NFC_DIGITAL_FRAMING_NFCA_STANDARD_WITH_CRC_A: i32 = 2;
pub const NFC_DIGITAL_FRAMING_NFCA_ANTICOL_COMPLETE: i32 = 3;
pub const NFC_DIGITAL_FRAMING_NFCA_T1T: i32 = 4;
pub const NFC_DIGITAL_FRAMING_NFCA_T2T: i32 = 5;
pub const NFC_DIGITAL_FRAMING_NFCA_T4T: i32 = 6;
pub const NFC_DIGITAL_FRAMING_NFCA_NFC_DEP: i32 = 7;
pub const NFC_DIGITAL_FRAMING_NFCF: i32 = 8;
pub const NFC_DIGITAL_FRAMING_NFCF_T3T: i32 = 9;
pub const NFC_DIGITAL_FRAMING_NFCF_NFC_DEP: i32 = 10;
pub const NFC_DIGITAL_FRAMING_NFC_DEP_ACTIVATED: i32 = 11;
pub const NFC_DIGITAL_FRAMING_ISO15693_INVENTORY: i32 = 12;
pub const NFC_DIGITAL_FRAMING_ISO15693_T5T: i32 = 13;
pub const NFC_DIGITAL_FRAMING_NFCB: i32 = 14;
pub const NFC_DIGITAL_FRAMING_NFCB_T4T: i32 = 15;
pub const NFC_DIGITAL_FRAMING_LAST: i32 = 16;

pub const DIGITAL_MDAA_NFCID1_SIZE: usize = 3;
pub const NFC_DIGITAL_POLL_MODE_COUNT_MAX: usize = 6; // 106A, 212F, and 424F in & tg
pub const NFC_DIGITAL_DRV_CAPS_IN_CRC: u32 = 0x0001;
pub const NFC_DIGITAL_DRV_CAPS_TG_CRC: u32 = 0x0002;

#[repr(C)]
pub struct digital_tg_mdaa_params {
    pub sens_res: u16,
    pub nfcid1: [u8; DIGITAL_MDAA_NFCID1_SIZE],
    pub sel_res: u8,
    pub nfcid2: [u8; NFC_NFCID2_MAXSIZE],
    pub sc: u16,
}

#[repr(C)]
pub struct nfc_digital_dev;

pub type nfc_digital_cmd_complete_t = unsafe extern "C" fn(
    ddev: *mut nfc_digital_dev,
    arg: *mut core::ffi::c_void,
    resp: *mut sk_buff,
);

#[repr(C)]
pub struct nfc_digital_ops {
    pub in_configure_hw: Option<unsafe extern "C" fn(*mut nfc_digital_dev, i32, i32) -> i32>,
    pub in_send_cmd: Option<unsafe extern "C" fn(*mut nfc_digital_dev, *mut sk_buff, u16, nfc_digital_cmd_complete_t, *mut core::ffi::c_void) -> i32>,
    pub tg_configure_hw: Option<unsafe extern "C" fn(*mut nfc_digital_dev, i32, i32) -> i32>,
    pub tg_send_cmd: Option<unsafe extern "C" fn(*mut nfc_digital_dev, *mut sk_buff, u16, nfc_digital_cmd_complete_t, *mut core::ffi::c_void) -> i32>,
    pub tg_listen: Option<unsafe extern "C" fn(*mut nfc_digital_dev, u16, nfc_digital_cmd_complete_t, *mut core::ffi::c_void) -> i32>,
    pub tg_listen_mdaa: Option<unsafe extern "C" fn(*mut nfc_digital_dev, *mut digital_tg_mdaa_params, u16, nfc_digital_cmd_complete_t, *mut core::ffi::c_void) -> i32>,
    pub tg_listen_md: Option<unsafe extern "C" fn(*mut nfc_digital_dev, u16, nfc_digital_cmd_complete_t, *mut core::ffi::c_void) -> i32>,
    pub tg_get_rf_tech: Option<unsafe extern "C" fn(*mut nfc_digital_dev, *mut u8) -> i32>,
    pub switch_rf: Option<unsafe extern "C" fn(*mut nfc_digital_dev, bool) -> i32>,
    pub abort_cmd: Option<unsafe extern "C" fn(*mut nfc_digital_dev)>,
}

pub type digital_poll_t = unsafe extern "C" fn(*mut nfc_digital_dev, u8) -> i32;

#[repr(C)]
pub struct digital_poll_tech {
    pub rf_tech: u8,
    pub poll_func: Option<digital_poll_t>,
}

#[repr(C)]
pub struct nfc_digital_dev {
    pub nfc_dev: *mut nfc_dev,
    pub ops: *const nfc_digital_ops,
    pub protocols: u32,
    pub tx_headroom: i32,
    pub tx_tailroom: i32,
    pub driver_capabilities: u32,
    pub driver_data: *mut core::ffi::c_void,
    pub poll_techs: [digital_poll_tech; NFC_DIGITAL_POLL_MODE_COUNT_MAX],
    pub poll_tech_count: u8,
    pub poll_tech_index: u8,
    pub poll_lock: mutex,
    pub cmd_work: work_struct,
    pub cmd_complete_work: work_struct,
    pub cmd_queue: list_head,
    pub cmd_lock: mutex,
    pub poll_work: delayed_work,
    pub curr_protocol: u8,
    pub curr_rf_tech: u8,
    pub curr_nfc_dep_pni: u8,
    pub did: u8,
    pub dep_rwt: u16,
    pub local_payload_max: u8,
    pub remote_payload_max: u8,
    pub chaining_skb: *mut sk_buff,
    pub data_exch: *mut digital_data_exch,
    pub atn_count: i32,
    pub nack_count: i32,
    pub saved_skb: *mut sk_buff,
    pub target_fsc: u16,
    pub skb_check_crc: Option<unsafe extern "C" fn(*mut sk_buff) -> i32>,
    pub skb_add_crc: Option<unsafe extern "C" fn(*mut sk_buff)>,
}

unsafe extern "C" {
    pub fn nfc_digital_allocate_device(ops: *const nfc_digital_ops, supported_protocols: u32, driver_capabilities: u32, tx_headroom: i32, tx_tailroom: i32) -> *mut nfc_digital_dev;
    pub fn nfc_digital_free_device(ndev: *mut nfc_digital_dev);
    pub fn nfc_digital_register_device(ndev: *mut nfc_digital_dev) -> i32;
    pub fn nfc_digital_unregister_device(ndev: *mut nfc_digital_dev);
}

#[inline]
pub unsafe fn nfc_digital_set_parent_dev(ndev: *mut nfc_digital_dev, dev: *mut device) {
    nfc_set_parent_dev((*ndev).nfc_dev, dev);
}

#[inline]
pub unsafe fn nfc_digital_set_drvdata(dev: *mut nfc_digital_dev, data: *mut core::ffi::c_void) {
    (*dev).driver_data = data;
}

#[inline]
pub unsafe fn nfc_digital_get_drvdata(dev: *mut nfc_digital_dev) -> *mut core::ffi::c_void {
    (*dev).driver_data
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
