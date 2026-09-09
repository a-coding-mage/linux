// SPDX-License-Identifier: GPL-2.0-or-later
/* Peer event handling, typically ICMP messages.
 *
 * Copyright (C) 2007 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Kernel and RxRPC declarations are supplied by the surrounding translation.

/* Find the peer associated with a local error. */
unsafe fn rxrpc_lookup_peer_local_rcu(
    local: *mut rxrpc_local,
    skb: *const sk_buff,
    srx: *mut sockaddr_rxrpc,
) -> *mut rxrpc_peer {
    let serr = SKB_EXT_ERR(skb);

    _enter!("");

    core::ptr::write_bytes(srx, 0, 1);
    (*srx).transport_type = (*local).srx.transport_type;
    (*srx).transport_len = (*local).srx.transport_len;
    (*srx).transport.family = (*local).srx.transport.family;

    /* Can we see an ICMP4 packet on an ICMP6 listening socket?  and vice
     * versa?
     */
    match (*srx).transport.family {
        AF_INET => {
            (*srx).transport_len = core::mem::size_of_val(&(*srx).transport.sin);
            (*srx).transport.family = AF_INET;
            (*srx).transport.sin.sin_port = (*serr).port;
            match (*serr).ee.ee_origin {
                SO_EE_ORIGIN_ICMP => {
                    core::ptr::copy_nonoverlapping(
                        skb_network_header(skb).add((*serr).addr_offset as usize),
                        &mut (*srx).transport.sin.sin_addr as *mut _ as *mut u8,
                        core::mem::size_of::<in_addr>(),
                    );
                }
                SO_EE_ORIGIN_ICMP6 => {
                    core::ptr::copy_nonoverlapping(
                        skb_network_header(skb).add((*serr).addr_offset as usize + 12),
                        &mut (*srx).transport.sin.sin_addr as *mut _ as *mut u8,
                        core::mem::size_of::<in_addr>(),
                    );
                }
                _ => {
                    core::ptr::copy_nonoverlapping(
                        &ip_hdr(skb).saddr as *const _ as *const u8,
                        &mut (*srx).transport.sin.sin_addr as *mut _ as *mut u8,
                        core::mem::size_of::<in_addr>(),
                    );
                }
            }
        }
        // Preserved from CONFIG_AF_RXRPC_IPV6; the surrounding build selects this branch.
        AF_INET6 => match (*serr).ee.ee_origin {
            SO_EE_ORIGIN_ICMP6 => {
                (*srx).transport.sin6.sin6_port = (*serr).port;
                core::ptr::copy_nonoverlapping(
                    skb_network_header(skb).add((*serr).addr_offset as usize),
                    &mut (*srx).transport.sin6.sin6_addr as *mut _ as *mut u8,
                    core::mem::size_of::<in6_addr>(),
                );
            }
            SO_EE_ORIGIN_ICMP => {
                (*srx).transport_len = core::mem::size_of_val(&(*srx).transport.sin);
                (*srx).transport.family = AF_INET;
                (*srx).transport.sin.sin_port = (*serr).port;
                core::ptr::copy_nonoverlapping(
                    skb_network_header(skb).add((*serr).addr_offset as usize),
                    &mut (*srx).transport.sin.sin_addr as *mut _ as *mut u8,
                    core::mem::size_of::<in_addr>(),
                );
            }
            _ => {
                core::ptr::copy_nonoverlapping(
                    &ipv6_hdr(skb).saddr as *const _ as *const u8,
                    &mut (*srx).transport.sin6.sin6_addr as *mut _ as *mut u8,
                    core::mem::size_of::<in6_addr>(),
                );
            }
        },
        _ => BUG!(),
    }

    rxrpc_lookup_peer_rcu(local, srx)
}

