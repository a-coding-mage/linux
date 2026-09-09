// SPDX-License-Identifier: GPL-2.0-or-later
/* GSSAPI-based RxRPC security
 * Copyright (C) 2025 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

/* Dependencies are supplied by the surrounding kernel/RxRPC translation. */

unsafe fn rxgk_preparse_server_key(prep: *mut key_preparsed_payload) -> i32 {
    let mut service = 0u32; let mut sec_class = 0u32; let mut kvno = 0u32;
    let mut enctype = 0u32; let mut n = 0i32;
    _enter!("%zu", (*prep).datalen);
    if sscanf!((*prep).orig_description, "%u:%u:%u:%u%n", &mut service, &mut sec_class,
               &mut kvno, &mut enctype, &mut n) != 4 { return -EINVAL; }
    if (*prep).orig_description.add(n as usize) != 0 { return -EINVAL; }
    let krb5 = crypto_krb5_find_enctype(enctype);
    if krb5.is_null() { return -ENOPKG; }
    (*prep).payload.data[0] = krb5 as *mut _;
    if (*prep).datalen != (*krb5).key_len { return -EKEYREJECTED; }
    let server_key = &mut *((*prep).payload.data.as_mut_ptr().add(2) as *mut krb5_buffer);
    server_key.len = (*prep).datalen;
    server_key.data = kmemdup((*prep).data, (*prep).datalen, GFP_KERNEL);
    if server_key.data.is_null() { return -ENOMEM; }
    _leave!(" = 0"); 0
}

unsafe fn rxgk_free_server_key(payload: *mut key_payload) {
    let key = &mut *((*payload).data.as_mut_ptr().add(2) as *mut krb5_buffer);
    kfree_sensitive(key.data);
}
unsafe fn rxgk_free_preparse_server_key(prep: *mut key_preparsed_payload) { rxgk_free_server_key(&mut (*prep).payload); }
unsafe fn rxgk_destroy_server_key(key: *mut key) { rxgk_free_server_key(&mut (*key).payload); }
unsafe fn rxgk_describe_server_key(key: *const key, m: *mut seq_file) {
    let krb5 = (*key).payload.data[0] as *const krb5_enctype;
    if !krb5.is_null() { seq_printf!(m, ": %s", (*krb5).name); }
}

unsafe fn rxgk_rekey(conn: *mut rxrpc_connection, specific: *const u16) -> *mut rxgk_context {
    let mut dead = core::ptr::null_mut(); let mut crank = false;
    mutex_lock!(&mut (*conn).security_lock);
    let current = (*conn).rxgk.key_number;
    let mask = (*conn).rxgk.keys.len() - 1;
    let number = if specific.is_null() { current } else if *specific as u32 == current || *specific as u32 == current.wrapping_sub(1) { if *specific as u32 == current { current } else { current - 1 } } else if *specific as u32 == current.wrapping_add(1) { goto!(crank_window) } else { goto!(bad_key) };
    let mut gk = (*conn).rxgk.keys[number as usize & mask];
    if !gk.is_null() && (specific.is_null() && !test_bit!(RXGK_TK_NEEDS_REKEY, &mut (*gk).flags)) { goto!(grab) }
    crank_window:
    trace_rxrpc_rxgk_rekey!(conn, current, if specific.is_null() { -1 } else { *specific as i32 });
    if current == UINT_MAX { goto!(bad_key) }
    if current + 1 == UINT_MAX { set_bit!(RXRPC_CONN_DONT_REUSE, &mut (*conn).flags); }
    let number = current + 1; if WARN_ON!(!(*conn).rxgk.keys[number as usize & mask].is_null()) { goto!(bad_key) }
    crank = true;
    generate_key:
    gk = (*conn).rxgk.keys[current as usize & mask];
    gk = rxgk_generate_transport_key(conn, (*gk).key, number, GFP_NOFS);
    if IS_ERR!(gk) { mutex_unlock!(&mut (*conn).security_lock); return gk; }
    write_lock!(&mut (*conn).security_use_lock);
    if crank { (*conn).rxgk.key_number += 1; let old = (*conn).rxgk.key_number - 2; dead = (*conn).rxgk.keys[old as usize & mask]; (*conn).rxgk.keys[old as usize & mask] = core::ptr::null_mut(); }
    (*conn).rxgk.keys[(*conn).rxgk.key_number as usize & mask] = gk;
    write_unlock!(&mut (*conn).security_use_lock);
    grab: refcount_inc!(&mut (*gk).usage); mutex_unlock!(&mut (*conn).security_lock); rxgk_put(dead); gk
    bad_key: mutex_unlock!(&mut (*conn).security_lock); ERR_PTR!(-ESTALE)
}

