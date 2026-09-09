// SPDX-License-Identifier: GPL-2.0
/*
 * llc_sap.c - driver routines for SAP component.
 *
 * Copyright (c) 1997 by Procom Technology, Inc.
 *              2001-2003 by Arnaldo Carvalho de Melo <acme@conectiva.com.br>
 */

// C dependencies supplied by the surrounding kernel translation.

unsafe fn llc_mac_header_len(devtype: u16) -> i32 {
    match devtype {
        ARPHRD_ETHER | ARPHRD_LOOPBACK => core::mem::size_of::<ethhdr>() as i32,
        _ => 0,
    }
}

/**
 * llc_alloc_frame - allocates sk_buff for frame
 * @sk:  socket to allocate frame to
 * @dev: network device this skb will be sent over
 * @type: pdu type to allocate
 * @data_size: data size to allocate
 *
 * Allocates an sk_buff for frame and initializes sk_buff fields.
 * Returns allocated skb or %NULL when out of memory.
 */
pub unsafe fn llc_alloc_frame(
    sk: *mut sock,
    dev: *mut net_device,
    type_: u8,
    data_size: u32,
) -> *mut sk_buff {
    let mut hlen: i32 = if type_ == LLC_PDU_TYPE_U { 3 } else { 4 };
    let mut skb: *mut sk_buff;

    hlen += llc_mac_header_len((*dev).type_);
    skb = alloc_skb((hlen as u32).wrapping_add(data_size), GFP_ATOMIC);

    if !skb.is_null() {
        skb_reset_mac_header(skb);
        skb_reserve(skb, hlen as u32);
        skb_reset_network_header(skb);
        skb_reset_transport_header(skb);
        (*skb).protocol = htons(ETH_P_802_2);
        (*skb).dev = dev;
        if !sk.is_null() {
            skb_set_owner_w(skb, sk);
        }
    }
    skb
}

pub unsafe fn llc_save_primitive(sk: *mut sock, skb: *mut sk_buff, prim: u8) {
    let addr: *mut sockaddr_llc = llc_ui_skb_cb(skb);

    /* save primitive for use by the user. */
    core::ptr::write_bytes(addr as *mut u8, 0, core::mem::size_of::<sockaddr_llc>());
    (*addr).sllc_family = (*sk).sk_family;
    (*addr).sllc_arphrd = (*(*skb).dev).type_;
    (*addr).sllc_test = (prim == LLC_TEST_PRIM) as _;
    (*addr).sllc_xid = (prim == LLC_XID_PRIM) as _;
    (*addr).sllc_ua = (prim == LLC_DATAUNIT_PRIM) as _;
    llc_pdu_decode_sa(skb, (*addr).sllc_mac.as_mut_ptr());
    llc_pdu_decode_ssap(skb, &mut (*addr).sllc_sap);
}

/** Informs upper layer on rx of an UI, XID or TEST pdu. */
pub unsafe fn llc_sap_rtn_pdu(_sap: *mut llc_sap, skb: *mut sk_buff) {
    let ev: *mut llc_sap_state_ev = llc_sap_ev(skb);
    let pdu: *mut llc_pdu_un = llc_pdu_un_hdr(skb);

    match LLC_U_PDU_RSP(pdu) {
        LLC_1_PDU_CMD_TEST => (*ev).prim = LLC_TEST_PRIM,
        LLC_1_PDU_CMD_XID => (*ev).prim = LLC_XID_PRIM,
        LLC_1_PDU_CMD_UI => (*ev).prim = LLC_DATAUNIT_PRIM,
        _ => {}
    }
    (*ev).ind_cfm_flag = LLC_IND;
}

unsafe fn llc_find_sap_trans(
    sap: *mut llc_sap,
    skb: *mut sk_buff,
) -> *const llc_sap_state_trans {
    let mut i: usize = 0;
    let mut rc: *const llc_sap_state_trans = core::ptr::null();
    let curr_state: *mut llc_sap_state = &mut llc_sap_state_table[((*sap).state - 1) as usize];
    let next_trans: *mut *const llc_sap_state_trans = (*curr_state).transitions;

    /* Search thru events for this state until list exhausted or until
     * its obvious the event is not valid for the current state */
    while !(*next_trans.add(i)).is_null()
        && !((*(*next_trans.add(i)).add(0)).ev)(sap, skb)
    {
        rc = *next_trans.add(i);
        break;
    }
    rc
}

