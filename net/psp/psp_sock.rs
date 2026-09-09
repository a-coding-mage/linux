// SPDX-License-Identifier: GPL-2.0-only

// Translated from psp_sock.c. Kernel types, helpers, constants, and globals
// supplied by the surrounding PSP and networking code are intentionally external.

pub unsafe fn psp_dev_get_for_sock(sk: *mut sock) -> *mut psp_dev {
    let mut psd: *mut psp_dev = core::ptr::null_mut();
    rcu_read_lock();
    let dst = __sk_dst_get(sk);
    if !dst.is_null() {
        psd = (*dst_dev_rcu(dst)).psp_dev;
        if !psd.is_null() && !psp_dev_tryget(psd) {
            psd = core::ptr::null_mut();
        }
    }
    rcu_read_unlock();
    psd
}

unsafe fn psp_validate_xmit(sk: *mut sock, dev: *mut net_device, skb: *mut sk_buff) -> *mut sk_buff {
    rcu_read_lock();
    let pas = psp_skb_get_assoc_rcu(skb);
    let good = pas.is_null() || (*dev).psp_dev == (*pas).psd;
    rcu_read_unlock();
    if !good {
        sk_skb_reason_drop(sk, skb, SKB_DROP_REASON_PSP_OUTPUT);
        return core::ptr::null_mut();
    }
    skb
}

pub unsafe fn psp_assoc_create(psd: *mut psp_dev) -> *mut psp_assoc {
    // lockdep_assert_held(&(*psd).lock)
    let pas = kzalloc_flex::<psp_assoc>((*psd).caps.assoc_drv_spc, GFP_KERNEL_ACCOUNT);
    if pas.is_null() { return core::ptr::null_mut(); }
    (*pas).psd = psd;
    (*pas).dev_id = (*psd).id;
    (*pas).generation = (*psd).generation;
    psp_dev_get(psd);
    refcount_set(&mut (*pas).refcnt, 1);
    list_add_tail(&mut (*pas).assocs_list, &mut (*psd).active_assocs);
    pas
}

unsafe fn psp_assoc_dummy(pas: *mut psp_assoc) -> *mut psp_assoc {
    let psd = (*pas).psd;
    // lockdep_assert_held(&(*psd).lock)
    let sz = struct_size_assoc((*psd).caps.assoc_drv_spc);
    kmemdup(pas as *const _, sz, GFP_KERNEL)
}

unsafe fn psp_dev_tx_key_add(psd: *mut psp_dev, pas: *mut psp_assoc, extack: *mut netlink_ext_ack) -> i32 {
    ((*(*psd).ops).tx_key_add)(psd, pas, extack)
}

pub unsafe fn psp_dev_tx_key_del(psd: *mut psp_dev, pas: *mut psp_assoc) {
    if (*pas).tx.spi != 0 { ((*(*psd).ops).tx_key_del)(psd, pas); }
    list_del(&mut (*pas).assocs_list);
}

unsafe fn psp_assoc_free(work: *mut work_struct) {
    let pas = container_of_assoc_work(work);
    let psd = (*pas).psd;
    mutex_lock(&mut (*psd).lock);
    if psp_dev_is_registered(psd) { psp_dev_tx_key_del(psd, pas); }
    mutex_unlock(&mut (*psd).lock);
    psp_dev_put(psd);
    kfree(pas);
}

unsafe fn psp_assoc_free_queue(head: *mut rcu_head) {
    let pas = container_of_assoc_rcu(head);
    INIT_WORK(&mut (*pas).work, psp_assoc_free);
    schedule_work(&mut (*pas).work);
}

/// psp_assoc_put() - release a reference on a PSP association
/// @pas: association to release
pub unsafe fn psp_assoc_put(pas: *mut psp_assoc) {
    if !pas.is_null() && refcount_dec_and_test(&mut (*pas).refcnt) {
        call_rcu(&mut (*pas).rcu, psp_assoc_free_queue);
    }
}

pub unsafe fn psp_sk_assoc_free(sk: *mut sock) {
    let pas = rcu_dereference_protected((*sk).psp_assoc, 1);
    rcu_assign_pointer(&mut (*sk).psp_assoc, core::ptr::null_mut());
    psp_assoc_put(pas);
}

pub unsafe fn psp_sock_assoc_set_rx(sk: *mut sock, pas: *mut psp_assoc, key: *mut psp_key_parsed, extack: *mut netlink_ext_ack) -> i32 {
    memcpy(&mut (*pas).rx as *mut _ as *mut _, key as *const _, core::mem::size_of::<psp_key_parsed>());
    lock_sock(sk);
    let err;
    if !psp_sk_assoc(sk).is_null() {
        NL_SET_ERR_MSG(extack, "Socket already has PSP state");
        err = -EBUSY;
    } else {
        refcount_inc(&mut (*pas).refcnt);
        rcu_assign_pointer(&mut (*sk).psp_assoc, pas);
        err = 0;
    }
    release_sock(sk);
    err
}

