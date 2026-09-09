// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * LAPB release 002
 *
 * This code REQUIRES 2.1.15 or higher/ NET3.038
 *
 * History
 * LAPB 001 Jonathan Naylor Started Coding
 */

// C kernel headers and build-time definitions are supplied by the surrounding
// translation unit.

extern "C" {
    fn skb_queue_purge(queue: *mut sk_buff_head);
    fn skb_peek(queue: *const sk_buff_head) -> *mut sk_buff;
    fn skb_dequeue(queue: *mut sk_buff_head) -> *mut sk_buff;
    fn kfree_skb(skb: *mut sk_buff);
    fn skb_append(prev: *mut sk_buff, skb: *mut sk_buff, queue: *mut sk_buff_head);
    fn pskb_may_pull(skb: *mut sk_buff, len: u32) -> bool;
    fn skb_pull(skb: *mut sk_buff, len: u32) -> *mut u8;
    fn skb_reserve(skb: *mut sk_buff, len: u32);
    fn skb_put(skb: *mut sk_buff, len: u32) -> *mut u8;
    fn alloc_skb(size: u32, priority: u32) -> *mut sk_buff;
    fn lapb_dbg(level: i32, fmt: *const u8, ...);
    fn lapb_transmit_buffer(lapb: *mut lapb_cb, skb: *mut sk_buff, typ: i32);
}

/* This routine purges all the queues of frames. */
pub unsafe fn lapb_clear_queues(lapb: *mut lapb_cb) {
    skb_queue_purge(&mut (*lapb).write_queue);
    skb_queue_purge(&mut (*lapb).ack_queue);
}

/*
 * This routine purges the input queue of those frames that have been
 * acknowledged. This replaces the boxes labelled "V(a) <- N(r)" on the
 * SDL diagram.
 */
pub unsafe fn lapb_frames_acked(lapb: *mut lapb_cb, nr: u16) {
    let modulus: i32 = if (*lapb).mode & LAPB_EXTENDED != 0 { LAPB_EMODULUS } else { LAPB_SMODULUS };

    /* Remove all the ack-ed frames from the ack queue. */
    if (*lapb).va != nr {
        while !skb_peek(&(*lapb).ack_queue).is_null() && (*lapb).va != nr {
            let skb = skb_dequeue(&mut (*lapb).ack_queue);
            kfree_skb(skb);
            (*lapb).va = ((*lapb).va as i32 + 1) as u16 % modulus as u16;
        }
    }
}

pub unsafe fn lapb_requeue_frames(lapb: *mut lapb_cb) {
    let mut skb_prev: *mut sk_buff = core::ptr::null_mut();

    /* Requeue all the un-ack-ed frames on the output queue. */
    loop {
        let skb = skb_dequeue(&mut (*lapb).ack_queue);
        if skb.is_null() { break; }
        if skb_prev.is_null() {
            skb_queue_head(&mut (*lapb).write_queue, skb);
        } else {
            skb_append(skb_prev, skb, &mut (*lapb).write_queue);
        }
        skb_prev = skb;
    }
}

pub unsafe fn lapb_validate_nr(lapb: *mut lapb_cb, nr: u16) -> i32 {
    let modulus: i32 = if (*lapb).mode & LAPB_EXTENDED != 0 { LAPB_EMODULUS } else { LAPB_SMODULUS };
    let mut vc = (*lapb).va;
    while vc != (*lapb).vs {
        if nr == vc { return 1; }
        vc = (vc as i32 + 1) as u16 % modulus as u16;
    }
    if nr == (*lapb).vs { 1 } else { 0 }
}

