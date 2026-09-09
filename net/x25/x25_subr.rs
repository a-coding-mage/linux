// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * X.25 Packet Layer release 002
 *
 * This is ALPHA test software. This code may break your machine,
 * randomly fail to work with new releases, misbehave and/or generally
 * screw up. It might even work.
 *
 * This code REQUIRES 2.1.15 or higher
 *
 * History
 * X.25 001 Jonathan Naylor Started coding.
 * X.25 002 Jonathan Naylor Centralised disconnection processing.
 * mar/20/00 Daniela Squassoni Disabling/enabling of facilities negotiation.
 * jun/24/01 Arnaldo C. Melo use skb_queue_purge, cleanups
 * apr/04/15 Shaun Pereira Fast select with no restriction on response.
 */

/* #define pr_fmt(fmt) "X25: " fmt */
/* Linux kernel headers and net/x25.h supply the declarations used below. */

pub unsafe fn x25_clear_queues(sk: *mut sock) {
    let x25 = x25_sk(sk);

    skb_queue_purge(&mut (*sk).sk_write_queue);
    skb_queue_purge(&mut (*x25).ack_queue);
    skb_queue_purge(&mut (*x25).interrupt_in_queue);
    skb_queue_purge(&mut (*x25).interrupt_out_queue);
    skb_queue_purge(&mut (*x25).fragment_queue);
    (*x25).fraglen = 0;
}

pub unsafe fn x25_frames_acked(sk: *mut sock, nr: u16) {
    let x25 = x25_sk(sk);
    let modulus = if (*(*x25).neighbour).extended { X25_EMODULUS } else { X25_SMODULUS };

    if (*x25).va != nr {
        while !skb_peek(&mut (*x25).ack_queue).is_null() && (*x25).va != nr {
            let skb = skb_dequeue(&mut (*x25).ack_queue);
            kfree_skb(skb);
            (*x25).va = ((*x25).va + 1) % modulus;
        }
    }
}

pub unsafe fn x25_requeue_frames(sk: *mut sock) {
    let mut skb_prev: *mut sk_buff = core::ptr::null_mut();
    let x25 = x25_sk(sk);

    while {
        let skb = skb_dequeue(&mut (*x25).ack_queue);
        if skb.is_null() { false } else {
            if skb_prev.is_null() {
                skb_queue_head(&mut (*sk).sk_write_queue, skb);
            } else {
                skb_append(skb_prev, skb, &mut (*sk).sk_write_queue);
            }
            skb_prev = skb;
            true
        }
    } {}
}

pub unsafe fn x25_validate_nr(sk: *mut sock, nr: u16) -> i32 {
    let x25 = x25_sk(sk);
    let modulus = if (*(*x25).neighbour).extended { X25_EMODULUS } else { X25_SMODULUS };
    let mut vc = (*x25).va;

    while vc != (*x25).vs {
        if nr == vc { return 1; }
        vc = (vc + 1) % modulus;
    }
    if nr == (*x25).vs { 1 } else { 0 }
}

