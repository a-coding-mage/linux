// SPDX-License-Identifier: GPL-2.0
/*
 * llc_input.c - Minimal input path for LLC
 *
 * Copyright (c) 1997 by Procom Technology, Inc.
 *             2001-2003 by Arnaldo Carvalho de Melo <acme@conectiva.com.br>
 */

// Kernel declarations and constants referenced below are supplied by the
// surrounding networking implementation.

static mut llc_station_handler: Option<unsafe extern "C" fn(*mut sk_buff)> = None;

static mut llc_type_handlers:
    [Option<unsafe extern "C" fn(*mut llc_sap, *mut sk_buff)>; 2] = [None, None];

#[no_mangle]
pub unsafe extern "C" fn llc_add_pack(
    type_: i32,
    handler: Option<unsafe extern "C" fn(*mut llc_sap, *mut sk_buff)>,
) {
    smp_wmb(); // ensure initialisation is complete before it's called
    if type_ == LLC_DEST_SAP || type_ == LLC_DEST_CONN {
        llc_type_handlers[(type_ - 1) as usize] = handler;
    }
}

#[no_mangle]
pub unsafe extern "C" fn llc_remove_pack(type_: i32) {
    if type_ == LLC_DEST_SAP || type_ == LLC_DEST_CONN {
        llc_type_handlers[(type_ - 1) as usize] = None;
    }
    synchronize_net();
}

#[no_mangle]
pub unsafe extern "C" fn llc_set_station_handler(
    handler: Option<unsafe extern "C" fn(*mut sk_buff)>,
) {
    // Ensure initialisation is complete before it's called
    if handler.is_some() {
        smp_wmb();
    }

    llc_station_handler = handler;

    if handler.is_none() {
        synchronize_net();
    }
}

/*
 * llc_pdu_type - returns which LLC component must handle for PDU
 * @skb: input skb
 *
 * This function returns which LLC component must handle this PDU.
 */
unsafe fn llc_pdu_type(skb: *mut sk_buff) -> i32 {
    let mut type_: i32 = LLC_DEST_CONN; // I-PDU or S-PDU type
    let pdu: *mut llc_pdu_sn = llc_pdu_sn_hdr(skb);

    if ((*pdu).ctrl_1 & LLC_PDU_TYPE_MASK) != LLC_PDU_TYPE_U {
        return type_;
    }
    match LLC_U_PDU_CMD(pdu) {
        LLC_1_PDU_CMD_XID | LLC_1_PDU_CMD_UI | LLC_1_PDU_CMD_TEST => {
            type_ = LLC_DEST_SAP;
        }
        LLC_2_PDU_CMD_SABME
        | LLC_2_PDU_CMD_DISC
        | LLC_2_PDU_RSP_UA
        | LLC_2_PDU_RSP_DM
        | LLC_2_PDU_RSP_FRMR => {}
        _ => {
            type_ = LLC_DEST_INVALID;
        }
    }
    type_
}

/*
 * llc_fixup_skb - initializes skb pointers
 * @skb: This argument points to incoming skb
 *
 * Initializes internal skb pointer to start of network layer by deriving
 * length of LLC header; finds length of LLC control field in LLC header
 * by looking at the two lowest-order bits of the first control field
 * byte; field is either 3 or 4 bytes long.
 */
unsafe fn llc_fixup_skb(skb: *mut sk_buff) -> i32 {
    let mut llc_len: u8 = 2;
    let mut pdu: *mut llc_pdu_un;

    if unlikely(!pskb_may_pull(skb, core::mem::size_of::<llc_pdu_un>())) {
        return 0;
    }

    pdu = (*skb).data as *mut llc_pdu_un;
    if ((*pdu).ctrl_1 & LLC_PDU_TYPE_MASK) == LLC_PDU_TYPE_U {
        llc_len = 1;
    }
    llc_len += 2;

    if unlikely(!pskb_may_pull(skb, llc_len as usize)) {
        return 0;
    }

    skb_pull(skb, llc_len as usize);
    skb_reset_transport_header(skb);
    if (*skb).protocol == htons(ETH_P_802_2) {
        let pdulen: u16;
        let data_size: i32;

        if (*skb).mac_len < ETH_HLEN {
            return 0;
        }

        pdulen = (*eth_hdr(skb)).h_proto;
        data_size = ntohs(pdulen) as i32 - llc_len as i32;

        if data_size < 0 || !pskb_may_pull(skb, data_size as usize) {
            return 0;
        }
        if unlikely(pskb_trim_rcsum(skb, data_size as usize) != 0) {
            return 0;
        }
    }
    1
}

/*
 * llc_rcv - 802.2 entry point from net lower layers
 * @skb: received pdu
 * @dev: device that receive pdu
 * @pt: packet type
 * @orig_dev: the original receive net device
 *
 * When the system receives a 802.2 frame this function is called. It
 * checks SAP and connection of received pdu and passes frame to
 * llc_{station,sap,conn}_rcv for sending to proper state machine. If the
 * frame is related to a busy connection (a connection is sending data now),
 * it queues this frame in the connection's backlog.
 */
#[no_mangle]
pub unsafe extern "C" fn llc_rcv(
    mut skb: *mut sk_buff,
    dev: *mut net_device,
    pt: *mut packet_type,
    orig_dev: *mut net_device,
) -> i32 {
    let mut sap: *mut llc_sap;
    let pdu: *mut llc_pdu_sn;
    let dest: i32;
    let rcv: Option<unsafe extern "C" fn(*mut sk_buff, *mut net_device, *mut packet_type, *mut net_device)>;
    let sta_handler: Option<unsafe extern "C" fn(*mut sk_buff)>;
    let sap_handler: Option<unsafe extern "C" fn(*mut llc_sap, *mut sk_buff)>;

    if unlikely((*skb).pkt_type == PACKET_OTHERHOST) {
        goto_drop(skb);
        return 0;
    }
    skb = skb_share_check(skb, GFP_ATOMIC);
    if skb.is_null() {
        return 0;
    }
    if unlikely(llc_fixup_skb(skb) == 0) {
        kfree_skb(skb);
        return 0;
    }
    pdu = llc_pdu_sn_hdr(skb);
    if unlikely((*pdu).dsap == 0) {
        sta_handler = core::ptr::read_volatile(&llc_station_handler);
        match sta_handler {
            Some(handler) => handler(skb),
            None => kfree_skb(skb),
        }
        return 0;
    }
    sap = llc_sap_find((*pdu).dsap);
    if sap.is_null() {
        kfree_skb(skb);
        return 0;
    }
    rcv = (*sap).rcv_func;
    dest = llc_pdu_type(skb);
    sap_handler = if dest != 0 {
        core::ptr::read_volatile(&llc_type_handlers[(dest - 1) as usize])
    } else {
        None
    };
    match sap_handler {
        None => match rcv {
            Some(handler) => { handler(skb, dev, pt, orig_dev); }
            None => kfree_skb(skb),
        },
        Some(handler) => {
            if let Some(receiver) = rcv {
                let cskb = skb_clone(skb, GFP_ATOMIC);
                if !cskb.is_null() {
                    receiver(cskb, dev, pt, orig_dev);
                }
            }
            handler(sap, skb);
        }
    }
    llc_sap_put(sap);
    0
}

unsafe fn goto_drop(skb: *mut sk_buff) {
    kfree_skb(skb);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