pub unsafe fn lapb_decode(lapb: *mut lapb_cb, skb: *mut sk_buff, frame: *mut lapb_frame) -> i32 {
    (*frame).type_ = LAPB_ILLEGAL;
    lapb_dbg(2, b"(%p) S%d RX %3ph\0".as_ptr(), (*lapb).dev, (*lapb).state, (*skb).data);
    if !pskb_may_pull(skb, 2) { return -1; }

    let address = (*skb).data;
    if (*lapb).mode & LAPB_MLP != 0 {
        if (*lapb).mode & LAPB_DCE != 0 {
            if *address == LAPB_ADDR_D { (*frame).cr = LAPB_COMMAND; }
            if *address == LAPB_ADDR_C { (*frame).cr = LAPB_RESPONSE; }
        } else {
            if *address == LAPB_ADDR_C { (*frame).cr = LAPB_COMMAND; }
            if *address == LAPB_ADDR_D { (*frame).cr = LAPB_RESPONSE; }
        }
    } else if (*lapb).mode & LAPB_DCE != 0 {
        if *address == LAPB_ADDR_B { (*frame).cr = LAPB_COMMAND; }
        if *address == LAPB_ADDR_A { (*frame).cr = LAPB_RESPONSE; }
    } else {
        if *address == LAPB_ADDR_A { (*frame).cr = LAPB_COMMAND; }
        if *address == LAPB_ADDR_B { (*frame).cr = LAPB_RESPONSE; }
    }
    skb_pull(skb, 1);

    let data = (*skb).data;
    if (*lapb).mode & LAPB_EXTENDED != 0 {
        if *data & LAPB_S == 0 {
            if !pskb_may_pull(skb, 2) { return -1; }
            (*frame).type_ = LAPB_I; (*frame).ns = (*data >> 1) & 0x7f;
            (*frame).nr = (*(data.add(1)) >> 1) & 0x7f; (*frame).pf = *(data.add(1)) & LAPB_EPF;
            (*frame).control[0] = *data; (*frame).control[1] = *data.add(1); skb_pull(skb, 2);
        } else if *data & LAPB_U == 1 {
            if !pskb_may_pull(skb, 2) { return -1; }
            (*frame).type_ = *data & 0x0f; (*frame).nr = (*(data.add(1)) >> 1) & 0x7f;
            (*frame).pf = *(data.add(1)) & LAPB_EPF; (*frame).control[0] = *data;
            (*frame).control[1] = *data.add(1); skb_pull(skb, 2);
        } else if *data & LAPB_U == 3 {
            (*frame).type_ = *data & !LAPB_SPF; (*frame).pf = *data & LAPB_SPF;
            (*frame).control[0] = *data; (*frame).control[1] = 0; skb_pull(skb, 1);
        }
    } else {
        if *data & LAPB_S == 0 {
            (*frame).type_ = LAPB_I; (*frame).ns = (*data >> 1) & 7;
            (*frame).nr = (*data >> 5) & 7; (*frame).pf = *data & LAPB_SPF;
        } else if *data & LAPB_U == 1 {
            (*frame).type_ = *data & 0x0f; (*frame).nr = (*data >> 5) & 7; (*frame).pf = *data & LAPB_SPF;
        } else if *data & LAPB_U == 3 {
            (*frame).type_ = *data & !LAPB_SPF; (*frame).pf = *data & LAPB_SPF;
        }
        (*frame).control[0] = *data; skb_pull(skb, 1);
    }
    0
}

pub unsafe fn lapb_send_control(lapb: *mut lapb_cb, frametype: i32, poll_bit: i32, typ: i32) {
    let skb = alloc_skb((LAPB_HEADER_LEN + 3) as u32, GFP_ATOMIC);
    if skb.is_null() { return; }
    skb_reserve(skb, (LAPB_HEADER_LEN + 1) as u32);
    let dptr = if (*lapb).mode & LAPB_EXTENDED != 0 && frametype & LAPB_U == LAPB_U {
        let p = skb_put(skb, 1); *p = (frametype as u8) | if poll_bit != 0 { LAPB_SPF } else { 0 }; p
    } else if (*lapb).mode & LAPB_EXTENDED != 0 {
        let p = skb_put(skb, 2); *p = frametype as u8; *p.add(1) = ((*lapb).vr << 1) as u8 | if poll_bit != 0 { LAPB_EPF } else { 0 }; p
    } else {
        let p = skb_put(skb, 1); *p = frametype as u8 | if poll_bit != 0 { LAPB_SPF } else { 0 };
        if frametype & LAPB_U == LAPB_S { *p |= ((*lapb).vr << 5) as u8; } p
    };
    let _ = dptr;
    lapb_transmit_buffer(lapb, skb, typ);
}

pub unsafe fn lapb_transmit_frmr(lapb: *mut lapb_cb) {
    let skb = alloc_skb((LAPB_HEADER_LEN + 7) as u32, GFP_ATOMIC);
    if skb.is_null() { return; }
    skb_reserve(skb, (LAPB_HEADER_LEN + 1) as u32);
    let p = skb_put(skb, if (*lapb).mode & LAPB_EXTENDED != 0 { 6 } else { 4 });
    *p = LAPB_FRMR;
    *p.add(1) = (*lapb).frmr_data.control[0];
    if (*lapb).mode & LAPB_EXTENDED != 0 {
        *p.add(2) = (*lapb).frmr_data.control[1]; *p.add(3) = ((*lapb).vs << 1) as u8 & 0xfe;
        *p.add(4) = ((*lapb).vr << 1) as u8 & 0xfe;
        if (*lapb).frmr_data.cr == LAPB_RESPONSE { *p.add(4) |= 1; }
        *p.add(5) = (*lapb).frmr_type;
    } else {
        *p.add(2) = ((*lapb).vs << 1) as u8 & 0x0e | ((*lapb).vr << 5) as u8 & 0xe0;
        if (*lapb).frmr_data.cr == LAPB_RESPONSE { *p.add(2) |= 0x10; }
        *p.add(3) = (*lapb).frmr_type;
    }
    lapb_transmit_buffer(lapb, skb, LAPB_RESPONSE);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
