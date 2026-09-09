/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2011 Instituto Nokia de Tecnologia
 * Copyright (C) 2014 Marvell International Ltd.
 */

// Dependencies supplied by the corresponding kernel headers remain external.
// nfc_dbg/nfc_info/nfc_err expand to the kernel device logging macros.

#[repr(C)]
pub struct nfc_phy_ops {
    pub write: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut sk_buff) -> i32>,
    pub enable: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
    pub disable: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
}

pub type data_exchange_cb_t = Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut sk_buff, i32)>;
pub type se_io_cb_t = Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut u8, usize, i32)>;

#[repr(C)]
pub struct nfc_ops {
    pub dev_up: Option<unsafe extern "C" fn(*mut nfc_dev) -> i32>,
    pub dev_down: Option<unsafe extern "C" fn(*mut nfc_dev) -> i32>,
    pub start_poll: Option<unsafe extern "C" fn(*mut nfc_dev, u32, u32) -> i32>,
    pub stop_poll: Option<unsafe extern "C" fn(*mut nfc_dev)>,
    pub dep_link_up: Option<unsafe extern "C" fn(*mut nfc_dev, *mut nfc_target, u8, *mut u8, usize) -> i32>,
    pub dep_link_down: Option<unsafe extern "C" fn(*mut nfc_dev) -> i32>,
    pub activate_target: Option<unsafe extern "C" fn(*mut nfc_dev, *mut nfc_target, u32) -> i32>,
    pub deactivate_target: Option<unsafe extern "C" fn(*mut nfc_dev, *mut nfc_target, u8)>,
    pub im_transceive: Option<unsafe extern "C" fn(*mut nfc_dev, *mut nfc_target, *mut sk_buff, data_exchange_cb_t, *mut core::ffi::c_void) -> i32>,
    pub tm_send: Option<unsafe extern "C" fn(*mut nfc_dev, *mut sk_buff) -> i32>,
    pub check_presence: Option<unsafe extern "C" fn(*mut nfc_dev, *mut nfc_target) -> i32>,
    pub fw_download: Option<unsafe extern "C" fn(*mut nfc_dev, *const core::ffi::c_char) -> i32>,
    pub discover_se: Option<unsafe extern "C" fn(*mut nfc_dev) -> i32>,
    pub enable_se: Option<unsafe extern "C" fn(*mut nfc_dev, u32) -> i32>,
    pub disable_se: Option<unsafe extern "C" fn(*mut nfc_dev, u32) -> i32>,
    pub se_io: Option<unsafe extern "C" fn(*mut nfc_dev, u32, *mut u8, usize, se_io_cb_t, *mut core::ffi::c_void) -> i32>,
}

pub const NFC_TARGET_IDX_ANY: i32 = -1;
pub const NFC_MAX_GT_LEN: i32 = 48;
pub const NFC_ATR_RES_GT_OFFSET: i32 = 15;
pub const NFC_ATR_REQ_GT_OFFSET: i32 = 14;

#[repr(C)]
pub struct nfc_target {
    pub idx: u32,
    pub supported_protocols: u32,
    pub sens_res: u16,
    pub sel_res: u8,
    pub nfcid1_len: u8,
    pub nfcid1: [u8; NFC_NFCID1_MAXSIZE as usize],
    pub nfcid2_len: u8,
    pub nfcid2: [u8; NFC_NFCID2_MAXSIZE as usize],
    pub sensb_res_len: u8,
    pub sensb_res: [u8; NFC_SENSB_RES_MAXSIZE as usize],
    pub sensf_res_len: u8,
    pub sensf_res: [u8; NFC_SENSF_RES_MAXSIZE as usize],
    pub hci_reader_gate: u8,
    pub logical_idx: u8,
    pub is_iso15693: u8,
    pub iso15693_dsfid: u8,
    pub iso15693_uid: [u8; NFC_ISO15693_UID_MAXSIZE as usize],
    pub ats_len: u8,
    pub ats: [u8; NFC_ATS_MAXSIZE as usize],
}

#[repr(C)]
pub struct nfc_se {
    pub list: list_head,
    pub idx: u32,
    pub type_: u16,
    pub state: u16,
}

pub const NFC_MIN_AID_LENGTH: u32 = 5;
pub const NFC_MAX_AID_LENGTH: u32 = 16;
pub const NFC_MAX_PARAMS_LENGTH: u32 = 255;
pub const NFC_EVT_TRANSACTION_AID_TAG: u32 = 0x81;
pub const NFC_EVT_TRANSACTION_PARAMS_TAG: u32 = 0x82;

#[repr(C, packed)]
pub struct nfc_evt_transaction {
    pub aid_len: u32,
    pub aid: [u8; NFC_MAX_AID_LENGTH as usize],
    pub params_len: u8,
    pub params: [u8; 0],
}

#[repr(C)]
pub struct nfc_genl_data {
    pub poll_req_portid: u32,
    pub genl_data_mutex: mutex,
}

#[repr(C)]
pub struct nfc_vendor_cmd {
    pub vendor_id: __u32,
    pub subcmd: __u32,
    pub doit: Option<unsafe extern "C" fn(*mut nfc_dev, *mut core::ffi::c_void, usize) -> i32>,
}

