// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * X.25 Packet Layer release 002
 *
 * Direct Rust translation of x25_out.c. Kernel and X.25 declarations are
 * supplied by the surrounding translation unit.
 */

use core::ffi::c_int;

// External kernel/X.25 types and operations supplied by other files.
#[repr(C)]
pub struct sock {
    pub sk_write_queue: sk_buff_head,
}
#[repr(C)]
pub struct sk_buff {
    pub len: usize,
    pub data: *mut u8,
}
#[repr(C)]
pub struct sk_buff_head;
#[repr(C)]
pub struct x25_neighbour {
    pub extended: bool,
}
#[repr(C)]
pub struct x25_facilities {
    pub pacsize_out: u32,
    pub winsize_out: u16,
}
#[repr(C)]
pub struct x25_sock {
    pub neighbour: *mut x25_neighbour,
    pub facilities: x25_facilities,
    pub state: c_int,
    pub flags: c_int,
    pub condition: c_int,
    pub vs: u16,
    pub vr: u16,
    pub va: u16,
    pub vl: u16,
    pub interrupt_out_queue: sk_buff_head,
    pub ack_queue: sk_buff_head,
}

extern "C" {
    fn x25_sk(sk: *mut sock) -> *mut x25_sock;
    fn skb_copy_from_linear_data(skb: *mut sk_buff, to: *mut u8, len: usize);
    fn skb_copy_to_linear_data(skb: *mut sk_buff, from: *const u8, len: usize);
    fn skb_pull(skb: *mut sk_buff, len: usize);
    fn skb_push(skb: *mut sk_buff, len: usize) -> *mut u8;
    fn skb_put(skb: *mut sk_buff, len: usize) -> *mut u8;
    fn skb_headroom(skb: *mut sk_buff) -> usize;
    fn sock_alloc_send_skb(sk: *mut sock, size: usize, noblock: c_int, err: *mut c_int) -> *mut sk_buff;
    fn skb_reserve(skb: *mut sk_buff, len: usize);
    fn skb_queue_tail(queue: *mut sk_buff_head, skb: *mut sk_buff);
    fn skb_queue_head(queue: *mut sk_buff_head, skb: *mut sk_buff);
    fn skb_dequeue(queue: *mut sk_buff_head) -> *mut sk_buff;
    fn skb_peek(queue: *mut sk_buff_head) -> *mut sk_buff;
    fn skb_clone(skb: *mut sk_buff, gfp: c_int) -> *mut sk_buff;
    fn skb_set_owner_w(skb: *mut sk_buff, sk: *mut sock);
    fn kfree_skb(skb: *mut sk_buff);
    fn x25_transmit_link(skb: *mut sk_buff, nb: *mut x25_neighbour);
    fn x25_write_internal(sk: *mut sock, frame: c_int);
    fn x25_stop_timer(sk: *mut sock);
    fn test_and_set_bit(bit: c_int, addr: *mut c_int) -> c_int;
    fn release_sock(sk: *mut sock);
    fn lock_sock(sk: *mut sock);
}

const X25_EXT_MIN_LEN: usize = 4;
const X25_STD_MIN_LEN: usize = 3;
const MSG_DONTWAIT: c_int = 0x40;
const X25_EXT_M_BIT: u8 = 0x10;
const X25_STD_M_BIT: u8 = 0x01;
const X25_STATE_3: c_int = 3;
const X25_COND_PEER_RX_BUSY: c_int = 1 << 0;
const X25_COND_OWN_RX_BUSY: c_int = 1 << 1;
const X25_COND_ACK_PENDING: c_int = 1 << 2;
const X25_INTERRUPT_FLAG: c_int = 0;
const X25_EMODULUS: i32 =  modulus_placeholder();
const X25_SMODULUS: i32 =  modulus_placeholder();
const X25_RNR: c_int = 0;
const X25_RR: c_int = 0;
const GFP_ATOMIC: c_int = 0;
const EWOULDBLOCK: c_int = 11;

const fn modulus_placeholder() -> i32 { 8 }

unsafe fn x25_pacsize_to_bytes(mut pacsize: u32) -> c_int {
    let mut bytes: c_int = 1;
    if pacsize == 0 { return 128; }
    while pacsize > 0 {
        pacsize -= 1;
        bytes *= 2;
    }
    bytes
}

