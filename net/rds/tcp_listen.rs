/* Translated from tcp_listen.c. External kernel/RDS declarations are supplied by dependencies. */

pub unsafe fn rds_tcp_keepalive(sock: *mut socket) {
    /* values below based on xs_udp_default_timeout */
    let keepidle: i32 = 5; /* send a probe 'keepidle' secs after last data */
    let keepcnt: i32 = 5; /* number of unack'ed probes before declaring dead */

    sock_set_keepalive((*sock).sk);
    tcp_sock_set_keepcnt((*sock).sk, keepcnt);
    tcp_sock_set_keepidle((*sock).sk, keepidle);
    /* KEEPINTVL is the interval between successive probes. We follow
     * the model in xs_tcp_finish_connecting() and re-use keepidle.
     */
    tcp_sock_set_keepintvl((*sock).sk, keepidle);
}

unsafe fn rds_tcp_get_peer_sport(sock: *mut socket) -> i32 {
    let sk = (*sock).sk;
    if sk.is_null() {
        return -1;
    }
    ntohs(READ_ONCE((*inet_sk(sk)).inet_dport)) as i32
}

/* rds_tcp_accept_one_path(): if accepting on cp_index > 0, make sure the
 * client's ipaddr < server's ipaddr. Otherwise, close the accepted
 * socket and force a reconnect from smaller -> larger ip addr. The reason
 * we special case cp_index 0 is to allow the rds probe ping itself to itself
 * get through efficiently.
 */
unsafe fn rds_tcp_accept_one_path(
    conn: *mut rds_connection,
    sock: *mut socket,
) -> *mut rds_tcp_connection {
    let sport: i32;
    let npaths: i32;
    let i_min: i32;
    let i_max: i32;

    if (*conn).c_with_sport_idx {
        /* cp->cp_index is encoded in lowest bits of source-port */
        sport = rds_tcp_get_peer_sport(sock);
    } else {
        sport = -1;
    }

    npaths = core::cmp::max(1, (*conn).c_npaths);
    if sport >= 0 {
        i_min = sport % npaths;
        i_max = i_min;
    } else {
        i_min = 0;
        i_max = npaths - 1;
    }

    let mut i = i_min;
    while i <= i_max {
        let cp = &mut *(*conn).c_path.add(i as usize);
        if rds_conn_path_transition(cp, RDS_CONN_DOWN, RDS_CONN_CONNECTING) {
            return cp.cp_transport_data;
        }
        i += 1;
    }
    core::ptr::null_mut()
}

pub unsafe fn rds_tcp_conn_slots_available(conn: *mut rds_connection, fan_out: bool) {
    if rds_destroy_pending(conn) { return; }
    let tc = (*conn).c_path.cp_transport_data;
    let rtn = (*tc).t_rtn;
    if rtn.is_null() { return; }
    let sock = (*tc).t_sock;

    /* During fan-out, check that the connection we already
     * accepted in slot#0 carried the proper source port modulo.
     */
    if fan_out && (*conn).c_with_sport_idx && !sock.is_null() &&
       rds_addr_cmp(&(*conn).c_laddr, &(*conn).c_faddr) > 0 {
        /* cp->cp_index is encoded in lowest bits of source-port */
        let sport = rds_tcp_get_peer_sport(sock);
        let npaths = core::cmp::max(1, (*conn).c_npaths);
        if sport >= 0 && sport % npaths != 0 {
            /* peer initiated with a non-#0 lane first */
            rds_conn_path_drop((*conn).c_path, 0);
        }
    }
    /* As soon as a connection went down, it is safe to schedule a
     * "rds_tcp_accept_one" attempt even if there are no connections pending.
     */
    rds_tcp_accept_work(rtn);
}

