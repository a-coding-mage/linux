// SPDX-License-Identifier: GPL-2.0-or-later
/* SCTP kernel implementation
 * Copyright (c) 1999-2000 Cisco, Inc.
 * Copyright (c) 1999-2001 Motorola, Inc.
 * Copyright (c) 2001-2002 International Business Machines, Corp.
 * Copyright (c) 2001 Intel Corp.
 * Copyright (c) 2001 Nokia, Inc.
 * Copyright (c) 2001 La Monte H.P. Yarroll
 *
 * This file is part of the SCTP kernel implementation
 *
 * This abstraction represents an SCTP endpoint.
 */

// Dependencies supplied by the surrounding SCTP/kernel translation.

unsafe extern "C" {
    fn get_random_bytes(buf: *mut u8, len: usize);
    fn hmac_sha256_preparekey(key: *mut hmac_sha256_key, raw_key: *const u8, len: usize);
    fn memzero_explicit(ptr: *mut core::ffi::c_void, len: usize);
}

static unsafe fn gen_cookie_auth_key(key: *mut hmac_sha256_key) {
    let mut raw_key: [u8; SCTP_COOKIE_KEY_SIZE] = [0; SCTP_COOKIE_KEY_SIZE];
    get_random_bytes(raw_key.as_mut_ptr(), core::mem::size_of_val(&raw_key));
    hmac_sha256_preparekey(key, raw_key.as_ptr(), core::mem::size_of_val(&raw_key));
    memzero_explicit(raw_key.as_mut_ptr() as *mut core::ffi::c_void,
                     core::mem::size_of_val(&raw_key));
}

/* Initialize the base fields of the endpoint structure. */
static unsafe fn sctp_endpoint_init(ep: *mut sctp_endpoint, sk: *mut sock, gfp: gfp_t)
    -> *mut sctp_endpoint
{
    let net: *mut net = sock_net(sk);
    let mut null_key: *mut sctp_shared_key;

    (*ep).asconf_enable = (*net).sctp.addip_enable;
    (*ep).auth_enable = (*net).sctp.auth_enable;
    if (*ep).auth_enable {
        if sctp_auth_init(ep, gfp) != 0 { goto_nomem!(); }
        if (*ep).asconf_enable {
            sctp_auth_ep_add_chunkid(ep, SCTP_CID_ASCONF);
            sctp_auth_ep_add_chunkid(ep, SCTP_CID_ASCONF_ACK);
        }
    }

    (*ep).base.type_ = SCTP_EP_TYPE_SOCKET;
    refcount_set(&mut (*ep).base.refcnt, 1);
    (*ep).base.dead = false;
    sctp_inq_init(&mut (*ep).base.inqueue);
    sctp_inq_set_th_handler(&mut (*ep).base.inqueue, sctp_endpoint_bh_rcv);
    sctp_bind_addr_init(&mut (*ep).base.bind_addr, 0);
    INIT_LIST_HEAD(&mut (*ep).asocs);
    (*ep).sndbuf_policy = (*net).sctp.sndbuf_policy;
    (*sk).sk_data_ready = Some(sctp_data_ready);
    (*sk).sk_write_space = Some(sctp_write_space);
    sock_set_flag(sk, SOCK_USE_WRITE_QUEUE);
    (*ep).rcvbuf_policy = (*net).sctp.rcvbuf_policy;
    gen_cookie_auth_key(&mut (*ep).cookie_auth_key);
    INIT_LIST_HEAD(&mut (*ep).endpoint_shared_keys);
    null_key = sctp_auth_shkey_create(0, gfp);
    if null_key.is_null() { goto_nomem_shkey!(); }
    list_add(&mut (*null_key).key_list, &mut (*ep).endpoint_shared_keys);
    (*ep).prsctp_enable = (*net).sctp.prsctp_enable;
    (*ep).reconf_enable = (*net).sctp.reconf_enable;
    (*ep).ecn_enable = (*net).sctp.ecn_enable;
    (*ep).base.sk = sk;
    (*ep).base.net = sock_net(sk);
    sock_hold((*ep).base.sk);
    return ep;

    goto_nomem_shkey!();
    sctp_auth_free(ep);
    goto_nomem!();
    core::ptr::null_mut()
}

