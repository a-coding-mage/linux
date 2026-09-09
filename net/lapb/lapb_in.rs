// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * LAPB release 002.  Direct Rust translation of lapb_in.c.
 * The types, constants, macros, and external functions used here are supplied
 * by the surrounding LAPB implementation.
 */

unsafe fn lapb_state0_machine(lapb: *mut lapb_cb, skb: *mut sk_buff,
                              frame: *mut lapb_frame) {
    match (*frame).type_ {
    LAPB_SABM => {
        lapb_dbg(1, "(%p) S0 RX SABM(%d)\n", (*lapb).dev, (*frame).pf);
        if (*lapb).mode & LAPB_EXTENDED != 0 {
            lapb_dbg(1, "(%p) S0 TX DM(%d)\n", (*lapb).dev, (*frame).pf);
            lapb_send_control(lapb, LAPB_DM, (*frame).pf, LAPB_RESPONSE);
        } else {
            lapb_dbg(1, "(%p) S0 TX UA(%d)\n", (*lapb).dev, (*frame).pf);
            lapb_dbg(0, "(%p) S0 -> S3\n", (*lapb).dev);
            lapb_send_control(lapb, LAPB_UA, (*frame).pf, LAPB_RESPONSE);
            lapb_stop_t1timer(lapb); lapb_stop_t2timer(lapb);
            (*lapb).state = LAPB_STATE_3; (*lapb).condition = 0x00;
            (*lapb).n2count = 0; (*lapb).vs = 0; (*lapb).vr = 0; (*lapb).va = 0;
            lapb_connect_indication(lapb, LAPB_OK);
        }
    }
    LAPB_SABME => {
        lapb_dbg(1, "(%p) S0 RX SABME(%d)\n", (*lapb).dev, (*frame).pf);
        if (*lapb).mode & LAPB_EXTENDED != 0 {
            lapb_dbg(1, "(%p) S0 TX UA(%d)\n", (*lapb).dev, (*frame).pf);
            lapb_dbg(0, "(%p) S0 -> S3\n", (*lapb).dev);
            lapb_send_control(lapb, LAPB_UA, (*frame).pf, LAPB_RESPONSE);
            lapb_stop_t1timer(lapb); lapb_stop_t2timer(lapb);
            (*lapb).state = LAPB_STATE_3; (*lapb).condition = 0x00;
            (*lapb).n2count = 0; (*lapb).vs = 0; (*lapb).vr = 0; (*lapb).va = 0;
            lapb_connect_indication(lapb, LAPB_OK);
        } else { lapb_dbg(1, "(%p) S0 TX DM(%d)\n", (*lapb).dev, (*frame).pf);
            lapb_send_control(lapb, LAPB_DM, (*frame).pf, LAPB_RESPONSE); }
    }
    LAPB_DISC => { lapb_dbg(1, "(%p) S0 RX DISC(%d)\n", (*lapb).dev, (*frame).pf);
        lapb_dbg(1, "(%p) S0 TX UA(%d)\n", (*lapb).dev, (*frame).pf);
        lapb_send_control(lapb, LAPB_UA, (*frame).pf, LAPB_RESPONSE); }
    _ => {}
    }
    kfree_skb(skb);
}

unsafe fn lapb_state1_machine(lapb: *mut lapb_cb, skb: *mut sk_buff,
                              frame: *mut lapb_frame) {
    match (*frame).type_ {
    LAPB_SABM => { if (*lapb).mode & LAPB_EXTENDED != 0 { lapb_send_control(lapb,LAPB_DM,(*frame).pf,LAPB_RESPONSE); } else { lapb_send_control(lapb,LAPB_UA,(*frame).pf,LAPB_RESPONSE); } }
    LAPB_SABME => { if (*lapb).mode & LAPB_EXTENDED != 0 { lapb_send_control(lapb,LAPB_UA,(*frame).pf,LAPB_RESPONSE); } else { lapb_send_control(lapb,LAPB_DM,(*frame).pf,LAPB_RESPONSE); } }
    LAPB_DISC => { lapb_send_control(lapb,LAPB_DM,(*frame).pf,LAPB_RESPONSE); }
    LAPB_UA => if (*frame).pf { lapb_stop_t1timer(lapb); lapb_stop_t2timer(lapb); (*lapb).state=LAPB_STATE_3; (*lapb).condition=0; (*lapb).n2count=0; (*lapb).vs=0; (*lapb).vr=0; (*lapb).va=0; lapb_connect_confirmation(lapb,LAPB_OK); },
    LAPB_DM => if (*frame).pf { lapb_clear_queues(lapb); (*lapb).state=LAPB_STATE_0; lapb_start_t1timer(lapb); lapb_stop_t2timer(lapb); lapb_disconnect_indication(lapb,LAPB_REFUSED); },
    _ => {}
    }
    kfree_skb(skb);
}

