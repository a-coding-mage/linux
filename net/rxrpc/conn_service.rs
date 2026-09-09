// SPDX-License-Identifier: GPL-2.0-or-later
/* Service connection management
 *
 * Copyright (C) 2016 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Dependencies supplied by the surrounding kernel/RxRPC translation.

/*
 * Find a service connection under RCU conditions.
 *
 * We could use a hash table, but that is subject to bucket stuffing by an
 * attacker as the client gets to pick the epoch and cid values and would know
 * the hash function.  So, instead, we use a hash table for the peer and from
 * that an rbtree to find the service connection.  Under ordinary circumstances
 * it might be slower than a large hash table, but it is at least limited in
 * depth.
 */
pub unsafe extern "C" fn rxrpc_find_service_conn_rcu(
    peer: *mut rxrpc_peer,
    skb: *mut sk_buff,
) -> *mut rxrpc_connection {
    let mut conn: *mut rxrpc_connection = core::ptr::null_mut();
    let mut k: rxrpc_conn_proto = core::mem::zeroed();
    let sp: *mut rxrpc_skb_priv = rxrpc_skb(skb);
    let mut p: *mut rb_node;
    let mut seq: u32 = 1;

    (*k).epoch = (*sp).hdr.epoch;
    (*k).cid = (*sp).hdr.cid & RXRPC_CIDMASK;

    loop {
        /* Unfortunately, rbtree walking doesn't give reliable results
         * under just the RCU read lock, so we have to check for
         * changes.
         */
        seq = seq.wrapping_add(1); /* 2 on the 1st/lockless path, otherwise odd */
        read_seqbegin_or_lock(&mut (*peer).service_conn_lock, &mut seq);

        p = rcu_dereference_raw((*peer).service_conns.rb_node);
        while !p.is_null() {
            conn = rb_entry(p, rxrpc_connection, service_node);

            if (*conn).proto.index_key < (*k).index_key {
                p = rcu_dereference_raw((*p).rb_left);
            } else if (*conn).proto.index_key > (*k).index_key {
                p = rcu_dereference_raw((*p).rb_right);
            } else {
                break;
            }
            conn = core::ptr::null_mut();
        }
        if !need_seqretry(&mut (*peer).service_conn_lock, seq) {
            break;
        }
    }

    done_seqretry(&mut (*peer).service_conn_lock, seq);
    _leave(" = %d", if !conn.is_null() { (*conn).debug_id } else { -1 });
    conn
}

/*
 * Insert a service connection into a peer's tree, thereby making it a target
 * for incoming packets.
 */
unsafe fn rxrpc_publish_service_conn(peer: *mut rxrpc_peer, conn: *mut rxrpc_connection) {
    let mut cursor: *mut rxrpc_connection = core::ptr::null_mut();
    let k: rxrpc_conn_proto = (*conn).proto;
    let mut pp: *mut *mut rb_node;
    let mut parent: *mut rb_node;

    write_seqlock(&mut (*peer).service_conn_lock);

    pp = &mut (*peer).service_conns.rb_node;
    parent = core::ptr::null_mut();
    while !(*pp).is_null() {
        parent = *pp;
        cursor = rb_entry(parent, rxrpc_connection, service_node);

        if (*cursor).proto.index_key < k.index_key {
            pp = &mut (**pp).rb_left;
        } else if (*cursor).proto.index_key > k.index_key {
            pp = &mut (**pp).rb_right;
        } else {
            if refcount_read(&(*cursor).ref) == 0 {
                break;
            }
            write_sequnlock(&mut (*peer).service_conn_lock);
            /* We should not be able to get here.  rxrpc_incoming_connection() is
             * called in a non-reentrant context, so there can't be a race to
             * insert a new connection.
             */
            BUG();
        }
    }

    if !(*pp).is_null() && !cursor.is_null() && (*cursor).proto.index_key == k.index_key {
        /* The old connection is from an outdated epoch. */
        _debug("replace conn");
        rb_replace_node_rcu(&mut (*cursor).service_node,
                            &mut (*conn).service_node,
                            &mut (*peer).service_conns);
        clear_bit(RXRPC_CONN_IN_SERVICE_CONNS, &mut (*cursor).flags);
    } else {
        rb_link_node_rcu(&mut (*conn).service_node, parent, pp);
        rb_insert_color(&mut (*conn).service_node, &mut (*peer).service_conns);
    }
    set_bit(RXRPC_CONN_IN_SERVICE_CONNS, &mut (*conn).flags);
    write_sequnlock(&mut (*peer).service_conn_lock);
    _leave(" = %d [new]", (*conn).debug_id);
}

