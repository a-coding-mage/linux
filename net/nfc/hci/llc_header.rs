/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Link Layer Control manager
 *
 * Copyright (C) 2012  Intel Corporation. All rights reserved.
 */

use core::ffi::c_char;

/* Types and structures supplied by the NFC and kernel dependencies. */
#[repr(C)]
pub struct nfc_hci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct nfc_llc {
    pub data: *mut core::ffi::c_void,
    pub ops: *const nfc_llc_ops,
    pub rx_headroom: i32,
    pub rx_tailroom: i32,
}

pub type xmit_to_drv_t = Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>;
pub type rcv_to_hci_t = Option<unsafe extern "C" fn(*mut core::ffi::c_void)>;
pub type llc_failure_t = Option<unsafe extern "C" fn(*mut core::ffi::c_void)>;

#[repr(C)]
pub struct nfc_llc_ops {
    pub init: Option<
        unsafe extern "C" fn(
            hdev: *mut nfc_hci_dev,
            xmit_to_drv: xmit_to_drv_t,
            rcv_to_hci: rcv_to_hci_t,
            tx_headroom: i32,
            tx_tailroom: i32,
            rx_headroom: *mut i32,
            rx_tailroom: *mut i32,
            llc_failure: llc_failure_t,
        ) -> *mut core::ffi::c_void,
    >,
    pub deinit: Option<unsafe extern "C" fn(llc: *mut nfc_llc)>,
    pub start: Option<unsafe extern "C" fn(llc: *mut nfc_llc) -> i32>,
    pub stop: Option<unsafe extern "C" fn(llc: *mut nfc_llc) -> i32>,
    pub rcv_from_drv:
        Option<unsafe extern "C" fn(llc: *mut nfc_llc, skb: *mut sk_buff)>,
    pub xmit_from_hci:
        Option<unsafe extern "C" fn(llc: *mut nfc_llc, skb: *mut sk_buff) -> i32>,
}

#[repr(C)]
pub struct nfc_llc_engine {
    pub name: *const c_char,
    pub ops: *const nfc_llc_ops,
    pub entry: list_head,
}

unsafe extern "C" {
    pub fn nfc_llc_get_data(llc: *mut nfc_llc) -> *mut core::ffi::c_void;

    pub fn nfc_llc_register(name: *const c_char, ops: *const nfc_llc_ops) -> i32;

    pub fn nfc_llc_nop_register() -> i32;
}

/* CONFIG_NFC_SHDLC controls whether the external registration function exists. */
#[cfg(feature = "CONFIG_NFC_SHDLC")]
unsafe extern "C" {
    pub fn nfc_llc_shdlc_register() -> i32;
}

#[cfg(not(feature = "CONFIG_NFC_SHDLC"))]
#[inline]
pub fn nfc_llc_shdlc_register() -> i32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
