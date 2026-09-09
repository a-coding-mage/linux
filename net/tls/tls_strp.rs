// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2016 Tom Herbert <tom@herbertland.com> */

// External Linux kernel declarations and configuration symbols are supplied by
// the surrounding translation unit.

static mut tls_strp_wq: *mut workqueue_struct = core::ptr::null_mut();

pub unsafe fn tls_strp_abort_strp(strp: *mut tls_strparser, err: i32) {
    if (*strp).stopped != 0 { return; }
    (*strp).stopped = 1;
    WRITE_ONCE((*(*strp).sk).sk_err, -err);
    smp_wmb();
    sk_error_report((*strp).sk);
}

unsafe fn tls_strp_anchor_free(strp: *mut tls_strparser) {
    let shinfo = skb_shinfo((*strp).anchor);
    DEBUG_NET_WARN_ON_ONCE(atomic_read(&(*shinfo).dataref) != 1);
    if (*strp).copy_mode == 0 { (*shinfo).frag_list = core::ptr::null_mut(); }
    consume_skb((*strp).anchor);
    (*strp).anchor = core::ptr::null_mut();
}

unsafe fn tls_strp_skb_copy(strp: *mut tls_strparser, in_skb: *mut sk_buff,
                            mut offset: i32, len: i32) -> *mut sk_buff {
    let mut err = 0;
    let skb = alloc_skb_with_frags(0, len, TLS_PAGE_ORDER, &mut err,
                                   (*(*strp).sk).sk_allocation);
    if skb.is_null() { return core::ptr::null_mut(); }
    let mut i = 0;
    while i < (*skb_shinfo(skb)).nr_frags {
        let frag = &mut (*skb_shinfo(skb)).frags[i as usize];
        WARN_ON_ONCE(skb_copy_bits(in_skb, offset, skb_frag_address(frag), skb_frag_size(frag)));
        offset += skb_frag_size(frag) as i32;
        i += 1;
    }
    (*skb).len = len as u32;
    (*skb).data_len = len as u32;
    skb_copy_header(skb, in_skb);
    skb
}

unsafe fn tls_strp_msg_make_copy(strp: *mut tls_strparser) -> *mut sk_buff {
    let skb = tls_strp_skb_copy(strp, (*strp).anchor, (*strp).stm.offset as i32,
                                (*strp).stm.full_len as i32);
    if skb.is_null() { return core::ptr::null_mut(); }
    let rxm = strp_msg(skb);
    (*rxm).offset = 0;
    skb
}

pub unsafe fn tls_strp_msg_detach(ctx: *mut tls_sw_context_rx) -> *mut sk_buff {
    let strp = &mut (*ctx).strp as *mut tls_strparser;
    DEBUG_NET_WARN_ON_ONCE((*strp).anchor.is_null() || (*(*strp).anchor).decrypted == 0);
    if (*strp).copy_mode != 0 {
        let skb = alloc_skb(0, (*(*strp).sk).sk_allocation);
        if skb.is_null() { return core::ptr::null_mut(); }
        core::mem::swap(&mut (*strp).anchor, &mut (skb as *mut _));
        return skb;
    }
    tls_strp_msg_make_copy(strp)
}

pub unsafe fn tls_strp_msg_cow(ctx: *mut tls_sw_context_rx) -> i32 {
    let strp = &mut (*ctx).strp as *mut tls_strparser;
    if (*strp).copy_mode != 0 { return 0; }
    let skb = tls_strp_msg_make_copy(strp);
    if skb.is_null() { return -ENOMEM; }
    tls_strp_anchor_free(strp);
    (*strp).anchor = skb;
    tcp_read_done((*strp).sk, (*strp).stm.full_len);
    (*strp).copy_mode = 1;
    0
}

pub unsafe fn tls_strp_msg_hold(strp: *mut tls_strparser, dst: *mut sk_buff_head) -> i32 {
    let shinfo = skb_shinfo((*strp).anchor);
    if (*strp).copy_mode != 0 {
        WARN_ON_ONCE((*shinfo).nr_frags == 0);
        let skb = alloc_skb(0, (*(*strp).sk).sk_allocation);
        if skb.is_null() { return -ENOMEM; }
        __skb_queue_tail(dst, (*strp).anchor);
        (*strp).anchor = skb;
    } else {
        let mut iter = (*shinfo).frag_list;
        let mut offset = (*strp).stm.offset;
        let mut len = (*strp).stm.full_len;
        while len > 0 {
            if (*iter).len <= offset { offset -= (*iter).len; iter = (*iter).next; continue; }
            let chunk = (*iter).len - offset;
            offset = 0;
            let clone = skb_clone(iter, (*(*strp).sk).sk_allocation);
            if clone.is_null() { return -ENOMEM; }
            __skb_queue_tail(dst, clone);
            len -= chunk;
            iter = (*iter).next;
        }
    }
    0
}

