// SPDX-License-Identifier: GPL-2.0-only
/*
 * Handshake request lifetime events
 *
 * Author: Chuck Lever <chuck.lever@oracle.com>
 *
 * Copyright (c) 2023, Oracle and/or its affiliates.
 */

// Dependencies are supplied by the surrounding kernel translation.

static mut HANDSHAKE_RHASHTBL: rhashtable = rhashtable::default();

static HANDSHAKE_RHASH_PARAMS: rhashtable_params = rhashtable_params {
    key_len: core::mem::size_of::<*mut sock>(),
    key_offset: core::mem::offset_of!(handshake_req, hr_sk),
    head_offset: core::mem::offset_of!(handshake_req, hr_rhash),
    automatic_shrinking: true,
};

pub unsafe fn handshake_req_hash_init() -> i32 {
    rhashtable_init(&raw mut HANDSHAKE_RHASHTBL, &HANDSHAKE_RHASH_PARAMS)
}

pub unsafe fn handshake_req_hash_destroy() {
    rhashtable_destroy(&raw mut HANDSHAKE_RHASHTBL);
}

pub unsafe fn handshake_req_hash_lookup(sk: *mut sock) -> *mut handshake_req {
    rhashtable_lookup_fast(
        &raw mut HANDSHAKE_RHASHTBL,
        &sk as *const *mut sock,
        HANDSHAKE_RHASH_PARAMS,
    )
}

unsafe fn handshake_req_hash_add(req: *mut handshake_req) -> bool {
    let ret = rhashtable_lookup_insert_fast(
        &raw mut HANDSHAKE_RHASHTBL,
        &mut (*req).hr_rhash,
        HANDSHAKE_RHASH_PARAMS,
    );
    ret == 0
}

unsafe fn handshake_req_destroy(req: *mut handshake_req) {
    if let Some(destroy) = (*(*req).hr_proto).hp_destroy {
        destroy(req);
    }
    rhashtable_remove_fast(
        &raw mut HANDSHAKE_RHASHTBL,
        &mut (*req).hr_rhash,
        HANDSHAKE_RHASH_PARAMS,
    );
    kfree(req);
}

unsafe fn handshake_sk_destruct(sk: *mut sock) {
    let mut sk_destruct: Option<unsafe extern "C" fn(*mut sock)> = None;
    let req = handshake_req_hash_lookup(sk);
    if req.is_null() {
        return;
    }

    trace_handshake_destruct(sock_net(sk), req, sk);
    sk_destruct = (*req).hr_odestruct;
    handshake_req_destroy(req);
    if let Some(destruct) = sk_destruct {
        destruct(sk);
    }
}

pub unsafe fn handshake_req_alloc(
    proto: *const handshake_proto,
    flags: gfp_t,
) -> *mut handshake_req {
    if proto.is_null()
        || (*proto).hp_handler_class <= HANDSHAKE_HANDLER_CLASS_NONE
        || (*proto).hp_handler_class >= HANDSHAKE_HANDLER_CLASS_MAX
        || (*proto).hp_accept.is_none()
        || (*proto).hp_done.is_none()
    {
        return core::ptr::null_mut();
    }

    let req = kzalloc_flex::<handshake_req>(
        core::mem::size_of::<handshake_req>() + (*proto).hp_privsize,
        flags,
    );
    if req.is_null() {
        return core::ptr::null_mut();
    }

    INIT_LIST_HEAD(&mut (*req).hr_list);
    (*req).hr_proto = proto;
    req
}

pub unsafe fn handshake_req_private(req: *mut handshake_req) -> *mut core::ffi::c_void {
    &mut (*req).hr_priv as *mut _ as *mut core::ffi::c_void
}

unsafe fn __add_pending_locked(hn: *mut handshake_net, req: *mut handshake_req) -> bool {
    if WARN_ON_ONCE(!list_empty(&(*req).hr_list)) {
        return false;
    }
    (*hn).hn_pending += 1;
    list_add_tail(&mut (*req).hr_list, &mut (*hn).hn_requests);
    true
}

unsafe fn __remove_pending_locked(hn: *mut handshake_net, req: *mut handshake_req) {
    (*hn).hn_pending -= 1;
    list_del_init(&mut (*req).hr_list);
}

unsafe fn remove_pending(hn: *mut handshake_net, req: *mut handshake_req) -> bool {
    let mut ret = false;
    spin_lock_bh(&mut (*hn).hn_lock);
    if !test_bit(HANDSHAKE_F_NET_DRAINING, &(*hn).hn_flags)
        && !list_empty(&(*req).hr_list)
    {
        __remove_pending_locked(hn, req);
        ret = true;
    }
    spin_unlock_bh(&mut (*hn).hn_lock);
    ret
}

