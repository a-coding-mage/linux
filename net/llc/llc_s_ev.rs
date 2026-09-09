// SPDX-License-Identifier: GPL-2.0
/*
 * llc_s_ev.rs - Defines SAP component events
 *
 * The followed event functions are SAP component events which are described
 * in 802.2 LLC protocol standard document.
 *
 * Copyright (c) 1997 by Procom Technology, Inc.
 *              2001-2003 by Arnaldo Carvalho de Melo <acme@conectiva.com.br>
 */

pub unsafe fn llc_sap_ev_activation_req(
    sap: *mut llc_sap,
    skb: *mut sk_buff,
) -> i32 {
    let ev: *mut llc_sap_state_ev = llc_sap_ev(skb);

    if (*ev).type_ == LLC_SAP_EV_TYPE_SIMPLE
        && (*ev).prim_type == LLC_SAP_EV_ACTIVATION_REQ
    {
        0
    } else {
        1
    }
}

pub unsafe fn llc_sap_ev_rx_ui(sap: *mut llc_sap, skb: *mut sk_buff) -> i32 {
    let ev: *mut llc_sap_state_ev = llc_sap_ev(skb);
    let pdu: *mut llc_pdu_un = llc_pdu_un_hdr(skb);

    if (*ev).type_ == LLC_SAP_EV_TYPE_PDU
        && LLC_PDU_IS_CMD(pdu)
        && LLC_PDU_TYPE_IS_U(pdu)
        && LLC_U_PDU_CMD(pdu) == LLC_1_PDU_CMD_UI
    {
        0
    } else {
        1
    }
}

pub unsafe fn llc_sap_ev_unitdata_req(sap: *mut llc_sap, skb: *mut sk_buff) -> i32 {
    let ev: *mut llc_sap_state_ev = llc_sap_ev(skb);

    if (*ev).type_ == LLC_SAP_EV_TYPE_PRIM
        && (*ev).prim == LLC_DATAUNIT_PRIM
        && (*ev).prim_type == LLC_PRIM_TYPE_REQ
    {
        0
    } else {
        1
    }
}

pub unsafe fn llc_sap_ev_xid_req(sap: *mut llc_sap, skb: *mut sk_buff) -> i32 {
    let ev: *mut llc_sap_state_ev = llc_sap_ev(skb);

    if (*ev).type_ == LLC_SAP_EV_TYPE_PRIM
        && (*ev).prim == LLC_XID_PRIM
        && (*ev).prim_type == LLC_PRIM_TYPE_REQ
    {
        0
    } else {
        1
    }
}

pub unsafe fn llc_sap_ev_rx_xid_c(sap: *mut llc_sap, skb: *mut sk_buff) -> i32 {
    let ev: *mut llc_sap_state_ev = llc_sap_ev(skb);
    let pdu: *mut llc_pdu_un = llc_pdu_un_hdr(skb);

    if (*ev).type_ == LLC_SAP_EV_TYPE_PDU
        && LLC_PDU_IS_CMD(pdu)
        && LLC_PDU_TYPE_IS_U(pdu)
        && LLC_U_PDU_CMD(pdu) == LLC_1_PDU_CMD_XID
    {
        0
    } else {
        1
    }
}

pub unsafe fn llc_sap_ev_rx_xid_r(sap: *mut llc_sap, skb: *mut sk_buff) -> i32 {
    let ev: *mut llc_sap_state_ev = llc_sap_ev(skb);
    let pdu: *mut llc_pdu_un = llc_pdu_un_hdr(skb);

    if (*ev).type_ == LLC_SAP_EV_TYPE_PDU
        && LLC_PDU_IS_RSP(pdu)
        && LLC_PDU_TYPE_IS_U(pdu)
        && LLC_U_PDU_RSP(pdu) == LLC_1_PDU_CMD_XID
    {
        0
    } else {
        1
    }
}

pub unsafe fn llc_sap_ev_test_req(sap: *mut llc_sap, skb: *mut sk_buff) -> i32 {
    let ev: *mut llc_sap_state_ev = llc_sap_ev(skb);

    if (*ev).type_ == LLC_SAP_EV_TYPE_PRIM
        && (*ev).prim == LLC_TEST_PRIM
        && (*ev).prim_type == LLC_PRIM_TYPE_REQ
    {
        0
    } else {
        1
    }
}

pub unsafe fn llc_sap_ev_rx_test_c(sap: *mut llc_sap, skb: *mut sk_buff) -> i32 {
    let ev: *mut llc_sap_state_ev = llc_sap_ev(skb);
    let pdu: *mut llc_pdu_un = llc_pdu_un_hdr(skb);

    if (*ev).type_ == LLC_SAP_EV_TYPE_PDU
        && LLC_PDU_IS_CMD(pdu)
        && LLC_PDU_TYPE_IS_U(pdu)
        && LLC_U_PDU_CMD(pdu) == LLC_1_PDU_CMD_TEST
    {
        0
    } else {
        1
    }
}

pub unsafe fn llc_sap_ev_rx_test_r(sap: *mut llc_sap, skb: *mut sk_buff) -> i32 {
    let ev: *mut llc_sap_state_ev = llc_sap_ev(skb);
    let pdu: *mut llc_pdu_un = llc_pdu_un_hdr(skb);

    if (*ev).type_ == LLC_SAP_EV_TYPE_PDU
        && LLC_PDU_IS_RSP(pdu)
        && LLC_PDU_TYPE_IS_U(pdu)
        && LLC_U_PDU_RSP(pdu) == LLC_1_PDU_CMD_TEST
    {
        0
    } else {
        1
    }
}

pub unsafe fn llc_sap_ev_deactivation_req(
    sap: *mut llc_sap,
    skb: *mut sk_buff,
) -> i32 {
    let ev: *mut llc_sap_state_ev = llc_sap_ev(skb);

    if (*ev).type_ == LLC_SAP_EV_TYPE_SIMPLE
        && (*ev).prim_type == LLC_SAP_EV_DEACTIVATION_REQ
    {
        0
    } else {
        1
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