pub unsafe fn x25_write_internal(sk: *mut sock, frametype: i32) {
    let x25 = x25_sk(sk);
    let mut facilities = [0u8; X25_MAX_FAC_LEN as usize];
    let mut addresses = [0u8; (1 + X25_ADDR_LEN) as usize];
    let mut len = X25_MAX_L2_LEN + X25_EXT_MIN_LEN;

    match frametype {
        X25_CALL_REQUEST => len += 1 + X25_ADDR_LEN + X25_MAX_FAC_LEN + X25_MAX_CUD_LEN,
        X25_CALL_ACCEPTED => {
            len += if (*x25).facilities.reverse & 0x80 != 0 { 1 + X25_MAX_FAC_LEN + X25_MAX_CUD_LEN } else { 1 + X25_MAX_FAC_LEN };
        }
        X25_CLEAR_REQUEST | X25_RESET_REQUEST => len += 2,
        X25_RR | X25_RNR | X25_REJ | X25_CLEAR_CONFIRMATION | X25_INTERRUPT_CONFIRMATION | X25_RESET_CONFIRMATION => {}
        _ => { pr_err!("invalid frame type {:02X}\n", frametype); return; }
    }

    let skb = alloc_skb(len, GFP_ATOMIC);
    if skb.is_null() { return; }
    skb_reserve(skb, X25_MAX_L2_LEN);
    let dptr = skb_put(skb, 2);
    let lci1 = ((*x25).lci >> 8) & 0x0F;
    let lci2 = (*x25).lci & 0xFF;
    *dptr = lci1 as u8 | if (*(*x25).neighbour).extended { X25_GFI_EXTSEQ as u8 } else { X25_GFI_STDSEQ as u8 };
    *dptr.add(1) = lci2 as u8;

    match frametype {
        X25_CALL_REQUEST => {
            *skb_put(skb, 1) = X25_CALL_REQUEST as u8;
            len = x25_addr_aton(addresses.as_mut_ptr(), &(*x25).dest_addr, &(*x25).source_addr);
            skb_put_data(skb, addresses.as_ptr(), len);
            len = x25_create_facilities(facilities.as_mut_ptr(), &(*x25).facilities, &(*x25).dte_facilities, (*(*x25).neighbour).global_facil_mask);
            skb_put_data(skb, facilities.as_ptr(), len);
            skb_put_data(skb, (*x25).calluserdata.cuddata.as_ptr(), (*x25).calluserdata.cudlength);
            (*x25).calluserdata.cudlength = 0;
        }
        X25_CALL_ACCEPTED => {
            let p = skb_put(skb, 2); *p = X25_CALL_ACCEPTED as u8; *p.add(1) = 0;
            len = x25_create_facilities(facilities.as_mut_ptr(), &(*x25).facilities, &(*x25).dte_facilities, (*x25).vc_facil_mask);
            skb_put_data(skb, facilities.as_ptr(), len);
            if (*x25).facilities.reverse & 0x80 != 0 { skb_put_data(skb, (*x25).calluserdata.cuddata.as_ptr(), (*x25).calluserdata.cudlength); }
            (*x25).calluserdata.cudlength = 0;
        }
        X25_CLEAR_REQUEST => { let p = skb_put(skb, 3); *p = frametype as u8; *p.add(1) = (*x25).causediag.cause; *p.add(2) = (*x25).causediag.diagnostic; }
        X25_RESET_REQUEST => { let p = skb_put(skb, 3); *p = frametype as u8; *p.add(1) = 0; *p.add(2) = 0; }
        X25_RR | X25_RNR | X25_REJ => {
            if (*(*x25).neighbour).extended { let p = skb_put(skb, 2); *p = frametype as u8; *p.add(1) = ((*x25).vr << 1) as u8 & 0xFE; }
            else { let p = skb_put(skb, 1); *p = frametype as u8 | (((*x25).vr << 5) as u8 & 0xE0); }
        }
        X25_CLEAR_CONFIRMATION | X25_INTERRUPT_CONFIRMATION | X25_RESET_CONFIRMATION => *skb_put(skb, 1) = frametype as u8,
        _ => {}
    }
    x25_transmit_link(skb, (*x25).neighbour);
}

