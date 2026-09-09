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
 * X.25 002 Jonathan Naylor Centralised disconnection code.
 *                         New timer architecture.
 * 2000-03-20 Daniela Squassoni Disabling/enabling of facilities
 *                         negotiation.
 * 2000-11-10 Henner Eisen Check and reset for out-of-sequence
 *                         i-frames.
 */

// Dependencies supplied by the kernel/X.25 implementation are intentionally external.

unsafe fn x25_queue_rx_frame(sk: *mut sock, skb: *mut sk_buff, more: c_int) -> c_int {
    let mut skbo: *mut sk_buff;
    let mut skbn = skb;
    let x25 = x25_sk(sk);

    if (*x25).fraglen + (*skb).len > USHRT_MAX {
        return 1;
    }

    if more != 0 {
        (*x25).fraglen += (*skb).len;
        skb_queue_tail(&mut (*x25).fragment_queue, skb);
        skb_set_owner_r(skb, sk);
        return 0;
    }

    if (*x25).fraglen > 0 {
        let len = (*x25).fraglen + (*skb).len;
        skbn = alloc_skb(len, GFP_ATOMIC);
        if skbn.is_null() {
            return 1;
        }

        skb_queue_tail(&mut (*x25).fragment_queue, skb);
        skb_reset_transport_header(skbn);

        skbo = skb_dequeue(&mut (*x25).fragment_queue);
        skb_copy_from_linear_data(skbo, skb_put(skbn, (*skbo).len), (*skbo).len);
        kfree_skb(skbo);

        loop {
            skbo = skb_dequeue(&mut (*x25).fragment_queue);
            if skbo.is_null() {
                break;
            }
            skb_pull(
                skbo,
                if (*(*x25).neighbour).extended { X25_EXT_MIN_LEN } else { X25_STD_MIN_LEN },
            );
            skb_copy_from_linear_data(skbo, skb_put(skbn, (*skbo).len), (*skbo).len);
            kfree_skb(skbo);
        }
        (*x25).fraglen = 0;
    }

    skb_set_owner_r(skbn, sk);
    skb_queue_tail(&mut (*sk).sk_receive_queue, skbn);
    if !sock_flag(sk, SOCK_DEAD) {
        ((*sk).sk_data_ready)(sk);
    }
    0
}

unsafe fn x25_state1_machine(sk: *mut sock, skb: *mut sk_buff, frametype: c_int) -> c_int {
    let mut source_addr = x25_address::default();
    let mut dest_addr = x25_address::default();
    let mut len: c_int;
    let x25 = x25_sk(sk);

    match frametype {
        X25_CALL_ACCEPTED => {
            x25_stop_timer(sk);
            (*x25).condition = 0x00;
            (*x25).vs = 0;
            (*x25).va = 0;
            (*x25).vr = 0;
            (*x25).vl = 0;
            (*x25).state = X25_STATE_3;
            (*sk).sk_state = TCP_ESTABLISHED;
            if !pskb_may_pull(skb, X25_STD_MIN_LEN) { goto out_clear; }
            skb_pull(skb, X25_STD_MIN_LEN);
            len = x25_parse_address_block(skb, &mut source_addr, &mut dest_addr);
            if len > 0 { skb_pull(skb, len as usize); } else if len < 0 { goto out_clear; }
            len = x25_parse_facilities(skb, &mut (*x25).facilities, &mut (*x25).dte_facilities, &mut (*x25).vc_facil_mask);
            if len > 0 { skb_pull(skb, len as usize); } else if len < 0 { goto out_clear; }
            if (*skb).len > 0 {
                if (*skb).len > X25_MAX_CUD_LEN { goto out_clear; }
                skb_copy_bits(skb, 0, (*x25).calluserdata.cuddata.as_mut_ptr().cast(), (*skb).len);
                (*x25).calluserdata.cudlength = (*skb).len;
            }
            if !sock_flag(sk, SOCK_DEAD) { ((*sk).sk_state_change)(sk); }
        }
        X25_CALL_REQUEST => {
            (*x25).causediag.cause = 0x01;
            (*x25).causediag.diagnostic = 0x48;
            x25_write_internal(sk, X25_CLEAR_REQUEST);
            x25_disconnect(sk, EISCONN, 0x01, 0x48);
        }
        X25_CLEAR_REQUEST => {
            if !pskb_may_pull(skb, X25_STD_MIN_LEN + 2) { goto out_clear; }
            x25_write_internal(sk, X25_CLEAR_CONFIRMATION);
            x25_disconnect(sk, ECONNREFUSED, (*skb).data[3] as c_int, (*skb).data[4] as c_int);
        }
        _ => {}
    }
    return 0;
out_clear:
    x25_write_internal(sk, X25_CLEAR_REQUEST);
    (*x25).state = X25_STATE_2;
    x25_start_t23timer(sk);
    0
}