/* Handle an MTU/fragmentation problem. */
unsafe fn rxrpc_adjust_mtu(peer: *mut rxrpc_peer, mut mtu: u32) {
    let mut max_data: u32;

    /* wind down the local interface MTU */
    if mtu > 0 && (*peer).if_mtu == 65535 && mtu < (*peer).if_mtu {
        (*peer).if_mtu = mtu;
    }

    if mtu == 0 {
        /* they didn't give us a size, estimate one */
        mtu = (*peer).if_mtu;
        if mtu > 1500 {
            mtu >>= 1;
            if mtu < 1500 { mtu = 1500; }
        } else {
            mtu -= 100;
            if mtu < (*peer).hdrsize { mtu = (*peer).hdrsize + 4; }
        }
    }

    max_data = core::cmp::max(mtu.saturating_sub((*peer).hdrsize), 500);
    if max_data < (*peer).max_data {
        if (*peer).pmtud_good > max_data { (*peer).pmtud_good = max_data; }
        if (*peer).pmtud_bad > max_data + 1 { (*peer).pmtud_bad = max_data + 1; }
        trace_rxrpc_pmtud_reduce(peer, 0, max_data, rxrpc_pmtud_reduce_icmp);
        (*peer).max_data = max_data;
    }
}

/* Handle an error received on the local endpoint. */
pub unsafe fn rxrpc_input_error(local: *mut rxrpc_local, skb: *mut sk_buff) {
    let serr = SKB_EXT_ERR(skb);
    let mut srx: sockaddr_rxrpc = core::mem::zeroed();
    let mut peer: *mut rxrpc_peer = core::ptr::null_mut();

    _enter!("L=%x", (*local).debug_id);
    if (*skb).len == 0 && (*serr).ee.ee_origin == SO_EE_ORIGIN_TIMESTAMPING { _leave!("UDP empty message"); return; }

    rcu_read_lock();
    peer = rxrpc_lookup_peer_local_rcu(local, skb, &mut srx);
    if !peer.is_null() && !rxrpc_get_peer_maybe(peer, rxrpc_peer_get_input_error) { peer = core::ptr::null_mut(); }
    rcu_read_unlock();
    if peer.is_null() { return; }

    trace_rxrpc_rx_icmp(peer, &(*serr).ee, &srx);
    if (*serr).ee.ee_origin == SO_EE_ORIGIN_ICMP && (*serr).ee.ee_type == ICMP_DEST_UNREACH && (*serr).ee.ee_code == ICMP_FRAG_NEEDED { rxrpc_adjust_mtu(peer, (*serr).ee.ee_info); rxrpc_put_peer(peer, rxrpc_peer_put_input_error); return; }
    if (*serr).ee.ee_origin == SO_EE_ORIGIN_ICMP6 && (*serr).ee.ee_type == ICMPV6_PKT_TOOBIG && (*serr).ee.ee_code == 0 { rxrpc_adjust_mtu(peer, (*serr).ee.ee_info); rxrpc_put_peer(peer, rxrpc_peer_put_input_error); return; }
    rxrpc_store_error(peer, skb);
    rxrpc_put_peer(peer, rxrpc_peer_put_input_error);
}

/* Map an error report to error codes on the peer record. */
unsafe fn rxrpc_store_error(peer: *mut rxrpc_peer, skb: *mut sk_buff) {
    let mut compl = RXRPC_CALL_NETWORK_ERROR;
    let serr = SKB_EXT_ERR(skb);
    let ee = &(*serr).ee;
    let mut err = ee.ee_errno;
    _enter!("");
    match ee.ee_origin {
        SO_EE_ORIGIN_NONE | SO_EE_ORIGIN_LOCAL => compl = RXRPC_CALL_LOCAL_ERROR,
        SO_EE_ORIGIN_ICMP6 => { if err == EACCES { err = EHOSTUNREACH; } }
        _ => {}
    }
    rxrpc_distribute_error(peer, skb, compl, err);
}

/* Distribute an error that occurred on a peer. */
unsafe fn rxrpc_distribute_error(peer: *mut rxrpc_peer, _skb: *mut sk_buff, compl: rxrpc_call_completion, err: i32) {
    let mut error_targets: hlist_head = HLIST_HEAD_INIT;
    spin_lock_irq(&mut (*peer).lock);
    hlist_move_list(&mut (*peer).error_targets, &mut error_targets);
    while !hlist_empty(&error_targets) {
        let call = hlist_entry(error_targets.first, rxrpc_call, error_link);
        hlist_del_init(&mut (*call).error_link);
        spin_unlock_irq(&mut (*peer).lock);
        rxrpc_see_call(call, rxrpc_call_see_distribute_error);
        rxrpc_set_call_completion(call, compl, 0, -err);
        rxrpc_input_call_event(call);
        spin_lock_irq(&mut (*peer).lock);
    }
    spin_unlock_irq(&mut (*peer).lock);
}

