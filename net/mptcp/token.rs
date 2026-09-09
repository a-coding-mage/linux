// SPDX-License-Identifier: GPL-2.0
/* Multipath TCP token management
 * Copyright (c) 2017 - 2019, Intel Corporation.
 */

// C headers and build-time configuration are supplied by the surrounding
// kernel translation unit.

const TOKEN_MAX_CHAIN_LEN: i32 = 4;

#[repr(C)]
struct token_bucket {
    lock: spinlock_t,
    chain_len: i32,
    req_chain: hlist_nulls_head,
    msk_chain: hlist_nulls_head,
}

static mut token_hash: *mut token_bucket = core::ptr::null_mut();
static mut token_mask: u32 = 0;

unsafe fn token_bucket_for(token: u32) -> *mut token_bucket {
    &mut *token_hash.add((token & token_mask) as usize)
}

/* called with bucket lock held */
unsafe fn __token_lookup_req(t: *mut token_bucket, token: u32) -> *mut mptcp_subflow_request_sock {
    let mut req: *mut mptcp_subflow_request_sock = core::ptr::null_mut();
    let mut pos: *mut hlist_nulls_node = core::ptr::null_mut();
    hlist_nulls_for_each_entry_rcu!(req, pos, &mut (*t).req_chain, token_node);
    if !req.is_null() && (*req).token == token { return req; }
    core::ptr::null_mut()
}

/* called with bucket lock held */
unsafe fn __token_lookup_msk(t: *mut token_bucket, token: u32) -> *mut mptcp_sock {
    let mut pos: *mut hlist_nulls_node = core::ptr::null_mut();
    let mut sk: *mut sock = core::ptr::null_mut();
    sk_nulls_for_each_rcu!(sk, pos, &mut (*t).msk_chain);
    if !sk.is_null() && (*mptcp_sk(sk)).token == token { return mptcp_sk(sk); }
    core::ptr::null_mut()
}

unsafe fn __token_bucket_busy(t: *mut token_bucket, token: u32) -> bool {
    token == 0 || (*t).chain_len >= TOKEN_MAX_CHAIN_LEN ||
        !__token_lookup_req(t, token).is_null() || !__token_lookup_msk(t, token).is_null()
}

unsafe fn mptcp_crypto_key_gen_sha(key: *mut u64, token: *mut u32, idsn: *mut u64) {
    /* Random data is used as the safest option when sockets are opened in
     * different namespaces at the same time. */
    get_random_bytes(key as *mut core::ffi::c_void, core::mem::size_of::<u64>());
    mptcp_crypto_key_sha(*key, token, idsn);
}

pub unsafe fn mptcp_token_new_request(req: *mut request_sock) -> i32 {
    let subflow_req = mptcp_subflow_rsk(req);
    (*subflow_req).token = 0;
    mptcp_crypto_key_sha((*subflow_req).local_key.as_mut_ptr(), &mut (*subflow_req).token, &mut (*subflow_req).idsn);
    let token = (*subflow_req).token;
    let bucket = token_bucket_for(token);
    spin_lock_bh(&mut (*bucket).lock);
    if __token_bucket_busy(bucket, token) { spin_unlock_bh(&mut (*bucket).lock); return -EBUSY; }
    hlist_nulls_add_head_rcu(&mut (*subflow_req).token_node, &mut (*bucket).req_chain);
    (*bucket).chain_len += 1;
    spin_unlock_bh(&mut (*bucket).lock);
    0
}

pub unsafe fn mptcp_token_new_connect(ssk: *mut sock) -> i32 {
    let subflow = mptcp_subflow_ctx(ssk);
    let msk = mptcp_sk((*subflow).conn);
    let mut retries = MPTCP_TOKEN_MAX_RETRIES;
    let sk = (*subflow).conn;
    loop {
        mptcp_crypto_key_gen_sha(&mut (*subflow).local_key, &mut (*subflow).token, &mut (*subflow).idsn);
        let bucket = token_bucket_for((*subflow).token);
        spin_lock_bh(&mut (*bucket).lock);
        if __token_bucket_busy(bucket, (*subflow).token) {
            spin_unlock_bh(&mut (*bucket).lock);
            retries -= 1;
            if retries == 0 { return -EBUSY; }
            continue;
        }
        WRITE_ONCE!((*msk).token, (*subflow).token);
        __sk_nulls_add_node_rcu(msk as *mut sock, &mut (*bucket).msk_chain);
        (*bucket).chain_len += 1;
        spin_unlock_bh(&mut (*bucket).lock);
        sock_prot_inuse_add(sock_net(sk), (*sk).sk_prot, 1);
        return 0;
    }
}

