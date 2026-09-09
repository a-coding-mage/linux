// SPDX-License-Identifier: GPL-2.0-only
/*
 * nop (passthrough) Link Layer Control
 *
 * Copyright (C) 2012  Intel Corporation. All rights reserved.
 */

use core::ffi::c_void;

// Supplied by the corresponding LLC and NFC headers.
extern "C" {
    fn nfc_llc_get_data(llc: *mut nfc_llc) -> *mut c_void;
    fn nfc_llc_register(name: *const u8, ops: *const nfc_llc_ops) -> i32;
    fn kfree(ptr: *mut c_void);
}

#[repr(C)]
pub struct nfc_hci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nfc_llc {
    _private: [u8; 0],
}

// These callback types and LLC operation declarations are supplied by llc.h.
pub type xmit_to_drv_t = unsafe extern "C" fn(*mut nfc_hci_dev, *mut sk_buff) -> i32;
pub type rcv_to_hci_t = unsafe extern "C" fn(*mut nfc_hci_dev, *mut sk_buff);
pub type llc_failure_t = unsafe extern "C" fn(*mut nfc_llc, i32);

#[repr(C)]
pub struct nfc_llc_ops {
    pub init: Option<unsafe extern "C" fn(
        *mut nfc_hci_dev,
        xmit_to_drv_t,
        rcv_to_hci_t,
        i32,
        i32,
        *mut i32,
        *mut i32,
        llc_failure_t,
    ) -> *mut c_void>,
    pub deinit: Option<unsafe extern "C" fn(*mut nfc_llc)>,
    pub start: Option<unsafe extern "C" fn(*mut nfc_llc) -> i32>,
    pub stop: Option<unsafe extern "C" fn(*mut nfc_llc) -> i32>,
    pub rcv_from_drv: Option<unsafe extern "C" fn(*mut nfc_llc, *mut sk_buff)>,
    pub xmit_from_hci: Option<unsafe extern "C" fn(*mut nfc_llc, *mut sk_buff) -> i32>,
}

#[repr(C)]
struct llc_nop {
    hdev: *mut nfc_hci_dev,
    xmit_to_drv: xmit_to_drv_t,
    rcv_to_hci: rcv_to_hci_t,
    tx_headroom: i32,
    tx_tailroom: i32,
    llc_failure: llc_failure_t,
}

unsafe extern "C" fn llc_nop_init(
    hdev: *mut nfc_hci_dev,
    xmit_to_drv: xmit_to_drv_t,
    rcv_to_hci: rcv_to_hci_t,
    tx_headroom: i32,
    tx_tailroom: i32,
    rx_headroom: *mut i32,
    rx_tailroom: *mut i32,
    llc_failure: llc_failure_t,
) -> *mut c_void {
    *rx_headroom = 0;
    *rx_tailroom = 0;

    let llc_nop = Box::into_raw(Box::new(llc_nop {
        hdev,
        xmit_to_drv,
        rcv_to_hci,
        tx_headroom,
        tx_tailroom,
        llc_failure,
    }));

    llc_nop as *mut c_void
}

unsafe extern "C" fn llc_nop_deinit(llc: *mut nfc_llc) {
    kfree(nfc_llc_get_data(llc));
}

unsafe extern "C" fn llc_nop_start(_llc: *mut nfc_llc) -> i32 {
    0
}

unsafe extern "C" fn llc_nop_stop(_llc: *mut nfc_llc) -> i32 {
    0
}

unsafe extern "C" fn llc_nop_rcv_from_drv(llc: *mut nfc_llc, skb: *mut sk_buff) {
    let llc_nop = nfc_llc_get_data(llc) as *mut llc_nop;

    ((*llc_nop).rcv_to_hci)((*llc_nop).hdev, skb);
}

unsafe extern "C" fn llc_nop_xmit_from_hci(llc: *mut nfc_llc, skb: *mut sk_buff) -> i32 {
    let llc_nop = nfc_llc_get_data(llc) as *mut llc_nop;

    ((*llc_nop).xmit_to_drv)((*llc_nop).hdev, skb)
}

static llc_nop_ops: nfc_llc_ops = nfc_llc_ops {
    init: Some(llc_nop_init),
    deinit: Some(llc_nop_deinit),
    start: Some(llc_nop_start),
    stop: Some(llc_nop_stop),
    rcv_from_drv: Some(llc_nop_rcv_from_drv),
    xmit_from_hci: Some(llc_nop_xmit_from_hci),
};

pub unsafe extern "C" fn nfc_llc_nop_register() -> i32 {
    nfc_llc_register(LLC_NOP_NAME.as_ptr(), &llc_nop_ops)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