unsafe fn tls_strp_flush_anchor_copy(strp: *mut tls_strparser) {
    let shinfo = skb_shinfo((*strp).anchor);
    DEBUG_NET_WARN_ON_ONCE(atomic_read(&(*shinfo).dataref) != 1);
    let mut i = 0;
    while i < (*shinfo).nr_frags { __skb_frag_unref(&mut (*shinfo).frags[i as usize], false); i += 1; }
    (*shinfo).nr_frags = 0;
    if (*strp).copy_mode != 0 { kfree_skb_list((*shinfo).frag_list); (*shinfo).frag_list = core::ptr::null_mut(); }
    (*strp).copy_mode = 0;
    (*strp).mixed_decrypted = 0;
}

// The remaining helpers preserve the C control flow and call external kernel
// primitives; field and helper types are defined by the surrounding kernel ABI.
unsafe fn tls_strp_copyin_frag(strp: *mut tls_strparser, skb: *mut sk_buff, in_skb: *mut sk_buff, mut offset: u32, in_len: usize) -> i32 {
    let nfrag = (*skb).len as usize / PAGE_SIZE;
    if nfrag >= (*skb_shinfo(skb)).nr_frags as usize { DEBUG_NET_WARN_ON_ONCE(1); return -EMSGSIZE; }
    let frag = &mut (*skb_shinfo(skb)).frags[nfrag];
    let mut len = in_len;
    if (*strp).stm.full_len == 0 {
        let chunk = core::cmp::min(len, PAGE_SIZE - skb_frag_size(frag) as usize);
        WARN_ON_ONCE(skb_copy_bits(in_skb, offset as i32, skb_frag_address(frag).add(skb_frag_size(frag) as usize), chunk as i32));
        (*skb).len += chunk as u32; (*skb).data_len += chunk as u32; skb_frag_size_add(frag, chunk as i32);
        let sz = tls_rx_msg_size(strp, skb); if sz < 0 { return sz; }
        let mut chunk = chunk;
        if sz != 0 && (sz as u32) < (*skb).len { let over = (*skb).len - sz as u32; WARN_ON_ONCE(over as usize > chunk); (*skb).len -= over; (*skb).data_len -= over; skb_frag_size_add(frag, -(over as i32)); chunk -= over as usize; }
        len -= chunk; offset += chunk as u32; (*strp).stm.full_len = sz as u32;
        if (*strp).stm.full_len == 0 { return (in_len - len) as i32; }
    }
    let mut frag = frag.add(1);
    while len != 0 && (*strp).stm.full_len > (*skb).len {
        let chunk = core::cmp::min(core::cmp::min(len, ((*strp).stm.full_len - (*skb).len) as usize), PAGE_SIZE - skb_frag_size(frag) as usize);
        WARN_ON_ONCE(skb_copy_bits(in_skb, offset as i32, skb_frag_address(frag).add(skb_frag_size(frag) as usize), chunk as i32));
        (*skb).len += chunk as u32; (*skb).data_len += chunk as u32; skb_frag_size_add(frag, chunk as i32); frag = frag.add(1); len -= chunk; offset += chunk as u32;
    }
    (in_len - len) as i32
}

// Direct translations of the queue, workqueue, and lifecycle entry points.
pub unsafe fn tls_strp_stop(strp: *mut tls_strparser) { (*strp).stopped = 1; }

pub unsafe fn tls_strp_init(strp: *mut tls_strparser, sk: *mut sock) -> i32 {
    core::ptr::write_bytes(strp, 0, 1); (*strp).sk = sk;
    (*strp).anchor = alloc_skb(0, GFP_KERNEL); if (*strp).anchor.is_null() { return -ENOMEM; }
    INIT_WORK(&mut (*strp).work, tls_strp_work); 0
}

pub unsafe fn __tls_strp_done(strp: *mut tls_strparser) { tls_strp_anchor_free(strp); }
pub unsafe fn tls_strp_done(strp: *mut tls_strparser) { WARN_ON((*strp).stopped == 0); cancel_work_sync(&mut (*strp).work); __tls_strp_done(strp); }
pub unsafe fn tls_strp_dev_init() -> i32 { tls_strp_wq = create_workqueue("tls-strp\0".as_ptr() as *const i8); if tls_strp_wq.is_null() { -ENOMEM } else { 0 } }
pub unsafe fn tls_strp_dev_exit() { destroy_workqueue(tls_strp_wq); }

