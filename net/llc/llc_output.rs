// SPDX-License-Identifier: GPL-2.0-only
/*
 * llc_output.c - LLC minimal output path
 *
 * Copyright (c) 1997 by Procom Technology, Inc.
 * 		 2001-2003 by Arnaldo Carvalho de Melo <acme@conectiva.com.br>
 */

// Linux/networking dependencies are supplied by other translated units.

/**
 *	llc_mac_hdr_init - fills MAC header fields
 *	@skb: Address of the frame to initialize its MAC header
 *	@sa: The MAC source address
 *	@da: The MAC destination address
 *
 *	Fills MAC header fields, depending on MAC type. Returns 0, If MAC type
 *	is a valid type and initialization completes correctly 1, otherwise.
 */
pub unsafe fn llc_mac_hdr_init(
    skb: *mut sk_buff,
    sa: *const ::std::os::raw::c_uchar,
    da: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    let mut rc: ::std::os::raw::c_int = -EINVAL;

    match (*(*skb).dev).type_ {
        ARPHRD_ETHER | ARPHRD_LOOPBACK => {
            rc = dev_hard_header(
                skb,
                (*skb).dev,
                ETH_P_802_2,
                da,
                sa,
                (*skb).len,
            );
            if rc > 0 {
                rc = 0;
            }
        }
        _ => {}
    }
    rc
}

/**
 *	llc_build_and_send_ui_pkt - unitdata request interface for upper layers
 *	@sap: sap to use
 *	@skb: packet to send
 *	@dmac: destination mac address
 *	@dsap: destination sap
 *
 *	Upper layers calls this function when upper layer wants to send data
 *	using connection-less mode communication (UI pdu).
 *
 *	Accept data frame from network layer to be sent using connection-
 *	less mode communication; timeout/retries handled by network layer;
 *	package primitive as an event and send to SAP event handler
 */
pub unsafe fn llc_build_and_send_ui_pkt(
    sap: *mut llc_sap,
    skb: *mut sk_buff,
    dmac: *const ::std::os::raw::c_uchar,
    dsap: ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    let mut rc: ::std::os::raw::c_int;
    llc_pdu_header_init(
        skb,
        LLC_PDU_TYPE_U,
        (*sap).laddr.lsap,
        dsap,
        LLC_PDU_CMD,
    );
    llc_pdu_init_as_ui_cmd(skb);
    rc = llc_mac_hdr_init(skb, (*(*skb).dev).dev_addr, dmac);
    if likely(!rc) {
        rc = dev_queue_xmit(skb);
    } else {
        kfree_skb(skb);
    }
    rc
}

// EXPORT_SYMBOL(llc_mac_hdr_init);
// EXPORT_SYMBOL(llc_build_and_send_ui_pkt);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
