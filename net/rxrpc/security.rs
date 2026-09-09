// SPDX-License-Identifier: GPL-2.0-or-later
/* RxRPC security handling
 *
 * Copyright (C) 2007 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

static mut RXRPC_SECURITY_TYPES: [*const rxrpc_security; 3] = [
    /* [RXRPC_SECURITY_NONE] */ &rxrpc_no_security,
    /* [RXRPC_SECURITY_RXKAD] (CONFIG_RXKAD) */ core::ptr::null(),
    /* [RXRPC_SECURITY_YFS_RXGK] (CONFIG_RXGK) */ core::ptr::null(),
];

pub unsafe fn rxrpc_init_security() -> i32 {
    let mut i: isize = 0;
    let mut ret: i32;

    while (i as usize) < RXRPC_SECURITY_TYPES.len() {
        if !RXRPC_SECURITY_TYPES[i as usize].is_null() {
            ret = ((*RXRPC_SECURITY_TYPES[i as usize]).init)();
            if ret < 0 {
                break;
            }
        }
        i += 1;
    }

    if (i as usize) == RXRPC_SECURITY_TYPES.len() {
        return 0;
    }

    while i >= 0 {
        if !RXRPC_SECURITY_TYPES[i as usize].is_null() {
            ((*RXRPC_SECURITY_TYPES[i as usize]).exit)();
        }
        i -= 1;
    }
    ret
}

pub unsafe fn rxrpc_exit_security() {
    let mut i = 0usize;

    while i < RXRPC_SECURITY_TYPES.len() {
        if !RXRPC_SECURITY_TYPES[i].is_null() {
            ((*RXRPC_SECURITY_TYPES[i]).exit)();
        }
        i += 1;
    }
}

/*
 * look up an rxrpc security module
 */
pub unsafe fn rxrpc_security_lookup(security_index: u8) -> *const rxrpc_security {
    if (security_index as usize) >= RXRPC_SECURITY_TYPES.len() {
        return core::ptr::null();
    }
    RXRPC_SECURITY_TYPES[security_index as usize]
}

/*
 * Initialise the security on a client call.
 */
pub unsafe fn rxrpc_init_client_call_security(call: *mut rxrpc_call) -> i32 {
    let mut sec: *const rxrpc_security = &rxrpc_no_security;
    let mut token: *mut rxrpc_key_token;
    let key = (*call).key;
    let ret: i32;

    if key.is_null() {
        (*call).security = sec;
        (*call).security_ix = (*sec).security_index;
        return 0;
    }

    ret = key_validate(key);
    if ret < 0 {
        return ret;
    }

    token = (*key).payload.data[0];
    while !token.is_null() {
        sec = rxrpc_security_lookup((*token).security_index);
        if !sec.is_null() {
            (*call).security = sec;
            (*call).security_ix = (*sec).security_index;
            return 0;
        }
        token = (*token).next;
    }
    -EKEYREJECTED
}

/*
 * initialise the security on a client connection
 */
pub unsafe fn rxrpc_init_client_conn_security(conn: *mut rxrpc_connection) -> i32 {
    let mut token: *mut rxrpc_key_token;
    let key = (*conn).key;
    let mut ret: i32 = 0;

    _enter!("{%d},{%x}", (*conn).debug_id, key_serial(key));

    token = (*key).payload.data[0];
    while !token.is_null() {
        if (*token).security_index == (*(*conn).security).security_index {
            break;
        }
        token = (*token).next;
    }
    if token.is_null() {
        return -EKEYREJECTED;
    }

    mutex_lock(&mut (*conn).security_lock);
    if (*conn).state == RXRPC_CONN_CLIENT_UNSECURED {
        ret = ((*(*conn).security).init_connection_security)(conn, token);
        if ret == 0 {
            spin_lock_irq(&mut (*conn).state_lock);
            if (*conn).state == RXRPC_CONN_CLIENT_UNSECURED {
                (*conn).state = RXRPC_CONN_CLIENT;
            }
            spin_unlock_irq(&mut (*conn).state_lock);
        }
    }
    mutex_unlock(&mut (*conn).security_lock);
    ret
}

/*
 * Set the ops a server connection.
 */
pub unsafe fn rxrpc_get_incoming_security(
    rx: *mut rxrpc_sock,
    skb: *mut sk_buff,
) -> *const rxrpc_security {
    let sp = rxrpc_skb(skb);
    _enter!("");

    let sec = rxrpc_security_lookup((*sp).hdr.securityIndex);
    if sec.is_null() {
        rxrpc_direct_conn_abort(skb, rxrpc_abort_unsupported_security,
                                RX_INVALID_OPERATION, -EKEYREJECTED);
        return core::ptr::null();
    }

    if (*sp).hdr.securityIndex != RXRPC_SECURITY_NONE && (*rx).securities.is_null() {
        rxrpc_direct_conn_abort(skb, rxrpc_abort_no_service_key,
                                (*sec).no_key_abort, -EKEYREJECTED);
        return core::ptr::null();
    }
    sec
}

/*
 * Find the security key for a server connection.
 */
pub unsafe fn rxrpc_look_up_server_security(
    conn: *mut rxrpc_connection,
    skb: *mut sk_buff,
    kvno: u32,
    enctype: u32,
) -> *mut key {
    let sp = rxrpc_skb(skb);
    let mut key: *mut key = ERR_PTR(-EKEYREJECTED);
    let mut kref: key_ref_t = core::ptr::null_mut();
    let mut kdesc = [0i8; 5 + 1 + 3 + 1 + 12 + 1 + 12 + 1];
    let ret: i32;

    _enter!("");

    if enctype != 0 {
        sprintf!(kdesc.as_mut_ptr(), "%u:%u:%u:%u", (*sp).hdr.serviceId,
                 (*sp).hdr.securityIndex, kvno, enctype);
    } else if kvno != 0 {
        sprintf!(kdesc.as_mut_ptr(), "%u:%u:%u", (*sp).hdr.serviceId,
                 (*sp).hdr.securityIndex, kvno);
    } else {
        sprintf!(kdesc.as_mut_ptr(), "%u:%u", (*sp).hdr.serviceId,
                 (*sp).hdr.securityIndex);
    }

    read_lock(&mut (*(*conn).local).services_lock);
    let rx = (*(*conn).local).service;
    if !rx.is_null() {
        kref = keyring_search(make_key_ref((*rx).securities, 1usize),
                              &key_type_rxrpc_s, kdesc.as_mut_ptr(), true);
        if IS_ERR(kref) {
            key = ERR_CAST(kref);
        } else {
            key = key_ref_to_ptr(kref);
            ret = key_validate(key);
            if ret < 0 {
                key_put(key);
                key = ERR_PTR(ret);
            }
        }
    }
    read_unlock(&mut (*(*conn).local).services_lock);
    key
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
