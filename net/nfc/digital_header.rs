/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * NFC Digital Protocol stack
 * Copyright (c) 2013, Intel Corporation.
 */

// C headers omitted; referenced types and functions are supplied by dependencies.

pub const DIGITAL_CMD_IN_SEND: u8 = 0;
pub const DIGITAL_CMD_TG_SEND: u8 = 1;
pub const DIGITAL_CMD_TG_LISTEN: u8 = 2;
pub const DIGITAL_CMD_TG_LISTEN_MDAA: u8 = 3;
pub const DIGITAL_CMD_TG_LISTEN_MD: u8 = 4;

pub const DIGITAL_MAX_HEADER_LEN: usize = 7;
pub const DIGITAL_CRC_LEN: usize = 2;

pub const DIGITAL_SENSF_NFCID2_NFC_DEP_B1: u8 = 0x01;
pub const DIGITAL_SENSF_NFCID2_NFC_DEP_B2: u8 = 0xFE;

pub const DIGITAL_SENS_RES_NFC_DEP: u16 = 0x0100;
pub const DIGITAL_SEL_RES_NFC_DEP: u8 = 0x40;
pub const DIGITAL_SENSF_FELICA_SC: u16 = 0xFFFF;

#[repr(C)]
pub struct digital_data_exch {
    pub cb: data_exchange_cb_t,
    pub cb_context: *mut core::ffi::c_void,
}

pub type crc_func_t = unsafe extern "C" fn(u16, *const u8, usize) -> u16;

pub const CRC_A_INIT: u16 = 0x6363;
pub const CRC_B_INIT: u16 = 0xFFFF;
pub const CRC_F_INIT: u16 = 0x0000;

