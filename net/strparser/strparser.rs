// SPDX-License-Identifier: GPL-2.0-only
/*
 * Stream Parser
 *
 * Copyright (c) 2016 Tom Herbert <tom@herbertland.com>
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

static mut strp_wq: *mut workqueue_struct = core::ptr::null_mut();

#[inline]
unsafe fn _strp_msg(skb: *mut sk_buff) -> *mut _strp_msg {
    (unsafe { (*skb).cb.as_mut_ptr().add(core::mem::offset_of!(sk_skb_cb, strp)) }) as *mut _strp_msg
}

/* Lower lock held */
unsafe fn strp_abort_strp(strp: *mut strparser, err: i32) {
    cancel_delayed_work(&mut (*strp).msg_timer_work);
    if (*strp).stopped != 0 { return; }
    (*strp).stopped = 1;
    if !(*strp).skb_head.is_null() {
        kfree_skb((*strp).skb_head);
        (*strp).skb_head = core::ptr::null_mut();
    }
    (*strp).skb_nextp = core::ptr::null_mut();
    (*strp).need_bytes = 0;
    if !(*strp).sk.is_null() {
        let sk = (*strp).sk;
        (*sk).sk_err = -err;
        sk_error_report(sk);
    }
}

unsafe fn strp_start_timer(strp: *mut strparser, timeo: i64) {
    if timeo != 0 && timeo != LONG_MAX { mod_delayed_work(strp_wq, &mut (*strp).msg_timer_work, timeo); }
}

/* Lower lock held */
unsafe fn strp_parser_err(strp: *mut strparser, err: i32, desc: *mut read_descriptor_t) {
    (*desc).error = err;
    kfree_skb((*strp).skb_head);
    (*strp).skb_head = core::ptr::null_mut();
    ((*strp).cb.abort_parser)(strp, err);
}

#[inline]
unsafe fn strp_peek_len(strp: *mut strparser) -> i32 {
    if !(*strp).sk.is_null() {
        let sock = (*(*strp).sk).sk_socket;
        return ((*(*sock).ops).peek_len)(sock);
    }
    INT_MAX
}

