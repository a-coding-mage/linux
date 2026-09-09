/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 1997 by Procom Technology,Inc.
 * 		 2001 by Arnaldo Carvalho de Melo <acme@conectiva.com.br>
 */

use core::ffi::c_int;

#[repr(C)]
pub struct llc_sap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

/* SAP component actions */
pub const SAP_ACT_UNITDATA_IND: c_int = 1;
pub const SAP_ACT_SEND_UI: c_int = 2;
pub const SAP_ACT_SEND_XID_C: c_int = 3;
pub const SAP_ACT_SEND_XID_R: c_int = 4;
pub const SAP_ACT_SEND_TEST_C: c_int = 5;
pub const SAP_ACT_SEND_TEST_R: c_int = 6;
pub const SAP_ACT_REPORT_STATUS: c_int = 7;
pub const SAP_ACT_XID_IND: c_int = 8;
pub const SAP_ACT_TEST_IND: c_int = 9;

/* All action functions must look like this */
pub type llc_sap_action_t =
    Option<unsafe extern "C" fn(sap: *mut llc_sap, skb: *mut sk_buff) -> c_int>;

unsafe extern "C" {
    pub fn llc_sap_action_unitdata_ind(sap: *mut llc_sap, skb: *mut sk_buff) -> c_int;
    pub fn llc_sap_action_send_ui(sap: *mut llc_sap, skb: *mut sk_buff) -> c_int;
    pub fn llc_sap_action_send_xid_c(sap: *mut llc_sap, skb: *mut sk_buff) -> c_int;
    pub fn llc_sap_action_send_xid_r(sap: *mut llc_sap, skb: *mut sk_buff) -> c_int;
    pub fn llc_sap_action_send_test_c(sap: *mut llc_sap, skb: *mut sk_buff) -> c_int;
    pub fn llc_sap_action_send_test_r(sap: *mut llc_sap, skb: *mut sk_buff) -> c_int;
    pub fn llc_sap_action_report_status(sap: *mut llc_sap, skb: *mut sk_buff) -> c_int;
    pub fn llc_sap_action_xid_ind(sap: *mut llc_sap, skb: *mut sk_buff) -> c_int;
    pub fn llc_sap_action_test_ind(sap: *mut llc_sap, skb: *mut sk_buff) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