/* Reconstruct the last transmission time. */
unsafe fn rxrpc_peer_get_tx_mark(peer: *const rxrpc_peer, base: time64_t) -> time64_t {
    let last_tx_at: i32 = READ_ONCE!((*peer).last_tx_at);
    let base_lsw: i32 = base as i32;
    let mut diff = last_tx_at.wrapping_sub(base_lsw);
    diff = diff.clamp(-RXRPC_KEEPALIVE_TIME, RXRPC_KEEPALIVE_TIME);
    diff as time64_t + base
}

/* Perform keep-alive pings. */
unsafe fn rxrpc_peer_keepalive_dispatch(rxnet: *mut rxrpc_net, collector: *mut list_head, base: time64_t, cursor: u8) {
    let mask = core::mem::size_of_val(&(*rxnet).peer_keepalive) as u8 - 1;
    spin_lock_bh(&mut (*rxnet).peer_hash_lock);
    while !list_empty(collector) {
        let peer = list_entry((*collector).next, rxrpc_peer, keepalive_link);
        list_del_init(&mut (*peer).keepalive_link);
        if !rxrpc_get_peer_maybe(peer, rxrpc_peer_get_keepalive) { continue; }
        let use_local = __rxrpc_use_local((*peer).local, rxrpc_local_use_peer_keepalive);
        spin_unlock_bh(&mut (*rxnet).peer_hash_lock);
        if use_local {
            let keepalive_at = rxrpc_peer_get_tx_mark(peer, base) + RXRPC_KEEPALIVE_TIME;
            let mut slot = (keepalive_at - base) as i32;
            _debug!("%02x peer %u t=%d {%pISp}", cursor, (*peer).debug_id, slot, &(*peer).srx.transport);
            if keepalive_at <= base || keepalive_at > base + RXRPC_KEEPALIVE_TIME { rxrpc_send_keepalive(peer); slot = RXRPC_KEEPALIVE_TIME; }
            slot = (slot + cursor as i32) & mask as i32;
            spin_lock_bh(&mut (*rxnet).peer_hash_lock);
            list_add_tail(&mut (*peer).keepalive_link, &mut (*rxnet).peer_keepalive[(slot as usize) & mask as usize]);
            spin_unlock_bh(&mut (*rxnet).peer_hash_lock);
            rxrpc_unuse_local((*peer).local, rxrpc_local_unuse_peer_keepalive);
        }
        rxrpc_put_peer(peer, rxrpc_peer_put_keepalive);
        spin_lock_bh(&mut (*rxnet).peer_hash_lock);
    }
    spin_unlock_bh(&mut (*rxnet).peer_hash_lock);
}

