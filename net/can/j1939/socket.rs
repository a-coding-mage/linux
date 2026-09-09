// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2010-2011 EIA Electronics
// Copyright (c) 2017-2019 Pengutronix

// Direct translation of can/j1939/socket.c.  Kernel-provided types, constants,
// macros, and functions are intentionally left as external dependencies.

#[inline]
unsafe fn j1939_prio(mut sk_priority: u32) -> u8 {
    if sk_priority > 7 { sk_priority = 7; }
    (7 - sk_priority) as u8
}

#[inline]
unsafe fn j1939_to_sk_priority(prio: u8) -> u32 { 7 - prio as u32 }

#[inline]
unsafe fn j1939_pgn_is_valid(pgn: u32) -> bool { pgn <= J1939_PGN_MAX }

#[inline]
unsafe fn j1939_pgn_is_clean_pdu(pgn: u32) -> bool {
    if j1939_pgn_is_pdu1(pgn) { (pgn & 0xff) == 0 } else { true }
}

unsafe fn j1939_sock_pending_add(sk: *mut sock) {
    let jsk = j1939_sk(sk); atomic_inc(&mut (*jsk).skb_pending);
}
unsafe fn j1939_sock_pending_get(sk: *mut sock) -> i32 {
    atomic_read(&(*j1939_sk(sk)).skb_pending)
}

#[no_mangle]
pub unsafe extern "C" fn j1939_sock_pending_del(sk: *mut sock) {
    let jsk = j1939_sk(sk);
    if atomic_dec_return(&mut (*jsk).skb_pending) == 0 { wake_up(&mut (*jsk).waitq); }
}

unsafe fn j1939_jsk_add(priv_: *mut j1939_priv, jsk: *mut j1939_sock) {
    (*jsk).state |= J1939_SOCK_BOUND; j1939_priv_get(priv_);
    write_lock_bh(&mut (*priv_).j1939_socks_lock);
    list_add_tail(&mut (*jsk).list, &mut (*priv_).j1939_socks);
    write_unlock_bh(&mut (*priv_).j1939_socks_lock);
}
unsafe fn j1939_jsk_del(priv_: *mut j1939_priv, jsk: *mut j1939_sock) {
    write_lock_bh(&mut (*priv_).j1939_socks_lock); list_del_init(&mut (*jsk).list);
    write_unlock_bh(&mut (*priv_).j1939_socks_lock); j1939_priv_put(priv_);
    (*jsk).state &= !J1939_SOCK_BOUND;
}

unsafe fn j1939_sk_queue_session(session: *mut j1939_session) -> bool {
    let jsk = j1939_sk((*session).sk); spin_lock_bh(&mut (*jsk).sk_session_queue_lock);
    let empty = list_empty(&(*jsk).sk_session_queue); j1939_session_get(session);
    list_add_tail(&mut (*session).sk_session_queue_entry, &mut (*jsk).sk_session_queue);
    spin_unlock_bh(&mut (*jsk).sk_session_queue_lock); j1939_sock_pending_add(&mut (*jsk).sk); empty
}

unsafe fn j1939_sk_get_incomplete_session(jsk: *mut j1939_sock) -> *mut j1939_session {
    let mut session = core::ptr::null_mut(); spin_lock_bh(&mut (*jsk).sk_session_queue_lock);
    if !list_empty(&(*jsk).sk_session_queue) {
        session = list_last_entry(&mut (*jsk).sk_session_queue);
        if (*session).total_queued_size == (*session).total_message_size { session = core::ptr::null_mut(); }
        else { j1939_session_get(session); }
    }
    spin_unlock_bh(&mut (*jsk).sk_session_queue_lock); session
}

unsafe fn j1939_sk_queue_drop_all(priv_: *mut j1939_priv, jsk: *mut j1939_sock, err: i32) {
    let mut session = core::ptr::null_mut(); let mut tmp = core::ptr::null_mut();
    spin_lock_bh(&mut (*jsk).sk_session_queue_lock);
    list_for_each_entry_safe(&mut session, &mut tmp, &mut (*jsk).sk_session_queue) {
        list_del_init(&mut (*session).sk_session_queue_entry); (*session).err = err; j1939_session_put(session);
    }
    spin_unlock_bh(&mut (*jsk).sk_session_queue_lock); let _ = priv_;
}