#[repr(C)]
pub struct nfc_dev {
    pub idx: i32,
    pub target_next_idx: u32,
    pub targets: *mut nfc_target,
    pub n_targets: i32,
    pub targets_generation: i32,
    pub dev: device,
    pub dev_up: bool,
    pub fw_download_in_progress: bool,
    pub rf_mode: u8,
    pub polling: bool,
    pub active_target: *mut nfc_target,
    pub dep_link_up: bool,
    pub genl_data: nfc_genl_data,
    pub supported_protocols: u32,
    pub secure_elements: list_head,
    pub tx_headroom: i32,
    pub tx_tailroom: i32,
    pub check_pres_timer: timer_list,
    pub check_pres_work: work_struct,
    pub shutting_down: bool,
    pub rfkill: *mut rfkill,
    pub vendor_cmds: *const nfc_vendor_cmd,
    pub n_vendor_cmds: i32,
    pub ops: *const nfc_ops,
    pub cur_cmd_info: *mut genl_info,
}

pub fn nfc_free_device(dev: *mut nfc_dev) {
    unsafe { put_device(&mut (*dev).dev); }
}

pub unsafe extern "C" fn nfc_set_parent_dev(nfc_dev: *mut nfc_dev, dev: *mut device) {
    (*nfc_dev).dev.parent = dev;
}

pub unsafe extern "C" fn nfc_set_drvdata(dev: *mut nfc_dev, data: *mut core::ffi::c_void) {
    dev_set_drvdata(&mut (*dev).dev, data);
}

pub unsafe extern "C" fn nfc_get_drvdata(dev: *const nfc_dev) -> *mut core::ffi::c_void {
    dev_get_drvdata(&(*dev).dev)
}

pub unsafe extern "C" fn nfc_device_name(dev: *const nfc_dev) -> *const core::ffi::c_char {
    dev_name(&(*dev).dev)
}

extern "C" {
    pub static nfc_class: class;
    pub fn nfc_allocate_device(*const nfc_ops, u32, i32, i32) -> *mut nfc_dev;
    pub fn nfc_register_device(*mut nfc_dev) -> i32;
    pub fn nfc_unregister_rfkill(*mut nfc_dev);
    pub fn nfc_remove_device(*mut nfc_dev);
    pub fn nfc_unregister_device(*mut nfc_dev);
    pub fn nfc_alloc_send_skb(*mut nfc_dev, *mut sock, u32, u32, *mut u32) -> *mut sk_buff;
    pub fn nfc_alloc_recv_skb(u32, gfp_t) -> *mut sk_buff;
    pub fn nfc_set_remote_general_bytes(*mut nfc_dev, *const u8, u8) -> i32;
    pub fn nfc_get_local_general_bytes(*mut nfc_dev, *mut usize) -> *mut u8;
    pub fn nfc_fw_download_done(*mut nfc_dev, *const core::ffi::c_char, u32) -> i32;
    pub fn nfc_targets_found(*mut nfc_dev, *mut nfc_target, i32) -> i32;
    pub fn nfc_target_lost(*mut nfc_dev, u32) -> i32;
    pub fn nfc_dep_link_is_up(*mut nfc_dev, u32, u8, u8) -> i32;
    pub fn nfc_tm_activated(*mut nfc_dev, u32, u8, *const u8, usize) -> i32;
    pub fn nfc_tm_deactivated(*mut nfc_dev) -> i32;
    pub fn nfc_tm_data_received(*mut nfc_dev, *mut sk_buff) -> i32;
    pub fn nfc_driver_failure(*mut nfc_dev, i32);
    pub fn nfc_se_transaction(*mut nfc_dev, u8, *mut nfc_evt_transaction) -> i32;
    pub fn nfc_se_connectivity(*mut nfc_dev, u8) -> i32;
    pub fn nfc_add_se(*mut nfc_dev, u32, u16) -> i32;
    pub fn nfc_remove_se(*mut nfc_dev, u32) -> i32;
    pub fn nfc_find_se(*mut nfc_dev, u32) -> *mut nfc_se;
    pub fn nfc_send_to_raw_sock(*mut nfc_dev, *mut sk_buff, u8, u8);
    pub fn __nfc_alloc_vendor_cmd_reply_skb(*mut nfc_dev, enum_nfc_attrs, u32, u32, i32) -> *mut sk_buff;
    pub fn nfc_vendor_cmd_reply(*mut sk_buff) -> i32;
}

pub unsafe fn nfc_set_vendor_cmds(dev: *mut nfc_dev, cmds: *const nfc_vendor_cmd, n_cmds: i32) -> i32 {
    if (*dev).vendor_cmds.is_null() == false || (*dev).n_vendor_cmds != 0 { return -EINVAL; }
    (*dev).vendor_cmds = cmds;
    (*dev).n_vendor_cmds = n_cmds;
    0
}

pub unsafe fn nfc_vendor_cmd_alloc_reply_skb(dev: *mut nfc_dev, oui: u32, subcmd: u32, approxlen: i32) -> *mut sk_buff {
    __nfc_alloc_vendor_cmd_reply_skb(dev, NFC_ATTR_VENDOR_DATA, oui, subcmd, approxlen)
}

// The C container_of macro is retained as an external dependency; its exact
// expansion depends on the defining kernel object layout.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
