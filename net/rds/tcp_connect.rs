/*
 * Copyright (c) 2006, 2017 Oracle and/or its affiliates. All rights reserved.
 *
 * This software is available to you under a choice of one of two
 * licenses.  You may choose to be licensed under the terms of the GNU
 * General Public License (GPL) Version 2, available from the file
 * COPYING in the main directory of this source tree, or the
 * OpenIB.org BSD license below:
 *
 *     Redistribution and use in source and binary forms, with or
 *     without modification, are permitted provided that the following
 *     conditions are met:
 *
 *      - Redistributions of source code must retain the above
 *        copyright notice, this list of conditions and the following
 *        disclaimer.
 *
 *      - Redistributions in binary form must reproduce the above
 *        copyright notice, this list of conditions and the following
 *        disclaimer in the documentation and/or other materials
 *        provided with the distribution.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 * BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

pub unsafe fn rds_tcp_state_change(sk: *mut sock) {
    let state_change: unsafe extern "C" fn(*mut sock);
    let cp: *mut rds_conn_path;
    let tc: *mut rds_tcp_connection;

    read_lock_bh(&mut (*sk).sk_callback_lock);
    cp = (*sk).sk_user_data as *mut rds_conn_path;
    if cp.is_null() {
        state_change = (*sk).sk_state_change;
        read_unlock_bh(&mut (*sk).sk_callback_lock);
        state_change(sk);
        return;
    }
    tc = (*cp).cp_transport_data;
    state_change = (*tc).t_orig_state_change;

    rdsdebug!("sock %p state_change to %d\n", (*tc).t_sock, (*sk).sk_state);

    match (*sk).sk_state {
        TCP_SYN_SENT | TCP_SYN_RECV => {}
        TCP_ESTABLISHED => {
            if rds_addr_cmp(&(*(*cp).cp_conn).c_laddr, &(*(*cp).cp_conn).c_faddr) >= 0
                && rds_conn_path_transition(cp, RDS_CONN_CONNECTING, RDS_CONN_ERROR)
            {
                rds_conn_path_drop(cp, false);
            } else {
                rds_connect_path_complete(cp, RDS_CONN_CONNECTING);
            }
        }
        TCP_CLOSING | TCP_TIME_WAIT => {
            if wq_has_sleeper(&(*tc).t_recv_done_waitq) {
                wake_up(&(*tc).t_recv_done_waitq);
            }
        }
        TCP_CLOSE_WAIT | TCP_LAST_ACK | TCP_CLOSE => {
            if wq_has_sleeper(&(*tc).t_recv_done_waitq) {
                wake_up(&(*tc).t_recv_done_waitq);
            }
            rds_conn_path_drop(cp, false);
        }
        _ => {}
    }

    read_unlock_bh(&mut (*sk).sk_callback_lock);
    state_change(sk);
}

pub unsafe fn rds_tcp_conn_path_connect(cp: *mut rds_conn_path) -> i32 {
    let mut sock: *mut socket = core::ptr::null_mut();
    let mut sin6: sockaddr_in6 = core::mem::zeroed();
    let mut sin: sockaddr_in = core::mem::zeroed();
    let mut addr: *mut sockaddr;
    let (mut port_low, mut port_high, mut port): (i32, i32, i32);
    let (mut port_groups, mut groups_left): (i32, i32);
    let mut addrlen: i32;
    let isv6: bool;
    let mut ret: i32;
    let conn: *mut rds_connection = (*cp).cp_conn;
    let tc: *mut rds_tcp_connection = (*cp).cp_transport_data;

    if (*cp).cp_index > 0 && (*(*cp).cp_conn).c_npaths < 2 {
        return -EAGAIN;
    }

    mutex_lock(&mut (*tc).t_conn_path_lock);
    if rds_conn_path_up(cp) {
        mutex_unlock(&mut (*tc).t_conn_path_lock);
        return 0;
    }
    if ipv6_addr_v4mapped(&(*conn).c_laddr) {
        ret = sock_create_kern(rds_conn_net(conn), PF_INET, SOCK_STREAM, IPPROTO_TCP, &mut sock);
        isv6 = false;
    } else {
        ret = sock_create_kern(rds_conn_net(conn), PF_INET6, SOCK_STREAM, IPPROTO_TCP, &mut sock);
        isv6 = true;
    }
    if ret < 0 { goto_out!(); }
    if !rds_tcp_tune(sock) {
        ret = -EINVAL;
        goto_out!();
    }
    if isv6 {
        sin6.sin6_family = AF_INET6;
        sin6.sin6_addr = (*conn).c_laddr;
        sin6.sin6_port = 0;
        sin6.sin6_flowinfo = 0;
        sin6.sin6_scope_id = (*conn).c_dev_if;
        addr = &mut sin6 as *mut _ as *mut sockaddr;
        addrlen = core::mem::size_of::<sockaddr_in6>() as i32;
    } else {
        sin.sin_family = AF_INET;
        sin.sin_addr.s_addr = (*conn).c_laddr.s6_addr32[3];
        sin.sin_port = 0;
        addr = &mut sin as *mut _ as *mut sockaddr;
        addrlen = core::mem::size_of::<sockaddr_in>() as i32;
    }

    inet_get_local_port_range(rds_conn_net(conn), &mut port_low, &mut port_high);
    port_low = ALIGN(port_low, RDS_MPATH_WORKERS);
    port_groups = (port_high - port_low + 1) / RDS_MPATH_WORKERS;
    ret = -EADDRINUSE;
    groups_left = port_groups;
    while groups_left > 0 && ret != 0 {
        groups_left -= 1;
        (*tc).t_client_port_group += 1;
        if (*tc).t_client_port_group >= port_groups { (*tc).t_client_port_group = 0; }
        port = port_low + (*tc).t_client_port_group * RDS_MPATH_WORKERS + (*cp).cp_index;
        if isv6 { sin6.sin6_port = htons(port as u16); } else { sin.sin_port = htons(port as u16); }
        ret = kernel_bind(sock, addr as *mut sockaddr_unsized, addrlen);
    }
    if ret != 0 { goto_out!(); }

    if isv6 {
        sin6.sin6_family = AF_INET6;
        sin6.sin6_addr = (*conn).c_faddr;
        sin6.sin6_port = htons(RDS_TCP_PORT as u16);
        sin6.sin6_flowinfo = 0;
        sin6.sin6_scope_id = (*conn).c_dev_if;
        addr = &mut sin6 as *mut _ as *mut sockaddr;
        addrlen = core::mem::size_of::<sockaddr_in6>() as i32;
    } else {
        sin.sin_family = AF_INET;
        sin.sin_addr.s_addr = (*conn).c_faddr.s6_addr32[3];
        sin.sin_port = htons(RDS_TCP_PORT as u16);
        addr = &mut sin as *mut _ as *mut sockaddr;
        addrlen = core::mem::size_of::<sockaddr_in>() as i32;
    }
    rds_tcp_set_callbacks(sock, cp);
    ret = kernel_connect(sock, addr as *mut sockaddr_unsized, addrlen, O_NONBLOCK);
    if ret == -EINPROGRESS { ret = 0; }
    if ret == 0 { rds_tcp_keepalive(sock); sock = core::ptr::null_mut(); }
    else { rds_tcp_restore_callbacks(sock, (*cp).cp_transport_data); }

    mutex_unlock(&mut (*tc).t_conn_path_lock);
    if !sock.is_null() { sock_release(sock); }
    ret
}

pub unsafe fn rds_tcp_conn_path_shutdown(cp: *mut rds_conn_path) {
    let tc = (*cp).cp_transport_data;
    let sock = (*tc).t_sock;
    let mut sk: *mut sock;
    let mut rounds: u32;
    if !sock.is_null() {
        sk = (*sock).sk;
        if rds_destroy_pending((*cp).cp_conn) { sock_no_linger(sk); }
        ((*(*sock).ops).shutdown)(sock, SHUT_WR);
        rounds = 0;
        loop {
            rds_tcp_recv_path(cp);
            if !(wait_event_timeout(&(*tc).t_recv_done_waitq,
                ((*sk).sk_state == TCP_CLOSING || (*sk).sk_state == TCP_TIME_WAIT ||
                 (*sk).sk_state == TCP_CLOSE_WAIT || (*sk).sk_state == TCP_LAST_ACK ||
                 (*sk).sk_state == TCP_CLOSE) &&
                skb_queue_empty_lockless(&(*sk).sk_receive_queue), msecs_to_jiffies(100)) != 0 &&
                { rounds += 1; rounds < 50 }) { break; }
        }
        lock_sock(sk);
        (*tc).t_last_seen_una = rds_tcp_snd_una(tc);
        rds_send_path_drop_acked(cp, rds_tcp_snd_una(tc), rds_tcp_is_acked);
        rds_tcp_restore_callbacks(sock, tc);
        release_sock(sk);
        sock_release(sock);
    }
    if !(*tc).t_tinc.is_null() {
        rds_inc_put(&mut (*(*tc).t_tinc).ti_inc);
        (*tc).t_tinc = core::ptr::null_mut();
    }
    (*tc).t_tinc_hdr_rem = core::mem::size_of::<rds_header>();
    (*tc).t_tinc_data_rem = 0;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