unsafe fn llc_exec_sap_trans_actions(
    sap: *mut llc_sap,
    trans: *const llc_sap_state_trans,
    skb: *mut sk_buff,
) -> i32 {
    let mut rc = 0;
    let mut next_action = (*trans).ev_actions;
    while !next_action.is_null() && !(*next_action).is_null() {
        if (*next_action)(sap, skb) != 0 {
            rc = 1;
        }
        next_action = next_action.add(1);
    }
    rc
}

unsafe fn llc_sap_next_state(sap: *mut llc_sap, skb: *mut sk_buff) -> i32 {
    let mut rc = 1;
    if (*sap).state > LLC_NR_SAP_STATES {
        return rc;
    }
    let trans = llc_find_sap_trans(sap, skb);
    if trans.is_null() {
        return rc;
    }
    rc = llc_exec_sap_trans_actions(sap, trans, skb);
    if rc != 0 {
        return rc;
    }
    (*sap).state = (*trans).next_state;
    rc
}

unsafe fn llc_sap_state_process(sap: *mut llc_sap, skb: *mut sk_buff) {
    let ev = llc_sap_ev(skb);
    (*ev).ind_cfm_flag = 0;
    llc_sap_next_state(sap, skb);

    if (*ev).ind_cfm_flag == LLC_IND
        && (*(*skb).sk).sk_state != TCP_LISTEN
    {
        llc_save_primitive((*skb).sk, skb, (*ev).prim);
        if sock_queue_rcv_skb((*skb).sk, skb) == 0 {
            return;
        }
    }
    kfree_skb(skb);
}

pub unsafe fn llc_build_and_send_test_pkt(
    sap: *mut llc_sap,
    skb: *mut sk_buff,
    dmac: *mut u8,
    dsap: u8,
) {
    let ev = llc_sap_ev(skb);
    (*ev).saddr.lsap = (*sap).laddr.lsap;
    (*ev).daddr.lsap = dsap;
    memcpy((*ev).saddr.mac.as_mut_ptr(), (*(*skb).dev).dev_addr, IFHWADDRLEN);
    memcpy((*ev).daddr.mac.as_mut_ptr(), dmac, IFHWADDRLEN);
    (*ev).type_ = LLC_SAP_EV_TYPE_PRIM;
    (*ev).prim = LLC_TEST_PRIM;
    (*ev).prim_type = LLC_PRIM_TYPE_REQ;
    llc_sap_state_process(sap, skb);
}

pub unsafe fn llc_build_and_send_xid_pkt(
    sap: *mut llc_sap,
    skb: *mut sk_buff,
    dmac: *mut u8,
    dsap: u8,
) {
    let ev = llc_sap_ev(skb);
    (*ev).saddr.lsap = (*sap).laddr.lsap;
    (*ev).daddr.lsap = dsap;
    memcpy((*ev).saddr.mac.as_mut_ptr(), (*(*skb).dev).dev_addr, IFHWADDRLEN);
    memcpy((*ev).daddr.mac.as_mut_ptr(), dmac, IFHWADDRLEN);
    (*ev).type_ = LLC_SAP_EV_TYPE_PRIM;
    (*ev).prim = LLC_XID_PRIM;
    (*ev).prim_type = LLC_PRIM_TYPE_REQ;
    llc_sap_state_process(sap, skb);
}

unsafe fn llc_sap_rcv(sap: *mut llc_sap, skb: *mut sk_buff, sk: *mut sock) {
    let ev = llc_sap_ev(skb);
    (*ev).type_ = LLC_SAP_EV_TYPE_PDU;
    (*ev).reason = 0;
    skb_orphan(skb);
    sock_hold(sk);
    (*skb).sk = sk;
    (*skb).destructor = Some(sock_efree);
    llc_sap_state_process(sap, skb);
}

unsafe fn llc_dgram_match(
    _sap: *const llc_sap,
    laddr: *const llc_addr,
    sk: *const sock,
    net: *const net,
) -> bool {
    let llc = llc_sk(sk);
    (*sk).sk_type == SOCK_DGRAM
        && net_eq(sock_net(sk), net)
        && (*llc).laddr.lsap == (*laddr).lsap
        && ether_addr_equal((*llc).laddr.mac.as_ptr(), (*laddr).mac.as_ptr())
}