unsafe fn lapb_state2_machine(lapb: *mut lapb_cb, skb: *mut sk_buff,
                              frame: *mut lapb_frame) {
    match (*frame).type_ {
    LAPB_SABM | LAPB_SABME => lapb_send_control(lapb,LAPB_DM,(*frame).pf,LAPB_RESPONSE),
    LAPB_DISC => lapb_send_control(lapb,LAPB_UA,(*frame).pf,LAPB_RESPONSE),
    LAPB_UA => if (*frame).pf { (*lapb).state=LAPB_STATE_0; lapb_start_t1timer(lapb); lapb_stop_t2timer(lapb); lapb_disconnect_confirmation(lapb,LAPB_OK); },
    LAPB_DM => if (*frame).pf { (*lapb).state=LAPB_STATE_0; lapb_start_t1timer(lapb); lapb_stop_t2timer(lapb); lapb_disconnect_confirmation(lapb,LAPB_NOTCONNECTED); },
    LAPB_I | LAPB_REJ | LAPB_RNR | LAPB_RR => if (*frame).pf { lapb_send_control(lapb,LAPB_DM,(*frame).pf,LAPB_RESPONSE); },
    _ => {}
    }
    kfree_skb(skb);
}

unsafe fn lapb_state3_machine(lapb: *mut lapb_cb, skb: *mut sk_buff,
                              frame: *mut lapb_frame) {
    let mut queued = false;
    let modulus = if (*lapb).mode & LAPB_EXTENDED != 0 { LAPB_EMODULUS } else { LAPB_SMODULUS };
    match (*frame).type_ {
    LAPB_SABM => if (*lapb).mode & LAPB_EXTENDED != 0 { lapb_send_control(lapb,LAPB_DM,(*frame).pf,LAPB_RESPONSE); } else { lapb_send_control(lapb,LAPB_UA,(*frame).pf,LAPB_RESPONSE); lapb_stop_t1timer(lapb); lapb_stop_t2timer(lapb); (*lapb).condition=0; (*lapb).n2count=0; (*lapb).vs=0; (*lapb).vr=0; (*lapb).va=0; lapb_requeue_frames(lapb); },
    LAPB_SABME => if (*lapb).mode & LAPB_EXTENDED != 0 { lapb_send_control(lapb,LAPB_UA,(*frame).pf,LAPB_RESPONSE); lapb_stop_t1timer(lapb); lapb_stop_t2timer(lapb); (*lapb).condition=0; (*lapb).n2count=0; (*lapb).vs=0; (*lapb).vr=0; (*lapb).va=0; lapb_requeue_frames(lapb); } else { lapb_send_control(lapb,LAPB_DM,(*frame).pf,LAPB_RESPONSE); },
    LAPB_DISC => { lapb_clear_queues(lapb); lapb_send_control(lapb,LAPB_UA,(*frame).pf,LAPB_RESPONSE); lapb_start_t1timer(lapb); lapb_stop_t2timer(lapb); (*lapb).state=LAPB_STATE_0; lapb_disconnect_indication(lapb,LAPB_OK); },
    LAPB_DM => { lapb_clear_queues(lapb); (*lapb).state=LAPB_STATE_0; lapb_start_t1timer(lapb); lapb_stop_t2timer(lapb); lapb_disconnect_indication(lapb,LAPB_NOTCONNECTED); },
    LAPB_RNR => { (*lapb).condition |= LAPB_PEER_RX_BUSY_CONDITION; lapb_check_need_response(lapb,(*frame).cr,(*frame).pf); if lapb_validate_nr(lapb,(*frame).nr) { lapb_check_iframes_acked(lapb,(*frame).nr); } else { (*lapb).frmr_data=*frame; (*lapb).frmr_type=LAPB_FRMR_Z; lapb_transmit_frmr(lapb); lapb_start_t1timer(lapb); lapb_stop_t2timer(lapb); (*lapb).state=LAPB_STATE_4; (*lapb).n2count=0; } },
    LAPB_RR => { (*lapb).condition &= !LAPB_PEER_RX_BUSY_CONDITION; lapb_check_need_response(lapb,(*frame).cr,(*frame).pf); if lapb_validate_nr(lapb,(*frame).nr) { lapb_check_iframes_acked(lapb,(*frame).nr); } else { (*lapb).frmr_data=*frame; (*lapb).frmr_type=LAPB_FRMR_Z; lapb_transmit_frmr(lapb); lapb_start_t1timer(lapb); lapb_stop_t2timer(lapb); (*lapb).state=LAPB_STATE_4; (*lapb).n2count=0; } },
    LAPB_REJ => { (*lapb).condition &= !LAPB_PEER_RX_BUSY_CONDITION; lapb_check_need_response(lapb,(*frame).cr,(*frame).pf); if lapb_validate_nr(lapb,(*frame).nr) { lapb_frames_acked(lapb,(*frame).nr); lapb_stop_t1timer(lapb); (*lapb).n2count=0; lapb_requeue_frames(lapb); } else { (*lapb).frmr_data=*frame; (*lapb).frmr_type=LAPB_FRMR_Z; lapb_transmit_frmr(lapb); lapb_start_t1timer(lapb); lapb_stop_t2timer(lapb); (*lapb).state=LAPB_STATE_4; (*lapb).n2count=0; } },
    LAPB_I => {
        if !lapb_validate_nr(lapb,(*frame).nr) { (*lapb).frmr_data=*frame; (*lapb).frmr_type=LAPB_FRMR_Z; lapb_transmit_frmr(lapb); lapb_start_t1timer(lapb); lapb_stop_t2timer(lapb); (*lapb).state=LAPB_STATE_4; (*lapb).n2count=0; }
        else { if (*lapb).condition & LAPB_PEER_RX_BUSY_CONDITION != 0 { lapb_frames_acked(lapb,(*frame).nr); } else { lapb_check_iframes_acked(lapb,(*frame).nr); }
            if (*frame).ns == (*lapb).vr { let cn=lapb_data_indication(lapb,skb); queued=true; if cn==NET_RX_DROP { break; } (*lapb).vr=((*lapb).vr+1)%modulus; (*lapb).condition &= !LAPB_REJECT_CONDITION; if (*frame).pf { lapb_enquiry_response(lapb); } else if (*lapb).condition & LAPB_ACK_PENDING_CONDITION == 0 { (*lapb).condition |= LAPB_ACK_PENDING_CONDITION; lapb_start_t2timer(lapb); } }
            else if (*lapb).condition & LAPB_REJECT_CONDITION != 0 { if (*frame).pf { lapb_enquiry_response(lapb); } } else { (*lapb).condition |= LAPB_REJECT_CONDITION; lapb_send_control(lapb,LAPB_REJ,(*frame).pf,LAPB_RESPONSE); (*lapb).condition &= !LAPB_ACK_PENDING_CONDITION; }
        }
    }
    LAPB_FRMR => { lapb_establish_data_link(lapb); lapb_requeue_frames(lapb); (*lapb).state=LAPB_STATE_1; },
    LAPB_ILLEGAL => { (*lapb).frmr_data=*frame; (*lapb).frmr_type=LAPB_FRMR_W; lapb_transmit_frmr(lapb); lapb_start_t1timer(lapb); lapb_stop_t2timer(lapb); (*lapb).state=LAPB_STATE_4; (*lapb).n2count=0; },
    _ => {}
    }
    if !queued { kfree_skb(skb); }
}