pub unsafe fn rds_tcp_accept_one(rtn: *mut rds_tcp_net) -> i32 {
    let listen_sock = (*rtn).rds_tcp_listen_sock;
    let mut new_sock: *mut socket = core::ptr::null_mut();
    let mut rs_tcp: *mut rds_tcp_connection = core::ptr::null_mut();
    let mut ret: i32;
    if listen_sock.is_null() { return -ENETUNREACH; }

    mutex_lock(&mut (*rtn).rds_tcp_accept_lock);
    new_sock = (*rtn).rds_tcp_accepted_sock;
    (*rtn).rds_tcp_accepted_sock = core::ptr::null_mut();
    if new_sock.is_null() {
        ret = kernel_accept(listen_sock, &mut new_sock, O_NONBLOCK);
        if ret != 0 { goto_out(rtn, new_sock, rs_tcp, ret); return ret; }
        rds_tcp_keepalive(new_sock);
        if !rds_tcp_tune(new_sock) { ret = -EINVAL; goto_out(rtn, new_sock, rs_tcp, ret); return ret; }
    }

    let inet = inet_sk((*new_sock).sk);
    let (my_addr, peer_addr): (*mut in6_addr, *mut in6_addr);
    #[cfg(feature = "ipv6")]
    { my_addr = &mut (*(*new_sock).sk).sk_v6_rcv_saddr; peer_addr = &mut (*(*new_sock).sk).sk_v6_daddr; }
    #[cfg(not(feature = "ipv6"))]
    { let mut saddr = core::mem::zeroed(); let mut daddr = core::mem::zeroed(); ipv6_addr_set_v4mapped((*inet).inet_saddr, &mut saddr); ipv6_addr_set_v4mapped((*inet).inet_daddr, &mut daddr); my_addr = &mut saddr; peer_addr = &mut daddr; }

    let dev_if: i32 = 0;
    if !rds_tcp_laddr_check(sock_net((*listen_sock).sk), peer_addr, dev_if) { ret = -EOPNOTSUPP; goto_out(rtn, new_sock, rs_tcp, ret); return ret; }
    let conn = rds_conn_create(sock_net((*listen_sock).sk), my_addr, peer_addr, &rds_tcp_transport, 0, GFP_KERNEL, dev_if);
    if IS_ERR(conn) { ret = PTR_ERR(conn); goto_out(rtn, new_sock, rs_tcp, ret); return ret; }
    if rds_addr_cmp(&(*conn).c_faddr, &(*conn).c_laddr) < 0 {
        rs_tcp = rds_tcp_accept_one_path(conn, new_sock);
        if rs_tcp.is_null() { (*rtn).rds_tcp_accepted_sock = new_sock; new_sock = core::ptr::null_mut(); ret = -ENOBUFS; goto_out(rtn, new_sock, rs_tcp, ret); return ret; }
    } else {
        if (*conn).c_npaths <= 1 { rds_conn_path_connect_if_down(&mut *(*conn).c_path); }
        goto rst_nsk;
    }
    mutex_lock(&mut (*rs_tcp).t_conn_path_lock);
    let cp = (*rs_tcp).t_cpath;
    let conn_state = rds_conn_path_state(cp);
    WARN_ON(conn_state == RDS_CONN_UP);
    if conn_state != RDS_CONN_CONNECTING && conn_state != RDS_CONN_ERROR { rds_conn_path_drop(cp, 0); goto rst_nsk; }
    let sk = (*new_sock).sk; sock_hold(sk);
    if !(*rs_tcp).t_sock.is_null() { rds_tcp_reset_callbacks(new_sock, cp); rds_connect_path_complete(cp, RDS_CONN_RESETTING); }
    else { rds_tcp_set_callbacks(new_sock, cp); rds_connect_path_complete(cp, RDS_CONN_CONNECTING); }
    if READ_ONCE((*sk).sk_state) == TCP_CLOSE_WAIT || READ_ONCE((*sk).sk_state) == TCP_LAST_ACK || READ_ONCE((*sk).sk_state) == TCP_CLOSE { rds_conn_path_drop(cp, 0); } else { queue_delayed_work((*cp).cp_wq, &mut (*cp).cp_recv_w, 0); }
    sock_put(sk); new_sock = core::ptr::null_mut(); ret = 0;
    if (*conn).c_npaths == 0 { rds_send_ping((*cp).cp_conn, (*cp).cp_index); }
    goto out;
rst_nsk:
    sock_no_linger((*new_sock).sk); kernel_sock_shutdown(new_sock, SHUT_RDWR); ret = 0;
out:
    if !rs_tcp.is_null() { mutex_unlock(&mut (*rs_tcp).t_conn_path_lock); }
    if !new_sock.is_null() { sock_release(new_sock); }
    mutex_unlock(&mut (*rtn).rds_tcp_accept_lock); return ret;
}