pub unsafe fn x25_decode(sk: *mut sock, skb: *mut sk_buff, ns: *mut i32, nr: *mut i32, q: *mut i32, d: *mut i32, m: *mut i32) -> i32 {
    let x25 = x25_sk(sk);
    if !pskb_may_pull(skb, X25_STD_MIN_LEN) { return X25_ILLEGAL; }
    let mut frame = (*skb).data;
    *ns = 0; *nr = 0; *q = 0; *d = 0; *m = 0;
    match *frame.add(2) {
        X25_CALL_REQUEST | X25_CALL_ACCEPTED | X25_CLEAR_REQUEST | X25_CLEAR_CONFIRMATION | X25_INTERRUPT | X25_INTERRUPT_CONFIRMATION | X25_RESET_REQUEST | X25_RESET_CONFIRMATION | X25_RESTART_REQUEST | X25_RESTART_CONFIRMATION | X25_REGISTRATION_REQUEST | X25_REGISTRATION_CONFIRMATION | X25_DIAGNOSTIC => return *frame.add(2) as i32,
        _ => {}
    }
    if (*(*x25).neighbour).extended {
        if *frame.add(2) == X25_RR || *frame.add(2) == X25_RNR || *frame.add(2) == X25_REJ { if !pskb_may_pull(skb, X25_EXT_MIN_LEN) { return X25_ILLEGAL; } frame = (*skb).data; *nr = ((*frame.add(3) >> 1) & 0x7F) as i32; return *frame.add(2) as i32; }
    } else if (*frame.add(2) & 0x1F) == X25_RR || (*frame.add(2) & 0x1F) == X25_RNR || (*frame.add(2) & 0x1F) == X25_REJ { *nr = ((*frame.add(2) >> 5) & 7) as i32; return (*frame.add(2) & 0x1F) as i32; }
    if (*frame.add(2) & 1) == X25_DATA {
        if (*(*x25).neighbour).extended { if !pskb_may_pull(skb, X25_EXT_MIN_LEN) { return X25_ILLEGAL; } frame = (*skb).data; *m = ((frame.add(3).read() & X25_EXT_M_BIT) == X25_EXT_M_BIT) as i32; *nr = ((frame.add(3).read() >> 1) & 0x7F) as i32; *ns = ((frame.add(2).read() >> 1) & 0x7F) as i32; }
        else { *m = ((frame.add(2).read() & X25_STD_M_BIT) == X25_STD_M_BIT) as i32; *nr = ((frame.add(2).read() >> 5) & 7) as i32; *ns = ((frame.add(2).read() >> 1) & 7) as i32; }
        *q = ((frame.read() & X25_Q_BIT) == X25_Q_BIT) as i32; *d = ((frame.read() & X25_D_BIT) == X25_D_BIT) as i32; return X25_DATA;
    }
    pr_debug!("invalid PLP frame %3ph\n", frame); X25_ILLEGAL
}

pub unsafe fn x25_disconnect(sk: *mut sock, reason: i32, cause: u8, diagnostic: u8) {
    let x25 = x25_sk(sk); x25_clear_queues(sk); x25_stop_timer(sk); (*x25).lci = 0; (*x25).state = X25_STATE_0; (*x25).causediag.cause = cause; (*x25).causediag.diagnostic = diagnostic; (*sk).sk_state = TCP_CLOSE; (*sk).sk_err = reason; (*sk).sk_shutdown |= SEND_SHUTDOWN;
    if !sock_flag(sk, SOCK_DEAD) { ((*sk).sk_state_change)(sk); sock_set_flag(sk, SOCK_DEAD); }
    if !(*x25).neighbour.is_null() { read_lock_bh(&x25_list_lock); x25_neigh_put((*x25).neighbour); (*x25).neighbour = core::ptr::null_mut(); read_unlock_bh(&x25_list_lock); }
}

pub unsafe fn x25_check_rbuf(sk: *mut sock) {
    let x25 = x25_sk(sk);
    if atomic_read(&(*sk).sk_rmem_alloc) < ((*sk).sk_rcvbuf >> 1) && ((*x25).condition & X25_COND_OWN_RX_BUSY) != 0 { (*x25).condition &= !X25_COND_OWN_RX_BUSY; (*x25).condition &= !X25_COND_ACK_PENDING; (*x25).vl = (*x25).vr; x25_write_internal(sk, X25_RR); x25_stop_timer(sk); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