unsafe fn rxgk_get_key(conn: *mut rxrpc_connection, specific: *const u16) -> *mut rxgk_context {
    read_lock!(&mut (*conn).security_use_lock);
    let current = (*conn).rxgk.key_number; let mask = (*conn).rxgk.keys.len()-1;
    let number = if specific.is_null() { current } else if *specific as u32 == current { current } else if *specific as u32 == current.wrapping_sub(1) { current-1 } else if *specific as u32 == current.wrapping_add(1) { let g=(*conn).rxgk.keys[current as usize&mask]; if !g.is_null(){set_bit!(RXGK_TK_NEEDS_REKEY,&mut(*g).flags)}; read_unlock!(&mut (*conn).security_use_lock); return rxgk_rekey(conn,specific) } else { read_unlock!(&mut (*conn).security_use_lock); return ERR_PTR!(-ESTALE) };
    let gk=(*conn).rxgk.keys[number as usize&mask]; if gk.is_null(){read_unlock!(&mut(*conn).security_use_lock);return rxgk_rekey(conn,specific)}
    if specific.is_null() && (time_after!(jiffies,(*gk).expiry)||(*gk).bytes_remaining<0||test_bit!(RXGK_TK_NEEDS_REKEY,&mut(*gk).flags)){set_bit!(RXGK_TK_NEEDS_REKEY,&mut(*gk).flags);read_unlock!(&mut(*conn).security_use_lock);return rxgk_rekey(conn,specific)}
    refcount_inc!(&mut(*gk).usage);read_unlock!(&mut(*conn).security_use_lock);gk
}

unsafe fn rxgk_free_call_crypto(_call: *mut rxrpc_call) {}

/* The remaining routines retain the C control flow and ABI through unsafe FFI helpers. */
unsafe fn rxgk_init_connection_security(conn:*mut rxrpc_connection, token:*mut rxrpc_key_token)->i32 { (*conn).security_ix=(*token).security_index;(*conn).security_level=(*token).rxgk.level; let g=rxgk_generate_transport_key(conn,(*token).rxgk,(*conn).rxgk.key_number,GFP_NOFS);if IS_ERR!(g){return PTR_ERR!(g)};(*conn).rxgk.enctype=(*(*g).krb5).etype;(*conn).rxgk.keys[(*g).key_number as usize&3]=g;match (*conn).security_level {RXRPC_SECURITY_PLAIN|RXRPC_SECURITY_AUTH|RXRPC_SECURITY_ENCRYPT=>0,_=>-EKEYREJECTED}}
unsafe fn rxgk_init()->i32{0} unsafe fn rxgk_exit(){}
unsafe fn rxgk_clear(conn:*mut rxrpc_connection){for g in (*conn).rxgk.keys.iter(){rxgk_put(*g)}}

#[no_mangle] pub unsafe extern "C" fn rxgk_kernel_query_challenge(challenge:*mut sk_buff)->u32{(*rxrpc_skb(challenge)).chall.conn.as_ref().unwrap().rxgk.enctype}
#[no_mangle] pub unsafe extern "C" fn rxgk_kernel_respond_to_challenge(challenge:*mut sk_buff,appdata:*mut krb5_buffer)->i32{let c=rxrpc_skb(challenge);rxgk_respond_to_challenge((*c).chall.conn,challenge,appdata)}

// Security registration and packet/challenge helpers are represented by the surrounding FFI declarations.
pub static mut rxgk_yfs: rxrpc_security = rxrpc_security { name: b"yfs-rxgk\0" as *const _ as _, security_index: RXRPC_SECURITY_YFS_RXGK, no_key_abort: RXGK_NOTAUTH, init: Some(rxgk_init), exit: Some(rxgk_exit), ..rxrpc_security::EMPTY };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
