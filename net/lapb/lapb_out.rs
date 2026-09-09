// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * LAPB release 002
 *
 * This code REQUIRES 2.1.15 or higher/ NET3.038
 */

use core::ffi::c_void;

#[repr(C)]
pub struct sk_buff {
    pub sk: *mut sock,
    pub data: *mut u8,
}

#[repr(C)]
pub struct sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct lapb_cb {
    pub mode: u32,
    pub state: i32,
    pub condition: u32,
    pub n2count: u16,
    pub va: u16,
    pub vs: u16,
    pub vr: u16,
    pub window: u16,
    pub dev: *mut c_void,
    pub ack_queue: *mut c_void,
    pub write_queue: *mut c_void,
}

extern "C" {
    fn skb_push(skb: *mut sk_buff, len: u32) -> *mut u8;
    fn skb_peek(queue: *mut c_void) -> *mut sk_buff;
    fn skb_dequeue(queue: *mut c_void) -> *mut sk_buff;
    fn skb_copy(skb: *mut sk_buff, gfp_mask: u32) -> *mut sk_buff;
    fn skb_set_owner_w(skb: *mut sk_buff, sk: *mut sock);
    fn skb_queue_head(queue: *mut c_void, skb: *mut sk_buff);
    fn skb_queue_tail(queue: *mut c_void, skb: *mut sk_buff);
    fn kfree_skb(skb: *mut sk_buff);
    fn lapb_dbg(level: i32, fmt: *const u8, ...);
    fn lapb_data_transmit(lapb: *mut lapb_cb, skb: *mut sk_buff) -> i32;
    fn lapb_send_control(lapb: *mut lapb_cb, frame: i32, poll_bit: i32, ty: i32);
    fn lapb_start_t1timer(lapb: *mut lapb_cb);
    fn lapb_stop_t1timer(lapb: *mut lapb_cb);
    fn lapb_start_t2timer(lapb: *mut lapb_cb);
    fn lapb_stop_t2timer(lapb: *mut lapb_cb);
    fn lapb_t1timer_running(lapb: *mut lapb_cb) -> i32;
    fn lapb_frames_acked(lapb: *mut lapb_cb, nr: u16);
}

const LAPB_EXTENDED: u32 = 0x01;
const LAPB_MLP: u32 = 0x02;
const LAPB_DCE: u32 = 0x04;
const LAPB_PEER_RX_BUSY_CONDITION: u32 = 0x01;
const LAPB_ACK_PENDING_CONDITION: u32 = 0x02;
const LAPB_EMODULUS: u16 = 128;
const LAPB_SMODULUS: u16 = 8;
const LAPB_I: u8 = 0x00;
const LAPB_EPF: u8 = 0x01;
const LAPB_SPF: u8 = 0x10;
const LAPB_COMMAND: i32 = 0;
const LAPB_RESPONSE: i32 = 1;
const LAPB_POLLOFF: i32 = 0;
const LAPB_POLLON: i32 = 1;
const LAPB_SABME: i32 = 0x6f;
const LAPB_SABM: i32 = 0x2f;
const LAPB_RR: i32 = 0x01;
const LAPB_ADDR_A: u8 = 0x03;
const LAPB_ADDR_B: u8 = 0x01;
const LAPB_ADDR_C: u8 = 0x0f;
const LAPB_ADDR_D: u8 = 0x07;
const GFP_ATOMIC: u32 = 0x20;

/* This procedure is passed a buffer descriptor for an iframe. */
unsafe fn lapb_send_iframe(lapb: *mut lapb_cb, skb: *mut sk_buff, poll_bit: i32) {
    if skb.is_null() { return; }
    let frame: *mut u8;
    if (*lapb).mode & LAPB_EXTENDED != 0 {
        frame = skb_push(skb, 2);
        *frame = LAPB_I | ((*lapb).vs as u8) << 1;
        *frame.add(1) = if poll_bit != 0 { LAPB_EPF } else { 0 } | ((*lapb).vr as u8) << 1;
    } else {
        frame = skb_push(skb, 1);
        *frame = LAPB_I | if poll_bit != 0 { LAPB_SPF } else { 0 } |
            ((*lapb).vr as u8) << 5 | ((*lapb).vs as u8) << 1;
    }
    lapb_dbg(1, b"(%p) S%d TX I(%d) S%d R%d\0".as_ptr(), (*lapb).dev, (*lapb).state,
             poll_bit, (*lapb).vs, (*lapb).vr);
    lapb_transmit_buffer(lapb, skb, LAPB_COMMAND);
}