pub unsafe fn rds_tcp_listen_data_ready(sk: *mut sock) {
    trace_sk_data_ready(sk); rdsdebug!("listen data ready sk %p\n", sk);
    read_lock_bh(&mut (*sk).sk_callback_lock);
    let mut ready = (*sk).sk_user_data;
    if ready.is_null() { ready = (*sk).sk_data_ready; }
    else if (*sk).sk_state == TCP_LISTEN { rds_tcp_accept_work(net_generic(sock_net(sk), rds_tcp_netid)); }
    else { ready = rds_tcp_listen_sock_def_readable(sock_net(sk)); }
    read_unlock_bh(&mut (*sk).sk_callback_lock);
    if !ready.is_null() { ready(sk); }
}

pub unsafe fn rds_tcp_listen_init(net: *mut net, isv6: bool) -> *mut socket {
    let mut sock: *mut socket = core::ptr::null_mut();
    let mut ss: sockaddr_storage = core::mem::zeroed();
    let (family, addr_len) = if isv6 { (PF_INET6, core::mem::size_of::<sockaddr_in6>()) } else { (PF_INET, core::mem::size_of::<sockaddr_in>()) };
    let mut ret = sock_create_kern(net, family, SOCK_STREAM, IPPROTO_TCP, &mut sock);
    if ret < 0 { rdsdebug!("could not create {} listener socket: {}\n", if isv6 { "IPv6" } else { "IPv4" }, ret); return core::ptr::null_mut(); }
    (*sock).sk.sk_reuse = SK_CAN_REUSE; tcp_sock_set_nodelay((*sock).sk);
    write_lock_bh(&mut (*(*sock).sk).sk_callback_lock); (*(*sock).sk).sk_user_data = (*(*sock).sk).sk_data_ready; (*(*sock).sk).sk_data_ready = rds_tcp_listen_data_ready; write_unlock_bh(&mut (*(*sock).sk).sk_callback_lock);
    if isv6 { let sin6 = &mut *(&mut ss as *mut _ as *mut sockaddr_in6); sin6.sin6_family = PF_INET6; sin6.sin6_addr = in6addr_any; sin6.sin6_port = htons(RDS_TCP_PORT); sin6.sin6_scope_id = 0; sin6.sin6_flowinfo = 0; } else { let sin = &mut *(&mut ss as *mut _ as *mut sockaddr_in); sin.sin_family = PF_INET; sin.sin_addr.s_addr = htonl(INADDR_ANY); sin.sin_port = htons(RDS_TCP_PORT); }
    ret = kernel_bind(sock, &ss as *const _ as *mut sockaddr_unsized, addr_len);
    if ret < 0 { rdsdebug!("could not bind {} listener socket: {}\n", if isv6 { "IPv6" } else { "IPv4" }, ret); sock_release(sock); return core::ptr::null_mut(); }
    ret = ((*(*sock).ops).listen)(sock, 64); if ret < 0 { sock_release(sock); return core::ptr::null_mut(); } sock
}

pub unsafe fn rds_tcp_listen_stop(sock: *mut socket, acceptor: *mut work_struct) {
    if sock.is_null() { return; }
    let sk = (*sock).sk; lock_sock(sk); write_lock_bh(&mut (*sk).sk_callback_lock);
    if !(*sk).sk_user_data.is_null() { (*sk).sk_data_ready = (*sk).sk_user_data; (*sk).sk_user_data = core::ptr::null_mut(); }
    write_unlock_bh(&mut (*sk).sk_callback_lock); release_sock(sk); flush_workqueue(rds_wq); flush_work(acceptor); sock_release(sock);
}

unsafe fn goto_out(_rtn: *mut rds_tcp_net, _sock: *mut socket, _rs: *mut rds_tcp_connection, _ret: i32) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