/* Create a sctp_endpoint with all that boring stuff initialized. */
pub unsafe fn sctp_endpoint_new(sk: *mut sock, gfp: gfp_t) -> *mut sctp_endpoint {
    let ep = kzalloc_obj::<sctp_endpoint>(gfp);
    if ep.is_null() { return core::ptr::null_mut(); }
    if sctp_endpoint_init(ep, sk, gfp).is_null() {
        kfree(ep);
        return core::ptr::null_mut();
    }
    SCTP_DBG_OBJCNT_INC(ep);
    ep
}

/* Add an association to an endpoint. */
pub unsafe fn sctp_endpoint_add_asoc(ep: *mut sctp_endpoint, asoc: *mut sctp_association) {
    let sk = (*ep).base.sk;
    if (*asoc).temp { return; }
    list_add_tail(&mut (*asoc).asocs, &mut (*ep).asocs);
    if sctp_style(sk, TCP) && sctp_sstate(sk, LISTENING) { sk_acceptq_added(sk); }
}

/* Free the endpoint structure. */
pub unsafe fn sctp_endpoint_free(ep: *mut sctp_endpoint) {
    (*ep).base.dead = true;
    inet_sk_set_state((*ep).base.sk, SCTP_SS_CLOSED);
    sctp_unhash_endpoint(ep);
    sctp_endpoint_put(ep);
}

static unsafe fn sctp_endpoint_destroy_rcu(head: *mut rcu_head) {
    let ep = container_of!(head, sctp_endpoint, rcu);
    let sk = (*ep).base.sk;
    (*sctp_sk(sk)).ep = core::ptr::null_mut();
    sock_put(sk);
    kfree(ep);
    SCTP_DBG_OBJCNT_DEC(ep);
}

static unsafe fn sctp_endpoint_destroy(ep: *mut sctp_endpoint) {
    if !(*ep).base.dead {
        WARN(1, "Attempt to destroy undead endpoint %p!\n", ep);
        return;
    }
    sctp_auth_destroy_keys(&mut (*ep).endpoint_shared_keys);
    sctp_auth_free(ep);
    sctp_inq_free(&mut (*ep).base.inqueue);
    sctp_bind_addr_free(&mut (*ep).base.bind_addr);
    memzero_explicit(&mut (*ep).cookie_auth_key as *mut _ as *mut core::ffi::c_void,
                     core::mem::size_of_val(&(*ep).cookie_auth_key));
    let sk = (*ep).base.sk;
    if !(*sctp_sk(sk)).bind_hash.is_null() { sctp_put_port(sk); }
    call_rcu(&mut (*ep).rcu, sctp_endpoint_destroy_rcu);
}

pub unsafe fn sctp_endpoint_hold(ep: *mut sctp_endpoint) -> i32 {
    refcount_inc_not_zero(&mut (*ep).base.refcnt)
}

pub unsafe fn sctp_endpoint_put(ep: *mut sctp_endpoint) {
    if refcount_dec_and_test(&mut (*ep).base.refcnt) != 0 { sctp_endpoint_destroy(ep); }
}

pub unsafe fn sctp_endpoint_is_match(ep: *mut sctp_endpoint, net: *mut net,
    laddr: *const sctp_addr_union, dif: i32, sdif: i32) -> *mut sctp_endpoint {
    let bound_dev_if = READ_ONCE!((*ep).base.sk, sk_bound_dev_if);
    if net_eq((*ep).base.net, net) && sctp_sk_bound_dev_eq(net, bound_dev_if, dif, sdif)
        && htons((*ep).base.bind_addr.port) == (*laddr).v4.sin_port
        && sctp_bind_addr_match(&mut (*ep).base.bind_addr, laddr, sctp_sk((*ep).base.sk)) { ep }
    else { core::ptr::null_mut() }
}