unsafe fn x25_state2_machine(sk: *mut sock, skb: *mut sk_buff, frametype: c_int) -> c_int {
    match frametype {
        X25_CLEAR_REQUEST => {
            if !pskb_may_pull(skb, X25_STD_MIN_LEN + 2) { goto out_clear; }
            x25_write_internal(sk, X25_CLEAR_CONFIRMATION);
            x25_disconnect(sk, 0, (*skb).data[3] as c_int, (*skb).data[4] as c_int);
        }
        X25_CLEAR_CONFIRMATION => x25_disconnect(sk, 0, 0, 0),
        _ => {}
    }
    return 0;
out_clear:
    x25_write_internal(sk, X25_CLEAR_REQUEST);
    x25_start_t23timer(sk);
    0
}

unsafe fn x25_state3_machine(sk: *mut sock, skb: *mut sk_buff, frametype: c_int, ns: c_int, nr: c_int, _q: c_int, _d: c_int, m: c_int) -> c_int {
    let mut queued = 0;
    let x25 = x25_sk(sk);
    let modulus = if (*(*x25).neighbour).extended { X25_EMODULUS } else { X25_SMODULUS };
    match frametype {
        X25_RESET_REQUEST => { x25_write_internal(sk, X25_RESET_CONFIRMATION); x25_stop_timer(sk); (*x25).condition=0; (*x25).vs=0; (*x25).vr=0; (*x25).va=0; (*x25).vl=0; x25_requeue_frames(sk); }
        X25_CLEAR_REQUEST => { if !pskb_may_pull(skb, X25_STD_MIN_LEN + 2) { goto out_clear; } x25_write_internal(sk, X25_CLEAR_CONFIRMATION); x25_disconnect(sk, 0, (*skb).data[3] as c_int, (*skb).data[4] as c_int); }
        X25_RR | X25_RNR => {
            if !x25_validate_nr(sk, nr) { x25_clear_queues(sk); x25_write_internal(sk, X25_RESET_REQUEST); x25_start_t22timer(sk); (*x25).condition=0; (*x25).vs=0; (*x25).vr=0; (*x25).va=0; (*x25).vl=0; (*x25).state=X25_STATE_4; }
            else { x25_frames_acked(sk, nr); if frametype == X25_RNR { (*x25).condition |= X25_COND_PEER_RX_BUSY; } else { (*x25).condition &= !X25_COND_PEER_RX_BUSY; } }
        }
        X25_DATA => {
            (*x25).condition &= !X25_COND_PEER_RX_BUSY;
            if ns != (*x25).vr || !x25_validate_nr(sk, nr) { x25_clear_queues(sk); x25_write_internal(sk, X25_RESET_REQUEST); x25_start_t22timer(sk); (*x25).condition=0; (*x25).vs=0; (*x25).vr=0; (*x25).va=0; (*x25).vl=0; (*x25).state=X25_STATE_4; }
            else {
                x25_frames_acked(sk, nr);
                if ns == (*x25).vr {
                    if x25_queue_rx_frame(sk, skb, m)==0 { (*x25).vr=((*x25).vr+1)%modulus; queued=1; }
                    else { x25_clear_queues(sk); x25_write_internal(sk, X25_RESET_REQUEST); x25_start_t22timer(sk); (*x25).condition=0; (*x25).vs=0; (*x25).vr=0; (*x25).va=0; (*x25).vl=0; (*x25).state=X25_STATE_4; }
                    if atomic_read(&(*sk).sk_rmem_alloc) > ((*sk).sk_rcvbuf >> 1) { (*x25).condition |= X25_COND_OWN_RX_BUSY; }
                }
                if (((*x25).vl + (*x25).facilities.winsize_in) % modulus == (*x25).vr) {
                    (*x25).condition &= !X25_COND_ACK_PENDING;
                    x25_stop_timer(sk);
                    x25_enquiry_response(sk);
                } else {
                    (*x25).condition |= X25_COND_ACK_PENDING;
                    x25_start_t2timer(sk);
                }
            }
        }
        X25_INTERRUPT_CONFIRMATION => clear_bit(X25_INTERRUPT_FLAG, &mut (*x25).flags),
        X25_INTERRUPT => { if sock_flag(sk, SOCK_URGINLINE) { queued = (!sock_queue_rcv_skb(sk, skb)) as c_int; } else { skb_set_owner_r(skb, sk); skb_queue_tail(&mut (*x25).interrupt_in_queue, skb); queued=1; } sk_send_sigurg(sk); x25_write_internal(sk, X25_INTERRUPT_CONFIRMATION); }
        _ => pr_warn!("unknown {:02X} in state 3\n", frametype),
    }
    return queued;
out_clear:
    x25_write_internal(sk, X25_CLEAR_REQUEST); (*x25).state=X25_STATE_2; x25_start_t23timer(sk); 0
}

