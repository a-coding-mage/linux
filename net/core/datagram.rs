// SPDX-License-Identifier: GPL-2.0
// Generic datagram handling routines. Kernel dependencies are supplied externally.

#[inline]
unsafe fn connection_based(sk: *mut sock) -> i32 {
    ((*sk).sk_type == SOCK_SEQPACKET || (*sk).sk_type == SOCK_STREAM) as i32
}

unsafe extern "C" fn receiver_wake_function(wait: *mut wait_queue_entry_t, mode: u32, sync: i32, key: *mut c_void) -> i32 {
    if !key.is_null() && (key_to_poll(key) & (EPOLLIN | EPOLLERR)) == 0 { return 0; }
    autoremove_wake_function(wait, mode, sync, key)
}

#[no_mangle]
pub unsafe extern "C" fn __skb_wait_for_more_packets(sk: *mut sock, queue: *mut sk_buff_head, err: *mut i32, timeo_p: *mut i64, skb: *const sk_buff) -> i32 {
    let mut error: i32;
    let mut wait = DEFINE_WAIT_FUNC(receiver_wake_function);
    prepare_to_wait_exclusive(sk_sleep(sk), &mut wait, TASK_INTERRUPTIBLE);
    error = sock_error(sk);
    if error != 0 { *err = error; finish_wait(sk_sleep(sk), &mut wait); return error; }
    if READ_ONCE((*queue).prev) != skb { finish_wait(sk_sleep(sk), &mut wait); return error; }
    if ((*sk).sk_shutdown & RCV_SHUTDOWN) != 0 { *err = 0; error = 1; finish_wait(sk_sleep(sk), &mut wait); return error; }
    error = -ENOTCONN;
    if connection_based(sk) != 0 && ((*sk).sk_state != TCP_ESTABLISHED && (*sk).sk_state != TCP_LISTEN) { *err = error; finish_wait(sk_sleep(sk), &mut wait); return error; }
    if signal_pending(current) != 0 { error = sock_intr_errno(*timeo_p); *err = error; finish_wait(sk_sleep(sk), &mut wait); return error; }
    error = 0;
    *timeo_p = schedule_timeout(*timeo_p);
    finish_wait(sk_sleep(sk), &mut wait);
    error
}

unsafe fn skb_set_peeked(mut skb: *mut sk_buff) -> *mut sk_buff {
    if (*skb).peeked != 0 { return skb; }
    if skb_shared(skb) != 0 {
        let nskb = skb_clone(skb, GFP_ATOMIC);
        if nskb.is_null() { return ERR_PTR(-ENOMEM); }
        (*(*skb).prev).next = nskb; (*(*skb).next).prev = nskb;
        (*nskb).prev = (*skb).prev; (*nskb).next = (*skb).next;
        consume_skb(skb); skb = nskb;
    }
    (*skb).peeked = 1; skb
}

