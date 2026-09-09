// SPDX-License-Identifier: GPL-2.0-or-later
/* RxRPC remote transport endpoint record management
 *
 * Copyright (C) 2007, 2016 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// C includes and kernel-provided declarations are supplied by other Rust units.

static RXRPC_NULL_ADDR: sockaddr_rxrpc = unsafe { core::mem::zeroed() };

/* Hash a peer key. */
unsafe fn rxrpc_peer_hash_key(local: *mut rxrpc_local, srx: *const sockaddr_rxrpc) -> c_ulong {
    let mut hash_key = local as c_ulong / core::mem::align_of::<rxrpc_local>() as c_ulong;
    hash_key = hash_key.wrapping_add((*srx).transport_type as c_ulong);
    hash_key = hash_key.wrapping_add((*srx).transport_len as c_ulong);
    hash_key = hash_key.wrapping_add((*srx).transport.family as c_ulong);

    match (*srx).transport.family {
        AF_INET => {
            hash_key = hash_key.wrapping_add((*srx).transport.sin.sin_port as c_ulong);
            let p = &(*srx).transport.sin.sin_addr as *const _ as *const u16;
            let size = core::mem::size_of_val(&(*srx).transport.sin.sin_addr);
            for i in (0..size).step_by(core::mem::size_of::<u16>()) {
                hash_key = hash_key.wrapping_add(*p.add(i / core::mem::size_of::<u16>()) as c_ulong);
            }
        }
        #[cfg(CONFIG_AF_RXRPC_IPV6)]
        AF_INET6 => {
            hash_key = hash_key.wrapping_add((*srx).transport.sin6.sin6_port as c_ulong);
            let p = &(*srx).transport.sin6.sin6_addr as *const _ as *const u16;
            let size = core::mem::size_of_val(&(*srx).transport.sin6.sin6_addr);
            for i in (0..size).step_by(core::mem::size_of::<u16>()) {
                hash_key = hash_key.wrapping_add(*p.add(i / core::mem::size_of::<u16>()) as c_ulong);
            }
        }
        _ => { WARN!(1, "AF_RXRPC: Unsupported transport address family\n"); return 0; }
    }
    hash_key
}

unsafe fn rxrpc_peer_cmp_key(peer: *const rxrpc_peer, local: *mut rxrpc_local,
                             srx: *const sockaddr_rxrpc, hash_key: c_ulong) -> c_long {
    let diff = ((*peer).hash_key as c_long - hash_key as c_long)
        .then_some(0).unwrap_or_else(|| ( (*peer).local as usize as c_long - local as usize as c_long)) ;
    let diff = if diff != 0 { diff } else { (*peer).srx.transport_type as c_long - (*srx).transport_type as c_long };
    let diff = if diff != 0 { diff } else { (*peer).srx.transport_len as c_long - (*srx).transport_len as c_long };
    let diff = if diff != 0 { diff } else { (*peer).srx.transport.family as c_long - (*srx).transport.family as c_long };
    if diff != 0 { return diff; }
    match (*srx).transport.family {
        AF_INET => {
            let d = (*peer).srx.transport.sin.sin_port as c_long - (*srx).transport.sin.sin_port as c_long;
            if d != 0 { d } else { libc::memcmp(&(*peer).srx.transport.sin.sin_addr as *const _ as *const _, &(*srx).transport.sin.sin_addr as *const _ as *const _, core::mem::size_of::<in_addr>()) as c_long }
        }
        #[cfg(CONFIG_AF_RXRPC_IPV6)]
        AF_INET6 => {
            let d = (*peer).srx.transport.sin6.sin6_port as c_long - (*srx).transport.sin6.sin6_port as c_long;
            if d != 0 { d } else { libc::memcmp(&(*peer).srx.transport.sin6.sin6_addr as *const _ as *const _, &(*srx).transport.sin6.sin6_addr as *const _ as *const _, core::mem::size_of::<in6_addr>()) as c_long }
        }
        _ => { BUG!(); 0 }
    }
}

// RCU/hash-list operations and kernel allocator/refcount primitives below are
// expressed through the corresponding dependencies supplied by the build.
unsafe fn __rxrpc_lookup_peer_rcu(local: *mut rxrpc_local, srx: *const sockaddr_rxrpc,
                                  hash_key: c_ulong) -> *mut rxrpc_peer {
    let rxnet = (*local).rxnet;
    hash_for_each_possible_rcu!((*rxnet).peer_hash, peer, hash_link, hash_key, {
        if rxrpc_peer_cmp_key(peer, local, srx, hash_key) == 0 && refcount_read!(&(*peer).ref_) > 0 { return peer; }
    });
    core::ptr::null_mut()
}

pub unsafe fn rxrpc_lookup_peer_rcu(local: *mut rxrpc_local, srx: *const sockaddr_rxrpc) -> *mut rxrpc_peer {
    __rxrpc_lookup_peer_rcu(local, srx, rxrpc_peer_hash_key(local, srx))
}