unsafe fn lapb_state4_machine(lapb: *mut lapb_cb, skb: *mut sk_buff,
                              frame: *mut lapb_frame) {
    match (*frame).type_ {
    LAPB_SABM => if (*lapb).mode & LAPB_EXTENDED != 0 { lapb_send_control(lapb,LAPB_DM,(*frame).pf,LAPB_RESPONSE); } else { lapb_send_control(lapb,LAPB_UA,(*frame).pf,LAPB_RESPONSE); lapb_stop_t1timer(lapb); lapb_stop_t2timer(lapb); (*lapb).state=LAPB_STATE_3; (*lapb).condition=0; (*lapb).n2count=0; (*lapb).vs=0; (*lapb).vr=0; (*lapb).va=0; lapb_connect_indication(lapb,LAPB_OK); },
    LAPB_SABME => if (*lapb).mode & LAPB_EXTENDED != 0 { lapb_send_control(lapb,LAPB_UA,(*frame).pf,LAPB_RESPONSE); lapb_stop_t1timer(lapb); lapb_stop_t2timer(lapb); (*lapb).state=LAPB_STATE_3; (*lapb).condition=0; (*lapb).n2count=0; (*lapb).vs=0; (*lapb).vr=0; (*lapb).va=0; lapb_connect_indication(lapb,LAPB_OK); } else { lapb_send_control(lapb,LAPB_DM,(*frame).pf,LAPB_RESPONSE); },
    _ => {}
    }
    kfree_skb(skb);
}

pub unsafe fn lapb_data_input(lapb: *mut lapb_cb, skb: *mut sk_buff) {
    let mut frame = lapb_frame::default();
    if lapb_decode(lapb, skb, &mut frame) < 0 { kfree_skb(skb); return; }
    match (*lapb).state {
        LAPB_STATE_0 => lapb_state0_machine(lapb,skb,&mut frame),
        LAPB_STATE_1 => lapb_state1_machine(lapb,skb,&mut frame),
        LAPB_STATE_2 => lapb_state2_machine(lapb,skb,&mut frame),
        LAPB_STATE_3 => lapb_state3_machine(lapb,skb,&mut frame),
        LAPB_STATE_4 => lapb_state4_machine(lapb,skb,&mut frame),
        _ => {}
    }
    lapb_kick(lapb);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