unsafe fn x25_state4_machine(sk: *mut sock, skb: *mut sk_buff, frametype: c_int) -> c_int {
    let x25=x25_sk(sk);
    match frametype {
        X25_RESET_REQUEST => { x25_write_internal(sk, X25_RESET_CONFIRMATION); x25_stop_timer(sk); (*x25).condition=0; (*x25).va=0; (*x25).vr=0; (*x25).vs=0; (*x25).vl=0; (*x25).state=X25_STATE_3; x25_requeue_frames(sk); }
        X25_RESET_CONFIRMATION => { x25_stop_timer(sk); (*x25).condition=0; (*x25).va=0; (*x25).vr=0; (*x25).vs=0; (*x25).vl=0; (*x25).state=X25_STATE_3; x25_requeue_frames(sk); }
        X25_CLEAR_REQUEST => { if !pskb_may_pull(skb, X25_STD_MIN_LEN+2) { goto out_clear; } x25_write_internal(sk, X25_CLEAR_CONFIRMATION); x25_disconnect(sk,0,(*skb).data[3] as c_int,(*skb).data[4] as c_int); }
        _ => {}
    }
    return 0;
out_clear: x25_write_internal(sk,X25_CLEAR_REQUEST); (*x25).state=X25_STATE_2; x25_start_t23timer(sk); 0
}

unsafe fn x25_state5_machine(sk: *mut sock, skb: *mut sk_buff, frametype: c_int) -> c_int {
    let x25=x25_sk(sk);
    if frametype == X25_CLEAR_REQUEST {
        if !pskb_may_pull(skb, X25_STD_MIN_LEN+2) { x25_write_internal(sk,X25_CLEAR_REQUEST); (*x25).state=X25_STATE_2; x25_start_t23timer(sk); return 0; }
        x25_write_internal(sk,X25_CLEAR_CONFIRMATION); x25_disconnect(sk,0,(*skb).data[3] as c_int,(*skb).data[4] as c_int);
    }
    0
}

pub unsafe fn x25_process_rx_frame(sk: *mut sock, skb: *mut sk_buff) -> c_int {
    let x25=x25_sk(sk); if (*x25).state == X25_STATE_0 { return 0; }
    let mut ns=0; let mut nr=0; let mut q=0; let mut d=0; let mut m=0;
    let frametype=x25_decode(sk,skb,&mut ns,&mut nr,&mut q,&mut d,&mut m);
    let queued=match (*x25).state { X25_STATE_1=>x25_state1_machine(sk,skb,frametype), X25_STATE_2=>x25_state2_machine(sk,skb,frametype), X25_STATE_3=>x25_state3_machine(sk,skb,frametype,ns,nr,q,d,m), X25_STATE_4=>x25_state4_machine(sk,skb,frametype), X25_STATE_5=>x25_state5_machine(sk,skb,frametype), _=>0 };
    x25_kick(sk); queued
}

pub unsafe fn x25_backlog_rcv(sk: *mut sock, skb: *mut sk_buff) -> c_int {
    if x25_process_rx_frame(sk,skb) == 0 { kfree_skb(skb); } 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
