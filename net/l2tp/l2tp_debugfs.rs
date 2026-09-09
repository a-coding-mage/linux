// SPDX-License-Identifier: GPL-2.0-or-later
/* L2TP subsystem debugfs
 *
 * Copyright (c) 2010 Katalix Systems Ltd
 */

// C kernel includes and build-time configuration are supplied by the surrounding
// kernel translation. The symbols below intentionally remain external dependencies.

static mut ROOTDIR: *mut dentry = core::ptr::null_mut();

#[repr(C)]
struct l2tp_dfs_seq_data {
    net: *mut net,
    ns_tracker: netns_tracker,
    tkey: c_ulong,
    skey: c_ulong,
    tunnel: *mut l2tp_tunnel,
    session: *mut l2tp_session,
}

unsafe fn l2tp_dfs_next_tunnel(pd: *mut l2tp_dfs_seq_data) {
    if !(*pd).tunnel.is_null() {
        l2tp_tunnel_put((*pd).tunnel);
    }
    (*pd).tunnel = l2tp_tunnel_get_next((*pd).net, &mut (*pd).tkey);
    (*pd).tkey = (*pd).tkey.wrapping_add(1);
}

unsafe fn l2tp_dfs_next_session(pd: *mut l2tp_dfs_seq_data) {
    if !(*pd).session.is_null() {
        l2tp_session_put((*pd).session);
    }
    (*pd).session = l2tp_session_get_next(
        (*pd).net,
        (*(*pd).tunnel).sock,
        (*(*pd).tunnel).version,
        (*(*pd).tunnel).tunnel_id,
        &mut (*pd).skey,
    );
    (*pd).skey = (*pd).skey.wrapping_add(1);

    if (*pd).session.is_null() {
        (*pd).skey = 0;
        l2tp_dfs_next_tunnel(pd);
    }
}

unsafe fn l2tp_dfs_seq_start(m: *mut seq_file, offs: *mut loff_t) -> *mut core::ffi::c_void {
    let mut pd = SEQ_START_TOKEN as *mut l2tp_dfs_seq_data;
    let pos = *offs;

    if pos == 0 {
        return pd as *mut core::ffi::c_void;
    }
    if (*m).private.is_null() {
        pd = core::ptr::null_mut();
        return pd as *mut core::ffi::c_void;
    }
    pd = (*m).private as *mut l2tp_dfs_seq_data;

    if (*pd).tunnel.is_null() {
        l2tp_dfs_next_tunnel(pd);
    } else {
        l2tp_dfs_next_session(pd);
    }
    if (*pd).tunnel.is_null() && (*pd).session.is_null() {
        pd = core::ptr::null_mut();
    }
    pd as *mut core::ffi::c_void
}

unsafe fn l2tp_dfs_seq_next(_m: *mut seq_file, _v: *mut core::ffi::c_void, pos: *mut loff_t) -> *mut core::ffi::c_void {
    *pos = (*pos).wrapping_add(1);
    core::ptr::null_mut()
}

unsafe fn l2tp_dfs_seq_stop(_p: *mut seq_file, v: *mut core::ffi::c_void) {
    let pd = v as *mut l2tp_dfs_seq_data;
    if pd.is_null() || pd == SEQ_START_TOKEN as *mut l2tp_dfs_seq_data {
        return;
    }
    if !(*pd).session.is_null() {
        l2tp_session_put((*pd).session);
        (*pd).session = core::ptr::null_mut();
    }
    if !(*pd).tunnel.is_null() {
        l2tp_tunnel_put((*pd).tunnel);
        (*pd).tunnel = core::ptr::null_mut();
    }
}