/* Perform keep-alive pings with VERSION packets to keep any NAT alive. */
pub unsafe fn rxrpc_peer_keepalive_worker(work: *mut work_struct) {
    let rxnet = container_of!(work, rxrpc_net, peer_keepalive_work);
    let mask = core::mem::size_of_val(&(*rxnet).peer_keepalive) as u8 - 1;
    let mut base: time64_t;
    let mut now: time64_t;
    let mut delay: time64_t;
    let mut cursor: u8;
    let mut stop: u8;
    let mut collector: list_head = LIST_HEAD_INIT;
    now = ktime_get_seconds(); base = (*rxnet).peer_keepalive_base; cursor = (*rxnet).peer_keepalive_cursor;
    _enter!("%lld,%u", base - now, cursor);
    if !(*rxnet).live { return; }
    spin_lock_bh(&mut (*rxnet).peer_hash_lock);
    list_splice_init(&mut (*rxnet).peer_keepalive_new, &mut collector);
    stop = cursor.wrapping_add(core::mem::size_of_val(&(*rxnet).peer_keepalive) as u8);
    while base <= now && (cursor.wrapping_sub(stop) as i8) < 0 { list_splice_tail_init(&mut (*rxnet).peer_keepalive[(cursor & mask) as usize], &mut collector); base += 1; cursor = cursor.wrapping_add(1); }
    base = now; spin_unlock_bh(&mut (*rxnet).peer_hash_lock);
    (*rxnet).peer_keepalive_base = base; (*rxnet).peer_keepalive_cursor = cursor;
    rxrpc_peer_keepalive_dispatch(rxnet, &mut collector, base, cursor);
    ASSERT!(list_empty(&collector));
    cursor = (*rxnet).peer_keepalive_cursor; stop = cursor.wrapping_add(RXRPC_KEEPALIVE_TIME as u8 - 1);
    while (cursor.wrapping_sub(stop) as i8) < 0 { if !list_empty(&(*rxnet).peer_keepalive[(cursor & mask) as usize]) { break; } base += 1; cursor = cursor.wrapping_add(1); }
    now = ktime_get_seconds(); delay = base - now; if delay < 1 { delay = 1; } delay *= HZ;
    if (*rxnet).live { timer_reduce(&mut (*rxnet).peer_keepalive_timer, jiffies + delay); }
    _leave!("");
}

/* Do path MTU probing. */
pub unsafe fn rxrpc_input_probe_for_pmtud(conn: *mut rxrpc_connection, acked_serial: rxrpc_serial_t, sendmsg_fail: bool) {
    let peer = (*conn).peer; let mut max_data = (*peer).max_data;
    let mut good = (*peer).pmtud_good; let mut trial = (*peer).pmtud_trial; let mut bad = (*peer).pmtud_bad; let mut jumbo;
    if good >= bad - 1 { (*conn).pmtud_probe = 0; (*peer).pmtud_lost = false; return; }
    if !(*peer).pmtud_probing { (*peer).pmtud_pending = true; return; }
    if sendmsg_fail || after(acked_serial, (*conn).pmtud_probe) {
        if !(*peer).pmtud_lost { trace_rxrpc_pmtud_lost(conn, acked_serial); (*conn).pmtud_probe = 0; (*peer).pmtud_lost = true; (*peer).pmtud_pending = true; return; }
        bad = trial; (*peer).pmtud_bad = bad; if bad <= max_data { max_data = bad - 1; }
    } else { good = trial; (*peer).pmtud_good = good; if good > max_data { max_data = good; } }
    max_data = core::cmp::min(max_data, (*peer).ackr_max_data); if max_data != (*peer).max_data { (*peer).max_data = max_data; }
    jumbo = (max_data + core::mem::size_of::<rxrpc_jumbo_header>() as u32) / RXRPC_JUMBO_SUBPKTLEN; (*peer).pmtud_jumbo = jumbo;
    trace_rxrpc_pmtud_rx(conn, acked_serial); (*conn).pmtud_probe = 0; (*peer).pmtud_lost = false;
    if good < RXRPC_JUMBO!(2) && bad > RXRPC_JUMBO!(2) { trial = RXRPC_JUMBO!(2); } else if good < RXRPC_JUMBO!(4) && bad > RXRPC_JUMBO!(4) { trial = RXRPC_JUMBO!(4); } else if good < RXRPC_JUMBO!(3) && bad > RXRPC_JUMBO!(3) { trial = RXRPC_JUMBO!(3); } else if good < RXRPC_JUMBO!(6) && bad > RXRPC_JUMBO!(6) { trial = RXRPC_JUMBO!(6); } else if good < RXRPC_JUMBO!(5) && bad > RXRPC_JUMBO!(5) { trial = RXRPC_JUMBO!(5); } else if good < RXRPC_JUMBO!(8) && bad > RXRPC_JUMBO!(8) { trial = RXRPC_JUMBO!(8); } else if good < RXRPC_JUMBO!(7) && bad > RXRPC_JUMBO!(7) { trial = RXRPC_JUMBO!(7); } else { trial = (good + bad) / 2; }
    (*peer).pmtud_trial = trial; if good >= bad { return; } (*peer).pmtud_pending = true;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
