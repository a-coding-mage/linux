/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 1997 by Procom Technology,Inc.
 *                 2001 by Arnaldo Carvalho de Melo <acme@conectiva.com.br>
 */

/* Dependency types are supplied by the corresponding kernel headers. */

/* Defines SAP component events */
/* Types of events (possible values in 'ev->type') */
pub const LLC_SAP_EV_TYPE_SIMPLE: u8 = 1;
pub const LLC_SAP_EV_TYPE_CONDITION: u8 = 2;
pub const LLC_SAP_EV_TYPE_PRIM: u8 = 3;
pub const LLC_SAP_EV_TYPE_PDU: u8 = 4; /* command/response PDU */
pub const LLC_SAP_EV_TYPE_ACK_TMR: u8 = 5;
pub const LLC_SAP_EV_TYPE_RPT_STATUS: u8 = 6;

pub const LLC_SAP_EV_ACTIVATION_REQ: u8 = 1;
pub const LLC_SAP_EV_RX_UI: u8 = 2;
pub const LLC_SAP_EV_UNITDATA_REQ: u8 = 3;
pub const LLC_SAP_EV_XID_REQ: u8 = 4;
pub const LLC_SAP_EV_RX_XID_C: u8 = 5;
pub const LLC_SAP_EV_RX_XID_R: u8 = 6;
pub const LLC_SAP_EV_TEST_REQ: u8 = 7;
pub const LLC_SAP_EV_RX_TEST_C: u8 = 8;
pub const LLC_SAP_EV_RX_TEST_R: u8 = 9;
pub const LLC_SAP_EV_DEACTIVATION_REQ: u8 = 10;

#[repr(C)]
pub struct llc_sap_state_ev {
    pub prim: u8,
    pub prim_type: u8,
    pub type_: u8,
    pub reason: u8,
    pub ind_cfm_flag: u8,
    pub saddr: crate::llc_addr,
    pub daddr: crate::llc_addr,
}

#[inline]
pub unsafe fn llc_sap_ev(skb: *mut crate::sk_buff) -> *mut llc_sap_state_ev {
    (*skb).cb.as_mut_ptr() as *mut llc_sap_state_ev
}

pub type llc_sap_ev_t = unsafe extern "C" fn(
    sap: *mut crate::llc_sap,
    skb: *mut crate::sk_buff,
) -> ::core::ffi::c_int;

unsafe extern "C" {
    pub fn llc_sap_ev_activation_req(
        sap: *mut crate::llc_sap,
        skb: *mut crate::sk_buff,
    ) -> ::core::ffi::c_int;
    pub fn llc_sap_ev_rx_ui(sap: *mut crate::llc_sap, skb: *mut crate::sk_buff) -> ::core::ffi::c_int;
    pub fn llc_sap_ev_unitdata_req(
        sap: *mut crate::llc_sap,
        skb: *mut crate::sk_buff,
    ) -> ::core::ffi::c_int;
    pub fn llc_sap_ev_xid_req(sap: *mut crate::llc_sap, skb: *mut crate::sk_buff) -> ::core::ffi::c_int;
    pub fn llc_sap_ev_rx_xid_c(sap: *mut crate::llc_sap, skb: *mut crate::sk_buff) -> ::core::ffi::c_int;
    pub fn llc_sap_ev_rx_xid_r(sap: *mut crate::llc_sap, skb: *mut crate::sk_buff) -> ::core::ffi::c_int;
    pub fn llc_sap_ev_test_req(sap: *mut crate::llc_sap, skb: *mut crate::sk_buff) -> ::core::ffi::c_int;
    pub fn llc_sap_ev_rx_test_c(sap: *mut crate::llc_sap, skb: *mut crate::sk_buff) -> ::core::ffi::c_int;
    pub fn llc_sap_ev_rx_test_r(sap: *mut crate::llc_sap, skb: *mut crate::sk_buff) -> ::core::ffi::c_int;
    pub fn llc_sap_ev_deactivation_req(
        sap: *mut crate::llc_sap,
        skb: *mut crate::sk_buff,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