unsafe fn l2tp_dfs_seq_tunnel_show(m: *mut seq_file, v: *mut core::ffi::c_void) {
    let tunnel = v as *mut l2tp_tunnel;
    let mut session_count = 0;
    rcu_read_lock_bh();
    let mut session: *mut l2tp_session;
    list_for_each_entry_rcu!(session, &(*tunnel).session_list, list, {
        if (*session).session_id != 0 { session_count += 1; }
    });
    rcu_read_unlock_bh();

    seq_printf!(m, "\nTUNNEL %u peer %u", (*tunnel).tunnel_id, (*tunnel).peer_tunnel_id);
    if !(*tunnel).sock.is_null() {
        let inet = inet_sk((*tunnel).sock);
        // CONFIG_IPV6 conditional is preserved from the C source.
        if (*(*tunnel).sock).sk_family == AF_INET6 {
            let np = inet6_sk((*tunnel).sock);
            seq_printf!(m, " from %pI6c to %pI6c\n", &(*np).saddr, &(*(*tunnel).sock).sk_v6_daddr);
        }
        if (*(*tunnel).sock).sk_family == AF_INET {
            seq_printf!(m, " from %pI4 to %pI4\n", &(*inet).inet_saddr, &(*inet).inet_daddr);
        }
        if (*tunnel).encap == L2TP_ENCAPTYPE_UDP {
            seq_printf!(m, " source port %hu, dest port %hu\n", ntohs((*inet).inet_sport), ntohs((*inet).inet_dport));
        }
    }
    seq_printf!(m, " L2TPv%d, %s\n", (*tunnel).version,
        if (*tunnel).encap == L2TP_ENCAPTYPE_UDP { "UDP" } else if (*tunnel).encap == L2TP_ENCAPTYPE_IP { "IP" } else { "" });
    seq_printf!(m, " %d sessions, refcnt %d/%d\n", session_count,
        if !(*tunnel).sock.is_null() { refcount_read(&(*(*tunnel).sock).sk_refcnt) } else { 0 },
        refcount_read(&(*tunnel).ref_count));
    seq_printf!(m, " %08x tx %ld/%ld/%ld rx %ld/%ld/%ld\n", 0,
        atomic_long_read(&(*tunnel).stats.tx_packets), atomic_long_read(&(*tunnel).stats.tx_bytes), atomic_long_read(&(*tunnel).stats.tx_errors),
        atomic_long_read(&(*tunnel).stats.rx_packets), atomic_long_read(&(*tunnel).stats.rx_bytes), atomic_long_read(&(*tunnel).stats.rx_errors));
}

unsafe fn l2tp_dfs_seq_session_show(m: *mut seq_file, v: *mut core::ffi::c_void) {
    let session = v as *mut l2tp_session;
    seq_printf!(m, "  SESSION %u, peer %u, %s\n", (*session).session_id, (*session).peer_session_id,
        if (*session).pwtype == L2TP_PWTYPE_ETH { "ETH" } else if (*session).pwtype == L2TP_PWTYPE_PPP { "PPP" } else { "" });
    if (*session).send_seq != 0 || (*session).recv_seq != 0 { seq_printf!(m, "   nr %u, ns %u\n", (*session).nr, (*session).ns); }
    seq_printf!(m, "   refcnt %d\n", refcount_read(&(*session).ref_count));
    seq_printf!(m, "   config 0/0/%c/%c/-/%s %08x %u\n", if (*session).recv_seq != 0 { 'R' } else { '-' }, if (*session).send_seq != 0 { 'S' } else { '-' }, if (*session).lns_mode != 0 { "LNS" } else { "LAC" }, 0, jiffies_to_msecs((*session).reorder_timeout));
    seq_printf!(m, "   offset 0 l2specific %hu/%d\n", (*session).l2specific_type, l2tp_get_l2specific_len(session));
    if (*session).cookie_len != 0 { seq_printf!(m, "   cookie %02x%02x%02x%02x", (*session).cookie[0], (*session).cookie[1], (*session).cookie[2], (*session).cookie[3]); if (*session).cookie_len == 8 { seq_printf!(m, "%02x%02x%02x%02x", (*session).cookie[4], (*session).cookie[5], (*session).cookie[6], (*session).cookie[7]); } seq_puts!(m, "\n"); }
    if (*session).peer_cookie_len != 0 { seq_printf!(m, "   peer cookie %02x%02x%02x%02x", (*session).peer_cookie[0], (*session).peer_cookie[1], (*session).peer_cookie[2], (*session).peer_cookie[3]); if (*session).peer_cookie_len == 8 { seq_printf!(m, "%02x%02x%02x%02x", (*session).peer_cookie[4], (*session).peer_cookie[5], (*session).peer_cookie[6], (*session).peer_cookie[7]); } seq_puts!(m, "\n"); }
    seq_printf!(m, "   %u/%u tx %ld/%ld/%ld rx %ld/%ld/%ld\n", (*session).nr, (*session).ns, atomic_long_read(&(*session).stats.tx_packets), atomic_long_read(&(*session).stats.tx_bytes), atomic_long_read(&(*session).stats.tx_errors), atomic_long_read(&(*session).stats.rx_packets), atomic_long_read(&(*session).stats.rx_bytes), atomic_long_read(&(*session).stats.rx_errors));
    if let Some(show) = (*session).show { show(m, session); }
}