pub unsafe fn handshake_req_next(hn: *mut handshake_net, class: i32) -> *mut handshake_req {
    let mut req = core::ptr::null_mut();
    spin_lock_bh(&mut (*hn).hn_lock);
    let mut pos = (*hn).hn_requests.next as *mut handshake_req;
    while pos != &mut (*hn).hn_requests as *mut _ as *mut handshake_req {
        if (*(*pos).hr_proto).hp_handler_class == class {
            __remove_pending_locked(hn, pos);
            get_file((*pos).hr_file);
            req = pos;
            break;
        }
        pos = (*pos).hr_list.next as *mut handshake_req;
    }
    spin_unlock_bh(&mut (*hn).hn_lock);
    req
}

pub unsafe fn handshake_req_submit(
    sock: *mut socket,
    req: *mut handshake_req,
    flags: gfp_t,
) -> i32 {
    if sock.is_null() || req.is_null() || (*sock).file.is_null() {
        kfree(req);
        return -EINVAL;
    }
    (*req).hr_sk = (*sock).sk;
    if (*req).hr_sk.is_null() {
        kfree(req);
        return -EINVAL;
    }
    (*req).hr_file = get_file((*sock).file);
    (*req).hr_odestruct = (*(*req).hr_sk).sk_destruct;
    (*(*req).hr_sk).sk_destruct = Some(handshake_sk_destruct);

    let net = sock_net((*req).hr_sk);
    let hn = handshake_pernet(net);
    let mut ret = -EOPNOTSUPP;
    if hn.is_null() { return submit_error(req, ret); }
    ret = -EAGAIN;
    if READ_ONCE((*hn).hn_pending) >= (*hn).hn_pending_max { return submit_error(req, ret); }
    spin_lock_bh(&mut (*hn).hn_lock);
    ret = -EOPNOTSUPP;
    if test_bit(HANDSHAKE_F_NET_DRAINING, &(*hn).hn_flags) { spin_unlock_bh(&mut (*hn).hn_lock); return submit_error(req, ret); }
    ret = -EBUSY;
    if !handshake_req_hash_add(req) || !__add_pending_locked(hn, req) { spin_unlock_bh(&mut (*hn).hn_lock); return submit_error(req, ret); }
    spin_unlock_bh(&mut (*hn).hn_lock);
    ret = handshake_genl_notify(net, (*req).hr_proto, flags);
    if ret != 0 {
        trace_handshake_notify_err(net, req, (*req).hr_sk, ret);
        if remove_pending(hn, req) { return submit_error(req, ret); }
    }
    trace_handshake_submit(net, req, (*req).hr_sk);
    0
}

unsafe fn submit_error(req: *mut handshake_req, ret: i32) -> i32 {
    trace_handshake_submit_err(sock_net((*req).hr_sk), req, (*req).hr_sk, ret);
    if !test_and_set_bit(HANDSHAKE_F_REQ_COMPLETED, &mut (*req).hr_flags) {
        (*(*req).hr_sk).sk_destruct = (*req).hr_odestruct;
        fput((*req).hr_file);
        handshake_req_destroy(req);
    }
    ret
}

pub unsafe fn handshake_complete(req: *mut handshake_req, status: i32, info: *mut genl_info) {
    let sk = (*req).hr_sk;
    let net = sock_net(sk);
    if !test_and_set_bit(HANDSHAKE_F_REQ_COMPLETED, &mut (*req).hr_flags) {
        let file = (*req).hr_file;
        trace_handshake_complete(net, req, sk, status);
        ((*(*req).hr_proto).hp_done.unwrap())(req, status, info);
        fput(file);
    }
}

pub unsafe fn handshake_req_cancel(sk: *mut sock) -> bool {
    let net = sock_net(sk);
    let req = handshake_req_hash_lookup(sk);
    if req.is_null() { trace_handshake_cancel_none(net, req, sk); return false; }
    let hn = handshake_pernet(net);
    if !hn.is_null() && remove_pending(hn, req) {
        if test_and_set_bit(HANDSHAKE_F_REQ_COMPLETED, &mut (*req).hr_flags) { trace_handshake_cancel_busy(net, req, sk); return false; }
    } else if test_and_set_bit(HANDSHAKE_F_REQ_COMPLETED, &mut (*req).hr_flags) {
        trace_handshake_cancel_busy(net, req, sk);
        return false;
    }
    trace_handshake_cancel(net, req, sk);
    fput((*req).hr_file);
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