pub unsafe fn mptcp_token_accept(req: *mut mptcp_subflow_request_sock, msk: *mut mptcp_sock) {
    let sk = msk as *mut sock;
    sock_prot_inuse_add(sock_net(sk), (*sk).sk_prot, 1);
    let bucket = token_bucket_for((*req).token);
    spin_lock_bh(&mut (*bucket).lock);
    let pos = __token_lookup_req(bucket, (*req).token);
    if !WARN_ON_ONCE!(pos != req) { hlist_nulls_del_init_rcu(&mut (*req).token_node); }
    __sk_nulls_add_node_rcu(msk as *mut sock, &mut (*bucket).msk_chain);
    spin_unlock_bh(&mut (*bucket).lock);
}

pub unsafe fn mptcp_token_exists(token: u32) -> bool {
    let bucket = token_bucket_for(token); rcu_read_lock();
    let mut pos = core::ptr::null_mut(); let mut sk = core::ptr::null_mut();
    sk_nulls_for_each_rcu!(sk, pos, &mut (*bucket).msk_chain);
    if !sk.is_null() && READ_ONCE!((*mptcp_sk(sk)).token) == token { rcu_read_unlock(); return true; }
    rcu_read_unlock(); false
}

pub unsafe fn mptcp_token_get_sock(net: *mut net, token: u32) -> *mut mptcp_sock {
    let bucket = token_bucket_for(token); rcu_read_lock();
    let mut pos = core::ptr::null_mut(); let mut sk = core::ptr::null_mut();
    sk_nulls_for_each_rcu!(sk, pos, &mut (*bucket).msk_chain);
    if !sk.is_null() && READ_ONCE!((*mptcp_sk(sk)).token) == token && net_eq(sock_net(sk), net) && refcount_inc_not_zero(&mut (*sk).sk_refcnt) { rcu_read_unlock(); return mptcp_sk(sk); }
    rcu_read_unlock(); core::ptr::null_mut()
}

pub unsafe fn mptcp_token_iter_next(net: *const net, s_slot: *mut i64, s_num: *mut i64) -> *mut mptcp_sock {
    let mut slot = *s_slot; let mut num = 0;
    while slot <= token_mask as i64 {
        let bucket = &mut *token_hash.add(slot as usize); num = 0;
        if hlist_nulls_empty(&mut bucket.msk_chain) { slot += 1; *s_num = 0; continue; }
        rcu_read_lock(); let mut pos = core::ptr::null_mut(); let mut sk = core::ptr::null_mut();
        sk_nulls_for_each_rcu!(sk, pos, &mut bucket.msk_chain);
        num += 1;
        if !sk.is_null() && net_eq(sock_net(sk), net) && num > *s_num && refcount_inc_not_zero(&mut (*sk).sk_refcnt) { rcu_read_unlock(); *s_slot = slot; *s_num = num; return mptcp_sk(sk); }
        rcu_read_unlock(); slot += 1; *s_num = 0;
    }
    *s_slot = slot; *s_num = num; core::ptr::null_mut()
}

pub unsafe fn mptcp_token_destroy_request(req: *mut request_sock) {
    let subflow_req = mptcp_subflow_rsk(req); if hlist_nulls_unhashed(&mut (*subflow_req).token_node) { return; }
    let bucket = token_bucket_for((*subflow_req).token); spin_lock_bh(&mut (*bucket).lock);
    let pos = __token_lookup_req(bucket, (*subflow_req).token);
    if !WARN_ON_ONCE!(pos != subflow_req) { hlist_nulls_del_init_rcu(&mut (*pos).token_node); (*bucket).chain_len -= 1; }
    spin_unlock_bh(&mut (*bucket).lock);
}

pub unsafe fn mptcp_token_destroy(msk: *mut mptcp_sock) {
    let sk = msk as *mut sock; if sk_unhashed(sk) { return; }
    sock_prot_inuse_add(sock_net(sk), (*sk).sk_prot, -1); let bucket = token_bucket_for((*msk).token); spin_lock_bh(&mut (*bucket).lock);
    let pos = __token_lookup_msk(bucket, (*msk).token);
    if !WARN_ON_ONCE!(pos != msk) { __sk_nulls_del_node_init_rcu(pos as *mut sock); (*bucket).chain_len -= 1; }
    spin_unlock_bh(&mut (*bucket).lock); WRITE_ONCE!((*msk).token, 0);
}

pub unsafe fn mptcp_token_init() {
    token_hash = alloc_large_system_hash("MPTCP token", core::mem::size_of::<token_bucket>(), 0, 20, HASH_ZERO, core::ptr::null_mut(), &mut token_mask, 0, 64 * 1024);
    for i in 0..=token_mask { INIT_HLIST_NULLS_HEAD!(&mut (*token_hash.add(i as usize)).req_chain, i); INIT_HLIST_NULLS_HEAD!(&mut (*token_hash.add(i as usize)).msk_chain, i); spin_lock_init(&mut (*token_hash.add(i as usize)).lock); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