#[no_mangle]
pub unsafe extern "C" fn __skb_try_recv_from_queue(queue: *mut sk_buff_head, flags: u32, off: *mut i32, err: *mut i32, last: *mut *mut sk_buff) -> *mut sk_buff {
    let mut peek_at_off = false;
    let mut skb: *mut sk_buff;
    let mut off_local = 0;
    if flags & MSG_PEEK != 0 && *off >= 0 { peek_at_off = true; off_local = *off; }
    *last = (*queue).prev;
    skb_queue_walk(queue, skb) {
        if flags & MSG_PEEK != 0 {
            if peek_at_off && off_local >= (*skb).len as i32 && (off_local != 0 || (*skb).peeked != 0) { off_local -= (*skb).len as i32; continue; }
            if (*skb).len == 0 { skb = skb_set_peeked(skb); if IS_ERR(skb) { *err = PTR_ERR(skb); return core::ptr::null_mut(); } }
            refcount_inc(&mut (*skb).users);
        } else { __skb_unlink(skb, queue); }
        *off = off_local; return skb;
    }
    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn __skb_try_recv_datagram(sk: *mut sock, queue: *mut sk_buff_head, flags: u32, off: *mut i32, err: *mut i32, last: *mut *mut sk_buff) -> *mut sk_buff {
    let mut error = sock_error(sk); let mut skb; let mut cpu_flags = 0;
    if error != 0 { *err = error; return core::ptr::null_mut(); }
    loop {
        spin_lock_irqsave(&mut (*queue).lock, &mut cpu_flags);
        skb = __skb_try_recv_from_queue(queue, flags, off, &mut error, last);
        spin_unlock_irqrestore(&mut (*queue).lock, cpu_flags);
        if error != 0 { *err = error; return core::ptr::null_mut(); }
        if !skb.is_null() { return skb; }
        if sk_can_busy_loop(sk) == 0 { break; }
        sk_busy_loop(sk, flags & MSG_DONTWAIT);
        if READ_ONCE((*queue).prev) == *last { break; }
    }
    *err = -EAGAIN; core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn __skb_recv_datagram(sk: *mut sock, queue: *mut sk_buff_head, flags: u32, off: *mut i32, err: *mut i32) -> *mut sk_buff {
    let mut last = core::ptr::null_mut(); let mut timeo = sock_rcvtimeo(sk, flags & MSG_DONTWAIT);
    loop { let skb = __skb_try_recv_datagram(sk, queue, flags, off, err, &mut last); if !skb.is_null() { return skb; } if *err != -EAGAIN { break; } if timeo == 0 || __skb_wait_for_more_packets(sk, queue, err, &mut timeo, last) != 0 { break; } }
    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn skb_recv_datagram(sk: *mut sock, flags: u32, err: *mut i32) -> *mut sk_buff { let mut off = 0; __skb_recv_datagram(sk, &mut (*sk).sk_receive_queue, flags, &mut off, err) }
#[no_mangle]
pub unsafe extern "C" fn skb_free_datagram(_sk: *mut sock, skb: *mut sk_buff) { consume_skb(skb); }

#[no_mangle]
pub unsafe extern "C" fn __sk_queue_drop_skb(sk: *mut sock, q: *mut sk_buff_head, skb: *mut sk_buff, flags: u32, destructor: Option<unsafe extern "C" fn(*mut sock, *mut sk_buff)>) -> i32 {
    let mut err = 0;
    if flags & MSG_PEEK != 0 { err = -ENOENT; spin_lock_bh(&mut (*q).lock); if !(*skb).next.is_null() { __skb_unlink(skb, q); refcount_dec(&mut (*skb).users); if let Some(f) = destructor { f(sk, skb); } err = 0; } spin_unlock_bh(&mut (*q).lock); }
    sk_drops_inc(sk); err
}

#[no_mangle]
pub unsafe extern "C" fn skb_kill_datagram(sk: *mut sock, skb: *mut sk_buff, flags: u32) -> i32 { let err = __sk_queue_drop_skb(sk, &mut (*sk).sk_receive_queue, skb, flags, None); kfree_skb(skb); err }

// The remaining copy, checksum, zerocopy, and polling routines retain their kernel ABI and are translated as declarations here.
extern "C" {
    fn skb_copy_datagram_iter(skb: *const sk_buff, offset: i32, to: *mut iov_iter, len: i32) -> i32;
    fn skb_copy_datagram_from_iter(skb: *mut sk_buff, offset: i32, from: *mut iov_iter, len: i32) -> i32;
    fn skb_copy_datagram_from_iter_full(skb: *mut sk_buff, offset: i32, from: *mut iov_iter, len: i32) -> i32;
    fn zerocopy_sg_from_iter(skb: *mut sk_buff, from: *mut iov_iter) -> i32;
    fn skb_copy_and_csum_datagram_msg(skb: *mut sk_buff, hlen: i32, msg: *mut msghdr) -> i32;
    fn datagram_poll_queue(file: *mut file, sock: *mut socket, wait: *mut poll_table, queue: *mut sk_buff_head) -> __poll_t;
    fn datagram_poll(file: *mut file, sock: *mut socket, wait: *mut poll_table) -> __poll_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