extern "C" {
    pub fn digital_skb_alloc(ddev: *mut nfc_digital_dev, len: u32) -> *mut sk_buff;

    pub fn digital_send_cmd(
        ddev: *mut nfc_digital_dev,
        cmd_type: u8,
        skb: *mut sk_buff,
        params: *mut digital_tg_mdaa_params,
        timeout: u16,
        cmd_cb: nfc_digital_cmd_complete_t,
        cb_context: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;

    pub fn digital_in_configure_hw(
        ddev: *mut nfc_digital_dev,
        type_: core::ffi::c_int,
        param: core::ffi::c_int,
    ) -> core::ffi::c_int;

    pub fn digital_poll_next_tech(ddev: *mut nfc_digital_dev);
    pub fn digital_in_send_sens_req(ddev: *mut nfc_digital_dev, rf_tech: u8) -> core::ffi::c_int;
    pub fn digital_in_send_sensb_req(ddev: *mut nfc_digital_dev, rf_tech: u8) -> core::ffi::c_int;
    pub fn digital_in_send_sensf_req(ddev: *mut nfc_digital_dev, rf_tech: u8) -> core::ffi::c_int;
    pub fn digital_in_send_iso15693_inv_req(ddev: *mut nfc_digital_dev, rf_tech: u8) -> core::ffi::c_int;
    pub fn digital_in_iso_dep_pull_sod(ddev: *mut nfc_digital_dev, skb: *mut sk_buff) -> core::ffi::c_int;
    pub fn digital_in_iso_dep_push_sod(ddev: *mut nfc_digital_dev, skb: *mut sk_buff) -> core::ffi::c_int;
    pub fn digital_target_found(ddev: *mut nfc_digital_dev, target: *mut nfc_target, protocol: u8) -> core::ffi::c_int;
    pub fn digital_in_recv_mifare_res(resp: *mut sk_buff) -> core::ffi::c_int;
    pub fn digital_in_send_atr_req(ddev: *mut nfc_digital_dev, target: *mut nfc_target, comm_mode: u8, gb: *mut u8, gb_len: usize) -> core::ffi::c_int;
    pub fn digital_in_send_dep_req(ddev: *mut nfc_digital_dev, target: *mut nfc_target, skb: *mut sk_buff, data_exch: *mut digital_data_exch) -> core::ffi::c_int;
    pub fn digital_tg_configure_hw(ddev: *mut nfc_digital_dev, type_: core::ffi::c_int, param: core::ffi::c_int) -> core::ffi::c_int;
    pub fn digital_tg_recv_sens_req(ddev: *mut nfc_digital_dev, arg: *mut core::ffi::c_void, resp: *mut sk_buff);
    pub fn digital_tg_recv_sensf_req(ddev: *mut nfc_digital_dev, arg: *mut core::ffi::c_void, resp: *mut sk_buff);
    pub fn digital_tg_recv_atr_req(ddev: *mut nfc_digital_dev, arg: *mut core::ffi::c_void, resp: *mut sk_buff);
    pub fn digital_tg_send_dep_res(ddev: *mut nfc_digital_dev, skb: *mut sk_buff) -> core::ffi::c_int;
    pub fn digital_tg_listen_nfca(ddev: *mut nfc_digital_dev, rf_tech: u8) -> core::ffi::c_int;
    pub fn digital_tg_listen_nfcf(ddev: *mut nfc_digital_dev, rf_tech: u8) -> core::ffi::c_int;
    pub fn digital_tg_recv_md_req(ddev: *mut nfc_digital_dev, arg: *mut core::ffi::c_void, resp: *mut sk_buff);
    pub fn digital_skb_add_crc(skb: *mut sk_buff, crc_func: crc_func_t, init: u16, bitwise_inv: u8, msb_first: u8);
    pub fn digital_skb_check_crc(skb: *mut sk_buff, crc_func: crc_func_t, crc_init: u16, bitwise_inv: u8, msb_first: u8) -> core::ffi::c_int;
}

pub unsafe fn digital_in_send_cmd(ddev: *mut nfc_digital_dev, skb: *mut sk_buff, timeout: u16, cmd_cb: nfc_digital_cmd_complete_t, cb_context: *mut core::ffi::c_void) -> core::ffi::c_int {
    digital_send_cmd(ddev, DIGITAL_CMD_IN_SEND, skb, core::ptr::null_mut(), timeout, cmd_cb, cb_context)
}

pub unsafe fn digital_tg_send_cmd(ddev: *mut nfc_digital_dev, skb: *mut sk_buff, timeout: u16, cmd_cb: nfc_digital_cmd_complete_t, cb_context: *mut core::ffi::c_void) -> core::ffi::c_int {
    digital_send_cmd(ddev, DIGITAL_CMD_TG_SEND, skb, core::ptr::null_mut(), timeout, cmd_cb, cb_context)
}

pub unsafe fn digital_tg_listen(ddev: *mut nfc_digital_dev, timeout: u16, cb: nfc_digital_cmd_complete_t, arg: *mut core::ffi::c_void) -> core::ffi::c_int {
    digital_send_cmd(ddev, DIGITAL_CMD_TG_LISTEN, core::ptr::null_mut(), core::ptr::null_mut(), timeout, cb, arg)
}

pub unsafe fn digital_skb_add_crc_a(skb: *mut sk_buff) { digital_skb_add_crc(skb, crc_ccitt, CRC_A_INIT, 0, 0); }
pub unsafe fn digital_skb_add_crc_b(skb: *mut sk_buff) { digital_skb_add_crc(skb, crc_ccitt, CRC_B_INIT, 1, 0); }
pub unsafe fn digital_skb_add_crc_f(skb: *mut sk_buff) { digital_skb_add_crc(skb, crc_itu_t, CRC_F_INIT, 0, 1); }
pub unsafe fn digital_skb_add_crc_none(_skb: *mut sk_buff) {}
pub unsafe fn digital_skb_check_crc_a(skb: *mut sk_buff) -> core::ffi::c_int { digital_skb_check_crc(skb, crc_ccitt, CRC_A_INIT, 0, 0) }
pub unsafe fn digital_skb_check_crc_b(skb: *mut sk_buff) -> core::ffi::c_int { digital_skb_check_crc(skb, crc_ccitt, CRC_B_INIT, 1, 0) }
pub unsafe fn digital_skb_check_crc_f(skb: *mut sk_buff) -> core::ffi::c_int { digital_skb_check_crc(skb, crc_itu_t, CRC_F_INIT, 0, 1) }
pub unsafe fn digital_skb_check_crc_none(_skb: *mut sk_buff) -> core::ffi::c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