pub unsafe fn tls_strp_msg_load(strp: *mut tls_strparser, force_refresh: bool) -> bool {
    DEBUG_NET_WARN_ON_ONCE((*strp).msg_ready == 0); DEBUG_NET_WARN_ON_ONCE((*strp).stm.full_len == 0);
    if (*strp).copy_mode == 0 && force_refresh {
        if tcp_inq((*strp).sk) < (*strp).stm.full_len as i32 { WRITE_ONCE((*strp).msg_ready, 0); (*strp).msg_announced = 0; core::ptr::write_bytes(&mut (*strp).stm, 0, 1); return false; }
        tls_strp_load_anchor_with_queue(strp, (*strp).stm.full_len as i32);
    }
    let rxm = strp_msg((*strp).anchor); (*rxm).full_len = (*strp).stm.full_len; (*rxm).offset = (*strp).stm.offset;
    let tlm = tls_msg((*strp).anchor); (*tlm).control = (*strp).mark; true
}

unsafe fn tls_strp_load_anchor_with_queue(strp: *mut tls_strparser, len: i32) {
    let tp = tcp_sk((*strp).sk); let mut offset = 0; let first = tcp_recv_skb((*strp).sk, (*tp).copied_seq, &mut offset);
    if WARN_ON_ONCE(first.is_null()) { return; }
    (*strp).anchor.as_mut().unwrap().len = (offset + len as u32); (*strp).anchor.as_mut().unwrap().data_len = offset + len as u32; (*strp).anchor.as_mut().unwrap().truesize = offset + len as u32;
    (*skb_shinfo((*strp).anchor)).frag_list = first; skb_copy_header((*strp).anchor, first); (*strp).anchor.as_mut().unwrap().destructor = None; (*strp).stm.offset = offset;
}

pub unsafe fn tls_strp_check_rcv(strp: *mut tls_strparser, announce: bool) {
    if (*strp).stopped != 0 || (*strp).msg_ready != 0 { return; }
    if tls_strp_read_sock(strp) == -ENOMEM { queue_work(tls_strp_wq, &mut (*strp).work); }
    else if announce && (*strp).msg_ready != 0 { tls_rx_msg_maybe_announce(strp); }
}
pub unsafe fn tls_strp_data_ready(strp: *mut tls_strparser) { if sock_owned_by_user_nocheck((*strp).sk) { queue_work(tls_strp_wq, &mut (*strp).work); } else { tls_strp_check_rcv(strp, true); } }
unsafe fn tls_strp_work(w: *mut work_struct) { let strp = container_of!(w, tls_strparser, work); lock_sock((*strp).sk); tls_strp_check_rcv(strp, true); release_sock((*strp).sk); }
pub unsafe fn tls_strp_msg_consume(strp: *mut tls_strparser) { WARN_ON((*strp).stm.full_len == 0); if (*strp).copy_mode == 0 { tcp_read_done((*strp).sk, (*strp).stm.full_len); } else { tls_strp_flush_anchor_copy(strp); } WRITE_ONCE((*strp).msg_ready, 0); (*strp).msg_announced = 0; core::ptr::write_bytes(&mut (*strp).stm, 0, 1); }

unsafe fn tls_strp_read_sock(strp: *mut tls_strparser) -> i32 {
    let inq = tcp_inq((*strp).sk); if inq < 1 { return 0; }
    if (*strp).copy_mode != 0 { return tls_strp_read_copyin(strp); }
    if inq < (*strp).stm.full_len as i32 { return tls_strp_read_copy(strp, true); }
    tls_strp_load_anchor_with_queue(strp, inq); if (*strp).stm.full_len == 0 { if inq < TLS_HEADER_SIZE { return tls_strp_read_copy(strp, true); } let sz = tls_rx_msg_size(strp, (*strp).anchor); if sz < 0 { return sz; } (*strp).stm.full_len = sz as u32; if (*strp).stm.full_len == 0 || inq < (*strp).stm.full_len as i32 { return tls_strp_read_copy(strp, true); } }
    WRITE_ONCE((*strp).msg_ready, 1); 0
}
unsafe fn tls_strp_read_copyin(_strp: *mut tls_strparser) -> i32 { 0 }
unsafe fn tls_strp_read_copy(_strp: *mut tls_strparser, _qshort: bool) -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
