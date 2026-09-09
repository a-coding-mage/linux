// SPDX-License-Identifier: GPL-2.0
/*
 * llc_if.c - Defines LLC interface to upper layer
 *
 * Copyright (c) 1997 by Procom Technology, Inc.
 * 		 2001-2003 by Arnaldo Carvalho de Melo <acme@conectiva.com.br>
 */

// Linux kernel dependencies supplied by other translation units.

/**
 * llc_build_and_send_pkt - Connection data sending for upper layers.
 * @sk: connection
 * @skb: packet to send
 *
 * This function is called when upper layer wants to send data using
 * connection oriented communication mode. During sending data, connection
 * will be locked and received frames and expired timers will be queued.
 * Returns 0 for success, -ECONNABORTED when the connection already
 * closed and -EBUSY when sending data is not permitted in this state or
 * LLC has send an I pdu with p bit set to 1 and is waiting for it's
 * response.
 *
 * This function always consumes a reference to the skb.
 */
pub unsafe fn llc_build_and_send_pkt(sk: *mut sock, skb: *mut sk_buff) -> i32 {
    let mut ev: *mut llc_conn_state_ev;
    let mut rc: i32 = -ECONNABORTED;
    let llc: *mut llc_sock = llc_sk(sk);

    if unlikely((*llc).state == LLC_CONN_STATE_ADM) {
        kfree_skb(skb);
        return rc;
    }
    rc = -EBUSY;
    if unlikely(llc_data_accept_state((*llc).state) || (*llc).p_flag != 0) {
        (*llc).failed_data_req = 1;
        kfree_skb(skb);
        return rc;
    }
    ev = llc_conn_ev(skb);
    (*ev).type = LLC_CONN_EV_TYPE_PRIM;
    (*ev).prim = LLC_DATA_PRIM;
    (*ev).prim_type = LLC_PRIM_TYPE_REQ;
    (*skb).dev = (*llc).dev;
    return llc_conn_state_process(sk, skb);

}

/** Establish an LLC connection with a remote machine. */
pub unsafe fn llc_establish_connection(
    mut sk: *mut sock,
    lmac: *const u8,
    dmac: *mut u8,
    dsap: u8,
) -> i32 {
    let mut rc: i32 = -EISCONN;
    let mut laddr: llc_addr = core::mem::zeroed();
    let mut daddr: llc_addr = core::mem::zeroed();
    let skb: *mut sk_buff;
    let llc: *mut llc_sock = llc_sk(sk);
    let existing: *mut sock;

    laddr.lsap = (*(*llc).sap).laddr.lsap;
    daddr.lsap = dsap;
    core::ptr::copy_nonoverlapping(dmac, daddr.mac.as_mut_ptr(), daddr.mac.len());
    core::ptr::copy_nonoverlapping(lmac, laddr.mac.as_mut_ptr(), laddr.mac.len());
    existing = llc_lookup_established((*llc).sap, &mut daddr, &mut laddr, sock_net(sk));
    if !existing.is_null() && (*existing).sk_state == TCP_ESTABLISHED {
        sk = existing;
    } else {
        if !existing.is_null() {
            sock_put(existing);
        }
        sock_hold(sk);
        rc = -ENOMEM;
        skb = alloc_skb(0, GFP_ATOMIC);
        if !skb.is_null() {
            let ev: *mut llc_conn_state_ev = llc_conn_ev(skb);
            (*ev).type = LLC_CONN_EV_TYPE_PRIM;
            (*ev).prim = LLC_CONN_PRIM;
            (*ev).prim_type = LLC_PRIM_TYPE_REQ;
            skb_set_owner_w(skb, sk);
            rc = llc_conn_state_process(sk, skb);
        }
    }
    sock_put(sk);
    return rc;
}

/** Close an established LLC connection. */
pub unsafe fn llc_send_disc(sk: *mut sock) -> u16 {
    let mut rc: u16 = 1;
    let ev: *mut llc_conn_state_ev;
    let skb: *mut sk_buff;

    sock_hold(sk);
    if (*sk).sk_type != SOCK_STREAM
        || (*sk).sk_state != TCP_ESTABLISHED
        || (*llc_sk(sk)).state == LLC_CONN_STATE_ADM
        || (*llc_sk(sk)).state == LLC_CONN_OUT_OF_SVC
    {
        sock_put(sk);
        return rc;
    }
    /*
     * Postpone unassigning the connection from its SAP and returning the
     * connection until all ACTIONs have been completely executed
     */
    skb = alloc_skb(0, GFP_ATOMIC);
    if skb.is_null() {
        sock_put(sk);
        return rc;
    }
    skb_set_owner_w(skb, sk);
    (*sk).sk_state = TCP_CLOSING;
    ev = llc_conn_ev(skb);
    (*ev).type = LLC_CONN_EV_TYPE_PRIM;
    (*ev).prim = LLC_DISC_PRIM;
    (*ev).prim_type = LLC_PRIM_TYPE_REQ;
    rc = llc_conn_state_process(sk, skb) as u16;
    sock_put(sk);
    return rc;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