pub unsafe fn rxrpc_assess_MTU_size(local: *mut rxrpc_local, peer: *mut rxrpc_peer) {
    (*peer).if_mtu = 1500;
    if (*peer).max_data < (*peer).if_mtu - (*peer).hdrsize { (*peer).max_data = (*peer).if_mtu - (*peer).hdrsize; }
    let net = (*local).net;
    let mut fl: flowi = core::mem::zeroed();
    let dst = match (*peer).srx.transport.family {
        AF_INET => ip_route_output_ports(net, &mut fl.u.ip4, core::ptr::null_mut(), (*peer).srx.transport.sin.sin_addr.s_addr, 0, htons(7000), htons(7001), IPPROTO_UDP, 0, 0),
        #[cfg(CONFIG_AF_RXRPC_IPV6)]
        AF_INET6 => { fl.u.ip6.flowi6_iif = LOOPBACK_IFINDEX; fl.u.ip6.flowi6_scope = RT_SCOPE_UNIVERSE; fl.u.ip6.flowi6_proto = IPPROTO_UDP; ip6_route_output(net, core::ptr::null_mut(), &mut fl.u.ip6) }
        _ => { BUG!(); return; }
    };
    if IS_ERR!(dst) { return; }
    (*peer).if_mtu = dst_mtu(dst);
    (*peer).hdrsize += (*dst).header_len + (*dst).trailer_len;
    (*peer).tx_seg_max = (*dst).dev.gso_max_segs;
    dst_release(dst);
    (*peer).max_data = umin!(RXRPC_JUMBO!(1), (*peer).if_mtu - (*peer).hdrsize);
    (*peer).pmtud_good = 500;
    (*peer).pmtud_bad = (*peer).if_mtu - (*peer).hdrsize + 1;
    (*peer).pmtud_trial = umin!((*peer).max_data, (*peer).pmtud_bad - 1);
    (*peer).pmtud_pending = true;
}

pub unsafe fn rxrpc_alloc_peer(local: *mut rxrpc_local, gfp: gfp_t, why: rxrpc_peer_trace) -> *mut rxrpc_peer {
    let peer = kzalloc_obj!(rxrpc_peer, gfp);
    if !peer.is_null() { refcount_set!(&mut (*peer).ref_, 1); (*peer).local = rxrpc_get_local(local, rxrpc_local_get_peer); INIT_HLIST_HEAD!(&mut (*peer).error_targets); (*peer).service_conns = RB_ROOT!(); seqlock_init!(&mut (*peer).service_conn_lock); spin_lock_init!(&mut (*peer).lock); (*peer).debug_id = atomic_inc_return!(&rxrpc_debug_id); (*peer).recent_srtt_us = UINT_MAX; (*peer).cong_ssthresh = RXRPC_TX_MAX_WINDOW; trace_rxrpc_peer!((*peer).debug_id, 1, why); }
    peer
}