/*
 * Preallocate a service connection.  The connection is placed on the proc and
 * reap lists so that we don't have to get the lock from BH context.
 */
pub unsafe extern "C" fn rxrpc_prealloc_service_connection(
    rxnet: *mut rxrpc_net,
    gfp: gfp_t,
) -> *mut rxrpc_connection {
    let conn = rxrpc_alloc_connection(rxnet, gfp);

    if !conn.is_null() {
        /* We maintain an extra ref on the connection whilst it is on
         * the rxrpc_connections list.
         */
        (*conn).state = RXRPC_CONN_SERVICE_PREALLOC;
        refcount_set(&mut (*conn).ref_, 2);

        atomic_inc(&mut (*rxnet).nr_conns);
        write_lock(&mut (*rxnet).conn_lock);
        list_add_tail(&mut (*conn).link, &mut (*rxnet).service_conns);
        list_add_tail(&mut (*conn).proc_link, &mut (*rxnet).conn_proc_list);
        write_unlock(&mut (*rxnet).conn_lock);

        rxrpc_see_connection(conn, rxrpc_conn_new_service);
    }

    conn
}

/*
 * Set up an incoming connection.  This is called in BH context with the RCU
 * read lock held.
 */
pub unsafe extern "C" fn rxrpc_new_incoming_connection(
    rx: *mut rxrpc_sock,
    conn: *mut rxrpc_connection,
    sec: *const rxrpc_security,
    skb: *mut sk_buff,
) {
    let sp = rxrpc_skb(skb);

    _enter("");

    (*conn).proto.epoch = (*sp).hdr.epoch;
    (*conn).proto.cid = (*sp).hdr.cid & RXRPC_CIDMASK;
    (*conn).orig_service_id = (*sp).hdr.serviceId;
    (*conn).service_id = (*sp).hdr.serviceId;
    (*conn).security_ix = (*sp).hdr.securityIndex;
    (*conn).out_clientflag = 0;
    (*conn).security = sec;
    if (*conn).security_ix != 0 {
        (*conn).state = RXRPC_CONN_SERVICE_UNSECURED;
    } else {
        (*conn).state = RXRPC_CONN_SERVICE;
    }

    /* See if we should upgrade the service.  This can only happen on the
     * first packet on a new connection.  Once done, it applies to all
     * subsequent calls on that connection.
     */
    if (*sp).hdr.userStatus == RXRPC_USERSTATUS_SERVICE_UPGRADE
        && (*conn).service_id == (*rx).service_upgrade.from
    {
        (*conn).service_id = (*rx).service_upgrade.to;
    }

    atomic_set(&mut (*conn).active, 1);

    /* Make the connection a target for incoming packets. */
    rxrpc_publish_service_conn((*conn).peer, conn);
}

/*
 * Remove the service connection from the peer's tree, thereby removing it as a
 * target for incoming packets.
 */
pub unsafe extern "C" fn rxrpc_unpublish_service_conn(conn: *mut rxrpc_connection) {
    let peer = (*conn).peer;

    write_seqlock(&mut (*peer).service_conn_lock);
    if test_and_clear_bit(RXRPC_CONN_IN_SERVICE_CONNS, &mut (*conn).flags) {
        rb_erase(&mut (*conn).service_node, &mut (*peer).service_conns);
    }
    write_sequnlock(&mut (*peer).service_conn_lock);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