pub unsafe fn x25_output(sk: *mut sock, skb: *mut sk_buff) -> c_int {
    let mut header = [0u8; X25_EXT_MIN_LEN];
    let mut err: c_int = 0;
    let mut sent: c_int = 0;
    let x25 = x25_sk(sk);
    let header_len = if (*(*x25).neighbour).extended { X25_EXT_MIN_LEN } else { X25_STD_MIN_LEN };
    let max_len = x25_pacsize_to_bytes((*x25).facilities.pacsize_out) as usize;
    let noblock = ((*skb).data as *const c_int).read() & MSG_DONTWAIT;

    if (*skb).len - header_len > max_len {
        skb_copy_from_linear_data(skb, header.as_mut_ptr(), header_len);
        skb_pull(skb, header_len);
        let frontlen = skb_headroom(skb);
        while (*skb).len > 0 {
            release_sock(sk);
            let skbn = sock_alloc_send_skb(sk, frontlen + max_len, noblock, &mut err);
            lock_sock(sk);
            if skbn.is_null() {
                if err == -EWOULDBLOCK && noblock != 0 { kfree_skb(skb); return sent; }
                return err;
            }
            skb_reserve(skbn, frontlen);
            let len = if max_len > (*skb).len { (*skb).len } else { max_len };
            skb_copy_from_linear_data(skb, skb_put(skbn, len), len);
            skb_pull(skb, len);
            skb_push(skbn, header_len);
            skb_copy_to_linear_data(skbn, header.as_ptr(), header_len);
            if (*skb).len > 0 {
                if (*(*x25).neighbour).extended { (*skbn).data.add(3).write((*skbn).data.add(3).read() | X25_EXT_M_BIT); }
                else { (*skbn).data.add(2).write((*skbn).data.add(2).read() | X25_STD_M_BIT); }
            }
            skb_queue_tail(&mut (*sk).sk_write_queue, skbn);
            sent += len as c_int;
        }
        kfree_skb(skb);
    } else {
        skb_queue_tail(&mut (*sk).sk_write_queue, skb);
        sent = ((*skb).len - header_len) as c_int;
    }
    sent
}

unsafe fn x25_send_iframe(sk: *mut sock, skb: *mut sk_buff) {
    if skb.is_null() { return; }
    let x25 = x25_sk(sk);
    if (*(*x25).neighbour).extended {
        (*skb).data.add(2).write(((*x25).vs << 1) as u8 & 0xfe);
        (*skb).data.add(3).write((*skb).data.add(3).read() & X25_EXT_M_BIT);
        (*skb).data.add(3).write((*skb).data.add(3).read() | (((*x25).vr << 1) as u8 & 0xfe));
    } else {
        (*skb).data.add(2).write((*skb).data.add(2).read() & X25_STD_M_BIT);
        (*skb).data.add(2).write((*skb).data.add(2).read() | (((*x25).vs << 1) as u8 & 0x0e));
        (*skb).data.add(2).write((*skb).data.add(2).read() | (((*x25).vr << 5) as u8 & 0xe0));
    }
    x25_transmit_link(skb, (*x25).neighbour);
}

pub unsafe fn x25_kick(sk: *mut sock) {
    let x25 = x25_sk(sk);
    if (*x25).state != X25_STATE_3 { return; }
    if !skb_peek(&mut (*x25).interrupt_out_queue).is_null() && test_and_set_bit(X25_INTERRUPT_FLAG, &mut (*x25).flags) == 0 {
        x25_transmit_link(skb_dequeue(&mut (*x25).interrupt_out_queue), (*x25).neighbour);
    }
    if (*x25).condition & X25_COND_PEER_RX_BUSY != 0 || skb_peek(&mut (*sk).sk_write_queue).is_null() { return; }
    let modulus = if (*(*x25).neighbour).extended { X25_EMODULUS } else { X25_SMODULUS } as u16;
    let start = if !skb_peek(&mut (*x25).ack_queue).is_null() { (*x25).vs } else { (*x25).va };
    let end = (((*x25).va + (*x25).facilities.winsize_out) % modulus) as u16;
    if start == end { return; }
    (*x25).vs = start;
    let mut skb = skb_dequeue(&mut (*sk).sk_write_queue);
    loop {
        let skbn = skb_clone(skb, GFP_ATOMIC);
        if skbn.is_null() { skb_queue_head(&mut (*sk).sk_write_queue, skb); break; }
        skb_set_owner_w(skbn, sk);
        x25_send_iframe(sk, skbn);
        (*x25).vs = ((*x25).vs + 1) % modulus;
        skb_queue_tail(&mut (*x25).ack_queue, skb);
        if (*x25).vs == end { break; }
        skb = skb_dequeue(&mut (*sk).sk_write_queue);
        if skb.is_null() { break; }
    }
    (*x25).vl = (*x25).vr;
    (*x25).condition &= !X25_COND_ACK_PENDING;
    x25_stop_timer(sk);
}

pub unsafe fn x25_enquiry_response(sk: *mut sock) {
    let x25 = x25_sk(sk);
    x25_write_internal(sk, if (*x25).condition & X25_COND_OWN_RX_BUSY != 0 { X25_RNR } else { X25_RR });
    (*x25).vl = (*x25).vr;
    (*x25).condition &= !X25_COND_ACK_PENDING;
    x25_stop_timer(sk);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
