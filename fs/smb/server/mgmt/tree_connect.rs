// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (C) 2018 Samsung Electronics Co., Ltd.
 */

// Translated from the C implementation. Kernel and project dependencies are
// supplied by the surrounding repository.

pub unsafe fn ksmbd_tree_conn_connect(
    work: *mut ksmbd_work,
    share_name: *const ::std::os::raw::c_char,
) -> ksmbd_tree_conn_status {
    let mut status = ksmbd_tree_conn_status { ret: -ENOENT, tree_conn: ::std::ptr::null_mut() };
    let mut resp: *mut ksmbd_tree_connect_response = ::std::ptr::null_mut();
    let mut sc: *mut ksmbd_share_config;
    let mut tree_conn: *mut ksmbd_tree_connect = ::std::ptr::null_mut();
    let peer_addr: *mut sockaddr;
    let conn = (*work).conn;
    let sess = (*work).sess;
    let mut ret: i32;

    sc = ksmbd_share_config_get(work, share_name);
    if sc.is_null() {
        return status;
    }

    tree_conn = kzalloc_obj::<ksmbd_tree_connect>(KSMBD_DEFAULT_GFP);
    if tree_conn.is_null() {
        status.ret = -ENOMEM;
        goto_out_error!();
    }

    (*tree_conn).id = ksmbd_acquire_tree_conn_id(sess);
    if (*tree_conn).id < 0 {
        status.ret = -EINVAL;
        goto_out_error!();
    }

    peer_addr = KSMBD_TCP_PEER_SOCKADDR(conn);
    resp = ksmbd_ipc_tree_connect_request(sess, sc, tree_conn, peer_addr);
    if resp.is_null() {
        status.ret = -EINVAL;
        goto_out_error!();
    }

    status.ret = (*resp).status;
    if status.ret != KSMBD_TREE_CONN_STATUS_OK {
        goto_out_error!();
    }

    (*tree_conn).flags = (*resp).connection_flags;
    if test_tree_conn_flag(tree_conn, KSMBD_TREE_CONN_FLAG_UPDATE) {
        let mut new_sc: *mut ksmbd_share_config;

        ksmbd_share_config_del(sc);
        new_sc = ksmbd_share_config_get(work, share_name);
        if new_sc.is_null() {
            pr_err!("Failed to update stale share config\n");
            status.ret = -ESTALE;
            goto_out_error!();
        }
        ksmbd_share_config_put(sc);
        sc = new_sc;
    }

    (*tree_conn).user = (*sess).user;
    (*tree_conn).share_conf = sc;
    (*tree_conn).t_state = TREE_NEW;
    status.tree_conn = tree_conn;
    atomic_set(&mut (*tree_conn).refcount, 1);

    down_write(&mut (*sess).tree_conns_lock);
    ret = xa_err(xa_store(&mut (*sess).tree_conns, (*tree_conn).id, tree_conn,
                          KSMBD_DEFAULT_GFP));
    up_write(&mut (*sess).tree_conns_lock);
    if ret != 0 {
        status.ret = -ENOMEM;
        goto_out_error!();
    }
    ksmbd_counter_inc(KSMBD_COUNTER_TREE_CONNS);
    ksmbd_share_tree_conn_inc(sc);
    kvfree(resp);
    return status;

    macro_rules! goto_out_error { () => {{
        if !tree_conn.is_null() {
            ksmbd_release_tree_conn_id(sess, (*tree_conn).id);
        }
        ksmbd_share_config_put(sc);
        kfree(tree_conn);
        kvfree(resp);
        return status;
    }} }
}

pub unsafe fn ksmbd_tree_connect_put(tcon: *mut ksmbd_tree_connect) {
    if atomic_dec_and_test(&mut (*tcon).refcount) {
        ksmbd_share_config_put((*tcon).share_conf);
        kfree(tcon);
    }
}

unsafe fn __ksmbd_tree_conn_disconnect(
    sess: *mut ksmbd_session,
    tree_conn: *mut ksmbd_tree_connect,
) -> i32 {
    let ret = ksmbd_ipc_tree_disconnect_request((*sess).id, (*tree_conn).id);
    ksmbd_release_tree_conn_id(sess, (*tree_conn).id);
    ksmbd_counter_dec(KSMBD_COUNTER_TREE_CONNS);
    ksmbd_share_tree_conn_dec((*tree_conn).share_conf);
    if atomic_dec_and_test(&mut (*tree_conn).refcount) {
        ksmbd_share_config_put((*tree_conn).share_conf);
        kfree(tree_conn);
    }
    ret
}

pub unsafe fn ksmbd_tree_conn_disconnect(
    sess: *mut ksmbd_session,
    tree_conn: *mut ksmbd_tree_connect,
) -> i32 {
    down_write(&mut (*sess).tree_conns_lock);
    xa_erase(&mut (*sess).tree_conns, (*tree_conn).id);
    up_write(&mut (*sess).tree_conns_lock);
    __ksmbd_tree_conn_disconnect(sess, tree_conn)
}

pub unsafe fn ksmbd_tree_conn_lookup(
    sess: *mut ksmbd_session,
    id: ::std::os::raw::c_uint,
) -> *mut ksmbd_tree_connect {
    let mut tcon: *mut ksmbd_tree_connect;
    down_read(&mut (*sess).tree_conns_lock);
    tcon = xa_load(&mut (*sess).tree_conns, id);
    if !tcon.is_null() {
        if (*tcon).t_state != TREE_CONNECTED {
            tcon = ::std::ptr::null_mut();
        } else if !atomic_inc_not_zero(&mut (*tcon).refcount) {
            tcon = ::std::ptr::null_mut();
        }
    }
    up_read(&mut (*sess).tree_conns_lock);
    tcon
}

pub unsafe fn ksmbd_tree_conn_session_logoff(sess: *mut ksmbd_session) -> i32 {
    let mut ret: i32 = 0;
    if sess.is_null() {
        return -EINVAL;
    }

    down_write(&mut (*sess).tree_conns_lock);
    xa_for_each!(&mut (*sess).tree_conns, id, tc, {
        if (*tc).t_state == TREE_DISCONNECTED {
            ret = -ENOENT;
            continue;
        }
        (*tc).t_state = TREE_DISCONNECTED;
        xa_erase(&mut (*sess).tree_conns, (*tc).id);
        ret |= __ksmbd_tree_conn_disconnect(sess, tc);
    });
    xa_destroy(&mut (*sess).tree_conns);
    up_write(&mut (*sess).tree_conns_lock);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