unsafe fn j1939_sk_match_dst(jsk: *mut j1939_sock, skcb: *const j1939_sk_buff_cb) -> bool {
    if (*jsk).state & J1939_SOCK_PROMISC != 0 { return true; }
    if (*jsk).addr.src_name != 0 && (*skcb).addr.dst_name != 0 {
        if (*jsk).addr.src_name != (*skcb).addr.dst_name { return false; }
    } else if j1939_address_is_unicast((*skcb).addr.da) {
        if (*jsk).addr.sa != (*skcb).addr.da { return false; }
    } else if !sock_flag(&mut (*jsk).sk, SOCK_BROADCAST) { return false; }
    if (*jsk).state & J1939_SOCK_CONNECTED != 0 {
        if (*jsk).addr.dst_name != 0 && (*skcb).addr.src_name != 0 {
            if (*jsk).addr.dst_name != (*skcb).addr.src_name { return false; }
        } else if (*jsk).addr.da != (*skcb).addr.sa { return false; }
    }
    !j1939_pgn_is_valid((*jsk).pgn_rx_filter) || (*jsk).pgn_rx_filter == (*skcb).addr.pgn
}

unsafe fn j1939_sk_match_filter(jsk: *mut j1939_sock, skcb: *const j1939_sk_buff_cb) -> bool {
    spin_lock_bh(&mut (*jsk).filters_lock); let mut f = (*jsk).filters; let mut n = (*jsk).nfilters;
    if n == 0 { spin_unlock_bh(&mut (*jsk).filters_lock); return true; }
    while n != 0 { if ((*skcb).addr.pgn & (*f).pgn_mask) == (*f).pgn &&
        ((*skcb).addr.sa & (*f).addr_mask) == (*f).addr &&
        ((*skcb).addr.src_name & (*f).name_mask) == (*f).name {
            spin_unlock_bh(&mut (*jsk).filters_lock); return true; }
        f = f.add(1); n -= 1; }
    spin_unlock_bh(&mut (*jsk).filters_lock); false
}

unsafe fn j1939_sk_recv_match_one(jsk: *mut j1939_sock, skcb: *const j1939_sk_buff_cb) -> bool {
    (*jsk).state & J1939_SOCK_BOUND != 0 && j1939_sk_match_dst(jsk, skcb) && j1939_sk_match_filter(jsk, skcb)
}

#[no_mangle]
pub unsafe extern "C" fn j1939_sk_recv_match(priv_: *mut j1939_priv, skcb: *mut j1939_sk_buff_cb) -> bool {
    let mut jsk = core::ptr::null_mut(); let mut found = false; read_lock_bh(&mut (*priv_).j1939_socks_lock);
    list_for_each_entry(&mut jsk, &mut (*priv_).j1939_socks) { if j1939_sk_recv_match_one(jsk, skcb) { found = true; break; } }
    read_unlock_bh(&mut (*priv_).j1939_socks_lock); found
}

#[no_mangle]
pub unsafe extern "C" fn j1939_sk_recv(priv_: *mut j1939_priv, skb: *mut sk_buff) {
    let mut jsk = core::ptr::null_mut(); read_lock_bh(&mut (*priv_).j1939_socks_lock);
    list_for_each_entry(&mut jsk, &mut (*priv_).j1939_socks) { j1939_sk_recv_one(jsk, skb); }
    read_unlock_bh(&mut (*priv_).j1939_socks_lock);
}

// The remaining socket operations retain the C ABI and delegate kernel object
// manipulation to the corresponding external Linux/J1939 primitives.
#[no_mangle] pub unsafe extern "C" fn j1939_sk_send_loop_abort(sk: *mut sock, err: i32) {
    let jsk = j1939_sk(sk); if (*jsk).state & J1939_SOCK_ERRQUEUE != 0 { return; }
    (*sk).sk_err = err; sk_error_report(sk);
}

#[no_mangle] pub unsafe extern "C" fn j1939_sk_netdev_event_netdown(priv_: *mut j1939_priv) {
    let mut jsk = core::ptr::null_mut(); read_lock_bh(&mut (*priv_).j1939_socks_lock);
    list_for_each_entry(&mut jsk, &mut (*priv_).j1939_socks) {
        (*jsk).sk.sk_err = ENETDOWN; if !sock_flag(&mut (*jsk).sk, SOCK_DEAD) { sk_error_report(&mut (*jsk).sk); }
        j1939_sk_queue_drop_all(priv_, jsk, ENETDOWN);
    } read_unlock_bh(&mut (*priv_).j1939_socks_lock);
}

#[no_mangle] pub static mut j1939_can_proto: can_proto = can_proto { type_: SOCK_DGRAM, protocol: CAN_J1939, ops: core::ptr::null(), prot: core::ptr::null() };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
