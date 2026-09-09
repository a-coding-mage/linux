// SPDX-License-Identifier: GPL-2.0
/*
 * llc_station.c - station component of LLC
 *
 * Copyright (c) 1997 by Procom Technology, Inc.
 *		 2001-2003 by Arnaldo Carvalho de Melo <acme@conectiva.com.br>
 */
// Linux kernel headers and their symbols are supplied by the surrounding translation.

unsafe fn llc_stat_ev_rx_null_dsap_xid_c(skb: *mut sk_buff) -> i32 {
    let pdu: *mut llc_pdu_un = llc_pdu_un_hdr(skb);

    (LLC_PDU_IS_CMD(pdu)
        && LLC_PDU_TYPE_IS_U(pdu)
        && LLC_U_PDU_CMD(pdu) == LLC_1_PDU_CMD_XID
        && (*pdu).dsap == 0) as i32
}

unsafe fn llc_stat_ev_rx_null_dsap_test_c(skb: *mut sk_buff) -> i32 {
    let pdu: *mut llc_pdu_un = llc_pdu_un_hdr(skb);

    (LLC_PDU_IS_CMD(pdu)
        && LLC_PDU_TYPE_IS_U(pdu)
        && LLC_U_PDU_CMD(pdu) == LLC_1_PDU_CMD_TEST
        && (*pdu).dsap == 0) as i32
}

unsafe fn llc_station_ac_send_xid_r(skb: *mut sk_buff) -> i32 {
    let mut mac_da: [u8; ETH_ALEN] = [0; ETH_ALEN];
    let mut dsap: u8 = 0;
    let mut rc: i32 = 1;
    let nskb: *mut sk_buff = llc_alloc_frame(
        core::ptr::null_mut(),
        (*skb).dev,
        LLC_PDU_TYPE_U,
        core::mem::size_of::<llc_xid_info>(),
    );

    if nskb.is_null() {
        return rc;
    }
    llc_pdu_decode_sa(skb, mac_da.as_mut_ptr());
    llc_pdu_decode_ssap(skb, &mut dsap as *mut u8);
    llc_pdu_header_init(nskb, LLC_PDU_TYPE_U, 0, dsap, LLC_PDU_RSP);
    llc_pdu_init_as_xid_rsp(nskb, LLC_XID_NULL_CLASS_2, 127);
    rc = llc_mac_hdr_init(nskb, (*(*skb).dev).dev_addr.as_ptr(), mac_da.as_ptr());
    if rc != 0 {
        kfree_skb(nskb);
        return rc;
    }
    dev_queue_xmit(nskb);
    rc
}

unsafe fn llc_station_ac_send_test_r(skb: *mut sk_buff) -> i32 {
    let mut mac_da: [u8; ETH_ALEN] = [0; ETH_ALEN];
    let mut dsap: u8 = 0;
    let mut rc: i32 = 1;
    let data_size: u32;
    let nskb: *mut sk_buff;

    if (*skb).mac_len < ETH_HLEN {
        return rc;
    }

    /* The test request command is type U (llc_len = 3) */
    data_size = ntohs((*eth_hdr(skb)).h_proto) as u32 - 3;
    nskb = llc_alloc_frame(core::ptr::null_mut(), (*skb).dev, LLC_PDU_TYPE_U, data_size);

    if nskb.is_null() {
        return rc;
    }
    llc_pdu_decode_sa(skb, mac_da.as_mut_ptr());
    llc_pdu_decode_ssap(skb, &mut dsap as *mut u8);
    llc_pdu_header_init(nskb, LLC_PDU_TYPE_U, 0, dsap, LLC_PDU_RSP);
    llc_pdu_init_as_test_rsp(nskb, skb);
    rc = llc_mac_hdr_init(nskb, (*(*skb).dev).dev_addr.as_ptr(), mac_da.as_ptr());
    if rc != 0 {
        kfree_skb(nskb);
        return rc;
    }
    dev_queue_xmit(nskb);
    rc
}

/**
 *	llc_station_rcv - send received pdu to the station state machine
 *	@skb: received frame.
 *
 *	Sends data unit to station state machine.
 */
unsafe fn llc_station_rcv(skb: *mut sk_buff) {
    if llc_stat_ev_rx_null_dsap_xid_c(skb) != 0 {
        llc_station_ac_send_xid_r(skb);
    } else if llc_stat_ev_rx_null_dsap_test_c(skb) != 0 {
        llc_station_ac_send_test_r(skb);
    }
    kfree_skb(skb);
}

pub unsafe fn llc_station_init() {
    llc_set_station_handler(Some(llc_station_rcv));
}

pub unsafe fn llc_station_exit() {
    llc_set_station_handler(None);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