unsafe fn llc_lookup_dgram(
    sap: *mut llc_sap,
    laddr: *const llc_addr,
    net: *const net,
) -> *mut sock {
    let slot = llc_sk_laddr_hashfn(sap, laddr);
    let laddr_hb = &mut (*sap).sk_laddr_hash[slot as usize];
    let mut rc: *mut sock = core::ptr::null_mut();
    let mut node: *mut hlist_nulls_node = core::ptr::null_mut();

    rcu_read_lock_bh();
    'again: loop {
        sk_nulls_for_each_rcu!(rc, node, laddr_hb, {
            if llc_dgram_match(sap, laddr, rc, net) {
                if unlikely(!refcount_inc_not_zero(&mut (*rc).sk_refcnt)) {
                    continue 'again;
                }
                if unlikely((*llc_sk(rc)).sap != sap
                    || !llc_dgram_match(sap, laddr, rc, net))
                {
                    sock_put(rc);
                    continue;
                }
                rcu_read_unlock_bh();
                return rc;
            }
        });
        rc = core::ptr::null_mut();
        if unlikely(get_nulls_value(node) != slot) {
            continue 'again;
        }
        break;
    }
    rcu_read_unlock_bh();
    rc
}

unsafe fn llc_mcast_match(
    _sap: *const llc_sap,
    laddr: *const llc_addr,
    skb: *const sk_buff,
    sk: *const sock,
) -> bool {
    let llc = llc_sk(sk);
    (*sk).sk_type == SOCK_DGRAM
        && (*llc).laddr.lsap == (*laddr).lsap
        && (*llc).dev == (*skb).dev
}

unsafe fn llc_do_mcast(
    sap: *mut llc_sap,
    skb: *mut sk_buff,
    stack: *mut *mut sock,
    count: i32,
) {
    for i in 0..count {
        let skb1 = skb_clone(skb, GFP_ATOMIC);
        if skb1.is_null() {
            sock_put(*stack.add(i as usize));
            continue;
        }
        llc_sap_rcv(sap, skb1, *stack.add(i as usize));
        sock_put(*stack.add(i as usize));
    }
}

unsafe fn llc_sap_mcast(sap: *mut llc_sap, laddr: *const llc_addr, skb: *mut sk_buff) {
    let mut i = 0;
    let mut stack: [*mut sock; 256 / core::mem::size_of::<*mut sock>()] =
        [core::ptr::null_mut(); 256 / core::mem::size_of::<*mut sock>()];
    let dev_hb = llc_sk_dev_hash(sap, (*(*skb).dev).ifindex);

    spin_lock_bh(&mut (*sap).sk_lock);
    hlist_for_each_entry!(llc, dev_hb, dev_hash_node, {
        let sk = &mut (*llc).sk as *mut sock;
        if !llc_mcast_match(sap, laddr, skb, sk) {
            continue;
        }
        sock_hold(sk);
        if i < stack.len() {
            stack[i] = sk;
            i += 1;
        } else {
            llc_do_mcast(sap, skb, stack.as_mut_ptr(), i as i32);
            i = 0;
        }
    });
    spin_unlock_bh(&mut (*sap).sk_lock);
    llc_do_mcast(sap, skb, stack.as_mut_ptr(), i as i32);
}

pub unsafe fn llc_sap_handler(sap: *mut llc_sap, skb: *mut sk_buff) {
    let mut laddr: llc_addr = core::mem::zeroed();
    llc_pdu_decode_da(skb, laddr.mac.as_mut_ptr());
    llc_pdu_decode_dsap(skb, &mut laddr.lsap);

    if is_multicast_ether_addr(laddr.mac.as_ptr()) {
        llc_sap_mcast(sap, &laddr, skb);
        kfree_skb(skb);
    } else {
        let sk = llc_lookup_dgram(sap, &laddr, dev_net((*skb).dev));
        if !sk.is_null() {
            llc_sap_rcv(sap, skb, sk);
            sock_put(sk);
        } else {
            kfree_skb(skb);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