unsafe fn rxrpc_init_peer(_local: *mut rxrpc_local, peer: *mut rxrpc_peer, hash_key: c_ulong) {
    (*peer).hash_key = hash_key;
    (*peer).hdrsize = match (*peer).srx.transport.family { AF_INET => core::mem::size_of::<iphdr>(), #[cfg(CONFIG_AF_RXRPC_IPV6)] AF_INET6 => core::mem::size_of::<ipv6hdr>(), _ => { BUG!(); 0 } };
    if (*peer).srx.transport_type == SOCK_DGRAM { (*peer).hdrsize += core::mem::size_of::<udphdr>(); } else { BUG!(); }
    (*peer).hdrsize += core::mem::size_of::<rxrpc_wire_header>();
    (*peer).max_data = (*peer).if_mtu - (*peer).hdrsize;
}

unsafe fn rxrpc_create_peer(local: *mut rxrpc_local, srx: *mut sockaddr_rxrpc, hash_key: c_ulong, gfp: gfp_t) -> *mut rxrpc_peer {
    let peer = rxrpc_alloc_peer(local, gfp, rxrpc_peer_new_client);
    if !peer.is_null() { core::ptr::copy_nonoverlapping(srx, &mut (*peer).srx, 1); rxrpc_init_peer(local, peer, hash_key); rxrpc_assess_MTU_size(local, peer); }
    peer
}

unsafe fn rxrpc_free_peer(peer: *mut rxrpc_peer) { trace_rxrpc_peer!((*peer).debug_id, 0, rxrpc_peer_free); rxrpc_put_local((*peer).local, rxrpc_local_put_peer); kfree_rcu!(peer, rcu); }

pub unsafe fn rxrpc_new_incoming_peer(local: *mut rxrpc_local, peer: *mut rxrpc_peer) { let rxnet = (*local).rxnet; let key = rxrpc_peer_hash_key(local, &(*peer).srx); rxrpc_init_peer(local, peer, key); spin_lock!(&mut (*rxnet).peer_hash_lock); hash_add_rcu!((*rxnet).peer_hash, &mut (*peer).hash_link, key); list_add_tail!(&mut (*peer).keepalive_link, &mut (*rxnet).peer_keepalive_new); spin_unlock!(&mut (*rxnet).peer_hash_lock); }

pub unsafe fn rxrpc_lookup_peer(local: *mut rxrpc_local, srx: *mut sockaddr_rxrpc, gfp: gfp_t) -> *mut rxrpc_peer {
    let key = rxrpc_peer_hash_key(local, srx); let rxnet = (*local).rxnet; rcu_read_lock!(); let mut peer = __rxrpc_lookup_peer_rcu(local, srx, key); if !peer.is_null() && rxrpc_get_peer_maybe(peer, rxrpc_peer_get_lookup_client).is_null() { peer = core::ptr::null_mut(); } rcu_read_unlock!();
    if peer.is_null() { let candidate = rxrpc_create_peer(local, srx, key, gfp); if candidate.is_null() { return core::ptr::null_mut(); } spin_lock_bh!(&mut (*rxnet).peer_hash_lock); peer = __rxrpc_lookup_peer_rcu(local, srx, key); if !peer.is_null() && rxrpc_get_peer_maybe(peer, rxrpc_peer_get_lookup_client).is_null() { peer = core::ptr::null_mut(); } if peer.is_null() { hash_add_rcu!((*rxnet).peer_hash, &mut (*candidate).hash_link, key); list_add_tail!(&mut (*candidate).keepalive_link, &mut (*rxnet).peer_keepalive_new); peer = candidate; } else { rxrpc_free_peer(candidate); } spin_unlock_bh!(&mut (*rxnet).peer_hash_lock); } peer
}

pub unsafe fn rxrpc_get_peer(peer: *mut rxrpc_peer, why: rxrpc_peer_trace) -> *mut rxrpc_peer { let r = __refcount_inc!(&mut (*peer).ref_); trace_rxrpc_peer!((*peer).debug_id, r + 1, why); peer }
pub unsafe fn rxrpc_get_peer_maybe(mut peer: *mut rxrpc_peer, why: rxrpc_peer_trace) -> *mut rxrpc_peer { if !peer.is_null() && !__refcount_inc_not_zero!(&mut (*peer).ref_) { peer = core::ptr::null_mut(); } else if !peer.is_null() { trace_rxrpc_peer!((*peer).debug_id, refcount_read!(&(*peer).ref_), why); } peer }
unsafe fn __rxrpc_put_peer(peer: *mut rxrpc_peer) { let rxnet = (*peer).local.rxnet; ASSERT!(hlist_empty!(&(*peer).error_targets)); spin_lock_bh!(&mut (*rxnet).peer_hash_lock); hash_del_rcu!(&mut (*peer).hash_link); list_del_init!(&mut (*peer).keepalive_link); spin_unlock_bh!(&mut (*rxnet).peer_hash_lock); rxrpc_free_peer(peer); }
pub unsafe fn rxrpc_put_peer(peer: *mut rxrpc_peer, why: rxrpc_peer_trace) { if !peer.is_null() && __refcount_dec_and_test!(&mut (*peer).ref_) { trace_rxrpc_peer!((*peer).debug_id, 0, why); __rxrpc_put_peer(peer); } }
pub unsafe fn rxrpc_destroy_all_peers(rxnet: *mut rxrpc_net) { for i in 0..HASH_SIZE!((*rxnet).peer_hash) { if !hlist_empty!(&(*rxnet).peer_hash[i]) { hlist_for_each_entry!((*rxnet).peer_hash[i], peer, hash_link, { pr_err!("Leaked peer %x {%u} %pISp\n", (*peer).debug_id, refcount_read!(&(*peer).ref_), &(*peer).srx.transport); }); } } }

pub unsafe fn rxrpc_kernel_get_call_peer(_sock: *mut socket, call: *mut rxrpc_call) -> *mut rxrpc_peer { rxrpc_get_peer((*call).peer, rxrpc_peer_get_application) }
pub unsafe fn rxrpc_kernel_get_srtt(peer: *const rxrpc_peer) -> c_uint { READ_ONCE!((*peer).recent_srtt_us) }
pub unsafe fn rxrpc_kernel_remote_srx(peer: *const rxrpc_peer) -> *const sockaddr_rxrpc { if peer.is_null() { &RXRPC_NULL_ADDR } else { &(*peer).srx } }
pub unsafe fn rxrpc_kernel_remote_addr(peer: *const rxrpc_peer) -> *const sockaddr { if peer.is_null() { &RXRPC_NULL_ADDR.transport as *const _ as *const sockaddr } else { &(*peer).srx.transport as *const _ as *const sockaddr } }
pub unsafe fn rxrpc_kernel_set_peer_data(peer: *mut rxrpc_peer, app_data: c_ulong) -> c_ulong { xchg!(&mut (*peer).app_data, app_data) }
pub unsafe fn rxrpc_kernel_get_peer_data(peer: *const rxrpc_peer) -> c_ulong { (*peer).app_data }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