unsafe fn psp_sock_recv_queue_check(sk: *mut sock, pas: *mut psp_assoc) -> i32 {
    let mut skb: *mut sk_buff = core::ptr::null_mut();
    let mut pse: *mut psp_skb_ext;
    skb_rbtree_walk!(skb, &mut (*tcp_sk(sk)).out_of_order_queue, {
        pse = skb_ext_find(skb, SKB_EXT_PSP);
        if !psp_pse_matches_pas(pse, pas) { return -EBUSY; }
    });
    skb_queue_walk!(sk, skb, {
        pse = skb_ext_find(skb, SKB_EXT_PSP);
        if !psp_pse_matches_pas(pse, pas) { return -EBUSY; }
    });
    0
}

pub unsafe fn psp_sock_assoc_set_tx(sk: *mut sock, psd: *mut psp_dev, version: u32, key: *mut psp_key_parsed, extack: *mut netlink_ext_ack) -> i32 {
    lock_sock(sk);
    let pas = psp_sk_assoc(sk);
    if pas.is_null() { NL_SET_ERR_MSG(extack, "Socket has no Rx key"); release_sock(sk); return -EINVAL; }
    if (*pas).psd != psd { NL_SET_ERR_MSG(extack, "Rx key from different device"); release_sock(sk); return -EINVAL; }
    if (*pas).version != version { NL_SET_ERR_MSG(extack, "PSP version mismatch with existing state"); release_sock(sk); return -EINVAL; }
    if (*pas).tx.spi != 0 { NL_SET_ERR_MSG(extack, "Tx key already set"); release_sock(sk); return -EBUSY; }
    let err = psp_sock_recv_queue_check(sk, pas);
    if err != 0 { NL_SET_ERR_MSG(extack, "Socket has incompatible segments already in the recv queue"); release_sock(sk); return err; }
    let dummy = psp_assoc_dummy(pas);
    if dummy.is_null() { release_sock(sk); return -ENOMEM; }
    memcpy(&mut (*dummy).tx as *mut _ as *mut _, key as *const _, core::mem::size_of::<psp_key_parsed>());
    let err = psp_dev_tx_key_add(psd, dummy, extack);
    if err != 0 { kfree(dummy); release_sock(sk); return err; }
    memcpy((*pas).drv_data, (*dummy).drv_data, (*psd).caps.assoc_drv_spc);
    memcpy(&mut (*pas).tx as *mut _ as *mut _, key as *const _, core::mem::size_of::<psp_key_parsed>());
    WRITE_ONCE(&mut (*sk).sk_validate_xmit_skb, Some(psp_validate_xmit));
    tcp_write_collapse_fence(sk);
    (*pas).upgrade_seq = (*tcp_sk(sk)).rcv_nxt;
    let icsk = inet_csk(sk);
    (*icsk).icsk_ext_hdr_len += psp_sk_overhead(sk);
    ((*icsk).icsk_sync_mss)(sk, (*icsk).icsk_pmtu_cookie);
    kfree(dummy);
    release_sock(sk);
    0
}

pub unsafe fn psp_assocs_key_rotated(psd: *mut psp_dev) {
    let mut pas: *mut psp_assoc;
    let mut next: *mut psp_assoc;
    list_for_each_entry_safe!(pas, next, &mut (*psd).prev_assocs, assocs_list, {
        (*pas).generation |= !PSP_GEN_VALID_MASK;
        (*psd).stats.stales += 1;
    });
    list_splice_init(&mut (*psd).prev_assocs, &mut (*psd).stale_assocs);
    list_splice_init(&mut (*psd).active_assocs, &mut (*psd).prev_assocs);
    // TODO: we should inform the sockets that got shut down
}

pub unsafe fn psp_twsk_init(tw: *mut inet_timewait_sock, sk: *const sock) {
    let pas = psp_sk_assoc(sk as *mut sock);
    if !pas.is_null() { refcount_inc(&mut (*pas).refcnt); }
    rcu_assign_pointer(&mut (*tw).psp_assoc, pas);
    (*tw).tw_validate_xmit_skb = Some(psp_validate_xmit);
}

pub unsafe fn psp_twsk_assoc_free(tw: *mut inet_timewait_sock) {
    let pas = rcu_dereference_protected((*tw).psp_assoc, 1);
    rcu_assign_pointer(&mut (*tw).psp_assoc, core::ptr::null_mut());
    psp_assoc_put(pas);
}

pub unsafe fn psp_reply_set_decrypted(sk: *const sock, skb: *mut sk_buff) {
    rcu_read_lock();
    let pas = psp_sk_get_assoc_rcu(sk);
    if !pas.is_null() && (*pas).tx.spi != 0 { (*skb).decrypted = 1; }
    rcu_read_unlock();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