/* Lower socket lock held */
unsafe fn __strp_recv(desc: *mut read_descriptor_t, mut orig_skb: *mut sk_buff,
    mut orig_offset: u32, orig_len: usize, max_msg_size: usize, timeo: i64) -> usize {
    let strp = (*desc).arg.data as *mut strparser;
    let mut stm: *mut _strp_msg;
    let mut head: *mut sk_buff;
    let mut skb: *mut sk_buff;
    let mut eaten: usize = 0;
    let mut cand_len: usize;
    let mut extra: isize;
    let mut err: i32;
    let mut cloned_orig = false;
    if (*strp).paused != 0 { return 0; }
    head = (*strp).skb_head;
    if !head.is_null() {
        if orig_offset != 0 {
            orig_skb = skb_clone(orig_skb, GFP_ATOMIC);
            if orig_skb.is_null() { STRP_STATS_INCR!((*strp).stats.mem_fail); (*desc).error = -ENOMEM; return 0; }
            if pskb_pull(orig_skb, orig_offset) == 0 { STRP_STATS_INCR!((*strp).stats.mem_fail); kfree_skb(orig_skb); (*desc).error = -ENOMEM; return 0; }
            cloned_orig = true; orig_offset = 0;
        }
        if (*strp).skb_nextp.is_null() {
            err = skb_unclone(head, GFP_ATOMIC);
            if err != 0 { STRP_STATS_INCR!((*strp).stats.mem_fail); (*desc).error = err; return 0; }
            if !skb_shinfo(head).frag_list.is_null() {
                if WARN_ON!(!(*head).next.is_null()) { (*desc).error = -EINVAL; return 0; }
                skb = alloc_skb_for_msg(head);
                if skb.is_null() { STRP_STATS_INCR!((*strp).stats.mem_fail); (*desc).error = -ENOMEM; return 0; }
                (*strp).skb_nextp = &mut (*head).next;
                (*strp).skb_head = skb; head = skb;
            } else { (*strp).skb_nextp = &mut skb_shinfo(head).frag_list; }
        }
    }
    while eaten < orig_len {
        skb = skb_clone(orig_skb, GFP_ATOMIC);
        if skb.is_null() { STRP_STATS_INCR!((*strp).stats.mem_fail); (*desc).error = -ENOMEM; break; }
        cand_len = orig_len - eaten;
        head = (*strp).skb_head;
        if head.is_null() {
            head = skb; (*strp).skb_head = head; (*strp).skb_nextp = core::ptr::null_mut();
            stm = _strp_msg(head); core::ptr::write_bytes(stm, 0, 1); (*stm).strp.offset = (orig_offset as usize) + eaten;
        } else {
            if skb_has_frag_list(skb) { err = skb_unclone(skb, GFP_ATOMIC); if err != 0 { STRP_STATS_INCR!((*strp).stats.mem_fail); (*desc).error = err; break; } }
            stm = _strp_msg(head); *(*strp).skb_nextp = skb; (*strp).skb_nextp = &mut (*skb).next;
            (*head).data_len += (*skb).len; (*head).len += (*skb).len; (*head).truesize += (*skb).truesize;
        }
        if (*stm).strp.full_len == 0 {
            let mut len = ((*strp).cb.parse_msg)(strp, head);
            if len == 0 { if (*stm).accum_len == 0 { strp_start_timer(strp, timeo); } (*stm).accum_len += cand_len; eaten += cand_len; STRP_STATS_INCR!((*strp).stats.need_more_hdr); WARN_ON!(eaten != orig_len); break; }
            if len < 0 { if len == -ESTRPIPE && (*stm).accum_len != 0 { len = -ENODATA; (*strp).unrecov_intr = 1; } else { (*strp).interrupted = 1; } strp_parser_err(strp, len, desc); break; }
            if (len as usize) > max_msg_size { STRP_STATS_INCR!((*strp).stats.msg_too_big); strp_parser_err(strp, -EMSGSIZE, desc); break; }
            if len as isize <= (*head).len as isize - (*skb).len as isize - (*stm).strp.offset as isize { STRP_STATS_INCR!((*strp).stats.bad_hdr_len); strp_parser_err(strp, -EPROTO, desc); break; }
            (*stm).strp.full_len = len as usize;
        }
        extra = ((*stm).accum_len + cand_len) as isize - (*stm).strp.full_len as isize;
        if extra < 0 { if ((*stm).strp.full_len - (*stm).accum_len) as i32 > strp_peek_len(strp) { if (*stm).accum_len == 0 { strp_start_timer(strp, timeo); } (*stm).accum_len += cand_len; eaten += cand_len; (*strp).need_bytes = (*stm).strp.full_len - (*stm).accum_len; STRP_STATS_ADD!((*strp).stats.bytes, cand_len); (*desc).count = 0; break; } (*stm).accum_len += cand_len; eaten += cand_len; WARN_ON!(eaten != orig_len); break; }
        WARN_ON!(extra as usize > cand_len); eaten += cand_len - extra as usize;
        cancel_delayed_work(&mut (*strp).msg_timer_work); (*strp).skb_head = core::ptr::null_mut(); (*strp).need_bytes = 0; STRP_STATS_INCR!((*strp).stats.msgs);
        ((*strp).cb.rcv_msg)(strp, head); if (*strp).paused != 0 { break; }
    }
    if cloned_orig { kfree_skb(orig_skb); }
    STRP_STATS_ADD!((*strp).stats.bytes, eaten); eaten
}

pub unsafe fn strp_process(strp: *mut strparser, orig_skb: *mut sk_buff, orig_offset: u32, orig_len: usize, max_msg_size: usize, timeo: i64) -> usize {
    let mut desc: read_descriptor_t = core::mem::zeroed(); desc.arg.data = strp as *mut _; __strp_recv(&mut desc, orig_skb, orig_offset, orig_len, max_msg_size, timeo)
}

unsafe fn strp_recv(desc: *mut read_descriptor_t, orig_skb: *mut sk_buff, orig_offset: u32, orig_len: usize) -> usize {
    let strp = (*desc).arg.data as *mut strparser;
    __strp_recv(desc, orig_skb, orig_offset, orig_len, (*(*strp).sk).sk_rcvbuf as usize, READ_ONCE!((*(*strp).sk).sk_rcvtimeo))
}

unsafe fn default_read_sock_done(_strp: *mut strparser, err: i32) -> i32 { err }