pub unsafe fn sctp_endpoint_lookup_assoc(ep: *const sctp_endpoint, paddr: *const sctp_addr_union,
    transport: *mut *mut sctp_transport) -> *mut sctp_association {
    *transport = core::ptr::null_mut();
    if (*ep).base.bind_addr.port == 0 { return core::ptr::null_mut(); }
    rcu_read_lock();
    let t = sctp_epaddr_lookup_transport(ep, paddr);
    let asoc = if t.is_null() { core::ptr::null_mut() } else { *transport = t; (*t).asoc };
    rcu_read_unlock();
    asoc
}

pub unsafe fn sctp_endpoint_is_peeled_off(ep: *mut sctp_endpoint, paddr: *const sctp_addr_union) -> bool {
    let bound_dev_if = READ_ONCE!((*ep).base.sk, sk_bound_dev_if);
    let bp = &mut (*ep).base.bind_addr;
    let mut pos = (*bp).address_list.next;
    while pos != &mut (*bp).address_list as *mut _ {
        let addr = list_entry!(pos, sctp_sockaddr_entry, list);
        if sctp_has_association((*ep).base.net, &(*addr).a, paddr, bound_dev_if, bound_dev_if) { return true; }
        pos = (*pos).next;
    }
    false
}

static unsafe fn sctp_endpoint_bh_rcv(work: *mut work_struct) {
    let ep = container_of!(work, sctp_endpoint, base.inqueue.immediate);
    if (*ep).base.dead { return; }
    let mut asoc: *mut sctp_association = core::ptr::null_mut();
    let inqueue = &mut (*ep).base.inqueue;
    let sk = (*ep).base.sk;
    let net = sock_net(sk);
    let mut transport: *mut sctp_transport = core::ptr::null_mut();
    let mut first_time = 1;
    while let Some(chunk) = sctp_inq_pop(inqueue) {
        let subtype = SCTP_ST_CHUNK((*chunk).chunk_hdr.type_);
        if first_time != 0 && subtype.chunk == SCTP_CID_AUTH {
            let next_hdr = sctp_inq_peek(inqueue);
            if next_hdr.is_null() { goto_normal!(); }
            if (*next_hdr).type_ == SCTP_CID_COOKIE_ECHO {
                (*chunk).auth_chunk = skb_clone((*chunk).skb, GFP_ATOMIC);
                if (*chunk).auth_chunk.is_null() { (*chunk).pdiscard = 1; continue; }
                (*chunk).auth = 1;
                continue;
            }
        }
        goto_normal!();
        if (*chunk).asoc.is_null() {
            asoc = sctp_endpoint_lookup_assoc(ep, sctp_source(chunk), &mut transport);
            (*chunk).asoc = asoc;
            (*chunk).transport = transport;
        }
        let state = if !asoc.is_null() { (*asoc).state } else { SCTP_STATE_CLOSED };
        if sctp_auth_recv_cid(subtype.chunk, asoc) && (*chunk).auth == 0 { continue; }
        if !asoc.is_null() && sctp_chunk_is_data(chunk) { (*asoc).peer.last_data_from = (*chunk).transport; }
        else { SCTP_INC_STATS((*ep).base.net, SCTP_MIB_INCTRLCHUNKS); if !asoc.is_null() { (*asoc).stats.ictrlchunks += 1; } }
        if !(*chunk).transport.is_null() { (*(*chunk).transport).last_time_heard = ktime_get(); }
        let error = sctp_do_sm(net, SCTP_EVENT_T_CHUNK, subtype, state, ep, asoc, chunk, GFP_ATOMIC);
        if error != 0 { (*chunk).pdiscard = 1; }
        if (*sctp_sk(sk)).ep.is_null() { break; }
        if first_time != 0 { first_time = 0; }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