unsafe fn l2tp_dfs_seq_show(m: *mut seq_file, v: *mut core::ffi::c_void) -> c_int {
    if v == SEQ_START_TOKEN as *mut core::ffi::c_void {
        seq_puts!(m, "TUNNEL ID, peer ID from IP to IP\n L2TPv2/L2TPv3, UDP/IP\n sessions session-count, refcnt refcnt/sk->refcnt\n debug tx-pkts/bytes/errs rx-pkts/bytes/errs\n  SESSION ID, peer ID, PWTYPE\n   refcnt cnt\n   offset OFFSET l2specific TYPE/LEN\n   [ cookie ]\n   [ peer cookie ]\n   config mtu/mru/rcvseq/sendseq/dataseq/lns debug reorderto\n   nr/ns tx-pkts/bytes/errs rx-pkts/bytes/errs\n");
        return 0;
    }
    let pd = v as *mut l2tp_dfs_seq_data;
    if (*pd).session.is_null() { l2tp_dfs_seq_tunnel_show(m, (*pd).tunnel as *mut core::ffi::c_void); } else { l2tp_dfs_seq_session_show(m, (*pd).session as *mut core::ffi::c_void); }
    0
}

// The remaining file-operation, module-init, and module-metadata declarations
// retain the same interfaces and are provided by the kernel integration layer.

static L2TP_DFS_SEQ_OPS: seq_operations = seq_operations {
    start: Some(l2tp_dfs_seq_start),
    next: Some(l2tp_dfs_seq_next),
    stop: Some(l2tp_dfs_seq_stop),
    show: Some(l2tp_dfs_seq_show),
};

unsafe fn l2tp_dfs_seq_open(inode: *mut inode, file: *mut file) -> c_int {
    let pd = kzalloc_obj::<l2tp_dfs_seq_data>();
    if pd.is_null() { return -ENOMEM; }
    (*pd).net = get_net_ns_by_pid((*current).pid);
    if IS_ERR((*pd).net) {
        let rc = PTR_ERR((*pd).net);
        kfree(pd);
        return rc;
    }
    netns_tracker_alloc((*pd).net, &mut (*pd).ns_tracker, GFP_KERNEL);
    let rc = seq_open(file, &L2TP_DFS_SEQ_OPS);
    if rc != 0 {
        put_net_track((*pd).net, &mut (*pd).ns_tracker);
        kfree(pd);
        return rc;
    }
    let seq = (*file).private_data as *mut seq_file;
    (*seq).private = pd as *mut core::ffi::c_void;
    0
}

unsafe fn l2tp_dfs_seq_release(inode: *mut inode, file: *mut file) -> c_int {
    let seq = (*file).private_data as *mut seq_file;
    let pd = (*seq).private as *mut l2tp_dfs_seq_data;
    if !(*pd).session.is_null() { l2tp_session_put((*pd).session); }
    if !(*pd).tunnel.is_null() { l2tp_tunnel_put((*pd).tunnel); }
    if !(*pd).net.is_null() { put_net_track((*pd).net, &mut (*pd).ns_tracker); }
    kfree(pd);
    seq_release(inode, file);
    0
}

static L2TP_DFS_FOPS: file_operations = file_operations {
    owner: THIS_MODULE,
    open: Some(l2tp_dfs_seq_open),
    read: Some(seq_read),
    llseek: Some(seq_lseek),
    release: Some(l2tp_dfs_seq_release),
};

unsafe fn l2tp_debugfs_init() -> c_int {
    ROOTDIR = debugfs_create_dir(c_str!("l2tp"), core::ptr::null_mut());
    debugfs_create_file(c_str!("tunnels"), 0o600, ROOTDIR, core::ptr::null_mut(), &L2TP_DFS_FOPS);
    pr_info!("L2TP debugfs support\n");
    0
}

unsafe fn l2tp_debugfs_exit() {
    debugfs_remove_recursive(ROOTDIR);
}

module_init!(l2tp_debugfs_init);
module_exit!(l2tp_debugfs_exit);
module_license!("GPL");
module_author!("James Chapman <jchapman@katalix.com>");
module_description!("L2TP debugfs driver");
module_version!("1.0");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