/* Called with lock held on lower socket */
unsafe fn strp_read_sock(strp: *mut strparser) -> i32 {
    let sock = (*strp).sk_socket();
    let mut desc: read_descriptor_t = core::mem::zeroed();
    if sock.is_null() || (*sock).ops.is_null() || ((*strp).cb.read_sock.is_none() && (*(*sock).ops).read_sock.is_none()) { return -EBUSY; }
    desc.arg.data = strp as *mut _; desc.error = 0; desc.count = 1;
    if let Some(f) = (*strp).cb.read_sock { f(strp, &mut desc, strp_recv); } else { ((*(*sock).ops).read_sock)(*strp).sk, &mut desc, strp_recv); }
    desc.error = ((*strp).cb.read_sock_done)(strp, desc.error); desc.error
}

pub unsafe fn strp_data_ready(strp: *mut strparser) {
    if (*strp).stopped != 0 || (*strp).paused != 0 { return; }
    if sock_owned_by_user_nocheck((*strp).sk) { queue_work(strp_wq, &mut (*strp).work); return; }
    if (*strp).need_bytes != 0 && (strp_peek_len(strp) as usize) < (*strp).need_bytes { return; }
    if strp_read_sock(strp) == -ENOMEM { queue_work(strp_wq, &mut (*strp).work); }
}

unsafe fn do_strp_work(strp: *mut strparser) { ((*strp).cb.lock)(strp); if (*strp).stopped == 0 && (*strp).paused == 0 && strp_read_sock(strp) == -ENOMEM { queue_work(strp_wq, &mut (*strp).work); } ((*strp).cb.unlock)(strp); }
unsafe fn strp_work(w: *mut work_struct) { do_strp_work(container_of!(w, strparser, work)); }
unsafe fn strp_msg_timeout(w: *mut work_struct) { let strp = container_of!(w, strparser, msg_timer_work.work); STRP_STATS_INCR!((*strp).stats.msg_timeouts); ((*strp).cb.lock)(strp); ((*strp).cb.abort_parser)(strp, -ETIMEDOUT); ((*strp).cb.unlock)(strp); }
unsafe fn strp_sock_lock(strp: *mut strparser) { lock_sock((*strp).sk); }
unsafe fn strp_sock_unlock(strp: *mut strparser) { release_sock((*strp).sk); }

pub unsafe fn strp_init(strp: *mut strparser, sk: *mut sock, cb: *const strp_callbacks) -> i32 {
    if cb.is_null() || (*cb).rcv_msg.is_none() || (*cb).parse_msg.is_none() { return -EINVAL; }
    if sk.is_null() && ((*cb).lock.is_none() || (*cb).unlock.is_none()) { return -EINVAL; }
    core::ptr::write_bytes(strp, 0, 1); (*strp).sk = sk;
    (*strp).cb.lock = (*cb).lock.or(Some(strp_sock_lock)); (*strp).cb.unlock = (*cb).unlock.or(Some(strp_sock_unlock));
    (*strp).cb.rcv_msg = (*cb).rcv_msg; (*strp).cb.parse_msg = (*cb).parse_msg; (*strp).cb.read_sock = (*cb).read_sock;
    (*strp).cb.read_sock_done = (*cb).read_sock_done.or(Some(default_read_sock_done)); (*strp).cb.abort_parser = (*cb).abort_parser.or(Some(strp_abort_strp));
    INIT_DELAYED_WORK!(&mut (*strp).msg_timer_work, strp_msg_timeout); INIT_WORK!(&mut (*strp).work, strp_work); 0
}

pub unsafe fn strp_unpause(strp: *mut strparser) { (*strp).paused = 0; smp_mb!(); queue_work(strp_wq, &mut (*strp).work); }
pub unsafe fn strp_done(strp: *mut strparser) { WARN_ON!((*strp).stopped == 0); cancel_delayed_work_sync(&mut (*strp).msg_timer_work); cancel_work_sync(&mut (*strp).work); if !(*strp).skb_head.is_null() { kfree_skb((*strp).skb_head); (*strp).skb_head = core::ptr::null_mut(); } }
pub unsafe fn strp_stop(strp: *mut strparser) { (*strp).stopped = 1; }
pub unsafe fn strp_check_rcv(strp: *mut strparser) { queue_work(strp_wq, &mut (*strp).work); }

unsafe fn strp_dev_init() -> i32 {
    BUILD_BUG_ON!(core::mem::size_of::<sk_skb_cb>() > core::mem::size_of::<sk_buff_cb>());
    strp_wq = create_singlethread_workqueue(c"kstrp".as_ptr());
    if strp_wq.is_null() { return -ENOMEM; } 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
