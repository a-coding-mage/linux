/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Link Layer Control manager public interface
 *
 * Copyright (C) 2012  Intel Corporation. All rights reserved.
 */

// C dependencies supplied by other translated units:
// #include <net/nfc/hci.h>
// #include <linux/skbuff.h>

pub const LLC_NOP_NAME: &str = "nop";
pub const LLC_SHDLC_NAME: &str = "shdlc";

pub type RcvToHciT =
    unsafe extern "C" fn(hdev: *mut NfcHciDev, skb: *mut SkBuff);
pub type XmitToDrvT =
    unsafe extern "C" fn(hdev: *mut NfcHciDev, skb: *mut SkBuff) -> ::core::ffi::c_int;
pub type LlcFailureT =
    unsafe extern "C" fn(hdev: *mut NfcHciDev, err: ::core::ffi::c_int);

#[repr(C)]
pub struct NfcHciDev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct SkBuff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct NfcLlc {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn nfc_llc_allocate(
        name: *const ::core::ffi::c_char,
        hdev: *mut NfcHciDev,
        xmit_to_drv: Option<XmitToDrvT>,
        rcv_to_hci: Option<RcvToHciT>,
        tx_headroom: ::core::ffi::c_int,
        tx_tailroom: ::core::ffi::c_int,
        llc_failure: Option<LlcFailureT>,
    ) -> *mut NfcLlc;
    pub fn nfc_llc_free(llc: *mut NfcLlc);

    pub fn nfc_llc_start(llc: *mut NfcLlc) -> ::core::ffi::c_int;
    pub fn nfc_llc_stop(llc: *mut NfcLlc) -> ::core::ffi::c_int;
    pub fn nfc_llc_rcv_from_drv(llc: *mut NfcLlc, skb: *mut SkBuff);
    pub fn nfc_llc_xmit_from_hci(
        llc: *mut NfcLlc,
        skb: *mut SkBuff,
    ) -> ::core::ffi::c_int;

    pub fn nfc_llc_init() -> ::core::ffi::c_int;
    pub fn nfc_llc_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
