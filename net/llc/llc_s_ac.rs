// SPDX-License-Identifier: GPL-2.0
/*
 * llc_s_ac.c - actions performed during sap state transition.
 *
 * Description :
 *   Functions in this module are implementation of sap component actions.
 *   Details of actions can be found in IEEE-802.2 standard document.
 *   All functions have one sap and one event as input argument. All of
 *   them return 0 On success and 1 otherwise.
 *
 * Copyright (c) 1997 by Procom Technology, Inc.
 *             2001-2003 by Arnaldo Carvalho de Melo <acme@conectiva.com.br>
 */

/* Linux LLC and networking types, constants, and functions are supplied by
 * the surrounding translation unit/dependencies. */

/// llc_sap_action_unitdata_ind - forward UI PDU to network layer.
pub unsafe fn llc_sap_action_unitdata_ind(
    sap: *mut llc_sap,
    skb: *mut sk_buff,
) -> i32 {
    llc_sap_rtn_pdu(sap, skb);
    0
}

unsafe fn llc_prepare_and_xmit(skb: *mut sk_buff) -> i32 {
    let ev: *mut llc_sap_state_ev = llc_sap_ev(skb);
    let nskb: *mut sk_buff;
    let rc: i32;

    rc = llc_mac_hdr_init(skb, (*ev).saddr.mac.as_ptr(), (*ev).daddr.mac.as_ptr());
    if rc != 0 {
        return rc;
    }

    nskb = skb_clone(skb, GFP_ATOMIC);
    if nskb.is_null() {
        return -ENOMEM;
    }

    if !(*skb).sk.is_null() {
        skb_set_owner_w(nskb, (*skb).sk);
    }

    dev_queue_xmit(nskb)
}

/// llc_sap_action_send_ui - sends UI PDU response to UNITDATA REQ.
pub unsafe fn llc_sap_action_send_ui(
    _sap: *mut llc_sap,
    skb: *mut sk_buff,
) -> i32 {
    let ev: *mut llc_sap_state_ev = llc_sap_ev(skb);

    llc_pdu_header_init(skb, LLC_PDU_TYPE_U, (*ev).saddr.lsap,
                        (*ev).daddr.lsap, LLC_PDU_CMD);
    llc_pdu_init_as_ui_cmd(skb);

    llc_prepare_and_xmit(skb)
}

/// llc_sap_action_send_xid_c - send XID PDU as response to XID REQ.
pub unsafe fn llc_sap_action_send_xid_c(
    _sap: *mut llc_sap,
    skb: *mut sk_buff,
) -> i32 {
    let ev: *mut llc_sap_state_ev = llc_sap_ev(skb);

    llc_pdu_header_init(skb, LLC_PDU_TYPE_U_XID, (*ev).saddr.lsap,
                        (*ev).daddr.lsap, LLC_PDU_CMD);
    llc_pdu_init_as_xid_cmd(skb, LLC_XID_NULL_CLASS_2, 0);

    llc_prepare_and_xmit(skb)
}

/// llc_sap_action_send_xid_r - send XID PDU response to MAC.
pub unsafe fn llc_sap_action_send_xid_r(
    sap: *mut llc_sap,
    skb: *mut sk_buff,
) -> i32 {
    let mut mac_da = [0u8; ETH_ALEN];
    let mut mac_sa = [0u8; ETH_ALEN];
    let mut dsap: u8 = 0;
    let mut rc: i32 = 1;
    let nskb: *mut sk_buff;

    llc_pdu_decode_sa(skb, mac_da.as_mut_ptr());
    llc_pdu_decode_da(skb, mac_sa.as_mut_ptr());
    llc_pdu_decode_ssap(skb, &mut dsap);
    nskb = llc_alloc_frame(core::ptr::null_mut(), (*skb).dev,
                           LLC_PDU_TYPE_U,
                           core::mem::size_of::<llc_xid_info>());
    if nskb.is_null() {
        return rc;
    }
    llc_pdu_header_init(nskb, LLC_PDU_TYPE_U, (*sap).laddr.lsap, dsap,
                        LLC_PDU_RSP);
    llc_pdu_init_as_xid_rsp(nskb, LLC_XID_NULL_CLASS_2, 0);
    rc = llc_mac_hdr_init(nskb, mac_sa.as_ptr(), mac_da.as_ptr());
    if rc == 0 {
        rc = dev_queue_xmit(nskb);
    }
    rc
}

/// llc_sap_action_send_test_c - send TEST PDU to MAC in response to TEST REQ.
pub unsafe fn llc_sap_action_send_test_c(
    _sap: *mut llc_sap,
    skb: *mut sk_buff,
) -> i32 {
    let ev: *mut llc_sap_state_ev = llc_sap_ev(skb);

    llc_pdu_header_init(skb, LLC_PDU_TYPE_U, (*ev).saddr.lsap,
                        (*ev).daddr.lsap, LLC_PDU_CMD);
    llc_pdu_init_as_test_cmd(skb);

    llc_prepare_and_xmit(skb)
}

pub unsafe fn llc_sap_action_send_test_r(
    sap: *mut llc_sap,
    skb: *mut sk_buff,
) -> i32 {
    let mut mac_da = [0u8; ETH_ALEN];
    let mut mac_sa = [0u8; ETH_ALEN];
    let mut dsap: u8 = 0;
    let nskb: *mut sk_buff;
    let mut rc: i32 = 1;
    let data_size: u32;

    if (*skb).mac_len < ETH_HLEN {
        return 1;
    }

    llc_pdu_decode_sa(skb, mac_da.as_mut_ptr());
    llc_pdu_decode_da(skb, mac_sa.as_mut_ptr());
    llc_pdu_decode_ssap(skb, &mut dsap);

    /* The test request command is type U (llc_len = 3). */
    data_size = u16::from_be((*eth_hdr(skb)).h_proto) as u32 - 3;
    nskb = llc_alloc_frame(core::ptr::null_mut(), (*skb).dev,
                           LLC_PDU_TYPE_U, data_size);
    if nskb.is_null() {
        return rc;
    }
    llc_pdu_header_init(nskb, LLC_PDU_TYPE_U, (*sap).laddr.lsap, dsap,
                        LLC_PDU_RSP);
    llc_pdu_init_as_test_rsp(nskb, skb);
    rc = llc_mac_hdr_init(nskb, mac_sa.as_ptr(), mac_da.as_ptr());
    if rc == 0 {
        rc = dev_queue_xmit(nskb);
    }
    rc
}

/// llc_sap_action_report_status - report data link status to layer management.
pub unsafe fn llc_sap_action_report_status(
    _sap: *mut llc_sap,
    _skb: *mut sk_buff,
) -> i32 {
    0
}

/// llc_sap_action_xid_ind - send XID PDU to network layer via XID IND.
pub unsafe fn llc_sap_action_xid_ind(
    sap: *mut llc_sap,
    skb: *mut sk_buff,
) -> i32 {
    llc_sap_rtn_pdu(sap, skb);
    0
}

/// llc_sap_action_test_ind - send TEST PDU to network layer via TEST IND.
pub unsafe fn llc_sap_action_test_ind(
    sap: *mut llc_sap,
    skb: *mut sk_buff,
) -> i32 {
    llc_sap_rtn_pdu(sap, skb);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