pub unsafe fn lapb_kick(lapb: *mut lapb_cb) {
    let modulus = if (*lapb).mode & LAPB_EXTENDED != 0 { LAPB_EMODULUS } else { LAPB_SMODULUS };
    let mut start = if skb_peek((*lapb).ack_queue).is_null() { (*lapb).va } else { (*lapb).vs };
    let end = ((*lapb).va + (*lapb).window) % modulus;
    if (*lapb).condition & LAPB_PEER_RX_BUSY_CONDITION == 0 && start != end && !skb_peek((*lapb).write_queue).is_null() {
        (*lapb).vs = start;
        let mut skb = skb_dequeue((*lapb).write_queue);
        loop {
            let skbn = skb_copy(skb, GFP_ATOMIC);
            if skbn.is_null() { skb_queue_head((*lapb).write_queue, skb); break; }
            if !(*skb).sk.is_null() { skb_set_owner_w(skbn, (*skb).sk); }
            lapb_send_iframe(lapb, skbn, LAPB_POLLOFF);
            (*lapb).vs = ((*lapb).vs + 1) % modulus;
            skb_queue_tail((*lapb).ack_queue, skb);
            if (*lapb).vs == end { break; }
            skb = skb_dequeue((*lapb).write_queue);
            if skb.is_null() { break; }
        }
        (*lapb).condition &= !LAPB_ACK_PENDING_CONDITION;
        if lapb_t1timer_running(lapb) == 0 { lapb_start_t1timer(lapb); }
    }
}

pub unsafe fn lapb_transmit_buffer(lapb: *mut lapb_cb, skb: *mut sk_buff, ty: i32) {
    let ptr = skb_push(skb, 1);
    *ptr = if (*lapb).mode & LAPB_MLP != 0 {
        if (*lapb).mode & LAPB_DCE != 0 { if ty == LAPB_COMMAND { LAPB_ADDR_C } else { LAPB_ADDR_D } }
        else { if ty == LAPB_COMMAND { LAPB_ADDR_D } else { LAPB_ADDR_C } }
    } else if (*lapb).mode & LAPB_DCE != 0 {
        if ty == LAPB_COMMAND { LAPB_ADDR_A } else { LAPB_ADDR_B }
    } else { if ty == LAPB_COMMAND { LAPB_ADDR_B } else { LAPB_ADDR_A } };
    lapb_dbg(2, b"(%p) S%d TX %3ph\0".as_ptr(), (*lapb).dev, (*lapb).state, (*skb).data);
    if lapb_data_transmit(lapb, skb) == 0 { kfree_skb(skb); }
}

pub unsafe fn lapb_establish_data_link(lapb: *mut lapb_cb) {
    (*lapb).condition = 0;
    (*lapb).n2count = 0;
    if (*lapb).mode & LAPB_EXTENDED != 0 { lapb_dbg(1, b"(%p) S%d TX SABME(1)\0".as_ptr(), (*lapb).dev, (*lapb).state); lapb_send_control(lapb, LAPB_SABME, LAPB_POLLON, LAPB_COMMAND); }
    else { lapb_dbg(1, b"(%p) S%d TX SABM(1)\0".as_ptr(), (*lapb).dev, (*lapb).state); lapb_send_control(lapb, LAPB_SABM, LAPB_POLLON, LAPB_COMMAND); }
    lapb_start_t1timer(lapb); lapb_stop_t2timer(lapb);
}

pub unsafe fn lapb_enquiry_response(lapb: *mut lapb_cb) { lapb_dbg(1, b"(%p) S%d TX RR(1) R%d\n\0".as_ptr(), (*lapb).dev, (*lapb).state, (*lapb).vr); lapb_send_control(lapb, LAPB_RR, LAPB_POLLON, LAPB_RESPONSE); (*lapb).condition &= !LAPB_ACK_PENDING_CONDITION; }
pub unsafe fn lapb_timeout_response(lapb: *mut lapb_cb) { lapb_dbg(1, b"(%p) S%d TX RR(0) R%d\n\0".as_ptr(), (*lapb).dev, (*lapb).state, (*lapb).vr); lapb_send_control(lapb, LAPB_RR, LAPB_POLLOFF, LAPB_RESPONSE); (*lapb).condition &= !LAPB_ACK_PENDING_CONDITION; }
pub unsafe fn lapb_check_iframes_acked(lapb: *mut lapb_cb, nr: u16) { if (*lapb).vs == nr { lapb_frames_acked(lapb, nr); lapb_stop_t1timer(lapb); (*lapb).n2count = 0; } else if (*lapb).va != nr { lapb_frames_acked(lapb, nr); lapb_start_t1timer(lapb); } }
pub unsafe fn lapb_check_need_response(lapb: *mut lapb_cb, ty: i32, pf: i32) { if ty == LAPB_COMMAND && pf != 0 { lapb_enquiry_response(lapb); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
